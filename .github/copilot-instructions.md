# Project Overview

This project is a command-line-application that aims to be the fastest, data-wrangling toolkit for tabular data.
See https://github.com/dathere/qsv?tab=readme-ov-file#goals--non-goals for a detailed breakdown of its goals.
It is built using the latest stable Rust.

## Folder Structure

- `/.github`: Contains all GitHub specific repository configuration files - including the issue template, all the Workflows and dependabot configuration.
- `/contrib`: Contains contributions from the ecosystem. Currently, its been seeded with tab completions and Jupyter notebooks.
- `/docs`: Contains documentation for the project, including API specifications and user guides.
- `/resources`: Contains examples, vendored code, and test files used by some tests.
- `/scripts`: Contains shell scripts that use qsv. Currently, the benchmark script that is used to populate https://qsv.dathere.com/benchmarks lives here.
- `/src`: The source code can be found here.
- `/tests`: The test suite lives here.

## Libraries and Frameworks

- Rust.
- Has a large dependency tree as detailed in Cargo.toml.
- Per the project's goals, qsv aims to have the latest version of its dependencies.
- The primary maintainer - @jqnatividad, actively creates pull requests against its dependencies
  to ensure they're also up-to-date.
- Polars is a central dependency.

## Coding Standards

- As qsv uses the latest Rust stable, it aims to use the latest langugage features.
- It makes extensive use of "unsafe" Rust, primarily for performance. All unsafe blocks have accompanying
  comments preceeded with the prefix "safety:" to document the unsafe reason, and why it's actually safe
- It also uses "unwrap()" with an accompanying "safety:" comment
