# dx-progress: Terminal Progress Library

`dx-progress` is a lightweight, zero-dependency terminal progress library with native integration for modern terminals.

## Overview

This crate provides progress reporting that integrates with modern terminals via **OSC 9;4 escape sequences**, which are supported by:

- **Ghostty** (1.2+) - Native progress bar in tab/title bar
- **Windows Terminal** - Progress indicator in taskbar
- **ConEmu** - Progress bar in title
- **iTerm2** - Partial support

## Why Use OSC 9;4?

Traditional progress bars like `indicatif` work great, but they only show progress *inside* the terminal window. With OSC 9;4:

- Progress shows in the **terminal tab** or **title bar**
- Works even when the terminal is **minimized**
- Integrates with system **taskbar** (Windows)
- No terminal detection issues - just works

## Quick Start

### Basic Progress Bar

```rust
use dx_progress::{TerminalProgress, ProgressState};

fn main() {
    let mut progress = TerminalProgress::new(100);

    for i in 0..=100 {
        progress.set(i);
        progress.draw(Some("Processing files..."));
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    progress.finish_with_message("✓ Complete!");
}
```

### Low-Level API

For more control, use the low-level functions directly:

```rust
use dx_progress::{osc_progress, osc_progress_clear, ProgressState};

// Report 50% progress
osc_progress(50, ProgressState::Normal);

// Show warning state (yellow in most terminals)
osc_progress(75, ProgressState::Warning);

// Show error state (red)
osc_progress(50, ProgressState::Error);

// Indeterminate/pulsing progress
osc_progress(0, ProgressState::Indeterminate);

// Clear when done (important!)
osc_progress_clear();
```

### Spinner Animation

```rust
use dx_progress::{draw_spinner, SPINNER_FRAMES};

for frame in 0..100 {
    draw_spinner(frame, "Loading...");
    std::thread::sleep(std::time::Duration::from_millis(80));
}
eprintln!("\r\x1b[K✓ Done!");
```

## API Reference

### `ProgressState`

```rust
pub enum ProgressState {
    Hidden = 0,       // Clear progress indicator
    Normal = 1,       // Normal progress (default, usually blue/green)
    Error = 2,        // Error state (red)
    Indeterminate = 3, // Unknown duration (pulsing)
    Warning = 4,      // Warning state (yellow)
}
```

### `TerminalProgress`

High-level progress reporter with automatic cleanup.

```rust
let mut p = TerminalProgress::new(100)
    .show_bar(true)      // Show visual progress bar (default: true)
    .bar_width(30);      // Width of visual bar (default: 30)

p.set(50);               // Set to 50%
p.inc(10);               // Increment by 10
p.set_state(ProgressState::Warning);
p.draw(Some("msg"));     // Draw visual bar to stderr
p.finish();              // Clear and cleanup
```

### Functions

| Function | Description |
|----------|-------------|
| `osc_progress(percent, state)` | Report progress to terminal |
| `osc_progress_clear()` | Clear terminal progress indicator |
| `draw_spinner(frame, msg)` | Draw spinner frame to stderr |
| `clear_line()` | Clear current line on stderr |

### Constants

| Constant | Description |
|----------|-------------|
| `SPINNER_FRAMES` | Braille spinner animation frames |

## The OSC 9;4 Protocol

OSC (Operating System Command) sequences let applications communicate with the terminal emulator. The format is:

```
ESC ] 9 ; 4 ; <state> ; <percent> ST
```

Where:
- `ESC ]` = `\x1b]` (OSC introducer)
- `ST` = `\x1b\\` (String Terminator) or `\x07` (BEL)
- `<state>` = 0-4 (see ProgressState)
- `<percent>` = 0-100

### Important Notes

1. **Use stdout, not stderr** - Terminals typically only parse OSC on stdout
2. **Use ESC \\ terminator** - More compatible than BEL (`\x07`)
3. **Always clear when done** - Otherwise progress indicator may get stuck

## Integration with dx CLI

The `dx fun` command uses this library:

```bash
# Fake progress bar with terminal integration
dx fun progress --duration 10

# Countdown timer
dx fun countdown 30

# Spinner showcase
dx fun spinners
```

## Comparison with indicatif

| Feature | dx-progress | indicatif |
|---------|-------------|-----------|
| Dependencies | 0 | Many |
| Terminal detection | Not needed | Required |
| Native terminal progress | ✓ OSC 9;4 | ✗ |
| Visual progress bars | ✓ | ✓ |
| Multi-progress | ✗ | ✓ |
| Download-style bars | ✗ | ✓ |

Use `dx-progress` when you want:
- Zero dependencies
- Native terminal integration
- Simple, focused API

Use `indicatif` when you need:
- Complex progress layouts
- Multiple concurrent progress bars
- Download-style formatting (bytes/sec, ETA)

## Troubleshooting

### Progress not showing in terminal

1. **Check terminal support** - Not all terminals support OSC 9;4
2. **Use stdout** - OSC sequences must go to stdout, not stderr
3. **Flush output** - Always flush after writing OSC sequences

### Progress stuck after program exits

The `TerminalProgress` struct automatically clears on drop. If using the low-level API, always call `osc_progress_clear()` when done.

```rust
// Good - automatic cleanup
{
    let mut p = TerminalProgress::new(100);
    p.set(50);
} // Cleared on drop

// Low-level - manual cleanup required
osc_progress(50, ProgressState::Normal);
// ... do work ...
osc_progress_clear(); // Don't forget this!
```
