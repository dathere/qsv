These benchmarks are compiled on an Apple Mac Mini 2023 model with an M2 Pro chip with 12 CPU cores & 32GB of RAM; a 1TB SSD primary drive & a 1TB Samsung SSD 970 Evo plus external drive running the latest macOS at the time (see [run_info_history.tsv](run_info_history.tsv)).

It uses the prebuilt, CPU optimized qsv binary variant in aarch64-apple-darwin.zip found at `https://github.com/dathere/qsv/releases/latest`. The benchmarks were performed on a million row, 41 column, 520 MB sample of NYC's 311 data.
`https://raw.githubusercontent.com/wiki/dathere/qsv/files/NYC_311_SR_2010-2020-sample-1M.7z`

Each benchmark is executed five times using hyperfine v1.18.0. Two warmup runs followed by three benchmark runs. Records per second is calculated by dividing the number of records (1M) by the mean of the three benchmark runs. All other measurements are in seconds.

The `delta (%)` column shows how much *faster* a benchmark is versus its previous qsv version, as a percentage of the previous version's mean run time (positive = faster, negative = a regression). It is empty when there is no earlier version to compare against. The `rank` column is the speed rank of a run among all recorded runs of the same benchmark (`1` = the fastest run ever recorded), ranked by mean run time.

Bear in mind that commands get expanded over time as new features are added, so the benchmarks are not necessarily an "apples-to-apples" comparison, especially with much older versions. For example, the `stats` command has had several new features added over time, so the benchmark for `stats` in v1.0.0 is not the same as the benchmark for `stats` in v21.1.0.
