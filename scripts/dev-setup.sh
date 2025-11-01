#!/bin/bash
set -euo pipefail

# NovaDesign Cross-Distro Developer Setup Script
# Detects Linux distribution and installs required build dependencies

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to detect distribution
detect_distro() {
    if [[ -f /etc/os-release ]]; then
        source /etc/os-release
        DISTRO_ID="${ID:-unknown}"
        DISTRO_ID_LIKE="${ID_LIKE:-}"
        
        print_info "Detected distribution: $DISTRO_ID"
        if [[ -n "$DISTRO_ID_LIKE" ]]; then
            print_info "Distribution family: $DISTRO_ID_LIKE"
        fi
    else
        print_error "/etc/os-release not found. Cannot detect distribution."
        exit 1
    fi
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install packages on Debian/Ubuntu
install_debian_packages() {
    print_info "Installing packages for Debian/Ubuntu..."
    
    local packages=(
        "build-essential"
        "pkg-config"
        "libgtk-4-dev"
        "libadwaita-1-dev"
        "libglib2.0-dev"
        "libpango1.0-dev"
        "libcairo2-dev"
        "libgdk-pixbuf-2.0-dev"
        "flatpak"
        "flatpak-builder"
    )
    
    sudo apt update
    sudo apt install -y "${packages[@]}"
}

# Function to install packages on Fedora/RHEL
install_fedora_packages() {
    print_info "Installing packages for Fedora/RHEL..."
    
    local packages=(
        "gcc"
        "gcc-c++"
        "pkg-config"
        "gtk4-devel"
        "libadwaita-devel"
        "glib2-devel"
        "pango-devel"
        "cairo-devel"
        "gdk-pixbuf2-devel"
        "flatpak"
        "flatpak-builder"
    )
    
    if command_exists dnf; then
        sudo dnf install -y "${packages[@]}"
    else
        sudo yum install -y "${packages[@]}"
    fi
}

# Function to install packages on Arch Linux
install_arch_packages() {
    print_info "Installing packages for Arch Linux..."
    
    local packages=(
        "base-devel"
        "pkgconf"
        "gtk4"
        "libadwaita"
        "glib2"
        "pango"
        "cairo"
        "gdk-pixbuf2"
        "flatpak"
        "flatpak-builder"
    )
    
    sudo pacman -Sy --noconfirm "${packages[@]}"
}

# Function to install packages on openSUSE
install_opensuse_packages() {
    print_info "Installing packages for openSUSE..."
    
    local packages=(
        "gcc"
        "gcc-c++"
        "pkg-config"
        "gtk4-devel"
        "libadwaita-1-devel"
        "glib2-devel"
        "pango-devel"
        "cairo-devel"
        "gdk-pixbuf-devel"
        "flatpak"
        "flatpak-builder"
    )
    
    sudo zypper install -y "${packages[@]}"
}

# Function to install packages on Gentoo
install_gentoo_packages() {
    print_info "Installing packages for Gentoo..."
    
    local packages=(
        "sys-devel/gcc"
        "virtual/pkgconfig"
        "gui-libs/gtk:4"
        "gui-libs/libadwaita"
        "dev-libs/glib"
        "x11-libs/pango"
        "x11-libs/cairo"
        "x11-libs/gdk-pixbuf"
        "sys-apps/flatpak"
        "dev-util/flatpak-builder"
    )
    
    print_warning "Gentoo installation may take a long time due to compilation..."
    sudo emerge --ask --verbose "${packages[@]}"
}

# Function to install Rust toolchain
install_rust() {
    if command_exists cargo; then
        print_info "Rust toolchain already installed: $(rustc --version)"
        return 0
    fi
    
    print_info "Installing Rust toolchain via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    print_warning "Please run 'source \$HOME/.cargo/env' to update your current shell environment"
    print_info "Or restart your terminal session"
}

# Function to setup Flatpak
setup_flatpak() {
    print_info "Setting up Flatpak..."
    
    # Add Flathub repository if not already added
    if ! flatpak remote-list | grep -q flathub; then
        print_info "Adding Flathub repository..."
        sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
    else
        print_info "Flathub repository already configured"
    fi
    
    # Install GNOME runtime for building
    print_info "Installing GNOME Platform runtime (this may take a while)..."
    sudo flatpak install -y flathub org.gnome.Platform//45 org.gnome.Sdk//45 org.freedesktop.Sdk.Extension.rust-stable//23.08
}

# Function to install packages based on detected distribution
install_packages() {
    case "$DISTRO_ID" in
        "ubuntu"|"debian"|"linuxmint"|"pop")
            install_debian_packages
            ;;
        "fedora"|"rhel"|"centos"|"rocky"|"almalinux")
            install_fedora_packages
            ;;
        "arch"|"manjaro"|"endeavouros")
            install_arch_packages
            ;;
        "opensuse"|"opensuse-leap"|"opensuse-tumbleweed"|"sles")
            install_opensuse_packages
            ;;
        "gentoo")
            install_gentoo_packages
            ;;
        *)
            # Try to match based on ID_LIKE
            if [[ "$DISTRO_ID_LIKE" =~ debian|ubuntu ]]; then
                install_debian_packages
            elif [[ "$DISTRO_ID_LIKE" =~ fedora|rhel ]]; then
                install_fedora_packages
            elif [[ "$DISTRO_ID_LIKE" =~ arch ]]; then
                install_arch_packages
            elif [[ "$DISTRO_ID_LIKE" =~ suse ]]; then
                install_opensuse_packages
            else
                print_error "Unsupported distribution: $DISTRO_ID"
                print_info "Please install the following packages manually:"
                print_info "- Build tools (gcc, make, etc.)"
                print_info "- pkg-config"
                print_info "- GTK4 development headers"
                print_info "- Libadwaita development headers"
                print_info "- GLib development headers"
                print_info "- Pango development headers"
                print_info "- Cairo development headers"
                print_info "- GdkPixbuf development headers"
                print_info "- Flatpak and flatpak-builder"
                exit 1
            fi
            ;;
    esac
}

# Function to provide Nix/NixOS guidance
show_nix_guidance() {
    if [[ "$DISTRO_ID" == "nixos" ]] || command_exists nix-shell; then
        print_info "For Nix/NixOS users, you can use a development shell:"
        echo ""
        echo "nix-shell -p pkg-config gtk4 libadwaita glib pango cairo gdk-pixbuf flatpak flatpak-builder rustc cargo"
        echo ""
        print_info "Or create a shell.nix file in your project directory with the required dependencies."
    fi
}

# Main function
main() {
    print_info "NovaDesign Cross-Distro Developer Setup"
    print_info "======================================="
    echo ""
    
    detect_distro
    echo ""
    
    show_nix_guidance
    echo ""
    
    install_packages
    echo ""
    
    install_rust
    echo ""
    
    setup_flatpak
    echo ""
    
    print_success "Setup completed successfully!"
    print_info "Summary of installed components:"
    print_info "• Build tools and development headers"
    print_info "• GTK4 and Libadwaita development libraries"
    print_info "• Rust toolchain (if not already present)"
    print_info "• Flatpak and flatpak-builder"
    print_info "• GNOME Platform runtime for Flatpak builds"
    echo ""
    print_info "Next steps:"
    print_info "1. If Rust was just installed, run: source \$HOME/.cargo/env"
    print_info "2. Build the project: cargo build --release"
    print_info "3. Run the application: cargo run -p nova-app"
    print_info "4. Or use the Makefile targets: make build, make run, etc."
    echo ""
    print_success "Happy coding with NovaDesign!"
}

# Run main function
main "$@"