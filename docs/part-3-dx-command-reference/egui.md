# egui - Native GUI Demos

The `egui` command provides native GUI application demos using the [egui](https://github.com/emilk/egui) immediate mode GUI library.

> **Note:** This command requires the `egui` feature flag: `cargo build --features egui`

## Usage

```bash
dx egui <SUBCOMMAND>
```

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `demo` | Basic hello world window |
| `counter` | Counter with increment/decrement buttons |
| `clock` | Live updating clock display |

---

## demo

Display a simple hello world GUI window.

```bash
dx egui demo
```

Opens a native window with a "Hello, World!" message demonstrating the basic egui/eframe setup.

### Features
- Native window with title bar
- Basic text rendering
- Window close button

---

## counter

Interactive counter application with buttons.

```bash
dx egui counter
```

Opens a window with:
- Current count display
- Increment (+) button
- Decrement (-) button

### Features
- State management
- Button interactions
- Real-time UI updates

---

## clock

Live updating clock display.

```bash
dx egui clock
```

Opens a window showing the current time that updates every second.

### Features
- Real-time updates
- Continuous repainting
- Time formatting

---

## Building with egui

The egui feature is optional and adds the following dependencies:
- `eframe` - egui framework for native apps
- `egui` - Immediate mode GUI library

### Build Commands

```bash
# Build with egui support
cargo build --features egui

# Build with all features (ui + egui)
cargo build --features ui,egui

# Install with egui
cargo install --path . --features egui
```

### Cross-Platform Support

egui/eframe applications work on:
- **Linux** - X11 and Wayland
- **macOS** - Native Cocoa
- **Windows** - Native Win32

> **Note:** Cross-compiling GUI applications requires platform-specific considerations. See the [Cross-Platform Testing Environments](../appendices/j-cross-platform-testing-environments.md) guide for details on testing GUI apps in VMs.

---

## Examples

### Quick Demo

```bash
# Show hello world window
dx egui demo

# Interactive counter
dx egui counter

# Live clock
dx egui clock
```

### From Makefile

```bash
# Build with all features including egui
make build    # Uses FEATURES=ui,egui by default

# Run egui demo
cargo run --features egui -- egui demo
```

---

## Technical Details

### Implementation

The egui command uses:
- **eframe** for window management and event loop
- **egui** for immediate mode UI rendering
- **glow** backend for OpenGL rendering (via egui_glow)

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
