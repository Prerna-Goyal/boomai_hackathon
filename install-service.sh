#!/bin/bash

# ECG Monitor Service Installation Script
# This script installs the ECG monitor as a systemd service on Raspberry Pi

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
    echo -e "${BLUE}============================================${NC}"
    echo -e "${BLUE}  ECG Monitor Service Installation Script  ${NC}"
    echo -e "${BLUE}============================================${NC}"
}

# Check if running as root
check_root() {
    if [ "$EUID" -eq 0 ]; then
        print_error "Please do not run this script as root"
        print_error "The service will be installed for the current user: $(whoami)"
        exit 1
    fi
}

# Check if we're on a systemd-based system
check_systemd() {
    if ! command -v systemctl &> /dev/null; then
        print_error "systemctl not found - this system doesn't use systemd"
        exit 1
    fi

    if ! systemctl --version &> /dev/null; then
        print_error "systemd not running or not available"
        exit 1
    fi

    print_status "systemd detected - proceeding with service installation"
}

# Check if ECG monitor is built
check_build() {
    if [ ! -f "target/release/ecg3" ]; then
        print_warning "ECG monitor not built yet"
        print_status "Building ECG monitor..."

        if cargo build --release; then
            print_status "Build successful!"
        else
            print_error "Build failed! Please fix compilation errors first."
            exit 1
        fi
    else
        print_status "ECG monitor binary found"
    fi
}

# Create service file with correct paths
create_service_file() {
    local username=$(whoami)
    local current_dir=$(pwd)
    local service_file="/tmp/ecg-monitor.service"

    print_status "Creating service file for user: $username"
    print_status "Working directory: $current_dir"

    cat > "$service_file" << EOF
[Unit]
Description=ECG Monitor - Real-time ECG Display for Raspberry Pi
Documentation=file://$current_dir/README.md
After=graphical-session.target
Wants=graphical-session.target

[Service]
Type=simple
User=$username
Group=$username
WorkingDirectory=$current_dir
Environment="DISPLAY=:0"
Environment="RUST_LOG=warn"
Environment="WINIT_UNIX_BACKEND=x11"
Environment="MESA_GL_VERSION_OVERRIDE=3.3"
Environment="LIBGL_ALWAYS_INDIRECT=0"
ExecStartPre=/bin/sleep 10
ExecStart=$current_dir/target/release/ecg3
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

# Performance optimizations
Nice=-10
IOSchedulingClass=1
IOSchedulingPriority=4

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=read-only
ReadWritePaths=$current_dir

# Resource limits
LimitNOFILE=4096
LimitNPROC=512

[Install]
WantedBy=graphical-session.target
EOF

    echo "$service_file"
}

# Install the service
install_service() {
    local service_file=$(create_service_file)
    local service_name="ecg-monitor.service"

    print_status "Installing service file..."

    # Copy service file to systemd directory
    sudo cp "$service_file" "/etc/systemd/system/$service_name"

    # Set proper permissions
    sudo chmod 644 "/etc/systemd/system/$service_name"

    # Reload systemd daemon
    print_status "Reloading systemd daemon..."
    sudo systemctl daemon-reload

    # Enable the service
    print_status "Enabling ECG monitor service..."
    sudo systemctl enable "$service_name"

    print_status "Service installed successfully!"

    # Clean up temp file
    rm -f "$service_file"
}

# Configure auto-login (optional)
configure_autologin() {
    local username=$(whoami)

    echo
    read -p "Do you want to configure auto-login for user '$username'? (y/N): " -n 1 -r
    echo

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Configuring auto-login..."

        # Create override directory
        sudo mkdir -p /etc/systemd/system/getty@tty1.service.d

        # Create override file
        sudo tee /etc/systemd/system/getty@tty1.service.d/override.conf > /dev/null << EOF
[Service]
ExecStart=
ExecStart=-/sbin/agetty --noissue --autologin $username %I \$TERM
Type=idle
EOF

        print_status "Auto-login configured for $username"
        print_warning "You may need to reboot for auto-login to take effect"
    else
        print_status "Skipping auto-login configuration"
    fi
}

