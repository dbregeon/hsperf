## Run Performance Analysis (Ubuntu)

After intalling perf and flamegraph for cargo.

Adjust the kernel event access:

```bash
sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'
```

Run flamegraph:
```bash
export CARGO_PROFILE_RELEASE_DEBUG=true && cargo flamegraph --example print_hsperfdata -o target/flamegraph.svg -c "record -e branch-misses -c 100 --call-graph lbr -g"
```

## Code coverage

``` bash
cargo install cargo-llvm-cov --locked

cargo llvm-cov
```

## Visualize coverage in VSCode using watch/coverage gutters

```bash
cargo install cargo-nextest --locked

cargo llvm-cov --lcov --output-path ./target/lcov.info

```