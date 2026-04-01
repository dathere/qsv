# blake3

> Compute or check [BLAKE3](https://github.com/BLAKE3-team/BLAKE3/?tab=readme-ov-file#blake3) hashes of files.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/blake3.rs](https://github.com/dathere/qsv/blob/master/src/cmd/blake3.rs)** | [🚀](TableOfContents.md#legend "multithreaded even without an index.")

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Blake3 Options](#blake3-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Compute cryptographic hashes of files using blake3.

This command is functionally similar to b3sum, providing fast, parallel blake3 hashing
of one or more files. It supports keyed hashing, key derivation, variable-length output,
and checksum verification. When no file is given, or when "-" is given, reads stdin.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_blake3.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv blake3 [options] [<input>...]
qsv blake3 --help
```

<a name="blake3-options"></a>

## Blake3 Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--keyed`&nbsp; | flag | Use the keyed mode, reading the 32-byte key from stdin. When using --keyed, file arguments are required (cannot also read data from stdin). |  |
| &nbsp;`--derive-key`&nbsp; | string | Use the key derivation mode, with the given context string. Cannot be used with --keyed. |  |
| &nbsp;`-l,`<br>`--length`&nbsp; | string | The number of output bytes, before hex encoding. | `32` |
| &nbsp;`--no-mmap`&nbsp; | flag | Disable memory mapping. Also disables multithreading. |  |
| &nbsp;`--no-names`&nbsp; | flag | Omit filenames in the output. |  |
| &nbsp;`--raw`&nbsp; | flag | Write raw output bytes to stdout, rather than hex. Only a single input is allowed. --no-names is implied. |  |
| &nbsp;`--tag`&nbsp; | flag | Output checksums in tagged format. |  |
| &nbsp;`-c,`<br>`--check`&nbsp; | flag | Read blake3 sums from the input files and check them. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel for hashing. When not set, uses the number of CPUs detected. Set to 1 to disable multithreading. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Skip printing OK for each checked file. Must be used with --check. |  |

---
**Source:** [`src/cmd/blake3.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/blake3.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