# Configure desktop auto-start (alternative to service)
configure_autostart() {
    local username=$(whoami)
    local current_dir=$(pwd)
    local autostart_dir="$HOME/.config/autostart"
    local desktop_file="$autostart_dir/ecg-monitor.desktop"

    echo
    read -p "Do you want to auto-start ECG monitor on desktop login? (y/N): " -n 1 -r
    echo

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Creating desktop autostart entry..."

        # Create autostart directory
        mkdir -p "$autostart_dir"

        # Create desktop file
        cat > "$desktop_file" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=ECG Monitor
Comment=Real-time ECG monitoring application
Exec=$current_dir/run_ecg_monitor.sh
Icon=applications-science
Path=$current_dir
Terminal=false
StartupNotify=true
Categories=Science;Medical;
X-GNOME-Autostart-enabled=true
Hidden=false
EOF

        chmod +x "$desktop_file"
        print_status "Desktop autostart configured"
    else
        print_status "Skipping desktop autostart configuration"
    fi
}

# Show service management commands
show_service_commands() {
    print_status "Service management commands:"
    echo
    echo "  Start service:    sudo systemctl start ecg-monitor"
    echo "  Stop service:     sudo systemctl stop ecg-monitor"
    echo "  Restart service:  sudo systemctl restart ecg-monitor"
    echo "  Service status:   sudo systemctl status ecg-monitor"
    echo "  View logs:        journalctl -u ecg-monitor -f"
    echo "  Disable service:  sudo systemctl disable ecg-monitor"
    echo
}

# Test the service
test_service() {
    echo
    read -p "Do you want to test the service now? (y/N): " -n 1 -r
    echo

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Starting ECG monitor service..."

        if sudo systemctl start ecg-monitor; then
            sleep 3
            print_status "Service started. Checking status..."
            sudo systemctl status ecg-monitor --no-pager -l

            echo
            print_status "Service is running. Check your display for the ECG monitor."
            print_status "To stop the service: sudo systemctl stop ecg-monitor"
        else
            print_error "Failed to start service. Check logs with: journalctl -u ecg-monitor"
        fi
    else
        print_status "Service installed but not started"
        print_status "Start it manually with: sudo systemctl start ecg-monitor"
    fi
}

# Uninstall function
uninstall_service() {
    print_status "Uninstalling ECG monitor service..."

    # Stop and disable service
    sudo systemctl stop ecg-monitor 2>/dev/null || true
    sudo systemctl disable ecg-monitor 2>/dev/null || true

    # Remove service file
    sudo rm -f /etc/systemd/system/ecg-monitor.service

    # Reload systemd
    sudo systemctl daemon-reload

    print_status "Service uninstalled successfully"
}

# Main execution
main() {
    print_header

    # Change to script directory
    cd "$(dirname "$0")"

    check_root
    check_systemd
    check_build
    install_service
    configure_autologin
    configure_autostart
    show_service_commands
    test_service

    echo
    print_status "Installation complete!"
    print_status "The ECG monitor service has been installed and configured."
    print_status "It will automatically start when the graphical session begins."
}

# Handle command line arguments
case "${1:-}" in
    --help|-h)
        echo "ECG Monitor Service Installation Script"
        echo
        echo "Usage: $0 [option]"
        echo
        echo "Options:"
        echo "  --help, -h        Show this help message"
        echo "  --uninstall, -u   Uninstall the service"
        echo "  --status, -s      Show service status"
        echo
        echo "This script will:"
        echo "  1. Install the ECG monitor as a systemd service"
        echo "  2. Configure auto-login (optional)"
        echo "  3. Configure desktop autostart (optional)"
        echo "  4. Test the service installation"
        echo
        exit 0
        ;;
    --uninstall|-u)
        print_header
        cd "$(dirname "$0")"
        uninstall_service
        exit 0
        ;;
    --status|-s)
        print_header
        print_status "ECG Monitor Service Status:"
        sudo systemctl status ecg-monitor --no-pager -l || true
        echo
        print_status "Recent logs:"
        journalctl -u ecg-monitor --no-pager -n 20 || true
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
