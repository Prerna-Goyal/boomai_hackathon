# ECG Monitor for Raspberry Pi

A real-time ECG (Electrocardiogram) monitoring application built in Rust, designed to run on Raspberry Pi with professional medical monitor styling.

A professional patient monitoring system displaying real-time ECG waveforms with Heart Rate and SpO2 monitoring in an authentic medical monitor interface.

## Features

### 🫀 Essential Patient Monitor Interface
- **Focused medical monitor layout** with clean, professional appearance
- **Dual-parameter display** - Heart Rate (ECG) and SpO2 monitoring
- **Large vital signs panels** with prominent numerical displays updating in real-time
- **Real-time ECG waveforms** (Lead II, V1) with medical-grade green traces
- **PLETH waveform** synchronized with pulse oximetry readings
- **Medical equipment styling** - dark backgrounds with bright vital sign numbers

### 📊 Data Sources
- **EDF file support** - Reads European Data Format ECG files (`r01.edf`)
- **QRS annotation parsing** - MIT-BIH compatible QRS detection files (`r01.edf.qrs`)
- **Synthetic ECG generation** - Fallback realistic waveform generation when files unavailable
- **Continuous playback** - Seamless looping of ECG data for continuous monitoring

### 🎛️ Medical Equipment Controls
- **Prominent Play/Pause button** - Large medical-style control with color coding (green/orange)
- **Precise speed control** - Professional slider for 0.1x to 5.0x playback adjustment
- **Live status display** - "MONITORING" / "PAUSED" status with real-time clock
- **Professional layout** - Medical blue control panel matching hospital equipment

### 🏥 Essential Patient Monitor Experience
- **Hospital-grade interface** - Clean, focused design matching medical equipment
- **Key vital signs panels** - Large numerical displays for Heart Rate and SpO2
- **Multi-waveform display** - ECG (green) and PLETH (cyan) traces
- **Medical grid system** - Standard ECG grid with proper calibration
- **Real-time updating values** - HR: 65-90 BPM, SpO2: 94-100% with natural variation
- **Professional layout** - Control panel, waveform area, and essential vital signs

### 🔧 Raspberry Pi Optimized
- **GPU acceleration** - Utilizes Raspberry Pi GPU for smooth 60fps graphics
- **Performance tuning** - Automatic CPU governor and memory optimization
- **Cross-platform compatibility** - Runs on Linux (ARM64/x86), macOS, and Windows
- **Lightweight design** - Minimal resource usage for embedded systems

## Hardware Requirements

### Minimum Requirements
- **Raspberry Pi 3B+** or newer
- **1GB RAM** (2GB+ recommended for Pi 4)
- **GPU memory split**: 64MB minimum, 128MB recommended
- **Display**: Any monitor/TV with HDMI connection
- **Storage**: 1GB free space for compilation

### Recommended Setup
- **Raspberry Pi 4B** with 4GB RAM
- **Official 7" Touchscreen** or external monitor
- **SD Card**: Class 10, 32GB+
- **Cooling**: Heatsink or fan for sustained performance

## Quick Start

### 1. Clone and Setup
```bash
git clone <repository_url>
cd ecg3
chmod +x run_ecg_monitor.sh
```

### 2. Run with Auto-Setup
```bash
./run_ecg_monitor.sh
```

The script will automatically:
- Check dependencies
- Build the application
- Optimize Raspberry Pi settings
- Launch the ECG monitor

### 3. Manual Installation
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install system dependencies (Ubuntu/Debian)
sudo apt update
sudo apt install libgl1-mesa-dev libx11-dev libxcursor-dev libxi-dev libxrandr-dev

