# Benchmarks

## Prerequisites
- [Nightly Rust toolchain](https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust)
- *Optional* (macOS): [`cargo-instruments`](https://crates.io/crates/cargo-instruments)
  - [Xcode](https://developer.apple.com/download/release)

## Running Benchmarks
Use [`cargo bench`](https://doc.rust-lang.org/cargo/commands/cargo-bench.html) to run the benchmarks in the `benches` subdirectory.

### Xcode Instruments
If using `cargo-instruments` on macOS, you can run tests, examples, and benchmarks with the following format: `cargo instruments --template <TEMPLATE> <--example <NAME>|--bin <NAME>|--bench <NAME>>`.
- For a list of Xcode Instruments templates, run `cargo instruments -l`.
- Example: `cargo instruments -t Leaks --bench blake3` to detect memory leaks in the BLAKE3 hashing benchmark.

#### Code Signing Issues
If you encounter issues connecting Instruments to a benchmark, it may be necessary to self-sign the benchmarking executable.
1. To generate the benchmarking executables, run `cargo bench --no-run`; this should output the paths to the executables.
2. Self-sign an executable with the following:
```zsh
codesign -s - -v -f --entitlements =(echo -n '<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "https://www.apple.com/DTDs/PropertyList-1.0.dtd"\>
<plist version="1.0">
    <dict>
        <key>com.apple.security.get-task-allow</key>
        <true/>
    </dict>
</plist>') <PATH_TO_EXECUTABLE>
```
3. Set the executable as the target in Instruments and begin a new recording.
- Alternatively, you can run `xcrun xctrace record --template <TEMPLATE> --launch <PATH_TO_EXECUTABLE>` to generate a trace file that can be opened in Instruments.
- To detect memory leaks without using the Instruments GUI, you can run `leaks --atExit -- <PATH_TO_EXECUTABLE>`.

## Additional Resources
- [Performance profiling on Linux](https://rust-lang.github.io/packed_simd/perf-guide/prof/linux.html#performance-profiling-on-linux)
- [`samply`](https://github.com/mstange/samply)
- [`flamegraph`](https://github.com/flamegraph-rs/flamegraph)
