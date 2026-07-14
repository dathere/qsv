# SCRIPTS

This directory contains various scripts for the project.

## benchmarks.sh - configurable benchmarking script
This script runs the benchmarks for the project. It takes one argument:

Usage: ./benchmarks.sh <argument>
  where <argument> is a substring pattern of the benchmark name.
  e.g. ./benchmarks.sh sort - will run benchmarks with "sort" in the benchmark name
  if <argument> is omitted, all benchmarks are executed.

  if <argument> is "reset", the benchmark data will be downloaded and prepared again.
   though the results/benchmark_results.csv and results/run_info_history.tsv historical
   archives will be preserved.
  if <argument> is "clean", temporary files will be deleted.
  if <argument> is "setup", setup and install all the required tools.
  if <argument> is "help", help text is displayed.

This script benchmarks Quicksilver (qsv) using a 520mb, 41 column, 1M row sample of
NYC's 311 data. If it doesn't exist on your system, it will be downloaded for you.

Though this script was primarily created to maintain the Benchmark page on the qsv site,
it was also designed to be a useful tool for users to benchmark qsv on their own systems,
so it be can run on hardware and workloads that reflect your requirements/environment.

See [benchmarks.sh](benchmarks.sh) for more details.

> [!NOTE]
> **Two binaries are benchmarked.** All commands run against the PGO-optimized prebuilt
> `qsv` (`qsv_bin`), except the `py_*` benchmarks, which run against a python-enabled
> `qsvpyNNN` flavor (`qsv_py_bin`, defaults to `qsvpy313`) since the `python` feature is
> not compiled into the regular prebuilt qsv. The py benchmarks are skipped if no
> python-enabled binary is found. Because of this, the Luau-vs-Python comparison
> (qsv issue #1351) is not strictly binary-for-binary: Luau runs on the PGO-optimized
> binary while Python runs on the non-PGO `qsvpyNNN` flavor. The comparison still reflects
> the interpreter-overhead gap it was designed to show, as PGO has little effect on the
> interpreter hot loops.

## gen_benchmark_viz.py - interactive benchmark dashboard generator

Regenerates `benchmarks/index.html`, an interactive [`qsv viz`](../docs/help/viz.md)
dashboard of the benchmark results in `results/*.csv` (produced by `benchmarks.sh`). It
dogfoods qsv end-to-end — shaping the data with `qsv` and charting it with `qsv viz` — so
the dashboard doubles as a `viz` showcase spanning bar, grouped-bar, line, heatmap, treemap
and box traces.

```bash
cargo build --bin qsv -F all_features   # or set QSV_BIN
python3 scripts/gen_benchmark_viz.py
git add benchmarks && git commit        # Pages redeploys on push to master
```

The interactive dashboard is deployed to GitHub Pages by
[`viz-gallery-pages.yml`](../.github/workflows/viz-gallery-pages.yml) →
<https://dathere.github.io/qsv/benchmarks/>. A static `hero.png` is also rendered
(best-effort, needs the `viz_static` feature + a local browser).

Publishing the Wiki **Benchmarks** page is a separate manual step (interactive Plotly can't
render in wiki markdown): copy the generated `benchmarks/Benchmarks.wiki.md` starter page
into the `qsv.wiki` repo as `Benchmarks.md` and push. See the script's docstring for the
exact commands.

## misc/docopt-wordlist.bash - optional qsv tab completion support
qsv's command-line options are quite extensive. Thankfully, since it uses [docopt](http://docopt.org/) for CLI processing,
we can take advantage of [docopt.rs' tab completion support](https://github.com/docopt/docopt.rs#tab-completion-support) to make it
easier to use qsv at the command-line (currently, only bash shell is supported):

```bash
# install docopt-wordlist
cargo install docopt

# IMPORTANT: run these commands from the root directory of your qsv git repository
# to setup bash qsv tab completion
echo "DOCOPT_WORDLIST_BIN=\"$(which docopt-wordlist)"\" >> $HOME/.bash_completion
echo "source \"$(pwd)/scripts/misc/docopt-wordlist.bash\"" >> $HOME/.bash_completion
echo "complete -F _docopt_wordlist_commands qsv" >> $HOME/.bash_completion
```
