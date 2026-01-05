# Rust Qt6 Template

Minimal template for building Qt6 GUI applications with Rust using `qmetaobject`.

## Features

- **Reactive UI**: Rust components with properties, signals, and methods exposed to QML
- **Separation of concerns**: Rust handles logic, QML handles UI
- **Minimal dependencies**: Only `qmetaobject` and `cstr`
- **cargo-generate ready**: Create new projects instantly

## Requirements

### System (Ubuntu/Debian)

```bash
sudo apt install build-essential cmake qt6-base-dev qt6-declarative-dev libgl1-mesa-dev
```

### Fedora

```bash
sudo dnf install gcc-c++ cmake qt6-qtbase-devel qt6-qtdeclarative-devel mesa-libGL-devel
```

### Arch Linux

```bash
sudo pacman -S base-devel cmake qt6-base qt6-declarative
```

## Quick Start

### From Template (Recommended)

```bash
# Install cargo-generate (once)
cargo install cargo-generate

# Create new project (interactive)
cargo generate --git https://github.com/andresgarcia0313/rust-qt-template

# Or non-interactive (for CI/scripts)
cargo generate --git https://github.com/andresgarcia0313/rust-qt-template \
  --name my-app \
  -d description="My Qt application"

# Run
cd my-app
cargo run
```

### Manual Clone

```bash
git clone https://github.com/andresgarcia0313/rust-qt-template my-app
cd my-app
# Edit Cargo.toml to change project name
cargo run
```

## Project Structure

```
src/
├── main.rs      # Rust component + entry point
└── ui/
    └── main.qml # Declarative UI
```

## Architecture

```
┌─────────────────┐     ┌─────────────────┐
│   Rust          │     │   QML           │
│   (State)       │◄───►│   (UI)          │
│                 │     │                 │
│ qt_property!    │────►│ Binding         │
│ qt_method!      │◄────│ onClicked       │
│ qt_signal!      │────►│ Notification    │
└─────────────────┘     └─────────────────┘
```

## QObject Pattern

```rust
use cstr::cstr;
use qmetaobject::prelude::*;

#[derive(QObject)]
struct MyComponent {
    base: qt_base_class!(trait QObject),

    // Reactive property with change notification
    value: qt_property!(QString; NOTIFY value_changed),
    value_changed: qt_signal!(),

    // Method callable from QML
    update: qt_method!(fn update(&mut self) {
        self.value = "New value".into();
        self.value_changed();
    }),
}

impl Default for MyComponent {
    fn default() -> Self {
        Self {
            base: Default::default(),
            value: "Initial".into(),
            value_changed: Default::default(),
            update: Default::default(),
        }
    }
}
```

## Development

```bash
cargo run              # Run app
cargo build --release  # Optimized build (476KB)
cargo fmt              # Format code
cargo clippy           # Lint
```

## License

MIT
