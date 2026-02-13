# safenames

![CKAN](../images/ckan.png)

> Modify headers of a CSV to only have "safe" names - guaranteed "database-ready"/"CKAN-ready" names.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/safenames.rs](https://github.com/dathere/qsv/blob/master/src/cmd/safenames.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Safenames Options](#safenames-options) | [Common Options](#common-options)

## Description [↩](#nav)

Modify headers of a CSV to only have "safe" names - guaranteed "database-ready" names
(optimized specifically for PostgreSQL column identifiers).

Fold to lowercase. Trim leading & trailing whitespaces. Replace whitespace/non-alphanumeric
characters with _. If name starts with a number & check_first_char is true, prepend the unsafe prefix.
If a header with the same name already exists, append a sequence suffix (e.g. col, col_2, col_3).
Names are limited to 60 characters in length. Empty names are replaced with the unsafe prefix.

In addition, specifically because of CKAN Datastore requirements:
- Headers with leading underscores are replaced with "unsafe_" prefix.
- Headers that are named "_id" are renamed to "reserved__id".

These CKAN Datastore options can be configured via the --prefix & --reserved options, respectively.

In Always (a) and Conditional (c) mode, returns number of modified headers to stderr,
and sends CSV with safe headers output to stdout.

In Verify (v) mode, returns number of unsafe headers to stderr.
In Verbose (V) mode, returns number of headers; duplicate count and unsafe & safe headers to stderr.
No stdout output is generated in Verify and Verbose mode.

In JSON (j) mode, returns Verbose mode info in minified JSON to stdout.
In Pretty JSON (J) mode, returns Verbose mode info in pretty printed JSON to stdout.

Given data.csv:
c1,12_col,Col with Embedded Spaces,,Column!@Invalid+Chars,c1
1,a2,a3,a4,a5,a6

```console
$ qsv safenames data.csv
```

c1,unsafe_12_col,col_with_embedded_spaces,unsafe_,column__invalid_chars,c1_2
1,a2,a3,a4,a5,a6
stderr: 5

Conditionally rename headers, allowing "quoted identifiers":
```console
$ qsv safenames --mode c data.csv
```

c1,unsafe_12_col,Col with Embedded Spaces,unsafe_,column__invalid_chars,c1_2
1,a2,a3,a4,a5,a6
stderr: 4

Verify how many "unsafe" headers are found:
```console
$ qsv safenames --mode v data.csv
```

stderr: 4

Verbose mode:
```console
$ qsv safenames --mode V data.csv
```

stderr: 6 header/s
1 duplicate/s: "c1:2"
4 unsafe header/s: ["12_col", "Col with Embedded Spaces", "", "Column!@Invalid+Chars"]
1 safe header/s: ["c1"]

Note that even if "Col with Embedded Spaces" is technically safe, it is generally discouraged.
Though it can be created as a "quoted identifier" in PostgreSQL, it is still marked "unsafe"
by default, unless mode is set to "conditional."

It is discouraged because the embedded spaces can cause problems later on.
(see <https://lerner.co.il/2013/11/30/quoting-postgresql/> for more info).

For more examples, see <https://github.com/dathere/qsv/blob/master/tests/test_safenames.rs>.


## Usage [↩](#nav)

```console
qsv safenames [options] [<input>]
qsv safenames --help
```

## Safenames Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--mode` | string | Rename header names to "safe" names - i.e. guaranteed "database-ready" names. It has six modes - conditional, always, verify, Verbose, with Verbose having two submodes - JSON & pretty JSON. | `Always` |
| `--reserved` | string | Comma-delimited list of additional case-insensitive reserved names that should be considered "unsafe." If a header name is found in the reserved list, it will be prefixed with "reserved_". | `_id` |
| `--prefix` | string | Certain systems do not allow header names to start with "_" (e.g. CKAN Datastore). This option allows the specification of the unsafe prefix to use when a header starts with "_". | `unsafe_` |

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. Note that no output is generated for Verify and Verbose modes. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/safenames.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/safenames.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
