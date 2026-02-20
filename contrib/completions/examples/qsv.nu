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
    --formatstr(-f): string
    --new-column(-c): string
    --comparand(-C): string
    --jobs(-j): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --replacement(-R): string
    --rename(-r): string
    --batch(-b): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --formatstr(-f): string
    --new-column(-c): string
    --comparand(-C): string
    --jobs(-j): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --replacement(-R): string
    --rename(-r): string
    --batch(-b): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --formatstr(-f): string
    --new-column(-c): string
    --comparand(-C): string
    --jobs(-j): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --replacement(-R): string
    --rename(-r): string
    --batch(-b): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --formatstr(-f): string
    --new-column(-c): string
    --comparand(-C): string
    --jobs(-j): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --replacement(-R): string
    --rename(-r): string
    --batch(-b): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --formatstr(-f): string
    --new-column(-c): string
    --comparand(-C): string
    --jobs(-j): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --replacement(-R): string
    --rename(-r): string
    --batch(-b): string
    --delimiter(-d): string
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
    --output(-o): string
    --flexible(-f)
    --help(-h)                # Print help
  ]

  export extern "qsv cat" [
    --output(-o): string
    --no-headers(-n)
    --group-name(-N): string
    --group(-g): string
    --pad(-p)
    --flexible
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --output(-o): string
    --no-headers(-n)
    --group-name(-N): string
    --group(-g): string
    --pad(-p)
    --flexible
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --output(-o): string
    --no-headers(-n)
    --group-name(-N): string
    --group(-g): string
    --pad(-p)
    --flexible
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --output(-o): string
    --no-headers(-n)
    --group-name(-N): string
    --group(-g): string
    --pad(-p)
    --flexible
    --delimiter(-d): string
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
    --row-numbers(-n)
    --color(-C)
    --memcheck
    --output(-o): string
    --delimiter(-d): string
    --title(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --human-readable(-H)
    --width-no-delims
    --low-memory
    --width
    --flexible(-f)
    --no-headers(-n)
    --no-polars
    --json
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --no-headers(-n)
    --new-column(-c): string
    --input-tz: string
    --rename(-r): string
    --output-tz: string
    --formatstr: string
    --ts-resolution(-R): string
    --utc
    --batch(-b): string
    --zulu
    --output(-o): string
    --progressbar(-p)
    --delimiter(-d): string
    --prefer-dmy
    --default-tz: string
    --jobs(-j): string
    --keep-zero-time
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --select(-s): string
    --human-readable(-H)
    --delimiter(-d): string
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --dupes-output(-D): string
    --memcheck
    --no-headers(-n)
    --ignore-case(-i)
    --numeric(-N)
    --sorted
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --prompt-file: string
    --dictionary
    --sql-results: string
    --tags
    --ckan-token: string
    --max-tokens(-t): string
    --stats-options: string
    --num-tags: string
    --cache-dir: string
    --truncate-str: string
    --language: string
    --export-prompt: string
    --disk-cache-dir: string
    --redis-cache
    --description
    --base-url(-u): string
    --user-agent: string
    --fewshot-examples
    --fresh
    --session-len: string
    --no-cache
    --tag-vocab: string
    --enum-threshold: string
    --api-key(-k): string
    --all(-A)
    --session: string
    --timeout: string
    --model(-m): string
    --addl-cols-list: string
    --output(-o): string
    --flush-cache
    --addl-cols
    --freq-options: string
    --ckan-api: string
    --sample-size: string
    --prompt(-p): string
    --forget
    --quiet(-q)
    --format: string
    --addl-props: string
    --num-examples: string
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --delimiter-right: string
    --no-headers-output
    --delimiter-left: string
    --no-headers-right
    --drop-equal-fields
    --key(-k): string
    --sort-columns: string
    --jobs(-j): string
    --output(-o): string
    --delimiter(-d): string
    --no-headers-left
    --delimiter-output: string
    --help(-h)                # Print help
  ]

  export extern "qsv edit" [
    --output(-o): string
    --in-place(-i)
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv enum" [
    --new-column(-c): string
    --start: string
    --output(-o): string
    --uuid4
    --uuid7
    --no-headers(-n)
    --delimiter(-d): string
    --copy: string
    --hash: string
    --constant: string
    --increment: string
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --jobs(-j): string
    --header-row: string
    --output(-o): string
    --metadata: string
    --date-format: string
    --range: string
    --trim
    --sheet(-s): string
    --error-format: string
    --keep-zero-time
    --cell: string
    --quiet(-q)
    --flexible
    --table: string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --output(-o): string
    --delimiter(-d): string
    --ignore-case(-i)
    --invert(-v)
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv explode" [
    --delimiter(-d): string
    --rename(-r): string
    --output(-o): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv extdedup" [
    --human-readable(-H)
    --no-headers(-n)
    --dupes-output(-D): string
    --memory-limit: string
    --select(-s): string
    --delimiter(-d): string
    --no-output
    --temp-dir: string
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --delimiter(-d): string
    --memory-limit: string
    --select(-s): string
    --no-headers(-n)
    --reverse(-R)
    --jobs(-j): string
    --tmp-dir: string
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --url-template: string
    --user-agent: string
    --max-errors: string
    --http-header(-H): string
    --new-column(-c): string
    --disk-cache-dir: string
    --store-error
    --pretty
    --disk-cache
    --delimiter(-d): string
    --redis-cache
    --progressbar(-p)
    --cookies
    --flush-cache
    --no-headers(-n)
    --timeout: string
    --max-retries: string
    --no-cache
    --report: string
    --mem-cache-size: string
    --jaqfile: string
    --jaq: string
    --rate-limit: string
    --cache-error
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --content-type: string
    --disk-cache-dir: string
    --compress
    --progressbar(-p)
    --report: string
    --globals-json(-j): string
    --user-agent: string
    --no-headers(-n)
    --disk-cache
    --output(-o): string
    --payload-tpl(-t): string
    --jaqfile: string
    --timeout: string
    --http-header(-H): string
    --rate-limit: string
    --no-cache
    --store-error
    --redis-cache
    --delimiter(-d): string
    --max-retries: string
    --jaq: string
    --new-column(-c): string
    --flush-cache
    --max-errors: string
    --mem-cache-size: string
    --cookies
    --cache-error
    --pretty
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --delimiter(-d): string
    --default(-v): string
    --no-headers(-n)
    --first(-f)
    --backfill(-b)
    --groupby(-g): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --quiet(-q)
    --output(-o): string
    --remove-empty(-r)
    --insert(-i): string
    --quote: string
    --length(-l): string
    --escape: string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --delimiter(-d): string
    --condense(-c): string
    --separator(-s): string
    --no-headers(-n)
    --field-separator(-f): string
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --crlf
    --output(-o): string
    --escape: string
    --quote: string
    --ascii
    --quote-always
    --no-final-newline
    --delimiter(-d): string
    --quote-never
    --out-delimiter(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --delimiter(-d): string
    --unify(-u)
    --dry-run: string
    --new-column(-c): string
    --no-headers(-n)
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --other-sorted
    --no-stats
    --no-headers(-n)
    --delimiter(-d): string
    --no-nulls
    --memcheck
    --select(-s): string
    --asc(-a)
    --no-float: string
    --high-card-pct: string
    --stats-filter: string
    --output(-o): string
    --rank-strategy(-r): string
    --force
    --high-card-threshold: string
    --limit(-l): string
    --vis-whitespace
    --pct-dec-places: string
    --json
    --null-sorted
    --jobs(-j): string
    --pct-nulls
    --null-text: string
    --weight: string
    --no-other
    --frequency-jsonl
    --other-text: string
    --unq-limit(-u): string
    --lmt-threshold: string
    --ignore-case(-i)
    --all-unique-text: string
    --pretty-json
    --toon
    --no-trim
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfo" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfonow" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-check" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-load" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-reset" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-update" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookup" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookupnow" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reverse" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reversenow" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggest" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggestnow" [
    --cache-dir: string
    --formatstr(-f): string
    --k_weight(-k): string
    --new-column(-c): string
    --invalid-result: string
    --cities-url: string
    --delimiter(-d): string
    --timeout: string
    --language(-l): string
    --country: string
    --jobs(-j): string
    --output(-o): string
    --languages: string
    --min-score: string
    --progressbar(-p)
    --admin1: string
    --batch(-b): string
    --rename(-r): string
    --force
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
    --max-length(-l): string
    --output(-o): string
    --latitude(-y): string
    --geometry(-g): string
    --longitude(-x): string
    --help(-h)                # Print help
  ]

  export extern "qsv headers" [
    --intersect
    --trim
    --delimiter(-d): string
    --just-count(-J)
    --just-names(-j)
    --help(-h)                # Print help
  ]

  export extern "qsv index" [
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv input" [
    --quote-style: string
    --auto-skip
    --trim-headers
    --comment: string
    --quote: string
    --skip-lastlines: string
    --output(-o): string
    --escape: string
    --no-quoting
    --encoding-errors: string
    --skip-lines: string
    --delimiter(-d): string
    --trim-fields
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --full
    --nulls
    --output(-o): string
    --cross
    --keys-output: string
    --left-anti
    --right
    --ignore-case(-i)
    --right-semi
    --delimiter(-d): string
    --ignore-leading-zeros(-z)
    --no-headers(-n)
    --left
    --left-semi
    --right-anti
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --time-format: string
    --float-precision: string
    --norm-unicode(-N): string
    --datetime-format: string
    --maintain-order: string
    --right_by: string
    --coalesce
    --validate: string
    --left-anti
    --left_by: string
    --no-sort
    --tolerance: string
    --delimiter(-d): string
    --full
    --allow-exact-matches(-X)
    --infer-len: string
    --asof
    --sql-filter: string
    --ignore-leading-zeros(-z)
    --ignore-case(-i)
    --no-optimizations
    --quiet(-q)
    --low-memory
    --right-semi
    --cross
    --left
    --decimal-comma
    --right
    --nulls
    --streaming
    --try-parsedates
    --date-format: string
    --left-semi
    --null-value: string
    --right-anti
    --strategy: string
    --filter-right: string
    --output(-o): string
    --non-equi: string
    --filter-left: string
    --cache-schema: string
    --ignore-errors
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --output(-o): string
    --select(-s): string
    --jaq: string
    --help(-h)                # Print help
  ]

  export extern "qsv jsonl" [
    --delimiter(-d): string
    --ignore-errors
    --jobs(-j): string
    --batch(-b): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv lens" [
    --delimiter(-d): string
    --streaming-stdin(-S)
    --auto-reload(-A)
    --tab-separated(-t)
    --monochrome(-m)
    --find: string
    --freeze-columns(-f): string
    --prompt(-P): string
    --no-headers
    --ignore-case(-i)
    --columns: string
    --echo-column: string
    --debug
    --wrap-mode(-W): string
    --filter: string
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --remap(-r)
    --ckan-api: string
    --progressbar(-p)
    --cache-dir: string
    --max-errors: string
    --no-globals(-g)
    --timeout: string
    --delimiter(-d): string
    --colindex
    --no-headers(-n)
    --end(-E): string
    --ckan-token: string
    --begin(-B): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv luau filter" [
    --remap(-r)
    --ckan-api: string
    --progressbar(-p)
    --cache-dir: string
    --max-errors: string
    --no-globals(-g)
    --timeout: string
    --delimiter(-d): string
    --colindex
    --no-headers(-n)
    --end(-E): string
    --ckan-token: string
    --begin(-B): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv luau map" [
    --remap(-r)
    --ckan-api: string
    --progressbar(-p)
    --cache-dir: string
    --max-errors: string
    --no-globals(-g)
    --timeout: string
    --delimiter(-d): string
    --colindex
    --no-headers(-n)
    --end(-E): string
    --ckan-token: string
    --begin(-B): string
    --output(-o): string
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
    --pct-thresholds: string
    --cardinality-threshold(-C): string
    --advanced
    --join-inputs(-J): string
    --join-type(-T): string
    --join-keys(-K): string
    --stats-options: string
    --output(-o): string
    --use-percentiles
    --bivariate(-B)
    --bivariate-stats(-S): string
    --force
    --progressbar(-p)
    --epsilon(-e): string
    --jobs(-j): string
    --xsd-gdate-scan: string
    --round: string
    --help(-h)                # Print help
  ]

  export extern "qsv partition" [
    --prefix-length(-p): string
    --drop
    --no-headers(-n)
    --delimiter(-d): string
    --limit: string
    --filename: string
    --help(-h)                # Print help
  ]

  export extern "qsv pivotp" [
    --decimal-comma
    --col-separator: string
    --quiet(-q)
    --index(-i): string
    --sort-columns
    --maintain-order
    --delimiter(-d): string
    --validate
    --try-parsedates
    --output(-o): string
    --ignore-errors
    --values(-v): string
    --infer-len: string
    --agg(-a): string
    --help(-h)                # Print help
  ]

  export extern "qsv pragmastat" [
    --memcheck
    --twosample(-t)
    --delimiter(-d): string
    --no-headers(-n)
    --output(-o): string
    --select(-s): string
    --misrate(-m): string
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
    --fd-output(-f)
    --base-delay-ms: string
    --filters(-F): string
    --save-fname: string
    --msg(-m): string
    --output(-o): string
    --quiet(-q)
    --workdir(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --increment: string
    --formatstr: string
    --delimiter(-d): string
    --start: string
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --no-headers(-n)
    --output(-o): string
    --helper(-f): string
    --delimiter(-d): string
    --batch(-b): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv py filter" [
    --no-headers(-n)
    --output(-o): string
    --helper(-f): string
    --delimiter(-d): string
    --batch(-b): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv py map" [
    --no-headers(-n)
    --output(-o): string
    --helper(-f): string
    --delimiter(-d): string
    --batch(-b): string
    --progressbar(-p)
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
    --delimiter(-d): string
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --delimiter(-d): string
    --ignore-case(-i)
    --select(-s): string
    --not-one
    --output(-o): string
    --unicode(-u)
    --literal
    --jobs(-j): string
    --progressbar(-p)
    --exact
    --quiet(-q)
    --dfa-size-limit: string
    --no-headers(-n)
    --size-limit: string
    --help(-h)                # Print help
  ]

  export extern "qsv reverse" [
    --output(-o): string
    --delimiter(-d): string
    --memcheck
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv safenames" [
    --prefix: string
    --delimiter(-d): string
    --output(-o): string
    --mode: string
    --reserved: string
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --rng: string
    --stratified: string
    --seed: string
    --cluster: string
    --ts-start: string
    --timeseries: string
    --ts-adaptive: string
    --ts-input-tz: string
    --user-agent: string
    --ts-interval: string
    --ts-aggregate: string
    --timeout: string
    --weighted: string
    --force
    --no-headers(-n)
    --max-size: string
    --bernoulli
    --ts-prefer-dmy
    --output(-o): string
    --delimiter(-d): string
    --systematic: string
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --stdout
    --strict-dates
    --no-headers(-n)
    --strict-formats
    --memcheck
    --force
    --output(-o): string
    --pattern-columns: string
    --enum-threshold: string
    --ignore-case(-i)
    --polars
    --prefer-dmy
    --delimiter(-d): string
    --dates-whitelist: string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --not-one
    --literal
    --invert-match(-v)
    --quick(-Q)
    --size-limit: string
    --no-headers(-n)
    --progressbar(-p)
    --quiet(-q)
    --ignore-case(-i)
    --count(-c)
    --delimiter(-d): string
    --jobs(-j): string
    --dfa-size-limit: string
    --exact
    --unicode(-u)
    --output(-o): string
    --select(-s): string
    --json
    --flag(-f): string
    --preview-match: string
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --select(-s): string
    --flag(-f): string
    --unmatched-output: string
    --count(-c)
    --json(-j)
    --not-one
    --flag-matches-only
    --literal
    --progressbar(-p)
    --invert-match(-v)
    --size-limit: string
    --quick(-Q)
    --jobs: string
    --no-headers(-n)
    --delimiter(-d): string
    --ignore-case(-i)
    --dfa-size-limit: string
    --output(-o): string
    --quiet(-q)
    --exact
    --unicode(-u)
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --sort(-S)
    --random(-R)
    --delimiter(-d): string
    --seed: string
    --output(-o): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --json
    --invert
    --start(-s): string
    --len(-l): string
    --no-headers(-n)
    --index(-i): string
    --output(-o): string
    --end(-e): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --quiet(-q)
    --jobs(-j): string
    --timeout: string
    --output(-o): string
    --user-agent: string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --quiet(-q)
    --jobs(-j): string
    --timeout: string
    --output(-o): string
    --user-agent: string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --quiet(-q)
    --jobs(-j): string
    --timeout: string
    --output(-o): string
    --user-agent: string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --quiet(-q)
    --jobs(-j): string
    --timeout: string
    --output(-o): string
    --user-agent: string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --quiet(-q)
    --jobs(-j): string
    --timeout: string
    --output(-o): string
    --user-agent: string
    --progressbar(-p)
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
    --stats-types
    --delimiter(-d): string
    --quote: string
    --json
    --user-agent: string
    --prefer-dmy
    --just-mime
    --sample: string
    --progressbar(-p)
    --pretty-json
    --quick(-Q)
    --timeout: string
    --harvest-mode
    --save-urlsample: string
    --no-infer
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --rng: string
    --output(-o): string
    --seed: string
    --ignore-case(-i)
    --memcheck
    --numeric(-N)
    --faster
    --natural
    --jobs(-j): string
    --delimiter(-d): string
    --reverse(-R)
    --random
    --unique(-u)
    --select(-s): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --pretty-json
    --json
    --delimiter(-d): string
    --no-headers(-n)
    --progressbar(-p)
    --select(-s): string
    --ignore-case(-i)
    --all
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --filter: string
    --kb-size(-k): string
    --delimiter(-d): string
    --pad: string
    --quiet(-q)
    --filter-ignore-errors
    --size(-s): string
    --filter-cleanup
    --jobs(-j): string
    --filename: string
    --chunks(-c): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --float-precision: string
    --datetime-format: string
    --quiet(-q)
    --output(-o): string
    --compress-level: string
    --streaming
    --rnull-values: string
    --cache-schema
    --compression: string
    --no-optimizations
    --statistics
    --try-parsedates
    --infer-len: string
    --ignore-errors
    --decimal-comma
    --wnull-value: string
    --low-memory
    --truncate-ragged-lines
    --format: string
    --delimiter(-d): string
    --time-format: string
    --date-format: string
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --typesonly
    --no-headers(-n)
    --boolean-patterns: string
    --mode
    --cardinality
    --stats-jsonl
    --percentile-list: string
    --infer-dates
    --dates-whitelist: string
    --mad
    --quartiles
    --force
    --prefer-dmy
    --memcheck
    --delimiter(-d): string
    --everything(-E)
    --jobs(-j): string
    --cache-threshold(-c): string
    --median
    --weight: string
    --round: string
    --select(-s): string
    --vis-whitespace
    --percentiles
    --nulls
    --output(-o): string
    --infer-boolean
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --condense(-c): string
    --output(-o): string
    --width(-w): string
    --delimiter(-d): string
    --memcheck
    --pad(-p): string
    --align(-a): string
    --help(-h)                # Print help
  ]

  export extern "qsv template" [
    --progressbar(-p)
    --no-headers(-n)
    --timeout: string
    --customfilter-error: string
    --template: string
    --jobs(-j): string
    --outsubdir-size: string
    --cache-dir: string
    --ckan-api: string
    --outfilename: string
    --delimiter: string
    --template-file(-t): string
    --globals-json: string
    --ckan-token: string
    --batch(-b): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --stats(-a)
    --delimiter(-d): string
    --drop
    --quiet(-q)
    --schema(-s): string
    --print-package(-k)
    --jobs(-j): string
    --pipe(-i)
    --all-strings(-A)
    --evolve(-e)
    --stats-csv(-c): string
    --dump(-u)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to datapackage" [
    --stats(-a)
    --delimiter(-d): string
    --drop
    --quiet(-q)
    --schema(-s): string
    --print-package(-k)
    --jobs(-j): string
    --pipe(-i)
    --all-strings(-A)
    --evolve(-e)
    --stats-csv(-c): string
    --dump(-u)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to ods" [
    --stats(-a)
    --delimiter(-d): string
    --drop
    --quiet(-q)
    --schema(-s): string
    --print-package(-k)
    --jobs(-j): string
    --pipe(-i)
    --all-strings(-A)
    --evolve(-e)
    --stats-csv(-c): string
    --dump(-u)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to postgres" [
    --stats(-a)
    --delimiter(-d): string
    --drop
    --quiet(-q)
    --schema(-s): string
    --print-package(-k)
    --jobs(-j): string
    --pipe(-i)
    --all-strings(-A)
    --evolve(-e)
    --stats-csv(-c): string
    --dump(-u)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to sqlite" [
    --stats(-a)
    --delimiter(-d): string
    --drop
    --quiet(-q)
    --schema(-s): string
    --print-package(-k)
    --jobs(-j): string
    --pipe(-i)
    --all-strings(-A)
    --evolve(-e)
    --stats-csv(-c): string
    --dump(-u)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to xlsx" [
    --stats(-a)
    --delimiter(-d): string
    --drop
    --quiet(-q)
    --schema(-s): string
    --print-package(-k)
    --jobs(-j): string
    --pipe(-i)
    --all-strings(-A)
    --evolve(-e)
    --stats-csv(-c): string
    --dump(-u)
    --separator(-p): string
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
    --batch(-b): string
    --jobs(-j): string
    --trim
    --quiet(-q)
    --no-boolean
    --memcheck
    --delimiter(-d): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --long: string
    --output(-o): string
    --memcheck
    --select(-s): string
    --delimiter(-d): string
    --multipass(-m)
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --no-headers(-n)
    --invalid: string
    --no-format-validation
    --timeout: string
    --jobs(-j): string
    --email-min-subdomains: string
    --progressbar(-p)
    --quiet(-q)
    --dfa-size-limit: string
    --delimiter(-d): string
    --ckan-token: string
    --backtrack-limit: string
    --trim
    --pretty-json
    --batch(-b): string
    --cache-dir: string
    --fail-fast
    --ckan-api: string
    --size-limit: string
    --valid-output: string
    --email-domain-literal
    --email-display-text
    --email-required-tld
    --json
    --fancy-regex
    --valid: string
    --help(-h)                # Print help
  ]

  export extern "qsv validate schema" [
    --no-headers(-n)
    --invalid: string
    --no-format-validation
    --timeout: string
    --jobs(-j): string
    --email-min-subdomains: string
    --progressbar(-p)
    --quiet(-q)
    --dfa-size-limit: string
    --delimiter(-d): string
    --ckan-token: string
    --backtrack-limit: string
    --trim
    --pretty-json
    --batch(-b): string
    --cache-dir: string
    --fail-fast
    --ckan-api: string
    --size-limit: string
    --valid-output: string
    --email-domain-literal
    --email-display-text
    --email-required-tld
    --json
    --fancy-regex
    --valid: string
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
