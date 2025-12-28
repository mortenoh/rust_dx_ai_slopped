# Appendix J: Cross-Platform Testing Environments

This guide provides exhaustive documentation for setting up local testing environments for all dx target platforms. Whether you're testing CLI commands or GUI applications (egui), this guide covers everything from Docker containers to full VM setups.

## Table of Contents

1. [Introduction](#introduction)
2. [Target Platforms Reference](#target-platforms-reference)
3. [Testing from macOS (Apple Silicon)](#testing-from-macos-apple-silicon)
4. [Testing from macOS (Intel)](#testing-from-macos-intel)
5. [Testing from Linux](#testing-from-linux)
6. [Testing from Windows](#testing-from-windows)
7. [QEMU Deep Dive](#qemu-deep-dive)
8. [Testing GUI Applications](#testing-gui-applications)
9. [Platform-Specific Setup Guides](#platform-specific-setup-guides)
10. [Docker Multi-Architecture Testing](#docker-multi-architecture-testing)
11. [CI vs Local Testing](#ci-vs-local-testing)
12. [Recommended Setups by Host](#recommended-setups-by-host)
13. [Troubleshooting](#troubleshooting)
14. [Quick Reference](#quick-reference)

---

## Introduction

### Why Local Cross-Platform Testing Matters

While CI/CD pipelines (like GitHub Actions) provide automated testing across platforms, local testing offers:

- **Faster feedback loop** - No waiting for CI
- **Interactive debugging** - Attach debuggers, inspect state
- **GUI testing** - CI runners are headless
- **Network testing** - Test actual network conditions
- **Edge case exploration** - Manual testing scenarios

### CLI vs GUI Testing

| Application Type | Docker | VM Required | Display Needed |
|------------------|--------|-------------|----------------|
| CLI tools | ✅ | No | No |
| TUI (ratatui) | ✅ | No | TTY only |
| GUI (egui) | ❌ | Yes | Yes |

### Testing Approaches Overview

| Approach | Speed | Isolation | GUI Support | Setup Complexity |
|----------|-------|-----------|-------------|------------------|
| Native | ⚡ Fast | None | ✅ | None |
| Rosetta 2 | ⚡ Fast | None | ✅ | None |
| Docker | 🚀 Very Fast | Container | ❌ | Low |
| OrbStack | 🚀 Very Fast | VM | ❌ | Low |
| UTM | 🔶 Medium | Full VM | ✅ | Medium |
| Parallels | ⚡ Fast | Full VM | ✅ | Medium |
| QEMU User-mode | 🔶 Medium | Process | ❌ | Medium |
| QEMU System | 🐢 Slow | Full VM | ✅ | High |

---

## Target Platforms Reference

The dx project builds for these targets:

| Target Triple | OS | Architecture | Libc | Notes |
|---------------|-----|--------------|------|-------|
| `x86_64-unknown-linux-gnu` | Linux | x86_64 | glibc | Most common Linux |
| `x86_64-unknown-linux-musl` | Linux | x86_64 | musl | Static, Alpine-compatible |
| `aarch64-unknown-linux-gnu` | Linux | ARM64 | glibc | Raspberry Pi 4, AWS Graviton |
| `x86_64-apple-darwin` | macOS | x86_64 | - | Intel Macs |
| `aarch64-apple-darwin` | macOS | ARM64 | - | Apple Silicon (M1/M2/M3) |
| `x86_64-pc-windows-gnullvm` | Windows | x86_64 | MSVC | Most Windows PCs |
| `aarch64-pc-windows-gnullvm` | Windows | ARM64 | MSVC | Surface Pro X, Windows ARM |

### Binary Naming Convention

```
dx-{target}           # Linux/macOS
dx-{target}.exe       # Windows

# Examples:
dx-x86_64-unknown-linux-gnu
dx-aarch64-apple-darwin
dx-x86_64-pc-windows-gnullvm.exe
```

---

## Testing from macOS (Apple Silicon)

If you're on an M1/M2/M3 Mac, you have excellent options for testing all platforms.

### What Runs Natively

| Target | Method | Speed |
|--------|--------|-------|
| `aarch64-apple-darwin` | Native | ⚡ Full speed |
| `x86_64-apple-darwin` | Rosetta 2 | ⚡ ~80-90% native |
| `aarch64-unknown-linux-gnu` | UTM/OrbStack | 🚀 Near-native |
| `x86_64-unknown-linux-gnu` | UTM+Rosetta/Docker | 🔶 ~60-70% |
| `aarch64-pc-windows-gnullvm` | UTM/Parallels | 🚀 Near-native |
| `x86_64-pc-windows-gnullvm` | UTM (emulated) | 🐢 ~10-20% |

### UTM (Free, Recommended)

UTM is a free, open-source VM manager based on QEMU with a native macOS UI.

#### Installation

```bash
# Via Homebrew
brew install --cask utm

# Or download from
# https://mac.getutm.app/
```

#### Pre-built VMs

UTM provides ready-to-use VMs at [mac.getutm.app/gallery](https://mac.getutm.app/gallery/):

- Ubuntu 22.04/24.04 (ARM64)
- Debian 12 (ARM64)
- Windows 11 (ARM64)
- Fedora (ARM64)
- Arch Linux (ARM64)

#### Creating an Ubuntu ARM64 VM

1. Download Ubuntu Server for ARM: [ubuntu.com/download/server/arm](https://ubuntu.com/download/server/arm)

2. Create new VM in UTM:
   - **Type**: Virtualize (not Emulate)
   - **OS**: Linux
   - **Boot ISO**: Select downloaded Ubuntu ISO
   - **Memory**: 4096 MB minimum (8192 recommended)
   - **CPU Cores**: 4+ recommended
   - **Storage**: 20 GB minimum

3. VM Settings for best performance:
   ```
   System → CPU → Force Multicore: ✓
   Display → Emulated Display Card: virtio-gpu-gl-pci
   Network → Network Mode: Shared Network
   Sharing → Enable Directory Sharing: ✓
   ```

4. Install Ubuntu, then install guest tools:
   ```bash
   sudo apt update
   sudo apt install spice-vdagent spice-webdavd
   ```

#### Creating a Windows 11 ARM VM

1. Download Windows 11 ARM64 VHDX from Microsoft:
   - Go to [uupdump.net](https://uupdump.net/)
   - Search "Windows 11 arm64"
   - Download and create VHDX

   Or use Windows Insider:
   - [aka.ms/intune](https://www.microsoft.com/en-us/software-download/windowsinsiderpreviewARM64)

2. Create VM in UTM:
   - **Type**: Virtualize
   - **OS**: Windows
   - **Import VHDX**: Select the downloaded file
   - **Memory**: 8192 MB recommended
   - **CPU Cores**: 4+

3. Install SPICE Guest Tools in Windows:
   - Download from [spice-space.org](https://www.spice-space.org/download.html)
   - Enables clipboard sharing, dynamic resolution

### Parallels Desktop

Commercial option with best Windows performance on Apple Silicon.

#### Pros vs UTM

| Feature | UTM (Free) | Parallels ($99/yr) |
|---------|------------|-------------------|
| Price | Free | $99/year |
| Windows Performance | Good | Excellent |
| GPU Acceleration | Basic | Full DirectX |
| Coherence Mode | No | Yes |
| Snapshots | Yes | Yes |
| Linux Performance | Excellent | Excellent |

#### Installation

```bash
brew install --cask parallels
```

#### Quick Setup

1. Parallels automatically downloads Windows 11 ARM
2. One-click Linux VM creation
3. Automatic guest tools installation

### VMware Fusion

Free for personal use, good performance.

```bash
brew install --cask vmware-fusion
```

- Download from [vmware.com/products/fusion](https://www.vmware.com/products/fusion.html)
- Free "Player" version for personal use
- Requires registration

### OrbStack (Fast Linux)

OrbStack provides extremely fast Linux VMs and Docker on macOS.

#### Installation

```bash
brew install orbstack
```

#### Usage

```bash
# Create an Ubuntu VM
orb create ubuntu my-ubuntu

# SSH into VM
orb shell my-ubuntu

# Or use Docker
orb docker run -it ubuntu bash
```

#### Performance

OrbStack uses Apple's Virtualization.framework with Rosetta, providing:
- Near-native ARM64 Linux performance
- Excellent x86_64 Linux performance (via Rosetta)
- Shared filesystem with macOS
- Automatic SSH configuration

### Docker Desktop

For headless CLI testing, Docker is the fastest option.

```bash
brew install --cask docker

# Run ARM64 Linux
docker run --platform linux/arm64 -it ubuntu bash

# Run x86_64 Linux (via Rosetta)
docker run --platform linux/amd64 -it ubuntu bash
```

---

## Testing from macOS (Intel)

If you're on an Intel Mac:

### Native Testing

| Target | Method |
|--------|--------|
| `x86_64-apple-darwin` | Native |
| `x86_64-unknown-linux-gnu` | Docker/VM |
| `x86_64-pc-windows-msvc` | Parallels/VMware |

### ARM Emulation (Slow)

ARM targets require full emulation:

```bash
# UTM with QEMU emulation (slow)
# Select "Emulate" instead of "Virtualize"
```

Performance is roughly 10-20x slower than native.

### Recommended Setup

1. **Docker Desktop** for Linux x86_64
2. **Parallels/VMware** for Windows
3. **Skip ARM testing locally** - rely on CI

---

## Testing from Linux

### Native Testing

On a Linux x86_64 machine:

| Target | Method |
|--------|--------|
| `x86_64-unknown-linux-gnu` | Native |
| `x86_64-unknown-linux-musl` | Native (install musl-tools) |

### QEMU User-Mode for ARM

Run ARM binaries directly without a full VM:

```bash
# Install QEMU user-mode
sudo apt install qemu-user-static binfmt-support

# Register binary formats
sudo update-binfmts --enable

# Now ARM binaries run transparently!
./dx-aarch64-unknown-linux-gnu --version
```

This uses binfmt_misc to intercept ARM binaries and run them through QEMU.

#### Performance

User-mode emulation is typically 5-10x slower than native, but much faster than full system emulation.

### Docker Multi-Architecture

```bash
# Enable multi-arch
docker run --privileged --rm tonistiigi/binfmt --install all

# Run ARM64
docker run --platform linux/arm64 -v $(pwd)/dist:/dist alpine \
  /dist/dx-aarch64-unknown-linux-gnu --version

# Run x86_64 (native on x86_64 host)
docker run --platform linux/amd64 -v $(pwd)/dist:/dist ubuntu \
  /dist/dx-x86_64-unknown-linux-gnu --version
```

### QEMU System Emulation

For full OS testing with GUI support:

```bash
# Install QEMU
sudo apt install qemu-system-x86 qemu-system-aarch64

# Download an ISO and run
qemu-system-aarch64 \
  -M virt \
  -cpu cortex-a72 \
  -m 4G \
  -drive file=ubuntu-arm64.qcow2,format=qcow2 \
  -cdrom ubuntu-arm64.iso \
  -nographic
```

### Windows Testing on Linux

#### Wine (Simple CLIs)

```bash
sudo apt install wine64
wine ./dx-x86_64-pc-windows-gnullvm.exe --version
```

Note: Wine has limitations with complex applications.

#### VirtualBox

```bash
sudo apt install virtualbox
```

Then install Windows from ISO.

#### QEMU + Windows

```bash
qemu-system-x86_64 \
  -enable-kvm \
  -m 8G \
  -cpu host \
  -drive file=windows.qcow2,format=qcow2 \
  -cdrom Win11_x64.iso
```

---

## Testing from Windows

### WSL2 for Linux Testing

```powershell
# Install WSL2
wsl --install

# Install specific distros
wsl --install -d Ubuntu

# Run Linux binaries
wsl ./dx-x86_64-unknown-linux-gnu --version
```

### ARM64 on ARM Windows

If you're on Windows ARM (Surface Pro X, etc.):

```powershell
# Native ARM64 Windows
.\dx-aarch64-pc-windows-gnullvm.exe --version

# x86_64 via emulation
.\dx-x86_64-pc-windows-gnullvm.exe --version
```

### Hyper-V for VMs

```powershell
# Enable Hyper-V (requires Pro/Enterprise)
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All

# Create VM via Hyper-V Manager
```

---

## QEMU Deep Dive

QEMU (Quick EMUlator) is the foundation of many virtualization tools.

### User-Mode vs System Emulation

| Mode | Use Case | Speed | Isolation |
|------|----------|-------|-----------|
| User-mode | Run single binary | Fast | Process only |
| System | Full OS | Slow | Complete |

### User-Mode Emulation

Run binaries for different architectures:

```bash
# Install (Debian/Ubuntu)
sudo apt install qemu-user-static

# Run ARM64 binary on x86_64
qemu-aarch64-static ./dx-aarch64-unknown-linux-gnu --version

# With library path
qemu-aarch64-static -L /usr/aarch64-linux-gnu ./my-binary
```

#### binfmt_misc Setup

Automatic execution of foreign binaries:

```bash
# Check current registrations
cat /proc/sys/fs/binfmt_misc/qemu-aarch64

# Register (usually automatic with qemu-user-static)
sudo update-binfmts --enable qemu-aarch64

# Now this works directly:
./dx-aarch64-unknown-linux-gnu --version
```

#### Docker + binfmt

```bash
# Register all architectures
docker run --privileged --rm tonistiigi/binfmt --install all

# Verify
docker run --rm --platform linux/arm64 alpine uname -m
# Output: aarch64
```

### System Emulation

Full OS emulation with QEMU:

#### Basic x86_64 VM

```bash
# Create disk image
qemu-img create -f qcow2 disk.qcow2 20G

# Boot installer
qemu-system-x86_64 \
  -enable-kvm \                    # Hardware acceleration (Linux only)
  -m 4G \                          # 4GB RAM
  -smp 4 \                         # 4 CPU cores
  -cpu host \                      # Use host CPU features
  -drive file=disk.qcow2,format=qcow2 \
  -cdrom ubuntu.iso \
  -boot d \                        # Boot from CD
  -vga virtio \                    # Virtio GPU
  -display gtk                     # GTK display window
```

#### ARM64 VM on x86_64 Host

```bash
qemu-system-aarch64 \
  -M virt \                        # Virtual machine type
  -cpu cortex-a72 \                # ARM CPU model
  -m 4G \
  -smp 4 \
  -bios /usr/share/qemu-efi-aarch64/QEMU_EFI.fd \  # UEFI firmware
  -drive file=disk-arm64.qcow2,format=qcow2 \
  -cdrom ubuntu-arm64.iso \
  -device virtio-gpu-pci \
  -display gtk
```

#### Accelerators

| Accelerator | Host OS | Guest Arch | Speed |
|-------------|---------|------------|-------|
| KVM | Linux | Same as host | Native |
| HVF | macOS | Same as host | Near-native |
| WHPX | Windows | Same as host | Near-native |
| TCG | Any | Any | Slow (software) |

```bash
# Check KVM availability
ls -la /dev/kvm

# Use KVM
qemu-system-x86_64 -enable-kvm ...

# Use HVF (macOS)
qemu-system-aarch64 -accel hvf ...
```

#### Networking Options

```bash
# User-mode networking (NAT, no root needed)
-netdev user,id=net0,hostfwd=tcp::2222-:22 \
-device virtio-net-pci,netdev=net0

# Bridge networking (requires root)
-netdev bridge,id=net0,br=br0 \
-device virtio-net-pci,netdev=net0
```

#### Display Options

```bash
-display gtk          # GTK window
-display cocoa        # macOS native
-display sdl          # SDL window
-display vnc=:1       # VNC server on :5901
-nographic            # Serial console only
```

#### Shared Folders

```bash
# 9p filesystem sharing
-virtfs local,path=/shared,mount_tag=host0,security_model=mapped-xattr

# In guest:
sudo mount -t 9p -o trans=virtio host0 /mnt/shared
```

---

## Testing GUI Applications

GUI applications (egui, native GUIs) require a display server.

### Why Docker Doesn't Work

Docker containers are headless - they have no:
- Display server (X11/Wayland)
- GPU access (by default)
- Window system

While technically possible to run X11 in Docker with complex setups, VMs are simpler.

### VM Requirements for GUI

| Component | Purpose |
|-----------|---------|
| Display | Render windows |
| GPU (emulated) | OpenGL/Vulkan |
| Input | Mouse/keyboard |

#### UTM Configuration for GUI

```
Display:
  - Type: virtio-gpu-gl-pci (hardware accelerated)
  - Or: virtio-gpu-pci (software)

Input:
  - USB Tablet (better mouse tracking)
```

#### QEMU Configuration for GUI

```bash
qemu-system-x86_64 \
  -device virtio-vga-gl \          # GPU with OpenGL
  -display gtk,gl=on \             # GTK with GL
  -device usb-tablet \             # Better mouse
  ...
```

### X11 Forwarding

Run GUI apps over SSH:

#### On macOS (with XQuartz)

```bash
# Install XQuartz
brew install --cask xquartz

# Log out and back in, then:
ssh -X user@linux-vm

# Run GUI app
./dx egui demo
```

#### Configuration

```bash
# On server (/etc/ssh/sshd_config):
X11Forwarding yes
X11DisplayOffset 10

# On client (~/.ssh/config):
Host myvm
  ForwardX11 yes
  ForwardX11Trusted yes
```

### VNC Access

For remote GUI access:

#### Server Setup (Linux VM)

```bash
# Install TigerVNC
sudo apt install tigervnc-standalone-server

# Start VNC server
vncserver :1 -geometry 1920x1080 -depth 24

# Set password when prompted
```

#### Client Connection

```bash
# On macOS, use built-in Screen Sharing
open vnc://192.168.64.2:5901

# Or install a VNC client
brew install --cask vnc-viewer
```

### RDP for Windows VMs

```bash
# On macOS
brew install --cask microsoft-remote-desktop

# Or use Remmina on Linux
sudo apt install remmina
```

### Headless GUI Testing (Xvfb)

For CI or automated testing:

```bash
# Install virtual framebuffer
sudo apt install xvfb

# Run with virtual display
xvfb-run -a ./dx egui demo &

# Or set DISPLAY manually
Xvfb :99 -screen 0 1920x1080x24 &
export DISPLAY=:99
./dx egui demo
```

**Limitations:**
- Can't visually inspect the app
- Some OpenGL features may not work
- Screenshot testing is possible but limited

---

## Platform-Specific Setup Guides

### Ubuntu 24.04 ARM64 in UTM

Complete walkthrough for the most common Linux testing setup.

#### Step 1: Download Ubuntu

```bash
# Download Ubuntu Server ARM64 (smaller, faster)
curl -LO https://cdimage.ubuntu.com/releases/24.04/release/ubuntu-24.04-live-server-arm64.iso

# Or Desktop (includes GUI)
curl -LO https://cdimage.ubuntu.com/releases/24.04/release/ubuntu-24.04-desktop-arm64.iso
```

#### Step 2: Create VM in UTM

1. Click "Create a New Virtual Machine"
2. Select "Virtualize" (not Emulate)
3. Select "Linux"
4. Browse to downloaded ISO
5. Configure:
   - Memory: 4096 MB (8192 for Desktop)
   - CPU Cores: 4
   - Storage: 30 GB

#### Step 3: Install Ubuntu

1. Boot the VM
2. Follow Ubuntu installer
3. Choose minimal installation for faster setup
4. Create user (e.g., `dev`)

#### Step 4: Post-Install Setup

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install development tools
sudo apt install -y build-essential git curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install SPICE tools (clipboard, resolution)
sudo apt install -y spice-vdagent spice-webdavd

# For GUI testing
sudo apt install -y ubuntu-desktop  # If using Server ISO
```

#### Step 5: Configure Shared Folder

In UTM:
1. VM Settings → Sharing
2. Enable Directory Sharing
3. Select folder to share

In guest:
```bash
# Mount shared folder
sudo mkdir /mnt/shared
sudo mount -t virtiofs share /mnt/shared

# Add to /etc/fstab for persistence
echo "share /mnt/shared virtiofs defaults 0 0" | sudo tee -a /etc/fstab
```

#### Step 6: Test dx Binaries

```bash
# Copy binary to VM
cp /mnt/shared/dx-aarch64-unknown-linux-gnu ./dx

# Make executable
chmod +x ./dx

# Test
./dx --version
./dx system info

# Test GUI (if desktop installed)
./dx egui demo
```

### Windows 11 ARM in UTM

#### Step 1: Get Windows 11 ARM

**Option A: Windows Insider (Official)**

1. Register at [insider.windows.com](https://insider.windows.com)
2. Download ARM64 VHDX from [aka.ms/intune](https://www.microsoft.com/en-us/software-download/windowsinsiderpreviewARM64)

**Option B: UUP Dump (Unofficial but works)**

1. Go to [uupdump.net](https://uupdump.net)
2. Search "Windows 11 arm64"
3. Select latest build
4. Choose "Download and convert to ISO"
5. Run the downloaded script to create ISO

#### Step 2: Create VM in UTM

1. Create New VM → Virtualize → Windows
2. Import VHDX or mount ISO
3. Configure:
   - Memory: 8192 MB
   - CPU Cores: 4+
   - Enable "Enable Windows Hypervisor" in advanced

#### Step 3: Install Windows

1. Boot VM
2. Follow Windows setup
3. Skip network during OOBE (use limited setup)
4. Create local account

#### Step 4: Install SPICE Guest Tools

1. Download from [spice-space.org](https://www.spice-space.org/download.html)
2. Install `spice-guest-tools` in Windows
3. Restart VM

#### Step 5: Install Rust (Optional, for building)

```powershell
# In PowerShell:
winget install Rustlang.Rustup

# Restart terminal
rustup default stable
```

#### Step 6: Test dx Binary

```powershell
# Copy from shared folder or download
.\dx-aarch64-pc-windows-gnullvm.exe --version
.\dx-aarch64-pc-windows-gnullvm.exe system info
.\dx-aarch64-pc-windows-gnullvm.exe egui demo
```

### Debian x86_64 in UTM (with Rosetta)

Apple Silicon Macs can run x86_64 Linux fast using Rosetta.

#### Step 1: Download Debian

```bash
curl -LO https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-12-amd64-netinst.iso
```

#### Step 2: Create VM with Rosetta

1. Create New VM → Virtualize → Linux
2. **Important**: Check "Use Apple Virtualization"
3. Check "Enable Rosetta (x86_64 Emulation)"
4. Mount Debian ISO
5. Configure memory and storage

#### Step 3: Install and Configure

```bash
# Install Debian normally

# After install, enable Rosetta in guest:
sudo mkdir -p /media/rosetta
sudo mount -t virtiofs rosetta /media/rosetta

# Register with binfmt
sudo /usr/sbin/update-binfmts --install rosetta /media/rosetta/rosetta \
  --magic "\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x3e\x00" \
  --mask "\xff\xff\xff\xff\xff\xfe\xfe\x00\xff\xff\xff\xff\xff\xff\xff\xff\xfe\xff\xff\xff" \
  --credentials yes --preserve no --fix-binary yes
```

Now x86_64 binaries run via Rosetta at near-native speed!

### Alpine Linux (musl Testing)

Alpine uses musl libc, perfect for testing `*-unknown-linux-musl` binaries.

#### Quick Docker Test

```bash
docker run --platform linux/amd64 -v $(pwd)/dist:/dist -it alpine sh
/dist/dx-x86_64-unknown-linux-musl --version
```

#### Full VM Setup

1. Download Alpine from [alpinelinux.org](https://alpinelinux.org/downloads/)
2. Create small VM (512MB RAM, 2GB disk)
3. Run `setup-alpine` installer

```bash
# After install:
apk add bash curl

# Test binary
./dx-x86_64-unknown-linux-musl --version
```

---

## Docker Multi-Architecture Testing

Docker is the fastest way to test CLI binaries across architectures.

### Initial Setup

```bash
# Install Docker
brew install --cask docker   # macOS
sudo apt install docker.io   # Linux

# Enable multi-arch
docker run --privileged --rm tonistiigi/binfmt --install all

# Verify
docker run --rm --platform linux/arm64 alpine uname -m
# Output: aarch64

docker run --rm --platform linux/amd64 alpine uname -m
# Output: x86_64
```

### Testing Script

Create `scripts/test-cross-platform.sh`:

```bash
#!/bin/bash
set -e

DIST_DIR="dist"

echo "=== Testing Linux x86_64 (glibc) ==="
docker run --rm --platform linux/amd64 \
  -v "$(pwd)/$DIST_DIR:/dist:ro" \
  ubuntu:22.04 \
  /dist/dx-x86_64-unknown-linux-gnu --version

echo "=== Testing Linux x86_64 (musl) ==="
docker run --rm --platform linux/amd64 \
  -v "$(pwd)/$DIST_DIR:/dist:ro" \
  alpine:latest \
  /dist/dx-x86_64-unknown-linux-musl --version

echo "=== Testing Linux ARM64 (glibc) ==="
docker run --rm --platform linux/arm64 \
  -v "$(pwd)/$DIST_DIR:/dist:ro" \
  ubuntu:22.04 \
  /dist/dx-aarch64-unknown-linux-gnu --version

echo "=== All tests passed! ==="
```

### Docker Compose for Testing

```yaml
# docker-compose.test.yml
version: "3"
services:
  test-linux-amd64:
    image: ubuntu:22.04
    platform: linux/amd64
    volumes:
      - ./dist:/dist:ro
    command: /dist/dx-x86_64-unknown-linux-gnu --version

  test-linux-arm64:
    image: ubuntu:22.04
    platform: linux/arm64
    volumes:
      - ./dist:/dist:ro
    command: /dist/dx-aarch64-unknown-linux-gnu --version

  test-alpine:
    image: alpine:latest
    platform: linux/amd64
    volumes:
      - ./dist:/dist:ro
    command: /dist/dx-x86_64-unknown-linux-musl --version
```

```bash
docker-compose -f docker-compose.test.yml up
```

### Limitations of Docker

| What Works | What Doesn't |
|------------|--------------|
| CLI binaries | GUI applications |
| Basic I/O | GPU access |
| Network | Audio |
| File system | USB devices |
| Environment variables | Systemd services |

---

## CI vs Local Testing

### What GitHub Actions Covers

The dx CI matrix tests:

| Platform | Runner | Native/Emulated |
|----------|--------|-----------------|
| Linux x86_64 | ubuntu-latest | Native |
| macOS x86_64 | macos-latest | Native |
| macOS ARM64 | macos-latest (M1) | Native |
| Windows x86_64 | windows-latest | Native |

### What Requires Local Testing

| Scenario | Why |
|----------|-----|
| GUI applications | CI is headless |
| Interactive debugging | Need direct access |
| Hardware-specific features | USB, GPU, etc. |
| Network edge cases | Real network conditions |
| Performance testing | Consistent hardware |
| ARM Windows | No CI runners available |

### Testing Strategy

```
┌─────────────────────────────────────────────────────────────┐
│                     Development Flow                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Local Development                                        │
│     └── Native build and test                                │
│                                                              │
│  2. Pre-commit                                               │
│     └── make lint test                                       │
│                                                              │
│  3. Push to GitHub                                           │
│     └── CI runs on Linux, macOS, Windows                     │
│                                                              │
│  4. Manual Testing (as needed)                               │
│     ├── Docker for Linux variants                            │
│     ├── UTM for Windows ARM                                  │
│     └── UTM for GUI testing                                  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Recommended Setups by Host

### Apple Silicon Mac (M1/M2/M3)

**Best overall cross-platform development experience.**

| Purpose | Tool | Install |
|---------|------|---------|
| Fast Linux CLI | OrbStack | `brew install orbstack` |
| Linux GUI testing | UTM | `brew install --cask utm` |
| Windows ARM | UTM | Pre-built VM from gallery |
| Quick CLI tests | Docker | `brew install --cask docker` |
| Windows (if needed for perf) | Parallels | Purchase license |

**Recommended install order:**

```bash
# Essential
brew install --cask docker
brew install --cask utm

# Optional but nice
brew install orbstack
```

### Intel Mac

| Purpose | Tool |
|---------|------|
| Linux CLI | Docker Desktop |
| Windows | Parallels or VMware Fusion |
| Linux GUI | UTM or VMware |

**Note:** ARM testing will be slow (full emulation).

### Linux x86_64

| Purpose | Tool |
|---------|------|
| ARM CLI binaries | QEMU user-mode |
| Full ARM OS | QEMU system |
| Windows | VirtualBox or QEMU |
| Containers | Docker + binfmt |

```bash
# Essential
sudo apt install qemu-user-static binfmt-support docker.io

# For full VMs
sudo apt install qemu-system-x86 qemu-system-aarch64 virt-manager
```

### Windows x86_64

| Purpose | Tool |
|---------|------|
| Linux CLI | WSL2 |
| Linux GUI | WSL2 + WSLg |
| Full VMs | Hyper-V |

```powershell
# Install WSL2
wsl --install -d Ubuntu

# Enable Hyper-V (Pro/Enterprise only)
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All
```

---

## Troubleshooting

### UTM Issues

#### "Virtualization is not supported"

- Ensure you selected "Virtualize" not "Emulate"
- Check that VM architecture matches host (ARM64 on Apple Silicon)

#### Slow Performance

- Increase memory and CPU cores
- Use virtio devices (disk, network, GPU)
- Enable "Force Multicore" in settings

#### No Network in Guest

- Use "Shared Network" mode
- Check firewall settings
- Try "Bridged" mode if needed

#### Shared Folder Not Working

1. Install SPICE tools in guest
2. Use correct mount command:
   ```bash
   # Linux
   sudo mount -t virtiofs share /mnt/shared
   ```

### QEMU Issues

#### "Could not access KVM kernel module"

```bash
# Check KVM availability
ls -la /dev/kvm

# Add user to kvm group
sudo usermod -aG kvm $USER

# Log out and back in
```

#### Very Slow Performance

- Ensure hardware acceleration is enabled (`-enable-kvm`, `-accel hvf`)
- Use virtio devices
- Increase memory

#### No Display

```bash
# Try different display backends
-display gtk
-display sdl
-display vnc=:1
```

### Docker Issues

#### "permission denied" on Volume Mount

```bash
# Check file ownership
ls -la dist/

# On Linux, may need to run as root or fix SELinux
docker run --privileged ...
```

#### Multi-arch Not Working

```bash
# Re-register binfmt handlers
docker run --privileged --rm tonistiigi/binfmt --install all

# Check registration
cat /proc/sys/fs/binfmt_misc/qemu-aarch64
```

#### "exec format error"

- Binary architecture doesn't match container platform
- Ensure `--platform` flag matches binary target

### GUI Not Displaying

#### X11 Forwarding Fails

```bash
# On client
echo $DISPLAY  # Should not be empty

# On server, check sshd_config
grep X11Forwarding /etc/ssh/sshd_config
# Should be: X11Forwarding yes
```

#### XQuartz Issues (macOS)

1. Install XQuartz: `brew install --cask xquartz`
2. Log out and log back in
3. Open XQuartz before SSH

### Performance Optimization

#### VM Performance

| Setting | Impact |
|---------|--------|
| More RAM | Reduces swapping |
| More CPU cores | Parallel workloads |
| SSD storage | Faster I/O |
| Virtio devices | Better than emulated |
| GPU acceleration | GUI performance |

#### Docker Performance

```bash
# Use overlay2 storage driver
docker info | grep "Storage Driver"

# Limit container resources if needed
docker run --memory 2g --cpus 2 ...
```

---

## Quick Reference

### Tool Comparison Matrix

| Feature | UTM | Parallels | VMware | Docker | OrbStack |
|---------|-----|-----------|--------|--------|----------|
| Price | Free | $99/yr | Free* | Free | $8/mo |
| Linux ARM64 | ✅ | ✅ | ✅ | ✅ | ✅ |
| Linux x86_64 | ✅ | ✅ | ✅ | ✅ | ✅ |
| Windows ARM | ✅ | ✅ | ✅ | ❌ | ❌ |
| GUI Support | ✅ | ✅ | ✅ | ❌ | ❌ |
| GPU Accel | Basic | Full | Good | ❌ | ❌ |
| Snapshots | ✅ | ✅ | ✅ | ✅ | ✅ |
| Shared Folders | ✅ | ✅ | ✅ | ✅ | ✅ |
| Setup Ease | Easy | Easiest | Medium | Easy | Easiest |

*VMware Fusion is free for personal use

### Platform Capability Matrix

What can run where (on Apple Silicon Mac):

| Target | Native | Rosetta | VM (Fast) | VM (Slow) | Docker |
|--------|--------|---------|-----------|-----------|--------|
| macOS ARM64 | ✅ | - | - | - | - |
| macOS x86_64 | - | ✅ | - | - | - |
| Linux ARM64 | - | - | ✅ | - | ✅ |
| Linux x86_64 | - | - | ✅* | ✅ | ✅ |
| Windows ARM64 | - | - | ✅ | - | ❌ |
| Windows x86_64 | - | - | - | ✅ | ❌ |

*With Rosetta in Linux VM

### Download Links

| Tool | URL |
|------|-----|
| UTM | [mac.getutm.app](https://mac.getutm.app/) |
| Parallels | [parallels.com](https://www.parallels.com/) |
| VMware Fusion | [vmware.com/products/fusion](https://www.vmware.com/products/fusion.html) |
| Docker Desktop | [docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop/) |
| OrbStack | [orbstack.dev](https://orbstack.dev/) |
| Ubuntu ARM64 | [ubuntu.com/download/server/arm](https://ubuntu.com/download/server/arm) |
| Windows 11 ARM | [uupdump.net](https://uupdump.net/) |
| XQuartz | [xquartz.org](https://www.xquartz.org/) |
| QEMU | [qemu.org](https://www.qemu.org/) |
| VirtualBox | [virtualbox.org](https://www.virtualbox.org/) |

### Quick Commands

```bash
# Docker: Test ARM64 Linux binary
docker run --platform linux/arm64 -v ./dist:/dist alpine /dist/dx-aarch64-unknown-linux-gnu --version

# Docker: Test x86_64 Linux binary
docker run --platform linux/amd64 -v ./dist:/dist ubuntu /dist/dx-x86_64-unknown-linux-gnu --version

# QEMU: Run ARM64 binary on x86_64 Linux
qemu-aarch64-static ./dx-aarch64-unknown-linux-gnu --version

# UTM: Install from command line
brew install --cask utm

# OrbStack: Create Ubuntu VM
orb create ubuntu dev-vm && orb shell dev-vm
```

---

## Summary

For most developers on Apple Silicon Macs:

1. **Start with Docker** for quick CLI testing
2. **Add UTM** when you need GUI testing or Windows
3. **Consider OrbStack** for faster Linux development
4. **Use CI** for comprehensive cross-platform validation

The combination of Docker + UTM covers 95% of local testing needs at zero cost.
