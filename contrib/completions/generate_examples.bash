#!/usr/bin/env bash
# Generate shell completion files for qsv.
# Must be run from contrib/completions/ within the qsv repository,
# since it reads USAGE text from ../../src/cmd/*.rs at runtime.

set -e

if [ ! -d "../../src/cmd" ]; then
    echo "Error: Cannot find ../../src/cmd/ directory." >&2
    echo "This script must be run from contrib/completions/ within the qsv repository." >&2
    exit 1
fi

# Assuming examples folder exists
cargo run -- bash > examples/qsv.bash
cargo run -- zsh > examples/qsv.zsh
cargo run -- powershell > examples/qsv.ps1
cargo run -- fish > examples/qsv.fish
cargo run -- elvish > examples/qsv.elv
cargo run -- fig > examples/qsv.fig.js
cargo run -- nushell > examples/qsv.nu
