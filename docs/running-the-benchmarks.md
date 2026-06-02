# Running the Benchmarks

All the solutions can be timed in order to see which of multiple different implementation is faster.

Unfortunately, I can either use a sensible benchmark framework per language and forgo the exact same setup between the languages or write my own benchmark harness that does the same thing, but perhaps with a sub-par technique.

## Rust

For Rust, I use Criterion. To run the benchmarks on Problem 8, one runs this:

```bash
cargo bench --bench pe_bench -- '8/'
```

If one wants a less elaborate benchmark, one can add `--quick` to the end of the command line.