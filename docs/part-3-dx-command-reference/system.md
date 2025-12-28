# system - System Information

The `system` command displays system information and statistics.

**Alias:** `sys`

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `info` | Show comprehensive system information |
| `uptime` | Show system uptime |

## Examples

### System Info
```bash
dx system info
dx sys info
```

Output includes:
- **OS**: Operating system name and version
- **Hostname**: Machine name
- **CPU**: Processor count and usage
- **Memory**: Total, used, and available RAM
- **Uptime**: How long the system has been running

### Uptime Only
```bash
dx system uptime
dx sys uptime
```

Shows system uptime in human-readable format (days, hours, minutes).

## Sample Output

```
System Information
==================

OS:        macOS 14.2.1
Hostname:  MacBook-Pro.local
Kernel:    Darwin 23.2.0

CPU:       12 cores
CPU Usage: 15.3%

Memory:    32.0 GB total
           18.5 GB used (57.8%)
           13.5 GB available

Uptime:    5d 12h 34m
```

## JSON Output

Use `-o json` for structured output:

```bash
dx system info -o json
```

```json
{
  "os_name": "macOS",
  "os_version": "14.2.1",
  "hostname": "MacBook-Pro.local",
  "cpu_count": 12,
  "cpu_usage": 15.3,
  "memory_total": 34359738368,
  "memory_used": 19864223744,
  "uptime_seconds": 486840
}
```

## Use Cases

### Scripts
```bash
# Check available memory
available=$(dx sys info -o json | jq '.memory_total - .memory_used')

# Log system stats
dx sys info -o json >> /var/log/system-stats.jsonl
```

### Monitoring
```bash
# Watch system stats
watch -n 5 'dx sys info'
```
