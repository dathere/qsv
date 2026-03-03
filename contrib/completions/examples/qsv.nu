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
    --comparand(-C): string
    --output(-o): string
    --replacement(-R): string
    --jobs(-j): string
    --progressbar(-p)
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --rename(-r): string
    --no-headers(-n)
    --batch(-b): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --comparand(-C): string
    --output(-o): string
    --replacement(-R): string
    --jobs(-j): string
    --progressbar(-p)
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --rename(-r): string
    --no-headers(-n)
    --batch(-b): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --comparand(-C): string
    --output(-o): string
    --replacement(-R): string
    --jobs(-j): string
    --progressbar(-p)
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --rename(-r): string
    --no-headers(-n)
    --batch(-b): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --comparand(-C): string
    --output(-o): string
    --replacement(-R): string
    --jobs(-j): string
    --progressbar(-p)
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --rename(-r): string
    --no-headers(-n)
    --batch(-b): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --comparand(-C): string
    --output(-o): string
    --replacement(-R): string
    --jobs(-j): string
    --progressbar(-p)
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --rename(-r): string
    --no-headers(-n)
    --batch(-b): string
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
    --group(-g): string
    --flexible
    --delimiter(-d): string
    --group-name(-N): string
    --output(-o): string
    --pad(-p)
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --group(-g): string
    --flexible
    --delimiter(-d): string
    --group-name(-N): string
    --output(-o): string
    --pad(-p)
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --group(-g): string
    --flexible
    --delimiter(-d): string
    --group-name(-N): string
    --output(-o): string
    --pad(-p)
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --group(-g): string
    --flexible
    --delimiter(-d): string
    --group-name(-N): string
    --output(-o): string
    --pad(-p)
    --no-headers(-n)
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
    --color(-C)
    --memcheck
    --row-numbers(-n)
    --title(-t): string
    --delimiter(-d): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --no-headers(-n)
    --json
    --width-no-delims
    --low-memory
    --width
    --no-polars
    --delimiter(-d): string
    --human-readable(-H)
    --flexible(-f)
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --zulu
    --rename(-r): string
    --formatstr: string
    --new-column(-c): string
    --prefer-dmy
    --jobs(-j): string
    --output(-o): string
    --input-tz: string
    --progressbar(-p)
    --utc
    --output-tz: string
    --batch(-b): string
    --no-headers(-n)
    --keep-zero-time
    --default-tz: string
    --delimiter(-d): string
    --ts-resolution(-R): string
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --numeric(-N)
    --quiet(-q)
    --sorted
    --dupes-output(-D): string
    --human-readable(-H)
    --no-headers(-n)
    --memcheck
    --ignore-case(-i)
    --output(-o): string
    --delimiter(-d): string
    --select(-s): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --redis-cache
    --truncate-str: string
    --prompt-file: string
    --forget
    --quiet(-q)
    --tags
    --ckan-api: string
    --max-tokens(-t): string
    --output(-o): string
    --description
    --fewshot-examples
    --ckan-token: string
    --format: string
    --disk-cache-dir: string
    --api-key(-k): string
    --export-prompt: string
    --timeout: string
    --user-agent: string
    --stats-options: string
    --all(-A)
    --prompt(-p): string
    --num-examples: string
    --session-len: string
    --freq-options: string
    --tag-vocab: string
    --model(-m): string
    --no-cache
    --fresh
    --addl-props: string
    --sample-size: string
    --language: string
    --dictionary
    --cache-dir: string
    --flush-cache
    --session: string
    --addl-cols-list: string
    --sql-results: string
    --addl-cols
    --num-tags: string
    --enum-threshold: string
    --base-url(-u): string
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --delimiter-right: string
    --delimiter-left: string
    --jobs(-j): string
    --output(-o): string
    --delimiter(-d): string
    --no-headers-right
    --sort-columns: string
    --key(-k): string
    --drop-equal-fields
    --no-headers-left
    --delimiter-output: string
    --no-headers-output
    --help(-h)                # Print help
  ]

  export extern "qsv edit" [
    --in-place(-i)
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv enum" [
    --no-headers(-n)
    --delimiter(-d): string
    --hash: string
    --new-column(-c): string
    --uuid4
    --uuid7
    --increment: string
    --constant: string
    --copy: string
    --start: string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --table: string
    --delimiter(-d): string
    --flexible
    --quiet(-q)
    --metadata: string
    --sheet(-s): string
    --cell: string
    --keep-zero-time
    --range: string
    --error-format: string
    --output(-o): string
    --header-row: string
    --date-format: string
    --jobs(-j): string
    --trim
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --ignore-case(-i)
    --invert(-v)
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv explode" [
    --delimiter(-d): string
    --rename(-r): string
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv extdedup" [
    --quiet(-q)
    --dupes-output(-D): string
    --memory-limit: string
    --temp-dir: string
    --delimiter(-d): string
    --select(-s): string
    --no-headers(-n)
    --no-output
    --human-readable(-H)
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --no-headers(-n)
    --reverse(-R)
    --delimiter(-d): string
    --select(-s): string
    --memory-limit: string
    --tmp-dir: string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --delimiter(-d): string
    --store-error
    --timeout: string
    --flush-cache
    --jaq: string
    --user-agent: string
    --cache-error
    --redis-cache
    --no-cache
    --jaqfile: string
    --output(-o): string
    --http-header(-H): string
    --max-retries: string
    --report: string
    --mem-cache-size: string
    --url-template: string
    --disk-cache-dir: string
    --progressbar(-p)
    --new-column(-c): string
    --max-errors: string
    --cookies
    --pretty
    --disk-cache
    --rate-limit: string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --no-headers(-n)
    --globals-json(-j): string
    --jaq: string
    --no-cache
    --output(-o): string
    --jaqfile: string
    --max-retries: string
    --new-column(-c): string
    --pretty
    --cookies
    --compress
    --progressbar(-p)
    --cache-error
    --delimiter(-d): string
    --http-header(-H): string
    --rate-limit: string
    --payload-tpl(-t): string
    --max-errors: string
    --report: string
    --mem-cache-size: string
    --disk-cache-dir: string
    --store-error
    --redis-cache
    --content-type: string
    --timeout: string
    --user-agent: string
    --disk-cache
    --flush-cache
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --backfill(-b)
    --default(-v): string
    --first(-f)
    --delimiter(-d): string
    --groupby(-g): string
    --output(-o): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --remove-empty(-r)
    --length(-l): string
    --escape: string
    --quote: string
    --delimiter(-d): string
    --insert(-i): string
    --output(-o): string
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --separator(-s): string
    --condense(-c): string
    --delimiter(-d): string
    --no-headers(-n)
    --field-separator(-f): string
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --no-final-newline
    --quote-always
    --delimiter(-d): string
    --quote-never
    --output(-o): string
    --out-delimiter(-t): string
    --quote: string
    --ascii
    --crlf
    --escape: string
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --progressbar(-p)
    --no-headers(-n)
    --delimiter(-d): string
    --unify(-u)
    --dry-run: string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --stats-filter: string
    --no-stats
    --frequency-jsonl
    --pct-nulls
    --weight: string
    --select(-s): string
    --pretty-json
    --no-headers(-n)
    --ignore-case(-i)
    --other-sorted
    --output(-o): string
    --rank-strategy(-r): string
    --null-text: string
    --toon
    --high-card-pct: string
    --high-card-threshold: string
    --limit(-l): string
    --no-trim
    --jobs(-j): string
    --unq-limit(-u): string
    --asc(-a)
    --all-unique-text: string
    --no-other
    --vis-whitespace
    --no-nulls
    --null-sorted
    --memcheck
    --force
    --delimiter(-d): string
    --other-text: string
    --pct-dec-places: string
    --json
    --no-float: string
    --lmt-threshold: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfo" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfonow" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-check" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-load" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-reset" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-update" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookup" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookupnow" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reverse" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reversenow" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggest" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggestnow" [
    --country: string
    --min-score: string
    --language(-l): string
    --languages: string
    --progressbar(-p)
    --cache-dir: string
    --timeout: string
    --admin1: string
    --formatstr(-f): string
    --jobs(-j): string
    --batch(-b): string
    --cities-url: string
    --delimiter(-d): string
    --invalid-result: string
    --new-column(-c): string
    --rename(-r): string
    --k_weight(-k): string
    --force
    --output(-o): string
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
    --longitude(-x): string
    --output(-o): string
    --max-length(-l): string
    --latitude(-y): string
    --geometry(-g): string
    --help(-h)                # Print help
  ]

  export extern "qsv headers" [
    --delimiter(-d): string
    --trim
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
    --auto-skip
    --encoding-errors: string
    --quote-style: string
    --output(-o): string
    --trim-fields
    --no-quoting
    --skip-lines: string
    --delimiter(-d): string
    --comment: string
    --escape: string
    --quote: string
    --skip-lastlines: string
    --trim-headers
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --cross
    --left
    --keys-output: string
    --full
    --output(-o): string
    --left-semi
    --no-headers(-n)
    --ignore-case(-i)
    --nulls
    --left-anti
    --right
    --right-semi
    --right-anti
    --delimiter(-d): string
    --ignore-leading-zeros(-z)
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --right-semi
    --try-parsedates
    --low-memory
    --maintain-order: string
    --left-anti
    --output(-o): string
    --infer-len: string
    --strategy: string
    --left-semi
    --filter-left: string
    --filter-right: string
    --allow-exact-matches(-X)
    --quiet(-q)
    --right_by: string
    --null-value: string
    --full
    --left_by: string
    --no-sort
    --date-format: string
    --no-optimizations
    --float-precision: string
    --datetime-format: string
    --ignore-errors
    --norm-unicode(-N): string
    --cross
    --ignore-leading-zeros(-z)
    --time-format: string
    --ignore-case(-i)
    --nulls
    --streaming
    --coalesce
    --decimal-comma
    --sql-filter: string
    --tolerance: string
    --asof
    --delimiter(-d): string
    --right-anti
    --validate: string
    --right
    --non-equi: string
    --cache-schema: string
    --left
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --select(-s): string
    --jaq: string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv jsonl" [
    --output(-o): string
    --ignore-errors
    --jobs(-j): string
    --batch(-b): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv lens" [
    --auto-reload(-A)
    --debug
    --filter: string
    --no-headers
    --delimiter(-d): string
    --ignore-case(-i)
    --monochrome(-m)
    --find: string
    --streaming-stdin(-S)
    --prompt(-P): string
    --tab-separated(-t)
    --columns: string
    --echo-column: string
    --wrap-mode(-W): string
    --freeze-columns(-f): string
    --help(-h)                # Print help
  ]

  export extern "qsv log" [
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --max-errors: string
    --output(-o): string
    --begin(-B): string
    --ckan-token: string
    --delimiter(-d): string
    --remap(-r)
    --cache-dir: string
    --no-headers(-n)
    --timeout: string
    --progressbar(-p)
    --colindex
    --end(-E): string
    --ckan-api: string
    --no-globals(-g)
    --help(-h)                # Print help
  ]

  export extern "qsv luau filter" [
    --max-errors: string
    --output(-o): string
    --begin(-B): string
    --ckan-token: string
    --delimiter(-d): string
    --remap(-r)
    --cache-dir: string
    --no-headers(-n)
    --timeout: string
    --progressbar(-p)
    --colindex
    --end(-E): string
    --ckan-api: string
    --no-globals(-g)
    --help(-h)                # Print help
  ]

  export extern "qsv luau map" [
    --max-errors: string
    --output(-o): string
    --begin(-B): string
    --ckan-token: string
    --delimiter(-d): string
    --remap(-r)
    --cache-dir: string
    --no-headers(-n)
    --timeout: string
    --progressbar(-p)
    --colindex
    --end(-E): string
    --ckan-api: string
    --no-globals(-g)
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
    --jobs(-j): string
    --bivariate(-B)
    --join-keys(-K): string
    --cardinality-threshold(-C): string
    --force
    --stats-options: string
    --join-inputs(-J): string
    --bivariate-stats(-S): string
    --advanced
    --pct-thresholds: string
    --output(-o): string
    --round: string
    --progressbar(-p)
    --xsd-gdate-scan: string
    --use-percentiles
    --epsilon(-e): string
    --join-type(-T): string
    --help(-h)                # Print help
  ]

  export extern "qsv partition" [
    --drop
    --delimiter(-d): string
    --filename: string
    --no-headers(-n)
    --limit: string
    --prefix-length(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv pivotp" [
    --validate
    --infer-len: string
    --ignore-errors
    --maintain-order
    --index(-i): string
    --decimal-comma
    --sort-columns
    --values(-v): string
    --col-separator: string
    --delimiter(-d): string
    --quiet(-q)
    --try-parsedates
    --output(-o): string
    --agg(-a): string
    --help(-h)                # Print help
  ]

  export extern "qsv pragmastat" [
    --memcheck
    --delimiter(-d): string
    --jobs(-j): string
    --output(-o): string
    --twosample(-t)
    --select(-s): string
    --misrate(-m): string
    --no-headers(-n)
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
    --msg(-m): string
    --workdir(-d): string
    --filters(-F): string
    --save-fname: string
    --fd-output(-f)
    --quiet(-q)
    --base-delay-ms: string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --increment: string
    --formatstr: string
    --no-headers(-n)
    --delimiter(-d): string
    --start: string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --output(-o): string
    --delimiter(-d): string
    --batch(-b): string
    --helper(-f): string
    --no-headers(-n)
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv py filter" [
    --output(-o): string
    --delimiter(-d): string
    --batch(-b): string
    --helper(-f): string
    --no-headers(-n)
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv py map" [
    --output(-o): string
    --delimiter(-d): string
    --batch(-b): string
    --helper(-f): string
    --no-headers(-n)
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
    --delimiter(-d): string
    --pairwise
    --output(-o): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --unicode(-u)
    --output(-o): string
    --no-headers(-n)
    --dfa-size-limit: string
    --quiet(-q)
    --jobs(-j): string
    --size-limit: string
    --exact
    --ignore-case(-i)
    --literal
    --delimiter(-d): string
    --select(-s): string
    --progressbar(-p)
    --not-one
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
    --mode: string
    --reserved: string
    --prefix: string
    --output(-o): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --stratified: string
    --systematic: string
    --cluster: string
    --user-agent: string
    --output(-o): string
    --weighted: string
    --ts-interval: string
    --ts-start: string
    --seed: string
    --rng: string
    --ts-aggregate: string
    --ts-prefer-dmy
    --max-size: string
    --timeseries: string
    --delimiter(-d): string
    --ts-input-tz: string
    --bernoulli
    --ts-adaptive: string
    --timeout: string
    --force
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --jobs(-j): string
    --strict-formats
    --enum-threshold: string
    --dates-whitelist: string
    --pattern-columns: string
    --force
    --delimiter(-d): string
    --output(-o): string
    --prefer-dmy
    --stdout
    --memcheck
    --polars
    --no-headers(-n)
    --ignore-case(-i)
    --strict-dates
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --count(-c)
    --quiet(-q)
    --ignore-case(-i)
    --unicode(-u)
    --quick(-Q)
    --not-one
    --output(-o): string
    --dfa-size-limit: string
    --select(-s): string
    --delimiter(-d): string
    --size-limit: string
    --flag(-f): string
    --progressbar(-p)
    --invert-match(-v)
    --no-headers(-n)
    --json
    --literal
    --exact
    --preview-match: string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --not-one
    --output(-o): string
    --progressbar(-p)
    --quiet(-q)
    --literal
    --json(-j)
    --dfa-size-limit: string
    --size-limit: string
    --unmatched-output: string
    --jobs: string
    --exact
    --ignore-case(-i)
    --quick(-Q)
    --no-headers(-n)
    --delimiter(-d): string
    --flag(-f): string
    --count(-c)
    --unicode(-u)
    --select(-s): string
    --invert-match(-v)
    --flag-matches-only
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --random(-R)
    --seed: string
    --delimiter(-d): string
    --sort(-S)
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --start(-s): string
    --delimiter(-d): string
    --invert
    --json
    --len(-l): string
    --index(-i): string
    --output(-o): string
    --no-headers(-n)
    --end(-e): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --timeout: string
    --user-agent: string
    --jobs(-j): string
    --output(-o): string
    --quiet(-q)
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --timeout: string
    --user-agent: string
    --jobs(-j): string
    --output(-o): string
    --quiet(-q)
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --timeout: string
    --user-agent: string
    --jobs(-j): string
    --output(-o): string
    --quiet(-q)
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --timeout: string
    --user-agent: string
    --jobs(-j): string
    --output(-o): string
    --quiet(-q)
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --timeout: string
    --user-agent: string
    --jobs(-j): string
    --output(-o): string
    --quiet(-q)
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
    --pretty-json
    --just-mime
    --save-urlsample: string
    --harvest-mode
    --no-infer
    --quote: string
    --user-agent: string
    --prefer-dmy
    --sample: string
    --stats-types
    --progressbar(-p)
    --delimiter(-d): string
    --json
    --timeout: string
    --quick(-Q)
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --output(-o): string
    --numeric(-N)
    --ignore-case(-i)
    --faster
    --delimiter(-d): string
    --rng: string
    --seed: string
    --unique(-u)
    --natural
    --jobs(-j): string
    --reverse(-R)
    --memcheck
    --random
    --select(-s): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --all
    --select(-s): string
    --delimiter(-d): string
    --ignore-case(-i)
    --no-headers(-n)
    --pretty-json
    --progressbar(-p)
    --json
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --filter: string
    --quiet(-q)
    --pad: string
    --size(-s): string
    --filter-ignore-errors
    --jobs(-j): string
    --chunks(-c): string
    --no-headers(-n)
    --kb-size(-k): string
    --delimiter(-d): string
    --filter-cleanup
    --filename: string
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --date-format: string
    --try-parsedates
    --format: string
    --output(-o): string
    --float-precision: string
    --no-optimizations
    --quiet(-q)
    --time-format: string
    --truncate-ragged-lines
    --low-memory
    --cache-schema
    --ignore-errors
    --rnull-values: string
    --wnull-value: string
    --compression: string
    --compress-level: string
    --infer-len: string
    --datetime-format: string
    --decimal-comma
    --statistics
    --streaming
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --infer-boolean
    --nulls
    --everything(-E)
    --prefer-dmy
    --vis-whitespace
    --percentiles
    --typesonly
    --stats-jsonl
    --memcheck
    --delimiter(-d): string
    --weight: string
    --infer-dates
    --round: string
    --force
    --jobs(-j): string
    --percentile-list: string
    --output(-o): string
    --cardinality
    --mode
    --no-headers(-n)
    --select(-s): string
    --dates-whitelist: string
    --median
    --cache-threshold(-c): string
    --mad
    --quartiles
    --boolean-patterns: string
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --pad(-p): string
    --output(-o): string
    --align(-a): string
    --condense(-c): string
    --width(-w): string
    --delimiter(-d): string
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv template" [
    --outfilename: string
    --progressbar(-p)
    --delimiter: string
    --timeout: string
    --batch(-b): string
    --no-headers(-n)
    --ckan-token: string
    --template: string
    --cache-dir: string
    --customfilter-error: string
    --globals-json(-j): string
    --outsubdir-size: string
    --output(-o): string
    --template-file(-t): string
    --jobs: string
    --ckan-api: string
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --jobs(-j): string
    --dump(-u)
    --pipe(-i)
    --stats(-a)
    --schema(-s): string
    --quiet(-q)
    --print-package(-k)
    --drop(-d)
    --all-strings(-A)
    --separator(-p): string
    --stats-csv(-c): string
    --delimiter: string
    --evolve(-e)
    --help(-h)                # Print help
  ]

  export extern "qsv to datapackage" [
    --jobs(-j): string
    --dump(-u)
    --pipe(-i)
    --stats(-a)
    --schema(-s): string
    --quiet(-q)
    --print-package(-k)
    --drop(-d)
    --all-strings(-A)
    --separator(-p): string
    --stats-csv(-c): string
    --delimiter: string
    --evolve(-e)
    --help(-h)                # Print help
  ]

  export extern "qsv to ods" [
    --jobs(-j): string
    --dump(-u)
    --pipe(-i)
    --stats(-a)
    --schema(-s): string
    --quiet(-q)
    --print-package(-k)
    --drop(-d)
    --all-strings(-A)
    --separator(-p): string
    --stats-csv(-c): string
    --delimiter: string
    --evolve(-e)
    --help(-h)                # Print help
  ]

  export extern "qsv to postgres" [
    --jobs(-j): string
    --dump(-u)
    --pipe(-i)
    --stats(-a)
    --schema(-s): string
    --quiet(-q)
    --print-package(-k)
    --drop(-d)
    --all-strings(-A)
    --separator(-p): string
    --stats-csv(-c): string
    --delimiter: string
    --evolve(-e)
    --help(-h)                # Print help
  ]

  export extern "qsv to sqlite" [
    --jobs(-j): string
    --dump(-u)
    --pipe(-i)
    --stats(-a)
    --schema(-s): string
    --quiet(-q)
    --print-package(-k)
    --drop(-d)
    --all-strings(-A)
    --separator(-p): string
    --stats-csv(-c): string
    --delimiter: string
    --evolve(-e)
    --help(-h)                # Print help
  ]

  export extern "qsv to xlsx" [
    --jobs(-j): string
    --dump(-u)
    --pipe(-i)
    --stats(-a)
    --schema(-s): string
    --quiet(-q)
    --print-package(-k)
    --drop(-d)
    --all-strings(-A)
    --separator(-p): string
    --stats-csv(-c): string
    --delimiter: string
    --evolve(-e)
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
    --trim
    --memcheck
    --batch(-b): string
    --quiet(-q)
    --no-boolean
    --jobs(-j): string
    --delimiter(-d): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --select(-s): string
    --multipass(-m)
    --long: string
    --memcheck
    --output(-o): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --batch(-b): string
    --fail-fast
    --dfa-size-limit: string
    --ckan-token: string
    --no-headers(-n)
    --progressbar(-p)
    --email-domain-literal
    --email-display-text
    --quiet(-q)
    --invalid: string
    --fancy-regex
    --cache-dir: string
    --no-format-validation
    --backtrack-limit: string
    --valid: string
    --email-required-tld
    --size-limit: string
    --ckan-api: string
    --trim
    --delimiter(-d): string
    --valid-output: string
    --json
    --jobs(-j): string
    --pretty-json
    --email-min-subdomains: string
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv validate schema" [
    --batch(-b): string
    --fail-fast
    --dfa-size-limit: string
    --ckan-token: string
    --no-headers(-n)
    --progressbar(-p)
    --email-domain-literal
    --email-display-text
    --quiet(-q)
    --invalid: string
    --fancy-regex
    --cache-dir: string
    --no-format-validation
    --backtrack-limit: string
    --valid: string
    --email-required-tld
    --size-limit: string
    --ckan-api: string
    --trim
    --delimiter(-d): string
    --valid-output: string
    --json
    --jobs(-j): string
    --pretty-json
    --email-min-subdomains: string
    --timeout: string
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

  export extern "qsv help log" [
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