# Build and run
cargo build --release
./target/release/ecg3
```

## Usage

### Controls
- **Play/Pause Button**: Toggle ECG playback
- **Speed Slider**: Adjust playback rate (0.1x - 5.0x)
- **Heart Rate Display**: Shows current BPM calculated from QRS intervals

### Data Files
Place your ECG data files in the project root:
- `r01.edf` - EDF format ECG signal data
- `r01.edf.qrs` - QRS annotation file

If these files are not present, the application will generate synthetic ECG data.

### Patient Monitor Features
- **Dual-Parameter Display**: Heart Rate (ECG) and SpO2 in dedicated large panels
- **Real-time Vital Numbers**: Hospital-style prominent numerical displays that update live
- **ECG Waveforms**: Lead II and V1 with authentic green medical monitor traces
- **PLETH Waveform**: Realistic plethysmography trace synchronized with SpO2 readings
- **Medical Grid**: Standard ECG grid system for accurate measurement
- **Professional Header**: Patient info bar with bed number and identification
- **Medical Controls**: Prominent play/pause, speed control, and status indicators

## File Formats

### EDF (European Data Format)
The application supports standard EDF files commonly used in medical equipment:
- **Header parsing**: Patient info, recording parameters
- **Signal extraction**: Multi-channel ECG data
- **Calibration**: Automatic digital-to-physical unit conversion

### QRS Annotations
Compatible with MIT-BIH annotation format:
- **Time stamps**: QRS complex locations
- **Beat classification**: Normal, PVC, aberrant beats
- **Heart rate calculation**: RR interval analysis

## Architecture

### Core Components

#### `main.rs`
- Application entry point and main event loop
- ECG monitor state management
- GUI framework integration (egui/eframe)

#### `edf_parser.rs`
- EDF file format parser
- Signal data extraction and calibration
- Header information parsing

#### `qrs_parser.rs`
- QRS annotation file parser
- Beat detection and classification
- Heart rate calculation algorithms

#### `ecg_display.rs`
- Professional medical monitor interface
- Real-time waveform rendering
- Grid overlay and calibration markers

### Dependencies
- **eframe/egui**: Cross-platform GUI framework
- **byteorder**: Binary data parsing
- **tokio**: Async runtime for data streaming
- **chrono**: Time and date handling

## Performance Optimization

### Raspberry Pi Specific
```bash
# GPU memory split (recommended: 128MB)
sudo raspi-config
# Advanced Options → Memory Split → 128

# CPU governor (for better performance)
echo performance | sudo tee /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
```

### Environment Variables
```bash
export RUST_LOG=warn              # Reduce logging overhead
export WINIT_UNIX_BACKEND=x11     # Force X11 backend
```

## Troubleshooting

### Common Issues

#### "No display server detected"
```bash
# Start X11 desktop
sudo systemctl start lightdm

# Or set display manually
export DISPLAY=:0.0
```

#### Graphics performance issues
```bash
# Check GPU memory
vcgencmd get_mem gpu

# Increase if less than 64MB
sudo raspi-config
```

#### Build errors
```bash
# Update Rust toolchain
rustup update

# Install missing dependencies
sudo apt install build-essential pkg-config
```

### Debug Mode
Run with detailed logging:
```bash
RUST_LOG=debug ./target/release/ecg3
```

## Development

### Building from Source
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code
cargo check
cargo clippy
```

### Adding New Features
1. **Signal Processing**: Extend `edf_parser.rs` for new file formats
2. **Display Options**: Modify `ecg_display.rs` for new visualization modes
3. **Analysis Tools**: Add algorithms to `qrs_parser.rs`

### Cross-Compilation for Raspberry Pi
```bash
# Add ARM target
rustup target add aarch64-unknown-linux-gnu

# Install cross-compilation tools
sudo apt install gcc-aarch64-linux-gnu

# Build for Raspberry Pi
cargo build --target=aarch64-unknown-linux-gnu --release
```

## Medical Disclaimer

⚠️ **Important**: This software is for educational and demonstration purposes only. It is NOT intended for clinical use, medical diagnosis, or patient monitoring. Always consult qualified medical professionals for health-related decisions.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Coding Standards
- Follow Rust standard formatting (`cargo fmt`)
- Run clippy lints (`cargo clippy`)
- Add tests for new features
- Update documentation

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **MIT-BIH Database**: ECG data format specifications
- **EDF Specification**: European Data Format standards
- **egui Framework**: Excellent immediate-mode GUI library
- **Raspberry Pi Foundation**: Amazing single-board computing platform

## Support

- **Issues**: Report bugs and feature requests via GitHub Issues
- **Documentation**: Check the `/docs` folder for detailed guides
- **Community**: Join discussions in GitHub Discussions

---

**Made with ❤️ for the Raspberry Pi and medical monitoring community**