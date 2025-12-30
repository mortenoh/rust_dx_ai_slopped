# markdown - Markdown Utilities

Render Markdown to HTML and extract table of contents.

**Alias:** `md`

## Subcommands

| Command | Description |
|---------|-------------|
| `render` | Render Markdown to HTML |
| `toc` | Extract table of contents |

## Usage

```bash
# Render Markdown to HTML
dx markdown render README.md
dx md render document.md > document.html

# Extract table of contents
dx markdown toc README.md
dx md toc document.md
```

## Examples

### Render to HTML

```bash
# Render a Markdown file
dx markdown render README.md

# Output:
# <h1>Title</h1>
# <p>This is a paragraph.</p>
# <ul>
# <li>Item 1</li>
# <li>Item 2</li>
# </ul>

# Save to file
dx md render README.md > README.html

# Render from stdin
echo "# Hello\n\nWorld" | dx md render -
```

### Extract TOC

```bash
# Extract table of contents
dx markdown toc README.md

# Output:
# - Title
#   - Section 1
#     - Subsection 1.1
#   - Section 2

# Get TOC as JSON
dx md toc README.md -o json
```

### Practical Uses

```bash
# Convert documentation
for f in docs/*.md; do
  dx md render "$f" > "${f%.md}.html"
done

# Generate navigation from TOC
dx md toc SUMMARY.md | grep "^-" | sed 's/^- //'
```

## Markdown Features

Supported Markdown syntax:

| Feature | Syntax |
|---------|--------|
| Headings | `# H1`, `## H2`, etc. |
| Bold | `**text**` |
| Italic | `*text*` |
| Links | `[text](url)` |
| Images | `![alt](url)` |
| Code | `` `code` `` or fenced blocks |
| Lists | `- item` or `1. item` |
| Blockquotes | `> quote` |
| Tables | `| a | b |` |
| Horizontal rules | `---` |

## Options

| Option | Description |
|--------|-------------|
| `--no-color` | Disable colored output |
| `-v, --verbose` | Enable verbose output |
| `-o, --output` | Output format (text, json, quiet) |

## See Also

- [text](./text.md) - Text transformations
- [template](./template.md) - Template rendering
