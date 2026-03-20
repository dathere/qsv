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
              name: ["-C", "--comparand"],
              isRepeatable: true,
              args: {
                name: "comparand",
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
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--progressbar"],
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
          name: "dynfmt",
          options: [
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
              name: ["-C", "--comparand"],
              isRepeatable: true,
              args: {
                name: "comparand",
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
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--progressbar"],
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
          name: "emptyreplace",
          options: [
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
              name: ["-C", "--comparand"],
              isRepeatable: true,
              args: {
                name: "comparand",
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
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--progressbar"],
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
          name: "operations",
          options: [
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
              name: ["-C", "--comparand"],
              isRepeatable: true,
              args: {
                name: "comparand",
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
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--progressbar"],
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
          name: ["-C", "--comparand"],
          isRepeatable: true,
          args: {
            name: "comparand",
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
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-p", "--progressbar"],
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
      name: "cat",
      subcommands: [
        {
          name: "columns",
          options: [
            {
              name: ["-N", "--group-name"],
              isRepeatable: true,
              args: {
                name: "group-name",
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
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
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
              name: ["-p", "--pad"],
            },
            {
              name: "--flexible",
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
          name: "rows",
          options: [
            {
              name: ["-N", "--group-name"],
              isRepeatable: true,
              args: {
                name: "group-name",
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
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
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
              name: ["-p", "--pad"],
            },
            {
              name: "--flexible",
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
          name: "rowskey",
          options: [
            {
              name: ["-N", "--group-name"],
              isRepeatable: true,
              args: {
                name: "group-name",
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
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
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
              name: ["-p", "--pad"],
            },
            {
              name: "--flexible",
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
          name: ["-N", "--group-name"],
          isRepeatable: true,
          args: {
            name: "group-name",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-p", "--pad"],
        },
        {
          name: "--flexible",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-C", "--color"],
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
          name: ["-n", "--no-headers"],
        },
        {
          name: "--low-memory",
        },
        {
          name: "--no-polars",
        },
        {
          name: "--width-no-delims",
        },
        {
          name: ["-H", "--human-readable"],
        },
        {
          name: "--json",
        },
        {
          name: "--width",
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
      name: "datefmt",
      options: [
        {
          name: "--default-tz",
          isRepeatable: true,
          args: {
            name: "default-tz",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
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
          name: ["-R", "--ts-resolution"],
          isRepeatable: true,
          args: {
            name: "ts-resolution",
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
          name: ["-r", "--rename"],
          isRepeatable: true,
          args: {
            name: "rename",
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
          name: "--output-tz",
          isRepeatable: true,
          args: {
            name: "output-tz",
            isOptional: true,
          },
        },
        {
          name: "--utc",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--keep-zero-time",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--zulu",
        },
        {
          name: "--prefer-dmy",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: "--sorted",
        },
        {
          name: ["-H", "--human-readable"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-N", "--numeric"],
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-i", "--ignore-case"],
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
          name: "--format",
          isRepeatable: true,
          args: {
            name: "format",
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
          name: "--addl-props",
          isRepeatable: true,
          args: {
            name: "addl-props",
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
          name: "--truncate-str",
          isRepeatable: true,
          args: {
            name: "truncate-str",
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
          name: ["-t", "--max-tokens"],
          isRepeatable: true,
          args: {
            name: "max-tokens",
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
          name: "--score-max-retries",
          isRepeatable: true,
          args: {
            name: "score-max-retries",
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
          name: "--freq-options",
          isRepeatable: true,
          args: {
            name: "freq-options",
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
          name: "--enum-threshold",
          isRepeatable: true,
          args: {
            name: "enum-threshold",
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
          name: "--session",
          isRepeatable: true,
          args: {
            name: "session",
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
          name: "--stats-options",
          isRepeatable: true,
          args: {
            name: "stats-options",
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
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
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
          name: ["-k", "--api-key"],
          isRepeatable: true,
          args: {
            name: "api-key",
            isOptional: true,
          },
        },
        {
          name: "--addl-cols-list",
          isRepeatable: true,
          args: {
            name: "addl-cols-list",
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
          name: "--score-threshold",
          isRepeatable: true,
          args: {
            name: "score-threshold",
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
          name: "--disk-cache-dir",
          isRepeatable: true,
          args: {
            name: "disk-cache-dir",
            isOptional: true,
          },
        },
        {
          name: "--tags",
        },
        {
          name: "--description",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--fresh",
        },
        {
          name: ["-A", "--all"],
        },
        {
          name: "--fewshot-examples",
        },
        {
          name: "--flush-cache",
        },
        {
          name: "--prepare-context",
        },
        {
          name: "--no-score-sql",
        },
        {
          name: "--redis-cache",
        },
        {
          name: "--forget",
        },
        {
          name: "--process-response",
        },
        {
          name: "--addl-cols",
        },
        {
          name: "--dictionary",
        },
        {
          name: "--no-cache",
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
          name: "--delimiter-right",
          isRepeatable: true,
          args: {
            name: "delimiter-right",
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
          name: "--sort-columns",
          isRepeatable: true,
          args: {
            name: "sort-columns",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
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
          name: "--drop-equal-fields",
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
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-i", "--in-place"],
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: "--start",
          isRepeatable: true,
          args: {
            name: "start",
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
          name: "--copy",
          isRepeatable: true,
          args: {
            name: "copy",
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
          name: "--table",
          isRepeatable: true,
          args: {
            name: "table",
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
          name: "--error-format",
          isRepeatable: true,
          args: {
            name: "error-format",
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
          name: "--cell",
          isRepeatable: true,
          args: {
            name: "cell",
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
          name: "--header-row",
          isRepeatable: true,
          args: {
            name: "header-row",
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
          name: "--metadata",
          isRepeatable: true,
          args: {
            name: "metadata",
            isOptional: true,
          },
        },
        {
          name: "--flexible",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--keep-zero-time",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-v", "--invert"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-i", "--ignore-case"],
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: "--temp-dir",
          isRepeatable: true,
          args: {
            name: "temp-dir",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-H", "--human-readable"],
        },
        {
          name: "--no-output",
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
          name: "--tmp-dir",
          isRepeatable: true,
          args: {
            name: "tmp-dir",
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
          name: "--memory-limit",
          isRepeatable: true,
          args: {
            name: "memory-limit",
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
          name: ["-R", "--reverse"],
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
      name: "fetch",
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
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
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
          name: "--mem-cache-size",
          isRepeatable: true,
          args: {
            name: "mem-cache-size",
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
          name: "--report",
          isRepeatable: true,
          args: {
            name: "report",
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
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: "--max-errors",
          isRepeatable: true,
          args: {
            name: "max-errors",
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
          name: "--max-retries",
          isRepeatable: true,
          args: {
            name: "max-retries",
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
          name: "--cookies",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--store-error",
        },
        {
          name: "--redis-cache",
        },
        {
          name: "--disk-cache",
        },
        {
          name: "--pretty",
        },
        {
          name: "--cache-error",
        },
        {
          name: "--no-cache",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--flush-cache",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
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
          name: "--jaq",
          isRepeatable: true,
          args: {
            name: "jaq",
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
          name: "--jaqfile",
          isRepeatable: true,
          args: {
            name: "jaqfile",
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
          name: "--report",
          isRepeatable: true,
          args: {
            name: "report",
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
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
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
          name: "--content-type",
          isRepeatable: true,
          args: {
            name: "content-type",
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
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: "--no-cache",
        },
        {
          name: "--redis-cache",
        },
        {
          name: "--disk-cache",
        },
        {
          name: "--pretty",
        },
        {
          name: "--cookies",
        },
        {
          name: "--compress",
        },
        {
          name: "--flush-cache",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--store-error",
        },
        {
          name: "--cache-error",
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
          name: ["-v", "--default"],
          isRepeatable: true,
          args: {
            name: "default",
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
          name: ["-f", "--first"],
        },
        {
          name: ["-b", "--backfill"],
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
          name: ["-l", "--length"],
          isRepeatable: true,
          args: {
            name: "length",
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
          name: "--escape",
          isRepeatable: true,
          args: {
            name: "escape",
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
          name: ["-i", "--insert"],
          isRepeatable: true,
          args: {
            name: "insert",
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
          name: ["-f", "--field-separator"],
          isRepeatable: true,
          args: {
            name: "field-separator",
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
          name: ["-c", "--condense"],
          isRepeatable: true,
          args: {
            name: "condense",
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
          name: "--quote",
          isRepeatable: true,
          args: {
            name: "quote",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--crlf",
        },
        {
          name: "--ascii",
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
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
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
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-u", "--unify"],
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
      name: "frequency",
      options: [
        {
          name: ["-l", "--limit"],
          isRepeatable: true,
          args: {
            name: "limit",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: "--pct-dec-places",
          isRepeatable: true,
          args: {
            name: "pct-dec-places",
            isOptional: true,
          },
        },
        {
          name: "--all-unique-text",
          isRepeatable: true,
          args: {
            name: "all-unique-text",
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
          name: "--other-text",
          isRepeatable: true,
          args: {
            name: "other-text",
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
          name: "--null-text",
          isRepeatable: true,
          args: {
            name: "null-text",
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
          name: ["-u", "--unq-limit"],
          isRepeatable: true,
          args: {
            name: "unq-limit",
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
          name: "--lmt-threshold",
          isRepeatable: true,
          args: {
            name: "lmt-threshold",
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
          name: "--high-card-threshold",
          isRepeatable: true,
          args: {
            name: "high-card-threshold",
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
          name: "--pretty-json",
        },
        {
          name: "--no-nulls",
        },
        {
          name: "--no-stats",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--memcheck",
        },
        {
          name: "--force",
        },
        {
          name: "--no-other",
        },
        {
          name: "--vis-whitespace",
        },
        {
          name: "--other-sorted",
        },
        {
          name: "--null-sorted",
        },
        {
          name: "--no-trim",
        },
        {
          name: "--frequency-jsonl",
        },
        {
          name: "--pct-nulls",
        },
        {
          name: "--json",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-a", "--asc"],
        },
        {
          name: "--toon",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
              name: ["-r", "--rename"],
              isRepeatable: true,
              args: {
                name: "rename",
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
              name: "--invalid-result",
              isRepeatable: true,
              args: {
                name: "invalid-result",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-f", "--formatstr"],
              isRepeatable: true,
              args: {
                name: "formatstr",
                isOptional: true,
              },
            },
            {
              name: "--admin1",
              isRepeatable: true,
              args: {
                name: "admin1",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: ["-p", "--progressbar"],
            },
            {
              name: "--force",
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
          name: ["-r", "--rename"],
          isRepeatable: true,
          args: {
            name: "rename",
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
          name: "--invalid-result",
          isRepeatable: true,
          args: {
            name: "invalid-result",
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
          name: "--languages",
          isRepeatable: true,
          args: {
            name: "languages",
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
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
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
          name: ["-f", "--formatstr"],
          isRepeatable: true,
          args: {
            name: "formatstr",
            isOptional: true,
          },
        },
        {
          name: "--admin1",
          isRepeatable: true,
          args: {
            name: "admin1",
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
          name: "--cache-dir",
          isRepeatable: true,
          args: {
            name: "cache-dir",
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
          name: "--country",
          isRepeatable: true,
          args: {
            name: "country",
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
          name: ["-p", "--progressbar"],
        },
        {
          name: "--force",
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
          name: ["-l", "--max-length"],
          isRepeatable: true,
          args: {
            name: "max-length",
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
          name: ["-g", "--geometry"],
          isRepeatable: true,
          args: {
            name: "geometry",
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
          name: "--trim",
        },
        {
          name: ["-j", "--just-names"],
        },
        {
          name: ["-J", "--just-count"],
        },
        {
          name: "--intersect",
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
          name: "--quote",
          isRepeatable: true,
          args: {
            name: "quote",
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
          name: "--skip-lastlines",
          isRepeatable: true,
          args: {
            name: "skip-lastlines",
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
          name: "--comment",
          isRepeatable: true,
          args: {
            name: "comment",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--no-quoting",
        },
        {
          name: "--trim-fields",
        },
        {
          name: "--auto-skip",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--left",
        },
        {
          name: "--cross",
        },
        {
          name: "--right-semi",
        },
        {
          name: "--nulls",
        },
        {
          name: "--left-anti",
        },
        {
          name: "--full",
        },
        {
          name: "--right",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--right-anti",
        },
        {
          name: "--left-semi",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-z", "--ignore-leading-zeros"],
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
          name: "--non-equi",
          isRepeatable: true,
          args: {
            name: "non-equi",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: "--sql-filter",
          isRepeatable: true,
          args: {
            name: "sql-filter",
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
          name: "--strategy",
          isRepeatable: true,
          args: {
            name: "strategy",
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
          name: "--datetime-format",
          isRepeatable: true,
          args: {
            name: "datetime-format",
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
          name: "--left_by",
          isRepeatable: true,
          args: {
            name: "left_by",
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
          name: ["-N", "--norm-unicode"],
          isRepeatable: true,
          args: {
            name: "norm-unicode",
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
          name: "--float-precision",
          isRepeatable: true,
          args: {
            name: "float-precision",
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
          name: ["-z", "--ignore-leading-zeros"],
        },
        {
          name: "--low-memory",
        },
        {
          name: "--asof",
        },
        {
          name: "--left-anti",
        },
        {
          name: "--ignore-errors",
        },
        {
          name: "--streaming",
        },
        {
          name: ["-X", "--allow-exact-matches"],
        },
        {
          name: "--full",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--left",
        },
        {
          name: "--decimal-comma",
        },
        {
          name: "--coalesce",
        },
        {
          name: "--no-optimizations",
        },
        {
          name: "--nulls",
        },
        {
          name: "--try-parsedates",
        },
        {
          name: "--no-sort",
        },
        {
          name: "--right-anti",
        },
        {
          name: "--right-semi",
        },
        {
          name: "--cross",
        },
        {
          name: "--left-semi",
        },
        {
          name: "--right",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: "--jaq",
          isRepeatable: true,
          args: {
            name: "jaq",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: "--echo-column",
          isRepeatable: true,
          args: {
            name: "echo-column",
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
          name: "--columns",
          isRepeatable: true,
          args: {
            name: "columns",
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
          name: "--filter",
          isRepeatable: true,
          args: {
            name: "filter",
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
          name: ["-m", "--monochrome"],
        },
        {
          name: ["-S", "--streaming-stdin"],
        },
        {
          name: ["-t", "--tab-separated"],
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--debug",
        },
        {
          name: "--no-headers",
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
              name: "--max-errors",
              isRepeatable: true,
              args: {
                name: "max-errors",
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
              name: "--ckan-api",
              isRepeatable: true,
              args: {
                name: "ckan-api",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
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
              name: ["-r", "--remap"],
            },
            {
              name: "--colindex",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-g", "--no-globals"],
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
              name: "--max-errors",
              isRepeatable: true,
              args: {
                name: "max-errors",
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
              name: "--ckan-api",
              isRepeatable: true,
              args: {
                name: "ckan-api",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
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
              name: ["-r", "--remap"],
            },
            {
              name: "--colindex",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-g", "--no-globals"],
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
          name: "--max-errors",
          isRepeatable: true,
          args: {
            name: "max-errors",
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
          name: "--ckan-api",
          isRepeatable: true,
          args: {
            name: "ckan-api",
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
          name: "--cache-dir",
          isRepeatable: true,
          args: {
            name: "cache-dir",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-r", "--remap"],
        },
        {
          name: "--colindex",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-g", "--no-globals"],
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
          name: "--stats-options",
          isRepeatable: true,
          args: {
            name: "stats-options",
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
          name: ["-C", "--cardinality-threshold"],
          isRepeatable: true,
          args: {
            name: "cardinality-threshold",
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
          name: "--pct-thresholds",
          isRepeatable: true,
          args: {
            name: "pct-thresholds",
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
          name: ["-e", "--epsilon"],
          isRepeatable: true,
          args: {
            name: "epsilon",
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
          name: "--force",
        },
        {
          name: ["-B", "--bivariate"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--use-percentiles",
        },
        {
          name: "--advanced",
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
          name: ["-p", "--prefix-length"],
          isRepeatable: true,
          args: {
            name: "prefix-length",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: "--try-parsedates",
        },
        {
          name: "--sort-columns",
        },
        {
          name: "--ignore-errors",
        },
        {
          name: "--validate",
        },
        {
          name: "--maintain-order",
        },
        {
          name: "--decimal-comma",
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
      name: "pragmastat",
      options: [
        {
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
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
          name: "--compare2",
          isRepeatable: true,
          args: {
            name: "compare2",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: ["-m", "--misrate"],
          isRepeatable: true,
          args: {
            name: "misrate",
            isOptional: true,
          },
        },
        {
          name: "--compare1",
          isRepeatable: true,
          args: {
            name: "compare1",
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
          name: "--no-bounds",
        },
        {
          name: "--standalone",
        },
        {
          name: "--memcheck",
        },
        {
          name: "--force",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-F", "--filters"],
          isRepeatable: true,
          args: {
            name: "filters",
            isOptional: true,
          },
        },
        {
          name: "--base-delay-ms",
          isRepeatable: true,
          args: {
            name: "base-delay-ms",
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
          name: "--save-fname",
          isRepeatable: true,
          args: {
            name: "save-fname",
            isOptional: true,
          },
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-f", "--fd-output"],
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
          name: "--start",
          isRepeatable: true,
          args: {
            name: "start",
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
          name: "--formatstr",
          isRepeatable: true,
          args: {
            name: "formatstr",
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
              name: ["-f", "--helper"],
              isRepeatable: true,
              args: {
                name: "helper",
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
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--progressbar"],
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
          name: "map",
          options: [
            {
              name: ["-f", "--helper"],
              isRepeatable: true,
              args: {
                name: "helper",
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
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--progressbar"],
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
          name: ["-f", "--helper"],
          isRepeatable: true,
          args: {
            name: "helper",
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
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
            isOptional: true,
          },
        },
        {
          name: ["-p", "--progressbar"],
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
          name: "--dfa-size-limit",
          isRepeatable: true,
          args: {
            name: "dfa-size-limit",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
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
          name: ["-u", "--unicode"],
        },
        {
          name: "--not-one",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--literal",
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
          name: "--reserved",
          isRepeatable: true,
          args: {
            name: "reserved",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sample",
      options: [
        {
          name: "--weighted",
          isRepeatable: true,
          args: {
            name: "weighted",
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
          name: "--max-size",
          isRepeatable: true,
          args: {
            name: "max-size",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: "--ts-aggregate",
          isRepeatable: true,
          args: {
            name: "ts-aggregate",
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
          name: "--ts-interval",
          isRepeatable: true,
          args: {
            name: "ts-interval",
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
          name: "--cluster",
          isRepeatable: true,
          args: {
            name: "cluster",
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
          name: "--ts-input-tz",
          isRepeatable: true,
          args: {
            name: "ts-input-tz",
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
          name: "--seed",
          isRepeatable: true,
          args: {
            name: "seed",
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
          name: "--rng",
          isRepeatable: true,
          args: {
            name: "rng",
            isOptional: true,
          },
        },
        {
          name: "--ts-prefer-dmy",
        },
        {
          name: "--force",
        },
        {
          name: "--bernoulli",
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
      name: "schema",
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
          name: "--pattern-columns",
          isRepeatable: true,
          args: {
            name: "pattern-columns",
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
          name: "--dates-whitelist",
          isRepeatable: true,
          args: {
            name: "dates-whitelist",
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
          name: "--strict-dates",
        },
        {
          name: "--force",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--strict-formats",
        },
        {
          name: "--stdout",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--polars",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: "--duckdb",
        },
        {
          name: "--truncate-ragged-lines",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--json",
        },
        {
          name: "--ignore-errors",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: ["-f", "--flag"],
          isRepeatable: true,
          args: {
            name: "flag",
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
          name: "--size-limit",
          isRepeatable: true,
          args: {
            name: "size-limit",
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
          name: "--preview-match",
          isRepeatable: true,
          args: {
            name: "preview-match",
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
          name: ["-Q", "--quick"],
        },
        {
          name: ["-p", "--progressbar"],
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
          name: ["-c", "--count"],
        },
        {
          name: "--exact",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-v", "--invert-match"],
        },
        {
          name: ["-u", "--unicode"],
        },
        {
          name: "--json",
        },
        {
          name: ["-i", "--ignore-case"],
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
          name: "--unmatched-output",
          isRepeatable: true,
          args: {
            name: "unmatched-output",
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
          name: "--size-limit",
          isRepeatable: true,
          args: {
            name: "size-limit",
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
          name: ["-f", "--flag"],
          isRepeatable: true,
          args: {
            name: "flag",
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
          name: "--dfa-size-limit",
          isRepeatable: true,
          args: {
            name: "dfa-size-limit",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-c", "--count"],
        },
        {
          name: ["-Q", "--quick"],
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
          name: "--exact",
        },
        {
          name: ["-j", "--json"],
        },
        {
          name: ["-v", "--invert-match"],
        },
        {
          name: "--flag-matches-only",
        },
        {
          name: "--literal",
        },
        {
          name: "--not-one",
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
          name: "--seed",
          isRepeatable: true,
          args: {
            name: "seed",
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
          name: ["-R", "--random"],
        },
        {
          name: ["-S", "--sort"],
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
      name: "slice",
      options: [
        {
          name: ["-e", "--end"],
          isRepeatable: true,
          args: {
            name: "end",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
              name: ["-q", "--quiet"],
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
              name: ["-q", "--quiet"],
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
              name: ["-q", "--quiet"],
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
              name: ["-q", "--quiet"],
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
          name: ["-q", "--quiet"],
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
      name: "sniff",
      options: [
        {
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
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
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
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
          name: "--save-urlsample",
          isRepeatable: true,
          args: {
            name: "save-urlsample",
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
          name: ["-p", "--progressbar"],
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--stats-types",
        },
        {
          name: "--pretty-json",
        },
        {
          name: "--just-mime",
        },
        {
          name: "--harvest-mode",
        },
        {
          name: ["-Q", "--quick"],
        },
        {
          name: "--json",
        },
        {
          name: "--no-infer",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: "--faster",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-u", "--unique"],
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-R", "--reverse"],
        },
        {
          name: "--random",
        },
        {
          name: ["-N", "--numeric"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--natural",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--all",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--json",
        },
        {
          name: "--pretty-json",
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
          name: "--filter",
          isRepeatable: true,
          args: {
            name: "filter",
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
          name: "--filename",
          isRepeatable: true,
          args: {
            name: "filename",
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
          name: "--filter-cleanup",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--filter-ignore-errors",
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
          name: "--format",
          isRepeatable: true,
          args: {
            name: "format",
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
          name: "--infer-len",
          isRepeatable: true,
          args: {
            name: "infer-len",
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
          name: "--compression",
          isRepeatable: true,
          args: {
            name: "compression",
            isOptional: true,
          },
        },
        {
          name: "--compress-level",
          isRepeatable: true,
          args: {
            name: "compress-level",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-q", "--quiet"],
        },
        {
          name: "--ignore-errors",
        },
        {
          name: "--statistics",
        },
        {
          name: "--try-parsedates",
        },
        {
          name: "--decimal-comma",
        },
        {
          name: "--no-optimizations",
        },
        {
          name: "--low-memory",
        },
        {
          name: "--cache-schema",
        },
        {
          name: "--streaming",
        },
        {
          name: "--truncate-ragged-lines",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: "--weight",
          isRepeatable: true,
          args: {
            name: "weight",
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
          name: "--dates-whitelist",
          isRepeatable: true,
          args: {
            name: "dates-whitelist",
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
          name: "--boolean-patterns",
          isRepeatable: true,
          args: {
            name: "boolean-patterns",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
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
          name: "--median",
        },
        {
          name: "--mode",
        },
        {
          name: "--memcheck",
        },
        {
          name: "--quartiles",
        },
        {
          name: "--nulls",
        },
        {
          name: "--force",
        },
        {
          name: "--vis-whitespace",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--cardinality",
        },
        {
          name: "--mad",
        },
        {
          name: "--stats-jsonl",
        },
        {
          name: "--infer-boolean",
        },
        {
          name: "--typesonly",
        },
        {
          name: "--infer-dates",
        },
        {
          name: "--percentiles",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-E", "--everything"],
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-c", "--condense"],
          isRepeatable: true,
          args: {
            name: "condense",
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
          name: ["-t", "--template-file"],
          isRepeatable: true,
          args: {
            name: "template-file",
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
          name: "--ckan-token",
          isRepeatable: true,
          args: {
            name: "ckan-token",
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
          name: "--delimiter",
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
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
          name: "--template",
          isRepeatable: true,
          args: {
            name: "template",
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
          name: "--customfilter-error",
          isRepeatable: true,
          args: {
            name: "customfilter-error",
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
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
            isOptional: true,
          },
        },
        {
          name: ["-p", "--progressbar"],
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
      name: "to",
      subcommands: [
        {
          name: "datapackage",
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
              name: ["-p", "--separator"],
              isRepeatable: true,
              args: {
                name: "separator",
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
              name: ["-s", "--schema"],
              isRepeatable: true,
              args: {
                name: "schema",
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
              name: ["-c", "--stats-csv"],
              isRepeatable: true,
              args: {
                name: "stats-csv",
                isOptional: true,
              },
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--drop",
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
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
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
              name: ["-t", "--table"],
              isRepeatable: true,
              args: {
                name: "table",
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
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
              name: ["-a", "--stats"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--drop",
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
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
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
              name: ["-t", "--table"],
              isRepeatable: true,
              args: {
                name: "table",
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
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
              name: ["-a", "--stats"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--drop",
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
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
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
              name: ["-t", "--table"],
              isRepeatable: true,
              args: {
                name: "table",
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
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
              name: ["-a", "--stats"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--drop",
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
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
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
              name: ["-t", "--table"],
              isRepeatable: true,
              args: {
                name: "table",
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
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
              name: ["-a", "--stats"],
            },
            {
              name: ["-k", "--print-package"],
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-e", "--evolve"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--drop",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
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
          name: ["-t", "--table"],
          isRepeatable: true,
          args: {
            name: "table",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: ["-a", "--stats"],
        },
        {
          name: ["-k", "--print-package"],
        },
        {
          name: ["-u", "--dump"],
        },
        {
          name: ["-A", "--all-strings"],
        },
        {
          name: ["-i", "--pipe"],
        },
        {
          name: ["-e", "--evolve"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--drop",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
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
          name: "--memcheck",
        },
        {
          name: "--trim",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--no-boolean",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
              name: "--dfa-size-limit",
              isRepeatable: true,
              args: {
                name: "dfa-size-limit",
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
              name: "--ckan-api",
              isRepeatable: true,
              args: {
                name: "ckan-api",
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
              name: ["-j", "--jobs"],
              isRepeatable: true,
              args: {
                name: "jobs",
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
              name: "--valid-output",
              isRepeatable: true,
              args: {
                name: "valid-output",
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
              name: "--invalid",
              isRepeatable: true,
              args: {
                name: "invalid",
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
              name: "--email-min-subdomains",
              isRepeatable: true,
              args: {
                name: "email-min-subdomains",
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
              name: "--trim",
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--pretty-json",
            },
            {
              name: "--email-domain-literal",
            },
            {
              name: "--fail-fast",
            },
            {
              name: "--email-display-text",
            },
            {
              name: "--json",
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: "--email-required-tld",
            },
            {
              name: "--fancy-regex",
            },
            {
              name: "--no-format-validation",
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
          name: "--dfa-size-limit",
          isRepeatable: true,
          args: {
            name: "dfa-size-limit",
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
          name: "--ckan-api",
          isRepeatable: true,
          args: {
            name: "ckan-api",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
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
          name: "--valid-output",
          isRepeatable: true,
          args: {
            name: "valid-output",
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
          name: "--invalid",
          isRepeatable: true,
          args: {
            name: "invalid",
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
          name: "--email-min-subdomains",
          isRepeatable: true,
          args: {
            name: "email-min-subdomains",
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
          name: "--trim",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--pretty-json",
        },
        {
          name: "--email-domain-literal",
        },
        {
          name: "--fail-fast",
        },
        {
          name: "--email-display-text",
        },
        {
          name: "--json",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--email-required-tld",
        },
        {
          name: "--fancy-regex",
        },
        {
          name: "--no-format-validation",
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
