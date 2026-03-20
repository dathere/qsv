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
    --no-headers(-n)
    --rename(-r): string
    --delimiter(-d): string
    --comparand(-C): string
    --progressbar(-p)
    --replacement(-R): string
    --output(-o): string
    --batch(-b): string
    --formatstr(-f): string
    --new-column(-c): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --no-headers(-n)
    --rename(-r): string
    --delimiter(-d): string
    --comparand(-C): string
    --progressbar(-p)
    --replacement(-R): string
    --output(-o): string
    --batch(-b): string
    --formatstr(-f): string
    --new-column(-c): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --no-headers(-n)
    --rename(-r): string
    --delimiter(-d): string
    --comparand(-C): string
    --progressbar(-p)
    --replacement(-R): string
    --output(-o): string
    --batch(-b): string
    --formatstr(-f): string
    --new-column(-c): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --no-headers(-n)
    --rename(-r): string
    --delimiter(-d): string
    --comparand(-C): string
    --progressbar(-p)
    --replacement(-R): string
    --output(-o): string
    --batch(-b): string
    --formatstr(-f): string
    --new-column(-c): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --no-headers(-n)
    --rename(-r): string
    --delimiter(-d): string
    --comparand(-C): string
    --progressbar(-p)
    --replacement(-R): string
    --output(-o): string
    --batch(-b): string
    --formatstr(-f): string
    --new-column(-c): string
    --jobs(-j): string
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
    --delimiter(-d): string
    --group-name(-N): string
    --flexible
    --no-headers(-n)
    --group(-g): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --output(-o): string
    --delimiter(-d): string
    --group-name(-N): string
    --flexible
    --no-headers(-n)
    --group(-g): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --output(-o): string
    --delimiter(-d): string
    --group-name(-N): string
    --flexible
    --no-headers(-n)
    --group(-g): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --output(-o): string
    --delimiter(-d): string
    --group-name(-N): string
    --flexible
    --no-headers(-n)
    --group(-g): string
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
    --memcheck
    --title(-t): string
    --color(-C)
    --output(-o): string
    --delimiter(-d): string
    --row-numbers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --human-readable(-H)
    --low-memory
    --width-no-delims
    --flexible(-f)
    --no-headers(-n)
    --delimiter(-d): string
    --no-polars
    --json
    --width
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --utc
    --progressbar(-p)
    --no-headers(-n)
    --output-tz: string
    --ts-resolution(-R): string
    --batch(-b): string
    --formatstr: string
    --rename(-r): string
    --input-tz: string
    --zulu
    --jobs(-j): string
    --output(-o): string
    --keep-zero-time
    --delimiter(-d): string
    --new-column(-c): string
    --default-tz: string
    --prefer-dmy
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --quiet(-q)
    --jobs(-j): string
    --sorted
    --dupes-output(-D): string
    --human-readable(-H)
    --ignore-case(-i)
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --numeric(-N)
    --select(-s): string
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --cache-dir: string
    --enum-threshold: string
    --tags
    --api-key(-k): string
    --redis-cache
    --prepare-context
    --addl-cols-list: string
    --no-score-sql
    --score-max-retries: string
    --max-tokens(-t): string
    --stats-options: string
    --description
    --disk-cache-dir: string
    --score-threshold: string
    --freq-options: string
    --flush-cache
    --timeout: string
    --all(-A)
    --output(-o): string
    --num-examples: string
    --prompt-file: string
    --truncate-str: string
    --sample-size: string
    --session: string
    --language: string
    --ckan-api: string
    --model(-m): string
    --addl-props: string
    --format: string
    --dictionary
    --fewshot-examples
    --session-len: string
    --user-agent: string
    --forget
    --base-url(-u): string
    --ckan-token: string
    --fresh
    --num-tags: string
    --addl-cols
    --prompt(-p): string
    --tag-vocab: string
    --no-cache
    --quiet(-q)
    --export-prompt: string
    --process-response
    --sql-results: string
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --jobs(-j): string
    --sort-columns: string
    --key(-k): string
    --no-headers-output
    --delimiter(-d): string
    --delimiter-output: string
    --no-headers-left
    --no-headers-right
    --delimiter-left: string
    --output(-o): string
    --drop-equal-fields
    --delimiter-right: string
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
    --constant: string
    --uuid7
    --copy: string
    --new-column(-c): string
    --increment: string
    --uuid4
    --output(-o): string
    --hash: string
    --delimiter(-d): string
    --start: string
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --sheet(-s): string
    --header-row: string
    --trim
    --output(-o): string
    --jobs(-j): string
    --metadata: string
    --flexible
    --keep-zero-time
    --cell: string
    --delimiter(-d): string
    --quiet(-q)
    --error-format: string
    --range: string
    --table: string
    --date-format: string
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --delimiter(-d): string
    --no-headers(-n)
    --invert(-v)
    --ignore-case(-i)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv explode" [
    --output(-o): string
    --no-headers(-n)
    --delimiter(-d): string
    --rename(-r): string
    --help(-h)                # Print help
  ]

  export extern "qsv extdedup" [
    --no-headers(-n)
    --quiet(-q)
    --no-output
    --human-readable(-H)
    --memory-limit: string
    --select(-s): string
    --delimiter(-d): string
    --dupes-output(-D): string
    --temp-dir: string
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --memory-limit: string
    --reverse(-R)
    --select(-s): string
    --delimiter(-d): string
    --tmp-dir: string
    --jobs(-j): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --max-retries: string
    --url-template: string
    --redis-cache
    --http-header(-H): string
    --new-column(-c): string
    --timeout: string
    --flush-cache
    --output(-o): string
    --no-cache
    --cookies
    --mem-cache-size: string
    --user-agent: string
    --report: string
    --max-errors: string
    --no-headers(-n)
    --cache-error
    --delimiter(-d): string
    --jaq: string
    --rate-limit: string
    --disk-cache-dir: string
    --jaqfile: string
    --disk-cache
    --pretty
    --progressbar(-p)
    --store-error
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --no-headers(-n)
    --compress
    --http-header(-H): string
    --cookies
    --flush-cache
    --rate-limit: string
    --globals-json(-j): string
    --progressbar(-p)
    --cache-error
    --output(-o): string
    --report: string
    --new-column(-c): string
    --max-errors: string
    --jaqfile: string
    --timeout: string
    --mem-cache-size: string
    --disk-cache
    --store-error
    --pretty
    --content-type: string
    --max-retries: string
    --disk-cache-dir: string
    --payload-tpl(-t): string
    --user-agent: string
    --no-cache
    --delimiter(-d): string
    --jaq: string
    --redis-cache
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --output(-o): string
    --backfill(-b)
    --delimiter(-d): string
    --default(-v): string
    --no-headers(-n)
    --first(-f)
    --groupby(-g): string
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --delimiter(-d): string
    --length(-l): string
    --escape: string
    --remove-empty(-r)
    --quote: string
    --insert(-i): string
    --quiet(-q)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --no-headers(-n)
    --condense(-c): string
    --field-separator(-f): string
    --separator(-s): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --output(-o): string
    --escape: string
    --quote-never
    --no-final-newline
    --delimiter(-d): string
    --quote-always
    --out-delimiter(-t): string
    --quote: string
    --ascii
    --crlf
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --unify(-u)
    --delimiter(-d): string
    --progressbar(-p)
    --dry-run: string
    --no-headers(-n)
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --null-text: string
    --no-nulls
    --rank-strategy(-r): string
    --high-card-threshold: string
    --all-unique-text: string
    --ignore-case(-i)
    --vis-whitespace
    --pct-nulls
    --memcheck
    --delimiter(-d): string
    --other-text: string
    --stats-filter: string
    --no-float: string
    --pct-dec-places: string
    --toon
    --weight: string
    --no-headers(-n)
    --lmt-threshold: string
    --limit(-l): string
    --no-trim
    --other-sorted
    --force
    --high-card-pct: string
    --jobs(-j): string
    --select(-s): string
    --no-stats
    --frequency-jsonl
    --no-other
    --null-sorted
    --pretty-json
    --output(-o): string
    --unq-limit(-u): string
    --json
    --asc(-a)
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfo" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfonow" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-check" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-load" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-reset" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-update" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookup" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookupnow" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reverse" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reversenow" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggest" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggestnow" [
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --cities-url: string
    --formatstr(-f): string
    --new-column(-c): string
    --k_weight(-k): string
    --languages: string
    --batch(-b): string
    --delimiter(-d): string
    --country: string
    --jobs(-j): string
    --timeout: string
    --cache-dir: string
    --min-score: string
    --invalid-result: string
    --language(-l): string
    --force
    --admin1: string
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
    --latitude(-y): string
    --geometry(-g): string
    --output(-o): string
    --longitude(-x): string
    --help(-h)                # Print help
  ]

  export extern "qsv headers" [
    --trim
    --delimiter(-d): string
    --just-names(-j)
    --intersect
    --just-count(-J)
    --help(-h)                # Print help
  ]

  export extern "qsv index" [
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv input" [
    --skip-lines: string
    --comment: string
    --no-quoting
    --quote-style: string
    --trim-headers
    --output(-o): string
    --delimiter(-d): string
    --auto-skip
    --encoding-errors: string
    --quote: string
    --escape: string
    --skip-lastlines: string
    --trim-fields
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --full
    --right-anti
    --left-anti
    --ignore-case(-i)
    --no-headers(-n)
    --cross
    --nulls
    --left-semi
    --left
    --keys-output: string
    --delimiter(-d): string
    --right-semi
    --output(-o): string
    --right
    --ignore-leading-zeros(-z)
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --delimiter(-d): string
    --ignore-errors
    --validate: string
    --left-anti
    --nulls
    --float-precision: string
    --ignore-case(-i)
    --cache-schema: string
    --no-optimizations
    --left_by: string
    --tolerance: string
    --filter-right: string
    --full
    --left-semi
    --maintain-order: string
    --sql-filter: string
    --try-parsedates
    --no-sort
    --right-anti
    --cross
    --date-format: string
    --quiet(-q)
    --datetime-format: string
    --output(-o): string
    --low-memory
    --null-value: string
    --non-equi: string
    --right
    --asof
    --right_by: string
    --filter-left: string
    --right-semi
    --infer-len: string
    --norm-unicode(-N): string
    --time-format: string
    --strategy: string
    --allow-exact-matches(-X)
    --decimal-comma
    --streaming
    --left
    --coalesce
    --ignore-leading-zeros(-z)
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --select(-s): string
    --output(-o): string
    --jaq: string
    --help(-h)                # Print help
  ]

  export extern "qsv jsonl" [
    --output(-o): string
    --delimiter(-d): string
    --batch(-b): string
    --ignore-errors
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv lens" [
    --echo-column: string
    --freeze-columns(-f): string
    --delimiter(-d): string
    --wrap-mode(-W): string
    --prompt(-P): string
    --filter: string
    --tab-separated(-t)
    --no-headers
    --find: string
    --ignore-case(-i)
    --debug
    --columns: string
    --auto-reload(-A)
    --monochrome(-m)
    --streaming-stdin(-S)
    --help(-h)                # Print help
  ]

  export extern "qsv log" [
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --delimiter(-d): string
    --cache-dir: string
    --no-globals(-g)
    --remap(-r)
    --end(-E): string
    --timeout: string
    --ckan-api: string
    --no-headers(-n)
    --max-errors: string
    --begin(-B): string
    --ckan-token: string
    --output(-o): string
    --colindex
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv luau filter" [
    --delimiter(-d): string
    --cache-dir: string
    --no-globals(-g)
    --remap(-r)
    --end(-E): string
    --timeout: string
    --ckan-api: string
    --no-headers(-n)
    --max-errors: string
    --begin(-B): string
    --ckan-token: string
    --output(-o): string
    --colindex
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv luau map" [
    --delimiter(-d): string
    --cache-dir: string
    --no-globals(-g)
    --remap(-r)
    --end(-E): string
    --timeout: string
    --ckan-api: string
    --no-headers(-n)
    --max-errors: string
    --begin(-B): string
    --ckan-token: string
    --output(-o): string
    --colindex
    --progressbar(-p)
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
    --xsd-gdate-scan: string
    --force
    --join-keys(-K): string
    --join-type(-T): string
    --bivariate(-B)
    --bivariate-stats(-S): string
    --pct-thresholds: string
    --join-inputs(-J): string
    --advanced
    --cardinality-threshold(-C): string
    --epsilon(-e): string
    --progressbar(-p)
    --use-percentiles
    --output(-o): string
    --stats-options: string
    --jobs(-j): string
    --round: string
    --help(-h)                # Print help
  ]

  export extern "qsv partition" [
    --drop
    --no-headers(-n)
    --limit: string
    --filename: string
    --delimiter(-d): string
    --prefix-length(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv pivotp" [
    --delimiter(-d): string
    --quiet(-q)
    --validate
    --col-separator: string
    --try-parsedates
    --values(-v): string
    --infer-len: string
    --ignore-errors
    --index(-i): string
    --output(-o): string
    --decimal-comma
    --agg(-a): string
    --maintain-order
    --sort-columns
    --help(-h)                # Print help
  ]

  export extern "qsv pragmastat" [
    --twosample(-t)
    --misrate(-m): string
    --stats-options: string
    --standalone
    --compare1: string
    --force
    --seed: string
    --select(-s): string
    --no-headers(-n)
    --no-bounds
    --jobs(-j): string
    --delimiter(-d): string
    --compare2: string
    --subsample: string
    --output(-o): string
    --round: string
    --memcheck
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
    --quiet(-q)
    --base-delay-ms: string
    --filters(-F): string
    --fd-output(-f)
    --workdir(-d): string
    --output(-o): string
    --msg(-m): string
    --save-fname: string
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --formatstr: string
    --output(-o): string
    --no-headers(-n)
    --increment: string
    --start: string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --no-headers(-n)
    --batch(-b): string
    --output(-o): string
    --progressbar(-p)
    --helper(-f): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv py filter" [
    --no-headers(-n)
    --batch(-b): string
    --output(-o): string
    --progressbar(-p)
    --helper(-f): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv py map" [
    --no-headers(-n)
    --batch(-b): string
    --output(-o): string
    --progressbar(-p)
    --helper(-f): string
    --delimiter(-d): string
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
    --output(-o): string
    --pairwise
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --exact
    --select(-s): string
    --output(-o): string
    --jobs(-j): string
    --size-limit: string
    --ignore-case(-i)
    --literal
    --quiet(-q)
    --progressbar(-p)
    --dfa-size-limit: string
    --no-headers(-n)
    --unicode(-u)
    --not-one
    --delimiter(-d): string
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
    --delimiter(-d): string
    --reserved: string
    --prefix: string
    --mode: string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --ts-aggregate: string
    --systematic: string
    --seed: string
    --ts-adaptive: string
    --cluster: string
    --ts-interval: string
    --ts-input-tz: string
    --ts-prefer-dmy
    --user-agent: string
    --timeout: string
    --bernoulli
    --output(-o): string
    --no-headers(-n)
    --delimiter(-d): string
    --stratified: string
    --weighted: string
    --force
    --rng: string
    --max-size: string
    --timeseries: string
    --ts-start: string
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --dates-whitelist: string
    --memcheck
    --output(-o): string
    --jobs(-j): string
    --delimiter(-d): string
    --enum-threshold: string
    --strict-formats
    --stdout
    --prefer-dmy
    --no-headers(-n)
    --ignore-case(-i)
    --strict-dates
    --pattern-columns: string
    --force
    --polars
    --help(-h)                # Print help
  ]

  export extern "qsv scoresql" [
    --delimiter(-d): string
    --duckdb
    --truncate-ragged-lines
    --try-parsedates
    --output(-o): string
    --quiet(-q)
    --json
    --infer-len: string
    --ignore-errors
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --exact
    --json
    --unicode(-u)
    --size-limit: string
    --dfa-size-limit: string
    --preview-match: string
    --select(-s): string
    --invert-match(-v)
    --flag(-f): string
    --jobs(-j): string
    --no-headers(-n)
    --ignore-case(-i)
    --count(-c)
    --quick(-Q)
    --not-one
    --progressbar(-p)
    --delimiter(-d): string
    --quiet(-q)
    --literal
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --dfa-size-limit: string
    --json(-j)
    --quiet(-q)
    --select(-s): string
    --flag-matches-only
    --output(-o): string
    --literal
    --no-headers(-n)
    --unmatched-output: string
    --size-limit: string
    --jobs: string
    --unicode(-u)
    --flag(-f): string
    --not-one
    --delimiter(-d): string
    --count(-c)
    --ignore-case(-i)
    --exact
    --quick(-Q)
    --invert-match(-v)
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --seed: string
    --sort(-S)
    --output(-o): string
    --no-headers(-n)
    --random(-R)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --output(-o): string
    --json
    --invert
    --len(-l): string
    --no-headers(-n)
    --index(-i): string
    --start(-s): string
    --end(-e): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --user-agent: string
    --timeout: string
    --progressbar(-p)
    --quiet(-q)
    --output(-o): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --user-agent: string
    --timeout: string
    --progressbar(-p)
    --quiet(-q)
    --output(-o): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --user-agent: string
    --timeout: string
    --progressbar(-p)
    --quiet(-q)
    --output(-o): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --user-agent: string
    --timeout: string
    --progressbar(-p)
    --quiet(-q)
    --output(-o): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --user-agent: string
    --timeout: string
    --progressbar(-p)
    --quiet(-q)
    --output(-o): string
    --jobs(-j): string
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
    --harvest-mode
    --save-urlsample: string
    --quick(-Q)
    --json
    --delimiter(-d): string
    --prefer-dmy
    --progressbar(-p)
    --user-agent: string
    --quote: string
    --pretty-json
    --timeout: string
    --no-infer
    --just-mime
    --sample: string
    --stats-types
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --random
    --faster
    --reverse(-R)
    --numeric(-N)
    --delimiter(-d): string
    --no-headers(-n)
    --natural
    --seed: string
    --memcheck
    --select(-s): string
    --output(-o): string
    --unique(-u)
    --rng: string
    --ignore-case(-i)
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --delimiter(-d): string
    --select(-s): string
    --no-headers(-n)
    --progressbar(-p)
    --json
    --ignore-case(-i)
    --pretty-json
    --all
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --filter-ignore-errors
    --filename: string
    --size(-s): string
    --kb-size(-k): string
    --filter-cleanup
    --chunks(-c): string
    --jobs(-j): string
    --filter: string
    --pad: string
    --delimiter(-d): string
    --no-headers(-n)
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --rnull-values: string
    --truncate-ragged-lines
    --ignore-errors
    --datetime-format: string
    --statistics
    --no-optimizations
    --delimiter(-d): string
    --time-format: string
    --output(-o): string
    --format: string
    --compression: string
    --date-format: string
    --streaming
    --compress-level: string
    --cache-schema
    --decimal-comma
    --infer-len: string
    --float-precision: string
    --try-parsedates
    --wnull-value: string
    --low-memory
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --infer-boolean
    --median
    --percentile-list: string
    --vis-whitespace
    --round: string
    --prefer-dmy
    --everything(-E)
    --jobs(-j): string
    --select(-s): string
    --dates-whitelist: string
    --force
    --mad
    --delimiter(-d): string
    --typesonly
    --infer-dates
    --weight: string
    --no-headers(-n)
    --mode
    --cardinality
    --output(-o): string
    --nulls
    --quartiles
    --cache-threshold(-c): string
    --percentiles
    --stats-jsonl
    --boolean-patterns: string
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --width(-w): string
    --pad(-p): string
    --delimiter(-d): string
    --output(-o): string
    --condense(-c): string
    --memcheck
    --align(-a): string
    --help(-h)                # Print help
  ]

  export extern "qsv template" [
    --ckan-token: string
    --ckan-api: string
    --timeout: string
    --cache-dir: string
    --output(-o): string
    --delimiter: string
    --jobs(-j): string
    --batch(-b): string
    --template-file(-t): string
    --progressbar(-p)
    --template: string
    --customfilter-error: string
    --globals-json: string
    --outsubdir-size: string
    --no-headers(-n)
    --outfilename: string
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --dump(-u)
    --pipe(-i)
    --separator(-p): string
    --evolve(-e)
    --schema(-s): string
    --print-package(-k)
    --drop(-d)
    --stats-csv(-c): string
    --stats(-a)
    --table(-t): string
    --delimiter: string
    --quiet(-q)
    --jobs(-j): string
    --all-strings(-A)
    --help(-h)                # Print help
  ]

  export extern "qsv to datapackage" [
    --dump(-u)
    --pipe(-i)
    --separator(-p): string
    --evolve(-e)
    --schema(-s): string
    --print-package(-k)
    --drop(-d)
    --stats-csv(-c): string
    --stats(-a)
    --table(-t): string
    --delimiter: string
    --quiet(-q)
    --jobs(-j): string
    --all-strings(-A)
    --help(-h)                # Print help
  ]

  export extern "qsv to ods" [
    --dump(-u)
    --pipe(-i)
    --separator(-p): string
    --evolve(-e)
    --schema(-s): string
    --print-package(-k)
    --drop(-d)
    --stats-csv(-c): string
    --stats(-a)
    --table(-t): string
    --delimiter: string
    --quiet(-q)
    --jobs(-j): string
    --all-strings(-A)
    --help(-h)                # Print help
  ]

  export extern "qsv to postgres" [
    --dump(-u)
    --pipe(-i)
    --separator(-p): string
    --evolve(-e)
    --schema(-s): string
    --print-package(-k)
    --drop(-d)
    --stats-csv(-c): string
    --stats(-a)
    --table(-t): string
    --delimiter: string
    --quiet(-q)
    --jobs(-j): string
    --all-strings(-A)
    --help(-h)                # Print help
  ]

  export extern "qsv to sqlite" [
    --dump(-u)
    --pipe(-i)
    --separator(-p): string
    --evolve(-e)
    --schema(-s): string
    --print-package(-k)
    --drop(-d)
    --stats-csv(-c): string
    --stats(-a)
    --table(-t): string
    --delimiter: string
    --quiet(-q)
    --jobs(-j): string
    --all-strings(-A)
    --help(-h)                # Print help
  ]

  export extern "qsv to xlsx" [
    --dump(-u)
    --pipe(-i)
    --separator(-p): string
    --evolve(-e)
    --schema(-s): string
    --print-package(-k)
    --drop(-d)
    --stats-csv(-c): string
    --stats(-a)
    --table(-t): string
    --delimiter: string
    --quiet(-q)
    --jobs(-j): string
    --all-strings(-A)
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
    --quiet(-q)
    --trim
    --output(-o): string
    --batch(-b): string
    --no-boolean
    --jobs(-j): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --long: string
    --select(-s): string
    --memcheck
    --multipass(-m)
    --output(-o): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --email-domain-literal
    --email-required-tld
    --trim
    --email-display-text
    --quiet(-q)
    --valid: string
    --ckan-api: string
    --fancy-regex
    --ckan-token: string
    --email-min-subdomains: string
    --progressbar(-p)
    --size-limit: string
    --invalid: string
    --timeout: string
    --json
    --cache-dir: string
    --no-headers(-n)
    --batch(-b): string
    --no-format-validation
    --valid-output: string
    --dfa-size-limit: string
    --backtrack-limit: string
    --jobs(-j): string
    --delimiter(-d): string
    --pretty-json
    --fail-fast
    --help(-h)                # Print help
  ]

  export extern "qsv validate schema" [
    --email-domain-literal
    --email-required-tld
    --trim
    --email-display-text
    --quiet(-q)
    --valid: string
    --ckan-api: string
    --fancy-regex
    --ckan-token: string
    --email-min-subdomains: string
    --progressbar(-p)
    --size-limit: string
    --invalid: string
    --timeout: string
    --json
    --cache-dir: string
    --no-headers(-n)
    --batch(-b): string
    --no-format-validation
    --valid-output: string
    --dfa-size-limit: string
    --backtrack-limit: string
    --jobs(-j): string
    --delimiter(-d): string
    --pretty-json
    --fail-fast
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

  export extern "qsv help scoresql" [
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
