# liblo-sys

Raw FFI bindings for the [liblo](https://github.com/radarsat1/liblo) library - a lightweight OSC (Open Sound Control) implementation.

## Overview

This crate provides unsafe Rust bindings to liblo, allowing you to send and receive OSC messages in Rust applications. liblo is a C library that implements the Open Sound Control protocol for networked sound and media applications.

## Installation

### Prerequisites

- **liblo version**: Requires liblo >= 0.32
- **Rust**: Standard Rust toolchain

### System Installation

By default, this crate will search for a system-installed liblo using pkg-config and attempt to link to it.

**On NixOS:**
The package is included in the default dev-shell of the crate's flake.
```bash
nix develop .
```

**On Ubuntu/Debian:**
```bash
sudo apt-get install liblo-dev
```

**On macOS (with Homebrew):**
```bash
brew install liblo
```

## Binding generation

Bindings can be regenerated using the `generate_bindings` binary. They will be placed in `src/lo_sys.rs`.

```bash
cargo run --bin generate-bindings
```

## Contributing

Contributions are welcome! Please ensure that:
- FFI bindings are complete and accurate
- Documentation is updated for any API changes

## License

This project is licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE) or
- [MIT license](LICENSE-MIT)

at your option.

## Links

- [liblo upstream repository](https://github.com/radarsat1/liblo)
- [Open Sound Control specification](http://opensoundcontrol.org/)
