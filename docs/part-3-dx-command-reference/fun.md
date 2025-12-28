# fun - Fun Terminal Effects

The `fun` command provides entertaining terminal animations and effects.

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `progress` | Fake progress bar with funny messages |
| `hacker` | Fake hacker terminal output |
| `countdown` | Countdown timer with visual effects |
| `spinners` | Showcase all available spinner styles |
| `work` | Simulate fake work with progress bars |
| `fortune` | Random programming wisdom with ASCII art |
| `bounce` | Bouncing indeterminate progress bar |
| `clock` | Big ASCII digital clock |
| `qr` | Generate QR code in terminal |
| `life` | Conway's Game of Life simulation |
| `matrix` | Matrix-style falling code rain |
| `banner` | Big ASCII text banner |

## Examples

### Progress Bar
```bash
dx fun progress              # 10 second fake progress
dx fun progress -d 30        # 30 second duration
dx fun progress -s bar       # Bar only (no spinner)
```

### Hacker Mode
```bash
dx fun hacker                # 15 seconds of "hacking"
dx fun hacker -d 60 -i 3     # 60 seconds, fast intensity
```

### Countdown
```bash
dx fun countdown 10          # 10 second countdown
dx fun countdown 60 -m "Time's up!"
dx fun countdown 5 --simple  # No box art
```

### Spinners Showcase
```bash
dx fun spinners              # Show all spinner styles
dx fun spinners -n dots      # Show specific spinner
dx fun spinners -d 5         # 5 seconds per spinner
```

### Fake Work
```bash
dx fun work                  # Default 30s, 8 tasks
dx fun work -d 60 -t 12      # 60 seconds, 12 tasks
dx fun work -s gradient      # Use gradient progress style
dx fun work --list-styles    # Show available styles
```

Progress bar styles: `block`, `gradient`, `arrow`, `dots`, `emoji`, `classic`

### Fortune
```bash
dx fun fortune               # Random fortune with random animal
dx fun fortune -a cow        # Use cow ASCII art
dx fun fortune -s "Hello!"   # Custom message
dx fun fortune -l            # List available animals
```

Animals: `cow`, `tux`, `ghost`, `dragon`, `cat`, `dog`

### Bouncing Progress
```bash
dx fun bounce                # 5 second bounce
dx fun bounce -d 10 -m "Loading..."
```

### ASCII Clock
```bash
dx fun clock                 # Run until Ctrl+C
dx fun clock -d 60           # Run for 60 seconds
dx fun clock --twelve-hour   # 12-hour format
```

Press `q` or `Esc` to exit.

### QR Code
```bash
dx fun qr "https://github.com"
dx fun qr "Hello World" --invert
```

### Game of Life
```bash
dx fun life                  # Random start pattern
dx fun life -p glider        # Start with glider
dx fun life -p pulsar        # Start with pulsar
dx fun life --width 80 --height 30
```

Patterns: `random`, `glider`, `blinker`, `pulsar`

Press `q` or `Esc` to exit.

### Matrix Rain
```bash
dx fun matrix                # Run until Ctrl+C
dx fun matrix -d 30          # Run for 30 seconds
dx fun matrix --density 8    # More columns (1-10)
```

Press `q` or `Esc` to exit.

### ASCII Banner
```bash
dx fun banner "HELLO"
dx fun banner "OK"
dx fun banner "DX CLI"
```

## OSC 9;4 Integration

Many `fun` animations integrate with terminal progress reporting via OSC 9;4 escape sequences. Supported terminals (Ghostty, Windows Terminal, iTerm2) will show native progress indicators in the title bar or tab.
