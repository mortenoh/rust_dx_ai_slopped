//! Hacker/developer-related data generators.
//!
//! Provides generators for developer-related fake data including programming
//! languages, frameworks, hacker phrases, and git operations.

use rand::Rng;

fn pick<R: ?Sized + Rng>(rng: &mut R, items: &[&'static str]) -> &'static str {
    items[rng.random_range(0..items.len())]
}

/// Programming languages.
static PROGRAMMING_LANGUAGES: &[&str] = &[
    "Rust",
    "Python",
    "JavaScript",
    "TypeScript",
    "Go",
    "Java",
    "C++",
    "C#",
    "Ruby",
    "Swift",
    "Kotlin",
    "Scala",
    "PHP",
    "Perl",
    "Haskell",
    "Elixir",
    "Clojure",
    "F#",
    "Dart",
    "Lua",
    "R",
    "Julia",
    "Zig",
    "Nim",
    "Crystal",
];

/// Frameworks and libraries.
static FRAMEWORKS: &[&str] = &[
    "React",
    "Vue",
    "Angular",
    "Svelte",
    "Next.js",
    "Django",
    "Flask",
    "FastAPI",
    "Rails",
    "Spring Boot",
    "Express",
    "NestJS",
    "Laravel",
    "ASP.NET",
    "Actix",
    "Axum",
    "Rocket",
    "Phoenix",
    "Gin",
    "Echo",
    "TensorFlow",
    "PyTorch",
    "Pandas",
    "NumPy",
    "Tokio",
];

/// Databases.
static DATABASES: &[&str] = &[
    "PostgreSQL",
    "MySQL",
    "SQLite",
    "MongoDB",
    "Redis",
    "Elasticsearch",
    "Cassandra",
    "DynamoDB",
    "CouchDB",
    "Neo4j",
    "InfluxDB",
    "TimescaleDB",
    "ClickHouse",
    "ScyllaDB",
    "CockroachDB",
    "MariaDB",
    "Oracle",
    "SQL Server",
    "Firebase",
    "Supabase",
];

/// Cloud providers.
static CLOUD_PROVIDERS: &[&str] = &[
    "AWS",
    "Google Cloud",
    "Azure",
    "DigitalOcean",
    "Linode",
    "Vultr",
    "Heroku",
    "Vercel",
    "Netlify",
    "Cloudflare",
    "Fly.io",
    "Railway",
    "Render",
    "PlanetScale",
    "Neon",
];

/// DevOps tools.
static DEVOPS_TOOLS: &[&str] = &[
    "Docker",
    "Kubernetes",
    "Terraform",
    "Ansible",
    "Jenkins",
    "GitHub Actions",
    "GitLab CI",
    "CircleCI",
    "Travis CI",
    "ArgoCD",
    "Helm",
    "Prometheus",
    "Grafana",
    "Datadog",
    "New Relic",
    "PagerDuty",
    "Sentry",
    "Nginx",
    "Apache",
    "Caddy",
];

/// Hacker verbs.
static HACKER_VERBS: &[&str] = &[
    "parse",
    "compile",
    "deploy",
    "refactor",
    "optimize",
    "debug",
    "encrypt",
    "hash",
    "serialize",
    "deserialize",
    "cache",
    "index",
    "query",
    "traverse",
    "transform",
    "validate",
    "sanitize",
    "authenticate",
    "authorize",
    "benchmark",
];

/// Hacker nouns.
static HACKER_NOUNS: &[&str] = &[
    "interface",
    "protocol",
    "algorithm",
    "buffer",
    "cache",
    "stack",
    "heap",
    "queue",
    "array",
    "hash map",
    "binary tree",
    "graph",
    "socket",
    "thread",
    "process",
    "mutex",
    "semaphore",
    "pipeline",
    "middleware",
    "microservice",
];

/// Hacker adjectives.
static HACKER_ADJECTIVES: &[&str] = &[
    "distributed",
    "concurrent",
    "asynchronous",
    "parallel",
    "reactive",
    "functional",
    "immutable",
    "stateless",
    "idempotent",
    "polymorphic",
    "recursive",
    "iterative",
    "virtual",
    "abstract",
    "generic",
    "modular",
    "scalable",
    "fault-tolerant",
    "high-performance",
    "memory-safe",
];

/// Hacker abbreviations.
static HACKER_ABBREVIATIONS: &[&str] = &[
    "API", "SDK", "CLI", "GUI", "ORM", "REST", "gRPC", "JSON", "XML", "YAML", "JWT", "OAuth",
    "SSL", "TLS", "SSH", "DNS", "CDN", "CI/CD", "CRUD", "DRY", "SOLID", "MVC", "OOP", "FP", "TDD",
];

/// Git branch prefixes.
static GIT_BRANCH_PREFIXES: &[&str] = &[
    "feature", "fix", "hotfix", "bugfix", "release", "chore", "refactor", "docs", "test", "ci",
];

/// Git commit types.
static GIT_COMMIT_TYPES: &[&str] = &[
    "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "revert",
];

/// Generate a random programming language.
pub fn programming_language<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, PROGRAMMING_LANGUAGES)
}

/// Generate a random framework/library name.
pub fn framework<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, FRAMEWORKS)
}

/// Generate a random database name.
pub fn database<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, DATABASES)
}

