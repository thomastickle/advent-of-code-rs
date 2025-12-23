# advent-of-code-rs
Advent of Code In Rust

## Running the project

### Debug Mode (Default)
To run the project in debug mode (with debug symbols and less optimization):
```powershell
cargo run
```

### Release Mode (Non-debug version)
To run the project in release mode (optimized for performance, without debug symbols):
```powershell
cargo run --release
```

## Testing
To run the unit tests:
```powershell
cargo test
```

## Benchmarking
To run the benchmarks (this will take some time as it runs multiple iterations):
```powershell
cargo bench
```
Benchmarks use the [Criterion](https://github.com/bheisler/criterion.rs) crate and will generate HTML reports in `target/criterion/report/index.html`.

### Troubleshooting Benchmarks
- **"Gnuplot not found, using plotters backend"**: This is a normal warning. Criterion falls back to the `plotters` crate to generate graphs when Gnuplot is not installed on the system. It does not affect the benchmark results.
- **"Unable to complete 100 samples in 5.0s"**: If you see this warning, the benchmark configuration has been adjusted in `benches\aoc_benchmarks.rs` by increasing the `measurement_time` to allow the benchmarks to complete reliably.
