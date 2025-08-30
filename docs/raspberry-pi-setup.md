# Raspberry Pi Setup Guide for ECG Monitor

This guide walks you through setting up the ECG Monitor on a Raspberry Pi from a fresh installation.

## Table of Contents
- [Initial Raspberry Pi Setup](#initial-raspberry-pi-setup)
- [System Configuration](#system-configuration)
- [Installing Dependencies](#installing-dependencies)
- [Hardware Optimization](#hardware-optimization)
- [ECG Monitor Installation](#ecg-monitor-installation)
- [Troubleshooting](#troubleshooting)

## Initial Raspberry Pi Setup

### 1. Prepare the SD Card

**Using Raspberry Pi Imager (Recommended):**
1. Download [Raspberry Pi Imager](https://www.raspberrypi.org/software/)
2. Select "Raspberry Pi OS (64-bit)" - Desktop version recommended
3. Configure advanced options:
   - Enable SSH (if you plan to use remote access)
   - Set username/password
   - Configure WiFi credentials
   - Set locale settings
4. Write to SD card (32GB+ recommended)

**Manual Installation:**
1. Download Raspberry Pi OS from the official website
2. Use `dd` or Etcher to write the image to SD card
3. Enable SSH by creating empty `ssh` file in boot partition

### 2. First Boot Setup

```bash
# Update system packages
sudo apt update && sudo apt upgrade -y

# Install essential tools
sudo apt install -y git curl wget vim nano htop

# Enable additional interfaces (if needed)
sudo raspi-config
# Interface Options → Enable what you need (SPI, I2C, etc.)
```

## System Configuration

### 1. GPU Memory Split

For optimal graphics performance:

```bash
sudo raspi-config
```
- Navigate to: `Advanced Options` → `Memory Split`
- Set to `128` MB (or `64` MB minimum)
- Reboot when prompted

**Alternative command line method:**
```bash
# Check current GPU memory
vcgencmd get_mem gpu

# Set GPU memory to 128MB
echo 'gpu_mem=128' | sudo tee -a /boot/config.txt
sudo reboot
```

### 2. CPU Performance

Enable performance governor for better real-time performance:

```bash
# Check available governors
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors

# Set to performance mode
echo 'performance' | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Make permanent by adding to rc.local
echo 'echo performance | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor' | sudo tee -a /etc/rc.local
```

### 3. Display Configuration

For optimal display performance:

```bash
# Edit boot config
sudo nano /boot/config.txt

# Add/modify these lines for better graphics:
hdmi_group=2
hdmi_mode=82          # 1920x1080 60Hz
hdmi_drive=2          # Normal HDMI mode
config_hdmi_boost=4   # Increase signal strength if needed

# For the official 7" touchscreen:
lcd_rotate=2          # If you need to rotate the display
```

### 4. Audio (Optional)

If you plan to add audio alerts:

```bash
# Enable audio
sudo raspi-config
# Advanced Options → Audio → Force 3.5mm jack (or HDMI)

# Test audio
speaker-test -t sine -f 1000 -l 1
```

## Installing Dependencies

### 1. Install Rust

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version

# Add to shell profile
echo 'source ~/.cargo/env' >> ~/.bashrc
```

### 2. System Libraries

Install required system libraries for graphics and GUI:

```bash
# Essential build tools
sudo apt install -y build-essential pkg-config

# Graphics libraries
sudo apt install -y \
    libgl1-mesa-dev \
    libegl1-mesa-dev \
    libgles2-mesa-dev \
    mesa-common-dev

# X11 libraries
sudo apt install -y \
    libx11-dev \
    libxcursor-dev \
    libxi-dev \
    libxinerama-dev \
    libxrandr-dev \
    libxss-dev \
    libxt-dev \
    libxmu-dev

# Additional libraries for egui/eframe
sudo apt install -y \
    libasound2-dev \
    libudev-dev \
    libxkbcommon-dev \
    libwayland-dev

# Optional: For better font rendering
sudo apt install -y fontconfig-config libfontconfig1-dev
```

### 3. Cross-compilation Support (Optional)

If you want to compile on another machine for Raspberry Pi:

```bash
# On the development machine:
rustup target add aarch64-unknown-linux-gnu

# Install cross-compilation toolchain
sudo apt install gcc-aarch64-linux-gnu

# Configure cargo for cross-compilation
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml << EOF
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF
```

## Hardware Optimization

### 1. Thermal Management

For sustained performance, ensure good cooling:

```bash
# Check CPU temperature
vcgencmd measure_temp

# Monitor temperature continuously
watch -n 1 'vcgencmd measure_temp && cat /sys/class/thermal/thermal_zone0/temp'

# Install temperature monitoring (optional)
sudo apt install -y lm-sensors
sensors
```

**Thermal throttling occurs at:**
- ARM: 80°C (warning), 85°C (throttling)
- GPU: 80°C (warning), 85°C (throttling)

### 2. Power Supply

Ensure adequate power supply:
- **Raspberry Pi 4**: 5V 3A USB-C power supply
- **Raspberry Pi 3**: 5V 2.5A micro-USB power supply

Check for power issues:
```bash
# Check for under-voltage
vcgencmd get_throttled
# Result 0x0 = no throttling
# Non-zero = throttling detected

# Monitor voltage
vcgencmd measure_volts
```

### 3. SD Card Performance

For better I/O performance:

```bash
# Check SD card performance
sudo hdparm -t /dev/mmcblk0

# Use Class 10 or better SD cards
# Consider SSD via USB 3.0 for Raspberry Pi 4
```

## ECG Monitor Installation

### 1. Clone the Repository

```bash
# Clone the ECG monitor
cd ~
git clone <repository_url> ecg3
cd ecg3

# Make the run script executable
chmod +x run_ecg_monitor.sh
```

### 2. Quick Installation

```bash
# Run the automated setup script
./run_ecg_monitor.sh --check

# If everything looks good, run the application
./run_ecg_monitor.sh
```

### 3. Manual Installation

```bash
# Build the application
cargo build --release

# Copy ECG data files (if you have them)
# cp /path/to/your/r01.edf .
# cp /path/to/your/r01.edf.qrs .

# Run the application
./target/release/ecg3
```

### 4. Desktop Integration (Optional)

Create a desktop shortcut:

```bash
cat > ~/Desktop/ECG-Monitor.desktop << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=ECG Monitor
Comment=Real-time ECG monitoring application
Exec=/home/$(whoami)/ecg3/run_ecg_monitor.sh
Icon=applications-science
Path=/home/$(whoami)/ecg3
Terminal=false
StartupNotify=true
Categories=Science;Medical;
EOF

chmod +x ~/Desktop/ECG-Monitor.desktop
```

## Performance Tuning

### 1. Memory Optimization

```bash
# Increase swap if needed (for compilation)
sudo dphys-swapfile swapoff
sudo nano /etc/dphys-swapfile
# Change CONF_SWAPSIZE=1024
sudo dphys-swapfile setup
sudo dphys-swapfile swapon

# Monitor memory usage
free -h
htop
```

### 2. Boot Optimization

Speed up boot time:

```bash
sudo systemctl disable bluetooth
sudo systemctl disable hciuart
sudo systemctl disable keyboard-setup
sudo systemctl disable triggerhappy

# Edit boot config
sudo nano /boot/config.txt
# Add: boot_delay=0
# Add: disable_splash=1
```

### 3. Runtime Environment

Set optimal environment variables:

```bash
# Add to ~/.bashrc
cat >> ~/.bashrc << EOF

# ECG Monitor optimizations
export RUST_LOG=warn
export WINIT_UNIX_BACKEND=x11
export MESA_GL_VERSION_OVERRIDE=3.3
export LIBGL_ALWAYS_INDIRECT=0

EOF

source ~/.bashrc
```

## Troubleshooting

### Common Issues

#### 1. Graphics Issues

**Black screen or window not appearing:**
```bash
# Check display
echo $DISPLAY
export DISPLAY=:0.0

# Check X11 is running
ps aux | grep Xorg

# Start desktop if needed
sudo systemctl start lightdm
```

**Poor graphics performance:**
```bash
# Check GPU memory
vcgencmd get_mem gpu

# Check for GPU throttling
vcgencmd measure_temp
vcgencmd get_throttled

# Try software rendering
export LIBGL_ALWAYS_SOFTWARE=1
```

#### 2. Build Issues

**Compilation fails:**
```bash
# Update Rust
rustup update

# Check available space
df -h

# Clean build cache
cargo clean

# Build with verbose output
cargo build --release --verbose
```

**Missing dependencies:**
```bash
# Update package list
sudo apt update

# Install missing libraries
sudo apt install -y $(apt-cache search --names-only '^lib.*-dev$' | awk '{print $1}' | grep -E '(gl|x11|wayland)')
```

#### 3. Performance Issues

**High CPU usage:**
```bash
# Check CPU frequency
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq

# Set performance governor
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

**Memory issues:**
```bash
# Check memory usage
free -h

# Reduce memory usage
export RUST_LOG=error
ulimit -v 1000000  # Limit virtual memory
```

### Debug Mode

Run in debug mode for detailed information:

```bash
# Enable debug logging
RUST_LOG=debug ./target/release/ecg3

# Enable backtrace on panic
RUST_BACKTRACE=1 ./target/release/ecg3

# Use gdb for debugging
gdb ./target/release/ecg3
```

### System Monitoring

Monitor system performance:

```bash
# Install monitoring tools
sudo apt install -y htop iotop nethogs

# Check system resources
htop                    # CPU and memory
iotop                   # Disk I/O
nethogs                 # Network usage
vcgencmd measure_temp   # Temperature
vcgencmd get_throttled  # Throttling status
```

## Maintenance

### Regular Updates

```bash
# Update system packages
sudo apt update && sudo apt upgrade -y

# Update Rust toolchain
rustup update

# Rebuild ECG monitor
cd ~/ecg3
cargo clean
cargo build --release
```

### Backup Configuration

```bash
# Backup important configurations
sudo cp /boot/config.txt /boot/config.txt.backup
cp ~/.bashrc ~/.bashrc.backup

# Create system image backup (external machine)
sudo dd if=/dev/mmcblk0 of=ecg-pi-backup.img bs=4M status=progress
```

---

**Note**: This guide assumes a standard Raspberry Pi OS installation. Adjust paths and commands as needed for your specific setup.