const completion: Fig.Spec = {
  name: "qsv",
  description: "",
  subcommands: [
    {
      name: "apply",
      subcommands: [
        {
          name: "calcconv",
          options: [
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: ["-C", "--comparand"],
              isRepeatable: true,
              args: {
                name: "comparand",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: ["-R", "--replacement"],
              isRepeatable: true,
              args: {
                name: "replacement",
                isOptional: true,
              },
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "dynfmt",
          options: [
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: ["-C", "--comparand"],
              isRepeatable: true,
              args: {
                name: "comparand",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: ["-R", "--replacement"],
              isRepeatable: true,
              args: {
                name: "replacement",
                isOptional: true,
              },
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "emptyreplace",
          options: [
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: ["-C", "--comparand"],
              isRepeatable: true,
              args: {
                name: "comparand",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: ["-R", "--replacement"],
              isRepeatable: true,
              args: {
                name: "replacement",
                isOptional: true,
              },
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "operations",
          options: [
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: ["-C", "--comparand"],
              isRepeatable: true,
              args: {
                name: "comparand",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: ["-R", "--replacement"],
              isRepeatable: true,
              args: {
                name: "replacement",
                isOptional: true,
              },
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "calcconv",
            },
            {
              name: "dynfmt",
            },
            {
              name: "emptyreplace",
            },
            {
              name: "operations",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
            isOptional: true,
          },
        },
        {
          name: ["-C", "--comparand"],
          isRepeatable: true,
          args: {
            name: "comparand",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-f", "--formatstr"],
          isRepeatable: true,
          args: {
            name: "formatstr",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-r", "--rename"],
          isRepeatable: true,
          args: {
            name: "rename",
            isOptional: true,
          },
        },
        {
          name: ["-R", "--replacement"],
          isRepeatable: true,
          args: {
            name: "replacement",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "behead",
      options: [
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-f", "--flexible"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "blake3",
      options: [
        {
          name: "--derive-key",
          isRepeatable: true,
          args: {
            name: "derive-key",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-l", "--length"],
          isRepeatable: true,
          args: {
            name: "length",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--check"],
        },
        {
          name: "--keyed",
        },
        {
          name: "--no-mmap",
        },
        {
          name: "--no-names",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--raw",
        },
        {
          name: "--tag",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "cat",
      subcommands: [
        {
          name: "columns",
          options: [
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-g", "--group"],
              isRepeatable: true,
              args: {
                name: "group",
                isOptional: true,
              },
            },
            {
              name: ["-N", "--group-name"],
              isRepeatable: true,
              args: {
                name: "group-name",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: "--flexible",
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--pad"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "rows",
          options: [
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-g", "--group"],
              isRepeatable: true,
              args: {
                name: "group",
                isOptional: true,
              },
            },
            {
              name: ["-N", "--group-name"],
              isRepeatable: true,
              args: {
                name: "group-name",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: "--flexible",
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--pad"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "rowskey",
          options: [
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-g", "--group"],
              isRepeatable: true,
              args: {
                name: "group",
                isOptional: true,
              },
            },
            {
              name: ["-N", "--group-name"],
              isRepeatable: true,
              args: {
                name: "group-name",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: "--flexible",
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--pad"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "columns",
            },
            {
              name: "rows",
            },
            {
              name: "rowskey",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-g", "--group"],
          isRepeatable: true,
          args: {
            name: "group",
            isOptional: true,
          },
        },
        {
          name: ["-N", "--group-name"],
          isRepeatable: true,
          args: {
            name: "group-name",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--flexible",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-p", "--pad"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "clipboard",
      options: [
        {
          name: ["-s", "--save"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "color",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-t", "--title"],
          isRepeatable: true,
          args: {
            name: "title",
            isOptional: true,
          },
        },
        {
          name: ["-C", "--color"],
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-n", "--row-numbers"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "count",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-f", "--flexible"],
        },
        {
          name: ["-H", "--human-readable"],
        },
        {
          name: "--json",
        },
        {
          name: "--low-memory",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--no-polars",
        },
        {
          name: "--width",
        },
        {
          name: "--width-no-delims",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "datefmt",
      options: [
        {
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
            isOptional: true,
          },
        },
        {
          name: "--default-tz",
          isRepeatable: true,
          args: {
            name: "default-tz",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--formatstr",
          isRepeatable: true,
          args: {
            name: "formatstr",
            isOptional: true,
          },
        },
        {
          name: "--input-tz",
          isRepeatable: true,
          args: {
            name: "input-tz",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--output-tz",
          isRepeatable: true,
          args: {
            name: "output-tz",
            isOptional: true,
          },
        },
        {
          name: ["-r", "--rename"],
          isRepeatable: true,
          args: {
            name: "rename",
            isOptional: true,
          },
        },
        {
          name: ["-R", "--ts-resolution"],
          isRepeatable: true,
          args: {
            name: "ts-resolution",
            isOptional: true,
          },
        },
        {
          name: "--keep-zero-time",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--utc",
        },
        {
          name: "--zulu",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "dedup",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-D", "--dupes-output"],
          isRepeatable: true,
          args: {
            name: "dupes-output",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: ["-H", "--human-readable"],
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-N", "--numeric"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--sorted",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "describegpt",
      options: [
        {
          name: "--addl-cols-list",
          isRepeatable: true,
          args: {
            name: "addl-cols-list",
            isOptional: true,
          },
        },
        {
          name: "--addl-props",
          isRepeatable: true,
          args: {
            name: "addl-props",
            isOptional: true,
          },
        },
        {
          name: ["-k", "--api-key"],
          isRepeatable: true,
          args: {
            name: "api-key",
            isOptional: true,
          },
        },
        {
          name: ["-u", "--base-url"],
          isRepeatable: true,
          args: {
            name: "base-url",
            isOptional: true,
          },
        },
        {
          name: "--cache-dir",
          isRepeatable: true,
          args: {
            name: "cache-dir",
            isOptional: true,
          },
        },
        {
          name: "--ckan-api",
          isRepeatable: true,
          args: {
            name: "ckan-api",
            isOptional: true,
          },
        },
        {
          name: "--ckan-token",
          isRepeatable: true,
          args: {
            name: "ckan-token",
            isOptional: true,
          },
        },
        {
          name: "--disk-cache-dir",
          isRepeatable: true,
          args: {
            name: "disk-cache-dir",
            isOptional: true,
          },
        },
        {
          name: "--enum-threshold",
          isRepeatable: true,
          args: {
            name: "enum-threshold",
            isOptional: true,
          },
        },
        {
          name: "--export-prompt",
          isRepeatable: true,
          args: {
            name: "export-prompt",
            isOptional: true,
          },
        },
        {
          name: "--format",
          isRepeatable: true,
          args: {
            name: "format",
            isOptional: true,
          },
        },
        {
          name: "--freq-options",
          isRepeatable: true,
          args: {
            name: "freq-options",
            isOptional: true,
          },
        },
        {
          name: "--language",
          isRepeatable: true,
          args: {
            name: "language",
            isOptional: true,
          },
        },
        {
          name: ["-t", "--max-tokens"],
          isRepeatable: true,
          args: {
            name: "max-tokens",
            isOptional: true,
          },
        },
        {
          name: ["-m", "--model"],
          isRepeatable: true,
          args: {
            name: "model",
            isOptional: true,
          },
        },
        {
          name: "--num-examples",
          isRepeatable: true,
          args: {
            name: "num-examples",
            isOptional: true,
          },
        },
        {
          name: "--num-tags",
          isRepeatable: true,
          args: {
            name: "num-tags",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-p", "--prompt"],
          isRepeatable: true,
          args: {
            name: "prompt",
            isOptional: true,
          },
        },
        {
          name: "--prompt-file",
          isRepeatable: true,
          args: {
            name: "prompt-file",
            isOptional: true,
          },
        },
        {
          name: "--sample-size",
          isRepeatable: true,
          args: {
            name: "sample-size",
            isOptional: true,
          },
        },
        {
          name: "--score-max-retries",
          isRepeatable: true,
          args: {
            name: "score-max-retries",
            isOptional: true,
          },
        },
        {
          name: "--score-threshold",
          isRepeatable: true,
          args: {
            name: "score-threshold",
            isOptional: true,
          },
        },
        {
          name: "--session",
          isRepeatable: true,
          args: {
            name: "session",
            isOptional: true,
          },
        },
        {
          name: "--session-len",
          isRepeatable: true,
          args: {
            name: "session-len",
            isOptional: true,
          },
        },
        {
          name: "--sql-results",
          isRepeatable: true,
          args: {
            name: "sql-results",
            isOptional: true,
          },
        },
        {
          name: "--stats-options",
          isRepeatable: true,
          args: {
            name: "stats-options",
            isOptional: true,
          },
        },
        {
          name: "--tag-vocab",
          isRepeatable: true,
          args: {
            name: "tag-vocab",
            isOptional: true,
          },
        },
        {
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: "--truncate-str",
          isRepeatable: true,
          args: {
            name: "truncate-str",
            isOptional: true,
          },
        },
        {
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
            isOptional: true,
          },
        },
        {
          name: "--addl-cols",
        },
        {
          name: ["-A", "--all"],
        },
        {
          name: "--description",
        },
        {
          name: "--dictionary",
        },
        {
          name: "--fewshot-examples",
        },
        {
          name: "--flush-cache",
        },
        {
          name: "--forget",
        },
        {
          name: "--fresh",
        },
        {
          name: "--no-cache",
        },
        {
          name: "--no-score-sql",
        },
        {
          name: "--prepare-context",
        },
        {
          name: "--process-response",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--redis-cache",
        },
        {
          name: "--tags",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "diff",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--delimiter-left",
          isRepeatable: true,
          args: {
            name: "delimiter-left",
            isOptional: true,
          },
        },
        {
          name: "--delimiter-output",
          isRepeatable: true,
          args: {
            name: "delimiter-output",
            isOptional: true,
          },
        },
        {
          name: "--delimiter-right",
          isRepeatable: true,
          args: {
            name: "delimiter-right",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-k", "--key"],
          isRepeatable: true,
          args: {
            name: "key",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--sort-columns",
          isRepeatable: true,
          args: {
            name: "sort-columns",
            isOptional: true,
          },
        },
        {
          name: "--drop-equal-fields",
        },
        {
          name: "--no-headers-left",
        },
        {
          name: "--no-headers-output",
        },
        {
          name: "--no-headers-right",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "edit",
      options: [
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-i", "--in-place"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "enum",
      options: [
        {
          name: "--constant",
          isRepeatable: true,
          args: {
            name: "constant",
            isOptional: true,
          },
        },
        {
          name: "--copy",
          isRepeatable: true,
          args: {
            name: "copy",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--hash",
          isRepeatable: true,
          args: {
            name: "hash",
            isOptional: true,
          },
        },
        {
          name: "--increment",
          isRepeatable: true,
          args: {
            name: "increment",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--start",
          isRepeatable: true,
          args: {
            name: "start",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--uuid4",
        },
        {
          name: "--uuid7",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "excel",
      options: [
        {
          name: "--cell",
          isRepeatable: true,
          args: {
            name: "cell",
            isOptional: true,
          },
        },
        {
          name: "--date-format",
          isRepeatable: true,
          args: {
            name: "date-format",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--error-format",
          isRepeatable: true,
          args: {
            name: "error-format",
            isOptional: true,
          },
        },
        {
          name: "--header-row",
          isRepeatable: true,
          args: {
            name: "header-row",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: "--metadata",
          isRepeatable: true,
          args: {
            name: "metadata",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--range",
          isRepeatable: true,
          args: {
            name: "range",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--sheet"],
          isRepeatable: true,
          args: {
            name: "sheet",
            isOptional: true,
          },
        },
        {
          name: "--table",
          isRepeatable: true,
          args: {
            name: "table",
            isOptional: true,
          },
        },
        {
          name: "--flexible",
        },
        {
          name: "--keep-zero-time",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--trim",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "exclude",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-v", "--invert"],
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "explode",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-r", "--rename"],
          isRepeatable: true,
          args: {
            name: "rename",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "extdedup",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-D", "--dupes-output"],
          isRepeatable: true,
          args: {
            name: "dupes-output",
            isOptional: true,
          },
        },
        {
          name: "--memory-limit",
          isRepeatable: true,
          args: {
            name: "memory-limit",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--temp-dir",
          isRepeatable: true,
          args: {
            name: "temp-dir",
            isOptional: true,
          },
        },
        {
          name: ["-H", "--human-readable"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--no-output",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "extsort",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: "--memory-limit",
          isRepeatable: true,
          args: {
            name: "memory-limit",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--tmp-dir",
          isRepeatable: true,
          args: {
            name: "tmp-dir",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-R", "--reverse"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "fetch",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--disk-cache-dir",
          isRepeatable: true,
          args: {
            name: "disk-cache-dir",
            isOptional: true,
          },
        },
        {
          name: ["-H", "--http-header"],
          isRepeatable: true,
          args: {
            name: "http-header",
            isOptional: true,
          },
        },
        {
          name: "--jaq",
          isRepeatable: true,
          args: {
            name: "jaq",
            isOptional: true,
          },
        },
        {
          name: "--jaqfile",
          isRepeatable: true,
          args: {
            name: "jaqfile",
            isOptional: true,
          },
        },
        {
          name: "--max-errors",
          isRepeatable: true,
          args: {
            name: "max-errors",
            isOptional: true,
          },
        },
        {
          name: "--max-retries",
          isRepeatable: true,
          args: {
            name: "max-retries",
            isOptional: true,
          },
        },
        {
          name: "--mem-cache-size",
          isRepeatable: true,
          args: {
            name: "mem-cache-size",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--rate-limit",
          isRepeatable: true,
          args: {
            name: "rate-limit",
            isOptional: true,
          },
        },
        {
          name: "--report",
          isRepeatable: true,
          args: {
            name: "report",
            isOptional: true,
          },
        },
        {
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: "--url-template",
          isRepeatable: true,
          args: {
            name: "url-template",
            isOptional: true,
          },
        },
        {
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
            isOptional: true,
          },
        },
        {
          name: "--cache-error",
        },
        {
          name: "--cookies",
        },
        {
          name: "--disk-cache",
        },
        {
          name: "--flush-cache",
        },
        {
          name: "--no-cache",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--pretty",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--redis-cache",
        },
        {
          name: "--store-error",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "fetchpost",
      options: [
        {
          name: "--content-type",
          isRepeatable: true,
          args: {
            name: "content-type",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--disk-cache-dir",
          isRepeatable: true,
          args: {
            name: "disk-cache-dir",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--globals-json"],
          isRepeatable: true,
          args: {
            name: "globals-json",
            isOptional: true,
          },
        },
        {
          name: ["-H", "--http-header"],
          isRepeatable: true,
          args: {
            name: "http-header",
            isOptional: true,
          },
        },
        {
          name: "--jaq",
          isRepeatable: true,
          args: {
            name: "jaq",
            isOptional: true,
          },
        },
        {
          name: "--jaqfile",
          isRepeatable: true,
          args: {
            name: "jaqfile",
            isOptional: true,
          },
        },
        {
          name: "--max-errors",
          isRepeatable: true,
          args: {
            name: "max-errors",
            isOptional: true,
          },
        },
        {
          name: "--max-retries",
          isRepeatable: true,
          args: {
            name: "max-retries",
            isOptional: true,
          },
        },
        {
          name: "--mem-cache-size",
          isRepeatable: true,
          args: {
            name: "mem-cache-size",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-t", "--payload-tpl"],
          isRepeatable: true,
          args: {
            name: "payload-tpl",
            isOptional: true,
          },
        },
        {
          name: "--rate-limit",
          isRepeatable: true,
          args: {
            name: "rate-limit",
            isOptional: true,
          },
        },
        {
          name: "--report",
          isRepeatable: true,
          args: {
            name: "report",
            isOptional: true,
          },
        },
        {
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
            isOptional: true,
          },
        },
        {
          name: "--cache-error",
        },
        {
          name: "--compress",
        },
        {
          name: "--cookies",
        },
        {
          name: "--disk-cache",
        },
        {
          name: "--flush-cache",
        },
        {
          name: "--no-cache",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--pretty",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--redis-cache",
        },
        {
          name: "--store-error",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "fill",
      options: [
        {
          name: ["-v", "--default"],
          isRepeatable: true,
          args: {
            name: "default",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-g", "--groupby"],
          isRepeatable: true,
          args: {
            name: "groupby",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-b", "--backfill"],
        },
        {
          name: ["-f", "--first"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "fixlengths",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--escape",
          isRepeatable: true,
          args: {
            name: "escape",
            isOptional: true,
          },
        },
        {
          name: ["-i", "--insert"],
          isRepeatable: true,
          args: {
            name: "insert",
            isOptional: true,
          },
        },
        {
          name: ["-l", "--length"],
          isRepeatable: true,
          args: {
            name: "length",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--quote",
          isRepeatable: true,
          args: {
            name: "quote",
            isOptional: true,
          },
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-r", "--remove-empty"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "flatten",
      options: [
        {
          name: ["-c", "--condense"],
          isRepeatable: true,
          args: {
            name: "condense",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-f", "--field-separator"],
          isRepeatable: true,
          args: {
            name: "field-separator",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--separator"],
          isRepeatable: true,
          args: {
            name: "separator",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "fmt",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--escape",
          isRepeatable: true,
          args: {
            name: "escape",
            isOptional: true,
          },
        },
        {
          name: ["-t", "--out-delimiter"],
          isRepeatable: true,
          args: {
            name: "out-delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--quote",
          isRepeatable: true,
          args: {
            name: "quote",
            isOptional: true,
          },
        },
        {
          name: "--ascii",
        },
        {
          name: "--crlf",
        },
        {
          name: "--no-final-newline",
        },
        {
          name: "--quote-always",
        },
        {
          name: "--quote-never",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "foreach",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--dry-run",
          isRepeatable: true,
          args: {
            name: "dry-run",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-u", "--unify"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "frequency",
      options: [
        {
          name: "--all-unique-text",
          isRepeatable: true,
          args: {
            name: "all-unique-text",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--high-card-pct",
          isRepeatable: true,
          args: {
            name: "high-card-pct",
            isOptional: true,
          },
        },
        {
          name: "--high-card-threshold",
          isRepeatable: true,
          args: {
            name: "high-card-threshold",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-l", "--limit"],
          isRepeatable: true,
          args: {
            name: "limit",
            isOptional: true,
          },
        },
        {
          name: "--lmt-threshold",
          isRepeatable: true,
          args: {
            name: "lmt-threshold",
            isOptional: true,
          },
        },
        {
          name: "--no-float",
          isRepeatable: true,
          args: {
            name: "no-float",
            isOptional: true,
          },
        },
        {
          name: "--null-text",
          isRepeatable: true,
          args: {
            name: "null-text",
            isOptional: true,
          },
        },
        {
          name: "--other-text",
          isRepeatable: true,
          args: {
            name: "other-text",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--pct-dec-places",
          isRepeatable: true,
          args: {
            name: "pct-dec-places",
            isOptional: true,
          },
        },
        {
          name: ["-r", "--rank-strategy"],
          isRepeatable: true,
          args: {
            name: "rank-strategy",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--stats-filter",
          isRepeatable: true,
          args: {
            name: "stats-filter",
            isOptional: true,
          },
        },
        {
          name: ["-u", "--unq-limit"],
          isRepeatable: true,
          args: {
            name: "unq-limit",
            isOptional: true,
          },
        },
        {
          name: "--weight",
          isRepeatable: true,
          args: {
            name: "weight",
            isOptional: true,
          },
        },
        {
          name: ["-a", "--asc"],
        },
        {
          name: "--force",
        },
        {
          name: "--frequency-jsonl",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--json",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--no-nulls",
        },
        {
          name: "--no-other",
        },
        {
          name: "--no-stats",
        },
        {
          name: "--no-trim",
        },
        {
          name: "--null-sorted",
        },
        {
          name: "--other-sorted",
        },
        {
          name: "--pct-nulls",
        },
        {
          name: "--pretty-json",
        },
        {
          name: "--toon",
        },
        {
          name: "--vis-whitespace",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "geocode",
      subcommands: [
        {
          name: "countryinfo",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "countryinfonow",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "index-check",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "index-load",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "index-reset",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "index-update",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "iplookup",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "iplookupnow",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "reverse",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "reversenow",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "suggest",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "suggestnow",
          options: [
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
                isOptional: true,
              },
            },
            {
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
                isOptional: true,
              },
            },
            {
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
                isOptional: true,
              },
            },
            {
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
                isOptional: true,
              },
            },
            {
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--force",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "countryinfo",
            },
            {
              name: "countryinfonow",
            },
            {
              name: "index-check",
            },
            {
              name: "index-load",
            },
            {
              name: "index-reset",
            },
            {
              name: "index-update",
            },
            {
              name: "iplookup",
            },
            {
              name: "iplookupnow",
            },
            {
              name: "reverse",
            },
            {
              name: "reversenow",
            },
            {
              name: "suggest",
            },
            {
              name: "suggestnow",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: "--admin1",
          isRepeatable: true,
          args: {
            name: "admin1",
            isOptional: true,
          },
        },
        {
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
            isOptional: true,
          },
        },
        {
          name: "--cache-dir",
          isRepeatable: true,
          args: {
            name: "cache-dir",
            isOptional: true,
          },
        },
        {
          name: "--cities-url",
          isRepeatable: true,
          args: {
            name: "cities-url",
            isOptional: true,
          },
        },
        {
          name: "--country",
          isRepeatable: true,
          args: {
            name: "country",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-f", "--formatstr"],
          isRepeatable: true,
          args: {
            name: "formatstr",
            isOptional: true,
          },
        },
        {
          name: "--invalid-result",
          isRepeatable: true,
          args: {
            name: "invalid-result",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-k", "--k_weight"],
          isRepeatable: true,
          args: {
            name: "k_weight",
            isOptional: true,
          },
        },
        {
          name: ["-l", "--language"],
          isRepeatable: true,
          args: {
            name: "language",
            isOptional: true,
          },
        },
        {
          name: "--languages",
          isRepeatable: true,
          args: {
            name: "languages",
            isOptional: true,
          },
        },
        {
          name: "--min-score",
          isRepeatable: true,
          args: {
            name: "min-score",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-r", "--rename"],
          isRepeatable: true,
          args: {
            name: "rename",
            isOptional: true,
          },
        },
        {
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: "--force",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "geoconvert",
      options: [
        {
          name: ["-g", "--geometry"],
          isRepeatable: true,
          args: {
            name: "geometry",
            isOptional: true,
          },
        },
        {
          name: ["-y", "--latitude"],
          isRepeatable: true,
          args: {
            name: "latitude",
            isOptional: true,
          },
        },
        {
          name: ["-x", "--longitude"],
          isRepeatable: true,
          args: {
            name: "longitude",
            isOptional: true,
          },
        },
        {
          name: ["-l", "--max-length"],
          isRepeatable: true,
          args: {
            name: "max-length",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "headers",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-J", "--just-count"],
        },
        {
          name: ["-j", "--just-names"],
        },
        {
          name: "--trim",
        },
        {
          name: "--union",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "implode",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-k", "--keys"],
          isRepeatable: true,
          args: {
            name: "keys",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-r", "--rename"],
          isRepeatable: true,
          args: {
            name: "rename",
            isOptional: true,
          },
        },
        {
          name: ["-v", "--value"],
          isRepeatable: true,
          args: {
            name: "value",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--skip-empty",
        },
        {
          name: "--sorted",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "index",
      options: [
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "input",
      options: [
        {
          name: "--comment",
          isRepeatable: true,
          args: {
            name: "comment",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--encoding-errors",
          isRepeatable: true,
          args: {
            name: "encoding-errors",
            isOptional: true,
          },
        },
        {
          name: "--escape",
          isRepeatable: true,
          args: {
            name: "escape",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--quote",
          isRepeatable: true,
          args: {
            name: "quote",
            isOptional: true,
          },
        },
        {
          name: "--quote-style",
          isRepeatable: true,
          args: {
            name: "quote-style",
            isOptional: true,
          },
        },
        {
          name: "--skip-lastlines",
          isRepeatable: true,
          args: {
            name: "skip-lastlines",
            isOptional: true,
          },
        },
        {
          name: "--skip-lines",
          isRepeatable: true,
          args: {
            name: "skip-lines",
            isOptional: true,
          },
        },
        {
          name: "--auto-skip",
        },
        {
          name: "--no-quoting",
        },
        {
          name: "--trim-fields",
        },
        {
          name: "--trim-headers",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "join",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--keys-output",
          isRepeatable: true,
          args: {
            name: "keys-output",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--cross",
        },
        {
          name: "--full",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-z", "--ignore-leading-zeros"],
        },
        {
          name: "--left",
        },
        {
          name: "--left-anti",
        },
        {
          name: "--left-semi",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--nulls",
        },
        {
          name: "--right",
        },
        {
          name: "--right-anti",
        },
        {
          name: "--right-semi",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "joinp",
      options: [
        {
          name: "--cache-schema",
          isRepeatable: true,
          args: {
            name: "cache-schema",
            isOptional: true,
          },
        },
        {
          name: "--date-format",
          isRepeatable: true,
          args: {
            name: "date-format",
            isOptional: true,
          },
        },
        {
          name: "--datetime-format",
          isRepeatable: true,
          args: {
            name: "datetime-format",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--filter-left",
          isRepeatable: true,
          args: {
            name: "filter-left",
            isOptional: true,
          },
        },
        {
          name: "--filter-right",
          isRepeatable: true,
          args: {
            name: "filter-right",
            isOptional: true,
          },
        },
        {
          name: "--float-precision",
          isRepeatable: true,
          args: {
            name: "float-precision",
            isOptional: true,
          },
        },
        {
          name: "--infer-len",
          isRepeatable: true,
          args: {
            name: "infer-len",
            isOptional: true,
          },
        },
        {
          name: "--left_by",
          isRepeatable: true,
          args: {
            name: "left_by",
            isOptional: true,
          },
        },
        {
          name: "--maintain-order",
          isRepeatable: true,
          args: {
            name: "maintain-order",
            isOptional: true,
          },
        },
        {
          name: "--non-equi",
          isRepeatable: true,
          args: {
            name: "non-equi",
            isOptional: true,
          },
        },
        {
          name: ["-N", "--norm-unicode"],
          isRepeatable: true,
          args: {
            name: "norm-unicode",
            isOptional: true,
          },
        },
        {
          name: "--null-value",
          isRepeatable: true,
          args: {
            name: "null-value",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--right_by",
          isRepeatable: true,
          args: {
            name: "right_by",
            isOptional: true,
          },
        },
        {
          name: "--sql-filter",
          isRepeatable: true,
          args: {
            name: "sql-filter",
            isOptional: true,
          },
        },
        {
          name: "--strategy",
          isRepeatable: true,
          args: {
            name: "strategy",
            isOptional: true,
          },
        },
        {
          name: "--time-format",
          isRepeatable: true,
          args: {
            name: "time-format",
            isOptional: true,
          },
        },
        {
          name: "--tolerance",
          isRepeatable: true,
          args: {
            name: "tolerance",
            isOptional: true,
          },
        },
        {
          name: "--validate",
          isRepeatable: true,
          args: {
            name: "validate",
            isOptional: true,
          },
        },
        {
          name: ["-X", "--allow-exact-matches"],
        },
        {
          name: "--asof",
        },
        {
          name: "--coalesce",
        },
        {
          name: "--cross",
        },
        {
          name: "--decimal-comma",
        },
        {
          name: "--full",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--ignore-errors",
        },
        {
          name: ["-z", "--ignore-leading-zeros"],
        },
        {
          name: "--left",
        },
        {
          name: "--left-anti",
        },
        {
          name: "--left-semi",
        },
        {
          name: "--low-memory",
        },
        {
          name: "--no-optimizations",
        },
        {
          name: "--no-sort",
        },
        {
          name: "--nulls",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--right",
        },
        {
          name: "--right-anti",
        },
        {
          name: "--right-semi",
        },
        {
          name: "--streaming",
        },
        {
          name: "--try-parsedates",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "json",
      options: [
        {
          name: "--jaq",
          isRepeatable: true,
          args: {
            name: "jaq",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "jsonl",
      options: [
        {
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--ignore-errors",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "lens",
      options: [
        {
          name: "--columns",
          isRepeatable: true,
          args: {
            name: "columns",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--echo-column",
          isRepeatable: true,
          args: {
            name: "echo-column",
            isOptional: true,
          },
        },
        {
          name: "--filter",
          isRepeatable: true,
          args: {
            name: "filter",
            isOptional: true,
          },
        },
        {
          name: "--find",
          isRepeatable: true,
          args: {
            name: "find",
            isOptional: true,
          },
        },
        {
          name: ["-f", "--freeze-columns"],
          isRepeatable: true,
          args: {
            name: "freeze-columns",
            isOptional: true,
          },
        },
        {
          name: ["-P", "--prompt"],
          isRepeatable: true,
          args: {
            name: "prompt",
            isOptional: true,
          },
        },
        {
          name: ["-W", "--wrap-mode"],
          isRepeatable: true,
          args: {
            name: "wrap-mode",
            isOptional: true,
          },
        },
        {
          name: ["-A", "--auto-reload"],
        },
        {
          name: "--debug",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-m", "--monochrome"],
        },
        {
          name: "--no-headers",
        },
        {
          name: ["-S", "--streaming-stdin"],
        },
        {
          name: ["-t", "--tab-separated"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "log",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "luau",
      subcommands: [
        {
          name: "filter",
          options: [
            {
              name: ["-B", "--begin"],
              isRepeatable: true,
              args: {
                name: "begin",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--ckan-api",
              isRepeatable: true,
              args: {
                name: "ckan-api",
                isOptional: true,
              },
            },
            {
              name: "--ckan-token",
              isRepeatable: true,
              args: {
                name: "ckan-token",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-E", "--end"],
              isRepeatable: true,
              args: {
                name: "end",
                isOptional: true,
              },
            },
            {
              name: "--max-errors",
              isRepeatable: true,
              args: {
                name: "max-errors",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--colindex",
            },
            {
              name: ["-g", "--no-globals"],
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-r", "--remap"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "map",
          options: [
            {
              name: ["-B", "--begin"],
              isRepeatable: true,
              args: {
                name: "begin",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--ckan-api",
              isRepeatable: true,
              args: {
                name: "ckan-api",
                isOptional: true,
              },
            },
            {
              name: "--ckan-token",
              isRepeatable: true,
              args: {
                name: "ckan-token",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-E", "--end"],
              isRepeatable: true,
              args: {
                name: "end",
                isOptional: true,
              },
            },
            {
              name: "--max-errors",
              isRepeatable: true,
              args: {
                name: "max-errors",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--colindex",
            },
            {
              name: ["-g", "--no-globals"],
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-r", "--remap"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "filter",
            },
            {
              name: "map",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: ["-B", "--begin"],
          isRepeatable: true,
          args: {
            name: "begin",
            isOptional: true,
          },
        },
        {
          name: "--cache-dir",
          isRepeatable: true,
          args: {
            name: "cache-dir",
            isOptional: true,
          },
        },
        {
          name: "--ckan-api",
          isRepeatable: true,
          args: {
            name: "ckan-api",
            isOptional: true,
          },
        },
        {
          name: "--ckan-token",
          isRepeatable: true,
          args: {
            name: "ckan-token",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-E", "--end"],
          isRepeatable: true,
          args: {
            name: "end",
            isOptional: true,
          },
        },
        {
          name: "--max-errors",
          isRepeatable: true,
          args: {
            name: "max-errors",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: "--colindex",
        },
        {
          name: ["-g", "--no-globals"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-r", "--remap"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "moarstats",
      options: [
        {
          name: ["-S", "--bivariate-stats"],
          isRepeatable: true,
          args: {
            name: "bivariate-stats",
            isOptional: true,
          },
        },
        {
          name: ["-C", "--cardinality-threshold"],
          isRepeatable: true,
          args: {
            name: "cardinality-threshold",
            isOptional: true,
          },
        },
        {
          name: ["-e", "--epsilon"],
          isRepeatable: true,
          args: {
            name: "epsilon",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-J", "--join-inputs"],
          isRepeatable: true,
          args: {
            name: "join-inputs",
            isOptional: true,
          },
        },
        {
          name: ["-K", "--join-keys"],
          isRepeatable: true,
          args: {
            name: "join-keys",
            isOptional: true,
          },
        },
        {
          name: ["-T", "--join-type"],
          isRepeatable: true,
          args: {
            name: "join-type",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--pct-thresholds",
          isRepeatable: true,
          args: {
            name: "pct-thresholds",
            isOptional: true,
          },
        },
        {
          name: "--round",
          isRepeatable: true,
          args: {
            name: "round",
            isOptional: true,
          },
        },
        {
          name: "--stats-options",
          isRepeatable: true,
          args: {
            name: "stats-options",
            isOptional: true,
          },
        },
        {
          name: "--xsd-gdate-scan",
          isRepeatable: true,
          args: {
            name: "xsd-gdate-scan",
            isOptional: true,
          },
        },
        {
          name: "--advanced",
        },
        {
          name: ["-B", "--bivariate"],
        },
        {
          name: "--force",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--use-percentiles",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "partition",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--filename",
          isRepeatable: true,
          args: {
            name: "filename",
            isOptional: true,
          },
        },
        {
          name: "--limit",
          isRepeatable: true,
          args: {
            name: "limit",
            isOptional: true,
          },
        },
        {
          name: ["-p", "--prefix-length"],
          isRepeatable: true,
          args: {
            name: "prefix-length",
            isOptional: true,
          },
        },
        {
          name: "--drop",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "pivotp",
      options: [
        {
          name: ["-a", "--agg"],
          isRepeatable: true,
          args: {
            name: "agg",
            isOptional: true,
          },
        },
        {
          name: "--col-separator",
          isRepeatable: true,
          args: {
            name: "col-separator",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-i", "--index"],
          isRepeatable: true,
          args: {
            name: "index",
            isOptional: true,
          },
        },
        {
          name: "--infer-len",
          isRepeatable: true,
          args: {
            name: "infer-len",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--total-label",
          isRepeatable: true,
          args: {
            name: "total-label",
            isOptional: true,
          },
        },
        {
          name: ["-v", "--values"],
          isRepeatable: true,
          args: {
            name: "values",
            isOptional: true,
          },
        },
        {
          name: "--decimal-comma",
        },
        {
          name: "--grand-total",
        },
        {
          name: "--ignore-errors",
        },
        {
          name: "--maintain-order",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--sort-columns",
        },
        {
          name: "--subtotal",
        },
        {
          name: "--try-parsedates",
        },
        {
          name: "--validate",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "pragmastat",
      options: [
        {
          name: "--compare1",
          isRepeatable: true,
          args: {
            name: "compare1",
            isOptional: true,
          },
        },
        {
          name: "--compare2",
          isRepeatable: true,
          args: {
            name: "compare2",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-m", "--misrate"],
          isRepeatable: true,
          args: {
            name: "misrate",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--round",
          isRepeatable: true,
          args: {
            name: "round",
            isOptional: true,
          },
        },
        {
          name: "--seed",
          isRepeatable: true,
          args: {
            name: "seed",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--stats-options",
          isRepeatable: true,
          args: {
            name: "stats-options",
            isOptional: true,
          },
        },
        {
          name: "--subsample",
          isRepeatable: true,
          args: {
            name: "subsample",
            isOptional: true,
          },
        },
        {
          name: "--force",
        },
        {
          name: "--memcheck",
        },
        {
          name: "--no-bounds",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--standalone",
        },
        {
          name: ["-t", "--twosample"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "pro",
      subcommands: [
        {
          name: "lens",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "workflow",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "lens",
            },
            {
              name: "workflow",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "prompt",
      options: [
        {
          name: "--base-delay-ms",
          isRepeatable: true,
          args: {
            name: "base-delay-ms",
            isOptional: true,
          },
        },
        {
          name: ["-F", "--filters"],
          isRepeatable: true,
          args: {
            name: "filters",
            isOptional: true,
          },
        },
        {
          name: ["-m", "--msg"],
          isRepeatable: true,
          args: {
            name: "msg",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--save-fname",
          isRepeatable: true,
          args: {
            name: "save-fname",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--workdir"],
          isRepeatable: true,
          args: {
            name: "workdir",
            isOptional: true,
          },
        },
        {
          name: ["-f", "--fd-output"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "pseudo",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--formatstr",
          isRepeatable: true,
          args: {
            name: "formatstr",
            isOptional: true,
          },
        },
        {
          name: "--increment",
          isRepeatable: true,
          args: {
            name: "increment",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--start",
          isRepeatable: true,
          args: {
            name: "start",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "py",
      subcommands: [
        {
          name: "filter",
          options: [
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--helper"],
              isRepeatable: true,
              args: {
                name: "helper",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "map",
          options: [
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-f", "--helper"],
              isRepeatable: true,
              args: {
                name: "helper",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "filter",
            },
            {
              name: "map",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-f", "--helper"],
          isRepeatable: true,
          args: {
            name: "helper",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "rename",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--pairwise",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "replace",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--dfa-size-limit",
          isRepeatable: true,
          args: {
            name: "dfa-size-limit",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--size-limit",
          isRepeatable: true,
          args: {
            name: "size-limit",
            isOptional: true,
          },
        },
        {
          name: "--exact",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--literal",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--not-one",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-u", "--unicode"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "reverse",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "safenames",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--mode",
          isRepeatable: true,
          args: {
            name: "mode",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--prefix",
          isRepeatable: true,
          args: {
            name: "prefix",
            isOptional: true,
          },
        },
        {
          name: "--reserved",
          isRepeatable: true,
          args: {
            name: "reserved",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sample",
      options: [
        {
          name: "--cluster",
          isRepeatable: true,
          args: {
            name: "cluster",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--max-size",
          isRepeatable: true,
          args: {
            name: "max-size",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--rng",
          isRepeatable: true,
          args: {
            name: "rng",
            isOptional: true,
          },
        },
        {
          name: "--seed",
          isRepeatable: true,
          args: {
            name: "seed",
            isOptional: true,
          },
        },
        {
          name: "--stratified",
          isRepeatable: true,
          args: {
            name: "stratified",
            isOptional: true,
          },
        },
        {
          name: "--systematic",
          isRepeatable: true,
          args: {
            name: "systematic",
            isOptional: true,
          },
        },
        {
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: "--timeseries",
          isRepeatable: true,
          args: {
            name: "timeseries",
            isOptional: true,
          },
        },
        {
          name: "--ts-adaptive",
          isRepeatable: true,
          args: {
            name: "ts-adaptive",
            isOptional: true,
          },
        },
        {
          name: "--ts-aggregate",
          isRepeatable: true,
          args: {
            name: "ts-aggregate",
            isOptional: true,
          },
        },
        {
          name: "--ts-input-tz",
          isRepeatable: true,
          args: {
            name: "ts-input-tz",
            isOptional: true,
          },
        },
        {
          name: "--ts-interval",
          isRepeatable: true,
          args: {
            name: "ts-interval",
            isOptional: true,
          },
        },
        {
          name: "--ts-start",
          isRepeatable: true,
          args: {
            name: "ts-start",
            isOptional: true,
          },
        },
        {
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
            isOptional: true,
          },
        },
        {
          name: "--weighted",
          isRepeatable: true,
          args: {
            name: "weighted",
            isOptional: true,
          },
        },
        {
          name: "--bernoulli",
        },
        {
          name: "--force",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--ts-prefer-dmy",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "schema",
      options: [
        {
          name: "--dates-whitelist",
          isRepeatable: true,
          args: {
            name: "dates-whitelist",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--enum-threshold",
          isRepeatable: true,
          args: {
            name: "enum-threshold",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--pattern-columns",
          isRepeatable: true,
          args: {
            name: "pattern-columns",
            isOptional: true,
          },
        },
        {
          name: "--force",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--polars",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--stdout",
        },
        {
          name: "--strict-dates",
        },
        {
          name: "--strict-formats",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "scoresql",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--infer-len",
          isRepeatable: true,
          args: {
            name: "infer-len",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--duckdb",
        },
        {
          name: "--ignore-errors",
        },
        {
          name: "--json",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--truncate-ragged-lines",
        },
        {
          name: "--try-parsedates",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "search",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--dfa-size-limit",
          isRepeatable: true,
          args: {
            name: "dfa-size-limit",
            isOptional: true,
          },
        },
        {
          name: ["-f", "--flag"],
          isRepeatable: true,
          args: {
            name: "flag",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--preview-match",
          isRepeatable: true,
          args: {
            name: "preview-match",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--size-limit",
          isRepeatable: true,
          args: {
            name: "size-limit",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--count"],
        },
        {
          name: "--exact",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-v", "--invert-match"],
        },
        {
          name: "--json",
        },
        {
          name: "--literal",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--not-one",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-Q", "--quick"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-u", "--unicode"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "searchset",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--dfa-size-limit",
          isRepeatable: true,
          args: {
            name: "dfa-size-limit",
            isOptional: true,
          },
        },
        {
          name: ["-f", "--flag"],
          isRepeatable: true,
          args: {
            name: "flag",
            isOptional: true,
          },
        },
        {
          name: "--jobs",
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--size-limit",
          isRepeatable: true,
          args: {
            name: "size-limit",
            isOptional: true,
          },
        },
        {
          name: "--unmatched-output",
          isRepeatable: true,
          args: {
            name: "unmatched-output",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--count"],
        },
        {
          name: "--exact",
        },
        {
          name: "--flag-matches-only",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-v", "--invert-match"],
        },
        {
          name: ["-j", "--json"],
        },
        {
          name: "--literal",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--not-one",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-Q", "--quick"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-u", "--unicode"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "select",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--seed",
          isRepeatable: true,
          args: {
            name: "seed",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-R", "--random"],
        },
        {
          name: ["-S", "--sort"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "slice",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-e", "--end"],
          isRepeatable: true,
          args: {
            name: "end",
            isOptional: true,
          },
        },
        {
          name: ["-i", "--index"],
          isRepeatable: true,
          args: {
            name: "index",
            isOptional: true,
          },
        },
        {
          name: ["-l", "--len"],
          isRepeatable: true,
          args: {
            name: "len",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--start"],
          isRepeatable: true,
          args: {
            name: "start",
            isOptional: true,
          },
        },
        {
          name: "--invert",
        },
        {
          name: "--json",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "snappy",
      subcommands: [
        {
          name: "check",
          options: [
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--user-agent",
              isRepeatable: true,
              args: {
                name: "user-agent",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "compress",
          options: [
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--user-agent",
              isRepeatable: true,
              args: {
                name: "user-agent",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "decompress",
          options: [
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--user-agent",
              isRepeatable: true,
              args: {
                name: "user-agent",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "validate",
          options: [
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--user-agent",
              isRepeatable: true,
              args: {
                name: "user-agent",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "check",
            },
            {
              name: "compress",
            },
            {
              name: "decompress",
            },
            {
              name: "validate",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
            isOptional: true,
          },
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sniff",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--quote",
          isRepeatable: true,
          args: {
            name: "quote",
            isOptional: true,
          },
        },
        {
          name: "--sample",
          isRepeatable: true,
          args: {
            name: "sample",
            isOptional: true,
          },
        },
        {
          name: "--save-urlsample",
          isRepeatable: true,
          args: {
            name: "save-urlsample",
            isOptional: true,
          },
        },
        {
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
            isOptional: true,
          },
        },
        {
          name: "--harvest-mode",
        },
        {
          name: "--json",
        },
        {
          name: "--just-mime",
        },
        {
          name: "--no-infer",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--pretty-json",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-Q", "--quick"],
        },
        {
          name: "--stats-types",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sort",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--rng",
          isRepeatable: true,
          args: {
            name: "rng",
            isOptional: true,
          },
        },
        {
          name: "--seed",
          isRepeatable: true,
          args: {
            name: "seed",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--faster",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--memcheck",
        },
        {
          name: "--natural",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-N", "--numeric"],
        },
        {
          name: "--random",
        },
        {
          name: ["-R", "--reverse"],
        },
        {
          name: ["-u", "--unique"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sortcheck",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--all",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--json",
        },
        {
          name: "--natural",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-N", "--numeric"],
        },
        {
          name: "--pretty-json",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "split",
      options: [
        {
          name: ["-c", "--chunks"],
          isRepeatable: true,
          args: {
            name: "chunks",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--filename",
          isRepeatable: true,
          args: {
            name: "filename",
            isOptional: true,
          },
        },
        {
          name: "--filter",
          isRepeatable: true,
          args: {
            name: "filter",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-k", "--kb-size"],
          isRepeatable: true,
          args: {
            name: "kb-size",
            isOptional: true,
          },
        },
        {
          name: "--pad",
          isRepeatable: true,
          args: {
            name: "pad",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--size"],
          isRepeatable: true,
          args: {
            name: "size",
            isOptional: true,
          },
        },
        {
          name: "--filter-cleanup",
        },
        {
          name: "--filter-ignore-errors",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sqlp",
      options: [
        {
          name: "--compress-level",
          isRepeatable: true,
          args: {
            name: "compress-level",
            isOptional: true,
          },
        },
        {
          name: "--compression",
          isRepeatable: true,
          args: {
            name: "compression",
            isOptional: true,
          },
        },
        {
          name: "--date-format",
          isRepeatable: true,
          args: {
            name: "date-format",
            isOptional: true,
          },
        },
        {
          name: "--datetime-format",
          isRepeatable: true,
          args: {
            name: "datetime-format",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--float-precision",
          isRepeatable: true,
          args: {
            name: "float-precision",
            isOptional: true,
          },
        },
        {
          name: "--format",
          isRepeatable: true,
          args: {
            name: "format",
            isOptional: true,
          },
        },
        {
          name: "--infer-len",
          isRepeatable: true,
          args: {
            name: "infer-len",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--rnull-values",
          isRepeatable: true,
          args: {
            name: "rnull-values",
            isOptional: true,
          },
        },
        {
          name: "--time-format",
          isRepeatable: true,
          args: {
            name: "time-format",
            isOptional: true,
          },
        },
        {
          name: "--wnull-value",
          isRepeatable: true,
          args: {
            name: "wnull-value",
            isOptional: true,
          },
        },
        {
          name: "--cache-schema",
        },
        {
          name: "--decimal-comma",
        },
        {
          name: "--ignore-errors",
        },
        {
          name: "--low-memory",
        },
        {
          name: "--no-optimizations",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--statistics",
        },
        {
          name: "--streaming",
        },
        {
          name: "--truncate-ragged-lines",
        },
        {
          name: "--try-parsedates",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "stats",
      options: [
        {
          name: "--boolean-patterns",
          isRepeatable: true,
          args: {
            name: "boolean-patterns",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--cache-threshold"],
          isRepeatable: true,
          args: {
            name: "cache-threshold",
            isOptional: true,
          },
        },
        {
          name: "--dates-whitelist",
          isRepeatable: true,
          args: {
            name: "dates-whitelist",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--percentile-list",
          isRepeatable: true,
          args: {
            name: "percentile-list",
            isOptional: true,
          },
        },
        {
          name: "--round",
          isRepeatable: true,
          args: {
            name: "round",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--weight",
          isRepeatable: true,
          args: {
            name: "weight",
            isOptional: true,
          },
        },
        {
          name: "--cardinality",
        },
        {
          name: ["-E", "--everything"],
        },
        {
          name: "--force",
        },
        {
          name: "--infer-boolean",
        },
        {
          name: "--infer-dates",
        },
        {
          name: "--mad",
        },
        {
          name: "--median",
        },
        {
          name: "--memcheck",
        },
        {
          name: "--mode",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--nulls",
        },
        {
          name: "--percentiles",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--quartiles",
        },
        {
          name: "--stats-jsonl",
        },
        {
          name: "--typesonly",
        },
        {
          name: "--vis-whitespace",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "table",
      options: [
        {
          name: ["-a", "--align"],
          isRepeatable: true,
          args: {
            name: "align",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--condense"],
          isRepeatable: true,
          args: {
            name: "condense",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-p", "--pad"],
          isRepeatable: true,
          args: {
            name: "pad",
            isOptional: true,
          },
        },
        {
          name: ["-w", "--width"],
          isRepeatable: true,
          args: {
            name: "width",
            isOptional: true,
          },
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "template",
      options: [
        {
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
            isOptional: true,
          },
        },
        {
          name: "--cache-dir",
          isRepeatable: true,
          args: {
            name: "cache-dir",
            isOptional: true,
          },
        },
        {
          name: "--ckan-api",
          isRepeatable: true,
          args: {
            name: "ckan-api",
            isOptional: true,
          },
        },
        {
          name: "--ckan-token",
          isRepeatable: true,
          args: {
            name: "ckan-token",
            isOptional: true,
          },
        },
        {
          name: "--customfilter-error",
          isRepeatable: true,
          args: {
            name: "customfilter-error",
            isOptional: true,
          },
        },
        {
          name: "--delimiter",
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-J", "--globals-json"],
          isRepeatable: true,
          args: {
            name: "globals-json",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: "--outfilename",
          isRepeatable: true,
          args: {
            name: "outfilename",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--outsubdir-size",
          isRepeatable: true,
          args: {
            name: "outsubdir-size",
            isOptional: true,
          },
        },
        {
          name: "--template",
          isRepeatable: true,
          args: {
            name: "template",
            isOptional: true,
          },
        },
        {
          name: ["-t", "--template-file"],
          isRepeatable: true,
          args: {
            name: "template-file",
            isOptional: true,
          },
        },
        {
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "to",
      subcommands: [
        {
          name: "datapackage",
          options: [
            {
              name: "--compress-level",
              isRepeatable: true,
              args: {
                name: "compress-level",
                isOptional: true,
              },
            },
            {
              name: "--compression",
              isRepeatable: true,
              args: {
                name: "compression",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: "--infer-len",
              isRepeatable: true,
              args: {
                name: "infer-len",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-s", "--schema"],
              isRepeatable: true,
              args: {
                name: "schema",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--separator"],
              isRepeatable: true,
              args: {
                name: "separator",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--stats-csv"],
              isRepeatable: true,
              args: {
                name: "stats-csv",
                isOptional: true,
              },
            },
            {
              name: ["-t", "--table"],
              isRepeatable: true,
              args: {
                name: "table",
                isOptional: true,
              },
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: "--try-parse-dates",
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "ods",
          options: [
            {
              name: "--compress-level",
              isRepeatable: true,
              args: {
                name: "compress-level",
                isOptional: true,
              },
            },
            {
              name: "--compression",
              isRepeatable: true,
              args: {
                name: "compression",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: "--infer-len",
              isRepeatable: true,
              args: {
                name: "infer-len",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-s", "--schema"],
              isRepeatable: true,
              args: {
                name: "schema",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--separator"],
              isRepeatable: true,
              args: {
                name: "separator",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--stats-csv"],
              isRepeatable: true,
              args: {
                name: "stats-csv",
                isOptional: true,
              },
            },
            {
              name: ["-t", "--table"],
              isRepeatable: true,
              args: {
                name: "table",
                isOptional: true,
              },
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: "--try-parse-dates",
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "parquet",
          options: [
            {
              name: "--compress-level",
              isRepeatable: true,
              args: {
                name: "compress-level",
                isOptional: true,
              },
            },
            {
              name: "--compression",
              isRepeatable: true,
              args: {
                name: "compression",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: "--infer-len",
              isRepeatable: true,
              args: {
                name: "infer-len",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-s", "--schema"],
              isRepeatable: true,
              args: {
                name: "schema",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--separator"],
              isRepeatable: true,
              args: {
                name: "separator",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--stats-csv"],
              isRepeatable: true,
              args: {
                name: "stats-csv",
                isOptional: true,
              },
            },
            {
              name: ["-t", "--table"],
              isRepeatable: true,
              args: {
                name: "table",
                isOptional: true,
              },
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: "--try-parse-dates",
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "postgres",
          options: [
            {
              name: "--compress-level",
              isRepeatable: true,
              args: {
                name: "compress-level",
                isOptional: true,
              },
            },
            {
              name: "--compression",
              isRepeatable: true,
              args: {
                name: "compression",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: "--infer-len",
              isRepeatable: true,
              args: {
                name: "infer-len",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-s", "--schema"],
              isRepeatable: true,
              args: {
                name: "schema",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--separator"],
              isRepeatable: true,
              args: {
                name: "separator",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--stats-csv"],
              isRepeatable: true,
              args: {
                name: "stats-csv",
                isOptional: true,
              },
            },
            {
              name: ["-t", "--table"],
              isRepeatable: true,
              args: {
                name: "table",
                isOptional: true,
              },
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: "--try-parse-dates",
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "sqlite",
          options: [
            {
              name: "--compress-level",
              isRepeatable: true,
              args: {
                name: "compress-level",
                isOptional: true,
              },
            },
            {
              name: "--compression",
              isRepeatable: true,
              args: {
                name: "compression",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: "--infer-len",
              isRepeatable: true,
              args: {
                name: "infer-len",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-s", "--schema"],
              isRepeatable: true,
              args: {
                name: "schema",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--separator"],
              isRepeatable: true,
              args: {
                name: "separator",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--stats-csv"],
              isRepeatable: true,
              args: {
                name: "stats-csv",
                isOptional: true,
              },
            },
            {
              name: ["-t", "--table"],
              isRepeatable: true,
              args: {
                name: "table",
                isOptional: true,
              },
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: "--try-parse-dates",
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "xlsx",
          options: [
            {
              name: "--compress-level",
              isRepeatable: true,
              args: {
                name: "compress-level",
                isOptional: true,
              },
            },
            {
              name: "--compression",
              isRepeatable: true,
              args: {
                name: "compression",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: "--infer-len",
              isRepeatable: true,
              args: {
                name: "infer-len",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: ["-s", "--schema"],
              isRepeatable: true,
              args: {
                name: "schema",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--separator"],
              isRepeatable: true,
              args: {
                name: "separator",
                isOptional: true,
              },
            },
            {
              name: ["-c", "--stats-csv"],
              isRepeatable: true,
              args: {
                name: "stats-csv",
                isOptional: true,
              },
            },
            {
              name: ["-t", "--table"],
              isRepeatable: true,
              args: {
                name: "table",
                isOptional: true,
              },
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: "--try-parse-dates",
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "datapackage",
            },
            {
              name: "ods",
            },
            {
              name: "parquet",
            },
            {
              name: "postgres",
            },
            {
              name: "sqlite",
            },
            {
              name: "xlsx",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: "--compress-level",
          isRepeatable: true,
          args: {
            name: "compress-level",
            isOptional: true,
          },
        },
        {
          name: "--compression",
          isRepeatable: true,
          args: {
            name: "compression",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--infer-len",
          isRepeatable: true,
          args: {
            name: "infer-len",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--schema"],
          isRepeatable: true,
          args: {
            name: "schema",
            isOptional: true,
          },
        },
        {
          name: ["-p", "--separator"],
          isRepeatable: true,
          args: {
            name: "separator",
            isOptional: true,
          },
        },
        {
          name: ["-c", "--stats-csv"],
          isRepeatable: true,
          args: {
            name: "stats-csv",
            isOptional: true,
          },
        },
        {
          name: ["-t", "--table"],
          isRepeatable: true,
          args: {
            name: "table",
            isOptional: true,
          },
        },
        {
          name: ["-A", "--all-strings"],
        },
        {
          name: "--drop",
        },
        {
          name: ["-u", "--dump"],
        },
        {
          name: ["-e", "--evolve"],
        },
        {
          name: ["-i", "--pipe"],
        },
        {
          name: ["-k", "--print-package"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-a", "--stats"],
        },
        {
          name: "--try-parse-dates",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "tojsonl",
      options: [
        {
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--memcheck",
        },
        {
          name: "--no-boolean",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--trim",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "transpose",
      options: [
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--long",
          isRepeatable: true,
          args: {
            name: "long",
            isOptional: true,
          },
        },
        {
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
            isOptional: true,
          },
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-m", "--multipass"],
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "validate",
      subcommands: [
        {
          name: "schema",
          options: [
            {
              name: "--backtrack-limit",
              isRepeatable: true,
              args: {
                name: "backtrack-limit",
                isOptional: true,
              },
            },
            {
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
                isOptional: true,
              },
            },
            {
              name: "--ckan-api",
              isRepeatable: true,
              args: {
                name: "ckan-api",
                isOptional: true,
              },
            },
            {
              name: "--ckan-token",
              isRepeatable: true,
              args: {
                name: "ckan-token",
                isOptional: true,
              },
            },
            {
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: "--dfa-size-limit",
              isRepeatable: true,
              args: {
                name: "dfa-size-limit",
                isOptional: true,
              },
            },
            {
              name: "--email-min-subdomains",
              isRepeatable: true,
              args: {
                name: "email-min-subdomains",
                isOptional: true,
              },
            },
            {
              name: "--invalid",
              isRepeatable: true,
              args: {
                name: "invalid",
                isOptional: true,
              },
            },
            {
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
                isOptional: true,
              },
            },
            {
              name: "--size-limit",
              isRepeatable: true,
              args: {
                name: "size-limit",
                isOptional: true,
              },
            },
            {
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
                isOptional: true,
              },
            },
            {
              name: "--valid",
              isRepeatable: true,
              args: {
                name: "valid",
                isOptional: true,
              },
            },
            {
              name: "--valid-output",
              isRepeatable: true,
              args: {
                name: "valid-output",
                isOptional: true,
              },
            },
            {
              name: "--email-display-text",
            },
            {
              name: "--email-domain-literal",
            },
            {
              name: "--email-required-tld",
            },
            {
              name: "--fail-fast",
            },
            {
              name: "--fancy-regex",
            },
            {
              name: "--json",
            },
            {
              name: "--no-format-validation",
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: "--pretty-json",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--trim",
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "schema",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: "--backtrack-limit",
          isRepeatable: true,
          args: {
            name: "backtrack-limit",
            isOptional: true,
          },
        },
        {
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
            isOptional: true,
          },
        },
        {
          name: "--cache-dir",
          isRepeatable: true,
          args: {
            name: "cache-dir",
            isOptional: true,
          },
        },
        {
          name: "--ckan-api",
          isRepeatable: true,
          args: {
            name: "ckan-api",
            isOptional: true,
          },
        },
        {
          name: "--ckan-token",
          isRepeatable: true,
          args: {
            name: "ckan-token",
            isOptional: true,
          },
        },
        {
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--dfa-size-limit",
          isRepeatable: true,
          args: {
            name: "dfa-size-limit",
            isOptional: true,
          },
        },
        {
          name: "--email-min-subdomains",
          isRepeatable: true,
          args: {
            name: "email-min-subdomains",
            isOptional: true,
          },
        },
        {
          name: "--invalid",
          isRepeatable: true,
          args: {
            name: "invalid",
            isOptional: true,
          },
        },
        {
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: "--size-limit",
          isRepeatable: true,
          args: {
            name: "size-limit",
            isOptional: true,
          },
        },
        {
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: "--valid",
          isRepeatable: true,
          args: {
            name: "valid",
            isOptional: true,
          },
        },
        {
          name: "--valid-output",
          isRepeatable: true,
          args: {
            name: "valid-output",
            isOptional: true,
          },
        },
        {
          name: "--email-display-text",
        },
        {
          name: "--email-domain-literal",
        },
        {
          name: "--email-required-tld",
        },
        {
          name: "--fail-fast",
        },
        {
          name: "--fancy-regex",
        },
        {
          name: "--json",
        },
        {
          name: "--no-format-validation",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--pretty-json",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--trim",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      subcommands: [
        {
          name: "apply",
          subcommands: [
            {
              name: "calcconv",
            },
            {
              name: "dynfmt",
            },
            {
              name: "emptyreplace",
            },
            {
              name: "operations",
            },
          ],
        },
        {
          name: "behead",
        },
        {
          name: "blake3",
        },
        {
          name: "cat",
          subcommands: [
            {
              name: "columns",
            },
            {
              name: "rows",
            },
            {
              name: "rowskey",
            },
          ],
        },
        {
          name: "clipboard",
        },
        {
          name: "color",
        },
        {
          name: "count",
        },
        {
          name: "datefmt",
        },
        {
          name: "dedup",
        },
        {
          name: "describegpt",
        },
        {
          name: "diff",
        },
        {
          name: "edit",
        },
        {
          name: "enum",
        },
        {
          name: "excel",
        },
        {
          name: "exclude",
        },
        {
          name: "explode",
        },
        {
          name: "extdedup",
        },
        {
          name: "extsort",
        },
        {
          name: "fetch",
        },
        {
          name: "fetchpost",
        },
        {
          name: "fill",
        },
        {
          name: "fixlengths",
        },
        {
          name: "flatten",
        },
        {
          name: "fmt",
        },
        {
          name: "foreach",
        },
        {
          name: "frequency",
        },
        {
          name: "geocode",
          subcommands: [
            {
              name: "countryinfo",
            },
            {
              name: "countryinfonow",
            },
            {
              name: "index-check",
            },
            {
              name: "index-load",
            },
            {
              name: "index-reset",
            },
            {
              name: "index-update",
            },
            {
              name: "iplookup",
            },
            {
              name: "iplookupnow",
            },
            {
              name: "reverse",
            },
            {
              name: "reversenow",
            },
            {
              name: "suggest",
            },
            {
              name: "suggestnow",
            },
          ],
        },
        {
          name: "geoconvert",
        },
        {
          name: "headers",
        },
        {
          name: "implode",
        },
        {
          name: "index",
        },
        {
          name: "input",
        },
        {
          name: "join",
        },
        {
          name: "joinp",
        },
        {
          name: "json",
        },
        {
          name: "jsonl",
        },
        {
          name: "lens",
        },
        {
          name: "log",
        },
        {
          name: "luau",
          subcommands: [
            {
              name: "filter",
            },
            {
              name: "map",
            },
          ],
        },
        {
          name: "moarstats",
        },
        {
          name: "partition",
        },
        {
          name: "pivotp",
        },
        {
          name: "pragmastat",
        },
        {
          name: "pro",
          subcommands: [
            {
              name: "lens",
            },
            {
              name: "workflow",
            },
          ],
        },
        {
          name: "prompt",
        },
        {
          name: "pseudo",
        },
        {
          name: "py",
          subcommands: [
            {
              name: "filter",
            },
            {
              name: "map",
            },
          ],
        },
        {
          name: "rename",
        },
        {
          name: "replace",
        },
        {
          name: "reverse",
        },
        {
          name: "safenames",
        },
        {
          name: "sample",
        },
        {
          name: "schema",
        },
        {
          name: "scoresql",
        },
        {
          name: "search",
        },
        {
          name: "searchset",
        },
        {
          name: "select",
        },
        {
          name: "slice",
        },
        {
          name: "snappy",
          subcommands: [
            {
              name: "check",
            },
            {
              name: "compress",
            },
            {
              name: "decompress",
            },
            {
              name: "validate",
            },
          ],
        },
        {
          name: "sniff",
        },
        {
          name: "sort",
        },
        {
          name: "sortcheck",
        },
        {
          name: "split",
        },
        {
          name: "sqlp",
        },
        {
          name: "stats",
        },
        {
          name: "table",
        },
        {
          name: "template",
        },
        {
          name: "to",
          subcommands: [
            {
              name: "datapackage",
            },
            {
              name: "ods",
            },
            {
              name: "parquet",
            },
            {
              name: "postgres",
            },
            {
              name: "sqlite",
            },
            {
              name: "xlsx",
            },
          ],
        },
        {
          name: "tojsonl",
        },
        {
          name: "transpose",
        },
        {
          name: "validate",
          subcommands: [
            {
              name: "schema",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
        },
      ],
    },
  ],
  options: [
    {
      name: "--list",
    },
    {
      name: "--envlist",
    },
    {
      name: "--update",
    },
    {
      name: "--updatenow",
    },
    {
      name: ["-V", "--version"],
    },
    {
      name: ["-h", "--help"],
      description: "Print help",
    },
  ],
};

export default completion;
