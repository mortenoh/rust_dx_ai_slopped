# text

Transform text between different cases and formats.

## Usage

```bash
dx text <SUBCOMMAND> [TEXT]
```

Text can be provided as an argument or piped via stdin.

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `upper` | Convert to UPPERCASE |
| `lower` | Convert to lowercase |
| `title` | Convert to Title Case |
| `snake` | Convert to snake_case |
| `camel` | Convert to camelCase |
| `pascal` | Convert to PascalCase |
| `kebab` | Convert to kebab-case |
| `scream` | Convert to SCREAMING_SNAKE_CASE |
| `slug` | Create URL-safe slug |
| `reverse` | Reverse the text |
| `count` | Count characters, words, lines |
| `lorem` | Generate lorem ipsum text |
| `repeat` | Repeat text N times |
| `trim` | Trim whitespace |

---

## Case Conversions

All case conversion commands accept text as argument or stdin.

```bash
# As argument
dx text upper "hello world"

# From stdin
echo "hello world" | dx text upper
```

### upper

```bash
dx text upper "hello world"    # HELLO WORLD
```

### lower

```bash
dx text lower "HELLO WORLD"    # hello world
```

### title

```bash
dx text title "hello world"    # Hello World
```

### snake

```bash
dx text snake "HelloWorld"     # hello_world
dx text snake "hello world"    # hello_world
```

### camel

```bash
dx text camel "hello_world"    # helloWorld
dx text camel "Hello World"    # helloWorld
```

### pascal

```bash
dx text pascal "hello_world"   # HelloWorld
dx text pascal "hello world"   # HelloWorld
```

### kebab

```bash
dx text kebab "HelloWorld"     # hello-world
dx text kebab "hello_world"    # hello-world
```

### scream

```bash
dx text scream "helloWorld"    # HELLO_WORLD
dx text scream "hello-world"   # HELLO_WORLD
```

---

## slug

Create URL-safe slugs from text.

```bash
dx text slug <TEXT>
```

Converts to lowercase, replaces spaces with hyphens, removes special characters.

```bash
dx text slug "Hello World!"           # hello-world
dx text slug "My Blog Post #123"      # my-blog-post-123
dx text slug "What's New?"            # whats-new
```

---

## reverse

Reverse the characters in text.

```bash
dx text reverse <TEXT>
```

```bash
dx text reverse "hello"        # olleh
dx text reverse "12345"        # 54321
```

---

## count

Count characters, words, and lines.

```bash
dx text count <TEXT>
```

```bash
dx text count "hello world"
# chars: 11
# words: 2
# lines: 1

cat file.txt | dx text count
# chars: 1234
# words: 200
# lines: 50
```

---

## lorem

Generate lorem ipsum placeholder text.

```bash
dx text lorem [PARAGRAPHS] [OPTIONS]
```

| Argument | Default | Description |
|----------|---------|-------------|
| `PARAGRAPHS` | `1` | Number of paragraphs |
| `-w, --words` | | Generate specific word count instead |

```bash
dx text lorem                  # 1 paragraph
dx text lorem 3                # 3 paragraphs
dx text lorem --words 50       # Exactly 50 words
```

---

## repeat

Repeat text multiple times.

```bash
dx text repeat <TEXT> <TIMES> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `TEXT` | Text to repeat |
| `TIMES` | Number of repetitions |
| `-s, --separator` | Separator between repetitions (default: newline) |

```bash
dx text repeat "hello" 3
# hello
# hello
# hello

dx text repeat "ab" 5 -s ""    # ababababab
dx text repeat "x" 3 -s "-"    # x-x-x
```

---

## trim

Remove leading and trailing whitespace.

```bash
dx text trim <TEXT>
```

```bash
dx text trim "  hello  "       # "hello"
echo "  spaced  " | dx text trim
```

---

## Examples

```bash
# Convert variable naming conventions
dx text snake "getUserName"    # get_user_name
dx text camel "get_user_name"  # getUserName
dx text pascal "get_user_name" # GetUserName

# Create URL slugs for blog posts
dx text slug "My Amazing Blog Post!"  # my-amazing-blog-post

# Count words in a file
cat essay.txt | dx text count

# Generate placeholder content
dx text lorem 5 > placeholder.txt

# Create separator lines
dx text repeat "=" 80 -s ""    # 80 equals signs

# Pipeline with other commands
cat file.txt | dx text lower | dx text trim
```
