default:
  just --list

check:
    cargo clippy

# Run the tests
test DAY:
    cargo nextest run -- day{{DAY}}

# Run the benchmarks
bench:
    cargo bench

# Run the stuff
run:
    cargo run
