#!/bin/bash

# ECG Monitor Runner Script for Raspberry Pi
# This script sets up the environment and runs the ECG monitor application

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}    ECG Monitor - Raspberry Pi  ${NC}"
    echo -e "${BLUE}================================${NC}"
}

# Check if we're running on Raspberry Pi
check_raspberry_pi() {
    if [ -f /proc/device-tree/model ]; then
        MODEL=$(cat /proc/device-tree/model 2>/dev/null | tr -d '\0')
        if [[ $MODEL == *"Raspberry Pi"* ]]; then
            print_status "Detected Raspberry Pi: $MODEL"
            return 0
        fi
    fi
    print_warning "Not running on Raspberry Pi - continuing anyway"
    return 0
}

# Check if ECG data files exist
check_data_files() {
    print_status "Checking ECG data files..."

    if [ ! -f "r01.edf" ]; then
        print_warning "r01.edf not found - will use synthetic ECG data"
    else
        print_status "Found r01.edf ($(du -h r01.edf | cut -f1))"
    fi

    if [ ! -f "r01.edf.qrs" ]; then
        print_warning "r01.edf.qrs not found - will generate synthetic QRS annotations"
    else
        print_status "Found r01.edf.qrs ($(du -h r01.edf.qrs | cut -f1))"
    fi
}

# Set up display for Raspberry Pi
setup_display() {
    if [ ! -z "${DISPLAY}" ]; then
        print_status "Display already set: $DISPLAY"
        return 0
    fi

    # Try to detect if we're in a desktop environment
    if pgrep -x "Xorg" > /dev/null || pgrep -x "Xwayland" > /dev/null; then
        export DISPLAY=:0.0
        print_status "Set DISPLAY to :0.0"
    elif [ -n "$WAYLAND_DISPLAY" ]; then
        print_status "Using Wayland display: $WAYLAND_DISPLAY"
    else
        print_warning "No display server detected. You may need to:"
        print_warning "  1. Start X11: sudo systemctl start lightdm"
        print_warning "  2. Or set DISPLAY manually: export DISPLAY=:0.0"
        print_warning "  3. Or use VNC/Remote Desktop"
    fi
}

