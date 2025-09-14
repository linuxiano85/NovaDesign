# NovaDesign

Nova Design is a Linux-only CAD/BIM application focused on trades work such as electrical, plumbing, masonry, drywall, and painting. Built in Rust with modern technologies, it provides an efficient workflow for trade professionals.

## Features

- **Multi-discipline support**: electrical, plumbing, drywall, suspended ceilings, masonry, and painting
- **Bill of Materials (BOM) calculations**: automated material lists with quantity and cost calculations
- **Phase-based design**: support for existing, demolition, and new construction phases
- **Symbol libraries**: electrical and plumbing component symbols with plan-view conventions
- **Layer management**: organize drawing elements by discipline with visibility controls
- **Material database**: comprehensive database with cost calculations
- **Plugin system**: WASM-based plugin SDK for extensibility
- **Internationalization**: support for Italian and English
- **Modern UI**: GTK4 and Libadwaita for native Linux integration

## Architecture

The project is organized as a Rust workspace with the following crates:

- **nova-core**: Core data models and types (buildings, floors, walls, components, materials)
- **nova-bom**: Bill of Materials calculation engine with discipline-specific calculators
- **nova-i18n**: Internationalization support using Fluent
- **nova-plugin-sdk**: Plugin system infrastructure with WASM/WASI support
- **nova-app**: GTK4/Libadwaita application (currently in development)

## Building

### Prerequisites

```bash
sudo apt-get install libgtk-4-dev libadwaita-1-dev build-essential
```

### Build

```bash
# Build all core libraries
cargo build --release --workspace --exclude nova-app

# Run tests
cargo test --workspace --exclude nova-app

# Check code style
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## BOM Calculation Features

### Drywall Calculations

- Panel area calculations based on wall dimensions
- Stud length calculations with standard 400mm spacing
- Track length calculations for perimeter
- Screw count estimation (25 screws per m²)
- Fixing calculations based on substrate type

### Suspended Ceiling Calculations

- Tile count with waste factor (600x600mm standard)
- T-grid main and cross profile lengths
- Hanger calculations with standard spacing
- Fixing requirements for structural attachment

### Electrical Calculations

- Cable length estimation based on component layout
- Device counting for outlets, switches, lights
- Basic material requirements

## Plugin System

The plugin SDK provides:

- WASM/WASI-based plugin architecture
- Type-safe plugin interfaces
- Hot-loading capabilities
- Sandboxed execution environment
- Host function APIs for accessing project data

## Material Database

Default materials include:

- Drywall panels, studs, tracks, screws
- Suspended ceiling tiles, grids, hangers
- Electrical cables, outlets, switches
- Fixings and fasteners
- Cost information in EUR

## File Format

The application uses a JSON-based project format with:

- Hierarchical project structure (buildings → floors → components)
- Phase information for renovation workflows  
- Discipline-based organization
- Extensible component properties

## Internationalization

Currently supported languages:
- English (en)
- Italian (it)

Translation keys cover:
- Application UI
- Material names
- Discipline terminology
- BOM export labels

## License

GPL-3.0-or-later

## Development Status

This is the initial release (v0.1.0) focusing on:
- ✅ Core data models and architecture
- ✅ BOM calculation engine for drywall and suspended ceilings
- ✅ Plugin SDK foundation
- ✅ Internationalization infrastructure
- 🚧 GTK4/Libadwaita UI (in progress)
- 🚧 2D/3D rendering capabilities (planned)
- 🚧 AI-assisted suggestions (planned)
- 🚧 Open format support (planned)

## Contributing

This project welcomes contributions! Please see the issue tracker for planned features and improvements.