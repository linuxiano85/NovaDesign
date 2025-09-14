# NovaDesign

NovaDesign is a Linux-only, Rust-based, CAD/BIM-lite application focused on construction trades. It provides a clean, modern interface for managing electrical, plumbing, masonry, drywall, ceiling, painting, and structural work with integrated bill of materials generation and business management tools.

## Features

### Core Design Tools
- **Multi-discipline support**: Architecture, Electrical, Plumbing, Masonry, Drywall, Ceilings, Painting, Structural
- **Phase-based construction planning**: Esistente (Existing), Demolizione (Demolition), Nuovo (New)
- **2D and 3D view placeholders** (3D rendering with wgpu planned for future releases)
- **Building data model** with levels, walls, and MEP systems

### Bill of Materials (BOM) Engine
- **Electrical device counting** by category (outlets, switches, lights, panels, etc.)
- **Drywall material calculations**: montanti (studs), guide (tracks), lastre (boards), tasselli (fixings by substrate type)
- **Suspended ceiling calculations**: profili T principali/secondari (T profiles), pendinature (hangers), angolari perimetrali (perimeter angles)
- **CSV export** for BOM data

### Business Suite
- **Customer management**: companies and individual clients with VAT/CF codes
- **Price lists**: categorized items with tax rates (IVA 22%, 10%, 4%, esente)
- **Quotes (Preventivi)**: comprehensive estimation with line items and VAT calculations
- **Invoices (Fatture)**: billing functionality based on quotes
- **DDT (Documenti di Trasporto)**: delivery notes with transport details
- **Company profiles**: complete business information including IBAN, logo, contacts
- **Numbering sequences**: automatic document numbering by year
- **CSV and JSON export** capabilities

### Internationalization
- **Italian and English** localization with fluent-rs
- **Extensible i18n system** for future language additions

### Modern Tech Stack
- **Rust**: Memory-safe, performant, and reliable
- **GTK4/Libadwaita**: Native Linux interface with light/dark theme support
- **Flatpak packaging**: Easy installation and distribution
- **Plugin SDK**: Extensible architecture with WASM/WASI support (planned)

## Installation

### From Flatpak (Recommended)

```bash
# Install from Flathub (when available)
flatpak install flathub io.nova.Design

# Run the application
flatpak run io.nova.Design
```

### From Source

#### Prerequisites
- Rust 1.70+ with Cargo
- GTK4 development libraries
- Libadwaita development libraries

On Ubuntu/Debian:
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev pkg-config
```

#### Building
```bash
git clone https://github.com/linuxiano85/NovaDesign
cd NovaDesign
cargo build --release
./target/release/nova-design
```

## Usage

### Basic Workflow

1. **Create a Project**: Start with a new building project
2. **Add Elements**: Place electrical devices, drywall walls, suspended ceilings
3. **Set Phases**: Categorize elements as Existing, Demolition, or New
4. **Generate BOM**: Automatically calculate material quantities
5. **Create Business Documents**: Generate quotes, invoices, and delivery notes

### Example Integration

```bash
# Run the integration test to see all features in action
cargo run --example integration_test
```

This example demonstrates:
- Creating a building with multiple elements
- Generating a complete bill of materials
- Business document creation with pricing
- CSV export functionality

## Project Structure

```
NovaDesign/
├── nova-core/          # Core data models (buildings, elements, phases)
├── nova-bom/           # Bill of materials engine
├── nova-biz/           # Business suite (quotes, invoices, customers)
├── nova-i18n/          # Internationalization support
├── nova-plugin-sdk/    # Plugin development SDK
├── nova-design/        # Main GTK4 application
└── examples/           # Integration tests and examples
```

## Development

### Running Tests
```bash
cargo test --all-features
```

### Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features

# Full CI check
cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-features
```

### Building Flatpak
```bash
flatpak-builder build-dir io.nova.Design.yaml --force-clean
flatpak build-export export build-dir
flatpak build-bundle export nova-design.flatpak io.nova.Design
```

## Contributing

Contributions are welcome! Please ensure:

1. Code follows Rust standards (use `cargo fmt` and `cargo clippy`)
2. Tests pass (`cargo test`)
3. New features include appropriate tests
4. Updates maintain GPL-3.0-or-later license compatibility

## Roadmap

### M1 (Current)
- ✅ Core data models and BOM engine
- ✅ Business suite foundation
- ✅ GTK4/Libadwaita UI framework
- ✅ Basic Italian/English i18n
- ✅ Flatpak packaging

### M2 (Planned)
- 3D rendering with wgpu
- Advanced electrical calculations
- Plumbing flow calculations
- Import/export formats (IFC, DWG)
- Advanced business reporting

### M3 (Future)
- WASM/WASI plugin system
- Cloud synchronization
- Mobile companion app
- AI-powered design assistance

## License

This project is licensed under GPL-3.0-or-later. See [LICENSE](LICENSE) for details.

## Contact

- **Repository**: https://github.com/linuxiano85/NovaDesign
- **Issues**: https://github.com/linuxiano85/NovaDesign/issues

---

*NovaDesign - Modern CAD/BIM tools for construction professionals*