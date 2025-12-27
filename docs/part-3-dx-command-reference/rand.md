# rand

Generate random numbers, strings, passwords, and make random choices.

## Usage

```bash
dx rand <SUBCOMMAND>
```

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `int` | Generate random integer |
| `float` | Generate random float |
| `string` | Generate random alphanumeric string |
| `hex` | Generate random hex string |
| `password` | Generate password with symbols |
| `choice` | Pick random item from list |
| `shuffle` | Shuffle items randomly |
| `coin` | Flip a coin |
| `dice` | Roll dice |

---

## int

Generate random integers.

```bash
dx rand int [MIN] [MAX] [OPTIONS]
```

| Argument | Default | Description |
|----------|---------|-------------|
| `MIN` | `1` | Minimum value (inclusive) |
| `MAX` | `100` | Maximum value (inclusive) |
| `-c, --count` | `1` | Number of values to generate |

```bash
dx rand int                    # 1-100
dx rand int 1 10               # 1-10
dx rand int 0 1000 -c 5        # 5 random numbers 0-1000
```

---

## float

Generate random floating-point numbers.

```bash
dx rand float [MIN] [MAX] [OPTIONS]
```

| Argument | Default | Description |
|----------|---------|-------------|
| `MIN` | `0.0` | Minimum value |
| `MAX` | `1.0` | Maximum value (exclusive) |
| `-c, --count` | `1` | Number of values to generate |

```bash
dx rand float                  # 0.0-1.0
dx rand float 0 100            # 0.0-100.0
dx rand float 0 1 -c 3         # 3 random floats
```

---

## string

Generate random alphanumeric strings.

```bash
dx rand string [LENGTH] [OPTIONS]
```

| Argument | Default | Description |
|----------|---------|-------------|
| `LENGTH` | `16` | Length of string |
| `-c, --count` | `1` | Number of strings to generate |

```bash
dx rand string                 # 16 chars
dx rand string 32              # 32 chars
dx rand string 8 -c 5          # 5 strings of 8 chars
```

---

## hex

Generate random hexadecimal strings.

```bash
dx rand hex [BYTES] [OPTIONS]
```

| Argument | Default | Description |
|----------|---------|-------------|
| `BYTES` | `16` | Number of random bytes (output is 2x length) |
| `-c, --count` | `1` | Number of hex strings to generate |

```bash
dx rand hex                    # 32 hex chars (16 bytes)
dx rand hex 4                  # 8 hex chars
dx rand hex 32 -c 3            # 3 strings of 64 hex chars
```

---

## password

Generate passwords with mixed characters.

```bash
dx rand password [LENGTH] [OPTIONS]
```

| Argument | Default | Description |
|----------|---------|-------------|
| `LENGTH` | `16` | Password length |
| `--no-symbols` | | Exclude special characters |
| `-c, --count` | `1` | Number of passwords to generate |

```bash
dx rand password               # 16-char with symbols
dx rand password 20            # 20-char with symbols
dx rand password 12 --no-symbols  # Letters and digits only
dx rand password 16 -c 5       # Generate 5 passwords
```

---

## choice

Pick a random item from a list.

```bash
dx rand choice <ITEMS>...
```

```bash
dx rand choice red green blue  # Random color
dx rand choice "yes" "no"      # Random decision
dx rand choice $(cat options.txt)  # From file
```

---

## shuffle

Randomly shuffle a list of items.

```bash
dx rand shuffle <ITEMS>...
```

```bash
dx rand shuffle a b c d e      # Shuffled order
dx rand shuffle 1 2 3 4 5      # Shuffled numbers
```

---

## coin

Flip a coin.

```bash
dx rand coin [OPTIONS]
```

| Option | Default | Description |
|--------|---------|-------------|
| `-c, --count` | `1` | Number of flips |

```bash
dx rand coin                   # heads or tails
dx rand coin -c 10             # Flip 10 times
```

---

## dice

Roll dice.

```bash
dx rand dice [SIDES] [OPTIONS]
```

| Argument | Default | Description |
|----------|---------|-------------|
| `SIDES` | `6` | Number of sides on the die |
| `-c, --count` | `1` | Number of rolls |

```bash
dx rand dice                   # Roll d6
dx rand dice 20                # Roll d20
dx rand dice 6 -c 3            # Roll 3d6
```

## Examples

```bash
# Generate API key
dx rand hex 32

# Create secure password
dx rand password 24

# Pick random team member
dx rand choice Alice Bob Carol Dave

# Shuffle playlist
dx rand shuffle song1 song2 song3 song4

# D&D dice rolls
dx rand dice 20                # Attack roll
dx rand dice 6 -c 4            # Roll 4d6 for stats
```
