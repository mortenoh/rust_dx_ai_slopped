# dx-progress

A lightweight, zero-dependency terminal progress library with native integration for modern terminals via **OSC 9;4 escape sequences**.

## Supported Terminals

- **Ghostty** (1.2+) - Native progress bar in tab/title bar
- **Windows Terminal** - Progress indicator in taskbar
- **ConEmu** - Progress bar in title
- **iTerm2** - Partial support

## Why OSC 9;4?

Traditional progress bars only show inside the terminal window. With OSC 9;4:
- Progress shows in the **terminal tab** or **title bar**
- Works even when the terminal is **minimized**
- Integrates with system **taskbar** (Windows)

## Quick Start

```rust
use dx_progress::{TerminalProgress, ProgressState};

fn main() {
    let mut progress = TerminalProgress::new(100);

    for i in 0..=100 {
        progress.set(i);
        progress.draw(Some("Processing..."));
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    progress.finish_with_message("Done!");
}
```

## Low-Level API

```rust
use dx_progress::{osc_progress, osc_progress_clear, ProgressState};

// Report 50% progress
osc_progress(50, ProgressState::Normal);

// Show warning state (yellow)
osc_progress(75, ProgressState::Warning);

// Show error state (red)
osc_progress(50, ProgressState::Error);

// Clear when done (important!)
osc_progress_clear();
```

## Progress States

```rust
pub enum ProgressState {
    Hidden = 0,       // Clear progress indicator
    Normal = 1,       // Normal progress (blue/green)
    Error = 2,        // Error state (red)
    Indeterminate = 3, // Unknown duration (pulsing)
    Warning = 4,      // Warning state (yellow)
}
```

## License

MIT
