# mdbook Themes

Customize the look of your documentation.

## Theme Files

```bash
mdbook init --theme docs/
```

Creates:
```
docs/theme/
├── index.hbs        # Main HTML template
├── head.hbs         # Additional <head> content
├── header.hbs       # Page header
├── css/
│   ├── chrome.css   # UI elements
│   ├── general.css  # Base styles
│   ├── print.css    # Print styles
│   └── variables.css # CSS variables
├── book.js          # JavaScript
├── highlight.js     # Syntax highlighting
└── fonts/           # Custom fonts
```

## CSS Variables

Edit `theme/css/variables.css`:

```css
:root {
    --bg: #ffffff;
    --fg: #333333;
    --sidebar-bg: #fafafa;
    --sidebar-fg: #333333;
    --sidebar-active: #1f6feb;
    --links: #1f6feb;
    --inline-code-bg: #f6f8fa;
    --code-bg: #f6f8fa;
}

.rust {
    --bg: #fff8f6;
    --sidebar-bg: #fff1ed;
}

.ayu {
    --bg: #0f1419;
    --fg: #c5c5c5;
    --sidebar-bg: #14191f;
    --links: #39afd7;
}
```

## Custom Header

Edit `theme/header.hbs`:

```handlebars
<header>
    <nav>
        <a href="/" class="logo">dx</a>
        <a href="/guide">Guide</a>
        <a href="/api">API</a>
        <a href="https://github.com/user/dx">GitHub</a>
    </nav>
</header>
```

## Custom Head

Edit `theme/head.hbs`:

```html
<!-- Analytics -->
<script async src="https://analytics.example.com/script.js"></script>

<!-- Custom fonts -->
<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Inter&display=swap"
      rel="stylesheet">

<!-- Custom styles -->
<style>
    body {
        font-family: 'Inter', sans-serif;
    }
</style>
```

## Syntax Highlighting

### Custom Theme

Edit `theme/highlight.css`:

```css
.hljs-keyword { color: #d73a49; }
.hljs-string { color: #22863a; }
.hljs-comment { color: #6a737d; }
.hljs-function { color: #6f42c1; }
```

### Using a Preset

Download from highlight.js themes:

```bash
curl -o theme/highlight.css \
  https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/styles/github.min.css
```

## Additional CSS

Add custom CSS without modifying theme:

```toml
[output.html]
additional-css = ["custom.css"]
```

`docs/custom.css`:
```css
/* CLI command styling */
.command {
    background: #1e1e1e;
    padding: 1rem;
    border-radius: 4px;
    color: #d4d4d4;
}

/* Highlight important notes */
.important {
    border-left: 4px solid #f0ad4e;
    padding-left: 1rem;
    background: #fcf8e3;
}
```

## Additional JavaScript

```toml
[output.html]
additional-js = ["custom.js"]
```

`docs/custom.js`:
```javascript
// Copy button for code blocks
document.querySelectorAll('pre code').forEach(block => {
    const button = document.createElement('button');
    button.textContent = 'Copy';
    button.onclick = () => navigator.clipboard.writeText(block.textContent);
    block.parentNode.insertBefore(button, block);
});
```

## Favicon

```toml
[output.html]
site-url = "https://example.com/dx/"
```

Place `favicon.svg` or `favicon.png` in `docs/theme/`.
