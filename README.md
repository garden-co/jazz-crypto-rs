# Jazz Crypto RS

A Rust implementation of cryptographic primitives for the Jazz project, compiled to WebAssembly.

## Module Structure

The codebase is organized into the following modules:

```
src
├── crypto
│   ├── ed25519.rs // Ed25519 functions for signing and verification
│   ├── encrypt.rs // High-level encryption functions
│   ├── seal.rs // High-level sealing and unsealing functions
│   ├── sign.rs // High-level signing and verification functions
│   ├── x25519.rs // X25519 key exchange
│   └── xsalsa20.rs // XSalsa20 and XSalsa20-Poly1305 encryption
├── error.rs // Error types for cryptographic operations
├── hash
│   └── blake3.rs // BLAKE3 hashing functionality
└── lib.rs // Main entry point for the library
```

## Features

- Ed25519 signing and verification
- X25519 key exchange
- XSalsa20 and XSalsa20-Poly1305 encryption
- BLAKE3 hashing with incremental state updates
- Secure nonce generation
- WebAssembly bindings for all operations

## Usage

The library exposes WebAssembly-compatible functions for all cryptographic operations. See the individual module files for detailed documentation of available functions.

## Installation

Get a working Rust environment (rustup). 

```bash
rustup install stable
rustup default stable
```

[Install `wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/).

Then add wasm target to your toolchain:

```bash
rustup target add wasm32-unknown-unknown
```

## Build

```bash
wasm-pack build --target web
```

## Development

Install the [`rust-analyzer` extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) to get Rust support in VSCode. `rust-analyzer` expects a `Cargo.toml` file in the root of the project, so configure the workspace setting with the root directory of the project:

```json
// .vscode/settings.json
{
  "rust-analyzer.linkedProjects": [
    "wasm/Cargo.toml"
  ]
}
```


## Test

Test both Rust source code and Node.js' build.

```bash
pnpm test
```
