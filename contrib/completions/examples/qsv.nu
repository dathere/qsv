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
    --batch(-b): string
    --comparand(-C): string
    --delimiter(-d): string
    --formatstr(-f): string
    --jobs(-j): string
    --new-column(-c): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --rename(-r): string
    --replacement(-R): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --batch(-b): string
    --comparand(-C): string
    --delimiter(-d): string
    --formatstr(-f): string
    --jobs(-j): string
    --new-column(-c): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --rename(-r): string
    --replacement(-R): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --batch(-b): string
    --comparand(-C): string
    --delimiter(-d): string
    --formatstr(-f): string
    --jobs(-j): string
    --new-column(-c): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --rename(-r): string
    --replacement(-R): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --batch(-b): string
    --comparand(-C): string
    --delimiter(-d): string
    --formatstr(-f): string
    --jobs(-j): string
    --new-column(-c): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --rename(-r): string
    --replacement(-R): string
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --batch(-b): string
    --comparand(-C): string
    --delimiter(-d): string
    --formatstr(-f): string
    --jobs(-j): string
    --new-column(-c): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --rename(-r): string
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
    --check(-c)
    --derive-key: string
    --jobs(-j): string
    --keyed
    --length(-l): string
    --no-mmap
    --no-names
    --output(-o): string
    --quiet(-q)
    --raw
    --tag
    --help(-h)                # Print help
  ]

  export extern "qsv cat" [
    --delimiter(-d): string
    --flexible
    --group(-g): string
    --group-name(-N): string
    --no-headers(-n)
    --output(-o): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --delimiter(-d): string
    --flexible
    --group(-g): string
    --group-name(-N): string
    --no-headers(-n)
    --output(-o): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --delimiter(-d): string
    --flexible
    --group(-g): string
    --group-name(-N): string
    --no-headers(-n)
    --output(-o): string
    --pad(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --delimiter(-d): string
    --flexible
    --group(-g): string
    --group-name(-N): string
    --no-headers(-n)
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
    --color(-C)
    --delimiter(-d): string
    --memcheck
    --output(-o): string
    --row-numbers(-n)
    --title(-t): string
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --delimiter(-d): string
    --flexible(-f)
    --human-readable(-H)
    --json
    --low-memory
    --no-headers(-n)
    --no-polars
    --width
    --width-no-delims
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --batch(-b): string
    --default-tz: string
    --delimiter(-d): string
    --formatstr: string
    --input-tz: string
    --jobs(-j): string
    --keep-zero-time
    --new-column(-c): string
    --no-headers(-n)
    --output(-o): string
    --output-tz: string
    --prefer-dmy
    --progressbar(-p)
    --rename(-r): string
    --ts-resolution(-R): string
    --utc
    --zulu
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --delimiter(-d): string
    --dupes-output(-D): string
    --human-readable(-H)
    --ignore-case(-i)
    --jobs(-j): string
    --memcheck
    --no-headers(-n)
    --numeric(-N)
    --output(-o): string
    --quiet(-q)
    --select(-s): string
    --sorted
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --addl-cols
    --addl-cols-list: string
    --addl-props: string
    --all(-A)
    --allow-extra-cols
    --api-key(-k): string
    --base-url(-u): string
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --description
    --dictionary
    --disk-cache-dir: string
    --ds-license: string
    --ds-source: string
    --ds-updated: string
    --enum-threshold: string
    --export-prompt: string
    --fewshot-examples
    --flush-cache
    --forget
    --format: string
    --freq-options: string
    --fresh
    --infer-content-type
    --language: string
    --markdown-template: string
    --max-tokens(-t): string
    --model(-m): string
    --no-cache
    --no-score-sql
    --num-examples: string
    --num-tags: string
    --output(-o): string
    --prepare-context
    --process-response
    --prompt(-p): string
    --prompt-file: string
    --quiet(-q)
    --redis-cache
    --sample-size: string
    --score-max-retries: string
    --score-threshold: string
    --session: string
    --session-len: string
    --sql-results: string
    --stats-options: string
    --strict-dates
    --tag-vocab: string
    --tags
    --timeout: string
    --truncate-str: string
    --two-pass
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --delimiter(-d): string
    --delimiter-left: string
    --delimiter-output: string
    --delimiter-right: string
    --drop-equal-fields
    --jobs(-j): string
    --key(-k): string
    --no-headers-left
    --no-headers-output
    --no-headers-right
    --output(-o): string
    --sort-columns: string
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
    --copy: string
    --delimiter(-d): string
    --hash: string
    --increment: string
    --new-column(-c): string
    --no-headers(-n)
    --output(-o): string
    --start: string
    --uuid4
    --uuid7
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --cell: string
    --date-format: string
    --delimiter(-d): string
    --error-format: string
    --flexible
    --header-row: string
    --jobs(-j): string
    --keep-zero-time
    --metadata: string
    --output(-o): string
    --quiet(-q)
    --range: string
    --sheet(-s): string
    --table: string
    --trim
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --delimiter(-d): string
    --ignore-case(-i)
    --invert(-v)
    --memcheck
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv explode" [
    --delimiter(-d): string
    --no-headers(-n)
    --output(-o): string
    --rename(-r): string
    --help(-h)                # Print help
  ]

  export extern "qsv extdedup" [
    --delimiter(-d): string
    --dupes-output(-D): string
    --human-readable(-H)
    --memory-limit: string
    --no-headers(-n)
    --no-output
    --quiet(-q)
    --select(-s): string
    --temp-dir: string
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --delimiter(-d): string
    --jobs(-j): string
    --memory-limit: string
    --no-headers(-n)
    --reverse(-R)
    --select(-s): string
    --tmp-dir: string
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --cache-error
    --cookies
    --delimiter(-d): string
    --disk-cache
    --disk-cache-dir: string
    --flush-cache
    --http-header(-H): string
    --jaq: string
    --jaqfile: string
    --max-errors: string
    --max-retries: string
    --mem-cache-size: string
    --new-column(-c): string
    --no-cache
    --no-headers(-n)
    --output(-o): string
    --pretty
    --progressbar(-p)
    --rate-limit: string
    --redis-cache
    --report: string
    --store-error
    --timeout: string
    --url-template: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --cache-error
    --compress
    --content-type: string
    --cookies
    --delimiter(-d): string
    --disk-cache
    --disk-cache-dir: string
    --flush-cache
    --globals-json(-j): string
    --http-header(-H): string
    --jaq: string
    --jaqfile: string
    --max-errors: string
    --max-retries: string
    --mem-cache-size: string
    --new-column(-c): string
    --no-cache
    --no-headers(-n)
    --output(-o): string
    --payload-tpl(-t): string
    --pretty
    --progressbar(-p)
    --rate-limit: string
    --redis-cache
    --report: string
    --store-error
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --backfill(-b)
    --default(-v): string
    --delimiter(-d): string
    --first(-f)
    --groupby(-g): string
    --no-headers(-n)
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --delimiter(-d): string
    --escape: string
    --insert(-i): string
    --length(-l): string
    --output(-o): string
    --quiet(-q)
    --quote: string
    --remove-empty(-r)
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --condense(-c): string
    --delimiter(-d): string
    --field-separator(-f): string
    --no-headers(-n)
    --separator(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --ascii
    --crlf
    --delimiter(-d): string
    --escape: string
    --no-final-newline
    --out-delimiter(-t): string
    --output(-o): string
    --quote: string
    --quote-always
    --quote-never
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --delimiter(-d): string
    --dry-run: string
    --new-column(-c): string
    --no-headers(-n)
    --progressbar(-p)
    --unify(-u)
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --all-unique-text: string
    --asc(-a)
    --delimiter(-d): string
    --force
    --frequency-jsonl
    --high-card-pct: string
    --high-card-threshold: string
    --ignore-case(-i)
    --jobs(-j): string
    --json
    --limit(-l): string
    --lmt-threshold: string
    --memcheck
    --no-float: string
    --no-headers(-n)
    --no-nulls
    --no-other
    --no-stats
    --no-trim
    --null-sorted
    --null-text: string
    --other-sorted
    --other-text: string
    --output(-o): string
    --pct-dec-places: string
    --pct-nulls
    --pretty-json
    --rank-strategy(-r): string
    --select(-s): string
    --sketch-map-size: string
    --sketch-method: string
    --stats-filter: string
    --toon
    --unq-limit(-u): string
    --vis-whitespace
    --weight: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode cache-clear" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode cache-info" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode cache-prune" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfo" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfonow" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-check" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-load" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-reset" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-update" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookup" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode iplookupnow" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode opencage" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode opencagenow" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reverse" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reversenow" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggest" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggestnow" [
    --admin1: string
    --api-key: string
    --batch(-b): string
    --cache-dir: string
    --cache-ttl: string
    --cities-url: string
    --country: string
    --delimiter(-d): string
    --force
    --formatstr(-f): string
    --invalid-result: string
    --jobs(-j): string
    --k_weight(-k): string
    --language(-l): string
    --languages: string
    --min-score: string
    --new-column(-c): string
    --no-annotations
    --no-cache
    --older-than: string
    --output(-o): string
    --progressbar(-p)
    --rate-limit: string
    --rename(-r): string
    --reverse
    --timeout: string
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv geocode help" [
  ]

  export extern "qsv geocode help cache-clear" [
  ]

  export extern "qsv geocode help cache-info" [
  ]

  export extern "qsv geocode help cache-prune" [
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

  export extern "qsv geocode help opencage" [
  ]

  export extern "qsv geocode help opencagenow" [
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
    --geometry(-g): string
    --latitude(-y): string
    --longitude(-x): string
    --max-length(-l): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv get" [
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --cloud-opt: string
    --compress: string
    --force
    --json
    --name: string
    --older-than: string
    --output(-o): string
    --quiet(-q)
    --refresh: string
    --timeout: string
    --ttl: string
    --verify
    --help(-h)                # Print help
  ]

  export extern "qsv get cache-clear" [
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --cloud-opt: string
    --compress: string
    --force
    --json
    --name: string
    --older-than: string
    --output(-o): string
    --quiet(-q)
    --refresh: string
    --timeout: string
    --ttl: string
    --verify
    --help(-h)                # Print help
  ]

  export extern "qsv get cache-info" [
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --cloud-opt: string
    --compress: string
    --force
    --json
    --name: string
    --older-than: string
    --output(-o): string
    --quiet(-q)
    --refresh: string
    --timeout: string
    --ttl: string
    --verify
    --help(-h)                # Print help
  ]

  export extern "qsv get cache-list" [
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --cloud-opt: string
    --compress: string
    --force
    --json
    --name: string
    --older-than: string
    --output(-o): string
    --quiet(-q)
    --refresh: string
    --timeout: string
    --ttl: string
    --verify
    --help(-h)                # Print help
  ]

  export extern "qsv get cache-prune" [
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --cloud-opt: string
    --compress: string
    --force
    --json
    --name: string
    --older-than: string
    --output(-o): string
    --quiet(-q)
    --refresh: string
    --timeout: string
    --ttl: string
    --verify
    --help(-h)                # Print help
  ]

  export extern "qsv get cache-set-policy" [
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --cloud-opt: string
    --compress: string
    --force
    --json
    --name: string
    --older-than: string
    --output(-o): string
    --quiet(-q)
    --refresh: string
    --timeout: string
    --ttl: string
    --verify
    --help(-h)                # Print help
  ]

  export extern "qsv get cache-set-ttl" [
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --cloud-opt: string
    --compress: string
    --force
    --json
    --name: string
    --older-than: string
    --output(-o): string
    --quiet(-q)
    --refresh: string
    --timeout: string
    --ttl: string
    --verify
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv get help" [
  ]

  export extern "qsv get help cache-clear" [
  ]

  export extern "qsv get help cache-info" [
  ]

  export extern "qsv get help cache-list" [
  ]

  export extern "qsv get help cache-prune" [
  ]

  export extern "qsv get help cache-set-policy" [
  ]

  export extern "qsv get help cache-set-ttl" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv get help help" [
  ]

  export extern "qsv headers" [
    --delimiter(-d): string
    --just-count(-J)
    --just-names(-j)
    --trim
    --union
    --help(-h)                # Print help
  ]

  export extern "qsv implode" [
    --delimiter(-d): string
    --keys(-k): string
    --no-headers(-n)
    --output(-o): string
    --rename(-r): string
    --skip-empty
    --sorted
    --value(-v): string
    --help(-h)                # Print help
  ]

  export extern "qsv index" [
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv input" [
    --auto-skip
    --comment: string
    --delimiter(-d): string
    --encoding-errors: string
    --escape: string
    --no-quoting
    --output(-o): string
    --quote: string
    --quote-style: string
    --skip-lastlines: string
    --skip-lines: string
    --trim-fields
    --trim-headers
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --cross
    --delimiter(-d): string
    --full
    --ignore-case(-i)
    --ignore-leading-zeros(-z)
    --keys-output: string
    --left
    --left-anti
    --left-semi
    --no-headers(-n)
    --nulls
    --output(-o): string
    --right
    --right-anti
    --right-semi
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --allow-exact-matches(-X)
    --asof
    --cache-schema: string
    --coalesce
    --cross
    --date-format: string
    --datetime-format: string
    --decimal-comma
    --delimiter(-d): string
    --filter-left: string
    --filter-right: string
    --float-precision: string
    --full
    --ignore-case(-i)
    --ignore-errors
    --ignore-leading-zeros(-z)
    --infer-len: string
    --left
    --left-anti
    --left-semi
    --left_by: string
    --low-memory
    --maintain-order: string
    --no-optimizations
    --no-sort
    --non-equi: string
    --norm-unicode(-N): string
    --null-value: string
    --nulls
    --output(-o): string
    --quiet(-q)
    --right
    --right-anti
    --right-semi
    --right_by: string
    --sql-filter: string
    --strategy: string
    --streaming
    --time-format: string
    --tolerance: string
    --try-parsedates
    --validate: string
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --jaq: string
    --output(-o): string
    --select(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv jsonl" [
    --batch(-b): string
    --delimiter(-d): string
    --ignore-errors
    --jobs(-j): string
    --output(-o): string
    --help(-h)                # Print help
  ]

  export extern "qsv lens" [
    --auto-reload(-A)
    --columns: string
    --debug
    --delimiter(-d): string
    --echo-column: string
    --filter: string
    --find: string
    --freeze-columns(-f): string
    --ignore-case(-i)
    --monochrome(-m)
    --no-headers
    --prompt(-P): string
    --streaming-stdin(-S)
    --tab-separated(-t)
    --wrap-mode(-W): string
    --help(-h)                # Print help
  ]

  export extern "qsv log" [
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --begin(-B): string
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --colindex
    --delimiter(-d): string
    --end(-E): string
    --max-errors: string
    --no-globals(-g)
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --remap(-r)
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv luau filter" [
    --begin(-B): string
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --colindex
    --delimiter(-d): string
    --end(-E): string
    --max-errors: string
    --no-globals(-g)
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --remap(-r)
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv luau map" [
    --begin(-B): string
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --colindex
    --delimiter(-d): string
    --end(-E): string
    --max-errors: string
    --no-globals(-g)
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --remap(-r)
    --timeout: string
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
    --advanced
    --bivariate(-B)
    --bivariate-stats(-S): string
    --cardinality-threshold(-C): string
    --epsilon(-e): string
    --force
    --jobs(-j): string
    --join-inputs(-J): string
    --join-keys(-K): string
    --join-type(-T): string
    --output(-o): string
    --pct-thresholds: string
    --progressbar(-p)
    --round: string
    --stats-options: string
    --use-percentiles
    --xsd-gdate-scan: string
    --help(-h)                # Print help
  ]

  export extern "qsv partition" [
    --delimiter(-d): string
    --drop
    --filename: string
    --limit: string
    --no-headers(-n)
    --prefix-length(-p): string
    --help(-h)                # Print help
  ]

  export extern "qsv pivotp" [
    --agg(-a): string
    --col-separator: string
    --decimal-comma
    --delimiter(-d): string
    --grand-total
    --ignore-errors
    --index(-i): string
    --infer-len: string
    --maintain-order
    --output(-o): string
    --quiet(-q)
    --sort-columns
    --subtotal
    --total-label: string
    --try-parsedates
    --validate
    --values(-v): string
    --help(-h)                # Print help
  ]

  export extern "qsv pragmastat" [
    --compare1: string
    --compare2: string
    --delimiter(-d): string
    --force
    --jobs(-j): string
    --memcheck
    --misrate(-m): string
    --no-bounds
    --no-headers(-n)
    --output(-o): string
    --round: string
    --seed: string
    --select(-s): string
    --standalone
    --stats-options: string
    --subsample: string
    --twosample(-t)
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

  export extern "qsv profile" [
    --allow-external-validator
    --catalog
    --croissant-frequency
    --dcat-discovery-timeout: string
    --dcat-legacy-license
    --delimiter(-d): string
    --force
    --initial-context: string
    --jobs(-j): string
    --memcheck
    --no-ckan
    --no-dcat-discovery
    --no-headers(-n)
    --no-projection
    --output(-o): string
    --profile: string
    --spec: string
    --strict
    --validate
    --help(-h)                # Print help
  ]

  export extern "qsv prompt" [
    --base-delay-ms: string
    --fd-output(-f)
    --filters(-F): string
    --msg(-m): string
    --output(-o): string
    --quiet(-q)
    --save-fname: string
    --workdir(-d): string
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --delimiter(-d): string
    --formatstr: string
    --increment: string
    --no-headers(-n)
    --output(-o): string
    --start: string
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --batch(-b): string
    --delimiter(-d): string
    --helper(-f): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv py filter" [
    --batch(-b): string
    --delimiter(-d): string
    --helper(-f): string
    --no-headers(-n)
    --output(-o): string
    --progressbar(-p)
    --help(-h)                # Print help
  ]

  export extern "qsv py map" [
    --batch(-b): string
    --delimiter(-d): string
    --helper(-f): string
    --no-headers(-n)
    --output(-o): string
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
    --no-headers(-n)
    --output(-o): string
    --pairwise
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --delimiter(-d): string
    --dfa-size-limit: string
    --exact
    --ignore-case(-i)
    --jobs(-j): string
    --literal
    --no-headers(-n)
    --not-one
    --output(-o): string
    --progressbar(-p)
    --quiet(-q)
    --select(-s): string
    --size-limit: string
    --unicode(-u)
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
    --delimiter(-d): string
    --mode: string
    --output(-o): string
    --prefix: string
    --reserved: string
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --bernoulli
    --cluster: string
    --delimiter(-d): string
    --force
    --max-size: string
    --mergeable-reservoir
    --no-headers(-n)
    --output(-o): string
    --rng: string
    --seed: string
    --sketch-in: string
    --sketch-out: string
    --stratified: string
    --systematic: string
    --timeout: string
    --timeseries: string
    --ts-adaptive: string
    --ts-aggregate: string
    --ts-input-tz: string
    --ts-interval: string
    --ts-prefer-dmy
    --ts-start: string
    --user-agent: string
    --varopt: string
    --weighted: string
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --dates-whitelist: string
    --delimiter(-d): string
    --enum-threshold: string
    --force
    --ignore-case(-i)
    --jobs(-j): string
    --memcheck
    --no-headers(-n)
    --output(-o): string
    --pattern-columns: string
    --polars
    --prefer-dmy
    --stdout
    --strict-dates
    --strict-formats
    --help(-h)                # Print help
  ]

  export extern "qsv scoresql" [
    --delimiter(-d): string
    --duckdb
    --ignore-errors
    --infer-len: string
    --json
    --output(-o): string
    --quiet(-q)
    --truncate-ragged-lines
    --try-parsedates
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --count(-c)
    --delimiter(-d): string
    --dfa-size-limit: string
    --exact
    --flag(-f): string
    --ignore-case(-i)
    --invert-match(-v)
    --jobs(-j): string
    --json
    --literal
    --no-headers(-n)
    --not-one
    --output(-o): string
    --preview-match: string
    --progressbar(-p)
    --quick(-Q)
    --quiet(-q)
    --select(-s): string
    --size-limit: string
    --unicode(-u)
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --count(-c)
    --delimiter(-d): string
    --dfa-size-limit: string
    --exact
    --flag(-f): string
    --flag-matches-only
    --ignore-case(-i)
    --invert-match(-v)
    --jobs: string
    --json(-j)
    --literal
    --no-headers(-n)
    --not-one
    --output(-o): string
    --progressbar(-p)
    --quick(-Q)
    --quiet(-q)
    --select(-s): string
    --size-limit: string
    --unicode(-u)
    --unmatched-output: string
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --delimiter(-d): string
    --no-headers(-n)
    --output(-o): string
    --random(-R)
    --seed: string
    --sort(-S)
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --delimiter(-d): string
    --end(-e): string
    --index(-i): string
    --invert
    --json
    --len(-l): string
    --no-headers(-n)
    --output(-o): string
    --start(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --jobs(-j): string
    --output(-o): string
    --progressbar(-p)
    --quiet(-q)
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --jobs(-j): string
    --output(-o): string
    --progressbar(-p)
    --quiet(-q)
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --jobs(-j): string
    --output(-o): string
    --progressbar(-p)
    --quiet(-q)
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --jobs(-j): string
    --output(-o): string
    --progressbar(-p)
    --quiet(-q)
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --jobs(-j): string
    --output(-o): string
    --progressbar(-p)
    --quiet(-q)
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
    --delimiter(-d): string
    --harvest-mode
    --json
    --just-mime
    --no-infer
    --prefer-dmy
    --pretty-json
    --progressbar(-p)
    --quick(-Q)
    --quote: string
    --sample: string
    --save-urlsample: string
    --stats-types
    --timeout: string
    --user-agent: string
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --delimiter(-d): string
    --faster
    --ignore-case(-i)
    --jobs(-j): string
    --memcheck
    --natural
    --no-headers(-n)
    --numeric(-N)
    --output(-o): string
    --random
    --reverse(-R)
    --rng: string
    --seed: string
    --select(-s): string
    --unique(-u)
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --all
    --delimiter(-d): string
    --ignore-case(-i)
    --json
    --natural
    --no-headers(-n)
    --numeric(-N)
    --pretty-json
    --progressbar(-p)
    --select(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --chunks(-c): string
    --delimiter(-d): string
    --filename: string
    --filter: string
    --filter-cleanup
    --filter-ignore-errors
    --jobs(-j): string
    --kb-size(-k): string
    --no-headers(-n)
    --pad: string
    --quiet(-q)
    --size(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --cache-schema
    --compress-level: string
    --compression: string
    --date-format: string
    --datetime-format: string
    --decimal-comma
    --delimiter(-d): string
    --float-precision: string
    --format: string
    --ignore-errors
    --infer-len: string
    --low-memory
    --no-optimizations
    --output(-o): string
    --quiet(-q)
    --rnull-values: string
    --statistics
    --streaming
    --time-format: string
    --truncate-ragged-lines
    --try-parsedates
    --wnull-value: string
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --boolean-patterns: string
    --cache-threshold(-c): string
    --cardinality
    --cardinality-method: string
    --dates-whitelist: string
    --delimiter(-d): string
    --everything(-E)
    --force
    --infer-boolean
    --infer-dates
    --jobs(-j): string
    --mad
    --median
    --memcheck
    --mode
    --mode-cardinality-cap: string
    --no-headers(-n)
    --nulls
    --output(-o): string
    --percentile-list: string
    --percentiles
    --prefer-dmy
    --quantile-method: string
    --quartiles
    --round: string
    --select(-s): string
    --stats-jsonl
    --typesonly
    --vis-whitespace
    --weight: string
    --zero-padded-numeric
    --help(-h)                # Print help
  ]

  export extern "qsv synthesize" [
    --consistent-fakes
    --correlation-threshold: string
    --delimiter(-d): string
    --dictionary: string
    --freq-limit: string
    --infer-content-type
    --jobs(-j): string
    --joint-cardinality-cap: string
    --locale: string
    --no-relationships
    --output(-o): string
    --rows(-n): string
    --seed: string
    --stats-options: string
    --strict-relationships
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --align(-a): string
    --condense(-c): string
    --delimiter(-d): string
    --memcheck
    --output(-o): string
    --pad(-p): string
    --width(-w): string
    --help(-h)                # Print help
  ]

  export extern "qsv template" [
    --batch(-b): string
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --customfilter-error: string
    --delimiter: string
    --globals-json(-J): string
    --jobs(-j): string
    --no-headers(-n)
    --outfilename: string
    --output(-o): string
    --outsubdir-size: string
    --progressbar(-p)
    --template: string
    --template-file(-t): string
    --timeout: string
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --all-strings(-A)
    --compress-level: string
    --compression: string
    --delimiter: string
    --drop(-d)
    --dump(-u)
    --evolve(-e)
    --infer-len: string
    --jobs(-j): string
    --pipe(-i)
    --print-package(-k)
    --quiet(-q)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --table(-t): string
    --try-parse-dates
    --help(-h)                # Print help
  ]

  export extern "qsv to datapackage" [
    --all-strings(-A)
    --compress-level: string
    --compression: string
    --delimiter: string
    --drop(-d)
    --dump(-u)
    --evolve(-e)
    --infer-len: string
    --jobs(-j): string
    --pipe(-i)
    --print-package(-k)
    --quiet(-q)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --table(-t): string
    --try-parse-dates
    --help(-h)                # Print help
  ]

  export extern "qsv to ods" [
    --all-strings(-A)
    --compress-level: string
    --compression: string
    --delimiter: string
    --drop(-d)
    --dump(-u)
    --evolve(-e)
    --infer-len: string
    --jobs(-j): string
    --pipe(-i)
    --print-package(-k)
    --quiet(-q)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --table(-t): string
    --try-parse-dates
    --help(-h)                # Print help
  ]

  export extern "qsv to parquet" [
    --all-strings(-A)
    --compress-level: string
    --compression: string
    --delimiter: string
    --drop(-d)
    --dump(-u)
    --evolve(-e)
    --infer-len: string
    --jobs(-j): string
    --pipe(-i)
    --print-package(-k)
    --quiet(-q)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --table(-t): string
    --try-parse-dates
    --help(-h)                # Print help
  ]

  export extern "qsv to postgres" [
    --all-strings(-A)
    --compress-level: string
    --compression: string
    --delimiter: string
    --drop(-d)
    --dump(-u)
    --evolve(-e)
    --infer-len: string
    --jobs(-j): string
    --pipe(-i)
    --print-package(-k)
    --quiet(-q)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --table(-t): string
    --try-parse-dates
    --help(-h)                # Print help
  ]

  export extern "qsv to sqlite" [
    --all-strings(-A)
    --compress-level: string
    --compression: string
    --delimiter: string
    --drop(-d)
    --dump(-u)
    --evolve(-e)
    --infer-len: string
    --jobs(-j): string
    --pipe(-i)
    --print-package(-k)
    --quiet(-q)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --table(-t): string
    --try-parse-dates
    --help(-h)                # Print help
  ]

  export extern "qsv to xlsx" [
    --all-strings(-A)
    --compress-level: string
    --compression: string
    --delimiter: string
    --drop(-d)
    --dump(-u)
    --evolve(-e)
    --infer-len: string
    --jobs(-j): string
    --pipe(-i)
    --print-package(-k)
    --quiet(-q)
    --schema(-s): string
    --separator(-p): string
    --stats(-a)
    --stats-csv(-c): string
    --table(-t): string
    --try-parse-dates
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
    --batch(-b): string
    --delimiter(-d): string
    --jobs(-j): string
    --memcheck
    --no-boolean
    --output(-o): string
    --quiet(-q)
    --trim
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --delimiter(-d): string
    --long: string
    --memcheck
    --multipass(-m)
    --output(-o): string
    --select(-s): string
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --backtrack-limit: string
    --batch(-b): string
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --delimiter(-d): string
    --dfa-size-limit: string
    --email-display-text
    --email-domain-literal
    --email-min-subdomains: string
    --email-required-tld
    --fail-fast
    --fancy-regex
    --invalid: string
    --jobs(-j): string
    --json
    --no-format-validation
    --no-headers(-n)
    --pretty-json
    --progressbar(-p)
    --quiet(-q)
    --size-limit: string
    --timeout: string
    --trim
    --valid: string
    --valid-output: string
    --help(-h)                # Print help
  ]

  export extern "qsv validate schema" [
    --backtrack-limit: string
    --batch(-b): string
    --cache-dir: string
    --ckan-api: string
    --ckan-token: string
    --delimiter(-d): string
    --dfa-size-limit: string
    --email-display-text
    --email-domain-literal
    --email-min-subdomains: string
    --email-required-tld
    --fail-fast
    --fancy-regex
    --invalid: string
    --jobs(-j): string
    --json
    --no-format-validation
    --no-headers(-n)
    --pretty-json
    --progressbar(-p)
    --quiet(-q)
    --size-limit: string
    --timeout: string
    --trim
    --valid: string
    --valid-output: string
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

  export extern "qsv help geocode cache-clear" [
  ]

  export extern "qsv help geocode cache-info" [
  ]

  export extern "qsv help geocode cache-prune" [
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

  export extern "qsv help geocode opencage" [
  ]

  export extern "qsv help geocode opencagenow" [
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

  export extern "qsv help get" [
  ]

  export extern "qsv help get cache-clear" [
  ]

  export extern "qsv help get cache-info" [
  ]

  export extern "qsv help get cache-list" [
  ]

  export extern "qsv help get cache-prune" [
  ]

  export extern "qsv help get cache-set-policy" [
  ]

  export extern "qsv help get cache-set-ttl" [
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

  export extern "qsv help profile" [
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

  export extern "qsv help synthesize" [
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
