//! Template interpolation.
//!
//! Provides mustache-style template interpolation with generator placeholders.
//!
//! # Example
//!
//! ```
//! use dx_datagen::text::template::{Template, ProviderRegistry};
//! use rand::{Rng, SeedableRng};
//! use rand::rngs::StdRng;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! // Create a registry with built-in providers
//! let mut registry = ProviderRegistry::with_defaults();
//!
//! // Add custom providers
//! registry.add("id", |r| format!("{}", r.random_range(1000..9999)));
//!
//! // Create and render a template
//! let template = Template::new("Hello {{first_name}}, your order #{{id}} is ready!");
//! let result = template.render(&mut rng, &registry);
//! println!("{}", result);
//! ```

use rand::Rng;
use std::collections::HashMap;

/// A provider function that generates a string value.
pub type ProviderFn = Box<dyn Fn(&mut dyn rand::RngCore) -> String + Send + Sync>;

/// Registry of named providers for template interpolation.
#[derive(Default)]
pub struct ProviderRegistry {
    providers: HashMap<String, ProviderFn>,
}

impl ProviderRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Create a registry with default providers from dx-datagen.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // Personal data
        registry.add("first_name", |r| crate::personal::first_name(r).to_string());
        registry.add("last_name", |r| crate::personal::last_name(r).to_string());
        registry.add("full_name", |r| crate::personal::full_name(r));
        registry.add("email", |r| crate::personal::email(r));
        registry.add("phone", |r| crate::personal::phone(r));
        registry.add("username", |r| crate::personal::username(r));

        // Network
        registry.add("domain", |r| crate::network::domain(r));
        registry.add("ipv4", |r| crate::network::ipv4(r).to_string());
        registry.add("url", |r| crate::network::url(r));

        // Commerce
        registry.add("company", |r| crate::commerce::company_name(r));
        registry.add("job_title", |r| crate::commerce::job_title(r));
        registry.add("product", |r| crate::commerce::product_name(r));
        registry.add("price", |r| {
            crate::commerce::price_formatted(r, "$", 1.0, 1000.0)
        });
        registry.add("currency", |r| {
            crate::commerce::currency_code(r).to_string()
        });

        // Text
        registry.add("word", |r| crate::text::word(r).to_string());
        registry.add("noun", |r| crate::text::noun(r).to_string());
        registry.add("adjective", |r| crate::text::adjective(r).to_string());
        registry.add("verb", |r| crate::text::verb(r).to_string());
        registry.add("sentence", |r| crate::text::lorem::sentence(r));
        registry.add("paragraph", |r| crate::text::lorem::paragraph(r));

        // UUID
        registry.add("uuid", |_| crate::uuid::v4().to_string());
        registry.add("ulid", |_| crate::uuid::ulid().to_string());

        // Numbers
        registry.add("int", |r| format!("{}", r.random_range(0..1000)));
        registry.add("float", |r| format!("{:.2}", r.random::<f64>() * 100.0));
        registry.add("digit", |r| format!("{}", r.random_range(0..10)));
        registry.add("bool", |r| format!("{}", r.random::<bool>()));

        // Color
        registry.add("color", |r| crate::color::color_name(r).to_string());
        registry.add("hex_color", |r| crate::color::hex_color(r));

        // Vehicle
        registry.add("vehicle", |r| crate::vehicle::vehicle_full(r));
        registry.add("license_plate", |r| crate::vehicle::license_plate(r));

        // Categories
        registry.add("city", |r| {
            crate::generators::pick_one(r, crate::categories::CITIES).to_string()
        });
        registry.add("country", |r| {
            crate::generators::pick_one(r, crate::categories::COUNTRIES).to_string()
        });
        registry.add("fruit", |r| crate::food::fruit(r).to_string());
        registry.add("animal", |r| crate::animals::animal(r).to_string());

        // Entertainment
        registry.add("book_title", |r| crate::entertainment::book_title(r));
        registry.add("movie_title", |r| crate::entertainment::movie_title(r));
        registry.add("music_artist", |r| {
            crate::entertainment::music_artist(r).to_string()
        });
        registry.add("game_title", |r| crate::entertainment::game_title(r));

        // Travel
        registry.add("airline", |r| crate::travel::airline(r).to_string());
        registry.add("airport", |r| crate::travel::airport_code(r).to_string());
        registry.add("destination", |r| crate::travel::destination(r).to_string());

        // Healthcare
        registry.add("medication", |r| {
            crate::healthcare::medication(r).to_string()
        });
        registry.add("blood_type", |r| {
            crate::healthcare::blood_type(r).to_string()
        });

        // Sports
        registry.add("sport", |r| crate::sports::sport(r).to_string());
        registry.add("team", |r| crate::sports::team_name(r));

        registry
    }

    /// Add a provider function.
    pub fn add<F>(&mut self, name: &str, provider: F)
    where
        F: Fn(&mut dyn rand::RngCore) -> String + Send + Sync + 'static,
    {
        self.providers.insert(name.to_string(), Box::new(provider));
    }

    /// Get a provider by name.
    pub fn get(&self, name: &str) -> Option<&ProviderFn> {
        self.providers.get(name)
    }

    /// Check if a provider exists.
    pub fn contains(&self, name: &str) -> bool {
        self.providers.contains_key(name)
    }

    /// List all available providers.
    pub fn list(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }
}

/// A template with placeholders for interpolation.
#[derive(Debug, Clone)]
pub struct Template {
    template: String,
}

impl Template {
    /// Create a new template from a string.
    ///
    /// Placeholders are in the format `{{name}}`.
    pub fn new(template: &str) -> Self {
        Self {
            template: template.to_string(),
        }
    }

