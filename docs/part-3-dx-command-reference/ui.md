# ui - TUI Dashboard

The `ui` command launches an interactive terminal user interface (TUI) dashboard.

> **Note:** This command requires the `ui` feature flag.
> Build with: `cargo build --features ui`

## Usage

```bash
dx ui [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `-t`, `--tick` | Refresh rate in milliseconds (default: 250) |

## Examples

```bash
# Launch dashboard
dx ui

# Slower refresh rate
dx ui --tick 500

# Faster updates
dx ui --tick 100
```

## Dashboard Features

The TUI dashboard displays:

### Header
- dx version
- Current time

### System Info Panel
- CPU usage (gauge)
- Memory usage (gauge with total/used)
- System uptime

### Quick Stats Panel
- Hostname
- Operating system
- CPU count
- Process count

### Footer
- Keyboard shortcuts

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` | Quit |
| `Esc` | Quit |
| `r` | Force refresh |

## Building with UI Support

The `ui` feature includes `ratatui` and `crossterm` dependencies:

```bash
# Build with UI
cargo build --features ui

# Install with UI
cargo install --path . --features ui
```

Without the feature flag, the `ui` command won't be available.

## Technical Details

- Built with [ratatui](https://ratatui.rs/) for terminal rendering
- Uses [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal handling
- Polls system info via `sysinfo` crate
- Automatic cleanup on exit (restores terminal state)

## Screenshot

```
┌─────────────────────────────────────────────────────────────────┐
│  dx  Developer Experience CLI  v0.1.0  │ 2024-01-15 14:32:05   │
├─────────────────────────────────────────────────────────────────┤
│ System Info                     │ Quick Stats                   │
│                                 │                               │
│ CPU: ████████░░░░░░░░░░ 45.2%  │ Host:      MacBook-Pro        │
│                                 │ OS:        macOS 14.2.1       │
│ Memory: ██████████░░░░░ 62.1%  │ CPUs:      12                 │
│         (20.0 GB / 32.0 GB)    │                               │
│                                 │ Processes: 423                │
│ Uptime: 5d 12h 34m             │                               │
├─────────────────────────────────────────────────────────────────┤
│  q  Quit   r  Refresh   Esc  Exit                               │
└─────────────────────────────────────────────────────────────────┘
```