# Check and install dependencies
check_dependencies() {
    print_status "Checking system dependencies..."

    # Check for Rust
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed!"
        print_error "Install Rust with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    else
        RUST_VERSION=$(rustc --version)
        print_status "Found Rust: $RUST_VERSION"
    fi

    # Check for required system libraries (for graphics)
    MISSING_LIBS=()

    if ! ldconfig -p | grep -q libGL.so; then
        MISSING_LIBS+=("libgl1-mesa-dev")
    fi

    if ! ldconfig -p | grep -q libX11.so; then
        MISSING_LIBS+=("libx11-dev")
    fi

    if ! ldconfig -p | grep -q libXcursor.so; then
        MISSING_LIBS+=("libxcursor-dev")
    fi

    if ! ldconfig -p | grep -q libXi.so; then
        MISSING_LIBS+=("libxi-dev")
    fi

    if ! ldconfig -p | grep -q libXrandr.so; then
        MISSING_LIBS+=("libxrandr-dev")
    fi

    if [ ${#MISSING_LIBS[@]} -ne 0 ]; then
        print_warning "Missing system libraries. Install with:"
        print_warning "sudo apt update && sudo apt install ${MISSING_LIBS[*]}"
    fi
}

# Build the application if needed
build_application() {
    print_status "Checking if build is needed..."

    if [ ! -f "target/release/ecg3" ] || [ "src/main.rs" -nt "target/release/ecg3" ]; then
        print_status "Building ECG monitor (this may take a few minutes)..."
        if cargo build --release; then
            print_status "Build successful!"
        else
            print_error "Build failed!"
            exit 1
        fi
    else
        print_status "Using existing build"
    fi
}

# Set GPU memory split for Raspberry Pi (if needed)
configure_gpu_memory() {
    if [ -f /proc/device-tree/model ]; then
        MODEL=$(cat /proc/device-tree/model 2>/dev/null | tr -d '\0')
        if [[ $MODEL == *"Raspberry Pi"* ]]; then
            if command -v vcgencmd &> /dev/null; then
                GPU_MEM=$(vcgencmd get_mem gpu | cut -d'=' -f2 | cut -d'M' -f1)
                if [ "$GPU_MEM" -lt "64" ]; then
                    print_warning "GPU memory is set to ${GPU_MEM}M"
                    print_warning "For better graphics performance, consider increasing GPU memory:"
                    print_warning "  sudo raspi-config -> Advanced Options -> Memory Split -> 128"
                else
                    print_status "GPU memory: ${GPU_MEM}M (good for graphics)"
                fi
            else
                print_warning "vcgencmd not available - cannot check GPU memory"
            fi
        else
            print_status "Not on Raspberry Pi - GPU memory configuration not needed"
        fi
    else
        print_status "Not on Raspberry Pi - GPU memory configuration not needed"
    fi
}

# Performance optimization for Raspberry Pi
optimize_performance() {
    if [ -f /proc/device-tree/model ]; then
        MODEL=$(cat /proc/device-tree/model 2>/dev/null | tr -d '\0')
        if [[ $MODEL == *"Raspberry Pi"* ]]; then
            print_status "Applying Raspberry Pi optimizations..."
        else
            print_status "Applying general optimizations..."
        fi
    else
        print_status "Applying general optimizations..."
    fi

    # Set CPU governor to performance (if available)
    if [ -f /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]; then
        CURRENT_GOVERNOR=$(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)
        print_status "CPU Governor: $CURRENT_GOVERNOR"

        if [ "$CURRENT_GOVERNOR" != "performance" ] && [ -w /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]; then
            echo performance > /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null || true
            print_status "Set CPU governor to performance mode"
        fi
    fi

    # Set environment variables for better performance
    export RUST_LOG=warn  # Reduce logging overhead
    export WINIT_UNIX_BACKEND=x11  # Force X11 backend for better compatibility
}

# Run the ECG monitor
run_monitor() {
    print_status "Starting ECG Monitor..."
    print_status "Press Ctrl+C to stop"
    echo

    # Add current directory to library path (in case of custom builds)
    export LD_LIBRARY_PATH="${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}."

    # Run the application with nice priority for smooth graphics
    if command -v nice &> /dev/null; then
        nice -n -10 ./target/release/ecg3
    else
        ./target/release/ecg3
    fi
}

# Cleanup function
cleanup() {
    print_status "Cleaning up..."
    # Reset CPU governor if we changed it
    if [ ! -z "$ORIGINAL_GOVERNOR" ] && [ -w /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]; then
        echo "$ORIGINAL_GOVERNOR" > /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null || true
    fi
}

# Set up signal handlers
trap cleanup EXIT INT TERM

# Main execution
main() {
    print_header

    # Change to script directory
    cd "$(dirname "$0")"

    check_data_files
    setup_display
    check_dependencies
    configure_gpu_memory
    optimize_performance
    build_application

    echo
    print_status "Starting ECG Monitor..."
    print_status "Use the following controls:"
    print_status "  - Play/Pause button to control playback"
    print_status "  - Speed slider to adjust playback rate"
    print_status "  - Heart rate is calculated from QRS detections"
    echo

    run_monitor
}

# Handle command line arguments
case "${1:-}" in
    --help|-h)
        echo "ECG Monitor Runner Script"
        echo
        echo "Usage: $0 [option]"
        echo
        echo "Options:"
        echo "  --help, -h     Show this help message"
        echo "  --check, -c    Check dependencies only"
        echo "  --build, -b    Build application only"
        echo "  --info, -i     Show system information"
        echo
        echo "Environment variables:"
        echo "  DISPLAY        X11 display (auto-detected)"
        echo "  RUST_LOG       Logging level (default: warn)"
        echo
        exit 0
        ;;
    --check|-c)
        print_header
        cd "$(dirname "$0")"
        check_data_files
        check_dependencies
        configure_gpu_memory
        exit 0
        ;;
    --build|-b)
        print_header
        cd "$(dirname "$0")"
        build_application
        exit 0
        ;;
    --info|-i)
        print_header
        cd "$(dirname "$0")"
        echo "System Information:"
        echo "  OS: $(uname -a)"
        echo "  Rust: $(rustc --version 2>/dev/null || echo 'Not installed')"
        echo "  Display: ${DISPLAY:-Not set}"
        if command -v vcgencmd &> /dev/null; then
            echo "  GPU Memory: $(vcgencmd get_mem gpu)"
        else
            echo "  GPU Memory: N/A (not Raspberry Pi)"
        fi
        echo "  ECG Data: $([ -f r01.edf ] && echo 'Present' || echo 'Missing')"
        echo "  QRS Data: $([ -f r01.edf.qrs ] && echo 'Present' || echo 'Missing')"
        exit 0
        ;;
    "")
        main
        ;;
    *)
        print_error "Unknown option: $1"
        print_error "Use --help for usage information"
        exit 1
        ;;
esac
