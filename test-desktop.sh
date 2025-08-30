#!/bin/bash

# Desktop Test Script for ECG Monitor
# This script tests the ECG monitor in a desktop environment

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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
    echo -e "${BLUE}  ECG Monitor Desktop Test      ${NC}"
    echo -e "${BLUE}================================${NC}"
}

# Check if we have a display
check_display() {
    if [ -z "$DISPLAY" ] && [ -z "$WAYLAND_DISPLAY" ]; then
        print_error "No display server detected!"
        print_error "Please ensure you're running in a desktop environment or set DISPLAY variable"
        echo
        echo "Options:"
        echo "  1. Run from desktop terminal"
        echo "  2. Set DISPLAY=:0.0 (for X11)"
        echo "  3. Use VNC or remote desktop"
        echo "  4. Run on actual Raspberry Pi with display"
        exit 1
    fi

    if [ -n "$DISPLAY" ]; then
        print_status "X11 display detected: $DISPLAY"
    fi

    if [ -n "$WAYLAND_DISPLAY" ]; then
        print_status "Wayland display detected: $WAYLAND_DISPLAY"
    fi
}

# Test if the binary exists and is executable
check_binary() {
    if [ ! -f "target/release/ecg3" ]; then
        print_warning "ECG monitor not built yet. Building now..."
        if cargo build --release; then
            print_status "Build successful!"
        else
            print_error "Build failed!"
            exit 1
        fi
    else
        print_status "ECG monitor binary found"
    fi

    if [ ! -x "target/release/ecg3" ]; then
        print_error "Binary is not executable"
        chmod +x target/release/ecg3
        print_status "Made binary executable"
    fi
}

# Check for data files
check_data() {
    print_status "Checking ECG data files..."

    if [ -f "r01.edf" ]; then
        size=$(du -h r01.edf | cut -f1)
        print_status "Found r01.edf ($size)"
    else
        print_warning "r01.edf not found - will use synthetic data"
    fi

    if [ -f "r01.edf.qrs" ]; then
        size=$(du -h r01.edf.qrs | cut -f1)
        print_status "Found r01.edf.qrs ($size)"
    else
        print_warning "r01.edf.qrs not found - will generate synthetic QRS"
    fi
}

# Set environment for best performance
setup_environment() {
    print_status "Setting up environment..."

    # Graphics performance
    export RUST_LOG=warn
    export WINIT_UNIX_BACKEND=x11
    export MESA_GL_VERSION_OVERRIDE=3.3

    # Disable compositing for better performance (if available)
    if command -v xfconf-query &> /dev/null; then
        xfconf-query -c xfwm4 -p /general/use_compositing -s false 2>/dev/null || true
    fi

    print_status "Environment configured for optimal performance"
}

# Test the application with a timeout
run_test() {
    print_status "Starting ECG Monitor test..."
    print_status "The application will run for 30 seconds, then close automatically"
    echo
    print_status "What to expect:"
    echo "  - Professional medical monitor interface"
    echo "  - 3 ECG leads (I, II, V1) with cyan waveforms"
    echo "  - Dark grid background with proper medical calibration"
    echo "  - Real-time heart rate display"
    echo "  - Play/Pause and speed controls"
    echo "  - QRS detection markers (yellow triangles)"
    echo

    # Create a wrapper script that kills the process after 30 seconds
    (
        sleep 30
        pkill -f "ecg3" 2>/dev/null || true
        print_status "Test completed automatically"
    ) &

    TIMEOUT_PID=$!

    # Run the ECG monitor
    print_status "Launching ECG Monitor..."
    ./target/release/ecg3 &
    ECG_PID=$!

    # Wait for the application to finish (either by timeout or user closing)
    wait $ECG_PID 2>/dev/null || true

    # Clean up timeout process
    kill $TIMEOUT_PID 2>/dev/null || true

    print_status "ECG Monitor test session ended"
}

# Interactive mode
interactive_mode() {
    echo
    read -p "Do you want to run the ECG monitor interactively (no timeout)? (y/N): " -n 1 -r
    echo

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Starting ECG Monitor in interactive mode..."
        print_status "Close the window or press Ctrl+C to exit"
        echo

        # Run without timeout
        ./target/release/ecg3
    fi
}

# Performance monitoring
show_performance_tips() {
    echo
    print_status "Performance Tips:"
    echo "  - Close other graphics-intensive applications"
    echo "  - Use a dedicated graphics card if available"
    echo "  - For Raspberry Pi: ensure GPU memory split ≥ 64MB"
    echo "  - For best results: use a 1920x1080 or higher resolution display"
    echo
}

# Main test function
main() {
    print_header

    # Change to script directory
    cd "$(dirname "$0")"

    # Run all checks
    check_display
    check_binary
    check_data
    setup_environment
    show_performance_tips

    # Run the test
    run_test

    # Ask if user wants interactive mode
    interactive_mode

    echo
    print_status "Desktop test complete!"
    print_status "If you saw the ECG monitor with cyan waveforms, the test was successful"
}

# Command line options
case "${1:-}" in
    --help|-h)
        echo "ECG Monitor Desktop Test Script"
        echo
        echo "Usage: $0 [option]"
        echo
        echo "Options:"
        echo "  --help, -h          Show this help"
        echo "  --interactive, -i   Run in interactive mode only"
        echo "  --build, -b         Build and test"
        echo
        echo "This script will:"
        echo "  1. Check for display server"
        echo "  2. Build the application if needed"
        echo "  3. Run a 30-second test session"
        echo "  4. Optionally run in interactive mode"
        echo
        exit 0
        ;;
    --interactive|-i)
        print_header
        cd "$(dirname "$0")"
        check_display
        check_binary
        setup_environment
        print_status "Starting ECG Monitor in interactive mode..."
        ./target/release/ecg3
        exit 0
        ;;
    --build|-b)
        print_header
        cd "$(dirname "$0")"
        print_status "Building ECG monitor..."
        cargo build --release
        print_status "Build complete - use './test-desktop.sh' to test"
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