/// Generate a random cloud provider.
pub fn cloud_provider<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, CLOUD_PROVIDERS)
}

/// Generate a random DevOps tool.
pub fn devops_tool<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, DEVOPS_TOOLS)
}

/// Generate a random hacker verb.
pub fn hacker_verb<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, HACKER_VERBS)
}

/// Generate a random hacker noun.
pub fn hacker_noun<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, HACKER_NOUNS)
}

/// Generate a random hacker adjective.
pub fn hacker_adjective<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, HACKER_ADJECTIVES)
}

/// Generate a random hacker abbreviation.
pub fn hacker_abbreviation<R: ?Sized + Rng>(rng: &mut R) -> &'static str {
    pick(rng, HACKER_ABBREVIATIONS)
}

/// Generate a random hacker phrase.
pub fn hacker_phrase<R: ?Sized + Rng>(rng: &mut R) -> String {
    let patterns = [
        "We need to {verb} the {adj} {noun}!",
        "Try to {verb} the {abbr} {noun} through the {adj} {noun}.",
        "Use the {adj} {abbr} {noun}, then you can {verb} the {noun}.",
        "The {abbr} {noun} is down, {verb} the {adj} {noun} so we can {verb} the {noun}!",
        "I'll {verb} the {adj} {abbr} {noun}, that should {verb} the {noun}!",
    ];
    let pattern = pick(rng, &patterns);
    pattern
        .replace("{verb}", pick(rng, HACKER_VERBS))
        .replace("{noun}", pick(rng, HACKER_NOUNS))
        .replace("{adj}", pick(rng, HACKER_ADJECTIVES))
        .replace("{abbr}", pick(rng, HACKER_ABBREVIATIONS))
}

/// Generate a random git branch name.
pub fn git_branch<R: ?Sized + Rng>(rng: &mut R) -> String {
    let prefix = pick(rng, GIT_BRANCH_PREFIXES);
    let topics = [
        "add-user-auth",
        "fix-login-bug",
        "update-deps",
        "refactor-api",
        "add-tests",
        "improve-performance",
        "fix-memory-leak",
        "add-caching",
        "update-docs",
        "fix-typo",
        "add-logging",
        "fix-security-issue",
        "add-validation",
        "refactor-database",
        "add-ci-pipeline",
    ];
    let topic = pick(rng, &topics);
    format!("{}/{}", prefix, topic)
}

/// Generate a random git commit message.
pub fn git_commit_message<R: ?Sized + Rng>(rng: &mut R) -> String {
    let commit_type = pick(rng, GIT_COMMIT_TYPES);
    let scopes = [
        "api", "auth", "db", "ui", "core", "utils", "config", "deps", "tests", "docs",
    ];
    let messages = [
        "add new endpoint for user management",
        "fix null pointer exception",
        "update dependency versions",
        "improve error handling",
        "refactor database queries",
        "add unit tests",
        "fix memory leak in cache",
        "implement caching layer",
        "update README",
        "fix typo in error message",
        "add input validation",
        "fix security vulnerability",
        "improve logging",
        "refactor authentication flow",
        "add CI/CD pipeline",
    ];

    let scope = pick(rng, &scopes);
    let message = pick(rng, &messages);
    format!("{}({}): {}", commit_type, scope, message)
}

/// Generate a random git SHA (abbreviated).
pub fn git_sha<R: ?Sized + Rng>(rng: &mut R) -> String {
    let chars: Vec<char> = "0123456789abcdef".chars().collect();
    (0..7)
        .map(|_| chars[rng.random_range(0..chars.len())])
        .collect()
}

/// Generate a random git SHA (full 40 characters).
pub fn git_sha_full<R: ?Sized + Rng>(rng: &mut R) -> String {
    let chars: Vec<char> = "0123456789abcdef".chars().collect();
    (0..40)
        .map(|_| chars[rng.random_range(0..chars.len())])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_programming_language() {
        let mut rng = StdRng::seed_from_u64(42);
        let lang = programming_language(&mut rng);
        assert!(PROGRAMMING_LANGUAGES.contains(&lang));
    }

    #[test]
    fn test_framework() {
        let mut rng = StdRng::seed_from_u64(42);
        let fw = framework(&mut rng);
        assert!(FRAMEWORKS.contains(&fw));
    }

    #[test]
    fn test_hacker_phrase() {
        let mut rng = StdRng::seed_from_u64(42);
        let phrase = hacker_phrase(&mut rng);
        assert!(!phrase.is_empty());
        assert!(!phrase.contains('{'));
    }

    #[test]
    fn test_git_branch() {
        let mut rng = StdRng::seed_from_u64(42);
        let branch = git_branch(&mut rng);
        assert!(branch.contains('/'));
    }

    #[test]
    fn test_git_commit_message() {
        let mut rng = StdRng::seed_from_u64(42);
        let msg = git_commit_message(&mut rng);
        assert!(msg.contains("("));
        assert!(msg.contains("):"));
    }

    #[test]
    fn test_git_sha() {
        let mut rng = StdRng::seed_from_u64(42);
        let sha = git_sha(&mut rng);
        assert_eq!(sha.len(), 7);
        assert!(sha.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_git_sha_full() {
        let mut rng = StdRng::seed_from_u64(42);
        let sha = git_sha_full(&mut rng);
        assert_eq!(sha.len(), 40);
        assert!(sha.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
