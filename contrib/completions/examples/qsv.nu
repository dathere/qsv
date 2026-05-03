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
    --comparand(-C): string
    --delimiter(-d): string
    --jobs(-j): string
    --replacement(-R): string
    --batch(-b): string
    --output(-o): string
    --progressbar(-p)
    --new-column(-c): string
    --rename(-r): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --no-headers(-n)
    --formatstr(-f): string
    --comparand(-C): string
    --delimiter(-d): string
    --jobs(-j): string
    --replacement(-R): string
    --batch(-b): string
    --output(-o): string
    --progressbar(-p)
    --new-column(-c): string
    --rename(-r): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --no-headers(-n)
    --formatstr(-f): string
    --comparand(-C): string
    --delimiter(-d): string
    --jobs(-j): string
    --replacement(-R): string
    --batch(-b): string
    --output(-o): string
    --progressbar(-p)
    --new-column(-c): string
    --rename(-r): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --no-headers(-n)
    --formatstr(-f): string
    --comparand(-C): string
    --delimiter(-d): string
    --jobs(-j): string
    --replacement(-R): string
    --batch(-b): string
    --output(-o): string
    --progressbar(-p)
    --new-column(-c): string
    --rename(-r): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --no-headers(-n)
    --formatstr(-f): string
    --comparand(-C): string
    --delimiter(-d): string
    --jobs(-j): string
    --replacement(-R): string
    --batch(-b): string
    --output(-o): string
    --progressbar(-p)
    --new-column(-c): string
    --rename(-r): string
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

  export extern "qsv blake3" [
    --output(-o): string
    --jobs(-j): string
    --raw
    --tag
    --derive-key: string
    --length(-l): string
    --quiet(-q)
    --keyed
    --no-mmap
    --check(-c)
    --no-names
    --help(-h)                # Print help
  ]

  export extern "qsv cat" [
    --no-headers(-n)
    --delimiter(-d): string
    --group-name(-N): string
    --group(-g): string
    --output(-o): string
    --flexible
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --no-headers(-n)
    --delimiter(-d): string
    --group-name(-N): string
    --group(-g): string
    --output(-o): string
    --flexible
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --no-headers(-n)
    --delimiter(-d): string
    --group-name(-N): string
    --group(-g): string
    --output(-o): string
    --flexible
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --no-headers(-n)
    --delimiter(-d): string
    --group-name(-N): string
    --group(-g): string
    --output(-o): string
    --flexible
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
    --output(-o): string
    --row-numbers(-n)
    --title(-t): string
    --delimiter(-d): string
    --memcheck
    --color(-C)
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --json
    --human-readable(-H)
    --delimiter(-d): string
    --width
    --width-no-delims
    --no-polars
    --low-memory
    --flexible(-f)
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --default-tz: string
    --formatstr: string
    --output-tz: string
    --output(-o): string
    --rename(-r): string
    --jobs(-j): string
    --prefer-dmy
    --ts-resolution(-R): string
    --utc
    --batch(-b): string
    --new-column(-c): string
    --input-tz: string
    --progressbar(-p)
    --keep-zero-time
    --zulu
    --delimiter(-d): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --jobs(-j): string
    --ignore-case(-i)
    --memcheck
    --quiet(-q)
    --output(-o): string
    --delimiter(-d): string
    --dupes-output(-D): string
    --select(-s): string
    --no-headers(-n)
    --sorted
    --numeric(-N)
    --human-readable(-H)
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --stats-options: string
    --addl-cols
    --export-prompt: string
    --forget
    --prepare-context
    --prompt(-p): string
    --cache-dir: string
    --sample-size: string
    --all(-A)
    --enum-threshold: string
    --session: string
    --score-threshold: string
    --user-agent: string
    --addl-cols-list: string
    --num-tags: string
    --no-cache
    --output(-o): string
    --quiet(-q)
    --session-len: string
    --max-tokens(-t): string
    --disk-cache-dir: string
    --model(-m): string
    --addl-props: string
    --prompt-file: string
    --base-url(-u): string
    --freq-options: string
    --format: string
    --process-response
    --score-max-retries: string
    --tag-vocab: string
    --no-score-sql
    --ckan-token: string
    --tags
    --fewshot-examples
    --timeout: string
    --language: string
    --flush-cache
    --num-examples: string
    --truncate-str: string
    --sql-results: string
    --dictionary
    --redis-cache
    --api-key(-k): string
    --description
    --ckan-api: string
    --fresh
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --output(-o): string
    --no-headers-output
    --key(-k): string
    --no-headers-left
    --delimiter-left: string
    --drop-equal-fields
    --delimiter-right: string
    --no-headers-right
    --sort-columns: string
    --delimiter(-d): string
    --delimiter-output: string
    --jobs(-j): string
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
    --increment: string
    --output(-o): string
    --start: string
    --copy: string
    --new-column(-c): string
    --uuid7
    --uuid4
    --no-headers(-n)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --metadata: string
    --cell: string
    --error-format: string
    --delimiter(-d): string
    --range: string
    --header-row: string
    --date-format: string
    --flexible
    --sheet(-s): string
    --jobs(-j): string
    --trim
    --table: string
    --output(-o): string
    --keep-zero-time
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --memcheck
    --output(-o): string
    --invert(-v)
    --ignore-case(-i)
    --no-headers(-n)
    --delimiter(-d): string
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
    --temp-dir: string
    --dupes-output(-D): string
    --human-readable(-H)
    --no-output
    --select(-s): string
    --quiet(-q)
    --no-headers(-n)
    --memory-limit: string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --jobs(-j): string
    --no-headers(-n)
    --select(-s): string
    --delimiter(-d): string
    --reverse(-R)
    --memory-limit: string
    --tmp-dir: string
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --report: string
    --url-template: string
    --cookies
    --rate-limit: string
    --cache-error
    --flush-cache
    --no-headers(-n)
    --http-header(-H): string
    --redis-cache
    --max-retries: string
    --new-column(-c): string
    --max-errors: string
    --no-cache
    --mem-cache-size: string
    --output(-o): string
    --disk-cache
    --jaq: string
    --store-error
    --progressbar(-p)
    --pretty
    --timeout: string
    --disk-cache-dir: string
    --user-agent: string
    --delimiter(-d): string
    --jaqfile: string
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --output(-o): string
    --globals-json(-j): string
    --max-errors: string
    --payload-tpl(-t): string
    --timeout: string
    --cache-error
    --content-type: string
    --redis-cache
    --user-agent: string
    --disk-cache
    --progressbar(-p)
    --http-header(-H): string
    --cookies
    --jaq: string
    --rate-limit: string
    --report: string
    --jaqfile: string
    --new-column(-c): string
    --no-headers(-n)
    --no-cache
    --disk-cache-dir: string
    --compress
    --store-error
    --max-retries: string
    --pretty
    --mem-cache-size: string
    --flush-cache
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --output(-o): string
    --no-headers(-n)
    --default(-v): string
    --delimiter(-d): string
    --first(-f)
    --groupby(-g): string
    --backfill(-b)
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --delimiter(-d): string
    --output(-o): string
    --quote: string
    --length(-l): string
    --quiet(-q)
    --insert(-i): string
    --escape: string
    --remove-empty(-r)
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --field-separator(-f): string
    --no-headers(-n)
    --separator(-s): string
    --condense(-c): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --delimiter(-d): string
    --crlf
    --output(-o): string
    --quote-never
    --quote-always
    --out-delimiter(-t): string
    --no-final-newline
    --quote: string
    --escape: string
    --ascii
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --progressbar(-p)
    --new-column(-c): string
    --dry-run: string
    --unify(-u)
    --no-headers(-n)
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --null-sorted
    --no-headers(-n)
    --no-nulls
    --other-text: string
    --memcheck
    --pretty-json
    --no-stats
    --stats-filter: string
    --json
    --other-sorted
    --ignore-case(-i)
    --lmt-threshold: string
    --limit(-l): string
    --no-other
    --all-unique-text: string
    --jobs(-j): string
    --high-card-threshold: string
    --weight: string
    --output(-o): string
    --delimiter(-d): string
    --pct-nulls
    --frequency-jsonl
    --rank-strategy(-r): string
    --no-trim
    --force
    --toon
    --pct-dec-places: string
    --select(-s): string
    --asc(-a)
    --unq-limit(-u): string
    --null-text: string
    --no-float: string
    --high-card-pct: string
    --vis-whitespace
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfo" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfonow" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-check" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-load" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-reset" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-update" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookup" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookupnow" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reverse" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reversenow" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggest" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggestnow" [
    --min-score: string
    --new-column(-c): string
    --invalid-result: string
    --timeout: string
    --admin1: string
    --languages: string
    --cache-dir: string
    --country: string
    --progressbar(-p)
    --force
    --cities-url: string
    --delimiter(-d): string
    --k_weight(-k): string
    --jobs(-j): string
    --rename(-r): string
    --language(-l): string
    --batch(-b): string
    --formatstr(-f): string
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
    --geometry(-g): string
    --max-length(-l): string
    --output(-o): string
    --latitude(-y): string
    --help(-h)                # Print help
  ]

  export extern "qsv headers" [
    --delimiter(-d): string
    --just-names(-j)
    --trim
    --union
    --just-count(-J)
    --help(-h)                # Print help
  ]

  export extern "qsv implode" [
    --no-headers(-n)
    --skip-empty
    --keys(-k): string
    --output(-o): string
    --rename(-r): string
    --sorted
    --delimiter(-d): string
    --value(-v): string
    --help(-h)                # Print help
  ]

  export extern "qsv index" [
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv input" [
    --skip-lastlines: string
    --encoding-errors: string
    --quote-style: string
    --escape: string
    --no-quoting
    --quote: string
    --auto-skip
    --trim-headers
    --delimiter(-d): string
    --comment: string
    --skip-lines: string
    --output(-o): string
    --trim-fields
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --right
    --right-semi
    --keys-output: string
    --output(-o): string
    --left-anti
    --delimiter(-d): string
    --ignore-leading-zeros(-z)
    --left-semi
    --left
    --full
    --cross
    --nulls
    --ignore-case(-i)
    --right-anti
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --left-anti
    --left-semi
    --strategy: string
    --datetime-format: string
    --decimal-comma
    --date-format: string
    --ignore-leading-zeros(-z)
    --quiet(-q)
    --low-memory
    --maintain-order: string
    --infer-len: string
    --nulls
    --left
    --ignore-case(-i)
    --tolerance: string
    --right_by: string
    --right
    --sql-filter: string
    --non-equi: string
    --asof
    --coalesce
    --right-semi
    --cache-schema: string
    --right-anti
    --full
    --left_by: string
    --null-value: string
    --no-sort
    --filter-left: string
    --cross
    --ignore-errors
    --float-precision: string
    --norm-unicode(-N): string
    --filter-right: string
    --time-format: string
    --streaming
    --try-parsedates
    --no-optimizations
    --delimiter(-d): string
    --output(-o): string
    --allow-exact-matches(-X)
    --validate: string
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --output(-o): string
    --jaq: string
    --select(-s): string
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
    --tab-separated(-t)
    --filter: string
    --echo-column: string
    --ignore-case(-i)
    --streaming-stdin(-S)
    --columns: string
    --delimiter(-d): string
    --find: string
    --monochrome(-m)
    --freeze-columns(-f): string
    --wrap-mode(-W): string
    --debug
    --prompt(-P): string
    --auto-reload(-A)
    --help(-h)                # Print help
  ]

  export extern "qsv log" [
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --output(-o): string
    --colindex
    --delimiter(-d): string
    --timeout: string
    --cache-dir: string
    --end(-E): string
    --no-headers(-n)
    --remap(-r)
    --no-globals(-g)
    --begin(-B): string
    --progressbar(-p)
    --max-errors: string
    --ckan-token: string
    --ckan-api: string
    --help(-h)                # Print help
  ]

  export extern "qsv luau filter" [
    --output(-o): string
    --colindex
    --delimiter(-d): string
    --timeout: string
    --cache-dir: string
    --end(-E): string
    --no-headers(-n)
    --remap(-r)
    --no-globals(-g)
    --begin(-B): string
    --progressbar(-p)
    --max-errors: string
    --ckan-token: string
    --ckan-api: string
    --help(-h)                # Print help
  ]

  export extern "qsv luau map" [
    --output(-o): string
    --colindex
    --delimiter(-d): string
    --timeout: string
    --cache-dir: string
    --end(-E): string
    --no-headers(-n)
    --remap(-r)
    --no-globals(-g)
    --begin(-B): string
    --progressbar(-p)
    --max-errors: string
    --ckan-token: string
    --ckan-api: string
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
    --join-keys(-K): string
    --xsd-gdate-scan: string
    --bivariate(-B)
    --force
    --round: string
    --use-percentiles
    --output(-o): string
    --jobs(-j): string
    --progressbar(-p)
    --epsilon(-e): string
    --stats-options: string
    --bivariate-stats(-S): string
    --pct-thresholds: string
    --cardinality-threshold(-C): string
    --advanced
    --join-inputs(-J): string
    --join-type(-T): string
    --help(-h)                # Print help
  ]

  export extern "qsv partition" [
    --limit: string
    --no-headers(-n)
    --delimiter(-d): string
    --filename: string
    --prefix-length(-p): string
    --drop
    --help(-h)                # Print help
  ]

  export extern "qsv pivotp" [
    --decimal-comma
    --grand-total
    --index(-i): string
    --agg(-a): string
    --output(-o): string
    --try-parsedates
    --quiet(-q)
    --validate
    --values(-v): string
    --total-label: string
    --subtotal
    --sort-columns
    --maintain-order
    --infer-len: string
    --col-separator: string
    --ignore-errors
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv pragmastat" [
    --no-headers(-n)
    --select(-s): string
    --round: string
    --twosample(-t)
    --no-bounds
    --misrate(-m): string
    --compare1: string
    --force
    --stats-options: string
    --subsample: string
    --jobs(-j): string
    --memcheck
    --standalone
    --seed: string
    --output(-o): string
    --compare2: string
    --delimiter(-d): string
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
    --quiet(-q)
    --save-fname: string
    --workdir(-d): string
    --base-delay-ms: string
    --filters(-F): string
    --fd-output(-f)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --formatstr: string
    --no-headers(-n)
    --output(-o): string
    --start: string
    --increment: string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --batch(-b): string
    --progressbar(-p)
    --output(-o): string
    --no-headers(-n)
    --helper(-f): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv py filter" [
    --batch(-b): string
    --progressbar(-p)
    --output(-o): string
    --no-headers(-n)
    --helper(-f): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv py map" [
    --batch(-b): string
    --progressbar(-p)
    --output(-o): string
    --no-headers(-n)
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
    --output(-o): string
    --pairwise
    --delimiter(-d): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --size-limit: string
    --jobs(-j): string
    --literal
    --not-one
    --quiet(-q)
    --delimiter(-d): string
    --select(-s): string
    --exact
    --output(-o): string
    --no-headers(-n)
    --ignore-case(-i)
    --dfa-size-limit: string
    --progressbar(-p)
    --unicode(-u)
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
    --prefix: string
    --mode: string
    --output(-o): string
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --ts-input-tz: string
    --weighted: string
    --no-headers(-n)
    --bernoulli
    --systematic: string
    --timeout: string
    --ts-aggregate: string
    --max-size: string
    --ts-prefer-dmy
    --rng: string
    --ts-interval: string
    --output(-o): string
    --delimiter(-d): string
    --seed: string
    --ts-start: string
    --ts-adaptive: string
    --stratified: string
    --timeseries: string
    --user-agent: string
    --cluster: string
    --force
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --delimiter(-d): string
    --polars
    --strict-formats
    --enum-threshold: string
    --ignore-case(-i)
    --output(-o): string
    --memcheck
    --strict-dates
    --dates-whitelist: string
    --prefer-dmy
    --pattern-columns: string
    --force
    --stdout
    --jobs(-j): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv scoresql" [
    --try-parsedates
    --json
    --truncate-ragged-lines
    --infer-len: string
    --ignore-errors
    --duckdb
    --output(-o): string
    --delimiter(-d): string
    --quiet(-q)
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --quick(-Q)
    --not-one
    --jobs(-j): string
    --unicode(-u)
    --invert-match(-v)
    --select(-s): string
    --flag(-f): string
    --size-limit: string
    --output(-o): string
    --progressbar(-p)
    --preview-match: string
    --json
    --dfa-size-limit: string
    --quiet(-q)
    --no-headers(-n)
    --exact
    --delimiter(-d): string
    --ignore-case(-i)
    --literal
    --count(-c)
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --flag-matches-only
    --count(-c)
    --no-headers(-n)
    --progressbar(-p)
    --invert-match(-v)
    --quick(-Q)
    --dfa-size-limit: string
    --json(-j)
    --select(-s): string
    --unmatched-output: string
    --size-limit: string
    --flag(-f): string
    --literal
    --quiet(-q)
    --delimiter(-d): string
    --jobs: string
    --ignore-case(-i)
    --not-one
    --output(-o): string
    --unicode(-u)
    --exact
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --random(-R)
    --seed: string
    --output(-o): string
    --sort(-S)
    --delimiter(-d): string
    --no-headers(-n)
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --start(-s): string
    --end(-e): string
    --index(-i): string
    --json
    --invert
    --delimiter(-d): string
    --len(-l): string
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --user-agent: string
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --progressbar(-p)
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --user-agent: string
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --progressbar(-p)
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --user-agent: string
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --progressbar(-p)
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --user-agent: string
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --progressbar(-p)
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --user-agent: string
    --output(-o): string
    --quiet(-q)
    --jobs(-j): string
    --progressbar(-p)
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
    --timeout: string
    --quick(-Q)
    --user-agent: string
    --no-infer
    --progressbar(-p)
    --save-urlsample: string
    --pretty-json
    --prefer-dmy
    --delimiter(-d): string
    --just-mime
    --quote: string
    --sample: string
    --stats-types
    --json
    --harvest-mode
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --delimiter(-d): string
    --unique(-u)
    --select(-s): string
    --rng: string
    --memcheck
    --reverse(-R)
    --faster
    --numeric(-N)
    --ignore-case(-i)
    --natural
    --output(-o): string
    --jobs(-j): string
    --no-headers(-n)
    --random
    --seed: string
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --ignore-case(-i)
    --no-headers(-n)
    --select(-s): string
    --delimiter(-d): string
    --numeric(-N)
    --natural
    --progressbar(-p)
    --pretty-json
    --json
    --all
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --quiet(-q)
    --jobs(-j): string
    --pad: string
    --size(-s): string
    --chunks(-c): string
    --filename: string
    --filter: string
    --no-headers(-n)
    --kb-size(-k): string
    --filter-cleanup
    --filter-ignore-errors
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --delimiter(-d): string
    --quiet(-q)
    --cache-schema
    --decimal-comma
    --compress-level: string
    --no-optimizations
    --statistics
    --output(-o): string
    --try-parsedates
    --ignore-errors
    --float-precision: string
    --low-memory
    --datetime-format: string
    --truncate-ragged-lines
    --infer-len: string
    --rnull-values: string
    --format: string
    --streaming
    --date-format: string
    --time-format: string
    --wnull-value: string
    --compression: string
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --mad
    --output(-o): string
    --select(-s): string
    --everything(-E)
    --dates-whitelist: string
    --typesonly
    --cardinality
    --infer-boolean
    --no-headers(-n)
    --infer-dates
    --delimiter(-d): string
    --nulls
    --percentile-list: string
    --stats-jsonl
    --prefer-dmy
    --boolean-patterns: string
    --jobs(-j): string
    --weight: string
    --percentiles
    --cache-threshold(-c): string
    --memcheck
    --median
    --mode
    --round: string
    --force
    --vis-whitespace
    --quartiles
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --pad(-p): string
    --output(-o): string
    --align(-a): string
    --memcheck
    --delimiter(-d): string
    --condense(-c): string
    --width(-w): string
    --help(-h)                # Print help
  ]

  export extern "qsv template" [
    --outsubdir-size: string
    --globals-json(-J): string
    --customfilter-error: string
    --progressbar(-p)
    --template: string
    --ckan-token: string
    --cache-dir: string
    --outfilename: string
    --no-headers(-n)
    --output(-o): string
    --jobs(-j): string
    --batch(-b): string
    --delimiter: string
    --template-file(-t): string
    --timeout: string
    --ckan-api: string
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --evolve(-e)
    --stats-csv(-c): string
    --stats(-a)
    --drop(-d)
    --pipe(-i)
    --all-strings(-A)
    --table(-t): string
    --jobs(-j): string
    --infer-len: string
    --dump(-u)
    --try-parse-dates
    --quiet(-q)
    --schema(-s): string
    --compress-level: string
    --delimiter: string
    --compression: string
    --print-package(-k)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to datapackage" [
    --evolve(-e)
    --stats-csv(-c): string
    --stats(-a)
    --drop(-d)
    --pipe(-i)
    --all-strings(-A)
    --table(-t): string
    --jobs(-j): string
    --infer-len: string
    --dump(-u)
    --try-parse-dates
    --quiet(-q)
    --schema(-s): string
    --compress-level: string
    --delimiter: string
    --compression: string
    --print-package(-k)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to ods" [
    --evolve(-e)
    --stats-csv(-c): string
    --stats(-a)
    --drop(-d)
    --pipe(-i)
    --all-strings(-A)
    --table(-t): string
    --jobs(-j): string
    --infer-len: string
    --dump(-u)
    --try-parse-dates
    --quiet(-q)
    --schema(-s): string
    --compress-level: string
    --delimiter: string
    --compression: string
    --print-package(-k)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to parquet" [
    --evolve(-e)
    --stats-csv(-c): string
    --stats(-a)
    --drop(-d)
    --pipe(-i)
    --all-strings(-A)
    --table(-t): string
    --jobs(-j): string
    --infer-len: string
    --dump(-u)
    --try-parse-dates
    --quiet(-q)
    --schema(-s): string
    --compress-level: string
    --delimiter: string
    --compression: string
    --print-package(-k)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to postgres" [
    --evolve(-e)
    --stats-csv(-c): string
    --stats(-a)
    --drop(-d)
    --pipe(-i)
    --all-strings(-A)
    --table(-t): string
    --jobs(-j): string
    --infer-len: string
    --dump(-u)
    --try-parse-dates
    --quiet(-q)
    --schema(-s): string
    --compress-level: string
    --delimiter: string
    --compression: string
    --print-package(-k)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to sqlite" [
    --evolve(-e)
    --stats-csv(-c): string
    --stats(-a)
    --drop(-d)
    --pipe(-i)
    --all-strings(-A)
    --table(-t): string
    --jobs(-j): string
    --infer-len: string
    --dump(-u)
    --try-parse-dates
    --quiet(-q)
    --schema(-s): string
    --compress-level: string
    --delimiter: string
    --compression: string
    --print-package(-k)
    --separator(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv to xlsx" [
    --evolve(-e)
    --stats-csv(-c): string
    --stats(-a)
    --drop(-d)
    --pipe(-i)
    --all-strings(-A)
    --table(-t): string
    --jobs(-j): string
    --infer-len: string
    --dump(-u)
    --try-parse-dates
    --quiet(-q)
    --schema(-s): string
    --compress-level: string
    --delimiter: string
    --compression: string
    --print-package(-k)
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
    --batch(-b): string
    --jobs(-j): string
    --quiet(-q)
    --memcheck
    --output(-o): string
    --no-boolean
    --delimiter(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --long: string
    --output(-o): string
    --delimiter(-d): string
    --memcheck
    --multipass(-m)
    --select(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --invalid: string
    --timeout: string
    --json
    --valid-output: string
    --ckan-api: string
    --trim
    --quiet(-q)
    --valid: string
    --email-domain-literal
    --email-min-subdomains: string
    --pretty-json
    --ckan-token: string
    --batch(-b): string
    --delimiter(-d): string
    --email-display-text
    --backtrack-limit: string
    --no-headers(-n)
    --fail-fast
    --size-limit: string
    --cache-dir: string
    --dfa-size-limit: string
    --no-format-validation
    --fancy-regex
    --progressbar(-p)
    --jobs(-j): string
    --email-required-tld
    --help(-h)                # Print help
  ]

  export extern "qsv validate schema" [
    --invalid: string
    --timeout: string
    --json
    --valid-output: string
    --ckan-api: string
    --trim
    --quiet(-q)
    --valid: string
    --email-domain-literal
    --email-min-subdomains: string
    --pretty-json
    --ckan-token: string
    --batch(-b): string
    --delimiter(-d): string
    --email-display-text
    --backtrack-limit: string
    --no-headers(-n)
    --fail-fast
    --size-limit: string
    --cache-dir: string
    --dfa-size-limit: string
    --no-format-validation
    --fancy-regex
    --progressbar(-p)
    --jobs(-j): string
    --email-required-tld
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

  export extern "qsv help implode" [
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