    /// Render the template using providers from the registry.
    ///
    /// Unknown placeholders are left unchanged.
    pub fn render<R: Rng>(&self, rng: &mut R, registry: &ProviderRegistry) -> String {
        let mut result = self.template.clone();
        let mut start = 0;

        while let Some(open) = result[start..].find("{{") {
            let open_abs = start + open;
            if let Some(close) = result[open_abs..].find("}}") {
                let close_abs = open_abs + close;
                let placeholder = &result[open_abs + 2..close_abs];
                let name = placeholder.trim();

                if let Some(provider) = registry.get(name) {
                    // Convert to RngCore for the provider
                    let value = provider(rng as &mut dyn rand::RngCore);
                    result = format!(
                        "{}{}{}",
                        &result[..open_abs],
                        value,
                        &result[close_abs + 2..]
                    );
                    start = open_abs + value.len();
                } else {
                    // Unknown placeholder, skip it
                    start = close_abs + 2;
                }
            } else {
                break;
            }
        }

        result
    }

    /// Render with custom variable values (no generators).
    pub fn render_with_vars(&self, vars: &HashMap<String, String>) -> String {
        let mut result = self.template.clone();

        for (name, value) in vars {
            let placeholder = format!("{{{{{}}}}}", name);
            result = result.replace(&placeholder, value);
        }

        result
    }

    /// Get the raw template string.
    pub fn as_str(&self) -> &str {
        &self.template
    }

    /// Extract all placeholder names from the template.
    pub fn placeholders(&self) -> Vec<String> {
        let mut names = Vec::new();
        let mut pos = 0;

        while let Some(open) = self.template[pos..].find("{{") {
            let open_abs = pos + open;
            if let Some(close) = self.template[open_abs..].find("}}") {
                let close_abs = open_abs + close;
                let name = self.template[open_abs + 2..close_abs].trim().to_string();
                if !names.contains(&name) {
                    names.push(name);
                }
                pos = close_abs + 2;
            } else {
                break;
            }
        }

        names
    }
}

/// Convenience function to render a template string.
pub fn render<R: Rng>(rng: &mut R, template: &str, registry: &ProviderRegistry) -> String {
    Template::new(template).render(rng, registry)
}

/// Convenience function to render a template with default providers.
pub fn render_default<R: Rng>(rng: &mut R, template: &str) -> String {
    let registry = ProviderRegistry::with_defaults();
    Template::new(template).render(rng, &registry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_template_basic() {
        let mut rng = StdRng::seed_from_u64(42);
        let registry = ProviderRegistry::with_defaults();

        let template = Template::new("Hello {{first_name}}!");
        let result = template.render(&mut rng, &registry);

        assert!(!result.contains("{{"));
        assert!(result.starts_with("Hello "));
        assert!(result.ends_with("!"));
    }

    #[test]
    fn test_template_multiple_placeholders() {
        let mut rng = StdRng::seed_from_u64(42);
        let registry = ProviderRegistry::with_defaults();

        let template = Template::new("{{first_name}} works at {{company}}");
        let result = template.render(&mut rng, &registry);

        assert!(!result.contains("{{"));
        assert!(result.contains(" works at "));
    }

    #[test]
    fn test_template_unknown_placeholder() {
        let mut rng = StdRng::seed_from_u64(42);
        let registry = ProviderRegistry::with_defaults();

        let template = Template::new("Hello {{unknown_placeholder}}!");
        let result = template.render(&mut rng, &registry);

        // Unknown placeholders are left unchanged
        assert!(result.contains("{{unknown_placeholder}}"));
    }

    #[test]
    fn test_custom_provider() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut registry = ProviderRegistry::new();

        registry.add("custom", |r| format!("custom_{}", r.random_range(1..100)));

        let template = Template::new("Value: {{custom}}");
        let result = template.render(&mut rng, &registry);

        assert!(result.starts_with("Value: custom_"));
    }

    #[test]
    fn test_render_with_vars() {
        let template = Template::new("Hello {{name}}, you have {{count}} items");
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Alice".to_string());
        vars.insert("count".to_string(), "5".to_string());

        let result = template.render_with_vars(&vars);
        assert_eq!(result, "Hello Alice, you have 5 items");
    }

    #[test]
    fn test_placeholders() {
        let template = Template::new("{{a}} and {{b}} and {{a}} again");
        let placeholders = template.placeholders();

        assert_eq!(placeholders.len(), 2);
        assert!(placeholders.contains(&"a".to_string()));
        assert!(placeholders.contains(&"b".to_string()));
    }

    #[test]
    fn test_render_default() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = render_default(&mut rng, "Email: {{email}}");

        assert!(!result.contains("{{"));
        assert!(result.contains("@"));
    }

    #[test]
    fn test_no_placeholders() {
        let mut rng = StdRng::seed_from_u64(42);
        let registry = ProviderRegistry::with_defaults();

        let template = Template::new("No placeholders here");
        let result = template.render(&mut rng, &registry);

        assert_eq!(result, "No placeholders here");
    }

    #[test]
    fn test_whitespace_in_placeholder() {
        let mut rng = StdRng::seed_from_u64(42);
        let registry = ProviderRegistry::with_defaults();

        let template = Template::new("Hello {{ first_name }}!");
        let result = template.render(&mut rng, &registry);

        assert!(!result.contains("{{"));
    }

    #[test]
    fn test_registry_list() {
        let registry = ProviderRegistry::with_defaults();
        let list = registry.list();

        assert!(list.contains(&"first_name"));
        assert!(list.contains(&"email"));
        assert!(list.contains(&"uuid"));
    }
}
