#compdef qsv

autoload -U is-at-least

_qsv() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'--list[]' \
'--envlist[]' \
'--update[]' \
'--updatenow[]' \
'-V[]' \
'--version[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv_commands" \
"*::: :->qsv" \
&& ret=0
    case $state in
    (qsv)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-command-$line[1]:"
        case $line[1] in
            (apply)
_arguments "${_arguments_options[@]}" : \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__apply_commands" \
"*::: :->apply" \
&& ret=0

    case $state in
    (apply)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-apply-command-$line[1]:"
        case $line[1] in
            (calcconv)
_arguments "${_arguments_options[@]}" : \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(dynfmt)
_arguments "${_arguments_options[@]}" : \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(emptyreplace)
_arguments "${_arguments_options[@]}" : \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(operations)
_arguments "${_arguments_options[@]}" : \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__apply__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-apply-help-command-$line[1]:"
        case $line[1] in
            (calcconv)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(dynfmt)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(emptyreplace)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(operations)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(behead)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f[]' \
'--flexible[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cat)
_arguments "${_arguments_options[@]}" : \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-p[]' \
'--pad[]' \
'--flexible[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__cat_commands" \
"*::: :->cat" \
&& ret=0

    case $state in
    (cat)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-cat-command-$line[1]:"
        case $line[1] in
            (columns)
_arguments "${_arguments_options[@]}" : \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-p[]' \
'--pad[]' \
'--flexible[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(rows)
_arguments "${_arguments_options[@]}" : \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-p[]' \
'--pad[]' \
'--flexible[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(rowskey)
_arguments "${_arguments_options[@]}" : \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-p[]' \
'--pad[]' \
'--flexible[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__cat__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-cat-help-command-$line[1]:"
        case $line[1] in
            (columns)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(rows)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(rowskey)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(clipboard)
_arguments "${_arguments_options[@]}" : \
'-s[]' \
'--save[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(color)
_arguments "${_arguments_options[@]}" : \
'-t+[]: :_default' \
'--title=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--row-numbers[]' \
'--memcheck[]' \
'-C[]' \
'--color[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(count)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-H[]' \
'--human-readable[]' \
'--json[]' \
'--no-polars[]' \
'-f[]' \
'--flexible[]' \
'--low-memory[]' \
'--width[]' \
'-n[]' \
'--no-headers[]' \
'--width-no-delims[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(datefmt)
_arguments "${_arguments_options[@]}" : \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--output-tz=[]: :_default' \
'--input-tz=[]: :_default' \
'--default-tz=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-R+[]: :_default' \
'--ts-resolution=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--utc[]' \
'--zulu[]' \
'-p[]' \
'--progressbar[]' \
'--prefer-dmy[]' \
'-n[]' \
'--no-headers[]' \
'--keep-zero-time[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(dedup)
_arguments "${_arguments_options[@]}" : \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-D+[]: :_default' \
'--dupes-output=[]: :_default' \
'-q[]' \
'--quiet[]' \
'-H[]' \
'--human-readable[]' \
'-N[]' \
'--numeric[]' \
'-i[]' \
'--ignore-case[]' \
'--memcheck[]' \
'-n[]' \
'--no-headers[]' \
'--sorted[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(describegpt)
_arguments "${_arguments_options[@]}" : \
'--language=[]: :_default' \
'--sql-results=[]: :_default' \
'--session-len=[]: :_default' \
'--export-prompt=[]: :_default' \
'-p+[]: :_default' \
'--prompt=[]: :_default' \
'--sample-size=[]: :_default' \
'-u+[]: :_default' \
'--base-url=[]: :_default' \
'--format=[]: :_default' \
'--timeout=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'--num-examples=[]: :_default' \
'--truncate-str=[]: :_default' \
'--ckan-token=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--stats-options=[]: :_default' \
'--prompt-file=[]: :_default' \
'--user-agent=[]: :_default' \
'-m+[]: :_default' \
'--model=[]: :_default' \
'-k+[]: :_default' \
'--api-key=[]: :_default' \
'--tag-vocab=[]: :_default' \
'--addl-props=[]: :_default' \
'--ckan-api=[]: :_default' \
'--freq-options=[]: :_default' \
'--num-tags=[]: :_default' \
'--cache-dir=[]: :_default' \
'--enum-threshold=[]: :_default' \
'-t+[]: :_default' \
'--max-tokens=[]: :_default' \
'--addl-cols-list=[]: :_default' \
'--session=[]: :_default' \
'--description[]' \
'--addl-cols[]' \
'--redis-cache[]' \
'--no-cache[]' \
'--fewshot-examples[]' \
'--tags[]' \
'-q[]' \
'--quiet[]' \
'--forget[]' \
'--flush-cache[]' \
'--dictionary[]' \
'--fresh[]' \
'-A[]' \
'--all[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(diff)
_arguments "${_arguments_options[@]}" : \
'--sort-columns=[]: :_default' \
'-k+[]: :_default' \
'--key=[]: :_default' \
'--delimiter-output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--delimiter-left=[]: :_default' \
'--delimiter-right=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--no-headers-left[]' \
'--no-headers-output[]' \
'--no-headers-right[]' \
'--drop-equal-fields[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-i[]' \
'--in-place[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enum)
_arguments "${_arguments_options[@]}" : \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--increment=[]: :_default' \
'--start=[]: :_default' \
'--constant=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--hash=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--copy=[]: :_default' \
'--uuid4[]' \
'--uuid7[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(excel)
_arguments "${_arguments_options[@]}" : \
'--header-row=[]: :_default' \
'--metadata=[]: :_default' \
'-s+[]: :_default' \
'--sheet=[]: :_default' \
'--cell=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--range=[]: :_default' \
'--error-format=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--table=[]: :_default' \
'--date-format=[]: :_default' \
'--keep-zero-time[]' \
'--trim[]' \
'--flexible[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(exclude)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-i[]' \
'--ignore-case[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(explode)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(extdedup)
_arguments "${_arguments_options[@]}" : \
'--memory-limit=[]: :_default' \
'--temp-dir=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-D+[]: :_default' \
'--dupes-output=[]: :_default' \
'--no-output[]' \
'-q[]' \
'--quiet[]' \
'-H[]' \
'--human-readable[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(extsort)
_arguments "${_arguments_options[@]}" : \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--memory-limit=[]: :_default' \
'--tmp-dir=[]: :_default' \
'-R[]' \
'--reverse[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fetch)
_arguments "${_arguments_options[@]}" : \
'--jaqfile=[]: :_default' \
'--rate-limit=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'--report=[]: :_default' \
'--jaq=[]: :_default' \
'--mem-cache-size=[]: :_default' \
'-H+[]: :_default' \
'--http-header=[]: :_default' \
'--max-retries=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--url-template=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--timeout=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--user-agent=[]: :_default' \
'--max-errors=[]: :_default' \
'--store-error[]' \
'--no-cache[]' \
'--pretty[]' \
'--cache-error[]' \
'-p[]' \
'--progressbar[]' \
'--cookies[]' \
'-n[]' \
'--no-headers[]' \
'--disk-cache[]' \
'--flush-cache[]' \
'--redis-cache[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fetchpost)
_arguments "${_arguments_options[@]}" : \
'-t+[]: :_default' \
'--payload-tpl=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--timeout=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'--jaqfile=[]: :_default' \
'--max-retries=[]: :_default' \
'--content-type=[]: :_default' \
'--user-agent=[]: :_default' \
'--mem-cache-size=[]: :_default' \
'--report=[]: :_default' \
'--max-errors=[]: :_default' \
'--jaq=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-j+[]: :_default' \
'--globals-json=[]: :_default' \
'--rate-limit=[]: :_default' \
'-H+[]: :_default' \
'--http-header=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--pretty[]' \
'--cookies[]' \
'--no-cache[]' \
'--flush-cache[]' \
'--store-error[]' \
'-n[]' \
'--no-headers[]' \
'--cache-error[]' \
'--redis-cache[]' \
'-p[]' \
'--progressbar[]' \
'--disk-cache[]' \
'--compress[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fill)
_arguments "${_arguments_options[@]}" : \
'-v+[]: :_default' \
'--default=[]: :_default' \
'-g+[]: :_default' \
'--groupby=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-b[]' \
'--backfill[]' \
'-f[]' \
'--first[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fixlengths)
_arguments "${_arguments_options[@]}" : \
'--quote=[]: :_default' \
'-l+[]: :_default' \
'--length=[]: :_default' \
'--escape=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-i+[]: :_default' \
'--insert=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-r[]' \
'--remove-empty[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(flatten)
_arguments "${_arguments_options[@]}" : \
'-f+[]: :_default' \
'--field-separator=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-s+[]: :_default' \
'--separator=[]: :_default' \
'-c+[]: :_default' \
'--condense=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fmt)
_arguments "${_arguments_options[@]}" : \
'-t+[]: :_default' \
'--out-delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--escape=[]: :_default' \
'--quote=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--quote-never[]' \
'--crlf[]' \
'--quote-always[]' \
'--ascii[]' \
'--no-final-newline[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(foreach)
_arguments "${_arguments_options[@]}" : \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--dry-run=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-u[]' \
'--unify[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(frequency)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--pct-dec-places=[]: :_default' \
'--stats-filter=[]: :_default' \
'--lmt-threshold=[]: :_default' \
'--no-float=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-u+[]: :_default' \
'--unq-limit=[]: :_default' \
'-l+[]: :_default' \
'--limit=[]: :_default' \
'-r+[]: :_default' \
'--rank-strategy=[]: :_default' \
'--other-text=[]: :_default' \
'--null-text=[]: :_default' \
'--all-unique-text=[]: :_default' \
'--weight=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--no-stats[]' \
'--memcheck[]' \
'--toon[]' \
'--pretty-json[]' \
'--no-nulls[]' \
'--vis-whitespace[]' \
'-i[]' \
'--ignore-case[]' \
'--no-trim[]' \
'--pct-nulls[]' \
'--json[]' \
'--null-sorted[]' \
'--other-sorted[]' \
'--no-other[]' \
'-a[]' \
'--asc[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(geocode)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__geocode_commands" \
"*::: :->geocode" \
&& ret=0

    case $state in
    (geocode)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-geocode-command-$line[1]:"
        case $line[1] in
            (countryinfo)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(countryinfonow)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-check)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-load)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-reset)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-update)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(iplookup)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(iplookupnow)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reversenow)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(suggest)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(suggestnow)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--language=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--admin1=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--country=[]: :_default' \
'--min-score=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--cities-url=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__geocode__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-geocode-help-command-$line[1]:"
        case $line[1] in
            (countryinfo)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(countryinfonow)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(index-check)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(index-load)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(index-reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(index-update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(iplookup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(iplookupnow)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reversenow)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(suggest)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(suggestnow)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(geoconvert)
_arguments "${_arguments_options[@]}" : \
'-x+[]: :_default' \
'--longitude=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--geometry=[]: :_default' \
'-y+[]: :_default' \
'--latitude=[]: :_default' \
'-l+[]: :_default' \
'--max-length=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(headers)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--trim[]' \
'-j[]' \
'--just-names[]' \
'--intersect[]' \
'-J[]' \
'--just-count[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(input)
_arguments "${_arguments_options[@]}" : \
'--encoding-errors=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--comment=[]: :_default' \
'--quote-style=[]: :_default' \
'--escape=[]: :_default' \
'--quote=[]: :_default' \
'--skip-lastlines=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--skip-lines=[]: :_default' \
'--trim-fields[]' \
'--auto-skip[]' \
'--trim-headers[]' \
'--no-quoting[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(join)
_arguments "${_arguments_options[@]}" : \
'--keys-output=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--left[]' \
'-i[]' \
'--ignore-case[]' \
'-z[]' \
'--ignore-leading-zeros[]' \
'--left-semi[]' \
'-n[]' \
'--no-headers[]' \
'--left-anti[]' \
'--right-anti[]' \
'--right[]' \
'--cross[]' \
'--full[]' \
'--right-semi[]' \
'--nulls[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(joinp)
_arguments "${_arguments_options[@]}" : \
'--date-format=[]: :_default' \
'-N+[]: :_default' \
'--norm-unicode=[]: :_default' \
'--right_by=[]: :_default' \
'--strategy=[]: :_default' \
'--maintain-order=[]: :_default' \
'--time-format=[]: :_default' \
'--tolerance=[]: :_default' \
'--left_by=[]: :_default' \
'--null-value=[]: :_default' \
'--non-equi=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--datetime-format=[]: :_default' \
'--cache-schema=[]: :_default' \
'--sql-filter=[]: :_default' \
'--float-precision=[]: :_default' \
'--filter-right=[]: :_default' \
'--infer-len=[]: :_default' \
'--filter-left=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--validate=[]: :_default' \
'-i[]' \
'--ignore-case[]' \
'--asof[]' \
'--cross[]' \
'--left-semi[]' \
'-X[]' \
'--allow-exact-matches[]' \
'--right-semi[]' \
'--right-anti[]' \
'-z[]' \
'--ignore-leading-zeros[]' \
'--left[]' \
'--full[]' \
'--nulls[]' \
'--ignore-errors[]' \
'--streaming[]' \
'--low-memory[]' \
'--no-optimizations[]' \
'--left-anti[]' \
'--try-parsedates[]' \
'--right[]' \
'--coalesce[]' \
'--no-sort[]' \
'-q[]' \
'--quiet[]' \
'--decimal-comma[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(json)
_arguments "${_arguments_options[@]}" : \
'--jaq=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(jsonl)
_arguments "${_arguments_options[@]}" : \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--ignore-errors[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(lens)
_arguments "${_arguments_options[@]}" : \
'--filter=[]: :_default' \
'--columns=[]: :_default' \
'-P+[]: :_default' \
'--prompt=[]: :_default' \
'--echo-column=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--find=[]: :_default' \
'-f+[]: :_default' \
'--freeze-columns=[]: :_default' \
'-W+[]: :_default' \
'--wrap-mode=[]: :_default' \
'-i[]' \
'--ignore-case[]' \
'-t[]' \
'--tab-separated[]' \
'--no-headers[]' \
'--debug[]' \
'-S[]' \
'--streaming-stdin[]' \
'-m[]' \
'--monochrome[]' \
'-A[]' \
'--auto-reload[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(luau)
_arguments "${_arguments_options[@]}" : \
'--ckan-api=[]: :_default' \
'--cache-dir=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--ckan-token=[]: :_default' \
'--max-errors=[]: :_default' \
'--timeout=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'-B+[]: :_default' \
'--begin=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-g[]' \
'--no-globals[]' \
'--colindex[]' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-r[]' \
'--remap[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__luau_commands" \
"*::: :->luau" \
&& ret=0

    case $state in
    (luau)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-luau-command-$line[1]:"
        case $line[1] in
            (filter)
_arguments "${_arguments_options[@]}" : \
'--ckan-api=[]: :_default' \
'--cache-dir=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--ckan-token=[]: :_default' \
'--max-errors=[]: :_default' \
'--timeout=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'-B+[]: :_default' \
'--begin=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-g[]' \
'--no-globals[]' \
'--colindex[]' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-r[]' \
'--remap[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(map)
_arguments "${_arguments_options[@]}" : \
'--ckan-api=[]: :_default' \
'--cache-dir=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--ckan-token=[]: :_default' \
'--max-errors=[]: :_default' \
'--timeout=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'-B+[]: :_default' \
'--begin=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-g[]' \
'--no-globals[]' \
'--colindex[]' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-r[]' \
'--remap[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__luau__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-luau-help-command-$line[1]:"
        case $line[1] in
            (filter)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(map)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(moarstats)
_arguments "${_arguments_options[@]}" : \
'--stats-options=[]: :_default' \
'--round=[]: :_default' \
'-e+[]: :_default' \
'--epsilon=[]: :_default' \
'-T+[]: :_default' \
'--join-type=[]: :_default' \
'-J+[]: :_default' \
'--join-inputs=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-K+[]: :_default' \
'--join-keys=[]: :_default' \
'--pct-thresholds=[]: :_default' \
'-S+[]: :_default' \
'--bivariate-stats=[]: :_default' \
'-C+[]: :_default' \
'--cardinality-threshold=[]: :_default' \
'--xsd-gdate-scan=[]: :_default' \
'--force[]' \
'--use-percentiles[]' \
'--advanced[]' \
'-B[]' \
'--bivariate[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(partition)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--filename=[]: :_default' \
'-p+[]: :_default' \
'--prefix-length=[]: :_default' \
'--limit=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--drop[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pivotp)
_arguments "${_arguments_options[@]}" : \
'-i+[]: :_default' \
'--index=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--col-separator=[]: :_default' \
'--infer-len=[]: :_default' \
'-v+[]: :_default' \
'--values=[]: :_default' \
'-a+[]: :_default' \
'--agg=[]: :_default' \
'--try-parsedates[]' \
'--maintain-order[]' \
'--sort-columns[]' \
'--validate[]' \
'--decimal-comma[]' \
'--ignore-errors[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pro)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__pro_commands" \
"*::: :->pro" \
&& ret=0

    case $state in
    (pro)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-pro-command-$line[1]:"
        case $line[1] in
            (lens)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(workflow)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__pro__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-pro-help-command-$line[1]:"
        case $line[1] in
            (lens)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(workflow)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(prompt)
_arguments "${_arguments_options[@]}" : \
'-F+[]: :_default' \
'--filters=[]: :_default' \
'-d+[]: :_default' \
'--workdir=[]: :_default' \
'--base-delay-ms=[]: :_default' \
'-m+[]: :_default' \
'--msg=[]: :_default' \
'--save-fname=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f[]' \
'--fd-output[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pseudo)
_arguments "${_arguments_options[@]}" : \
'--increment=[]: :_default' \
'--start=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--formatstr=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(py)
_arguments "${_arguments_options[@]}" : \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--helper=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__py_commands" \
"*::: :->py" \
&& ret=0

    case $state in
    (py)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-py-command-$line[1]:"
        case $line[1] in
            (filter)
_arguments "${_arguments_options[@]}" : \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--helper=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(map)
_arguments "${_arguments_options[@]}" : \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--helper=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__py__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-py-help-command-$line[1]:"
        case $line[1] in
            (filter)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(map)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(rename)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--pairwise[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(replace)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--size-limit=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-i[]' \
'--ignore-case[]' \
'--not-one[]' \
'--literal[]' \
'-n[]' \
'--no-headers[]' \
'--exact[]' \
'-q[]' \
'--quiet[]' \
'-u[]' \
'--unicode[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--memcheck[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(safenames)
_arguments "${_arguments_options[@]}" : \
'--mode=[]: :_default' \
'--reserved=[]: :_default' \
'--prefix=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sample)
_arguments "${_arguments_options[@]}" : \
'--systematic=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--seed=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--ts-start=[]: :_default' \
'--stratified=[]: :_default' \
'--ts-aggregate=[]: :_default' \
'--max-size=[]: :_default' \
'--weighted=[]: :_default' \
'--ts-adaptive=[]: :_default' \
'--ts-input-tz=[]: :_default' \
'--cluster=[]: :_default' \
'--timeout=[]: :_default' \
'--rng=[]: :_default' \
'--timeseries=[]: :_default' \
'--ts-interval=[]: :_default' \
'--user-agent=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--force[]' \
'--ts-prefer-dmy[]' \
'--bernoulli[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(schema)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--pattern-columns=[]: :_default' \
'--dates-whitelist=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--enum-threshold=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--strict-dates[]' \
'-n[]' \
'--no-headers[]' \
'--force[]' \
'--stdout[]' \
'--polars[]' \
'--memcheck[]' \
'-i[]' \
'--ignore-case[]' \
'--prefer-dmy[]' \
'--strict-formats[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(search)
_arguments "${_arguments_options[@]}" : \
'--size-limit=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--flag=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--preview-match=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--exact[]' \
'-Q[]' \
'--quick[]' \
'--literal[]' \
'--json[]' \
'-i[]' \
'--ignore-case[]' \
'--not-one[]' \
'-n[]' \
'--no-headers[]' \
'-c[]' \
'--count[]' \
'-q[]' \
'--quiet[]' \
'-v[]' \
'--invert-match[]' \
'-u[]' \
'--unicode[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(searchset)
_arguments "${_arguments_options[@]}" : \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--jobs=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-f+[]: :_default' \
'--flag=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--size-limit=[]: :_default' \
'--unmatched-output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-q[]' \
'--quiet[]' \
'-i[]' \
'--ignore-case[]' \
'-u[]' \
'--unicode[]' \
'--not-one[]' \
'-p[]' \
'--progressbar[]' \
'-v[]' \
'--invert-match[]' \
'--exact[]' \
'-Q[]' \
'--quick[]' \
'--literal[]' \
'--flag-matches-only[]' \
'-c[]' \
'--count[]' \
'-j[]' \
'--json[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(select)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--seed=[]: :_default' \
'-R[]' \
'--random[]' \
'-n[]' \
'--no-headers[]' \
'-S[]' \
'--sort[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(slice)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--len=[]: :_default' \
'-s+[]: :_default' \
'--start=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-e+[]: :_default' \
'--end=[]: :_default' \
'-i+[]: :_default' \
'--index=[]: :_default' \
'--json[]' \
'-n[]' \
'--no-headers[]' \
'--invert[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(snappy)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--user-agent=[]: :_default' \
'--timeout=[]: :_default' \
'-q[]' \
'--quiet[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__snappy_commands" \
"*::: :->snappy" \
&& ret=0

    case $state in
    (snappy)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-snappy-command-$line[1]:"
        case $line[1] in
            (check)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--user-agent=[]: :_default' \
'--timeout=[]: :_default' \
'-q[]' \
'--quiet[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(compress)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--user-agent=[]: :_default' \
'--timeout=[]: :_default' \
'-q[]' \
'--quiet[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(decompress)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--user-agent=[]: :_default' \
'--timeout=[]: :_default' \
'-q[]' \
'--quiet[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--user-agent=[]: :_default' \
'--timeout=[]: :_default' \
'-q[]' \
'--quiet[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__snappy__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-snappy-help-command-$line[1]:"
        case $line[1] in
            (check)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(compress)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(decompress)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(sniff)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--quote=[]: :_default' \
'--timeout=[]: :_default' \
'--user-agent=[]: :_default' \
'--sample=[]: :_default' \
'--save-urlsample=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--just-mime[]' \
'--harvest-mode[]' \
'--prefer-dmy[]' \
'--json[]' \
'-Q[]' \
'--quick[]' \
'--stats-types[]' \
'--no-infer[]' \
'--pretty-json[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sort)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--rng=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--seed=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--random[]' \
'-R[]' \
'--reverse[]' \
'-i[]' \
'--ignore-case[]' \
'-N[]' \
'--numeric[]' \
'--faster[]' \
'--memcheck[]' \
'-u[]' \
'--unique[]' \
'--natural[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sortcheck)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--pretty-json[]' \
'--all[]' \
'--json[]' \
'-i[]' \
'--ignore-case[]' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(split)
_arguments "${_arguments_options[@]}" : \
'-c+[]: :_default' \
'--chunks=[]: :_default' \
'-s+[]: :_default' \
'--size=[]: :_default' \
'--filter=[]: :_default' \
'--filename=[]: :_default' \
'-k+[]: :_default' \
'--kb-size=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--pad=[]: :_default' \
'--filter-ignore-errors[]' \
'-n[]' \
'--no-headers[]' \
'--filter-cleanup[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sqlp)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--compress-level=[]: :_default' \
'--compression=[]: :_default' \
'--time-format=[]: :_default' \
'--infer-len=[]: :_default' \
'--float-precision=[]: :_default' \
'--date-format=[]: :_default' \
'--rnull-values=[]: :_default' \
'--format=[]: :_default' \
'--datetime-format=[]: :_default' \
'--wnull-value=[]: :_default' \
'--truncate-ragged-lines[]' \
'--statistics[]' \
'--try-parsedates[]' \
'--cache-schema[]' \
'--low-memory[]' \
'-q[]' \
'--quiet[]' \
'--no-optimizations[]' \
'--streaming[]' \
'--ignore-errors[]' \
'--decimal-comma[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stats)
_arguments "${_arguments_options[@]}" : \
'--percentile-list=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-c+[]: :_default' \
'--cache-threshold=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--boolean-patterns=[]: :_default' \
'--weight=[]: :_default' \
'--round=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--dates-whitelist=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--nulls[]' \
'-E[]' \
'--everything[]' \
'--cardinality[]' \
'--stats-jsonl[]' \
'--infer-boolean[]' \
'-n[]' \
'--no-headers[]' \
'--median[]' \
'--memcheck[]' \
'--infer-dates[]' \
'--mad[]' \
'--force[]' \
'--vis-whitespace[]' \
'--prefer-dmy[]' \
'--mode[]' \
'--typesonly[]' \
'--percentiles[]' \
'--quartiles[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(table)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-c+[]: :_default' \
'--condense=[]: :_default' \
'-w+[]: :_default' \
'--width=[]: :_default' \
'-p+[]: :_default' \
'--pad=[]: :_default' \
'-a+[]: :_default' \
'--align=[]: :_default' \
'--memcheck[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(template)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--globals-json=[]: :_default' \
'--delimiter=[]: :_default' \
'--ckan-api=[]: :_default' \
'--cache-dir=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--jobs=[]: :_default' \
'--outsubdir-size=[]: :_default' \
'--timeout=[]: :_default' \
'--ckan-token=[]: :_default' \
'--template=[]: :_default' \
'-t+[]: :_default' \
'--template-file=[]: :_default' \
'--customfilter-error=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--outfilename=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(to)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'--delimiter=[]: :_default' \
'-a[]' \
'--stats[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-u[]' \
'--dump[]' \
'-A[]' \
'--all-strings[]' \
'-e[]' \
'--evolve[]' \
'-d[]' \
'--drop[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__to_commands" \
"*::: :->to" \
&& ret=0

    case $state in
    (to)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-to-command-$line[1]:"
        case $line[1] in
            (datapackage)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'--delimiter=[]: :_default' \
'-a[]' \
'--stats[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-u[]' \
'--dump[]' \
'-A[]' \
'--all-strings[]' \
'-e[]' \
'--evolve[]' \
'-d[]' \
'--drop[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(ods)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'--delimiter=[]: :_default' \
'-a[]' \
'--stats[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-u[]' \
'--dump[]' \
'-A[]' \
'--all-strings[]' \
'-e[]' \
'--evolve[]' \
'-d[]' \
'--drop[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(postgres)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'--delimiter=[]: :_default' \
'-a[]' \
'--stats[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-u[]' \
'--dump[]' \
'-A[]' \
'--all-strings[]' \
'-e[]' \
'--evolve[]' \
'-d[]' \
'--drop[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sqlite)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'--delimiter=[]: :_default' \
'-a[]' \
'--stats[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-u[]' \
'--dump[]' \
'-A[]' \
'--all-strings[]' \
'-e[]' \
'--evolve[]' \
'-d[]' \
'--drop[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(xlsx)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'--delimiter=[]: :_default' \
'-a[]' \
'--stats[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-u[]' \
'--dump[]' \
'-A[]' \
'--all-strings[]' \
'-e[]' \
'--evolve[]' \
'-d[]' \
'--drop[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__to__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-to-help-command-$line[1]:"
        case $line[1] in
            (datapackage)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(ods)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(postgres)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(sqlite)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(xlsx)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(tojsonl)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--trim[]' \
'--no-boolean[]' \
'--memcheck[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(transpose)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--long=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-m[]' \
'--multipass[]' \
'--memcheck[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
'--valid-output=[]: :_default' \
'--backtrack-limit=[]: :_default' \
'--timeout=[]: :_default' \
'--cache-dir=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--size-limit=[]: :_default' \
'--valid=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--invalid=[]: :_default' \
'--ckan-token=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'--email-min-subdomains=[]: :_default' \
'--ckan-api=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--fancy-regex[]' \
'--email-display-text[]' \
'--trim[]' \
'--fail-fast[]' \
'--pretty-json[]' \
'-q[]' \
'--quiet[]' \
'--email-required-tld[]' \
'--email-domain-literal[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'--no-format-validation[]' \
'--json[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__validate_commands" \
"*::: :->validate" \
&& ret=0

    case $state in
    (validate)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-validate-command-$line[1]:"
        case $line[1] in
            (schema)
_arguments "${_arguments_options[@]}" : \
'--valid-output=[]: :_default' \
'--backtrack-limit=[]: :_default' \
'--timeout=[]: :_default' \
'--cache-dir=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--size-limit=[]: :_default' \
'--valid=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--invalid=[]: :_default' \
'--ckan-token=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'--email-min-subdomains=[]: :_default' \
'--ckan-api=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--fancy-regex[]' \
'--email-display-text[]' \
'--trim[]' \
'--fail-fast[]' \
'--pretty-json[]' \
'-q[]' \
'--quiet[]' \
'--email-required-tld[]' \
'--email-domain-literal[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'--no-format-validation[]' \
'--json[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__validate__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-validate-help-command-$line[1]:"
        case $line[1] in
            (schema)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-command-$line[1]:"
        case $line[1] in
            (apply)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__help__apply_commands" \
"*::: :->apply" \
&& ret=0

    case $state in
    (apply)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-apply-command-$line[1]:"
        case $line[1] in
            (calcconv)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(dynfmt)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(emptyreplace)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(operations)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(behead)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cat)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__help__cat_commands" \
"*::: :->cat" \
&& ret=0

    case $state in
    (cat)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-cat-command-$line[1]:"
        case $line[1] in
            (columns)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(rows)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(rowskey)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(clipboard)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(color)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(count)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(datefmt)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(dedup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(describegpt)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(diff)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enum)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(excel)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(exclude)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(explode)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(extdedup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(extsort)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(fetch)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(fetchpost)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(fill)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(fixlengths)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(flatten)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(fmt)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(foreach)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(frequency)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(geocode)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__help__geocode_commands" \
"*::: :->geocode" \
&& ret=0

    case $state in
    (geocode)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-geocode-command-$line[1]:"
        case $line[1] in
            (countryinfo)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(countryinfonow)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(index-check)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(index-load)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(index-reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(index-update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(iplookup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(iplookupnow)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reversenow)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(suggest)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(suggestnow)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(geoconvert)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(headers)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(index)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(input)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(join)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(joinp)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(json)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(jsonl)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(lens)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(luau)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__help__luau_commands" \
"*::: :->luau" \
&& ret=0

    case $state in
    (luau)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-luau-command-$line[1]:"
        case $line[1] in
            (filter)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(map)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(moarstats)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(partition)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(pivotp)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(pro)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__help__pro_commands" \
"*::: :->pro" \
&& ret=0

    case $state in
    (pro)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-pro-command-$line[1]:"
        case $line[1] in
            (lens)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(workflow)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(prompt)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(pseudo)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(py)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__help__py_commands" \
"*::: :->py" \
&& ret=0

    case $state in
    (py)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-py-command-$line[1]:"
        case $line[1] in
            (filter)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(map)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(rename)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(replace)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(safenames)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(sample)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(schema)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(search)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(searchset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(select)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(slice)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(snappy)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__help__snappy_commands" \
"*::: :->snappy" \
&& ret=0

    case $state in
    (snappy)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-snappy-command-$line[1]:"
        case $line[1] in
            (check)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(compress)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(decompress)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(sniff)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(sort)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(sortcheck)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(split)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(sqlp)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stats)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(table)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(template)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(to)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__help__to_commands" \
"*::: :->to" \
&& ret=0

    case $state in
    (to)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-to-command-$line[1]:"
        case $line[1] in
            (datapackage)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(ods)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(postgres)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(sqlite)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(xlsx)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(tojsonl)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(transpose)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__help__validate_commands" \
"*::: :->validate" \
&& ret=0

    case $state in
    (validate)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-validate-command-$line[1]:"
        case $line[1] in
            (schema)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_qsv_commands] )) ||
_qsv_commands() {
    local commands; commands=(
'apply:' \
'behead:' \
'cat:' \
'clipboard:' \
'color:' \
'count:' \
'datefmt:' \
'dedup:' \
'describegpt:' \
'diff:' \
'edit:' \
'enum:' \
'excel:' \
'exclude:' \
'explode:' \
'extdedup:' \
'extsort:' \
'fetch:' \
'fetchpost:' \
'fill:' \
'fixlengths:' \
'flatten:' \
'fmt:' \
'foreach:' \
'frequency:' \
'geocode:' \
'geoconvert:' \
'headers:' \
'index:' \
'input:' \
'join:' \
'joinp:' \
'json:' \
'jsonl:' \
'lens:' \
'luau:' \
'moarstats:' \
'partition:' \
'pivotp:' \
'pro:' \
'prompt:' \
'pseudo:' \
'py:' \
'rename:' \
'replace:' \
'reverse:' \
'safenames:' \
'sample:' \
'schema:' \
'search:' \
'searchset:' \
'select:' \
'slice:' \
'snappy:' \
'sniff:' \
'sort:' \
'sortcheck:' \
'split:' \
'sqlp:' \
'stats:' \
'table:' \
'template:' \
'to:' \
'tojsonl:' \
'transpose:' \
'validate:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv commands' commands "$@"
}
(( $+functions[_qsv__apply_commands] )) ||
_qsv__apply_commands() {
    local commands; commands=(
'calcconv:' \
'dynfmt:' \
'emptyreplace:' \
'operations:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv apply commands' commands "$@"
}
(( $+functions[_qsv__apply__calcconv_commands] )) ||
_qsv__apply__calcconv_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply calcconv commands' commands "$@"
}
(( $+functions[_qsv__apply__dynfmt_commands] )) ||
_qsv__apply__dynfmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply dynfmt commands' commands "$@"
}
(( $+functions[_qsv__apply__emptyreplace_commands] )) ||
_qsv__apply__emptyreplace_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply emptyreplace commands' commands "$@"
}
(( $+functions[_qsv__apply__help_commands] )) ||
_qsv__apply__help_commands() {
    local commands; commands=(
'calcconv:' \
'dynfmt:' \
'emptyreplace:' \
'operations:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv apply help commands' commands "$@"
}
(( $+functions[_qsv__apply__help__calcconv_commands] )) ||
_qsv__apply__help__calcconv_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply help calcconv commands' commands "$@"
}
(( $+functions[_qsv__apply__help__dynfmt_commands] )) ||
_qsv__apply__help__dynfmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply help dynfmt commands' commands "$@"
}
(( $+functions[_qsv__apply__help__emptyreplace_commands] )) ||
_qsv__apply__help__emptyreplace_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply help emptyreplace commands' commands "$@"
}
(( $+functions[_qsv__apply__help__help_commands] )) ||
_qsv__apply__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply help help commands' commands "$@"
}
(( $+functions[_qsv__apply__help__operations_commands] )) ||
_qsv__apply__help__operations_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply help operations commands' commands "$@"
}
(( $+functions[_qsv__apply__operations_commands] )) ||
_qsv__apply__operations_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply operations commands' commands "$@"
}
(( $+functions[_qsv__behead_commands] )) ||
_qsv__behead_commands() {
    local commands; commands=()
    _describe -t commands 'qsv behead commands' commands "$@"
}
(( $+functions[_qsv__cat_commands] )) ||
_qsv__cat_commands() {
    local commands; commands=(
'columns:' \
'rows:' \
'rowskey:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv cat commands' commands "$@"
}
(( $+functions[_qsv__cat__columns_commands] )) ||
_qsv__cat__columns_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat columns commands' commands "$@"
}
(( $+functions[_qsv__cat__help_commands] )) ||
_qsv__cat__help_commands() {
    local commands; commands=(
'columns:' \
'rows:' \
'rowskey:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv cat help commands' commands "$@"
}
(( $+functions[_qsv__cat__help__columns_commands] )) ||
_qsv__cat__help__columns_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat help columns commands' commands "$@"
}
(( $+functions[_qsv__cat__help__help_commands] )) ||
_qsv__cat__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat help help commands' commands "$@"
}
(( $+functions[_qsv__cat__help__rows_commands] )) ||
_qsv__cat__help__rows_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat help rows commands' commands "$@"
}
(( $+functions[_qsv__cat__help__rowskey_commands] )) ||
_qsv__cat__help__rowskey_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat help rowskey commands' commands "$@"
}
(( $+functions[_qsv__cat__rows_commands] )) ||
_qsv__cat__rows_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat rows commands' commands "$@"
}
(( $+functions[_qsv__cat__rowskey_commands] )) ||
_qsv__cat__rowskey_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat rowskey commands' commands "$@"
}
(( $+functions[_qsv__clipboard_commands] )) ||
_qsv__clipboard_commands() {
    local commands; commands=()
    _describe -t commands 'qsv clipboard commands' commands "$@"
}
(( $+functions[_qsv__color_commands] )) ||
_qsv__color_commands() {
    local commands; commands=()
    _describe -t commands 'qsv color commands' commands "$@"
}
(( $+functions[_qsv__count_commands] )) ||
_qsv__count_commands() {
    local commands; commands=()
    _describe -t commands 'qsv count commands' commands "$@"
}
(( $+functions[_qsv__datefmt_commands] )) ||
_qsv__datefmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv datefmt commands' commands "$@"
}
(( $+functions[_qsv__dedup_commands] )) ||
_qsv__dedup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv dedup commands' commands "$@"
}
(( $+functions[_qsv__describegpt_commands] )) ||
_qsv__describegpt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv describegpt commands' commands "$@"
}
(( $+functions[_qsv__diff_commands] )) ||
_qsv__diff_commands() {
    local commands; commands=()
    _describe -t commands 'qsv diff commands' commands "$@"
}
(( $+functions[_qsv__edit_commands] )) ||
_qsv__edit_commands() {
    local commands; commands=()
    _describe -t commands 'qsv edit commands' commands "$@"
}
(( $+functions[_qsv__enum_commands] )) ||
_qsv__enum_commands() {
    local commands; commands=()
    _describe -t commands 'qsv enum commands' commands "$@"
}
(( $+functions[_qsv__excel_commands] )) ||
_qsv__excel_commands() {
    local commands; commands=()
    _describe -t commands 'qsv excel commands' commands "$@"
}
(( $+functions[_qsv__exclude_commands] )) ||
_qsv__exclude_commands() {
    local commands; commands=()
    _describe -t commands 'qsv exclude commands' commands "$@"
}
(( $+functions[_qsv__explode_commands] )) ||
_qsv__explode_commands() {
    local commands; commands=()
    _describe -t commands 'qsv explode commands' commands "$@"
}
(( $+functions[_qsv__extdedup_commands] )) ||
_qsv__extdedup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv extdedup commands' commands "$@"
}
(( $+functions[_qsv__extsort_commands] )) ||
_qsv__extsort_commands() {
    local commands; commands=()
    _describe -t commands 'qsv extsort commands' commands "$@"
}
(( $+functions[_qsv__fetch_commands] )) ||
_qsv__fetch_commands() {
    local commands; commands=()
    _describe -t commands 'qsv fetch commands' commands "$@"
}
(( $+functions[_qsv__fetchpost_commands] )) ||
_qsv__fetchpost_commands() {
    local commands; commands=()
    _describe -t commands 'qsv fetchpost commands' commands "$@"
}
(( $+functions[_qsv__fill_commands] )) ||
_qsv__fill_commands() {
    local commands; commands=()
    _describe -t commands 'qsv fill commands' commands "$@"
}
(( $+functions[_qsv__fixlengths_commands] )) ||
_qsv__fixlengths_commands() {
    local commands; commands=()
    _describe -t commands 'qsv fixlengths commands' commands "$@"
}
(( $+functions[_qsv__flatten_commands] )) ||
_qsv__flatten_commands() {
    local commands; commands=()
    _describe -t commands 'qsv flatten commands' commands "$@"
}
(( $+functions[_qsv__fmt_commands] )) ||
_qsv__fmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv fmt commands' commands "$@"
}
(( $+functions[_qsv__foreach_commands] )) ||
_qsv__foreach_commands() {
    local commands; commands=()
    _describe -t commands 'qsv foreach commands' commands "$@"
}
(( $+functions[_qsv__frequency_commands] )) ||
_qsv__frequency_commands() {
    local commands; commands=()
    _describe -t commands 'qsv frequency commands' commands "$@"
}
(( $+functions[_qsv__geocode_commands] )) ||
_qsv__geocode_commands() {
    local commands; commands=(
'countryinfo:' \
'countryinfonow:' \
'index-check:' \
'index-load:' \
'index-reset:' \
'index-update:' \
'iplookup:' \
'iplookupnow:' \
'reverse:' \
'reversenow:' \
'suggest:' \
'suggestnow:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv geocode commands' commands "$@"
}
(( $+functions[_qsv__geocode__countryinfo_commands] )) ||
_qsv__geocode__countryinfo_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode countryinfo commands' commands "$@"
}
(( $+functions[_qsv__geocode__countryinfonow_commands] )) ||
_qsv__geocode__countryinfonow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode countryinfonow commands' commands "$@"
}
(( $+functions[_qsv__geocode__help_commands] )) ||
_qsv__geocode__help_commands() {
    local commands; commands=(
'countryinfo:' \
'countryinfonow:' \
'index-check:' \
'index-load:' \
'index-reset:' \
'index-update:' \
'iplookup:' \
'iplookupnow:' \
'reverse:' \
'reversenow:' \
'suggest:' \
'suggestnow:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv geocode help commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__countryinfo_commands] )) ||
_qsv__geocode__help__countryinfo_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help countryinfo commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__countryinfonow_commands] )) ||
_qsv__geocode__help__countryinfonow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help countryinfonow commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__help_commands] )) ||
_qsv__geocode__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help help commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__index-check_commands] )) ||
_qsv__geocode__help__index-check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help index-check commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__index-load_commands] )) ||
_qsv__geocode__help__index-load_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help index-load commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__index-reset_commands] )) ||
_qsv__geocode__help__index-reset_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help index-reset commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__index-update_commands] )) ||
_qsv__geocode__help__index-update_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help index-update commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__iplookup_commands] )) ||
_qsv__geocode__help__iplookup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help iplookup commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__iplookupnow_commands] )) ||
_qsv__geocode__help__iplookupnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help iplookupnow commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__reverse_commands] )) ||
_qsv__geocode__help__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help reverse commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__reversenow_commands] )) ||
_qsv__geocode__help__reversenow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help reversenow commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__suggest_commands] )) ||
_qsv__geocode__help__suggest_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help suggest commands' commands "$@"
}
(( $+functions[_qsv__geocode__help__suggestnow_commands] )) ||
_qsv__geocode__help__suggestnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help suggestnow commands' commands "$@"
}
(( $+functions[_qsv__geocode__index-check_commands] )) ||
_qsv__geocode__index-check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode index-check commands' commands "$@"
}
(( $+functions[_qsv__geocode__index-load_commands] )) ||
_qsv__geocode__index-load_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode index-load commands' commands "$@"
}
(( $+functions[_qsv__geocode__index-reset_commands] )) ||
_qsv__geocode__index-reset_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode index-reset commands' commands "$@"
}
(( $+functions[_qsv__geocode__index-update_commands] )) ||
_qsv__geocode__index-update_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode index-update commands' commands "$@"
}
(( $+functions[_qsv__geocode__iplookup_commands] )) ||
_qsv__geocode__iplookup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode iplookup commands' commands "$@"
}
(( $+functions[_qsv__geocode__iplookupnow_commands] )) ||
_qsv__geocode__iplookupnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode iplookupnow commands' commands "$@"
}
(( $+functions[_qsv__geocode__reverse_commands] )) ||
_qsv__geocode__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode reverse commands' commands "$@"
}
(( $+functions[_qsv__geocode__reversenow_commands] )) ||
_qsv__geocode__reversenow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode reversenow commands' commands "$@"
}
(( $+functions[_qsv__geocode__suggest_commands] )) ||
_qsv__geocode__suggest_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode suggest commands' commands "$@"
}
(( $+functions[_qsv__geocode__suggestnow_commands] )) ||
_qsv__geocode__suggestnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode suggestnow commands' commands "$@"
}
(( $+functions[_qsv__geoconvert_commands] )) ||
_qsv__geoconvert_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geoconvert commands' commands "$@"
}
(( $+functions[_qsv__headers_commands] )) ||
_qsv__headers_commands() {
    local commands; commands=()
    _describe -t commands 'qsv headers commands' commands "$@"
}
(( $+functions[_qsv__help_commands] )) ||
_qsv__help_commands() {
    local commands; commands=(
'apply:' \
'behead:' \
'cat:' \
'clipboard:' \
'color:' \
'count:' \
'datefmt:' \
'dedup:' \
'describegpt:' \
'diff:' \
'edit:' \
'enum:' \
'excel:' \
'exclude:' \
'explode:' \
'extdedup:' \
'extsort:' \
'fetch:' \
'fetchpost:' \
'fill:' \
'fixlengths:' \
'flatten:' \
'fmt:' \
'foreach:' \
'frequency:' \
'geocode:' \
'geoconvert:' \
'headers:' \
'index:' \
'input:' \
'join:' \
'joinp:' \
'json:' \
'jsonl:' \
'lens:' \
'luau:' \
'moarstats:' \
'partition:' \
'pivotp:' \
'pro:' \
'prompt:' \
'pseudo:' \
'py:' \
'rename:' \
'replace:' \
'reverse:' \
'safenames:' \
'sample:' \
'schema:' \
'search:' \
'searchset:' \
'select:' \
'slice:' \
'snappy:' \
'sniff:' \
'sort:' \
'sortcheck:' \
'split:' \
'sqlp:' \
'stats:' \
'table:' \
'template:' \
'to:' \
'tojsonl:' \
'transpose:' \
'validate:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv help commands' commands "$@"
}
(( $+functions[_qsv__help__apply_commands] )) ||
_qsv__help__apply_commands() {
    local commands; commands=(
'calcconv:' \
'dynfmt:' \
'emptyreplace:' \
'operations:' \
    )
    _describe -t commands 'qsv help apply commands' commands "$@"
}
(( $+functions[_qsv__help__apply__calcconv_commands] )) ||
_qsv__help__apply__calcconv_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help apply calcconv commands' commands "$@"
}
(( $+functions[_qsv__help__apply__dynfmt_commands] )) ||
_qsv__help__apply__dynfmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help apply dynfmt commands' commands "$@"
}
(( $+functions[_qsv__help__apply__emptyreplace_commands] )) ||
_qsv__help__apply__emptyreplace_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help apply emptyreplace commands' commands "$@"
}
(( $+functions[_qsv__help__apply__operations_commands] )) ||
_qsv__help__apply__operations_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help apply operations commands' commands "$@"
}
(( $+functions[_qsv__help__behead_commands] )) ||
_qsv__help__behead_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help behead commands' commands "$@"
}
(( $+functions[_qsv__help__cat_commands] )) ||
_qsv__help__cat_commands() {
    local commands; commands=(
'columns:' \
'rows:' \
'rowskey:' \
    )
    _describe -t commands 'qsv help cat commands' commands "$@"
}
(( $+functions[_qsv__help__cat__columns_commands] )) ||
_qsv__help__cat__columns_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help cat columns commands' commands "$@"
}
(( $+functions[_qsv__help__cat__rows_commands] )) ||
_qsv__help__cat__rows_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help cat rows commands' commands "$@"
}
(( $+functions[_qsv__help__cat__rowskey_commands] )) ||
_qsv__help__cat__rowskey_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help cat rowskey commands' commands "$@"
}
(( $+functions[_qsv__help__clipboard_commands] )) ||
_qsv__help__clipboard_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help clipboard commands' commands "$@"
}
(( $+functions[_qsv__help__color_commands] )) ||
_qsv__help__color_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help color commands' commands "$@"
}
(( $+functions[_qsv__help__count_commands] )) ||
_qsv__help__count_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help count commands' commands "$@"
}
(( $+functions[_qsv__help__datefmt_commands] )) ||
_qsv__help__datefmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help datefmt commands' commands "$@"
}
(( $+functions[_qsv__help__dedup_commands] )) ||
_qsv__help__dedup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help dedup commands' commands "$@"
}
(( $+functions[_qsv__help__describegpt_commands] )) ||
_qsv__help__describegpt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help describegpt commands' commands "$@"
}
(( $+functions[_qsv__help__diff_commands] )) ||
_qsv__help__diff_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help diff commands' commands "$@"
}
(( $+functions[_qsv__help__edit_commands] )) ||
_qsv__help__edit_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help edit commands' commands "$@"
}
(( $+functions[_qsv__help__enum_commands] )) ||
_qsv__help__enum_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help enum commands' commands "$@"
}
(( $+functions[_qsv__help__excel_commands] )) ||
_qsv__help__excel_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help excel commands' commands "$@"
}
(( $+functions[_qsv__help__exclude_commands] )) ||
_qsv__help__exclude_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help exclude commands' commands "$@"
}
(( $+functions[_qsv__help__explode_commands] )) ||
_qsv__help__explode_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help explode commands' commands "$@"
}
(( $+functions[_qsv__help__extdedup_commands] )) ||
_qsv__help__extdedup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help extdedup commands' commands "$@"
}
(( $+functions[_qsv__help__extsort_commands] )) ||
_qsv__help__extsort_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help extsort commands' commands "$@"
}
(( $+functions[_qsv__help__fetch_commands] )) ||
_qsv__help__fetch_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help fetch commands' commands "$@"
}
(( $+functions[_qsv__help__fetchpost_commands] )) ||
_qsv__help__fetchpost_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help fetchpost commands' commands "$@"
}
(( $+functions[_qsv__help__fill_commands] )) ||
_qsv__help__fill_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help fill commands' commands "$@"
}
(( $+functions[_qsv__help__fixlengths_commands] )) ||
_qsv__help__fixlengths_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help fixlengths commands' commands "$@"
}
(( $+functions[_qsv__help__flatten_commands] )) ||
_qsv__help__flatten_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help flatten commands' commands "$@"
}
(( $+functions[_qsv__help__fmt_commands] )) ||
_qsv__help__fmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help fmt commands' commands "$@"
}
(( $+functions[_qsv__help__foreach_commands] )) ||
_qsv__help__foreach_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help foreach commands' commands "$@"
}
(( $+functions[_qsv__help__frequency_commands] )) ||
_qsv__help__frequency_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help frequency commands' commands "$@"
}
(( $+functions[_qsv__help__geocode_commands] )) ||
_qsv__help__geocode_commands() {
    local commands; commands=(
'countryinfo:' \
'countryinfonow:' \
'index-check:' \
'index-load:' \
'index-reset:' \
'index-update:' \
'iplookup:' \
'iplookupnow:' \
'reverse:' \
'reversenow:' \
'suggest:' \
'suggestnow:' \
    )
    _describe -t commands 'qsv help geocode commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__countryinfo_commands] )) ||
_qsv__help__geocode__countryinfo_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode countryinfo commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__countryinfonow_commands] )) ||
_qsv__help__geocode__countryinfonow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode countryinfonow commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__index-check_commands] )) ||
_qsv__help__geocode__index-check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode index-check commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__index-load_commands] )) ||
_qsv__help__geocode__index-load_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode index-load commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__index-reset_commands] )) ||
_qsv__help__geocode__index-reset_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode index-reset commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__index-update_commands] )) ||
_qsv__help__geocode__index-update_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode index-update commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__iplookup_commands] )) ||
_qsv__help__geocode__iplookup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode iplookup commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__iplookupnow_commands] )) ||
_qsv__help__geocode__iplookupnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode iplookupnow commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__reverse_commands] )) ||
_qsv__help__geocode__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode reverse commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__reversenow_commands] )) ||
_qsv__help__geocode__reversenow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode reversenow commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__suggest_commands] )) ||
_qsv__help__geocode__suggest_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode suggest commands' commands "$@"
}
(( $+functions[_qsv__help__geocode__suggestnow_commands] )) ||
_qsv__help__geocode__suggestnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode suggestnow commands' commands "$@"
}
(( $+functions[_qsv__help__geoconvert_commands] )) ||
_qsv__help__geoconvert_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geoconvert commands' commands "$@"
}
(( $+functions[_qsv__help__headers_commands] )) ||
_qsv__help__headers_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help headers commands' commands "$@"
}
(( $+functions[_qsv__help__help_commands] )) ||
_qsv__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help help commands' commands "$@"
}
(( $+functions[_qsv__help__index_commands] )) ||
_qsv__help__index_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help index commands' commands "$@"
}
(( $+functions[_qsv__help__input_commands] )) ||
_qsv__help__input_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help input commands' commands "$@"
}
(( $+functions[_qsv__help__join_commands] )) ||
_qsv__help__join_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help join commands' commands "$@"
}
(( $+functions[_qsv__help__joinp_commands] )) ||
_qsv__help__joinp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help joinp commands' commands "$@"
}
(( $+functions[_qsv__help__json_commands] )) ||
_qsv__help__json_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help json commands' commands "$@"
}
(( $+functions[_qsv__help__jsonl_commands] )) ||
_qsv__help__jsonl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help jsonl commands' commands "$@"
}
(( $+functions[_qsv__help__lens_commands] )) ||
_qsv__help__lens_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help lens commands' commands "$@"
}
(( $+functions[_qsv__help__luau_commands] )) ||
_qsv__help__luau_commands() {
    local commands; commands=(
'filter:' \
'map:' \
    )
    _describe -t commands 'qsv help luau commands' commands "$@"
}
(( $+functions[_qsv__help__luau__filter_commands] )) ||
_qsv__help__luau__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help luau filter commands' commands "$@"
}
(( $+functions[_qsv__help__luau__map_commands] )) ||
_qsv__help__luau__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help luau map commands' commands "$@"
}
(( $+functions[_qsv__help__moarstats_commands] )) ||
_qsv__help__moarstats_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help moarstats commands' commands "$@"
}
(( $+functions[_qsv__help__partition_commands] )) ||
_qsv__help__partition_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help partition commands' commands "$@"
}
(( $+functions[_qsv__help__pivotp_commands] )) ||
_qsv__help__pivotp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help pivotp commands' commands "$@"
}
(( $+functions[_qsv__help__pro_commands] )) ||
_qsv__help__pro_commands() {
    local commands; commands=(
'lens:' \
'workflow:' \
    )
    _describe -t commands 'qsv help pro commands' commands "$@"
}
(( $+functions[_qsv__help__pro__lens_commands] )) ||
_qsv__help__pro__lens_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help pro lens commands' commands "$@"
}
(( $+functions[_qsv__help__pro__workflow_commands] )) ||
_qsv__help__pro__workflow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help pro workflow commands' commands "$@"
}
(( $+functions[_qsv__help__prompt_commands] )) ||
_qsv__help__prompt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help prompt commands' commands "$@"
}
(( $+functions[_qsv__help__pseudo_commands] )) ||
_qsv__help__pseudo_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help pseudo commands' commands "$@"
}
(( $+functions[_qsv__help__py_commands] )) ||
_qsv__help__py_commands() {
    local commands; commands=(
'filter:' \
'map:' \
    )
    _describe -t commands 'qsv help py commands' commands "$@"
}
(( $+functions[_qsv__help__py__filter_commands] )) ||
_qsv__help__py__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help py filter commands' commands "$@"
}
(( $+functions[_qsv__help__py__map_commands] )) ||
_qsv__help__py__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help py map commands' commands "$@"
}
(( $+functions[_qsv__help__rename_commands] )) ||
_qsv__help__rename_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help rename commands' commands "$@"
}
(( $+functions[_qsv__help__replace_commands] )) ||
_qsv__help__replace_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help replace commands' commands "$@"
}
(( $+functions[_qsv__help__reverse_commands] )) ||
_qsv__help__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help reverse commands' commands "$@"
}
(( $+functions[_qsv__help__safenames_commands] )) ||
_qsv__help__safenames_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help safenames commands' commands "$@"
}
(( $+functions[_qsv__help__sample_commands] )) ||
_qsv__help__sample_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help sample commands' commands "$@"
}
(( $+functions[_qsv__help__schema_commands] )) ||
_qsv__help__schema_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help schema commands' commands "$@"
}
(( $+functions[_qsv__help__search_commands] )) ||
_qsv__help__search_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help search commands' commands "$@"
}
(( $+functions[_qsv__help__searchset_commands] )) ||
_qsv__help__searchset_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help searchset commands' commands "$@"
}
(( $+functions[_qsv__help__select_commands] )) ||
_qsv__help__select_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help select commands' commands "$@"
}
(( $+functions[_qsv__help__slice_commands] )) ||
_qsv__help__slice_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help slice commands' commands "$@"
}
(( $+functions[_qsv__help__snappy_commands] )) ||
_qsv__help__snappy_commands() {
    local commands; commands=(
'check:' \
'compress:' \
'decompress:' \
'validate:' \
    )
    _describe -t commands 'qsv help snappy commands' commands "$@"
}
(( $+functions[_qsv__help__snappy__check_commands] )) ||
_qsv__help__snappy__check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help snappy check commands' commands "$@"
}
(( $+functions[_qsv__help__snappy__compress_commands] )) ||
_qsv__help__snappy__compress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help snappy compress commands' commands "$@"
}
(( $+functions[_qsv__help__snappy__decompress_commands] )) ||
_qsv__help__snappy__decompress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help snappy decompress commands' commands "$@"
}
(( $+functions[_qsv__help__snappy__validate_commands] )) ||
_qsv__help__snappy__validate_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help snappy validate commands' commands "$@"
}
(( $+functions[_qsv__help__sniff_commands] )) ||
_qsv__help__sniff_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help sniff commands' commands "$@"
}
(( $+functions[_qsv__help__sort_commands] )) ||
_qsv__help__sort_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help sort commands' commands "$@"
}
(( $+functions[_qsv__help__sortcheck_commands] )) ||
_qsv__help__sortcheck_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help sortcheck commands' commands "$@"
}
(( $+functions[_qsv__help__split_commands] )) ||
_qsv__help__split_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help split commands' commands "$@"
}
(( $+functions[_qsv__help__sqlp_commands] )) ||
_qsv__help__sqlp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help sqlp commands' commands "$@"
}
(( $+functions[_qsv__help__stats_commands] )) ||
_qsv__help__stats_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help stats commands' commands "$@"
}
(( $+functions[_qsv__help__table_commands] )) ||
_qsv__help__table_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help table commands' commands "$@"
}
(( $+functions[_qsv__help__template_commands] )) ||
_qsv__help__template_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help template commands' commands "$@"
}
(( $+functions[_qsv__help__to_commands] )) ||
_qsv__help__to_commands() {
    local commands; commands=(
'datapackage:' \
'ods:' \
'postgres:' \
'sqlite:' \
'xlsx:' \
    )
    _describe -t commands 'qsv help to commands' commands "$@"
}
(( $+functions[_qsv__help__to__datapackage_commands] )) ||
_qsv__help__to__datapackage_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to datapackage commands' commands "$@"
}
(( $+functions[_qsv__help__to__ods_commands] )) ||
_qsv__help__to__ods_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to ods commands' commands "$@"
}
(( $+functions[_qsv__help__to__postgres_commands] )) ||
_qsv__help__to__postgres_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to postgres commands' commands "$@"
}
(( $+functions[_qsv__help__to__sqlite_commands] )) ||
_qsv__help__to__sqlite_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to sqlite commands' commands "$@"
}
(( $+functions[_qsv__help__to__xlsx_commands] )) ||
_qsv__help__to__xlsx_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to xlsx commands' commands "$@"
}
(( $+functions[_qsv__help__tojsonl_commands] )) ||
_qsv__help__tojsonl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help tojsonl commands' commands "$@"
}
(( $+functions[_qsv__help__transpose_commands] )) ||
_qsv__help__transpose_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help transpose commands' commands "$@"
}
(( $+functions[_qsv__help__validate_commands] )) ||
_qsv__help__validate_commands() {
    local commands; commands=(
'schema:' \
    )
    _describe -t commands 'qsv help validate commands' commands "$@"
}
(( $+functions[_qsv__help__validate__schema_commands] )) ||
_qsv__help__validate__schema_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help validate schema commands' commands "$@"
}
(( $+functions[_qsv__index_commands] )) ||
_qsv__index_commands() {
    local commands; commands=()
    _describe -t commands 'qsv index commands' commands "$@"
}
(( $+functions[_qsv__input_commands] )) ||
_qsv__input_commands() {
    local commands; commands=()
    _describe -t commands 'qsv input commands' commands "$@"
}
(( $+functions[_qsv__join_commands] )) ||
_qsv__join_commands() {
    local commands; commands=()
    _describe -t commands 'qsv join commands' commands "$@"
}
(( $+functions[_qsv__joinp_commands] )) ||
_qsv__joinp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv joinp commands' commands "$@"
}
(( $+functions[_qsv__json_commands] )) ||
_qsv__json_commands() {
    local commands; commands=()
    _describe -t commands 'qsv json commands' commands "$@"
}
(( $+functions[_qsv__jsonl_commands] )) ||
_qsv__jsonl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv jsonl commands' commands "$@"
}
(( $+functions[_qsv__lens_commands] )) ||
_qsv__lens_commands() {
    local commands; commands=()
    _describe -t commands 'qsv lens commands' commands "$@"
}
(( $+functions[_qsv__luau_commands] )) ||
_qsv__luau_commands() {
    local commands; commands=(
'filter:' \
'map:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv luau commands' commands "$@"
}
(( $+functions[_qsv__luau__filter_commands] )) ||
_qsv__luau__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv luau filter commands' commands "$@"
}
(( $+functions[_qsv__luau__help_commands] )) ||
_qsv__luau__help_commands() {
    local commands; commands=(
'filter:' \
'map:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv luau help commands' commands "$@"
}
(( $+functions[_qsv__luau__help__filter_commands] )) ||
_qsv__luau__help__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv luau help filter commands' commands "$@"
}
(( $+functions[_qsv__luau__help__help_commands] )) ||
_qsv__luau__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv luau help help commands' commands "$@"
}
(( $+functions[_qsv__luau__help__map_commands] )) ||
_qsv__luau__help__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv luau help map commands' commands "$@"
}
(( $+functions[_qsv__luau__map_commands] )) ||
_qsv__luau__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv luau map commands' commands "$@"
}
(( $+functions[_qsv__moarstats_commands] )) ||
_qsv__moarstats_commands() {
    local commands; commands=()
    _describe -t commands 'qsv moarstats commands' commands "$@"
}
(( $+functions[_qsv__partition_commands] )) ||
_qsv__partition_commands() {
    local commands; commands=()
    _describe -t commands 'qsv partition commands' commands "$@"
}
(( $+functions[_qsv__pivotp_commands] )) ||
_qsv__pivotp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pivotp commands' commands "$@"
}
(( $+functions[_qsv__pro_commands] )) ||
_qsv__pro_commands() {
    local commands; commands=(
'lens:' \
'workflow:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv pro commands' commands "$@"
}
(( $+functions[_qsv__pro__help_commands] )) ||
_qsv__pro__help_commands() {
    local commands; commands=(
'lens:' \
'workflow:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv pro help commands' commands "$@"
}
(( $+functions[_qsv__pro__help__help_commands] )) ||
_qsv__pro__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pro help help commands' commands "$@"
}
(( $+functions[_qsv__pro__help__lens_commands] )) ||
_qsv__pro__help__lens_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pro help lens commands' commands "$@"
}
(( $+functions[_qsv__pro__help__workflow_commands] )) ||
_qsv__pro__help__workflow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pro help workflow commands' commands "$@"
}
(( $+functions[_qsv__pro__lens_commands] )) ||
_qsv__pro__lens_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pro lens commands' commands "$@"
}
(( $+functions[_qsv__pro__workflow_commands] )) ||
_qsv__pro__workflow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pro workflow commands' commands "$@"
}
(( $+functions[_qsv__prompt_commands] )) ||
_qsv__prompt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv prompt commands' commands "$@"
}
(( $+functions[_qsv__pseudo_commands] )) ||
_qsv__pseudo_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pseudo commands' commands "$@"
}
(( $+functions[_qsv__py_commands] )) ||
_qsv__py_commands() {
    local commands; commands=(
'filter:' \
'map:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv py commands' commands "$@"
}
(( $+functions[_qsv__py__filter_commands] )) ||
_qsv__py__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv py filter commands' commands "$@"
}
(( $+functions[_qsv__py__help_commands] )) ||
_qsv__py__help_commands() {
    local commands; commands=(
'filter:' \
'map:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv py help commands' commands "$@"
}
(( $+functions[_qsv__py__help__filter_commands] )) ||
_qsv__py__help__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv py help filter commands' commands "$@"
}
(( $+functions[_qsv__py__help__help_commands] )) ||
_qsv__py__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv py help help commands' commands "$@"
}
(( $+functions[_qsv__py__help__map_commands] )) ||
_qsv__py__help__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv py help map commands' commands "$@"
}
(( $+functions[_qsv__py__map_commands] )) ||
_qsv__py__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv py map commands' commands "$@"
}
(( $+functions[_qsv__rename_commands] )) ||
_qsv__rename_commands() {
    local commands; commands=()
    _describe -t commands 'qsv rename commands' commands "$@"
}
(( $+functions[_qsv__replace_commands] )) ||
_qsv__replace_commands() {
    local commands; commands=()
    _describe -t commands 'qsv replace commands' commands "$@"
}
(( $+functions[_qsv__reverse_commands] )) ||
_qsv__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'qsv reverse commands' commands "$@"
}
(( $+functions[_qsv__safenames_commands] )) ||
_qsv__safenames_commands() {
    local commands; commands=()
    _describe -t commands 'qsv safenames commands' commands "$@"
}
(( $+functions[_qsv__sample_commands] )) ||
_qsv__sample_commands() {
    local commands; commands=()
    _describe -t commands 'qsv sample commands' commands "$@"
}
(( $+functions[_qsv__schema_commands] )) ||
_qsv__schema_commands() {
    local commands; commands=()
    _describe -t commands 'qsv schema commands' commands "$@"
}
(( $+functions[_qsv__search_commands] )) ||
_qsv__search_commands() {
    local commands; commands=()
    _describe -t commands 'qsv search commands' commands "$@"
}
(( $+functions[_qsv__searchset_commands] )) ||
_qsv__searchset_commands() {
    local commands; commands=()
    _describe -t commands 'qsv searchset commands' commands "$@"
}
(( $+functions[_qsv__select_commands] )) ||
_qsv__select_commands() {
    local commands; commands=()
    _describe -t commands 'qsv select commands' commands "$@"
}
(( $+functions[_qsv__slice_commands] )) ||
_qsv__slice_commands() {
    local commands; commands=()
    _describe -t commands 'qsv slice commands' commands "$@"
}
(( $+functions[_qsv__snappy_commands] )) ||
_qsv__snappy_commands() {
    local commands; commands=(
'check:' \
'compress:' \
'decompress:' \
'validate:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv snappy commands' commands "$@"
}
(( $+functions[_qsv__snappy__check_commands] )) ||
_qsv__snappy__check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy check commands' commands "$@"
}
(( $+functions[_qsv__snappy__compress_commands] )) ||
_qsv__snappy__compress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy compress commands' commands "$@"
}
(( $+functions[_qsv__snappy__decompress_commands] )) ||
_qsv__snappy__decompress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy decompress commands' commands "$@"
}
(( $+functions[_qsv__snappy__help_commands] )) ||
_qsv__snappy__help_commands() {
    local commands; commands=(
'check:' \
'compress:' \
'decompress:' \
'validate:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv snappy help commands' commands "$@"
}
(( $+functions[_qsv__snappy__help__check_commands] )) ||
_qsv__snappy__help__check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy help check commands' commands "$@"
}
(( $+functions[_qsv__snappy__help__compress_commands] )) ||
_qsv__snappy__help__compress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy help compress commands' commands "$@"
}
(( $+functions[_qsv__snappy__help__decompress_commands] )) ||
_qsv__snappy__help__decompress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy help decompress commands' commands "$@"
}
(( $+functions[_qsv__snappy__help__help_commands] )) ||
_qsv__snappy__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy help help commands' commands "$@"
}
(( $+functions[_qsv__snappy__help__validate_commands] )) ||
_qsv__snappy__help__validate_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy help validate commands' commands "$@"
}
(( $+functions[_qsv__snappy__validate_commands] )) ||
_qsv__snappy__validate_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy validate commands' commands "$@"
}
(( $+functions[_qsv__sniff_commands] )) ||
_qsv__sniff_commands() {
    local commands; commands=()
    _describe -t commands 'qsv sniff commands' commands "$@"
}
(( $+functions[_qsv__sort_commands] )) ||
_qsv__sort_commands() {
    local commands; commands=()
    _describe -t commands 'qsv sort commands' commands "$@"
}
(( $+functions[_qsv__sortcheck_commands] )) ||
_qsv__sortcheck_commands() {
    local commands; commands=()
    _describe -t commands 'qsv sortcheck commands' commands "$@"
}
(( $+functions[_qsv__split_commands] )) ||
_qsv__split_commands() {
    local commands; commands=()
    _describe -t commands 'qsv split commands' commands "$@"
}
(( $+functions[_qsv__sqlp_commands] )) ||
_qsv__sqlp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv sqlp commands' commands "$@"
}
(( $+functions[_qsv__stats_commands] )) ||
_qsv__stats_commands() {
    local commands; commands=()
    _describe -t commands 'qsv stats commands' commands "$@"
}
(( $+functions[_qsv__table_commands] )) ||
_qsv__table_commands() {
    local commands; commands=()
    _describe -t commands 'qsv table commands' commands "$@"
}
(( $+functions[_qsv__template_commands] )) ||
_qsv__template_commands() {
    local commands; commands=()
    _describe -t commands 'qsv template commands' commands "$@"
}
(( $+functions[_qsv__to_commands] )) ||
_qsv__to_commands() {
    local commands; commands=(
'datapackage:' \
'ods:' \
'postgres:' \
'sqlite:' \
'xlsx:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv to commands' commands "$@"
}
(( $+functions[_qsv__to__datapackage_commands] )) ||
_qsv__to__datapackage_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to datapackage commands' commands "$@"
}
(( $+functions[_qsv__to__help_commands] )) ||
_qsv__to__help_commands() {
    local commands; commands=(
'datapackage:' \
'ods:' \
'postgres:' \
'sqlite:' \
'xlsx:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv to help commands' commands "$@"
}
(( $+functions[_qsv__to__help__datapackage_commands] )) ||
_qsv__to__help__datapackage_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help datapackage commands' commands "$@"
}
(( $+functions[_qsv__to__help__help_commands] )) ||
_qsv__to__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help help commands' commands "$@"
}
(( $+functions[_qsv__to__help__ods_commands] )) ||
_qsv__to__help__ods_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help ods commands' commands "$@"
}
(( $+functions[_qsv__to__help__postgres_commands] )) ||
_qsv__to__help__postgres_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help postgres commands' commands "$@"
}
(( $+functions[_qsv__to__help__sqlite_commands] )) ||
_qsv__to__help__sqlite_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help sqlite commands' commands "$@"
}
(( $+functions[_qsv__to__help__xlsx_commands] )) ||
_qsv__to__help__xlsx_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help xlsx commands' commands "$@"
}
(( $+functions[_qsv__to__ods_commands] )) ||
_qsv__to__ods_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to ods commands' commands "$@"
}
(( $+functions[_qsv__to__postgres_commands] )) ||
_qsv__to__postgres_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to postgres commands' commands "$@"
}
(( $+functions[_qsv__to__sqlite_commands] )) ||
_qsv__to__sqlite_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to sqlite commands' commands "$@"
}
(( $+functions[_qsv__to__xlsx_commands] )) ||
_qsv__to__xlsx_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to xlsx commands' commands "$@"
}
(( $+functions[_qsv__tojsonl_commands] )) ||
_qsv__tojsonl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv tojsonl commands' commands "$@"
}
(( $+functions[_qsv__transpose_commands] )) ||
_qsv__transpose_commands() {
    local commands; commands=()
    _describe -t commands 'qsv transpose commands' commands "$@"
}
(( $+functions[_qsv__validate_commands] )) ||
_qsv__validate_commands() {
    local commands; commands=(
'schema:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv validate commands' commands "$@"
}
(( $+functions[_qsv__validate__help_commands] )) ||
_qsv__validate__help_commands() {
    local commands; commands=(
'schema:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv validate help commands' commands "$@"
}
(( $+functions[_qsv__validate__help__help_commands] )) ||
_qsv__validate__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv validate help help commands' commands "$@"
}
(( $+functions[_qsv__validate__help__schema_commands] )) ||
_qsv__validate__help__schema_commands() {
    local commands; commands=()
    _describe -t commands 'qsv validate help schema commands' commands "$@"
}
(( $+functions[_qsv__validate__schema_commands] )) ||
_qsv__validate__schema_commands() {
    local commands; commands=()
    _describe -t commands 'qsv validate schema commands' commands "$@"
}

if [ "$funcstack[1]" = "_qsv" ]; then
    _qsv "$@"
else
    compdef _qsv qsv
fi
