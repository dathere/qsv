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
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
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
              name: ["-R", "--replacement"],
              isRepeatable: true,
              args: {
                name: "replacement",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
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
              name: ["-R", "--replacement"],
              isRepeatable: true,
              args: {
                name: "replacement",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
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
              name: ["-R", "--replacement"],
              isRepeatable: true,
              args: {
                name: "replacement",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
              name: ["-o", "--output"],
              isRepeatable: true,
              args: {
                name: "output",
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
              name: ["-R", "--replacement"],
              isRepeatable: true,
              args: {
                name: "replacement",
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
              name: ["-c", "--new-column"],
              isRepeatable: true,
              args: {
                name: "new-column",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-R", "--replacement"],
          isRepeatable: true,
          args: {
            name: "replacement",
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
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
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
      name: "cat",
      subcommands: [
        {
          name: "columns",
          options: [
            {
              name: ["-g", "--group"],
              isRepeatable: true,
              args: {
                name: "group",
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
              name: ["-N", "--group-name"],
              isRepeatable: true,
              args: {
                name: "group-name",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--pad"],
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: "--flexible",
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
              name: ["-g", "--group"],
              isRepeatable: true,
              args: {
                name: "group",
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
              name: ["-N", "--group-name"],
              isRepeatable: true,
              args: {
                name: "group-name",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--pad"],
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: "--flexible",
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
              name: ["-g", "--group"],
              isRepeatable: true,
              args: {
                name: "group",
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
              name: ["-N", "--group-name"],
              isRepeatable: true,
              args: {
                name: "group-name",
                isOptional: true,
              },
            },
            {
              name: ["-p", "--pad"],
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: "--flexible",
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
          name: ["-g", "--group"],
          isRepeatable: true,
          args: {
            name: "group",
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
          name: ["-N", "--group-name"],
          isRepeatable: true,
          args: {
            name: "group-name",
            isOptional: true,
          },
        },
        {
          name: ["-p", "--pad"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--flexible",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-H", "--human-readable"],
        },
        {
          name: ["-f", "--flexible"],
        },
        {
          name: "--json",
        },
        {
          name: "--width",
        },
        {
          name: "--width-no-delims",
        },
        {
          name: "--no-polars",
        },
        {
          name: "--low-memory",
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
          name: ["-c", "--new-column"],
          isRepeatable: true,
          args: {
            name: "new-column",
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
          name: "--input-tz",
          isRepeatable: true,
          args: {
            name: "input-tz",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
            isOptional: true,
          },
        },
        {
          name: "--utc",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--zulu",
        },
        {
          name: "--keep-zero-time",
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
          name: ["-N", "--numeric"],
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--sorted",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-H", "--human-readable"],
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
      name: "describegpt",
      options: [
        {
          name: "--addl-props",
          isRepeatable: true,
          args: {
            name: "addl-props",
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
          name: "--export-prompt",
          isRepeatable: true,
          args: {
            name: "export-prompt",
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
          name: "--enum-threshold",
          isRepeatable: true,
          args: {
            name: "enum-threshold",
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
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
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
          name: "--sample-size",
          isRepeatable: true,
          args: {
            name: "sample-size",
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
          name: "--freq-options",
          isRepeatable: true,
          args: {
            name: "freq-options",
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
          name: "--format",
          isRepeatable: true,
          args: {
            name: "format",
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
          name: ["-t", "--max-tokens"],
          isRepeatable: true,
          args: {
            name: "max-tokens",
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
          name: ["-m", "--model"],
          isRepeatable: true,
          args: {
            name: "model",
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
          name: "--truncate-str",
          isRepeatable: true,
          args: {
            name: "truncate-str",
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
          name: "--language",
          isRepeatable: true,
          args: {
            name: "language",
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
          name: ["-k", "--api-key"],
          isRepeatable: true,
          args: {
            name: "api-key",
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
          name: "--sql-results",
          isRepeatable: true,
          args: {
            name: "sql-results",
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
          name: "--num-examples",
          isRepeatable: true,
          args: {
            name: "num-examples",
            isOptional: true,
          },
        },
        {
          name: "--fresh",
        },
        {
          name: ["-A", "--all"],
        },
        {
          name: "--redis-cache",
        },
        {
          name: "--description",
        },
        {
          name: "--addl-cols",
        },
        {
          name: "--tags",
        },
        {
          name: "--no-cache",
        },
        {
          name: "--fewshot-examples",
        },
        {
          name: "--dictionary",
        },
        {
          name: "--forget",
        },
        {
          name: "--flush-cache",
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
      name: "diff",
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
          name: "--delimiter-right",
          isRepeatable: true,
          args: {
            name: "delimiter-right",
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
          name: "--no-headers-output",
        },
        {
          name: "--no-headers-left",
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
          name: "--copy",
          isRepeatable: true,
          args: {
            name: "copy",
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
          name: "--start",
          isRepeatable: true,
          args: {
            name: "start",
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
          name: "--hash",
          isRepeatable: true,
          args: {
            name: "hash",
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
          name: "--uuid7",
        },
        {
          name: "--uuid4",
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
          name: "--header-row",
          isRepeatable: true,
          args: {
            name: "header-row",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
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
          name: "--error-format",
          isRepeatable: true,
          args: {
            name: "error-format",
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
          name: "--keep-zero-time",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--flexible",
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
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-v", "--invert"],
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
          name: ["-r", "--rename"],
          isRepeatable: true,
          args: {
            name: "rename",
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
          name: "--no-output",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-H", "--human-readable"],
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
          name: "--memory-limit",
          isRepeatable: true,
          args: {
            name: "memory-limit",
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
          name: "--mem-cache-size",
          isRepeatable: true,
          args: {
            name: "mem-cache-size",
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
          name: "--jaqfile",
          isRepeatable: true,
          args: {
            name: "jaqfile",
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
          name: "--disk-cache-dir",
          isRepeatable: true,
          args: {
            name: "disk-cache-dir",
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
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
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
          name: "--url-template",
          isRepeatable: true,
          args: {
            name: "url-template",
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
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
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
          name: "--rate-limit",
          isRepeatable: true,
          args: {
            name: "rate-limit",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--flush-cache",
        },
        {
          name: "--redis-cache",
        },
        {
          name: "--cookies",
        },
        {
          name: "--no-cache",
        },
        {
          name: "--pretty",
        },
        {
          name: "--store-error",
        },
        {
          name: "--cache-error",
        },
        {
          name: "--disk-cache",
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
      name: "fetchpost",
      options: [
        {
          name: "--jaqfile",
          isRepeatable: true,
          args: {
            name: "jaqfile",
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
          name: ["-H", "--http-header"],
          isRepeatable: true,
          args: {
            name: "http-header",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: "--max-retries",
          isRepeatable: true,
          args: {
            name: "max-retries",
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
          name: "--jaq",
          isRepeatable: true,
          args: {
            name: "jaq",
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
          name: "--mem-cache-size",
          isRepeatable: true,
          args: {
            name: "mem-cache-size",
            isOptional: true,
          },
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--no-cache",
        },
        {
          name: "--compress",
        },
        {
          name: "--store-error",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--cache-error",
        },
        {
          name: "--flush-cache",
        },
        {
          name: "--cookies",
        },
        {
          name: "--pretty",
        },
        {
          name: "--disk-cache",
        },
        {
          name: "--redis-cache",
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
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-f", "--first"],
        },
        {
          name: ["-b", "--backfill"],
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
          name: ["-i", "--insert"],
          isRepeatable: true,
          args: {
            name: "insert",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: ["-r", "--remove-empty"],
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
          name: "--quote",
          isRepeatable: true,
          args: {
            name: "quote",
            isOptional: true,
          },
        },
        {
          name: "--quote-never",
        },
        {
          name: "--crlf",
        },
        {
          name: "--quote-always",
        },
        {
          name: "--no-final-newline",
        },
        {
          name: "--ascii",
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
          name: "--stats-filter",
          isRepeatable: true,
          args: {
            name: "stats-filter",
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
          name: "--pct-dec-places",
          isRepeatable: true,
          args: {
            name: "pct-dec-places",
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
          name: ["-r", "--rank-strategy"],
          isRepeatable: true,
          args: {
            name: "rank-strategy",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: ["-l", "--limit"],
          isRepeatable: true,
          args: {
            name: "limit",
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
          name: "--pct-nulls",
        },
        {
          name: "--other-sorted",
        },
        {
          name: "--no-stats",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--pretty-json",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--no-trim",
        },
        {
          name: "--memcheck",
        },
        {
          name: "--frequency-jsonl",
        },
        {
          name: ["-a", "--asc"],
        },
        {
          name: "--toon",
        },
        {
          name: "--json",
        },
        {
          name: "--null-sorted",
        },
        {
          name: "--force",
        },
        {
          name: "--no-other",
        },
        {
          name: "--no-nulls",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
              name: "--country",
              isRepeatable: true,
              args: {
                name: "country",
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
              name: "--cache-dir",
              isRepeatable: true,
              args: {
                name: "cache-dir",
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
              name: ["-k", "--k_weight"],
              isRepeatable: true,
              args: {
                name: "k_weight",
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
              name: "--cities-url",
              isRepeatable: true,
              args: {
                name: "cities-url",
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
              name: "--min-score",
              isRepeatable: true,
              args: {
                name: "min-score",
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
              name: "--languages",
              isRepeatable: true,
              args: {
                name: "languages",
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
              name: ["-l", "--language"],
              isRepeatable: true,
              args: {
                name: "language",
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
          name: "--country",
          isRepeatable: true,
          args: {
            name: "country",
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
          name: "--cache-dir",
          isRepeatable: true,
          args: {
            name: "cache-dir",
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
          name: ["-k", "--k_weight"],
          isRepeatable: true,
          args: {
            name: "k_weight",
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
          name: "--cities-url",
          isRepeatable: true,
          args: {
            name: "cities-url",
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
          name: "--min-score",
          isRepeatable: true,
          args: {
            name: "min-score",
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
          name: "--languages",
          isRepeatable: true,
          args: {
            name: "languages",
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
          name: ["-l", "--language"],
          isRepeatable: true,
          args: {
            name: "language",
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
          name: ["-y", "--latitude"],
          isRepeatable: true,
          args: {
            name: "latitude",
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
          name: "--trim",
        },
        {
          name: "--intersect",
        },
        {
          name: ["-j", "--just-names"],
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
          name: "--skip-lines",
          isRepeatable: true,
          args: {
            name: "skip-lines",
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
          name: "--comment",
          isRepeatable: true,
          args: {
            name: "comment",
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
          name: "--auto-skip",
        },
        {
          name: "--trim-headers",
        },
        {
          name: "--no-quoting",
        },
        {
          name: "--trim-fields",
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
          name: "--cross",
        },
        {
          name: "--nulls",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--full",
        },
        {
          name: ["-z", "--ignore-leading-zeros"],
        },
        {
          name: "--right",
        },
        {
          name: "--right-anti",
        },
        {
          name: "--left",
        },
        {
          name: "--right-semi",
        },
        {
          name: "--left-anti",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--left-semi",
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
          name: "--filter-right",
          isRepeatable: true,
          args: {
            name: "filter-right",
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
          name: "--strategy",
          isRepeatable: true,
          args: {
            name: "strategy",
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
          name: "--tolerance",
          isRepeatable: true,
          args: {
            name: "tolerance",
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
          name: "--float-precision",
          isRepeatable: true,
          args: {
            name: "float-precision",
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
          name: "--left_by",
          isRepeatable: true,
          args: {
            name: "left_by",
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
          name: "--right_by",
          isRepeatable: true,
          args: {
            name: "right_by",
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
          name: "--non-equi",
          isRepeatable: true,
          args: {
            name: "non-equi",
            isOptional: true,
          },
        },
        {
          name: "--cache-schema",
          isRepeatable: true,
          args: {
            name: "cache-schema",
            isOptional: true,
          },
        },
        {
          name: "--ignore-errors",
        },
        {
          name: "--no-optimizations",
        },
        {
          name: ["-z", "--ignore-leading-zeros"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--right-semi",
        },
        {
          name: "--right-anti",
        },
        {
          name: "--full",
        },
        {
          name: "--streaming",
        },
        {
          name: "--decimal-comma",
        },
        {
          name: "--left-semi",
        },
        {
          name: "--left",
        },
        {
          name: "--coalesce",
        },
        {
          name: "--right",
        },
        {
          name: "--low-memory",
        },
        {
          name: "--left-anti",
        },
        {
          name: "--no-sort",
        },
        {
          name: "--cross",
        },
        {
          name: ["-X", "--allow-exact-matches"],
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--asof",
        },
        {
          name: "--try-parsedates",
        },
        {
          name: "--nulls",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-P", "--prompt"],
          isRepeatable: true,
          args: {
            name: "prompt",
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
          name: "--echo-column",
          isRepeatable: true,
          args: {
            name: "echo-column",
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
          name: ["-W", "--wrap-mode"],
          isRepeatable: true,
          args: {
            name: "wrap-mode",
            isOptional: true,
          },
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
          name: ["-t", "--tab-separated"],
        },
        {
          name: ["-A", "--auto-reload"],
        },
        {
          name: "--debug",
        },
        {
          name: ["-S", "--streaming-stdin"],
        },
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-B", "--begin"],
              isRepeatable: true,
              args: {
                name: "begin",
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
              name: "--max-errors",
              isRepeatable: true,
              args: {
                name: "max-errors",
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
              name: ["-E", "--end"],
              isRepeatable: true,
              args: {
                name: "end",
                isOptional: true,
              },
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-g", "--no-globals"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-r", "--remap"],
            },
            {
              name: "--colindex",
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
                isOptional: true,
              },
            },
            {
              name: ["-B", "--begin"],
              isRepeatable: true,
              args: {
                name: "begin",
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
              name: "--max-errors",
              isRepeatable: true,
              args: {
                name: "max-errors",
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
              name: ["-E", "--end"],
              isRepeatable: true,
              args: {
                name: "end",
                isOptional: true,
              },
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-g", "--no-globals"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: ["-r", "--remap"],
            },
            {
              name: "--colindex",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-B", "--begin"],
          isRepeatable: true,
          args: {
            name: "begin",
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
          name: "--max-errors",
          isRepeatable: true,
          args: {
            name: "max-errors",
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
          name: ["-E", "--end"],
          isRepeatable: true,
          args: {
            name: "end",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-g", "--no-globals"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-r", "--remap"],
        },
        {
          name: "--colindex",
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
          name: "--round",
          isRepeatable: true,
          args: {
            name: "round",
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
          name: ["-S", "--bivariate-stats"],
          isRepeatable: true,
          args: {
            name: "bivariate-stats",
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
          name: ["-e", "--epsilon"],
          isRepeatable: true,
          args: {
            name: "epsilon",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
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
          name: "--pct-thresholds",
          isRepeatable: true,
          args: {
            name: "pct-thresholds",
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
          name: ["-p", "--progressbar"],
        },
        {
          name: "--force",
        },
        {
          name: "--use-percentiles",
        },
        {
          name: "--advanced",
        },
        {
          name: ["-B", "--bivariate"],
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
          name: ["-p", "--prefix-length"],
          isRepeatable: true,
          args: {
            name: "prefix-length",
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
          name: "--limit",
          isRepeatable: true,
          args: {
            name: "limit",
            isOptional: true,
          },
        },
        {
          name: ["-n", "--no-headers"],
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
      name: "pivotp",
      options: [
        {
          name: ["-v", "--values"],
          isRepeatable: true,
          args: {
            name: "values",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: "--maintain-order",
        },
        {
          name: "--decimal-comma",
        },
        {
          name: "--sort-columns",
        },
        {
          name: "--validate",
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
      name: "pragmastat",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: "--memcheck",
        },
        {
          name: ["-t", "--twosample"],
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
          name: "--save-fname",
          isRepeatable: true,
          args: {
            name: "save-fname",
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
          name: ["-F", "--filters"],
          isRepeatable: true,
          args: {
            name: "filters",
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
          name: "--formatstr",
          isRepeatable: true,
          args: {
            name: "formatstr",
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
              name: ["-f", "--helper"],
              isRepeatable: true,
              args: {
                name: "helper",
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
              name: ["-f", "--helper"],
              isRepeatable: true,
              args: {
                name: "helper",
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
          name: ["-f", "--helper"],
          isRepeatable: true,
          args: {
            name: "helper",
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
          name: "--pairwise",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--exact",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--literal",
        },
        {
          name: "--not-one",
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
          name: ["-n", "--no-headers"],
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
          name: "--mode",
          isRepeatable: true,
          args: {
            name: "mode",
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
          name: "--seed",
          isRepeatable: true,
          args: {
            name: "seed",
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
          name: "--systematic",
          isRepeatable: true,
          args: {
            name: "systematic",
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
          name: "--rng",
          isRepeatable: true,
          args: {
            name: "rng",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: "--stratified",
          isRepeatable: true,
          args: {
            name: "stratified",
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
          name: "--ts-aggregate",
          isRepeatable: true,
          args: {
            name: "ts-aggregate",
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
          name: "--ts-interval",
          isRepeatable: true,
          args: {
            name: "ts-interval",
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
          name: "--ts-start",
          isRepeatable: true,
          args: {
            name: "ts-start",
            isOptional: true,
          },
        },
        {
          name: "--bernoulli",
        },
        {
          name: "--ts-prefer-dmy",
        },
        {
          name: ["-n", "--no-headers"],
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
      name: "schema",
      options: [
        {
          name: "--enum-threshold",
          isRepeatable: true,
          args: {
            name: "enum-threshold",
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
          name: ["-j", "--jobs"],
          isRepeatable: true,
          args: {
            name: "jobs",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--strict-formats",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--stdout",
        },
        {
          name: "--polars",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--strict-dates",
        },
        {
          name: "--memcheck",
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
      name: "search",
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
          name: ["-f", "--flag"],
          isRepeatable: true,
          args: {
            name: "flag",
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
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-v", "--invert-match"],
        },
        {
          name: ["-Q", "--quick"],
        },
        {
          name: ["-c", "--count"],
        },
        {
          name: "--json",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--not-one",
        },
        {
          name: ["-u", "--unicode"],
        },
        {
          name: "--literal",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--exact",
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
          name: "--unmatched-output",
          isRepeatable: true,
          args: {
            name: "unmatched-output",
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
            isOptional: true,
          },
        },
        {
          name: ["-u", "--unicode"],
        },
        {
          name: ["-v", "--invert-match"],
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
          name: "--flag-matches-only",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--exact",
        },
        {
          name: ["-j", "--json"],
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: "--literal",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: ["-Q", "--quick"],
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
          name: ["-i", "--index"],
          isRepeatable: true,
          args: {
            name: "index",
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
          name: "--json",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--invert",
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
              name: "--user-agent",
              isRepeatable: true,
              args: {
                name: "user-agent",
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
              name: "--user-agent",
              isRepeatable: true,
              args: {
                name: "user-agent",
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
              name: "--user-agent",
              isRepeatable: true,
              args: {
                name: "user-agent",
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
              name: "--user-agent",
              isRepeatable: true,
              args: {
                name: "user-agent",
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
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
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
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
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
          name: "--user-agent",
          isRepeatable: true,
          args: {
            name: "user-agent",
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
          name: "--harvest-mode",
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--just-mime",
        },
        {
          name: "--stats-types",
        },
        {
          name: "--pretty-json",
        },
        {
          name: "--json",
        },
        {
          name: ["-Q", "--quick"],
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
          name: ["-s", "--select"],
          isRepeatable: true,
          args: {
            name: "select",
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
          name: "--faster",
        },
        {
          name: ["-N", "--numeric"],
        },
        {
          name: "--natural",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-u", "--unique"],
        },
        {
          name: ["-R", "--reverse"],
        },
        {
          name: "--random",
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
          name: "--pretty-json",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--all",
        },
        {
          name: ["-i", "--ignore-case"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--json",
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
          name: ["-k", "--kb-size"],
          isRepeatable: true,
          args: {
            name: "kb-size",
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
          name: ["-s", "--size"],
          isRepeatable: true,
          args: {
            name: "size",
            isOptional: true,
          },
        },
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
          name: "--pad",
          isRepeatable: true,
          args: {
            name: "pad",
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
          name: ["-q", "--quiet"],
        },
        {
          name: "--filter-ignore-errors",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: "--wnull-value",
          isRepeatable: true,
          args: {
            name: "wnull-value",
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
          name: "--format",
          isRepeatable: true,
          args: {
            name: "format",
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
          name: "--datetime-format",
          isRepeatable: true,
          args: {
            name: "datetime-format",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: "--compression",
          isRepeatable: true,
          args: {
            name: "compression",
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
          name: "--statistics",
        },
        {
          name: "--try-parsedates",
        },
        {
          name: "--low-memory",
        },
        {
          name: "--no-optimizations",
        },
        {
          name: "--decimal-comma",
        },
        {
          name: "--cache-schema",
        },
        {
          name: "--truncate-ragged-lines",
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--streaming",
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
      name: "stats",
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
          name: "--weight",
          isRepeatable: true,
          args: {
            name: "weight",
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
            isOptional: true,
          },
        },
        {
          name: "--cardinality",
        },
        {
          name: "--vis-whitespace",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: "--percentiles",
        },
        {
          name: ["-E", "--everything"],
        },
        {
          name: "--mode",
        },
        {
          name: "--nulls",
        },
        {
          name: "--stats-jsonl",
        },
        {
          name: "--mad",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--force",
        },
        {
          name: "--typesonly",
        },
        {
          name: "--infer-dates",
        },
        {
          name: "--quartiles",
        },
        {
          name: "--median",
        },
        {
          name: "--memcheck",
        },
        {
          name: "--infer-boolean",
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
          name: ["-a", "--align"],
          isRepeatable: true,
          args: {
            name: "align",
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
          name: "--delimiter",
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
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
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
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
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
          name: "--globals-json",
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
          name: "--cache-dir",
          isRepeatable: true,
          args: {
            name: "cache-dir",
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
          name: "--outfilename",
          isRepeatable: true,
          args: {
            name: "outfilename",
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
              name: ["-k", "--print-package"],
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: ["-e", "--evolve"],
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
              name: ["-k", "--print-package"],
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: ["-e", "--evolve"],
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
              name: ["-k", "--print-package"],
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: ["-e", "--evolve"],
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
              name: ["-k", "--print-package"],
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: ["-e", "--evolve"],
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
              name: ["-k", "--print-package"],
            },
            {
              name: ["-u", "--dump"],
            },
            {
              name: ["-i", "--pipe"],
            },
            {
              name: ["-A", "--all-strings"],
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--drop",
            },
            {
              name: ["-a", "--stats"],
            },
            {
              name: ["-e", "--evolve"],
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
          name: ["-k", "--print-package"],
        },
        {
          name: ["-u", "--dump"],
        },
        {
          name: ["-i", "--pipe"],
        },
        {
          name: ["-A", "--all-strings"],
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--drop",
        },
        {
          name: ["-a", "--stats"],
        },
        {
          name: ["-e", "--evolve"],
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
          name: ["-o", "--output"],
          isRepeatable: true,
          args: {
            name: "output",
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
          name: "--no-boolean",
        },
        {
          name: "--trim",
        },
        {
          name: "--memcheck",
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
              name: "--valid",
              isRepeatable: true,
              args: {
                name: "valid",
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
              name: "--timeout",
              isRepeatable: true,
              args: {
                name: "timeout",
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
              name: ["-d", "--delimiter"],
              isRepeatable: true,
              args: {
                name: "delimiter",
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
              name: ["-b", "--batch"],
              isRepeatable: true,
              args: {
                name: "batch",
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
              name: "--ckan-api",
              isRepeatable: true,
              args: {
                name: "ckan-api",
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
              name: "--ckan-token",
              isRepeatable: true,
              args: {
                name: "ckan-token",
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
              name: "--size-limit",
              isRepeatable: true,
              args: {
                name: "size-limit",
                isOptional: true,
              },
            },
            {
              name: ["-q", "--quiet"],
            },
            {
              name: "--json",
            },
            {
              name: "--trim",
            },
            {
              name: "--email-domain-literal",
            },
            {
              name: ["-n", "--no-headers"],
            },
            {
              name: ["-p", "--progressbar"],
            },
            {
              name: "--fail-fast",
            },
            {
              name: "--no-format-validation",
            },
            {
              name: "--pretty-json",
            },
            {
              name: "--email-required-tld",
            },
            {
              name: "--email-display-text",
            },
            {
              name: "--fancy-regex",
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
          name: "--valid",
          isRepeatable: true,
          args: {
            name: "valid",
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
          name: "--timeout",
          isRepeatable: true,
          args: {
            name: "timeout",
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
          name: ["-d", "--delimiter"],
          isRepeatable: true,
          args: {
            name: "delimiter",
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
          name: ["-b", "--batch"],
          isRepeatable: true,
          args: {
            name: "batch",
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
          name: "--ckan-api",
          isRepeatable: true,
          args: {
            name: "ckan-api",
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
          name: "--ckan-token",
          isRepeatable: true,
          args: {
            name: "ckan-token",
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
          name: "--size-limit",
          isRepeatable: true,
          args: {
            name: "size-limit",
            isOptional: true,
          },
        },
        {
          name: ["-q", "--quiet"],
        },
        {
          name: "--json",
        },
        {
          name: "--trim",
        },
        {
          name: "--email-domain-literal",
        },
        {
          name: ["-n", "--no-headers"],
        },
        {
          name: ["-p", "--progressbar"],
        },
        {
          name: "--fail-fast",
        },
        {
          name: "--no-format-validation",
        },
        {
          name: "--pretty-json",
        },
        {
          name: "--email-required-tld",
        },
        {
          name: "--email-display-text",
        },
        {
          name: "--fancy-regex",
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
