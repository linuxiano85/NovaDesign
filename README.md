# NovaDesign

A modern GTK4/Libadwaita application for Linux.

## Cross-distro Developer Setup

NovaDesign targets Linux universally and provides automated setup across major distributions. The recommended way for end users to run NovaDesign is via **Flatpak** (universal runtime), while native builds are primarily for developers.

### Quick Setup (Automated)

For a fully automated setup that detects your distribution and installs all required dependencies:

```bash
./scripts/dev-setup.sh
```

This script supports:
- **Debian/Ubuntu** (apt)
- **Fedora/RHEL** (dnf/yum) 
- **Arch Linux** (pacman)
- **openSUSE** (zypper)
- **Gentoo** (emerge)

### Manual Setup by Distribution

If you prefer manual installation or the automated script doesn't work for your setup:

#### Debian/Ubuntu
```bash
sudo apt update
sudo apt install build-essential pkg-config libgtk-4-dev libadwaita-1-dev \
    libglib2.0-dev libpango1.0-dev libcairo2-dev libgdk-pixbuf-2.0-dev \
    flatpak flatpak-builder
```

#### Fedora/RHEL
```bash
sudo dnf install gcc gcc-c++ pkg-config gtk4-devel libadwaita-devel \
    glib2-devel pango-devel cairo-devel gdk-pixbuf2-devel \
    flatpak flatpak-builder
```

#### Arch Linux
```bash
sudo pacman -Sy base-devel pkgconf gtk4 libadwaita glib2 pango cairo \
    gdk-pixbuf2 flatpak flatpak-builder
```

#### openSUSE
```bash
sudo zypper install gcc gcc-c++ pkg-config gtk4-devel libadwaita-1-devel \
    glib2-devel pango-devel cairo-devel gdk-pixbuf-devel \
    flatpak flatpak-builder
```

#### Gentoo
```bash
sudo emerge sys-devel/gcc virtual/pkgconfig gui-libs/gtk:4 gui-libs/libadwaita \
    dev-libs/glib x11-libs/pango x11-libs/cairo x11-libs/gdk-pixbuf \
    sys-apps/flatpak dev-util/flatpak-builder
```

#### Nix/NixOS

For Nix users, you can use a development shell:

```bash
nix-shell -p pkg-config gtk4 libadwaita glib pango cairo gdk-pixbuf flatpak flatpak-builder rustc cargo
```

Or create a `shell.nix` file in your project directory:

```nix
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    gtk4
    libadwaita
    glib
    pango
    cairo
    gdk-pixbuf
    flatpak
    flatpak-builder
    rustc
    cargo
  ];
}
```

### Rust Installation

If you don't have Rust installed, the setup script will install it automatically via rustup. You can also install it manually:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Building and Running

### Using Make (Recommended)

The project includes a Makefile with convenient targets:

```bash
# First-time setup
make setup

# Build the project
make build

# Run the application
make run

# Format code
make fmt

# Run linter
make clippy

# Run tests  
make test

# Build Flatpak package
make flatpak-build

# Run via Flatpak
make flatpak-run
```

### Using Cargo Directly

```bash
# Build the project
cargo build --release

# Run the application
cargo run -p nova-app

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Flatpak Usage (Recommended for End Users)

NovaDesign is designed to be distributed primarily via Flatpak for a universal Linux experience:

### Building the Flatpak

```bash
# Install required Flatpak runtimes
sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
sudo flatpak install flathub org.gnome.Platform//45 org.gnome.Sdk//45

# Build the Flatpak package
flatpak-builder build flatpak/io.nova.Design.json --force-clean --install-deps-from=flathub

# Install locally
flatpak-builder --user --install --force-clean build flatpak/io.nova.Design.json

# Run the application
flatpak run io.nova.Design
```

### Or use Make targets:

```bash
make flatpak-build
make flatpak-run
```

## Development

### Project Structure

- `nova-app/` - Main application package
- `scripts/` - Development and setup scripts
- `flatpak/` - Flatpak packaging configuration
- `Makefile` - Convenient development targets

### Contributing

1. Run the setup script: `./scripts/dev-setup.sh`
2. Build the project: `make build`
3. Make your changes
4. Test your changes: `make test`
5. Format code: `make fmt`
6. Run linter: `make clippy`

## License

This project is licensed under the terms specified in the LICENSE file.