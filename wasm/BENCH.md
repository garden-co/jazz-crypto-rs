# Running Benchmarks

### Prerequisites
- [Nightly Rust toolchain](https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust)
- *Optional* (macOS): [`cargo-instruments`](https://crates.io/crates/cargo-instruments)
  - [Xcode](https://developer.apple.com/download/release)

### Running Benchmarks
Use [`cargo bench`](https://doc.rust-lang.org/cargo/commands/cargo-bench.html) to run the benchmarks in the `benches` subdirectory.

If using `cargo-instruments` on macOS, you can run tests, examples, and benchmarks with the following format: `cargo instruments --template <TEMPLATE> <--example <NAME>|--bin <NAME>|--bench <NAME>>`.
- For a list of Xcode Instruments templates, run `cargo instruments -l`.
- Example: `cargo instruments -t Leaks --bench blake3` to detect memory leaks in the BLAKE3 hashing benchmark.

## Additional Resources
- [Performance profiling on Linux](https://rust-lang.github.io/packed_simd/perf-guide/prof/linux.html#performance-profiling-on-linux)
- [`samply`](https://github.com/mstange/samply)
- [`flamegraph`](https://github.com/flamegraph-rs/flamegraph)
