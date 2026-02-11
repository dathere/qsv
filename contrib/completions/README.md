# qsv completions - bash, zsh, fish, powershell, nushell, fig, elvish

Generate shell completions for qsv including the following shells:

-   bash
-   zsh
-   powershell
-   fish
-   nushell
-   fig
-   elvish

Completions are **auto-generated** from qsv's `static USAGE` text in `src/cmd/*.rs`, so they stay in sync with the CLI automatically. No manual command definition files to maintain.

> Status as of qsv release 16.0.0: Completions for all commands except `applydp` and `generate` (`applydp` is specific to DataPusher+ and `generate` is not usually distributed with qsv anymore) are auto-generated with short and long flags, subcommands, and value-taking hints. Completions may not account for file paths (you may need to explicitly use a relative path for example starting with `./` to begin file completions). Not all shells have been verified to work with the generated completions.

## Usage

You may use one of the completions in the `examples` folder or follow the following instructions to generate them.

To generate completions for all shells into an examples folder run the `generate_examples.bash` script from `contrib/completions/` within the qsv repository:

```bash
bash generate_examples.bash
```

To generate a completion manually run:

```bash
cargo run -- <shell>
```

Replace `<shell>` with any of the shells mentioned above.

The completions output should be printed to your terminal. You may redirect this to a file. For example for Bash completions:

```bash
cargo run -- bash > completions.bash
```

Then run them as your shell intends it to be ran.

## How It Works

At runtime, the tool:

1. Locates the qsv repository root (walks up from CWD looking for `Cargo.toml` + `src/cmd/`)
2. Reads the `static USAGE` text from each `src/cmd/*.rs` file
3. Parses USAGE text with `qsv_docopt` to discover flags, option types, and subcommands
4. Builds a `clap::Command` tree and feeds it to `clap_complete` for the requested shell

## Updating Completions

When qsv adds, removes, or modifies commands or flags, just regenerate:

```bash
bash generate_examples.bash
```

No other changes are needed in this project.
