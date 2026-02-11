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
    --rename(-r): string
    --batch(-b): string
    --formatstr(-f): string
    --no-headers(-n)
    --output(-o): string
    --delimiter(-d): string
    --jobs(-j): string
    --replacement(-R): string
    --new-column(-c): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --comparand(-C): string
    --rename(-r): string
    --batch(-b): string
    --formatstr(-f): string
    --no-headers(-n)
    --output(-o): string
    --delimiter(-d): string
    --jobs(-j): string
    --replacement(-R): string
    --new-column(-c): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --comparand(-C): string
    --rename(-r): string
    --batch(-b): string
    --formatstr(-f): string
    --no-headers(-n)
    --output(-o): string
    --delimiter(-d): string
    --jobs(-j): string
    --replacement(-R): string
    --new-column(-c): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --comparand(-C): string
    --rename(-r): string
    --batch(-b): string
    --formatstr(-f): string
    --no-headers(-n)
    --output(-o): string
    --delimiter(-d): string
    --jobs(-j): string
    --replacement(-R): string
    --new-column(-c): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --comparand(-C): string
    --rename(-r): string
    --batch(-b): string
    --formatstr(-f): string
    --no-headers(-n)
    --output(-o): string
    --delimiter(-d): string
    --jobs(-j): string
    --replacement(-R): string
    --new-column(-c): string
    --progressbar(-p)
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
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --group-name(-N): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --group(-g): string
    --flexible
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --group-name(-N): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --group(-g): string
    --flexible
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --group-name(-N): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --group(-g): string
    --flexible
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --group-name(-N): string
    --pad(-p)
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
    --title(-t): string
    --output(-o): string
    --row-numbers(-n)
    --memcheck
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --width
    --human-readable(-H)
    --width-no-delims
    --no-headers(-n)
    --low-memory
    --no-polars
    --flexible(-f)
    --json
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --batch(-b): string
    --utc
    --output-tz: string
    --input-tz: string
    --formatstr: string
    --ts-resolution(-R): string
    --prefer-dmy
    --new-column(-c): string
    --default-tz: string
    --progressbar(-p)
    --delimiter(-d): string
    --output(-o): string
    --rename(-r): string
    --jobs(-j): string
    --no-headers(-n)
    --keep-zero-time
    --zulu
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --human-readable(-H)
    --delimiter(-d): string
    --select(-s): string
    --ignore-case(-i)
    --no-headers(-n)
    --jobs(-j): string
    --output(-o): string
    --sorted
    --memcheck
    --numeric(-N)
    --dupes-output(-D): string
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --num-examples: string
    --sql-results: string
    --ckan-api: string
    --model(-m): string
    --api-key(-k): string
    --freq-options: string
    --session: string
    --num-tags: string
    --description
    --language: string
    --addl-props: string
    --addl-cols
    --tag-vocab: string
    --truncate-str: string
    --prompt(-p): string
    --dictionary
    --no-cache
    --cache-dir: string
    --stats-options: string
    --max-tokens(-t): string
    --base-url(-u): string
    --fresh
    --output(-o): string
    --ckan-token: string
    --user-agent: string
    --tags
    --format: string
    --timeout: string
    --sample-size: string
    --fewshot-examples
    --export-prompt: string
    --prompt-file: string
    --enum-threshold: string
    --disk-cache-dir: string
    --redis-cache
    --session-len: string
    --forget
    --addl-cols-list: string
    --flush-cache
    --all(-A)
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --drop-equal-fields
    --delimiter-right: string
    --no-headers-output
    --delimiter-output: string
    --no-headers-left
    --delimiter(-d): string
    --sort-columns: string
    --delimiter-left: string
    --jobs(-j): string
    --key(-k): string
    --no-headers-right
    --output(-o): string
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
    --hash: string
    --output(-o): string
    --no-headers(-n)
    --delimiter(-d): string
    --increment: string
    --uuid4
    --copy: string
    --constant: string
    --uuid7
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --trim
    --table: string
    --cell: string
    --output(-o): string
    --date-format: string
    --quiet(-q)
    --range: string
    --delimiter(-d): string
    --jobs(-j): string
    --sheet(-s): string
    --header-row: string
    --metadata: string
    --keep-zero-time
    --flexible
    --error-format: string
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --ignore-case(-i)
    --output(-o): string
    --delimiter(-d): string
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
    --temp-dir: string
    --select(-s): string
    --dupes-output(-D): string
    --delimiter(-d): string
    --no-headers(-n)
    --no-output
    --quiet(-q)
    --human-readable(-H)
    --memory-limit: string
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --memory-limit: string
    --reverse(-R)
    --select(-s): string
    --jobs(-j): string
    --tmp-dir: string
    --no-headers(-n)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --report: string
    --jaq: string
    --redis-cache
    --disk-cache-dir: string
    --mem-cache-size: string
    --cache-error
    --max-errors: string
    --no-cache
    --url-template: string
    --no-headers(-n)
    --progressbar(-p)
    --flush-cache
    --jaqfile: string
    --store-error
    --pretty
    --cookies
    --http-header(-H): string
    --max-retries: string
    --user-agent: string
    --delimiter(-d): string
    --timeout: string
    --new-column(-c): string
    --rate-limit: string
    --disk-cache
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --globals-json(-j): string
    --new-column(-c): string
    --store-error
    --user-agent: string
    --no-cache
    --report: string
    --mem-cache-size: string
    --cookies
    --jaqfile: string
    --redis-cache
    --no-headers(-n)
    --compress
    --timeout: string
    --http-header(-H): string
    --disk-cache-dir: string
    --progressbar(-p)
    --jaq: string
    --output(-o): string
    --rate-limit: string
    --content-type: string
    --pretty
    --max-errors: string
    --max-retries: string
    --flush-cache
    --disk-cache
    --cache-error
    --delimiter(-d): string
    --payload-tpl(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --first(-f)
    --default(-v): string
    --no-headers(-n)
    --delimiter(-d): string
    --groupby(-g): string
    --backfill(-b)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --quiet(-q)
    --insert(-i): string
    --quote: string
    --output(-o): string
    --delimiter(-d): string
    --length(-l): string
    --escape: string
    --remove-empty(-r)
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --field-separator(-f): string
    --condense(-c): string
    --separator(-s): string
    --no-headers(-n)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --delimiter(-d): string
    --output(-o): string
    --crlf
    --escape: string
    --ascii
    --quote-always
    --out-delimiter(-t): string
    --quote: string
    --no-final-newline
    --quote-never
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --dry-run: string
    --new-column(-c): string
    --unify(-u)
    --delimiter(-d): string
    --progressbar(-p)
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --output(-o): string
    --pct-dec-places: string
    --no-headers(-n)
    --weight: string
    --other-sorted
    --pretty-json
    --jobs(-j): string
    --delimiter(-d): string
    --lmt-threshold: string
    --pct-nulls
    --all-unique-text: string
    --json
    --null-text: string
    --no-nulls
    --vis-whitespace
    --ignore-case(-i)
    --rank-strategy(-r): string
    --null-sorted
    --memcheck
    --no-stats
    --limit(-l): string
    --no-trim
    --asc(-a)
    --select(-s): string
    --unq-limit(-u): string
    --other-text: string
    --no-other
    --toon
    --stats-filter: string
    --no-float: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfo" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfonow" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-check" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-load" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-reset" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-update" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookup" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookupnow" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reverse" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reversenow" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggest" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggestnow" [
    --admin1: string
    --output(-o): string
    --rename(-r): string
    --new-column(-c): string
    --delimiter(-d): string
    --formatstr(-f): string
    --invalid-result: string
    --k_weight(-k): string
    --jobs(-j): string
    --batch(-b): string
    --force
    --languages: string
    --cache-dir: string
    --min-score: string
    --progressbar(-p)
    --timeout: string
    --country: string
    --language(-l): string
    --cities-url: string
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
    --longitude(-x): string
    --latitude(-y): string
    --geometry(-g): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv headers" [
    --intersect
    --trim
    --delimiter(-d): string
    --just-names(-j)
    --just-count(-J)
    --help(-h)                # Print help
  ]

  export extern "qsv index" [
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv input" [
    --trim-fields
    --skip-lastlines: string
    --delimiter(-d): string
    --auto-skip
    --quote-style: string
    --comment: string
    --escape: string
    --encoding-errors: string
    --no-quoting
    --skip-lines: string
    --trim-headers
    --output(-o): string
    --quote: string
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --nulls
    --left-semi
    --left
    --right-anti
    --output(-o): string
    --right-semi
    --delimiter(-d): string
    --keys-output: string
    --ignore-case(-i)
    --ignore-leading-zeros(-z)
    --cross
    --no-headers(-n)
    --left-anti
    --full
    --right
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --coalesce
    --null-value: string
    --nulls
    --no-optimizations
    --datetime-format: string
    --date-format: string
    --no-sort
    --validate: string
    --right-semi
    --allow-exact-matches(-X)
    --ignore-leading-zeros(-z)
    --maintain-order: string
    --left-anti
    --filter-left: string
    --ignore-errors
    --asof
    --left_by: string
    --cache-schema: string
    --streaming
    --strategy: string
    --non-equi: string
    --right
    --filter-right: string
    --sql-filter: string
    --float-precision: string
    --norm-unicode(-N): string
    --output(-o): string
    --quiet(-q)
    --time-format: string
    --delimiter(-d): string
    --right_by: string
    --left-semi
    --ignore-case(-i)
    --cross
    --left
    --right-anti
    --decimal-comma
    --tolerance: string
    --infer-len: string
    --low-memory
    --full
    --try-parsedates
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --jaq: string
    --select(-s): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv jsonl" [
    --output(-o): string
    --batch(-b): string
    --delimiter(-d): string
    --jobs(-j): string
    --ignore-errors
    --help(-h)                # Print help
  ]

  export extern "qsv lens" [
    --ignore-case(-i)
    --monochrome(-m)
    --prompt(-P): string
    --echo-column: string
    --delimiter(-d): string
    --wrap-mode(-W): string
    --streaming-stdin(-S)
    --columns: string
    --debug
    --auto-reload(-A)
    --find: string
    --filter: string
    --freeze-columns(-f): string
    --no-headers
    --tab-separated(-t)
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --colindex
    --max-errors: string
    --delimiter(-d): string
    --no-globals(-g)
    --progressbar(-p)
    --ckan-api: string
    --timeout: string
    --begin(-B): string
    --ckan-token: string
    --cache-dir: string
    --end(-E): string
    --output(-o): string
    --no-headers(-n)
    --remap(-r)
    --help(-h)                # Print help
  ]

  export extern "qsv luau filter" [
    --colindex
    --max-errors: string
    --delimiter(-d): string
    --no-globals(-g)
    --progressbar(-p)
    --ckan-api: string
    --timeout: string
    --begin(-B): string
    --ckan-token: string
    --cache-dir: string
    --end(-E): string
    --output(-o): string
    --no-headers(-n)
    --remap(-r)
    --help(-h)                # Print help
  ]

  export extern "qsv luau map" [
    --colindex
    --max-errors: string
    --delimiter(-d): string
    --no-globals(-g)
    --progressbar(-p)
    --ckan-api: string
    --timeout: string
    --begin(-B): string
    --ckan-token: string
    --cache-dir: string
    --end(-E): string
    --output(-o): string
    --no-headers(-n)
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
    --stats-options: string
    --xsd-gdate-scan: string
    --epsilon(-e): string
    --pct-thresholds: string
    --join-keys(-K): string
    --advanced
    --jobs(-j): string
    --bivariate(-B)
    --progressbar(-p)
    --output(-o): string
    --join-inputs(-J): string
    --bivariate-stats(-S): string
    --round: string
    --force
    --use-percentiles
    --join-type(-T): string
    --cardinality-threshold(-C): string
    --help(-h)                # Print help
  ]

  export extern "qsv partition" [
    --drop
    --limit: string
    --no-headers(-n)
    --prefix-length(-p): string
    --delimiter(-d): string
    --filename: string
    --help(-h)                # Print help
  ]

  export extern "qsv pivotp" [
    --index(-i): string
    --infer-len: string
    --ignore-errors
    --try-parsedates
    --agg(-a): string
    --decimal-comma
    --values(-v): string
    --col-separator: string
    --maintain-order
    --output(-o): string
    --delimiter(-d): string
    --sort-columns
    --validate
    --quiet(-q)
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
    --base-delay-ms: string
    --workdir(-d): string
    --output(-o): string
    --save-fname: string
    --quiet(-q)
    --msg(-m): string
    --filters(-F): string
    --fd-output(-f)
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --formatstr: string
    --start: string
    --increment: string
    --delimiter(-d): string
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --batch(-b): string
    --no-headers(-n)
    --helper(-f): string
    --delimiter(-d): string
    --progressbar(-p)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv py filter" [
    --batch(-b): string
    --no-headers(-n)
    --helper(-f): string
    --delimiter(-d): string
    --progressbar(-p)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv py map" [
    --batch(-b): string
    --no-headers(-n)
    --helper(-f): string
    --delimiter(-d): string
    --progressbar(-p)
    --output(-o): string
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
    --output(-o): string
    --no-headers(-n)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --size-limit: string
    --dfa-size-limit: string
    --literal
    --exact
    --no-headers(-n)
    --select(-s): string
    --delimiter(-d): string
    --output(-o): string
    --not-one
    --ignore-case(-i)
    --progressbar(-p)
    --unicode(-u)
    --quiet(-q)
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv reverse" [
    --delimiter(-d): string
    --memcheck
    --output(-o): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv safenames" [
    --output(-o): string
    --delimiter(-d): string
    --mode: string
    --reserved: string
    --prefix: string
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --force
    --timeseries: string
    --delimiter(-d): string
    --stratified: string
    --bernoulli
    --weighted: string
    --cluster: string
    --ts-aggregate: string
    --ts-start: string
    --timeout: string
    --output(-o): string
    --ts-prefer-dmy
    --rng: string
    --ts-interval: string
    --ts-adaptive: string
    --seed: string
    --systematic: string
    --ts-input-tz: string
    --max-size: string
    --user-agent: string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --enum-threshold: string
    --force
    --no-headers(-n)
    --ignore-case(-i)
    --polars
    --strict-dates
    --jobs(-j): string
    --memcheck
    --output(-o): string
    --prefer-dmy
    --strict-formats
    --delimiter(-d): string
    --dates-whitelist: string
    --stdout
    --pattern-columns: string
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --flag(-f): string
    --exact
    --unicode(-u)
    --size-limit: string
    --literal
    --preview-match: string
    --jobs(-j): string
    --ignore-case(-i)
    --invert-match(-v)
    --output(-o): string
    --json
    --no-headers(-n)
    --progressbar(-p)
    --quiet(-q)
    --count(-c)
    --quick(-Q)
    --not-one
    --dfa-size-limit: string
    --delimiter(-d): string
    --select(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --no-headers(-n)
    --select(-s): string
    --size-limit: string
    --not-one
    --quiet(-q)
    --dfa-size-limit: string
    --unicode(-u)
    --json(-j)
    --flag(-f): string
    --invert-match(-v)
    --jobs: string
    --quick(-Q)
    --count(-c)
    --delimiter(-d): string
    --ignore-case(-i)
    --output(-o): string
    --progressbar(-p)
    --flag-matches-only
    --unmatched-output: string
    --literal
    --exact
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --output(-o): string
    --seed: string
    --sort(-S)
    --delimiter(-d): string
    --no-headers(-n)
    --random(-R)
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --len(-l): string
    --start(-s): string
    --delimiter(-d): string
    --index(-i): string
    --invert
    --output(-o): string
    --no-headers(-n)
    --end(-e): string
    --json
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --user-agent: string
    --output(-o): string
    --progressbar(-p)
    --jobs(-j): string
    --timeout: string
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --user-agent: string
    --output(-o): string
    --progressbar(-p)
    --jobs(-j): string
    --timeout: string
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --user-agent: string
    --output(-o): string
    --progressbar(-p)
    --jobs(-j): string
    --timeout: string
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --user-agent: string
    --output(-o): string
    --progressbar(-p)
    --jobs(-j): string
    --timeout: string
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --user-agent: string
    --output(-o): string
    --progressbar(-p)
    --jobs(-j): string
    --timeout: string
    --quiet(-q)
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
    --delimiter(-d): string
    --prefer-dmy
    --stats-types
    --quote: string
    --quick(-Q)
    --user-agent: string
    --harvest-mode
    --json
    --sample: string
    --progressbar(-p)
    --pretty-json
    --just-mime
    --timeout: string
    --no-infer
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --jobs(-j): string
    --memcheck
    --natural
    --numeric(-N)
    --select(-s): string
    --seed: string
    --rng: string
    --output(-o): string
    --delimiter(-d): string
    --random
    --ignore-case(-i)
    --faster
    --unique(-u)
    --no-headers(-n)
    --reverse(-R)
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --ignore-case(-i)
    --json
    --all
    --progressbar(-p)
    --delimiter(-d): string
    --no-headers(-n)
    --select(-s): string
    --pretty-json
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --filename: string
    --pad: string
    --jobs(-j): string
    --size(-s): string
    --delimiter(-d): string
    --filter-cleanup
    --chunks(-c): string
    --no-headers(-n)
    --filter-ignore-errors
    --kb-size(-k): string
    --quiet(-q)
    --filter: string
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --datetime-format: string
    --try-parsedates
    --low-memory
    --no-optimizations
    --time-format: string
    --statistics
    --format: string
    --truncate-ragged-lines
    --infer-len: string
    --cache-schema
    --streaming
    --wnull-value: string
    --compress-level: string
    --date-format: string
    --output(-o): string
    --delimiter(-d): string
    --ignore-errors
    --decimal-comma
    --rnull-values: string
    --float-precision: string
    --quiet(-q)
    --compression: string
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --infer-dates
    --cache-threshold(-c): string
    --typesonly
    --percentiles
    --stats-jsonl
    --mode
    --infer-boolean
    --boolean-patterns: string
    --median
    --no-headers(-n)
    --output(-o): string
    --jobs(-j): string
    --prefer-dmy
    --vis-whitespace
    --force
    --delimiter(-d): string
    --memcheck
    --round: string
    --cardinality
    --everything(-E)
    --weight: string
    --select(-s): string
    --quartiles
    --dates-whitelist: string
    --mad
    --nulls
    --percentile-list: string
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --pad(-p): string
    --condense(-c): string
    --width(-w): string
    --output(-o): string
    --delimiter(-d): string
    --memcheck
    --align(-a): string
    --help(-h)                # Print help
  ]

  export extern "qsv template" [
    --outsubdir-size: string
    --progressbar(-p)
    --jobs(-j): string
    --ckan-api: string
    --timeout: string
    --customfilter-error: string
    --template-file(-t): string
    --globals-json: string
    --cache-dir: string
    --ckan-token: string
    --batch(-b): string
    --template: string
    --output(-o): string
    --no-headers(-n)
    --delimiter: string
    --outfilename: string
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --schema(-s): string
    --delimiter(-d): string
    --pipe(-i)
    --stats-csv(-c): string
    --print-package(-k)
    --evolve(-e)
    --jobs(-j): string
    --drop
    --stats(-a)
    --all-strings(-A)
    --dump(-u)
    --quiet(-q)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to datapackage" [
    --schema(-s): string
    --delimiter(-d): string
    --pipe(-i)
    --stats-csv(-c): string
    --print-package(-k)
    --evolve(-e)
    --jobs(-j): string
    --drop
    --stats(-a)
    --all-strings(-A)
    --dump(-u)
    --quiet(-q)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to ods" [
    --schema(-s): string
    --delimiter(-d): string
    --pipe(-i)
    --stats-csv(-c): string
    --print-package(-k)
    --evolve(-e)
    --jobs(-j): string
    --drop
    --stats(-a)
    --all-strings(-A)
    --dump(-u)
    --quiet(-q)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to postgres" [
    --schema(-s): string
    --delimiter(-d): string
    --pipe(-i)
    --stats-csv(-c): string
    --print-package(-k)
    --evolve(-e)
    --jobs(-j): string
    --drop
    --stats(-a)
    --all-strings(-A)
    --dump(-u)
    --quiet(-q)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to sqlite" [
    --schema(-s): string
    --delimiter(-d): string
    --pipe(-i)
    --stats-csv(-c): string
    --print-package(-k)
    --evolve(-e)
    --jobs(-j): string
    --drop
    --stats(-a)
    --all-strings(-A)
    --dump(-u)
    --quiet(-q)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to xlsx" [
    --schema(-s): string
    --delimiter(-d): string
    --pipe(-i)
    --stats-csv(-c): string
    --print-package(-k)
    --evolve(-e)
    --jobs(-j): string
    --drop
    --stats(-a)
    --all-strings(-A)
    --dump(-u)
    --quiet(-q)
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
    --delimiter(-d): string
    --memcheck
    --trim
    --jobs(-j): string
    --quiet(-q)
    --batch(-b): string
    --output(-o): string
    --no-boolean
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --select(-s): string
    --long: string
    --delimiter(-d): string
    --multipass(-m)
    --memcheck
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --quiet(-q)
    --trim
    --timeout: string
    --progressbar(-p)
    --ckan-api: string
    --valid: string
    --delimiter(-d): string
    --email-required-tld
    --fancy-regex
    --no-headers(-n)
    --fail-fast
    --batch(-b): string
    --email-display-text
    --json
    --jobs(-j): string
    --dfa-size-limit: string
    --no-format-validation
    --email-min-subdomains: string
    --email-domain-literal
    --pretty-json
    --valid-output: string
    --ckan-token: string
    --backtrack-limit: string
    --invalid: string
    --cache-dir: string
    --size-limit: string
    --help(-h)                # Print help
  ]

  export extern "qsv validate schema" [
    --quiet(-q)
    --trim
    --timeout: string
    --progressbar(-p)
    --ckan-api: string
    --valid: string
    --delimiter(-d): string
    --email-required-tld
    --fancy-regex
    --no-headers(-n)
    --fail-fast
    --batch(-b): string
    --email-display-text
    --json
    --jobs(-j): string
    --dfa-size-limit: string
    --no-format-validation
    --email-min-subdomains: string
    --email-domain-literal
    --pretty-json
    --valid-output: string
    --ckan-token: string
    --backtrack-limit: string
    --invalid: string
    --cache-dir: string
    --size-limit: string
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
