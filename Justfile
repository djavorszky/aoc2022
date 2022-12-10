check:
    cargo clippy

# Run the tests
test:
    cargo nextest run

# Run the benchmarks
bench:
    cargo bench

# Run the stuff
run:
    cargo run