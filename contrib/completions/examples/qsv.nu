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
    --formatstr(-f): string
    --delimiter(-d): string
    --comparand(-C): string
    --new-column(-c): string
    --batch(-b): string
    --replacement(-R): string
    --jobs(-j): string
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --no-headers(-n)
    --formatstr(-f): string
    --delimiter(-d): string
    --comparand(-C): string
    --new-column(-c): string
    --batch(-b): string
    --replacement(-R): string
    --jobs(-j): string
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --no-headers(-n)
    --formatstr(-f): string
    --delimiter(-d): string
    --comparand(-C): string
    --new-column(-c): string
    --batch(-b): string
    --replacement(-R): string
    --jobs(-j): string
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --no-headers(-n)
    --formatstr(-f): string
    --delimiter(-d): string
    --comparand(-C): string
    --new-column(-c): string
    --batch(-b): string
    --replacement(-R): string
    --jobs(-j): string
    --rename(-r): string
    --output(-o): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --no-headers(-n)
    --formatstr(-f): string
    --delimiter(-d): string
    --comparand(-C): string
    --new-column(-c): string
    --batch(-b): string
    --replacement(-R): string
    --jobs(-j): string
    --rename(-r): string
    --output(-o): string
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
    --flexible(-f)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv blake3" [
    --derive-key: string
    --quiet(-q)
    --keyed
    --output(-o): string
    --no-names
    --length(-l): string
    --jobs(-j): string
    --tag
    --no-mmap
    --raw
    --check(-c)
    --help(-h)                # Print help
  ]

  export extern "qsv cat" [
    --output(-o): string
    --pad(-p)
    --flexible
    --delimiter(-d): string
    --no-headers(-n)
    --group(-g): string
    --group-name(-N): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --output(-o): string
    --pad(-p)
    --flexible
    --delimiter(-d): string
    --no-headers(-n)
    --group(-g): string
    --group-name(-N): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --output(-o): string
    --pad(-p)
    --flexible
    --delimiter(-d): string
    --no-headers(-n)
    --group(-g): string
    --group-name(-N): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --output(-o): string
    --pad(-p)
    --flexible
    --delimiter(-d): string
    --no-headers(-n)
    --group(-g): string
    --group-name(-N): string
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
    --memcheck
    --delimiter(-d): string
    --row-numbers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --flexible(-f)
    --human-readable(-H)
    --delimiter(-d): string
    --no-polars
    --no-headers(-n)
    --low-memory
    --width-no-delims
    --json
    --width
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --formatstr: string
    --batch(-b): string
    --progressbar(-p)
    --zulu
    --utc
    --rename(-r): string
    --output-tz: string
    --jobs(-j): string
    --output(-o): string
    --delimiter(-d): string
    --prefer-dmy
    --ts-resolution(-R): string
    --default-tz: string
    --no-headers(-n)
    --new-column(-c): string
    --keep-zero-time
    --input-tz: string
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --sorted
    --dupes-output(-D): string
    --jobs(-j): string
    --numeric(-N)
    --output(-o): string
    --ignore-case(-i)
    --no-headers(-n)
    --delimiter(-d): string
    --select(-s): string
    --human-readable(-H)
    --quiet(-q)
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --cache-dir: string
    --ckan-api: string
    --num-tags: string
    --tag-vocab: string
    --fewshot-examples
    --no-score-sql
    --max-tokens(-t): string
    --timeout: string
    --user-agent: string
    --forget
    --freq-options: string
    --process-response
    --ckan-token: string
    --output(-o): string
    --dictionary
    --api-key(-k): string
    --no-cache
    --quiet(-q)
    --score-max-retries: string
    --num-examples: string
    --format: string
    --session: string
    --description
    --stats-options: string
    --addl-cols-list: string
    --prompt(-p): string
    --disk-cache-dir: string
    --addl-cols
    --model(-m): string
    --all(-A)
    --enum-threshold: string
    --tags
    --sample-size: string
    --base-url(-u): string
    --flush-cache
    --export-prompt: string
    --redis-cache
    --addl-props: string
    --fresh
    --session-len: string
    --language: string
    --prepare-context
    --prompt-file: string
    --sql-results: string
    --score-threshold: string
    --truncate-str: string
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --no-headers-left
    --key(-k): string
    --delimiter-right: string
    --sort-columns: string
    --drop-equal-fields
    --delimiter-output: string
    --delimiter(-d): string
    --no-headers-output
    --output(-o): string
    --no-headers-right
    --delimiter-left: string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv edit" [
    --output(-o): string
    --no-headers(-n)
    --in-place(-i)
    --help(-h)                # Print help
  ]

  export extern "qsv enum" [
    --uuid4
    --start: string
    --output(-o): string
    --increment: string
    --copy: string
    --no-headers(-n)
    --uuid7
    --delimiter(-d): string
    --new-column(-c): string
    --hash: string
    --constant: string
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --flexible
    --quiet(-q)
    --keep-zero-time
    --sheet(-s): string
    --trim
    --error-format: string
    --date-format: string
    --cell: string
    --delimiter(-d): string
    --header-row: string
    --range: string
    --table: string
    --jobs(-j): string
    --metadata: string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --no-headers(-n)
    --output(-o): string
    --invert(-v)
    --ignore-case(-i)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv explode" [
    --no-headers(-n)
    --output(-o): string
    --rename(-r): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv extdedup" [
    --no-output
    --temp-dir: string
    --dupes-output(-D): string
    --no-headers(-n)
    --select(-s): string
    --memory-limit: string
    --delimiter(-d): string
    --human-readable(-H)
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --memory-limit: string
    --tmp-dir: string
    --delimiter(-d): string
    --no-headers(-n)
    --reverse(-R)
    --jobs(-j): string
    --select(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --progressbar(-p)
    --no-cache
    --pretty
    --no-headers(-n)
    --user-agent: string
    --disk-cache
    --output(-o): string
    --http-header(-H): string
    --new-column(-c): string
    --cookies
    --rate-limit: string
    --max-retries: string
    --jaqfile: string
    --disk-cache-dir: string
    --jaq: string
    --delimiter(-d): string
    --flush-cache
    --mem-cache-size: string
    --redis-cache
    --cache-error
    --url-template: string
    --report: string
    --store-error
    --timeout: string
    --max-errors: string
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --max-retries: string
    --flush-cache
    --redis-cache
    --content-type: string
    --jaqfile: string
    --rate-limit: string
    --disk-cache
    --pretty
    --delimiter(-d): string
    --mem-cache-size: string
    --cookies
    --globals-json(-j): string
    --timeout: string
    --store-error
    --output(-o): string
    --progressbar(-p)
    --jaq: string
    --new-column(-c): string
    --report: string
    --user-agent: string
    --http-header(-H): string
    --cache-error
    --no-headers(-n)
    --max-errors: string
    --payload-tpl(-t): string
    --compress
    --disk-cache-dir: string
    --no-cache
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --output(-o): string
    --no-headers(-n)
    --default(-v): string
    --backfill(-b)
    --first(-f)
    --groupby(-g): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --quiet(-q)
    --escape: string
    --insert(-i): string
    --quote: string
    --output(-o): string
    --length(-l): string
    --remove-empty(-r)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --delimiter(-d): string
    --no-headers(-n)
    --condense(-c): string
    --separator(-s): string
    --field-separator(-f): string
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --crlf
    --quote-always
    --out-delimiter(-t): string
    --ascii
    --quote: string
    --no-final-newline
    --quote-never
    --escape: string
    --delimiter(-d): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --dry-run: string
    --new-column(-c): string
    --delimiter(-d): string
    --unify(-u)
    --progressbar(-p)
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --all-unique-text: string
    --output(-o): string
    --json
    --rank-strategy(-r): string
    --high-card-threshold: string
    --asc(-a)
    --no-nulls
    --weight: string
    --no-headers(-n)
    --no-other
    --other-sorted
    --ignore-case(-i)
    --no-stats
    --force
    --delimiter(-d): string
    --memcheck
    --jobs(-j): string
    --pretty-json
    --high-card-pct: string
    --pct-dec-places: string
    --other-text: string
    --frequency-jsonl
    --lmt-threshold: string
    --no-trim
    --null-text: string
    --vis-whitespace
    --unq-limit(-u): string
    --toon
    --stats-filter: string
    --pct-nulls
    --limit(-l): string
    --no-float: string
    --null-sorted
    --select(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfo" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfonow" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-check" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-load" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-reset" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-update" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookup" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookupnow" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reverse" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reversenow" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggest" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggestnow" [
    --languages: string
    --k_weight(-k): string
    --invalid-result: string
    --formatstr(-f): string
    --output(-o): string
    --language(-l): string
    --jobs(-j): string
    --cities-url: string
    --progressbar(-p)
    --rename(-r): string
    --batch(-b): string
    --new-column(-c): string
    --min-score: string
    --cache-dir: string
    --force
    --admin1: string
    --timeout: string
    --delimiter(-d): string
    --country: string
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
    --output(-o): string
    --max-length(-l): string
    --longitude(-x): string
    --geometry(-g): string
    --help(-h)                # Print help
  ]

  export extern "qsv headers" [
    --just-names(-j)
    --just-count(-J)
    --trim
    --intersect
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv index" [
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv input" [
    --escape: string
    --auto-skip
    --skip-lines: string
    --output(-o): string
    --skip-lastlines: string
    --encoding-errors: string
    --quote-style: string
    --trim-fields
    --trim-headers
    --delimiter(-d): string
    --quote: string
    --no-quoting
    --comment: string
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --right-semi
    --cross
    --keys-output: string
    --delimiter(-d): string
    --no-headers(-n)
    --left-semi
    --right
    --ignore-case(-i)
    --left-anti
    --left
    --full
    --output(-o): string
    --nulls
    --right-anti
    --ignore-leading-zeros(-z)
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --nulls
    --sql-filter: string
    --datetime-format: string
    --cross
    --asof
    --try-parsedates
    --validate: string
    --left-semi
    --no-optimizations
    --filter-left: string
    --left
    --left-anti
    --strategy: string
    --allow-exact-matches(-X)
    --left_by: string
    --filter-right: string
    --norm-unicode(-N): string
    --null-value: string
    --quiet(-q)
    --time-format: string
    --right-anti
    --ignore-case(-i)
    --cache-schema: string
    --date-format: string
    --non-equi: string
    --float-precision: string
    --delimiter(-d): string
    --coalesce
    --no-sort
    --decimal-comma
    --ignore-leading-zeros(-z)
    --right
    --infer-len: string
    --right-semi
    --full
    --ignore-errors
    --maintain-order: string
    --streaming
    --low-memory
    --right_by: string
    --tolerance: string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --output(-o): string
    --select(-s): string
    --jaq: string
    --help(-h)                # Print help
  ]

  export extern "qsv jsonl" [
    --jobs(-j): string
    --output(-o): string
    --delimiter(-d): string
    --batch(-b): string
    --ignore-errors
    --help(-h)                # Print help
  ]

  export extern "qsv lens" [
    --find: string
    --auto-reload(-A)
    --delimiter(-d): string
    --columns: string
    --streaming-stdin(-S)
    --debug
    --filter: string
    --ignore-case(-i)
    --monochrome(-m)
    --no-headers
    --prompt(-P): string
    --freeze-columns(-f): string
    --echo-column: string
    --wrap-mode(-W): string
    --tab-separated(-t)
    --help(-h)                # Print help
  ]

  export extern "qsv log" [
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --output(-o): string
    --remap(-r)
    --ckan-api: string
    --cache-dir: string
    --begin(-B): string
    --no-globals(-g)
    --no-headers(-n)
    --end(-E): string
    --progressbar(-p)
    --max-errors: string
    --delimiter(-d): string
    --timeout: string
    --colindex
    --ckan-token: string
    --help(-h)                # Print help
  ]

  export extern "qsv luau filter" [
    --output(-o): string
    --remap(-r)
    --ckan-api: string
    --cache-dir: string
    --begin(-B): string
    --no-globals(-g)
    --no-headers(-n)
    --end(-E): string
    --progressbar(-p)
    --max-errors: string
    --delimiter(-d): string
    --timeout: string
    --colindex
    --ckan-token: string
    --help(-h)                # Print help
  ]

  export extern "qsv luau map" [
    --output(-o): string
    --remap(-r)
    --ckan-api: string
    --cache-dir: string
    --begin(-B): string
    --no-globals(-g)
    --no-headers(-n)
    --end(-E): string
    --progressbar(-p)
    --max-errors: string
    --delimiter(-d): string
    --timeout: string
    --colindex
    --ckan-token: string
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
    --use-percentiles
    --bivariate(-B)
    --advanced
    --join-type(-T): string
    --jobs(-j): string
    --output(-o): string
    --join-keys(-K): string
    --xsd-gdate-scan: string
    --cardinality-threshold(-C): string
    --stats-options: string
    --round: string
    --bivariate-stats(-S): string
    --epsilon(-e): string
    --pct-thresholds: string
    --progressbar(-p)
    --join-inputs(-J): string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv partition" [
    --no-headers(-n)
    --prefix-length(-p): string
    --drop
    --limit: string
    --delimiter(-d): string
    --filename: string
    --help(-h)                # Print help
  ]

  export extern "qsv pivotp" [
    --sort-columns
    --col-separator: string
    --total-label: string
    --quiet(-q)
    --output(-o): string
    --validate
    --maintain-order
    --ignore-errors
    --agg(-a): string
    --decimal-comma
    --index(-i): string
    --values(-v): string
    --grand-total
    --subtotal
    --try-parsedates
    --delimiter(-d): string
    --infer-len: string
    --help(-h)                # Print help
  ]

  export extern "qsv pragmastat" [
    --force
    --round: string
    --subsample: string
    --stats-options: string
    --select(-s): string
    --misrate(-m): string
    --no-bounds
    --twosample(-t)
    --compare2: string
    --seed: string
    --memcheck
    --standalone
    --delimiter(-d): string
    --output(-o): string
    --jobs(-j): string
    --compare1: string
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
    --workdir(-d): string
    --save-fname: string
    --filters(-F): string
    --msg(-m): string
    --fd-output(-f)
    --output(-o): string
    --base-delay-ms: string
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --increment: string
    --start: string
    --formatstr: string
    --output(-o): string
    --no-headers(-n)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --no-headers(-n)
    --delimiter(-d): string
    --output(-o): string
    --batch(-b): string
    --progressbar(-p)
    --helper(-f): string
    --help(-h)                # Print help
  ]

  export extern "qsv py filter" [
    --no-headers(-n)
    --delimiter(-d): string
    --output(-o): string
    --batch(-b): string
    --progressbar(-p)
    --helper(-f): string
    --help(-h)                # Print help
  ]

  export extern "qsv py map" [
    --no-headers(-n)
    --delimiter(-d): string
    --output(-o): string
    --batch(-b): string
    --progressbar(-p)
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
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --pairwise
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --unicode(-u)
    --select(-s): string
    --dfa-size-limit: string
    --delimiter(-d): string
    --progressbar(-p)
    --literal
    --ignore-case(-i)
    --size-limit: string
    --not-one
    --exact
    --output(-o): string
    --jobs(-j): string
    --no-headers(-n)
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv reverse" [
    --no-headers(-n)
    --delimiter(-d): string
    --output(-o): string
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv safenames" [
    --mode: string
    --reserved: string
    --output(-o): string
    --delimiter(-d): string
    --prefix: string
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --cluster: string
    --user-agent: string
    --timeseries: string
    --bernoulli
    --delimiter(-d): string
    --ts-prefer-dmy
    --seed: string
    --ts-input-tz: string
    --ts-adaptive: string
    --rng: string
    --ts-aggregate: string
    --timeout: string
    --max-size: string
    --force
    --output(-o): string
    --ts-interval: string
    --systematic: string
    --stratified: string
    --weighted: string
    --ts-start: string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --force
    --output(-o): string
    --delimiter(-d): string
    --strict-dates
    --dates-whitelist: string
    --ignore-case(-i)
    --polars
    --memcheck
    --pattern-columns: string
    --jobs(-j): string
    --stdout
    --enum-threshold: string
    --prefer-dmy
    --strict-formats
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv scoresql" [
    --ignore-errors
    --infer-len: string
    --json
    --output(-o): string
    --delimiter(-d): string
    --try-parsedates
    --quiet(-q)
    --duckdb
    --truncate-ragged-lines
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --exact
    --no-headers(-n)
    --select(-s): string
    --flag(-f): string
    --preview-match: string
    --jobs(-j): string
    --invert-match(-v)
    --size-limit: string
    --dfa-size-limit: string
    --json
    --delimiter(-d): string
    --output(-o): string
    --quick(-Q)
    --progressbar(-p)
    --unicode(-u)
    --count(-c)
    --ignore-case(-i)
    --literal
    --quiet(-q)
    --not-one
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --delimiter(-d): string
    --ignore-case(-i)
    --not-one
    --json(-j)
    --progressbar(-p)
    --invert-match(-v)
    --flag-matches-only
    --literal
    --exact
    --size-limit: string
    --unmatched-output: string
    --quick(-Q)
    --no-headers(-n)
    --count(-c)
    --flag(-f): string
    --select(-s): string
    --output(-o): string
    --quiet(-q)
    --dfa-size-limit: string
    --jobs: string
    --unicode(-u)
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --no-headers(-n)
    --random(-R)
    --output(-o): string
    --sort(-S)
    --seed: string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --json
    --no-headers(-n)
    --start(-s): string
    --output(-o): string
    --invert
    --end(-e): string
    --len(-l): string
    --index(-i): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --progressbar(-p)
    --quiet(-q)
    --timeout: string
    --user-agent: string
    --output(-o): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --progressbar(-p)
    --quiet(-q)
    --timeout: string
    --user-agent: string
    --output(-o): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --progressbar(-p)
    --quiet(-q)
    --timeout: string
    --user-agent: string
    --output(-o): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --progressbar(-p)
    --quiet(-q)
    --timeout: string
    --user-agent: string
    --output(-o): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --progressbar(-p)
    --quiet(-q)
    --timeout: string
    --user-agent: string
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
    --json
    --harvest-mode
    --just-mime
    --progressbar(-p)
    --stats-types
    --user-agent: string
    --quick(-Q)
    --quote: string
    --delimiter(-d): string
    --save-urlsample: string
    --no-infer
    --timeout: string
    --pretty-json
    --prefer-dmy
    --sample: string
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --reverse(-R)
    --natural
    --numeric(-N)
    --memcheck
    --random
    --faster
    --delimiter(-d): string
    --no-headers(-n)
    --output(-o): string
    --rng: string
    --jobs(-j): string
    --seed: string
    --unique(-u)
    --select(-s): string
    --ignore-case(-i)
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --select(-s): string
    --json
    --progressbar(-p)
    --ignore-case(-i)
    --all
    --no-headers(-n)
    --delimiter(-d): string
    --pretty-json
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --kb-size(-k): string
    --jobs(-j): string
    --no-headers(-n)
    --filter-ignore-errors
    --quiet(-q)
    --size(-s): string
    --filename: string
    --chunks(-c): string
    --filter-cleanup
    --delimiter(-d): string
    --filter: string
    --pad: string
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --delimiter(-d): string
    --quiet(-q)
    --streaming
    --wnull-value: string
    --no-optimizations
    --compression: string
    --ignore-errors
    --time-format: string
    --infer-len: string
    --compress-level: string
    --cache-schema
    --datetime-format: string
    --try-parsedates
    --decimal-comma
    --statistics
    --output(-o): string
    --truncate-ragged-lines
    --float-precision: string
    --date-format: string
    --format: string
    --rnull-values: string
    --low-memory
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --no-headers(-n)
    --memcheck
    --quartiles
    --percentile-list: string
    --mode
    --infer-dates
    --force
    --delimiter(-d): string
    --nulls
    --output(-o): string
    --select(-s): string
    --prefer-dmy
    --boolean-patterns: string
    --typesonly
    --jobs(-j): string
    --everything(-E)
    --round: string
    --infer-boolean
    --cache-threshold(-c): string
    --mad
    --cardinality
    --dates-whitelist: string
    --percentiles
    --vis-whitespace
    --median
    --weight: string
    --stats-jsonl
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --output(-o): string
    --memcheck
    --align(-a): string
    --pad(-p): string
    --width(-w): string
    --delimiter(-d): string
    --condense(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv template" [
    --template: string
    --progressbar(-p)
    --customfilter-error: string
    --batch(-b): string
    --globals-json(-j): string
    --jobs: string
    --ckan-token: string
    --ckan-api: string
    --output(-o): string
    --no-headers(-n)
    --template-file(-t): string
    --outsubdir-size: string
    --delimiter: string
    --cache-dir: string
    --outfilename: string
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --evolve(-e)
    --pipe(-i)
    --stats-csv(-c): string
    --delimiter(-d): string
    --infer-len: string
    --separator(-p): string
    --all-strings(-A)
    --compression: string
    --dump(-u)
    --stats(-a)
    --quiet(-q)
    --jobs(-j): string
    --drop
    --print-package(-k)
    --try-parse-dates
    --compress-level: string
    --schema(-s): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to datapackage" [
    --evolve(-e)
    --pipe(-i)
    --stats-csv(-c): string
    --delimiter(-d): string
    --infer-len: string
    --separator(-p): string
    --all-strings(-A)
    --compression: string
    --dump(-u)
    --stats(-a)
    --quiet(-q)
    --jobs(-j): string
    --drop
    --print-package(-k)
    --try-parse-dates
    --compress-level: string
    --schema(-s): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to ods" [
    --evolve(-e)
    --pipe(-i)
    --stats-csv(-c): string
    --delimiter(-d): string
    --infer-len: string
    --separator(-p): string
    --all-strings(-A)
    --compression: string
    --dump(-u)
    --stats(-a)
    --quiet(-q)
    --jobs(-j): string
    --drop
    --print-package(-k)
    --try-parse-dates
    --compress-level: string
    --schema(-s): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to parquet" [
    --evolve(-e)
    --pipe(-i)
    --stats-csv(-c): string
    --delimiter(-d): string
    --infer-len: string
    --separator(-p): string
    --all-strings(-A)
    --compression: string
    --dump(-u)
    --stats(-a)
    --quiet(-q)
    --jobs(-j): string
    --drop
    --print-package(-k)
    --try-parse-dates
    --compress-level: string
    --schema(-s): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to postgres" [
    --evolve(-e)
    --pipe(-i)
    --stats-csv(-c): string
    --delimiter(-d): string
    --infer-len: string
    --separator(-p): string
    --all-strings(-A)
    --compression: string
    --dump(-u)
    --stats(-a)
    --quiet(-q)
    --jobs(-j): string
    --drop
    --print-package(-k)
    --try-parse-dates
    --compress-level: string
    --schema(-s): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to sqlite" [
    --evolve(-e)
    --pipe(-i)
    --stats-csv(-c): string
    --delimiter(-d): string
    --infer-len: string
    --separator(-p): string
    --all-strings(-A)
    --compression: string
    --dump(-u)
    --stats(-a)
    --quiet(-q)
    --jobs(-j): string
    --drop
    --print-package(-k)
    --try-parse-dates
    --compress-level: string
    --schema(-s): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to xlsx" [
    --evolve(-e)
    --pipe(-i)
    --stats-csv(-c): string
    --delimiter(-d): string
    --infer-len: string
    --separator(-p): string
    --all-strings(-A)
    --compression: string
    --dump(-u)
    --stats(-a)
    --quiet(-q)
    --jobs(-j): string
    --drop
    --print-package(-k)
    --try-parse-dates
    --compress-level: string
    --schema(-s): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv to help" [
  ]

  export extern "qsv to help datapackage" [
  ]

  export extern "qsv to help ods" [
  ]

  export extern "qsv to help parquet" [
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
    --quiet(-q)
    --output(-o): string
    --batch(-b): string
    --delimiter(-d): string
    --no-boolean
    --jobs(-j): string
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --delimiter(-d): string
    --long: string
    --select(-s): string
    --memcheck
    --output(-o): string
    --multipass(-m)
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --cache-dir: string
    --email-required-tld
    --size-limit: string
    --valid: string
    --json
    --fail-fast
    --delimiter(-d): string
    --jobs(-j): string
    --ckan-api: string
    --trim
    --email-min-subdomains: string
    --pretty-json
    --timeout: string
    --invalid: string
    --ckan-token: string
    --batch(-b): string
    --dfa-size-limit: string
    --quiet(-q)
    --valid-output: string
    --no-format-validation
    --email-display-text
    --fancy-regex
    --backtrack-limit: string
    --email-domain-literal
    --no-headers(-n)
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv validate schema" [
    --cache-dir: string
    --email-required-tld
    --size-limit: string
    --valid: string
    --json
    --fail-fast
    --delimiter(-d): string
    --jobs(-j): string
    --ckan-api: string
    --trim
    --email-min-subdomains: string
    --pretty-json
    --timeout: string
    --invalid: string
    --ckan-token: string
    --batch(-b): string
    --dfa-size-limit: string
    --quiet(-q)
    --valid-output: string
    --no-format-validation
    --email-display-text
    --fancy-regex
    --backtrack-limit: string
    --email-domain-literal
    --no-headers(-n)
    --progressbar(-p)
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

  export extern "qsv help blake3" [
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

  export extern "qsv help to parquet" [
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
