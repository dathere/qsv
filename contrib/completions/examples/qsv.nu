module completions {

  export extern qsv [
    --list
    --envlist
    --update
    --updatenow
    --version(-V)
    --help(-h)                # Print help
  ]

  export extern "qsv apply" [
    --progressbar(-p)
    --new-column(-c): string
    --replacement(-R): string
    --formatstr(-f): string
    --batch(-b): string
    --delimiter(-d): string
    --jobs(-j): string
    --rename(-r): string
    --no-headers(-n)
    --comparand(-C): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --progressbar(-p)
    --new-column(-c): string
    --replacement(-R): string
    --formatstr(-f): string
    --batch(-b): string
    --delimiter(-d): string
    --jobs(-j): string
    --rename(-r): string
    --no-headers(-n)
    --comparand(-C): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --progressbar(-p)
    --new-column(-c): string
    --replacement(-R): string
    --formatstr(-f): string
    --batch(-b): string
    --delimiter(-d): string
    --jobs(-j): string
    --rename(-r): string
    --no-headers(-n)
    --comparand(-C): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --progressbar(-p)
    --new-column(-c): string
    --replacement(-R): string
    --formatstr(-f): string
    --batch(-b): string
    --delimiter(-d): string
    --jobs(-j): string
    --rename(-r): string
    --no-headers(-n)
    --comparand(-C): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --progressbar(-p)
    --new-column(-c): string
    --replacement(-R): string
    --formatstr(-f): string
    --batch(-b): string
    --delimiter(-d): string
    --jobs(-j): string
    --rename(-r): string
    --no-headers(-n)
    --comparand(-C): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv apply help" [
  ]

  export extern "qsv apply help calcconv" [
  ]

  export extern "qsv apply help dynfmt" [
  ]

  export extern "qsv apply help emptyreplace" [
  ]

  export extern "qsv apply help operations" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv apply help help" [
  ]

  export extern "qsv behead" [
    --flexible(-f)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat" [
    --group-name(-N): string
    --output(-o): string
    --no-headers(-n)
    --pad(-p)
    --delimiter(-d): string
    --flexible
    --group(-g): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --group-name(-N): string
    --output(-o): string
    --no-headers(-n)
    --pad(-p)
    --delimiter(-d): string
    --flexible
    --group(-g): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --group-name(-N): string
    --output(-o): string
    --no-headers(-n)
    --pad(-p)
    --delimiter(-d): string
    --flexible
    --group(-g): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --group-name(-N): string
    --output(-o): string
    --no-headers(-n)
    --pad(-p)
    --delimiter(-d): string
    --flexible
    --group(-g): string
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv cat help" [
  ]

  export extern "qsv cat help columns" [
  ]

  export extern "qsv cat help rows" [
  ]

  export extern "qsv cat help rowskey" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv cat help help" [
  ]

  export extern "qsv clipboard" [
    --save(-s)
    --help(-h)                # Print help
  ]

  export extern "qsv color" [
    --memcheck
    --title(-t): string
    --delimiter(-d): string
    --color(-C)
    --row-numbers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --delimiter(-d): string
    --flexible(-f)
    --json
    --no-polars
    --human-readable(-H)
    --width-no-delims
    --low-memory
    --width
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --formatstr: string
    --delimiter(-d): string
    --prefer-dmy
    --rename(-r): string
    --progressbar(-p)
    --keep-zero-time
    --default-tz: string
    --output-tz: string
    --input-tz: string
    --utc
    --ts-resolution(-R): string
    --jobs(-j): string
    --batch(-b): string
    --no-headers(-n)
    --zulu
    --output(-o): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --jobs(-j): string
    --ignore-case(-i)
    --delimiter(-d): string
    --human-readable(-H)
    --dupes-output(-D): string
    --memcheck
    --sorted
    --no-headers(-n)
    --quiet(-q)
    --numeric(-N)
    --select(-s): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --addl-cols-list: string
    --addl-props: string
    --addl-cols
    --ckan-api: string
    --fewshot-examples
    --timeout: string
    --export-prompt: string
    --session-len: string
    --redis-cache
    --format: string
    --stats-options: string
    --prompt(-p): string
    --model(-m): string
    --prompt-file: string
    --sample-size: string
    --tag-vocab: string
    --all(-A)
    --fresh
    --cache-dir: string
    --ckan-token: string
    --description
    --session: string
    --language: string
    --forget
    --dictionary
    --tags
    --no-cache
    --flush-cache
    --enum-threshold: string
    --base-url(-u): string
    --num-tags: string
    --freq-options: string
    --api-key(-k): string
    --max-tokens(-t): string
    --quiet(-q)
    --num-examples: string
    --truncate-str: string
    --sql-results: string
    --user-agent: string
    --disk-cache-dir: string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --delimiter-left: string
    --delimiter-output: string
    --key(-k): string
    --jobs(-j): string
    --sort-columns: string
    --drop-equal-fields
    --no-headers-right
    --no-headers-output
    --delimiter-right: string
    --no-headers-left
    --output(-o): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv edit" [
    --in-place(-i)
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv enum" [
    --constant: string
    --hash: string
    --delimiter(-d): string
    --no-headers(-n)
    --copy: string
    --new-column(-c): string
    --start: string
    --increment: string
    --uuid7
    --uuid4
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --metadata: string
    --date-format: string
    --output(-o): string
    --keep-zero-time
    --trim
    --flexible
    --header-row: string
    --sheet(-s): string
    --quiet(-q)
    --table: string
    --jobs(-j): string
    --range: string
    --cell: string
    --delimiter(-d): string
    --error-format: string
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --delimiter(-d): string
    --invert(-v)
    --output(-o): string
    --ignore-case(-i)
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv explode" [
    --rename(-r): string
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv extdedup" [
    --dupes-output(-D): string
    --no-headers(-n)
    --human-readable(-H)
    --no-output
    --memory-limit: string
    --quiet(-q)
    --select(-s): string
    --delimiter(-d): string
    --temp-dir: string
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --no-headers(-n)
    --reverse(-R)
    --delimiter(-d): string
    --select(-s): string
    --tmp-dir: string
    --jobs(-j): string
    --memory-limit: string
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --jaqfile: string
    --redis-cache
    --no-headers(-n)
    --new-column(-c): string
    --jaq: string
    --timeout: string
    --disk-cache
    --store-error
    --cookies
    --mem-cache-size: string
    --url-template: string
    --flush-cache
    --http-header(-H): string
    --max-retries: string
    --output(-o): string
    --report: string
    --disk-cache-dir: string
    --cache-error
    --rate-limit: string
    --user-agent: string
    --progressbar(-p)
    --no-cache
    --pretty
    --max-errors: string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --store-error
    --mem-cache-size: string
    --jaq: string
    --new-column(-c): string
    --cookies
    --disk-cache-dir: string
    --flush-cache
    --compress
    --max-errors: string
    --content-type: string
    --disk-cache
    --progressbar(-p)
    --no-headers(-n)
    --redis-cache
    --cache-error
    --globals-json(-j): string
    --timeout: string
    --rate-limit: string
    --output(-o): string
    --user-agent: string
    --pretty
    --delimiter(-d): string
    --report: string
    --http-header(-H): string
    --jaqfile: string
    --no-cache
    --max-retries: string
    --payload-tpl(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --first(-f)
    --output(-o): string
    --no-headers(-n)
    --default(-v): string
    --backfill(-b)
    --delimiter(-d): string
    --groupby(-g): string
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --insert(-i): string
    --quote: string
    --remove-empty(-r)
    --delimiter(-d): string
    --output(-o): string
    --escape: string
    --quiet(-q)
    --length(-l): string
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --field-separator(-f): string
    --no-headers(-n)
    --separator(-s): string
    --delimiter(-d): string
    --condense(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --out-delimiter(-t): string
    --output(-o): string
    --quote-never
    --quote: string
    --no-final-newline
    --quote-always
    --escape: string
    --delimiter(-d): string
    --crlf
    --ascii
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --dry-run: string
    --unify(-u)
    --new-column(-c): string
    --no-headers(-n)
    --delimiter(-d): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --force
    --no-headers(-n)
    --pct-nulls
    --no-float: string
    --output(-o): string
    --unq-limit(-u): string
    --select(-s): string
    --ignore-case(-i)
    --frequency-jsonl
    --other-text: string
    --toon
    --weight: string
    --delimiter(-d): string
    --json
    --vis-whitespace
    --high-card-threshold: string
    --null-sorted
    --all-unique-text: string
    --high-card-pct: string
    --no-nulls
    --pretty-json
    --null-text: string
    --no-trim
    --other-sorted
    --limit(-l): string
    --no-stats
    --memcheck
    --jobs(-j): string
    --lmt-threshold: string
    --no-other
    --asc(-a)
    --pct-dec-places: string
    --stats-filter: string
    --rank-strategy(-r): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfo" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfonow" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-check" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-load" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-reset" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-update" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookup" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookupnow" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reverse" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reversenow" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggest" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggestnow" [
    --cities-url: string
    --jobs(-j): string
    --delimiter(-d): string
    --language(-l): string
    --timeout: string
    --languages: string
    --formatstr(-f): string
    --batch(-b): string
    --cache-dir: string
    --output(-o): string
    --country: string
    --admin1: string
    --progressbar(-p)
    --min-score: string
    --k_weight(-k): string
    --force
    --rename(-r): string
    --new-column(-c): string
    --invalid-result: string
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv geocode help" [
  ]

  export extern "qsv geocode help countryinfo" [
  ]

  export extern "qsv geocode help countryinfonow" [
  ]

  export extern "qsv geocode help index-check" [
  ]

  export extern "qsv geocode help index-load" [
  ]

  export extern "qsv geocode help index-reset" [
  ]

  export extern "qsv geocode help index-update" [
  ]

  export extern "qsv geocode help iplookup" [
  ]

  export extern "qsv geocode help iplookupnow" [
  ]

  export extern "qsv geocode help reverse" [
  ]

  export extern "qsv geocode help reversenow" [
  ]

  export extern "qsv geocode help suggest" [
  ]

  export extern "qsv geocode help suggestnow" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv geocode help help" [
  ]

  export extern "qsv geoconvert" [
    --latitude(-y): string
    --longitude(-x): string
    --geometry(-g): string
    --output(-o): string
    --max-length(-l): string
    --help(-h)                # Print help
  ]

  export extern "qsv headers" [
    --trim
    --delimiter(-d): string
    --just-count(-J)
    --intersect
    --just-names(-j)
    --help(-h)                # Print help
  ]

  export extern "qsv index" [
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv input" [
    --skip-lastlines: string
    --trim-fields
    --encoding-errors: string
    --quote: string
    --skip-lines: string
    --escape: string
    --auto-skip
    --quote-style: string
    --delimiter(-d): string
    --comment: string
    --output(-o): string
    --trim-headers
    --no-quoting
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --left
    --ignore-case(-i)
    --ignore-leading-zeros(-z)
    --left-anti
    --full
    --nulls
    --left-semi
    --right-anti
    --output(-o): string
    --right-semi
    --keys-output: string
    --cross
    --right
    --no-headers(-n)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --maintain-order: string
    --norm-unicode(-N): string
    --right-semi
    --cache-schema: string
    --no-sort
    --filter-right: string
    --tolerance: string
    --non-equi: string
    --output(-o): string
    --filter-left: string
    --delimiter(-d): string
    --try-parsedates
    --low-memory
    --streaming
    --validate: string
    --no-optimizations
    --asof
    --allow-exact-matches(-X)
    --ignore-errors
    --date-format: string
    --null-value: string
    --sql-filter: string
    --coalesce
    --right
    --quiet(-q)
    --left-semi
    --datetime-format: string
    --infer-len: string
    --left
    --nulls
    --ignore-case(-i)
    --ignore-leading-zeros(-z)
    --strategy: string
    --cross
    --right_by: string
    --decimal-comma
    --left-anti
    --full
    --float-precision: string
    --right-anti
    --time-format: string
    --left_by: string
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --output(-o): string
    --select(-s): string
    --jaq: string
    --help(-h)                # Print help
  ]

  export extern "qsv jsonl" [
    --ignore-errors
    --delimiter(-d): string
    --batch(-b): string
    --jobs(-j): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv lens" [
    --no-headers
    --wrap-mode(-W): string
    --delimiter(-d): string
    --ignore-case(-i)
    --streaming-stdin(-S)
    --filter: string
    --monochrome(-m)
    --debug
    --find: string
    --echo-column: string
    --tab-separated(-t)
    --freeze-columns(-f): string
    --columns: string
    --auto-reload(-A)
    --prompt(-P): string
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --timeout: string
    --ckan-api: string
    --delimiter(-d): string
    --colindex
    --no-headers(-n)
    --cache-dir: string
    --ckan-token: string
    --max-errors: string
    --no-globals(-g)
    --progressbar(-p)
    --output(-o): string
    --end(-E): string
    --begin(-B): string
    --remap(-r)
    --help(-h)                # Print help
  ]

  export extern "qsv luau filter" [
    --timeout: string
    --ckan-api: string
    --delimiter(-d): string
    --colindex
    --no-headers(-n)
    --cache-dir: string
    --ckan-token: string
    --max-errors: string
    --no-globals(-g)
    --progressbar(-p)
    --output(-o): string
    --end(-E): string
    --begin(-B): string
    --remap(-r)
    --help(-h)                # Print help
  ]

  export extern "qsv luau map" [
    --timeout: string
    --ckan-api: string
    --delimiter(-d): string
    --colindex
    --no-headers(-n)
    --cache-dir: string
    --ckan-token: string
    --max-errors: string
    --no-globals(-g)
    --progressbar(-p)
    --output(-o): string
    --end(-E): string
    --begin(-B): string
    --remap(-r)
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv luau help" [
  ]

  export extern "qsv luau help filter" [
  ]

  export extern "qsv luau help map" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv luau help help" [
  ]

  export extern "qsv moarstats" [
    --epsilon(-e): string
    --use-percentiles
    --round: string
    --pct-thresholds: string
    --join-keys(-K): string
    --bivariate(-B)
    --stats-options: string
    --cardinality-threshold(-C): string
    --join-inputs(-J): string
    --force
    --xsd-gdate-scan: string
    --output(-o): string
    --advanced
    --join-type(-T): string
    --bivariate-stats(-S): string
    --progressbar(-p)
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv partition" [
    --no-headers(-n)
    --delimiter(-d): string
    --limit: string
    --drop
    --filename: string
    --prefix-length(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv pivotp" [
    --agg(-a): string
    --infer-len: string
    --sort-columns
    --output(-o): string
    --index(-i): string
    --maintain-order
    --quiet(-q)
    --validate
    --ignore-errors
    --col-separator: string
    --delimiter(-d): string
    --values(-v): string
    --try-parsedates
    --decimal-comma
    --help(-h)                # Print help
  ]

  export extern "qsv pragmastat" [
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --twosample(-t)
    --memcheck
    --misrate(-m): string
    --select(-s): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv pro" [
    --help(-h)                # Print help
  ]

  export extern "qsv pro lens" [
    --help(-h)                # Print help
  ]

  export extern "qsv pro workflow" [
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv pro help" [
  ]

  export extern "qsv pro help lens" [
  ]

  export extern "qsv pro help workflow" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv pro help help" [
  ]

  export extern "qsv prompt" [
    --filters(-F): string
    --save-fname: string
    --output(-o): string
    --quiet(-q)
    --fd-output(-f)
    --base-delay-ms: string
    --msg(-m): string
    --workdir(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --output(-o): string
    --formatstr: string
    --delimiter(-d): string
    --no-headers(-n)
    --start: string
    --increment: string
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --progressbar(-p)
    --batch(-b): string
    --helper(-f): string
    --help(-h)                # Print help
  ]

  export extern "qsv py filter" [
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --progressbar(-p)
    --batch(-b): string
    --helper(-f): string
    --help(-h)                # Print help
  ]

  export extern "qsv py map" [
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --progressbar(-p)
    --batch(-b): string
    --helper(-f): string
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv py help" [
  ]

  export extern "qsv py help filter" [
  ]

  export extern "qsv py help map" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv py help help" [
  ]

  export extern "qsv rename" [
    --pairwise
    --no-headers(-n)
    --output(-o): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --ignore-case(-i)
    --dfa-size-limit: string
    --exact
    --literal
    --select(-s): string
    --no-headers(-n)
    --delimiter(-d): string
    --not-one
    --unicode(-u)
    --progressbar(-p)
    --quiet(-q)
    --jobs(-j): string
    --output(-o): string
    --size-limit: string
    --help(-h)                # Print help
  ]

  export extern "qsv reverse" [
    --delimiter(-d): string
    --memcheck
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv safenames" [
    --reserved: string
    --output(-o): string
    --prefix: string
    --mode: string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --timeseries: string
    --stratified: string
    --rng: string
    --systematic: string
    --ts-start: string
    --weighted: string
    --timeout: string
    --user-agent: string
    --ts-adaptive: string
    --max-size: string
    --ts-aggregate: string
    --ts-input-tz: string
    --bernoulli
    --ts-interval: string
    --ts-prefer-dmy
    --output(-o): string
    --seed: string
    --cluster: string
    --delimiter(-d): string
    --no-headers(-n)
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --jobs(-j): string
    --output(-o): string
    --force
    --pattern-columns: string
    --ignore-case(-i)
    --stdout
    --no-headers(-n)
    --enum-threshold: string
    --dates-whitelist: string
    --delimiter(-d): string
    --strict-dates
    --strict-formats
    --polars
    --prefer-dmy
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --quiet(-q)
    --preview-match: string
    --count(-c)
    --select(-s): string
    --output(-o): string
    --progressbar(-p)
    --literal
    --exact
    --json
    --not-one
    --dfa-size-limit: string
    --jobs(-j): string
    --no-headers(-n)
    --quick(-Q)
    --delimiter(-d): string
    --flag(-f): string
    --invert-match(-v)
    --unicode(-u)
    --ignore-case(-i)
    --size-limit: string
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --jobs: string
    --count(-c)
    --literal
    --not-one
    --invert-match(-v)
    --exact
    --unmatched-output: string
    --ignore-case(-i)
    --quick(-Q)
    --flag-matches-only
    --output(-o): string
    --delimiter(-d): string
    --select(-s): string
    --flag(-f): string
    --quiet(-q)
    --unicode(-u)
    --size-limit: string
    --json(-j)
    --dfa-size-limit: string
    --no-headers(-n)
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --output(-o): string
    --sort(-S)
    --seed: string
    --no-headers(-n)
    --delimiter(-d): string
    --random(-R)
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --len(-l): string
    --json
    --end(-e): string
    --invert
    --output(-o): string
    --index(-i): string
    --no-headers(-n)
    --delimiter(-d): string
    --start(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --progressbar(-p)
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --progressbar(-p)
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --progressbar(-p)
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --progressbar(-p)
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --progressbar(-p)
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv snappy help" [
  ]

  export extern "qsv snappy help check" [
  ]

  export extern "qsv snappy help compress" [
  ]

  export extern "qsv snappy help decompress" [
  ]

  export extern "qsv snappy help validate" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv snappy help help" [
  ]

  export extern "qsv sniff" [
    --save-urlsample: string
    --prefer-dmy
    --harvest-mode
    --quote: string
    --stats-types
    --user-agent: string
    --json
    --quick(-Q)
    --delimiter(-d): string
    --sample: string
    --no-infer
    --pretty-json
    --timeout: string
    --progressbar(-p)
    --just-mime
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --delimiter(-d): string
    --jobs(-j): string
    --output(-o): string
    --faster
    --unique(-u)
    --random
    --reverse(-R)
    --ignore-case(-i)
    --natural
    --numeric(-N)
    --seed: string
    --select(-s): string
    --no-headers(-n)
    --memcheck
    --rng: string
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --delimiter(-d): string
    --pretty-json
    --select(-s): string
    --json
    --progressbar(-p)
    --ignore-case(-i)
    --all
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --filter: string
    --jobs(-j): string
    --delimiter(-d): string
    --filename: string
    --kb-size(-k): string
    --chunks(-c): string
    --quiet(-q)
    --filter-cleanup
    --pad: string
    --filter-ignore-errors
    --size(-s): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --compress-level: string
    --infer-len: string
    --format: string
    --try-parsedates
    --float-precision: string
    --delimiter(-d): string
    --quiet(-q)
    --no-optimizations
    --decimal-comma
    --streaming
    --compression: string
    --date-format: string
    --truncate-ragged-lines
    --ignore-errors
    --time-format: string
    --statistics
    --datetime-format: string
    --wnull-value: string
    --rnull-values: string
    --cache-schema
    --low-memory
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --cardinality
    --memcheck
    --typesonly
    --infer-dates
    --weight: string
    --force
    --vis-whitespace
    --dates-whitelist: string
    --delimiter(-d): string
    --percentiles
    --select(-s): string
    --quartiles
    --output(-o): string
    --cache-threshold(-c): string
    --jobs(-j): string
    --mode
    --percentile-list: string
    --boolean-patterns: string
    --median
    --stats-jsonl
    --prefer-dmy
    --mad
    --nulls
    --round: string
    --no-headers(-n)
    --infer-boolean
    --everything(-E)
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --memcheck
    --condense(-c): string
    --pad(-p): string
    --width(-w): string
    --output(-o): string
    --align(-a): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv template" [
    --delimiter: string
    --template: string
    --progressbar(-p)
    --no-headers(-n)
    --template-file(-t): string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --outsubdir-size: string
    --output(-o): string
    --outfilename: string
    --globals-json: string
    --batch(-b): string
    --ckan-api: string
    --ckan-token: string
    --customfilter-error: string
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --evolve(-e)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --dump(-u)
    --jobs(-j): string
    --all-strings(-A)
    --delimiter(-d): string
    --print-package(-k)
    --drop
    --pipe(-i)
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv to datapackage" [
    --evolve(-e)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --dump(-u)
    --jobs(-j): string
    --all-strings(-A)
    --delimiter(-d): string
    --print-package(-k)
    --drop
    --pipe(-i)
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv to ods" [
    --evolve(-e)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --dump(-u)
    --jobs(-j): string
    --all-strings(-A)
    --delimiter(-d): string
    --print-package(-k)
    --drop
    --pipe(-i)
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv to postgres" [
    --evolve(-e)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --dump(-u)
    --jobs(-j): string
    --all-strings(-A)
    --delimiter(-d): string
    --print-package(-k)
    --drop
    --pipe(-i)
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv to sqlite" [
    --evolve(-e)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --dump(-u)
    --jobs(-j): string
    --all-strings(-A)
    --delimiter(-d): string
    --print-package(-k)
    --drop
    --pipe(-i)
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv to xlsx" [
    --evolve(-e)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --dump(-u)
    --jobs(-j): string
    --all-strings(-A)
    --delimiter(-d): string
    --print-package(-k)
    --drop
    --pipe(-i)
    --quiet(-q)
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv to help" [
  ]

  export extern "qsv to help datapackage" [
  ]

  export extern "qsv to help ods" [
  ]

  export extern "qsv to help postgres" [
  ]

  export extern "qsv to help sqlite" [
  ]

  export extern "qsv to help xlsx" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv to help help" [
  ]

  export extern "qsv tojsonl" [
    --memcheck
    --output(-o): string
    --trim
    --delimiter(-d): string
    --no-boolean
    --batch(-b): string
    --jobs(-j): string
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --select(-s): string
    --delimiter(-d): string
    --long: string
    --multipass(-m)
    --memcheck
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --invalid: string
    --trim
    --pretty-json
    --backtrack-limit: string
    --delimiter(-d): string
    --email-domain-literal
    --no-headers(-n)
    --size-limit: string
    --jobs(-j): string
    --dfa-size-limit: string
    --email-display-text
    --fail-fast
    --json
    --timeout: string
    --ckan-api: string
    --quiet(-q)
    --fancy-regex
    --valid: string
    --progressbar(-p)
    --valid-output: string
    --batch(-b): string
    --email-required-tld
    --no-format-validation
    --cache-dir: string
    --ckan-token: string
    --email-min-subdomains: string
    --help(-h)                # Print help
  ]

  export extern "qsv validate schema" [
    --invalid: string
    --trim
    --pretty-json
    --backtrack-limit: string
    --delimiter(-d): string
    --email-domain-literal
    --no-headers(-n)
    --size-limit: string
    --jobs(-j): string
    --dfa-size-limit: string
    --email-display-text
    --fail-fast
    --json
    --timeout: string
    --ckan-api: string
    --quiet(-q)
    --fancy-regex
    --valid: string
    --progressbar(-p)
    --valid-output: string
    --batch(-b): string
    --email-required-tld
    --no-format-validation
    --cache-dir: string
    --ckan-token: string
    --email-min-subdomains: string
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv validate help" [
  ]

  export extern "qsv validate help schema" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv validate help help" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv help" [
  ]

  export extern "qsv help apply" [
  ]

  export extern "qsv help apply calcconv" [
  ]

  export extern "qsv help apply dynfmt" [
  ]

  export extern "qsv help apply emptyreplace" [
  ]

  export extern "qsv help apply operations" [
  ]

  export extern "qsv help behead" [
  ]

  export extern "qsv help cat" [
  ]

  export extern "qsv help cat columns" [
  ]

  export extern "qsv help cat rows" [
  ]

  export extern "qsv help cat rowskey" [
  ]

  export extern "qsv help clipboard" [
  ]

  export extern "qsv help color" [
  ]

  export extern "qsv help count" [
  ]

  export extern "qsv help datefmt" [
  ]

  export extern "qsv help dedup" [
  ]

  export extern "qsv help describegpt" [
  ]

  export extern "qsv help diff" [
  ]

  export extern "qsv help edit" [
  ]

  export extern "qsv help enum" [
  ]

  export extern "qsv help excel" [
  ]

  export extern "qsv help exclude" [
  ]

  export extern "qsv help explode" [
  ]

  export extern "qsv help extdedup" [
  ]

  export extern "qsv help extsort" [
  ]

  export extern "qsv help fetch" [
  ]

  export extern "qsv help fetchpost" [
  ]

  export extern "qsv help fill" [
  ]

  export extern "qsv help fixlengths" [
  ]

  export extern "qsv help flatten" [
  ]

  export extern "qsv help fmt" [
  ]

  export extern "qsv help foreach" [
  ]

  export extern "qsv help frequency" [
  ]

  export extern "qsv help geocode" [
  ]

  export extern "qsv help geocode countryinfo" [
  ]

  export extern "qsv help geocode countryinfonow" [
  ]

  export extern "qsv help geocode index-check" [
  ]

  export extern "qsv help geocode index-load" [
  ]

  export extern "qsv help geocode index-reset" [
  ]

  export extern "qsv help geocode index-update" [
  ]

  export extern "qsv help geocode iplookup" [
  ]

  export extern "qsv help geocode iplookupnow" [
  ]

  export extern "qsv help geocode reverse" [
  ]

  export extern "qsv help geocode reversenow" [
  ]

  export extern "qsv help geocode suggest" [
  ]

  export extern "qsv help geocode suggestnow" [
  ]

  export extern "qsv help geoconvert" [
  ]

  export extern "qsv help headers" [
  ]

  export extern "qsv help index" [
  ]

  export extern "qsv help input" [
  ]

  export extern "qsv help join" [
  ]

  export extern "qsv help joinp" [
  ]

  export extern "qsv help json" [
  ]

  export extern "qsv help jsonl" [
  ]

  export extern "qsv help lens" [
  ]

  export extern "qsv help luau" [
  ]

  export extern "qsv help luau filter" [
  ]

  export extern "qsv help luau map" [
  ]

  export extern "qsv help moarstats" [
  ]

  export extern "qsv help partition" [
  ]

  export extern "qsv help pivotp" [
  ]

  export extern "qsv help pragmastat" [
  ]

  export extern "qsv help pro" [
  ]

  export extern "qsv help pro lens" [
  ]

  export extern "qsv help pro workflow" [
  ]

  export extern "qsv help prompt" [
  ]

  export extern "qsv help pseudo" [
  ]

  export extern "qsv help py" [
  ]

  export extern "qsv help py filter" [
  ]

  export extern "qsv help py map" [
  ]

  export extern "qsv help rename" [
  ]

  export extern "qsv help replace" [
  ]

  export extern "qsv help reverse" [
  ]

  export extern "qsv help safenames" [
  ]

  export extern "qsv help sample" [
  ]

  export extern "qsv help schema" [
  ]

  export extern "qsv help search" [
  ]

  export extern "qsv help searchset" [
  ]

  export extern "qsv help select" [
  ]

  export extern "qsv help slice" [
  ]

  export extern "qsv help snappy" [
  ]

  export extern "qsv help snappy check" [
  ]

  export extern "qsv help snappy compress" [
  ]

  export extern "qsv help snappy decompress" [
  ]

  export extern "qsv help snappy validate" [
  ]

  export extern "qsv help sniff" [
  ]

  export extern "qsv help sort" [
  ]

  export extern "qsv help sortcheck" [
  ]

  export extern "qsv help split" [
  ]

  export extern "qsv help sqlp" [
  ]

  export extern "qsv help stats" [
  ]

  export extern "qsv help table" [
  ]

  export extern "qsv help template" [
  ]

  export extern "qsv help to" [
  ]

  export extern "qsv help to datapackage" [
  ]

  export extern "qsv help to ods" [
  ]

  export extern "qsv help to postgres" [
  ]

  export extern "qsv help to sqlite" [
  ]

  export extern "qsv help to xlsx" [
  ]

  export extern "qsv help tojsonl" [
  ]

  export extern "qsv help transpose" [
  ]

  export extern "qsv help validate" [
  ]

  export extern "qsv help validate schema" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv help help" [
  ]

}

export use completions *
