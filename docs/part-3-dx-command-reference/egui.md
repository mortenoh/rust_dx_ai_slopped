# egui - Native GUI Demos

The `egui` command provides native GUI application demos using the [egui](https://github.com/emilk/egui) immediate mode GUI library. These tools showcase the full range of egui widgets and serve as practical utilities.

> **Note:** This command requires the `egui` feature flag: `cargo build --features egui`

## Usage

```bash
dx egui <SUBCOMMAND>
```

## All Subcommands (24 total)

| Subcommand | Description | Key Widgets |
|------------|-------------|-------------|
| `demo` | Basic hello world window | TextEdit, Hyperlink |
| `counter` | Counter with increment/decrement buttons | Button, Label |
| `clock` | Live updating clock display | RichText |
| **Generators** | | |
| `uuid` | UUID generator with format selection | RadioButton, ComboBox, Slider, ScrollArea, Grid |
| `password` | Password generator with strength indicator | Slider, Checkbox, ProgressBar |
| `qrcode` | QR code generator with live preview | TextEdit, Checkbox, custom painting |
| `lorem` | Lorem ipsum text generator | RadioButton, DragValue, ScrollArea |
| `color` | Color picker with HEX/RGB/HSL conversion | color_picker, Grid |
| **Encoders/Decoders** | | |
| `hash` | Hash calculator (MD5, SHA256, SHA512) | TextEdit, Grid, Checkbox, RichText |
| `base64` | Base64 encoder/decoder | Columns, TextEdit, Checkbox |
| `hex` | Hex encoder/decoder with ASCII view | Grid, ScrollArea |
| `url` | URL encoder/decoder with breakdown | CollapsingHeader, Hyperlink, Grid |
| **Converters** | | |
| `timestamp` | Timestamp converter with multiple formats | DragValue, Button, Grid |
| `units` | Unit converter (bytes, time) | RadioButton, DragValue, Grid |
| `base` | Number base converter (bin/oct/dec/hex) | TextEdit, custom bit visualization |
| `json` | JSON formatter and validator | TopBottomPanel, Slider, Checkbox, Columns |
| **Utilities** | | |
| `regex` | Regex pattern tester with highlighting | ComboBox, Checkbox, Grid, RichText |
| `diff` | Text diff viewer | Columns, TextEdit, RichText colored |
| `stopwatch` | Stopwatch with lap times | Button, RichText, ScrollArea |
| `calculator` | Expression calculator | Button grid, CollapsingHeader, ScrollArea |
| **Text Tools** | | |
| `case` | Case converter (10 formats) | TextEdit, Grid |
| `text-stats` | Text statistics (chars, words, lines) | TextEdit, Grid, ScrollArea |
| `markdown` | Markdown preview editor | TopBottomPanel, Columns, ScrollArea |
| `timer` | Pomodoro timer | ProgressBar, DragValue, Checkbox |

---

## Generators

### uuid

Generate UUIDs with format selection and bulk generation.

```bash
dx egui uuid
```

**Features:**
- v4 (random) and v7 (time-based) UUID selection
- Format options: standard, simple, URN, braced
- Bulk generation (1-50 UUIDs)
- Copy buttons for each UUID

### password

Generate secure passwords with strength indicator.

```bash
dx egui password
```

**Features:**
- Adjustable length (8-128 characters)
- Character set toggles (uppercase, lowercase, digits, symbols)
- Visual strength indicator with entropy calculation
- Generate multiple passwords at once

### qrcode

Generate QR codes with live preview.

```bash
dx egui qrcode
```

**Features:**
- Multi-line text input
- Live QR code rendering
- Invert colors toggle
- Adjustable scale

### lorem

Generate Lorem Ipsum placeholder text.

```bash
dx egui lorem
```

**Features:**
- Generate words, sentences, or paragraphs
- Adjustable count
- Copy button

### color

Color picker with format conversion.

```bash
dx egui color
```

**Features:**
- Visual color picker
- HEX, RGB, and HSL format display
- Copy buttons for each format
- Manual hex input

---

## Encoders/Decoders

### hash

Calculate hashes for text input.

```bash
dx egui hash
```

**Features:**
- Shows MD5, SHA-256, and SHA-512 simultaneously
- Compare mode (paste expected hash to verify)
- Copy buttons

### base64

Encode and decode Base64.

```bash
dx egui base64
```

**Features:**
- Split-pane layout
- URL-safe encoding toggle
- Real-time conversion
- Error display

### hex

Encode and decode hexadecimal.

```bash
dx egui hex
```

**Features:**
- Hex output with byte view
- ASCII sidebar display
- Hex-to-text decoding

### url

URL encoding and breakdown.

```bash
dx egui url
```

**Features:**
- URL encode/decode
- Collapsible URL breakdown (scheme, host, path, port)
- Query parameter grid
- Clickable hyperlink to test URL

---

## Converters

### timestamp

Convert Unix timestamps to various formats.

```bash
dx egui timestamp
```

**Features:**
- Unix timestamp input with drag value
- "Now" button
- Milliseconds toggle
- Shows: UTC, Local, ISO 8601, RFC 2822, date, time, day of week, relative time

### units

Convert between units.

```bash
dx egui units
```

**Features:**
- Bytes: B, KB, MB, GB, TB (with human-readable output)
- Time: seconds, minutes, hours, days, weeks (with humantime)

### base

Convert between number bases.

```bash
dx egui base
```

**Features:**
- Decimal, Binary, Octal, Hexadecimal
- Live bidirectional conversion
- Visual bit display with color coding

### json

Format and validate JSON.

```bash
dx egui json
```

**Features:**
- Format with custom indentation (1-8 spaces)
- Minify option
- Sort keys toggle
- Error display
- Split-pane input/output

---

## Utilities

### regex

Test regular expressions.

```bash
dx egui regex
```

**Features:**
- Pattern input with preset patterns (Email, URL, Phone, IPv4, Date)
- Case-insensitive and multiline flags
- Matches list with positions
- Capture group display

### diff

Compare two texts.

```bash
dx egui diff
```

**Features:**
- Side-by-side text input
- Colored diff output (red for removed, green for added)

### stopwatch

Stopwatch with lap functionality.

```bash
dx egui stopwatch
```

**Features:**
- Large time display (HH:MM:SS.mmm)
- Start/Stop/Reset/Lap buttons
- Lap time list

### calculator

Expression calculator using dx's expr evaluator.

```bash
dx egui calculator
```

**Features:**
- Calculator button grid
- Expression input
- Calculation history
- Collapsible function reference (sin, cos, sqrt, etc.)
- Uses existing `expr` library

---

## Text Tools

### case

Convert text between case formats.

```bash
dx egui case
```

**Formats:**
- UPPERCASE
- lowercase
- Title Case
- camelCase
- PascalCase
- snake_case
- SCREAMING_SNAKE_CASE
- kebab-case
- SCREAMING-KEBAB-CASE
- Train-Case

### text-stats

Analyze text statistics.

```bash
dx egui text-stats
```

**Statistics:**
- Characters (with and without spaces)
- Words
- Sentences
- Lines
- Paragraphs
- Reading time estimate
- Top words frequency

### markdown

Simple markdown editor with live preview.

```bash
dx egui markdown
```

**Features:**
- Split-pane editor/preview
- Toolbar buttons (Bold, Italic, Code, H1, H2, List, Quote)
- Basic markdown rendering (headings, lists, blockquotes, bold, italic, code)

### timer

Pomodoro timer for productivity.

```bash
dx egui timer
```

**Features:**
- Configurable work/break durations
- Visual progress bar
- Auto-start break option
- Session counter
- Start/Pause/Reset/Skip controls

---

## Building with egui

The egui feature is optional and adds the `eframe` dependency.

### Build Commands

```bash
# Build with egui support
cargo build --features egui

# Build with all features (ui + egui)
cargo build --features ui,egui

# Install with egui
cargo install --path . --features egui

# Run any egui tool
cargo run --features egui -- egui uuid
cargo run --features egui -- egui password
cargo run --features egui -- egui calculator
```

### Cross-Platform Support

egui/eframe applications work on:
- **Linux** - X11 and Wayland
- **macOS** - Native Cocoa
- **Windows** - Native Win32

---

## Widget Showcase

These 24 tools showcase the following egui widgets:

| Widget | Used In |
|--------|---------|
| Button | All tools |
| Label | All tools |
| TextEdit | hash, base64, hex, json, regex, diff, case, text-stats, markdown, qrcode, url |
| Slider | uuid, password, qrcode, json |
| DragValue | lorem, timestamp, units, stopwatch, timer |
| Checkbox | password, hash, base64, regex, json, timer |
| RadioButton | uuid, lorem, units |
| ComboBox | uuid, timestamp, regex |
| ProgressBar | password, stopwatch, timer |
| Grid | uuid, password, color, hash, hex, url, timestamp, units, base, regex, calculator, case, text-stats |
| ScrollArea | uuid, lorem, hex, json, regex, stopwatch, calculator, text-stats, markdown |
| CollapsingHeader | url, calculator |
| Columns | base64, diff, markdown |
| TopBottomPanel | json, markdown |
| RichText | All tools (styling: bold, size, color, monospace) |
| color_picker | color |
| Hyperlink | url, demo |
| Separator | All tools |
| Frame | color, qrcode |
| Custom painting | qrcode, base |

---

## Technical Details

### Source Files

- CLI arguments: `src/cli/commands/egui.rs`
- Implementation: `src/commands/egui.rs`

### Feature Gate

The command is conditionally compiled:

```rust
#[cfg(feature = "egui")]
pub mod egui;
```

This ensures the egui dependencies are only included when explicitly requested.
