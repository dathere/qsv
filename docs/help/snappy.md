# snappy

> Does streaming compression/decompression of the input using Google's Snappy framing format (more info).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/snappy.rs](https://github.com/dathere/qsv/blob/master/src/cmd/snappy.rs)** | 🚀🌐

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Snappy Options](#snappy-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Does streaming compression/decompression of the input using the Snappy framing format.
<https://github.com/google/snappy/blob/main/framing_format.txt>

It has four subcommands:
compress:   Compress the input (multithreaded).
decompress: Decompress the input (single-threaded).
check:      Quickly check if the input is a Snappy file by inspecting the
first 50 bytes of the input is valid Snappy data.
Returns exitcode 0 if the first 50 bytes is valid Snappy data,
exitcode 1 otherwise.
validate:   Validate if the ENTIRE input is a valid Snappy file.
Returns exitcode 0 if valid, exitcode 1 otherwise.

Note that most qsv commands already automatically decompresses Snappy files if the
input file has an ".sz" extension. It will also automatically compress the output
file (though only single-threaded) if the --output file has an ".sz" extension.

This command's multithreaded compression is 5-6x faster than qsv's automatic
single-threaded compression.

Also, this command is not specific to CSV data, it can compress/decompress ANY file.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_snappy.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv snappy compress [options] [<input>]
qsv snappy decompress [options] [<input>]
qsv snappy check [options] [<input>]
qsv snappy validate [options] [<input>]
qsv snappy --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument | Description |
|----------|-------------|
| `<input>` | The input file to compress/decompress. This can be a local file, stdin, or a URL (http and https schemes supported). |

<a name="snappy-options"></a>

## Snappy Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--user-agent` | string | Specify custom user agent to use when the input is a URL. It supports the following variables - $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND. Try to follow the syntax here - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent> |  |
| `--timeout` | string | Timeout for downloading URLs in seconds. | `60` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h,`<br>`--help` | flag | Display this message |  |
| `-o,`<br>`--output` | string | Write output to <output> instead of stdout. |  |
| `-j,`<br>`--jobs` | string | The number of jobs to run in parallel when compressing. When not set, its set to the number of CPUs - 1 |  |
| `-q,`<br>`--quiet` | flag | Suppress status messages to stderr. |  |
| `-p,`<br>`--progressbar` | flag | Show download progress bars. Only valid for URL input. |  |

---
**Source:** [`src/cmd/snappy.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/snappy.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
