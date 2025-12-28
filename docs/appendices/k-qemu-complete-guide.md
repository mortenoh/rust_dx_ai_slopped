# Appendix K: QEMU Complete CLI Guide

A comprehensive, step-by-step guide to using QEMU from the command line. This covers everything from basic usage to advanced configurations for disk, network, display, and more.

## Table of Contents

1. [Introduction to QEMU](#introduction-to-qemu)
2. [Installation](#installation)
3. [QEMU Binaries Reference](#qemu-binaries-reference)
4. [Disk Images](#disk-images)
5. [Basic VM Creation](#basic-vm-creation)
6. [CPU and Memory](#cpu-and-memory)
7. [Storage Devices](#storage-devices)
8. [Network Configuration](#network-configuration)
9. [Display and Graphics](#display-and-graphics)
10. [Input Devices](#input-devices)
11. [Audio](#audio)
12. [USB Devices](#usb-devices)
13. [UEFI and BIOS](#uefi-and-bios)
14. [Shared Folders](#shared-folders)
15. [Snapshots and Backing Files](#snapshots-and-backing-files)
16. [Hardware Acceleration](#hardware-acceleration)
17. [Complete Examples](#complete-examples)
18. [QEMU Monitor](#qemu-monitor)
19. [Scripting and Automation](#scripting-and-automation)
20. [Performance Tuning](#performance-tuning)
21. [Troubleshooting](#troubleshooting)
22. [Complete Option Reference](#complete-option-reference)

---

## Introduction to QEMU

QEMU (Quick EMUlator) is a generic, open-source machine emulator and virtualizer.

### Two Modes of Operation

| Mode | Description | Speed | Use Case |
|------|-------------|-------|----------|
| **Full System Emulation** | Emulates entire machine | Slower | Different architecture |
| **User-Mode Emulation** | Runs single binaries | Faster | Same OS, different arch |

### Key Concepts

- **Guest**: The emulated/virtualized system
- **Host**: Your physical machine
- **Target**: Architecture being emulated
- **Accelerator**: Hardware virtualization (KVM, HVF, WHPX)
- **Device**: Emulated hardware component
- **Drive**: Storage device (disk, CD-ROM)
- **Netdev**: Network backend
- **Chardev**: Character device (serial, console)

---

## Installation

### macOS

```bash
# Via Homebrew
brew install qemu

# Verify installation
qemu-system-x86_64 --version
qemu-system-aarch64 --version
qemu-img --version

# List all installed QEMU binaries
ls /opt/homebrew/bin/qemu-*
```

### Ubuntu/Debian

```bash
# Full installation
sudo apt update
sudo apt install qemu-system-x86 qemu-system-arm qemu-system-misc \
                 qemu-user-static qemu-utils ovmf

# Minimal (x86 only)
sudo apt install qemu-system-x86 qemu-utils

# Verify
qemu-system-x86_64 --version

# Check KVM support
ls -la /dev/kvm
sudo apt install cpu-checker
kvm-ok
```

### Fedora/RHEL

```bash
sudo dnf install qemu-kvm qemu-img qemu-system-x86 qemu-system-aarch64
sudo dnf install edk2-ovmf  # UEFI support
```

### Windows

```powershell
# Via Chocolatey
choco install qemu

# Or download from https://www.qemu.org/download/#windows
# Add to PATH: C:\Program Files\qemu
```

### Arch Linux

```bash
sudo pacman -S qemu-full
```

---

## QEMU Binaries Reference

### System Emulators

| Binary | Target Architecture | Common Use |
|--------|---------------------|------------|
| `qemu-system-x86_64` | x86 64-bit | Most PCs, servers |
| `qemu-system-i386` | x86 32-bit | Legacy systems |
| `qemu-system-aarch64` | ARM 64-bit | Raspberry Pi 4, M1 VMs |
| `qemu-system-arm` | ARM 32-bit | Raspberry Pi 1-3, embedded |
| `qemu-system-riscv64` | RISC-V 64-bit | Emerging architecture |
| `qemu-system-ppc64` | PowerPC 64-bit | IBM systems |
| `qemu-system-s390x` | IBM Z | Mainframes |
| `qemu-system-mips64` | MIPS 64-bit | Networking equipment |

### User-Mode Emulators

| Binary | Target Architecture |
|--------|---------------------|
| `qemu-x86_64` | x86 64-bit |
| `qemu-aarch64` | ARM 64-bit |
| `qemu-arm` | ARM 32-bit |
| `qemu-riscv64` | RISC-V 64-bit |
| `qemu-*-static` | Statically linked (for containers) |

### Utilities

| Binary | Purpose |
|--------|---------|
| `qemu-img` | Disk image manipulation |
| `qemu-nbd` | Network block device server |
| `qemu-io` | Low-level disk I/O testing |
| `qemu-ga` | Guest agent |

---

## Disk Images

### Image Formats

| Format | Description | Features | Size |
|--------|-------------|----------|------|
| `raw` | Raw disk image | Fastest, simple | Fixed |
| `qcow2` | QEMU Copy-On-Write v2 | Snapshots, compression, encryption | Dynamic |
| `vdi` | VirtualBox format | Compatible with VBox | Dynamic |
| `vmdk` | VMware format | Compatible with VMware | Dynamic |
| `vhdx` | Hyper-V format | Compatible with Hyper-V | Dynamic |

### Creating Disk Images

#### Basic Creation

```bash
# Create 20GB qcow2 image (sparse, starts small)
qemu-img create -f qcow2 disk.qcow2 20G

# Create 20GB raw image (pre-allocated)
qemu-img create -f raw disk.raw 20G

# Create with specific cluster size (performance tuning)
qemu-img create -f qcow2 -o cluster_size=2M disk.qcow2 50G

# Create with preallocation (better performance, uses more space)
qemu-img create -f qcow2 -o preallocation=metadata disk.qcow2 50G
qemu-img create -f qcow2 -o preallocation=full disk.qcow2 50G

# Create with compression enabled
qemu-img create -f qcow2 -o compression_type=zstd disk.qcow2 50G
```

#### qcow2 Creation Options

```bash
qemu-img create -f qcow2 \
  -o cluster_size=65536 \        # Cluster size (default 64K)
  -o preallocation=metadata \    # off, metadata, falloc, full
  -o lazy_refcounts=on \         # Faster, less safe
  -o compression_type=zlib \     # zlib or zstd
  -o encrypt.format=luks \       # Encryption
  -o encrypt.key-secret=sec0 \   # Encryption key
  disk.qcow2 100G
```

### Image Information

```bash
# Show image info
qemu-img info disk.qcow2

# Output:
# image: disk.qcow2
# file format: qcow2
# virtual size: 20 GiB (21474836480 bytes)
# disk size: 196 KiB
# cluster_size: 65536
# Format specific information:
#     compat: 1.1
#     compression type: zlib
#     lazy refcounts: false
#     refcount bits: 16
#     corrupt: false

# JSON output
qemu-img info --output=json disk.qcow2

# Check for errors
qemu-img check disk.qcow2

# Detailed check with repair
qemu-img check -r all disk.qcow2
```

### Converting Images

```bash
# qcow2 to raw
qemu-img convert -f qcow2 -O raw disk.qcow2 disk.raw

# raw to qcow2
qemu-img convert -f raw -O qcow2 disk.raw disk.qcow2

# With compression
qemu-img convert -c -f raw -O qcow2 disk.raw disk.qcow2

# VirtualBox VDI to qcow2
qemu-img convert -f vdi -O qcow2 disk.vdi disk.qcow2

# VMware VMDK to qcow2
qemu-img convert -f vmdk -O qcow2 disk.vmdk disk.qcow2

# Progress indicator
qemu-img convert -p -f raw -O qcow2 disk.raw disk.qcow2
```

### Resizing Images

```bash
# Increase size by 10GB
qemu-img resize disk.qcow2 +10G

# Set to specific size
qemu-img resize disk.qcow2 50G

# Shrink (DANGEROUS - ensure filesystem is shrunk first!)
qemu-img resize --shrink disk.qcow2 30G

# After resizing, grow filesystem in guest:
# Linux: sudo resize2fs /dev/vda1
# Or use: sudo growpart /dev/vda 1 && sudo resize2fs /dev/vda1
```

### Backing Files (Copy-on-Write)

```bash
# Create base image
qemu-img create -f qcow2 base.qcow2 20G
# ... install OS ...

# Create overlay that only stores differences
qemu-img create -f qcow2 -b base.qcow2 -F qcow2 overlay.qcow2

# Create another overlay (for different config)
qemu-img create -f qcow2 -b base.qcow2 -F qcow2 overlay2.qcow2

# View backing file chain
qemu-img info --backing-chain overlay.qcow2

# Commit overlay changes to base (merge)
qemu-img commit overlay.qcow2

# Rebase to new backing file
qemu-img rebase -b new-base.qcow2 -F qcow2 overlay.qcow2
```

### Snapshots

```bash
# Create snapshot
qemu-img snapshot -c snap1 disk.qcow2

# List snapshots
qemu-img snapshot -l disk.qcow2

# Apply (revert to) snapshot
qemu-img snapshot -a snap1 disk.qcow2

# Delete snapshot
qemu-img snapshot -d snap1 disk.qcow2
```

---

## Basic VM Creation

### Minimal Examples

#### Linux x86_64

```bash
# Boot Linux ISO
qemu-system-x86_64 \
  -m 2G \
  -cdrom ubuntu-22.04.iso \
  -boot d

# Boot existing disk
qemu-system-x86_64 \
  -m 2G \
  -hda disk.qcow2
```

#### Linux ARM64

```bash
# ARM64 requires machine type and firmware
qemu-system-aarch64 \
  -M virt \
  -cpu cortex-a72 \
  -m 2G \
  -bios /usr/share/qemu-efi-aarch64/QEMU_EFI.fd \
  -cdrom ubuntu-arm64.iso \
  -boot d
```

#### Windows x86_64

```bash
qemu-system-x86_64 \
  -m 4G \
  -cdrom Win11_x64.iso \
  -hda windows.qcow2 \
  -boot d
```

### Step-by-Step: Installing Ubuntu x86_64

```bash
# Step 1: Create disk
qemu-img create -f qcow2 ubuntu.qcow2 30G

# Step 2: Boot installer
qemu-system-x86_64 \
  -enable-kvm \
  -m 4G \
  -smp 4 \
  -cpu host \
  -drive file=ubuntu.qcow2,format=qcow2 \
  -cdrom ubuntu-22.04-desktop-amd64.iso \
  -boot d \
  -vga virtio \
  -display gtk

# Step 3: After installation, boot from disk
qemu-system-x86_64 \
  -enable-kvm \
  -m 4G \
  -smp 4 \
  -cpu host \
  -drive file=ubuntu.qcow2,format=qcow2 \
  -vga virtio \
  -display gtk
```

### Step-by-Step: Installing Ubuntu ARM64 (on x86_64 host)

```bash
# Step 1: Download UEFI firmware
# Ubuntu/Debian:
sudo apt install qemu-efi-aarch64
# Path: /usr/share/qemu-efi-aarch64/QEMU_EFI.fd

# macOS:
brew install qemu
# Firmware included

# Step 2: Create disk and UEFI vars
qemu-img create -f qcow2 ubuntu-arm64.qcow2 30G
cp /usr/share/qemu-efi-aarch64/QEMU_EFI.fd UEFI.fd
truncate -s 64M UEFI_VARS.fd

# Step 3: Boot installer
qemu-system-aarch64 \
  -M virt \
  -cpu cortex-a72 \
  -m 4G \
  -smp 4 \
  -drive if=pflash,format=raw,file=UEFI.fd,readonly=on \
  -drive if=pflash,format=raw,file=UEFI_VARS.fd \
  -drive file=ubuntu-arm64.qcow2,format=qcow2,if=virtio \
  -cdrom ubuntu-22.04-live-server-arm64.iso \
  -boot d \
  -device virtio-gpu-pci \
  -device usb-ehci \
  -device usb-kbd \
  -device usb-mouse \
  -display gtk

# Step 4: After installation, boot from disk
qemu-system-aarch64 \
  -M virt \
  -cpu cortex-a72 \
  -m 4G \
  -smp 4 \
  -drive if=pflash,format=raw,file=UEFI.fd,readonly=on \
  -drive if=pflash,format=raw,file=UEFI_VARS.fd \
  -drive file=ubuntu-arm64.qcow2,format=qcow2,if=virtio \
  -device virtio-gpu-pci \
  -device usb-ehci \
  -device usb-kbd \
  -device usb-mouse \
  -display gtk
```

---

## CPU and Memory

### Memory Options

```bash
# Basic memory
-m 4G                    # 4 gigabytes
-m 4096M                 # 4096 megabytes
-m 4096                  # 4096 megabytes (default unit)

# Memory with slots for hotplug
-m 4G,slots=4,maxmem=16G

# NUMA configuration
-m 8G \
-object memory-backend-ram,id=mem0,size=4G \
-object memory-backend-ram,id=mem1,size=4G \
-numa node,memdev=mem0,cpus=0-1,nodeid=0 \
-numa node,memdev=mem1,cpus=2-3,nodeid=1

# Huge pages (performance)
-object memory-backend-file,id=mem,size=4G,mem-path=/dev/hugepages,share=on \
-numa node,memdev=mem
```

### CPU Options

```bash
# Number of CPUs
-smp 4                   # 4 CPUs

# Detailed topology
-smp 8,sockets=2,cores=2,threads=2
# Creates: 2 sockets × 2 cores × 2 threads = 8 CPUs

# Maximum CPUs for hotplug
-smp 4,maxcpus=8

# CPU model
-cpu host               # Use host CPU (requires KVM/HVF)
-cpu max                # Best CPU QEMU can emulate
-cpu qemu64             # Generic x86_64
-cpu Haswell            # Intel Haswell
-cpu EPYC               # AMD EPYC
-cpu cortex-a72         # ARM Cortex-A72
-cpu cortex-a53         # ARM Cortex-A53

# List available CPU models
qemu-system-x86_64 -cpu help
qemu-system-aarch64 -cpu help

# CPU features
-cpu host,+avx2,+aes           # Add features
-cpu host,-avx512f             # Remove features
-cpu Haswell,+vmx,+svm         # Enable virtualization

# View CPU features
qemu-system-x86_64 -cpu Haswell,help
```

### CPU Pinning (Linux)

```bash
# Pin QEMU vCPUs to physical CPUs
taskset -c 0-3 qemu-system-x86_64 ...

# Or use cgroups
sudo cgcreate -g cpuset:qemu
sudo cgset -r cpuset.cpus=0-3 qemu
sudo cgexec -g cpuset:qemu qemu-system-x86_64 ...
```

---

## Storage Devices

### Drive Types

| Interface | Use Case | Performance |
|-----------|----------|-------------|
| `ide` | Legacy, CD-ROM | Slow |
| `sata` | Modern compatibility | Medium |
| `scsi` | Server workloads | Good |
| `virtio` | Best performance | Excellent |
| `nvme` | NVMe emulation | Excellent |

### Basic Drive Options

```bash
# Simple (deprecated but works)
-hda disk.qcow2
-hdb second.qcow2
-hdc data.qcow2
-hdd fourth.qcow2
-cdrom install.iso

# Modern syntax
-drive file=disk.qcow2,format=qcow2,if=virtio
-drive file=data.qcow2,format=qcow2,if=virtio,index=1

# Read-only
-drive file=disk.qcow2,format=qcow2,if=virtio,readonly=on

# Snapshot mode (changes discarded on exit)
-drive file=disk.qcow2,format=qcow2,if=virtio,snapshot=on

# Cache modes
-drive file=disk.qcow2,format=qcow2,cache=none      # Direct I/O
-drive file=disk.qcow2,format=qcow2,cache=writeback # Default
-drive file=disk.qcow2,format=qcow2,cache=writethrough
-drive file=disk.qcow2,format=qcow2,cache=unsafe    # Fastest, unsafe
```

### VirtIO Drives (Recommended)

```bash
# VirtIO block device
-drive file=disk.qcow2,format=qcow2,if=none,id=drive0 \
-device virtio-blk-pci,drive=drive0,bootindex=1

# Multiple drives
-drive file=system.qcow2,format=qcow2,if=none,id=drive0 \
-device virtio-blk-pci,drive=drive0,bootindex=1 \
-drive file=data.qcow2,format=qcow2,if=none,id=drive1 \
-device virtio-blk-pci,drive=drive1

# With I/O thread (better performance)
-object iothread,id=io1 \
-drive file=disk.qcow2,format=qcow2,if=none,id=drive0,aio=native,cache.direct=on \
-device virtio-blk-pci,drive=drive0,iothread=io1
```

### SCSI Drives

```bash
# Add SCSI controller
-device virtio-scsi-pci,id=scsi0

# Add SCSI disk
-drive file=disk.qcow2,format=qcow2,if=none,id=drive0 \
-device scsi-hd,bus=scsi0.0,drive=drive0

# SCSI CD-ROM
-drive file=install.iso,format=raw,if=none,id=cd0,media=cdrom \
-device scsi-cd,bus=scsi0.0,drive=cd0
```

### NVMe Drives

```bash
# NVMe controller with namespace
-drive file=disk.qcow2,format=qcow2,if=none,id=nvme0 \
-device nvme,serial=deadbeef,drive=nvme0

# Multiple NVMe namespaces
-device nvme,id=nvme0,serial=1234 \
-drive file=disk1.qcow2,format=qcow2,if=none,id=ns1 \
-device nvme-ns,drive=ns1,bus=nvme0,nsid=1 \
-drive file=disk2.qcow2,format=qcow2,if=none,id=ns2 \
-device nvme-ns,drive=ns2,bus=nvme0,nsid=2
```

### CD-ROM and Floppy

```bash
# CD-ROM
-cdrom install.iso

# Or explicitly
-drive file=install.iso,media=cdrom,if=ide

# Empty CD-ROM (for hot-swap)
-drive if=none,id=cd0,media=cdrom \
-device ide-cd,drive=cd0

# Floppy disk
-fda floppy.img
-drive file=floppy.img,if=floppy,format=raw
```

### Boot Order

```bash
# Boot from CD first, then disk
-boot order=dc

# Boot menu
-boot menu=on,splash-time=5000

# Boot order with timeout
-boot order=cdn,once=d

# Per-device boot priority
-device virtio-blk-pci,drive=drive0,bootindex=1
-device virtio-net-pci,netdev=net0,bootindex=2
```

---

## Network Configuration

### Network Backends (netdev)

| Type | Description | Performance | Isolation |
|------|-------------|-------------|-----------|
| `user` | NAT, no root needed | Medium | High |
| `tap` | Bridge, requires root | Excellent | Low |
| `bridge` | Like tap, simpler | Excellent | Low |
| `socket` | QEMU-to-QEMU | Varies | Medium |
| `vde` | Virtual Distributed Ethernet | Good | Medium |

### User-Mode Networking (NAT)

Easiest setup, no root required:

```bash
# Basic user network
-netdev user,id=net0 \
-device virtio-net-pci,netdev=net0

# With port forwarding
-netdev user,id=net0,hostfwd=tcp::2222-:22,hostfwd=tcp::8080-:80 \
-device virtio-net-pci,netdev=net0

# Custom network settings
-netdev user,id=net0,net=192.168.100.0/24,dhcpstart=192.168.100.10,host=192.168.100.2 \
-device virtio-net-pci,netdev=net0

# With DNS
-netdev user,id=net0,dns=8.8.8.8 \
-device virtio-net-pci,netdev=net0

# SMB sharing (Windows guests)
-netdev user,id=net0,smb=/path/to/share \
-device virtio-net-pci,netdev=net0

# TFTP server (for PXE boot)
-netdev user,id=net0,tftp=/path/to/tftp,bootfile=pxelinux.0 \
-device virtio-net-pci,netdev=net0

# Multiple port forwards
-netdev user,id=net0,\
hostfwd=tcp::2222-:22,\
hostfwd=tcp::3389-:3389,\
hostfwd=tcp::5900-:5900,\
hostfwd=udp::5353-:5353 \
-device virtio-net-pci,netdev=net0
```

#### Connecting to Guest via SSH

```bash
# Start VM with port forwarding
qemu-system-x86_64 ... \
  -netdev user,id=net0,hostfwd=tcp::2222-:22 \
  -device virtio-net-pci,netdev=net0

# Connect from host
ssh -p 2222 user@localhost
```

### TAP Networking (Bridge)

Full network access, requires root:

#### Linux Setup

```bash
# Create bridge (one-time setup)
sudo ip link add br0 type bridge
sudo ip link set br0 up
sudo ip addr add 192.168.100.1/24 dev br0

# Enable IP forwarding
sudo sysctl -w net.ipv4.ip_forward=1

# NAT for internet access
sudo iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
sudo iptables -A FORWARD -i br0 -o eth0 -j ACCEPT
sudo iptables -A FORWARD -i eth0 -o br0 -m state --state RELATED,ESTABLISHED -j ACCEPT

# Create TAP device
sudo ip tuntap add dev tap0 mode tap user $(whoami)
sudo ip link set tap0 up
sudo ip link set tap0 master br0

# Run QEMU with TAP
qemu-system-x86_64 ... \
  -netdev tap,id=net0,ifname=tap0,script=no,downscript=no \
  -device virtio-net-pci,netdev=net0,mac=52:54:00:12:34:56
```

#### QEMU Bridge Helper

Simpler approach using qemu-bridge-helper:

```bash
# Setup (one-time)
sudo mkdir -p /etc/qemu
echo "allow br0" | sudo tee /etc/qemu/bridge.conf
sudo chmod 4755 /usr/lib/qemu/qemu-bridge-helper

# Use in QEMU
qemu-system-x86_64 ... \
  -netdev bridge,id=net0,br=br0 \
  -device virtio-net-pci,netdev=net0
```

### macOS Network Setup

```bash
# User-mode networking (easiest)
-netdev user,id=net0,hostfwd=tcp::2222-:22 \
-device virtio-net-pci,netdev=net0

# vmnet (requires root, macOS specific)
# Requires vmnet.framework entitlements
sudo qemu-system-aarch64 ... \
  -netdev vmnet-shared,id=net0 \
  -device virtio-net-pci,netdev=net0
```

### Network Device Types

```bash
# VirtIO (best performance, requires driver)
-device virtio-net-pci,netdev=net0

# Intel E1000 (widely compatible)
-device e1000,netdev=net0

# Intel E1000e (newer)
-device e1000e,netdev=net0

# Realtek RTL8139 (legacy)
-device rtl8139,netdev=net0

# List available network devices
qemu-system-x86_64 -device help | grep -i net
```

### MAC Address

```bash
# Custom MAC address
-device virtio-net-pci,netdev=net0,mac=52:54:00:12:34:56

# Generate random MAC
# Format: 52:54:00:XX:XX:XX (QEMU OUI)
```

### Multiple Network Interfaces

```bash
# Two NICs
-netdev user,id=net0,hostfwd=tcp::2222-:22 \
-device virtio-net-pci,netdev=net0,mac=52:54:00:00:00:01 \
-netdev user,id=net1,net=192.168.2.0/24 \
-device virtio-net-pci,netdev=net1,mac=52:54:00:00:00:02
```

### Socket Networking (VM-to-VM)

```bash
# VM 1 (listen)
qemu-system-x86_64 ... \
  -netdev socket,id=net0,listen=:1234 \
  -device virtio-net-pci,netdev=net0

# VM 2 (connect)
qemu-system-x86_64 ... \
  -netdev socket,id=net0,connect=127.0.0.1:1234 \
  -device virtio-net-pci,netdev=net0

# Multicast (multiple VMs)
qemu-system-x86_64 ... \
  -netdev socket,id=net0,mcast=230.0.0.1:1234 \
  -device virtio-net-pci,netdev=net0
```

---

## Display and Graphics

### Display Backends

| Backend | Platform | Description |
|---------|----------|-------------|
| `gtk` | Linux, macOS | GTK window |
| `cocoa` | macOS | Native macOS |
| `sdl` | All | SDL window |
| `vnc` | All | VNC server |
| `spice` | Linux | SPICE protocol |
| `none` | All | No display |
| `curses` | All | Text mode in terminal |

### Display Options

```bash
# GTK (Linux/macOS)
-display gtk
-display gtk,gl=on          # With OpenGL

# Cocoa (macOS native)
-display cocoa

# SDL
-display sdl
-display sdl,gl=on

# VNC
-display vnc=:1             # Listen on :5901
-display vnc=127.0.0.1:1    # Localhost only
-display vnc=:1,password=on # Password protected

# No display (headless)
-display none
-nographic                  # Also redirects serial to stdio

# Curses (text mode)
-display curses
```

### VGA Devices

```bash
# Standard VGA
-vga std

# Cirrus (legacy, small memory)
-vga cirrus

# VMware SVGA
-vga vmware

# QXL (with SPICE)
-vga qxl

# VirtIO GPU (best performance)
-vga virtio
-device virtio-vga
-device virtio-vga-gl        # With OpenGL

# None (for -nographic)
-vga none

# VirtIO GPU (explicit, more options)
-device virtio-gpu-pci
-device virtio-gpu-pci,virgl=on   # 3D acceleration
```

### OpenGL Acceleration

```bash
# GTK with OpenGL
-display gtk,gl=on \
-device virtio-vga-gl

# SDL with OpenGL
-display sdl,gl=on \
-device virtio-vga-gl

# macOS (requires special build)
-display cocoa,gl=es \
-device virtio-gpu-pci
```

### Multiple Monitors

```bash
# Two monitors with QXL
-device qxl-vga,id=video0 \
-device qxl,id=video1

# VNC with two displays
-display vnc=:1 \
-device virtio-gpu-pci,max_outputs=2
```

### SPICE (Linux)

SPICE provides better performance than VNC:

```bash
# SPICE server
-spice port=5930,disable-ticketing=on \
-device virtio-serial-pci \
-device virtserialport,chardev=spicechannel0,name=com.redhat.spice.0 \
-chardev spicevmc,id=spicechannel0,name=vdagent \
-device qxl-vga

# Connect with spice client
spicy -h 127.0.0.1 -p 5930

# Or remote-viewer
remote-viewer spice://127.0.0.1:5930
```

### Framebuffer and Serial Console

```bash
# Serial console only (no graphics)
-nographic

# With explicit serial
-nographic \
-serial mon:stdio

# Separate serial and monitor
-serial stdio \
-monitor telnet:127.0.0.1:4444,server,nowait
```

---

## Input Devices

### Keyboard and Mouse

```bash
# USB tablet (recommended, absolute positioning)
-device usb-tablet

# USB keyboard and mouse
-device usb-kbd
-device usb-mouse

# VirtIO input (best performance)
-device virtio-keyboard-pci
-device virtio-mouse-pci
-device virtio-tablet-pci

# PS/2 (default, legacy)
# No explicit option needed

# Combined USB setup
-device usb-ehci,id=usb \
-device usb-tablet,bus=usb.0
```

### USB Controller Types

```bash
# EHCI (USB 2.0)
-device usb-ehci,id=usb

# XHCI (USB 3.0)
-device qemu-xhci,id=usb

# UHCI (USB 1.1, legacy)
-device ich9-usb-uhci1,id=usb

# Multiple controllers
-device qemu-xhci,id=usb3 \
-device usb-ehci,id=usb2
```

### Keyboard Layout

```bash
# Set keyboard layout
-k en-us
-k de
-k fr

# List available layouts
ls /usr/share/qemu/keymaps/
```

---

## Audio

### Audio Backends

```bash
# PulseAudio
-audiodev pa,id=snd0
-device ich9-intel-hda -device hda-output,audiodev=snd0

# ALSA (Linux)
-audiodev alsa,id=snd0
-device ich9-intel-hda -device hda-output,audiodev=snd0

# CoreAudio (macOS)
-audiodev coreaudio,id=snd0
-device ich9-intel-hda -device hda-output,audiodev=snd0

# SDL audio
-audiodev sdl,id=snd0
-device ich9-intel-hda -device hda-output,audiodev=snd0

# With input (microphone)
-device ich9-intel-hda \
-device hda-duplex,audiodev=snd0
```

### Audio Device Types

```bash
# Intel HDA (recommended)
-device ich9-intel-hda -device hda-output,audiodev=snd0

# AC97 (legacy)
-device AC97,audiodev=snd0

# Intel ICH6 (older)
-device intel-hda -device hda-output,audiodev=snd0
```

---

## USB Devices

### USB Passthrough

```bash
# List host USB devices
lsusb
# Example output:
# Bus 001 Device 005: ID 1234:5678 Example Device

# Passthrough by vendor:product
-device usb-host,vendorid=0x1234,productid=0x5678

# Passthrough by bus:address
-device usb-host,hostbus=1,hostaddr=5

# Passthrough with USB 3.0
-device qemu-xhci,id=usb \
-device usb-host,bus=usb.0,vendorid=0x1234,productid=0x5678
```

### USB Storage

```bash
# USB flash drive from image
-drive if=none,id=usbdrive,file=usb.img,format=raw \
-device usb-storage,drive=usbdrive

# USB CD-ROM
-drive if=none,id=usbcd,file=cd.iso,media=cdrom \
-device usb-storage,drive=usbcd
```

---

## UEFI and BIOS

### UEFI Firmware

```bash
# x86_64 UEFI (OVMF)
# Ubuntu/Debian: apt install ovmf
# Path: /usr/share/OVMF/OVMF_CODE.fd

-drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd \
-drive if=pflash,format=raw,file=OVMF_VARS.fd

# Create writable vars copy
cp /usr/share/OVMF/OVMF_VARS.fd my_VARS.fd

# ARM64 UEFI
# Path: /usr/share/qemu-efi-aarch64/QEMU_EFI.fd
-drive if=pflash,format=raw,readonly=on,file=/usr/share/qemu-efi-aarch64/QEMU_EFI.fd \
-drive if=pflash,format=raw,file=UEFI_VARS.fd

# With Secure Boot
-drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.secboot.fd \
-drive if=pflash,format=raw,file=OVMF_VARS.secboot.fd \
-machine q35,smm=on \
-global driver=cfi.pflash01,property=secure,value=on
```

### BIOS

```bash
# Custom BIOS
-bios /path/to/bios.bin

# SeaBIOS (default for x86)
# Usually automatic

# List available firmwares
ls /usr/share/seabios/
ls /usr/share/OVMF/
```

---

## Shared Folders

### 9P Filesystem (VirtFS)

```bash
# Share host directory with guest
-virtfs local,path=/host/path,mount_tag=hostshare,security_model=mapped-xattr,id=host0

# Security models:
# - passthrough: Direct permissions (requires root)
# - mapped-xattr: Stores permissions in xattrs
# - mapped-file: Stores permissions in files
# - none: No permission mapping

# In guest (Linux):
sudo mkdir /mnt/shared
sudo mount -t 9p -o trans=virtio hostshare /mnt/shared

# Auto-mount in guest /etc/fstab:
hostshare /mnt/shared 9p trans=virtio,version=9p2000.L 0 0
```

### virtiofs (Better Performance)

```bash
# Start virtiofsd daemon first
/usr/lib/qemu/virtiofsd \
  --socket-path=/tmp/vhost.sock \
  --shared-dir=/host/path \
  --cache=auto

# QEMU command
-chardev socket,id=char0,path=/tmp/vhost.sock \
-device vhost-user-fs-pci,chardev=char0,tag=myfs \
-object memory-backend-memfd,id=mem,size=4G,share=on \
-numa node,memdev=mem

# In guest:
sudo mount -t virtiofs myfs /mnt/shared
```

### SMB/CIFS (Windows Guests)

```bash
# With user-mode networking
-netdev user,id=net0,smb=/path/to/share \
-device virtio-net-pci,netdev=net0

# In Windows guest:
# Access via \\10.0.2.4\qemu
```

---

## Snapshots and Backing Files

### Internal Snapshots

```bash
# Via QEMU monitor (runtime)
# Press Ctrl+Alt+2 to enter monitor

(qemu) savevm snap1
(qemu) loadvm snap1
(qemu) delvm snap1
(qemu) info snapshots

# Via qemu-img (offline)
qemu-img snapshot -c snap1 disk.qcow2
qemu-img snapshot -l disk.qcow2
qemu-img snapshot -a snap1 disk.qcow2
qemu-img snapshot -d snap1 disk.qcow2
```

### External Snapshots

```bash
# Create external snapshot
qemu-img create -f qcow2 -b base.qcow2 -F qcow2 snapshot.qcow2

# Boot from snapshot
qemu-system-x86_64 -drive file=snapshot.qcow2,format=qcow2 ...

# Commit changes back to base
qemu-img commit snapshot.qcow2

# Create snapshot chain
qemu-img create -f qcow2 -b base.qcow2 -F qcow2 snap1.qcow2
qemu-img create -f qcow2 -b snap1.qcow2 -F qcow2 snap2.qcow2
# base.qcow2 <- snap1.qcow2 <- snap2.qcow2
```

### Live Block Operations

```bash
# In QEMU monitor:

# Create live snapshot
(qemu) snapshot_blkdev drive0 snapshot.qcow2 qcow2

# Live commit
(qemu) block-commit drive0

# Live backup
(qemu) drive_backup drive0 /backup/disk.qcow2 qcow2
```

---

## Hardware Acceleration

### KVM (Linux)

```bash
# Check KVM availability
ls -la /dev/kvm
lsmod | grep kvm

# Enable KVM
-enable-kvm

# Or explicit accelerator
-accel kvm

# With fallback
-accel kvm:tcg

# CPU options with KVM
-enable-kvm -cpu host

# Nested virtualization
-enable-kvm -cpu host,+vmx   # Intel
-enable-kvm -cpu host,+svm   # AMD
```

### HVF (macOS)

```bash
# Hypervisor.framework (Apple Silicon or Intel Mac)
-accel hvf

# With CPU
-accel hvf -cpu host

# For Apple Silicon ARM64
qemu-system-aarch64 -accel hvf -cpu host -M virt ...
```

### WHPX (Windows)

```bash
# Windows Hypervisor Platform
-accel whpx

# With Hyper-V
-accel whpx -cpu host
```

### TCG (Software)

```bash
# Software emulation (default, slowest)
-accel tcg

# Multi-threaded TCG
-accel tcg,thread=multi

# Single-threaded
-accel tcg,thread=single

# TCG specific options
-accel tcg,tb-size=2048    # Translation buffer size in MB
```

### Checking Accelerator

```bash
# In QEMU monitor
(qemu) info kvm
# kvm support: enabled

# From command line
qemu-system-x86_64 -accel help
# Accelerators supported in QEMU binary:
# tcg
# kvm

# Check in running VM
cat /sys/module/kvm/parameters/*
```

---

## Complete Examples

### Production-Ready Linux VM

```bash
#!/bin/bash
# linux-vm.sh

DISK="ubuntu.qcow2"
ISO="ubuntu-22.04-desktop-amd64.iso"
MEM="8G"
CPUS="4"
SSH_PORT="2222"

# Create disk if doesn't exist
if [ ! -f "$DISK" ]; then
    qemu-img create -f qcow2 "$DISK" 50G
    BOOT="-cdrom $ISO -boot d"
else
    BOOT=""
fi

qemu-system-x86_64 \
    -name "Ubuntu VM" \
    -enable-kvm \
    -machine q35,accel=kvm \
    -cpu host \
    -smp "$CPUS" \
    -m "$MEM" \
    \
    `# Storage` \
    -drive file="$DISK",format=qcow2,if=virtio,cache=writeback \
    $BOOT \
    \
    `# Network` \
    -netdev user,id=net0,hostfwd=tcp::"$SSH_PORT"-:22 \
    -device virtio-net-pci,netdev=net0 \
    \
    `# Display` \
    -vga virtio \
    -display gtk,gl=on \
    \
    `# Input` \
    -device usb-ehci \
    -device usb-tablet \
    \
    `# Audio` \
    -audiodev pa,id=snd0 \
    -device ich9-intel-hda \
    -device hda-output,audiodev=snd0 \
    \
    `# Misc` \
    -device virtio-balloon-pci \
    -device virtio-rng-pci \
    "$@"
```

### Windows 11 VM

```bash
#!/bin/bash
# windows-vm.sh

qemu-system-x86_64 \
    -name "Windows 11" \
    -enable-kvm \
    -machine q35,accel=kvm \
    -cpu host,+hypervisor,+invtsc \
    -smp 4,sockets=1,cores=4,threads=1 \
    -m 8G \
    \
    `# UEFI` \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd \
    -drive if=pflash,format=raw,file=win_VARS.fd \
    \
    `# TPM 2.0 (required for Windows 11)` \
    -chardev socket,id=chrtpm,path=/tmp/tpm/swtpm-sock \
    -tpmdev emulator,id=tpm0,chardev=chrtpm \
    -device tpm-tis,tpmdev=tpm0 \
    \
    `# Storage` \
    -drive file=windows.qcow2,format=qcow2,if=virtio \
    -drive file=virtio-win.iso,media=cdrom \
    \
    `# Network` \
    -netdev user,id=net0,hostfwd=tcp::3389-:3389 \
    -device virtio-net-pci,netdev=net0 \
    \
    `# Display` \
    -device qxl-vga,vgamem_mb=64 \
    -display gtk \
    \
    `# USB` \
    -device qemu-xhci,id=usb \
    -device usb-tablet \
    \
    "$@"

# Start TPM emulator first:
# mkdir -p /tmp/tpm
# swtpm socket --tpmstate dir=/tmp/tpm --ctrl type=unixio,path=/tmp/tpm/swtpm-sock --tpm2
```

### ARM64 Linux VM (on x86_64 host)

```bash
#!/bin/bash
# arm64-vm.sh

qemu-system-aarch64 \
    -name "Ubuntu ARM64" \
    -machine virt \
    -cpu cortex-a72 \
    -smp 4 \
    -m 4G \
    \
    `# UEFI` \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/qemu-efi-aarch64/QEMU_EFI.fd \
    -drive if=pflash,format=raw,file=aarch64_VARS.fd \
    \
    `# Storage` \
    -drive file=ubuntu-arm64.qcow2,format=qcow2,if=virtio \
    \
    `# Network` \
    -netdev user,id=net0,hostfwd=tcp::2222-:22 \
    -device virtio-net-pci,netdev=net0 \
    \
    `# Display` \
    -device virtio-gpu-pci \
    -display gtk \
    \
    `# USB` \
    -device qemu-xhci,id=usb \
    -device usb-kbd \
    -device usb-tablet \
    \
    "$@"
```

### Headless Server

```bash
#!/bin/bash
# headless-server.sh

qemu-system-x86_64 \
    -name "Headless Server" \
    -enable-kvm \
    -cpu host \
    -smp 2 \
    -m 2G \
    -drive file=server.qcow2,format=qcow2,if=virtio \
    -netdev user,id=net0,hostfwd=tcp::2222-:22 \
    -device virtio-net-pci,netdev=net0 \
    -nographic \
    -serial mon:stdio \
    -pidfile /tmp/qemu-server.pid \
    -daemonize

# Connect via serial
# screen /dev/pts/X

# Or SSH
# ssh -p 2222 user@localhost

# Kill with
# kill $(cat /tmp/qemu-server.pid)
```

### macOS VM (on Apple Silicon)

```bash
#!/bin/bash
# macos-arm64.sh (requires macOS host)

qemu-system-aarch64 \
    -accel hvf \
    -machine virt,highmem=on \
    -cpu host \
    -smp 4 \
    -m 8G \
    \
    `# Storage` \
    -drive file=linux.qcow2,format=qcow2,if=virtio \
    \
    `# Network` \
    -netdev user,id=net0,hostfwd=tcp::2222-:22 \
    -device virtio-net-pci,netdev=net0 \
    \
    `# Display` \
    -device virtio-gpu-pci \
    -display cocoa \
    \
    `# USB` \
    -device qemu-xhci \
    -device usb-kbd \
    -device usb-tablet
```

---

## QEMU Monitor

The QEMU monitor provides runtime control of the VM.

### Accessing the Monitor

```bash
# Switch in graphical mode
# Ctrl+Alt+2 (monitor), Ctrl+Alt+1 (display)

# Via telnet
-monitor telnet:127.0.0.1:4444,server,nowait
# Then: telnet 127.0.0.1 4444

# Via stdio
-monitor stdio

# Via Unix socket
-monitor unix:/tmp/qemu-monitor.sock,server,nowait
# Then: socat - unix-connect:/tmp/qemu-monitor.sock
```

### Common Monitor Commands

```bash
# System
(qemu) info version          # QEMU version
(qemu) info status           # VM status
(qemu) info cpus             # CPU info
(qemu) info kvm              # KVM status
(qemu) system_powerdown      # ACPI poweroff
(qemu) system_reset          # Hard reset
(qemu) quit                  # Exit QEMU

# Snapshots
(qemu) savevm name           # Create snapshot
(qemu) loadvm name           # Load snapshot
(qemu) delvm name            # Delete snapshot
(qemu) info snapshots        # List snapshots

# Devices
(qemu) info block            # Block devices
(qemu) info network          # Network
(qemu) info usb              # USB devices
(qemu) info pci              # PCI devices

# Block operations
(qemu) change drive0 /path/to/new.iso    # Change CD
(qemu) eject drive0                       # Eject CD
(qemu) drive_add 0 file=/path/to/disk.qcow2,if=none,id=drive1

# USB
(qemu) device_add usb-host,vendorid=0x1234,productid=0x5678
(qemu) device_del usb1

# Network
(qemu) set_link net0 off     # Disable NIC
(qemu) set_link net0 on      # Enable NIC

# Misc
(qemu) screendump file.ppm   # Screenshot
(qemu) sendkey ctrl-alt-delete
(qemu) mouse_button 1        # Left click
(qemu) mouse_move 100 100    # Move mouse
```

### QMP (QEMU Machine Protocol)

Programmatic interface (JSON):

```bash
# Enable QMP
-qmp unix:/tmp/qemu-qmp.sock,server,nowait

# Connect
echo '{"execute":"qmp_capabilities"}' | socat - unix-connect:/tmp/qemu-qmp.sock

# Query status
echo '{"execute":"query-status"}' | socat - unix-connect:/tmp/qemu-qmp.sock

# Poweroff
echo '{"execute":"system_powerdown"}' | socat - unix-connect:/tmp/qemu-qmp.sock
```

---

## Scripting and Automation

### Wrapper Script Template

```bash
#!/bin/bash
set -e

# Configuration
VM_NAME="${VM_NAME:-myvm}"
DISK="${DISK:-disk.qcow2}"
MEM="${MEM:-4G}"
CPUS="${CPUS:-4}"
SSH_PORT="${SSH_PORT:-2222}"
VNC_PORT="${VNC_PORT:-5900}"

# Paths
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
VM_DIR="$SCRIPT_DIR/vms/$VM_NAME"

# Create VM directory
mkdir -p "$VM_DIR"
cd "$VM_DIR"

# Check for KVM
ACCEL=""
if [ -e /dev/kvm ]; then
    ACCEL="-enable-kvm -cpu host"
elif [ "$(uname)" = "Darwin" ]; then
    ACCEL="-accel hvf -cpu host"
fi

# Build command
CMD=(
    qemu-system-x86_64
    -name "$VM_NAME"
    $ACCEL
    -smp "$CPUS"
    -m "$MEM"
    -drive "file=$DISK,format=qcow2,if=virtio"
    -netdev "user,id=net0,hostfwd=tcp::${SSH_PORT}-:22"
    -device virtio-net-pci,netdev=net0
    -display vnc=:$((VNC_PORT - 5900))
    -monitor unix:"$VM_DIR/monitor.sock",server,nowait
    -pidfile "$VM_DIR/qemu.pid"
)

# Add extra arguments
CMD+=("$@")

# Run
echo "Starting $VM_NAME..."
echo "SSH: ssh -p $SSH_PORT localhost"
echo "VNC: localhost:$VNC_PORT"
"${CMD[@]}"
```

### systemd Service

```ini
# /etc/systemd/system/qemu-myvm.service
[Unit]
Description=QEMU VM - myvm
After=network.target

[Service]
Type=simple
User=qemu
Group=qemu
ExecStart=/usr/bin/qemu-system-x86_64 \
    -name myvm \
    -enable-kvm \
    -cpu host \
    -smp 4 \
    -m 8G \
    -drive file=/var/lib/qemu/myvm/disk.qcow2,format=qcow2,if=virtio \
    -netdev user,id=net0,hostfwd=tcp::2222-:22 \
    -device virtio-net-pci,netdev=net0 \
    -nographic \
    -monitor unix:/run/qemu/myvm-monitor.sock,server,nowait
ExecStop=/bin/sh -c 'echo system_powerdown | socat - unix-connect:/run/qemu/myvm-monitor.sock'
TimeoutStopSec=120
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable qemu-myvm
sudo systemctl start qemu-myvm
```

### Libvirt Integration

```bash
# Install libvirt
sudo apt install libvirt-daemon-system virtinst

# Import existing QEMU VM
virt-install \
    --name myvm \
    --import \
    --disk path=/path/to/disk.qcow2 \
    --memory 4096 \
    --vcpus 4 \
    --os-variant ubuntu22.04

# Manage with virsh
virsh list --all
virsh start myvm
virsh shutdown myvm
virsh console myvm
```

---

## Performance Tuning

### CPU Optimization

```bash
# Use host CPU (best performance)
-cpu host

# Pin vCPUs to physical CPUs
taskset -c 0-3 qemu-system-x86_64 ...

# CPU topology matching
-smp 4,sockets=1,cores=4,threads=1
```

### Memory Optimization

```bash
# Huge pages
# Setup on host:
sudo sysctl vm.nr_hugepages=2048
sudo mount -t hugetlbfs hugetlbfs /dev/hugepages

# QEMU command:
-mem-path /dev/hugepages
-mem-prealloc

# Or:
-object memory-backend-file,id=mem,size=4G,mem-path=/dev/hugepages,share=on \
-numa node,memdev=mem
```

### Disk I/O Optimization

```bash
# Use virtio with cache settings
-drive file=disk.qcow2,format=qcow2,if=virtio,cache=none,aio=native

# Cache modes:
# - none: O_DIRECT, bypasses host cache (best for data)
# - writeback: Uses host cache (better performance)
# - writethrough: Safe, slower
# - unsafe: No flush, fastest, data loss risk

# I/O threads
-object iothread,id=io1 \
-drive file=disk.qcow2,format=qcow2,if=none,id=d0,aio=native,cache.direct=on \
-device virtio-blk-pci,drive=d0,iothread=io1

# Use NVMe for better queue depth
-device nvme,serial=nvme1,drive=d0

# Preallocation for better performance
qemu-img create -f qcow2 -o preallocation=full disk.qcow2 50G
```

### Network Optimization

```bash
# Use virtio
-device virtio-net-pci,netdev=net0

# With vhost (better performance)
-netdev tap,id=net0,vhost=on,script=no \
-device virtio-net-pci,netdev=net0

# Multiple queues
-netdev tap,id=net0,queues=4,vhost=on \
-device virtio-net-pci,netdev=net0,mq=on,vectors=10
```

### Display Optimization

```bash
# VirtIO GPU with 3D
-device virtio-vga-gl \
-display gtk,gl=on

# Or SDL
-device virtio-vga-gl \
-display sdl,gl=on
```

---

## Troubleshooting

### Common Errors

#### "Could not access KVM kernel module"

```bash
# Check KVM module
lsmod | grep kvm

# Load module
sudo modprobe kvm_intel  # or kvm_amd

# Check permissions
ls -la /dev/kvm
# Add user to kvm group
sudo usermod -aG kvm $USER
# Log out and back in
```

#### "Failed to find romfile"

```bash
# Install SeaBIOS/OVMF
sudo apt install seabios ovmf

# Or specify path
-bios /usr/share/seabios/bios.bin
```

#### "VNC server running on..."

```bash
# Connect with VNC client
vncviewer :5900

# Or change display
-display gtk
```

#### "qemu-system-aarch64: Unable to find CPU definition"

```bash
# List available CPUs
qemu-system-aarch64 -cpu help

# Use available CPU
-cpu cortex-a72
# or
-cpu max
```

#### Network Not Working

```bash
# Check inside guest
ip addr
ping 10.0.2.2  # Host (in user-mode)

# Check host firewall
sudo iptables -L

# Try different network
-netdev user,id=net0,net=192.168.100.0/24 \
-device virtio-net-pci,netdev=net0
```

#### Disk Not Detected

```bash
# Check driver
# Linux: needs virtio driver (usually built-in)
# Windows: needs virtio-win drivers

# Use IDE temporarily
-drive file=disk.qcow2,format=qcow2,if=ide
```

### Debug Options

```bash
# Verbose output
-d cpu,int,mmu

# Log to file
-D /tmp/qemu.log

# GDB server
-s -S
# Then: gdb -ex "target remote :1234"

# Trace events
-trace enable=virtio*,file=/tmp/trace.log
```

---

## Complete Option Reference

### Most Common Options

| Option | Description | Example |
|--------|-------------|---------|
| `-m` | Memory size | `-m 4G` |
| `-smp` | CPU count | `-smp 4` |
| `-cpu` | CPU model | `-cpu host` |
| `-enable-kvm` | Enable KVM | |
| `-accel` | Accelerator | `-accel hvf` |
| `-drive` | Add drive | `-drive file=disk.qcow2,format=qcow2` |
| `-cdrom` | CD-ROM | `-cdrom install.iso` |
| `-boot` | Boot order | `-boot d` |
| `-netdev` | Network backend | `-netdev user,id=net0` |
| `-device` | Add device | `-device virtio-net-pci,netdev=net0` |
| `-display` | Display backend | `-display gtk` |
| `-vga` | VGA type | `-vga virtio` |
| `-nographic` | No display | |
| `-serial` | Serial port | `-serial stdio` |
| `-monitor` | Monitor | `-monitor stdio` |
| `-name` | VM name | `-name "My VM"` |
| `-daemonize` | Run in background | |
| `-pidfile` | PID file | `-pidfile /tmp/qemu.pid` |

### Getting Help

```bash
# General help
qemu-system-x86_64 -h

# Device help
qemu-system-x86_64 -device help

# Specific device
qemu-system-x86_64 -device virtio-net-pci,help

# CPU help
qemu-system-x86_64 -cpu help

# Machine help
qemu-system-x86_64 -machine help

# Audio help
qemu-system-x86_64 -audiodev help

# Network help
qemu-system-x86_64 -netdev help
```

---

## Summary

QEMU is an incredibly powerful tool with extensive options. Key takeaways:

1. **Use hardware acceleration** (KVM/HVF/WHPX) whenever possible
2. **VirtIO devices** provide the best performance
3. **qcow2** format gives you snapshots and thin provisioning
4. **User-mode networking** is easiest; TAP is fastest
5. **Monitor** provides runtime control
6. **Scripts and systemd** help manage complex configurations

For most use cases:
- Linux VM: KVM + VirtIO + qcow2
- Windows VM: KVM + VirtIO drivers + UEFI
- ARM emulation: Full system mode (slow)
- Cross-arch CLI testing: User-mode emulation (fast)
