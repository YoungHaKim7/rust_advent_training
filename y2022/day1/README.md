# cargo bench (criterion.rs)

- https://github.com/bheisler/criterion.rs


- https://seenaburns.com/benchmarking-rust-with-cargo-bench/

# hyperfind

- final Result

```bash
$ hyperfine --warmup 3 'a01_day1_final_trait_optimization/target/release/a01_day1_final_trait_optimization' 'a01_day_add_iterator/target/release/a01_day_add_iterator'
Benchmark 1: a01_day1_final_trait_optimization/target/release/a01_day1_final_trait_optimization
  Time (mean ± σ):     664.7 µs ± 157.8 µs    [User: 327.3 µs, System: 234.4 µs]
  Range (min … max):   415.9 µs … 1533.1 µs    919 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

Benchmark 2: a01_day_add_iterator/target/release/a01_day_add_iterator
  Time (mean ± σ):     641.3 µs ± 141.3 µs    [User: 319.6 µs, System: 224.7 µs]
  Range (min … max):   426.5 µs … 1393.5 µs    902 runs

```

```bash

$ hyperfine --warmup 3 'a01_day1_add_trait_iterator/target/release/a01_day1_add_trait_iterator' 'a01_day_add_iterator/target/release/a01_day_add_iterator'
Benchmark 1: a01_day1_add_trait_iterator/target/release/a01_day1_add_trait_iterator
  Time (mean ± σ):     666.0 µs ± 154.6 µs    [User: 325.4 µs, System: 241.3 µs]
  Range (min … max):   432.2 µs … 1566.0 µs    902 runs

Benchmark 2: a01_day_add_iterator/target/release/a01_day_add_iterator
  Time (mean ± σ):     647.1 µs ± 150.7 µs    [User: 321.0 µs, System: 237.3 µs]
  Range (min … max):   410.0 µs … 1377.9 µs    1007 runs
  
```
