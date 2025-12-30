# template - Template Rendering

Render Jinja2-style templates using the Tera templating engine.

## Subcommands

| Command | Description |
|---------|-------------|
| `render` | Render a template with data |
| `validate` | Validate template syntax |

## Usage

```bash
# Render template with data file
dx template render template.tera --data data.json

# Render with inline data
dx template render template.tera --set name=Alice --set count=42

# Validate template syntax
dx template validate template.tera
```

## Examples

### Basic Rendering

```bash
# Create a template
echo "Hello, {{ name }}!" > greeting.tera

# Render with data
dx template render greeting.tera --set name=World
# Output: Hello, World!
```

### Using Data Files

```bash
# data.json
# {"user": {"name": "Alice", "age": 30}, "items": ["a", "b", "c"]}

# template.tera
# User: {{ user.name }} ({{ user.age }})
# Items: {% for item in items %}{{ item }}{% if not loop.last %}, {% endif %}{% endfor %}

dx template render template.tera --data data.json
# Output:
# User: Alice (30)
# Items: a, b, c
```

### Template Features

```bash
# Conditionals
echo "{% if count > 0 %}Has items{% else %}Empty{% endif %}" > cond.tera
dx template render cond.tera --set count=5

# Loops
echo "{% for i in range(end=3) %}Item {{ i }}\n{% endfor %}" > loop.tera
dx template render loop.tera

# Filters
echo "{{ name | upper }}" > filter.tera
dx template render filter.tera --set name=hello
# Output: HELLO
```

### Validate Syntax

```bash
# Check template for errors
dx template validate template.tera
# Output: Template is valid

# Check broken template
dx template validate broken.tera
# Output: Error: unexpected token...
```

## Template Syntax

Tera uses Jinja2-style syntax:

| Syntax | Description |
|--------|-------------|
| `{{ var }}` | Variable output |
| `{% if %}` | Conditionals |
| `{% for %}` | Loops |
| `{% block %}` | Template blocks |
| `{% include %}` | Include templates |
| `{% macro %}` | Reusable macros |
| `{# comment #}` | Comments |

### Built-in Filters

| Filter | Description |
|--------|-------------|
| `upper` | Uppercase |
| `lower` | Lowercase |
| `capitalize` | Capitalize first letter |
| `trim` | Remove whitespace |
| `length` | Get length |
| `default(val)` | Default value |
| `join(sep)` | Join array |
| `first` / `last` | First/last element |

## Options

| Option | Description |
|--------|-------------|
| `--data` | JSON data file |
| `--set` | Set individual values (key=value) |
| `--no-color` | Disable colored output |
| `-v, --verbose` | Enable verbose output |
| `-o, --output` | Output format (text, json, quiet) |

## See Also

- [json](./json.md) - JSON processing
- [text](./text.md) - Text transformations
- [Tera Documentation](https://keats.github.io/tera/) - Full template syntax
