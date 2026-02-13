# sniff

> Quickly sniff & infer CSV metadata (delimiter, header row, preamble rows, quote character, flexible, is_utf8, average record length, number of records, content length & estimated number of records if sniffing a CSV on a URL, number of fields, field names & data types). It is also a general mime type detector.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/sniff.rs](https://github.com/dathere/qsv/blob/master/src/cmd/sniff.rs)** | 📇🌐🤖 ![CKAN](../images/ckan.png)

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Sniff Options](#sniff-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Quickly sniff the first n rows and infer CSV metadata (delimiter, header row, number of
preamble rows, quote character, flexible, is_utf8, average record length, number of records,
content length and estimated number of records if sniffing a URL, file size, number of fields,
field names & data types).

`sniff` is also a mime type detector, returning the detected mime type, file size and
last modified date. If --no-infer is enabled, it doesn't even bother to infer the CSV's schema.
This makes it useful for accelerated CKAN harvesting and for checking stale/broken resource URLs.

When qsv is compiled with the optional `magika` feature, it uses Google's Magika
AI-powered content detection to identify file types with high accuracy. Magika detects over
200 content types including CSV, MS Office/Open Document files, JSON, XML, PDF, PNG, JPEG
and many more.
See <https://opensource.googleblog.com/2025/11/announcing-magika-10-now-faster-smarter.html>.

When the `magika` feature is not enabled in a build (e.g., MUSL builds, qsvlite, qsvdp), it falls back
to the file-format crate which provides basic MIME type detection.

NOTE: This command "sniffs" a CSV's schema by sampling the first n rows (default: 1000)
of a file. Its inferences are sometimes wrong if the the file is too small to infer a pattern
or if the CSV has unusual formatting - with atypical delimiters, quotes, etc.

In such cases, selectively use the --sample, --delimiter and --quote options to improve
the accuracy of the sniffed schema.

If you want more robust, guaranteed schemata, use the "schema" or "stats" commands
instead as they scan the entire file. However, they only work on local files and well-formed
CSVs, unlike `sniff` which can work with remote files, various CSV dialects and is very fast
regardless of file size.


<a name="examples"></a>

## Examples [↩](#nav)

> Sniff a local CSV file

```console
qsv sniff data.csv
```

> Sniff a remote TSV file over HTTPS

```console
qsv sniff https://example.com/data.tsv
```

> Get the mime type of a remote file without inferring the CSV schema

```console
qsv sniff --no-infer https://example.com/data.xlsx
```

> Sniff the first 20 percent of a SSV file

```console
qsv sniff --sample 0.20 data.ssv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_sniff.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv sniff [options] [<input>]
qsv sniff --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument | Description |
|----------|-------------|
| &nbsp;`<input>`&nbsp; | The file to sniff. This can be a local file, stdin or a URL (http and https schemes supported). |

<a name="sniff-options"></a>

## Sniff Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--sample`&nbsp; | string | First n rows to sample to sniff out the metadata. When sample size is between 0 and 1 exclusive, it is treated as a percentage of the CSV to sample (e.g. 0.20 is 20 percent). When it is zero, the entire file will be sampled. When the input is a URL, the sample size dictates how many lines to sample without having to download the entire file. Ignored when --no-infer is enabled. | `1000` |
| &nbsp;`--prefer-dmy`&nbsp; | flag | Prefer to parse dates in dmy format. Otherwise, use mdy format. Ignored when --no-infer is enabled. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The delimiter for reading CSV data. Specify this when the delimiter is known beforehand, as the delimiter inferencing algorithm can sometimes fail. Must be a single ascii character. |  |
| &nbsp;`--quote`&nbsp; | string | The quote character for reading CSV data. Specify this when the quote character is known beforehand, as the quote char inferencing algorithm can sometimes fail. Must be a single ascii character - typically, double quote ("), single quote ('), or backtick (`). |  |
| &nbsp;`--json`&nbsp; | flag | Return results in JSON format. |  |
| &nbsp;`--pretty-json`&nbsp; | flag | Return results in pretty JSON format. |  |
| &nbsp;`--save-urlsample`&nbsp; | string | Save the URL sample to a file. Valid only when input is a URL. |  |
| &nbsp;`--timeout`&nbsp; | string | Timeout when sniffing URLs in seconds. If 0, no timeout is used. | `30` |
| &nbsp;`--user-agent`&nbsp; | string | Specify custom user agent to use when sniffing a CSV on a URL. It supports the following variables - $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND. Try to follow the syntax here - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent> |  |
| &nbsp;`--stats-types`&nbsp; | flag | Use the same data type names as `stats`. (Unsigned, Signed => Integer, Text => String, everything else the same) |  |
| &nbsp;`--no-infer`&nbsp; | flag | Do not infer the schema. Only return the file's mime type, size and last modified date. Use this to use sniff as a general mime type detector. Note that CSV and TSV files will only be detected as mime type plain/text in this mode. |  |
| &nbsp;`--just-mime`&nbsp; | flag | Only return the file's mime type. Use this to use sniff as a general mime type detector. Synonym for --no-infer. |  |
| &nbsp;`-Q,`<br>`--quick`&nbsp; | flag | When sniffing a non-CSV remote file, only download the first chunk of the file before attempting to detect the mime type. This is faster but less accurate as some mime types cannot be detected with just the first downloaded chunk. |  |
| &nbsp;`--harvest-mode`&nbsp; | flag | This is a convenience flag when using sniff in CKAN harvesters. It is equivalent to --quick --timeout 10 --stats-types --json and --user-agent "CKAN-harvest/$QSV_VERSION ($QSV_TARGET; $QSV_BIN_NAME)" |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars. Only valid for URL input. |  |

---
**Source:** [`src/cmd/sniff.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/sniff.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
