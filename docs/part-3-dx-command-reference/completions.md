# completions - Shell Completions

Generate shell completion scripts for dx commands.

## Usage

```bash
dx completions <SHELL>
```

## Supported Shells

| Shell | Description |
|-------|-------------|
| `bash` | GNU Bourne-Again Shell |
| `zsh` | Z Shell |
| `fish` | Friendly Interactive Shell |
| `powershell` | PowerShell |
| `elvish` | Elvish Shell |

## Installation

### Bash

```bash
# Generate and install completions
dx completions bash > ~/.local/share/bash-completion/completions/dx

# Or add to .bashrc
dx completions bash >> ~/.bashrc
source ~/.bashrc
```

### Zsh

```bash
# Generate completion file
dx completions zsh > ~/.zfunc/_dx

# Add to .zshrc (if not already present)
echo 'fpath+=~/.zfunc' >> ~/.zshrc
echo 'autoload -Uz compinit && compinit' >> ~/.zshrc

# Reload
source ~/.zshrc
```

### Fish

```bash
# Generate and install completions
dx completions fish > ~/.config/fish/completions/dx.fish

# Reload fish
source ~/.config/fish/completions/dx.fish
```

### PowerShell

```powershell
# Add to PowerShell profile
dx completions powershell >> $PROFILE

# Reload profile
. $PROFILE
```

### Elvish

```bash
# Generate and install
dx completions elvish > ~/.elvish/lib/dx.elv

# Add to rc.elv
echo "use dx" >> ~/.elvish/rc.elv
```

## Examples

### Using Completions

After installation, use Tab to complete:

```bash
# Complete commands
dx ha<TAB>          # completes to: dx hash

# Complete subcommands
dx hash <TAB>       # shows: md5, sha256, sha512, bcrypt, argon2

# Complete options
dx hash --<TAB>     # shows: --algorithm, --file, --string, etc.

# Complete file arguments
dx hash file.<TAB>  # completes filenames
```

### System-wide Installation

```bash
# Linux (bash)
sudo dx completions bash > /etc/bash_completion.d/dx

# macOS (zsh with Homebrew)
dx completions zsh > $(brew --prefix)/share/zsh/site-functions/_dx
```

## Options

| Option | Description |
|--------|-------------|
| `--no-color` | Disable colored output |
| `-v, --verbose` | Enable verbose output |

## See Also

- [Clap Shell Completions](https://docs.rs/clap_complete/latest/clap_complete/) - Technical details
