{
  description = "Rust WebAssembly Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Latest stable Rust
        rust = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust
            rust
            rust-analyzer
            wasm-pack
            
            # Node.js
            nodejs_20
            nodePackages.pnpm
            
            # Tools
            pkg-config
            
            # Optional but recommended
            cargo-watch
            cargo-audit
          ];

          shellHook = ''
            echo "Rust WebAssembly Development Environment"
            echo "Available commands:"
            echo "  pnpm build    - Build the WebAssembly module"
            echo "  pnpm test     - Run tests"
          '';
        };
      }
    );
} 