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
    --rename(-r): string
    --new-column(-c): string
    --jobs(-j): string
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --batch(-b): string
    --comparand(-C): string
    --formatstr(-f): string
    --replacement(-R): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --progressbar(-p)
    --rename(-r): string
    --new-column(-c): string
    --jobs(-j): string
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --batch(-b): string
    --comparand(-C): string
    --formatstr(-f): string
    --replacement(-R): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --progressbar(-p)
    --rename(-r): string
    --new-column(-c): string
    --jobs(-j): string
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --batch(-b): string
    --comparand(-C): string
    --formatstr(-f): string
    --replacement(-R): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --progressbar(-p)
    --rename(-r): string
    --new-column(-c): string
    --jobs(-j): string
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --batch(-b): string
    --comparand(-C): string
    --formatstr(-f): string
    --replacement(-R): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --progressbar(-p)
    --rename(-r): string
    --new-column(-c): string
    --jobs(-j): string
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --batch(-b): string
    --comparand(-C): string
    --formatstr(-f): string
    --replacement(-R): string
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
    --keyed
    --check(-c)
    --derive-key: string
    --no-mmap
    --no-names
    --tag
    --jobs(-j): string
    --output(-o): string
    --raw
    --quiet(-q)
    --length(-l): string
    --help(-h)                # Print help
  ]

  export extern "qsv cat" [
    --delimiter(-d): string
    --no-headers(-n)
    --group-name(-N): string
    --group(-g): string
    --flexible
    --output(-o): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --delimiter(-d): string
    --no-headers(-n)
    --group-name(-N): string
    --group(-g): string
    --flexible
    --output(-o): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --delimiter(-d): string
    --no-headers(-n)
    --group-name(-N): string
    --group(-g): string
    --flexible
    --output(-o): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --delimiter(-d): string
    --no-headers(-n)
    --group-name(-N): string
    --group(-g): string
    --flexible
    --output(-o): string
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
    --row-numbers(-n)
    --output(-o): string
    --color(-C)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --no-headers(-n)
    --width
    --no-polars
    --delimiter(-d): string
    --flexible(-f)
    --width-no-delims
    --low-memory
    --json
    --human-readable(-H)
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --zulu
    --default-tz: string
    --formatstr: string
    --output-tz: string
    --batch(-b): string
    --utc
    --jobs(-j): string
    --keep-zero-time
    --output(-o): string
    --prefer-dmy
    --rename(-r): string
    --ts-resolution(-R): string
    --no-headers(-n)
    --delimiter(-d): string
    --progressbar(-p)
    --new-column(-c): string
    --input-tz: string
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --memcheck
    --dupes-output(-D): string
    --numeric(-N)
    --delimiter(-d): string
    --select(-s): string
    --output(-o): string
    --ignore-case(-i)
    --jobs(-j): string
    --quiet(-q)
    --human-readable(-H)
    --no-headers(-n)
    --sorted
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --truncate-str: string
    --language: string
    --addl-cols
    --forget
    --addl-props: string
    --timeout: string
    --addl-cols-list: string
    --tag-vocab: string
    --freq-options: string
    --max-tokens(-t): string
    --redis-cache
    --no-score-sql
    --ckan-api: string
    --api-key(-k): string
    --stats-options: string
    --export-prompt: string
    --description
    --ckan-token: string
    --process-response
    --num-examples: string
    --output(-o): string
    --session: string
    --prompt(-p): string
    --disk-cache-dir: string
    --score-threshold: string
    --cache-dir: string
    --prompt-file: string
    --model(-m): string
    --sample-size: string
    --dictionary
    --base-url(-u): string
    --score-max-retries: string
    --format: string
    --sql-results: string
    --num-tags: string
    --tags
    --user-agent: string
    --flush-cache
    --fewshot-examples
    --prepare-context
    --quiet(-q)
    --no-cache
    --fresh
    --session-len: string
    --enum-threshold: string
    --all(-A)
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --no-headers-output
    --no-headers-left
    --delimiter-right: string
    --delimiter(-d): string
    --delimiter-output: string
    --key(-k): string
    --sort-columns: string
    --drop-equal-fields
    --no-headers-right
    --output(-o): string
    --jobs(-j): string
    --delimiter-left: string
    --help(-h)                # Print help
  ]

  export extern "qsv edit" [
    --no-headers(-n)
    --output(-o): string
    --in-place(-i)
    --help(-h)                # Print help
  ]

  export extern "qsv enum" [
    --increment: string
    --start: string
    --uuid7
    --copy: string
    --uuid4
    --output(-o): string
    --no-headers(-n)
    --delimiter(-d): string
    --hash: string
    --new-column(-c): string
    --constant: string
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --metadata: string
    --sheet(-s): string
    --error-format: string
    --range: string
    --date-format: string
    --jobs(-j): string
    --quiet(-q)
    --flexible
    --table: string
    --trim
    --cell: string
    --output(-o): string
    --header-row: string
    --delimiter(-d): string
    --keep-zero-time
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --no-headers(-n)
    --invert(-v)
    --output(-o): string
    --delimiter(-d): string
    --ignore-case(-i)
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
    --temp-dir: string
    --select(-s): string
    --dupes-output(-D): string
    --human-readable(-H)
    --memory-limit: string
    --no-headers(-n)
    --delimiter(-d): string
    --quiet(-q)
    --no-output
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --reverse(-R)
    --memory-limit: string
    --select(-s): string
    --delimiter(-d): string
    --jobs(-j): string
    --tmp-dir: string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --jaq: string
    --progressbar(-p)
    --disk-cache
    --max-retries: string
    --pretty
    --new-column(-c): string
    --rate-limit: string
    --user-agent: string
    --report: string
    --mem-cache-size: string
    --http-header(-H): string
    --disk-cache-dir: string
    --timeout: string
    --delimiter(-d): string
    --redis-cache
    --cookies
    --jaqfile: string
    --url-template: string
    --no-cache
    --output(-o): string
    --no-headers(-n)
    --flush-cache
    --cache-error
    --store-error
    --max-errors: string
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --cookies
    --payload-tpl(-t): string
    --user-agent: string
    --mem-cache-size: string
    --content-type: string
    --http-header(-H): string
    --max-errors: string
    --jaqfile: string
    --new-column(-c): string
    --disk-cache
    --no-headers(-n)
    --max-retries: string
    --disk-cache-dir: string
    --progressbar(-p)
    --compress
    --rate-limit: string
    --redis-cache
    --globals-json(-j): string
    --timeout: string
    --report: string
    --delimiter(-d): string
    --output(-o): string
    --flush-cache
    --pretty
    --cache-error
    --store-error
    --jaq: string
    --no-cache
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --backfill(-b)
    --delimiter(-d): string
    --no-headers(-n)
    --first(-f)
    --default(-v): string
    --output(-o): string
    --groupby(-g): string
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --escape: string
    --delimiter(-d): string
    --length(-l): string
    --quote: string
    --insert(-i): string
    --output(-o): string
    --quiet(-q)
    --remove-empty(-r)
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --no-headers(-n)
    --field-separator(-f): string
    --separator(-s): string
    --condense(-c): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --escape: string
    --quote-always
    --delimiter(-d): string
    --quote: string
    --out-delimiter(-t): string
    --ascii
    --crlf
    --no-final-newline
    --quote-never
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --dry-run: string
    --delimiter(-d): string
    --progressbar(-p)
    --no-headers(-n)
    --unify(-u)
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --vis-whitespace
    --no-nulls
    --force
    --other-text: string
    --asc(-a)
    --no-float: string
    --json
    --toon
    --lmt-threshold: string
    --no-other
    --no-headers(-n)
    --high-card-threshold: string
    --stats-filter: string
    --other-sorted
    --all-unique-text: string
    --weight: string
    --no-trim
    --null-sorted
    --unq-limit(-u): string
    --limit(-l): string
    --frequency-jsonl
    --select(-s): string
    --null-text: string
    --high-card-pct: string
    --rank-strategy(-r): string
    --pretty-json
    --delimiter(-d): string
    --jobs(-j): string
    --memcheck
    --pct-dec-places: string
    --pct-nulls
    --no-stats
    --ignore-case(-i)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfo" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfonow" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-check" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-load" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-reset" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-update" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookup" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookupnow" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reverse" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reversenow" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggest" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggestnow" [
    --rename(-r): string
    --languages: string
    --cache-dir: string
    --formatstr(-f): string
    --force
    --progressbar(-p)
    --output(-o): string
    --admin1: string
    --invalid-result: string
    --min-score: string
    --timeout: string
    --batch(-b): string
    --k_weight(-k): string
    --delimiter(-d): string
    --country: string
    --cities-url: string
    --language(-l): string
    --jobs(-j): string
    --new-column(-c): string
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
    --max-length(-l): string
    --longitude(-x): string
    --output(-o): string
    --geometry(-g): string
    --help(-h)                # Print help
  ]

  export extern "qsv headers" [
    --delimiter(-d): string
    --just-names(-j)
    --just-count(-J)
    --trim
    --intersect
    --help(-h)                # Print help
  ]

  export extern "qsv index" [
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv input" [
    --trim-headers
    --skip-lines: string
    --skip-lastlines: string
    --quote-style: string
    --encoding-errors: string
    --delimiter(-d): string
    --no-quoting
    --auto-skip
    --trim-fields
    --escape: string
    --comment: string
    --quote: string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --right
    --right-anti
    --delimiter(-d): string
    --left
    --keys-output: string
    --cross
    --full
    --ignore-leading-zeros(-z)
    --output(-o): string
    --left-anti
    --right-semi
    --ignore-case(-i)
    --left-semi
    --no-headers(-n)
    --nulls
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --no-optimizations
    --cross
    --filter-left: string
    --right
    --coalesce
    --full
    --filter-right: string
    --right_by: string
    --right-semi
    --low-memory
    --asof
    --ignore-case(-i)
    --decimal-comma
    --sql-filter: string
    --streaming
    --norm-unicode(-N): string
    --non-equi: string
    --ignore-errors
    --left-anti
    --validate: string
    --right-anti
    --try-parsedates
    --null-value: string
    --strategy: string
    --delimiter(-d): string
    --quiet(-q)
    --allow-exact-matches(-X)
    --cache-schema: string
    --maintain-order: string
    --left_by: string
    --left
    --datetime-format: string
    --ignore-leading-zeros(-z)
    --float-precision: string
    --time-format: string
    --date-format: string
    --infer-len: string
    --no-sort
    --left-semi
    --nulls
    --tolerance: string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --jaq: string
    --select(-s): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv jsonl" [
    --batch(-b): string
    --output(-o): string
    --ignore-errors
    --delimiter(-d): string
    --jobs(-j): string
    --help(-h)                # Print help
  ]

  export extern "qsv lens" [
    --no-headers
    --debug
    --wrap-mode(-W): string
    --prompt(-P): string
    --filter: string
    --streaming-stdin(-S)
    --echo-column: string
    --auto-reload(-A)
    --ignore-case(-i)
    --freeze-columns(-f): string
    --tab-separated(-t)
    --find: string
    --delimiter(-d): string
    --monochrome(-m)
    --columns: string
    --help(-h)                # Print help
  ]

  export extern "qsv log" [
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --delimiter(-d): string
    --ckan-token: string
    --no-headers(-n)
    --timeout: string
    --no-globals(-g)
    --end(-E): string
    --max-errors: string
    --progressbar(-p)
    --cache-dir: string
    --colindex
    --output(-o): string
    --remap(-r)
    --ckan-api: string
    --begin(-B): string
    --help(-h)                # Print help
  ]

  export extern "qsv luau filter" [
    --delimiter(-d): string
    --ckan-token: string
    --no-headers(-n)
    --timeout: string
    --no-globals(-g)
    --end(-E): string
    --max-errors: string
    --progressbar(-p)
    --cache-dir: string
    --colindex
    --output(-o): string
    --remap(-r)
    --ckan-api: string
    --begin(-B): string
    --help(-h)                # Print help
  ]

  export extern "qsv luau map" [
    --delimiter(-d): string
    --ckan-token: string
    --no-headers(-n)
    --timeout: string
    --no-globals(-g)
    --end(-E): string
    --max-errors: string
    --progressbar(-p)
    --cache-dir: string
    --colindex
    --output(-o): string
    --remap(-r)
    --ckan-api: string
    --begin(-B): string
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
    --bivariate(-B)
    --join-inputs(-J): string
    --join-keys(-K): string
    --progressbar(-p)
    --xsd-gdate-scan: string
    --pct-thresholds: string
    --stats-options: string
    --round: string
    --join-type(-T): string
    --use-percentiles
    --jobs(-j): string
    --force
    --advanced
    --output(-o): string
    --epsilon(-e): string
    --cardinality-threshold(-C): string
    --bivariate-stats(-S): string
    --help(-h)                # Print help
  ]

  export extern "qsv partition" [
    --filename: string
    --drop
    --no-headers(-n)
    --prefix-length(-p): string
    --delimiter(-d): string
    --limit: string
    --help(-h)                # Print help
  ]

  export extern "qsv pivotp" [
    --decimal-comma
    --total-label: string
    --index(-i): string
    --agg(-a): string
    --ignore-errors
    --sort-columns
    --maintain-order
    --values(-v): string
    --infer-len: string
    --grand-total
    --delimiter(-d): string
    --validate
    --subtotal
    --col-separator: string
    --try-parsedates
    --quiet(-q)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv pragmastat" [
    --select(-s): string
    --stats-options: string
    --standalone
    --twosample(-t)
    --no-bounds
    --jobs(-j): string
    --delimiter(-d): string
    --compare1: string
    --force
    --no-headers(-n)
    --subsample: string
    --compare2: string
    --misrate(-m): string
    --seed: string
    --memcheck
    --output(-o): string
    --round: string
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
    --save-fname: string
    --msg(-m): string
    --workdir(-d): string
    --fd-output(-f)
    --base-delay-ms: string
    --output(-o): string
    --filters(-F): string
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --no-headers(-n)
    --start: string
    --output(-o): string
    --formatstr: string
    --delimiter(-d): string
    --increment: string
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --batch(-b): string
    --no-headers(-n)
    --progressbar(-p)
    --output(-o): string
    --helper(-f): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv py filter" [
    --batch(-b): string
    --no-headers(-n)
    --progressbar(-p)
    --output(-o): string
    --helper(-f): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv py map" [
    --batch(-b): string
    --no-headers(-n)
    --progressbar(-p)
    --output(-o): string
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
    --no-headers(-n)
    --delimiter(-d): string
    --pairwise
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --select(-s): string
    --no-headers(-n)
    --ignore-case(-i)
    --dfa-size-limit: string
    --jobs(-j): string
    --delimiter(-d): string
    --not-one
    --quiet(-q)
    --literal
    --size-limit: string
    --exact
    --unicode(-u)
    --progressbar(-p)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv reverse" [
    --output(-o): string
    --delimiter(-d): string
    --no-headers(-n)
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv safenames" [
    --reserved: string
    --output(-o): string
    --mode: string
    --prefix: string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --seed: string
    --cluster: string
    --weighted: string
    --rng: string
    --stratified: string
    --ts-input-tz: string
    --ts-prefer-dmy
    --bernoulli
    --ts-interval: string
    --no-headers(-n)
    --output(-o): string
    --ts-start: string
    --user-agent: string
    --delimiter(-d): string
    --systematic: string
    --timeout: string
    --max-size: string
    --ts-adaptive: string
    --timeseries: string
    --ts-aggregate: string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --ignore-case(-i)
    --strict-formats
    --no-headers(-n)
    --memcheck
    --dates-whitelist: string
    --prefer-dmy
    --force
    --pattern-columns: string
    --strict-dates
    --jobs(-j): string
    --enum-threshold: string
    --output(-o): string
    --stdout
    --polars
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv scoresql" [
    --duckdb
    --infer-len: string
    --output(-o): string
    --quiet(-q)
    --try-parsedates
    --truncate-ragged-lines
    --delimiter(-d): string
    --json
    --ignore-errors
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --literal
    --output(-o): string
    --quiet(-q)
    --size-limit: string
    --quick(-Q)
    --preview-match: string
    --ignore-case(-i)
    --exact
    --jobs(-j): string
    --invert-match(-v)
    --count(-c)
    --not-one
    --no-headers(-n)
    --flag(-f): string
    --select(-s): string
    --dfa-size-limit: string
    --delimiter(-d): string
    --progressbar(-p)
    --unicode(-u)
    --json
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --progressbar(-p)
    --size-limit: string
    --quiet(-q)
    --unmatched-output: string
    --unicode(-u)
    --ignore-case(-i)
    --not-one
    --invert-match(-v)
    --select(-s): string
    --exact
    --flag(-f): string
    --literal
    --dfa-size-limit: string
    --jobs: string
    --flag-matches-only
    --json(-j)
    --no-headers(-n)
    --delimiter(-d): string
    --quick(-Q)
    --count(-c)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --no-headers(-n)
    --delimiter(-d): string
    --output(-o): string
    --sort(-S)
    --seed: string
    --random(-R)
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --end(-e): string
    --index(-i): string
    --start(-s): string
    --len(-l): string
    --output(-o): string
    --no-headers(-n)
    --delimiter(-d): string
    --json
    --invert
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --progressbar(-p)
    --quiet(-q)
    --output(-o): string
    --user-agent: string
    --jobs(-j): string
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --progressbar(-p)
    --quiet(-q)
    --output(-o): string
    --user-agent: string
    --jobs(-j): string
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --progressbar(-p)
    --quiet(-q)
    --output(-o): string
    --user-agent: string
    --jobs(-j): string
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --progressbar(-p)
    --quiet(-q)
    --output(-o): string
    --user-agent: string
    --jobs(-j): string
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --progressbar(-p)
    --quiet(-q)
    --output(-o): string
    --user-agent: string
    --jobs(-j): string
    --timeout: string
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
    --quick(-Q)
    --timeout: string
    --save-urlsample: string
    --quote: string
    --just-mime
    --stats-types
    --delimiter(-d): string
    --progressbar(-p)
    --json
    --pretty-json
    --prefer-dmy
    --sample: string
    --no-infer
    --user-agent: string
    --harvest-mode
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --reverse(-R)
    --ignore-case(-i)
    --unique(-u)
    --jobs(-j): string
    --select(-s): string
    --memcheck
    --natural
    --no-headers(-n)
    --faster
    --output(-o): string
    --seed: string
    --numeric(-N)
    --rng: string
    --delimiter(-d): string
    --random
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --select(-s): string
    --all
    --delimiter(-d): string
    --json
    --pretty-json
    --ignore-case(-i)
    --progressbar(-p)
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --kb-size(-k): string
    --filter-ignore-errors
    --filter-cleanup
    --no-headers(-n)
    --delimiter(-d): string
    --filter: string
    --quiet(-q)
    --chunks(-c): string
    --size(-s): string
    --filename: string
    --jobs(-j): string
    --pad: string
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --infer-len: string
    --streaming
    --format: string
    --rnull-values: string
    --statistics
    --date-format: string
    --delimiter(-d): string
    --quiet(-q)
    --decimal-comma
    --cache-schema
    --output(-o): string
    --no-optimizations
    --truncate-ragged-lines
    --float-precision: string
    --low-memory
    --compress-level: string
    --compression: string
    --time-format: string
    --datetime-format: string
    --ignore-errors
    --try-parsedates
    --wnull-value: string
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --everything(-E)
    --mad
    --quartiles
    --weight: string
    --jobs(-j): string
    --cache-threshold(-c): string
    --delimiter(-d): string
    --infer-boolean
    --cardinality
    --force
    --nulls
    --mode
    --median
    --percentiles
    --dates-whitelist: string
    --boolean-patterns: string
    --memcheck
    --round: string
    --select(-s): string
    --prefer-dmy
    --percentile-list: string
    --output(-o): string
    --vis-whitespace
    --stats-jsonl
    --typesonly
    --no-headers(-n)
    --infer-dates
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --memcheck
    --align(-a): string
    --condense(-c): string
    --pad(-p): string
    --output(-o): string
    --delimiter(-d): string
    --width(-w): string
    --help(-h)                # Print help
  ]

  export extern "qsv template" [
    --template: string
    --output(-o): string
    --outfilename: string
    --delimiter: string
    --timeout: string
    --jobs(-j): string
    --outsubdir-size: string
    --ckan-api: string
    --template-file(-t): string
    --customfilter-error: string
    --globals-json: string
    --no-headers(-n)
    --batch(-b): string
    --cache-dir: string
    --ckan-token: string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --dump(-u)
    --compress-level: string
    --evolve(-e)
    --delimiter(-d): string
    --quiet(-q)
    --pipe(-i)
    --compression: string
    --try-parse-dates
    --stats(-a)
    --all-strings(-A)
    --print-package(-k)
    --jobs(-j): string
    --infer-len: string
    --schema(-s): string
    --drop
    --stats-csv(-c): string
    --separator(-p): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to datapackage" [
    --dump(-u)
    --compress-level: string
    --evolve(-e)
    --delimiter(-d): string
    --quiet(-q)
    --pipe(-i)
    --compression: string
    --try-parse-dates
    --stats(-a)
    --all-strings(-A)
    --print-package(-k)
    --jobs(-j): string
    --infer-len: string
    --schema(-s): string
    --drop
    --stats-csv(-c): string
    --separator(-p): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to ods" [
    --dump(-u)
    --compress-level: string
    --evolve(-e)
    --delimiter(-d): string
    --quiet(-q)
    --pipe(-i)
    --compression: string
    --try-parse-dates
    --stats(-a)
    --all-strings(-A)
    --print-package(-k)
    --jobs(-j): string
    --infer-len: string
    --schema(-s): string
    --drop
    --stats-csv(-c): string
    --separator(-p): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to parquet" [
    --dump(-u)
    --compress-level: string
    --evolve(-e)
    --delimiter(-d): string
    --quiet(-q)
    --pipe(-i)
    --compression: string
    --try-parse-dates
    --stats(-a)
    --all-strings(-A)
    --print-package(-k)
    --jobs(-j): string
    --infer-len: string
    --schema(-s): string
    --drop
    --stats-csv(-c): string
    --separator(-p): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to postgres" [
    --dump(-u)
    --compress-level: string
    --evolve(-e)
    --delimiter(-d): string
    --quiet(-q)
    --pipe(-i)
    --compression: string
    --try-parse-dates
    --stats(-a)
    --all-strings(-A)
    --print-package(-k)
    --jobs(-j): string
    --infer-len: string
    --schema(-s): string
    --drop
    --stats-csv(-c): string
    --separator(-p): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to sqlite" [
    --dump(-u)
    --compress-level: string
    --evolve(-e)
    --delimiter(-d): string
    --quiet(-q)
    --pipe(-i)
    --compression: string
    --try-parse-dates
    --stats(-a)
    --all-strings(-A)
    --print-package(-k)
    --jobs(-j): string
    --infer-len: string
    --schema(-s): string
    --drop
    --stats-csv(-c): string
    --separator(-p): string
    --table(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv to xlsx" [
    --dump(-u)
    --compress-level: string
    --evolve(-e)
    --delimiter(-d): string
    --quiet(-q)
    --pipe(-i)
    --compression: string
    --try-parse-dates
    --stats(-a)
    --all-strings(-A)
    --print-package(-k)
    --jobs(-j): string
    --infer-len: string
    --schema(-s): string
    --drop
    --stats-csv(-c): string
    --separator(-p): string
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
    --quiet(-q)
    --output(-o): string
    --trim
    --no-boolean
    --jobs(-j): string
    --delimiter(-d): string
    --batch(-b): string
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --output(-o): string
    --memcheck
    --select(-s): string
    --long: string
    --multipass(-m)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --progressbar(-p)
    --backtrack-limit: string
    --email-display-text
    --invalid: string
    --batch(-b): string
    --fail-fast
    --no-headers(-n)
    --valid: string
    --email-min-subdomains: string
    --ckan-token: string
    --cache-dir: string
    --jobs(-j): string
    --trim
    --valid-output: string
    --quiet(-q)
    --email-required-tld
    --fancy-regex
    --no-format-validation
    --dfa-size-limit: string
    --timeout: string
    --email-domain-literal
    --json
    --ckan-api: string
    --delimiter(-d): string
    --pretty-json
    --size-limit: string
    --help(-h)                # Print help
  ]

  export extern "qsv validate schema" [
    --progressbar(-p)
    --backtrack-limit: string
    --email-display-text
    --invalid: string
    --batch(-b): string
    --fail-fast
    --no-headers(-n)
    --valid: string
    --email-min-subdomains: string
    --ckan-token: string
    --cache-dir: string
    --jobs(-j): string
    --trim
    --valid-output: string
    --quiet(-q)
    --email-required-tld
    --fancy-regex
    --no-format-validation
    --dfa-size-limit: string
    --timeout: string
    --email-domain-literal
    --json
    --ckan-api: string
    --delimiter(-d): string
    --pretty-json
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
