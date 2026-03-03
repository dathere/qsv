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
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
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
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
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
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
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
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
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
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
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
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'--flexible[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--pad[]' \
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
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'--flexible[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--pad[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(rows)
_arguments "${_arguments_options[@]}" : \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'--flexible[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--pad[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(rowskey)
_arguments "${_arguments_options[@]}" : \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'--flexible[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--pad[]' \
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
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--memcheck[]' \
'-C[]' \
'--color[]' \
'-n[]' \
'--row-numbers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(count)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--json[]' \
'--width[]' \
'--width-no-delims[]' \
'--low-memory[]' \
'-H[]' \
'--human-readable[]' \
'-f[]' \
'--flexible[]' \
'-n[]' \
'--no-headers[]' \
'--no-polars[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(datefmt)
_arguments "${_arguments_options[@]}" : \
'-R+[]: :_default' \
'--ts-resolution=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--default-tz=[]: :_default' \
'--output-tz=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--input-tz=[]: :_default' \
'--formatstr=[]: :_default' \
'--utc[]' \
'--zulu[]' \
'--keep-zero-time[]' \
'--prefer-dmy[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(dedup)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-D+[]: :_default' \
'--dupes-output=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-H[]' \
'--human-readable[]' \
'-q[]' \
'--quiet[]' \
'-i[]' \
'--ignore-case[]' \
'-n[]' \
'--no-headers[]' \
'--memcheck[]' \
'--sorted[]' \
'-N[]' \
'--numeric[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(describegpt)
_arguments "${_arguments_options[@]}" : \
'--disk-cache-dir=[]: :_default' \
'--addl-cols-list=[]: :_default' \
'--format=[]: :_default' \
'--language=[]: :_default' \
'--export-prompt=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--addl-props=[]: :_default' \
'-u+[]: :_default' \
'--base-url=[]: :_default' \
'--ckan-token=[]: :_default' \
'--session-len=[]: :_default' \
'--freq-options=[]: :_default' \
'--timeout=[]: :_default' \
'--sql-results=[]: :_default' \
'-k+[]: :_default' \
'--api-key=[]: :_default' \
'-p+[]: :_default' \
'--prompt=[]: :_default' \
'--enum-threshold=[]: :_default' \
'--ckan-api=[]: :_default' \
'-m+[]: :_default' \
'--model=[]: :_default' \
'--prompt-file=[]: :_default' \
'--session=[]: :_default' \
'--tag-vocab=[]: :_default' \
'--truncate-str=[]: :_default' \
'--stats-options=[]: :_default' \
'--sample-size=[]: :_default' \
'--user-agent=[]: :_default' \
'--num-examples=[]: :_default' \
'-t+[]: :_default' \
'--max-tokens=[]: :_default' \
'--num-tags=[]: :_default' \
'--cache-dir=[]: :_default' \
'-A[]' \
'--all[]' \
'-q[]' \
'--quiet[]' \
'--fewshot-examples[]' \
'--fresh[]' \
'--redis-cache[]' \
'--addl-cols[]' \
'--tags[]' \
'--description[]' \
'--forget[]' \
'--no-cache[]' \
'--flush-cache[]' \
'--dictionary[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(diff)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--delimiter-right=[]: :_default' \
'-k+[]: :_default' \
'--key=[]: :_default' \
'--delimiter-left=[]: :_default' \
'--delimiter-output=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--sort-columns=[]: :_default' \
'--drop-equal-fields[]' \
'--no-headers-left[]' \
'--no-headers-right[]' \
'--no-headers-output[]' \
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
'--start=[]: :_default' \
'--copy=[]: :_default' \
'--increment=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--constant=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--hash=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--uuid7[]' \
'--uuid4[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(excel)
_arguments "${_arguments_options[@]}" : \
'--error-format=[]: :_default' \
'-s+[]: :_default' \
'--sheet=[]: :_default' \
'--table=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--metadata=[]: :_default' \
'--date-format=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--range=[]: :_default' \
'--cell=[]: :_default' \
'--header-row=[]: :_default' \
'-q[]' \
'--quiet[]' \
'--flexible[]' \
'--keep-zero-time[]' \
'--trim[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(exclude)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-i[]' \
'--ignore-case[]' \
'-n[]' \
'--no-headers[]' \
'-v[]' \
'--invert[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(explode)
_arguments "${_arguments_options[@]}" : \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
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
'-n[]' \
'--no-headers[]' \
'-q[]' \
'--quiet[]' \
'--no-output[]' \
'-H[]' \
'--human-readable[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(extsort)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--memory-limit=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--tmp-dir=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-R[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fetch)
_arguments "${_arguments_options[@]}" : \
'--mem-cache-size=[]: :_default' \
'--max-errors=[]: :_default' \
'--jaqfile=[]: :_default' \
'--rate-limit=[]: :_default' \
'--user-agent=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-H+[]: :_default' \
'--http-header=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'--timeout=[]: :_default' \
'--url-template=[]: :_default' \
'--jaq=[]: :_default' \
'--report=[]: :_default' \
'--max-retries=[]: :_default' \
'--cookies[]' \
'--pretty[]' \
'--cache-error[]' \
'-p[]' \
'--progressbar[]' \
'--store-error[]' \
'--redis-cache[]' \
'-n[]' \
'--no-headers[]' \
'--no-cache[]' \
'--flush-cache[]' \
'--disk-cache[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fetchpost)
_arguments "${_arguments_options[@]}" : \
'--timeout=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--max-retries=[]: :_default' \
'--report=[]: :_default' \
'--user-agent=[]: :_default' \
'-j+[]: :_default' \
'--globals-json=[]: :_default' \
'--rate-limit=[]: :_default' \
'--jaqfile=[]: :_default' \
'--max-errors=[]: :_default' \
'--jaq=[]: :_default' \
'-H+[]: :_default' \
'--http-header=[]: :_default' \
'--mem-cache-size=[]: :_default' \
'-t+[]: :_default' \
'--payload-tpl=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--content-type=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'--cookies[]' \
'--flush-cache[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'--disk-cache[]' \
'--no-cache[]' \
'--cache-error[]' \
'--store-error[]' \
'--pretty[]' \
'--compress[]' \
'--redis-cache[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fill)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-v+[]: :_default' \
'--default=[]: :_default' \
'-g+[]: :_default' \
'--groupby=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-b[]' \
'--backfill[]' \
'-f[]' \
'--first[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fixlengths)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--length=[]: :_default' \
'-i+[]: :_default' \
'--insert=[]: :_default' \
'--quote=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--escape=[]: :_default' \
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
'-c+[]: :_default' \
'--condense=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-s+[]: :_default' \
'--separator=[]: :_default' \
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
'--quote=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--escape=[]: :_default' \
'--ascii[]' \
'--quote-never[]' \
'--quote-always[]' \
'--no-final-newline[]' \
'--crlf[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(foreach)
_arguments "${_arguments_options[@]}" : \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--dry-run=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-u[]' \
'--unify[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(frequency)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--limit=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--high-card-threshold=[]: :_default' \
'--high-card-pct=[]: :_default' \
'--all-unique-text=[]: :_default' \
'--null-text=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-u+[]: :_default' \
'--unq-limit=[]: :_default' \
'-r+[]: :_default' \
'--rank-strategy=[]: :_default' \
'--weight=[]: :_default' \
'--other-text=[]: :_default' \
'--no-float=[]: :_default' \
'--stats-filter=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--pct-dec-places=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--lmt-threshold=[]: :_default' \
'--vis-whitespace[]' \
'--other-sorted[]' \
'--frequency-jsonl[]' \
'--null-sorted[]' \
'--force[]' \
'--toon[]' \
'--no-other[]' \
'--memcheck[]' \
'-n[]' \
'--no-headers[]' \
'--json[]' \
'--pct-nulls[]' \
'--no-trim[]' \
'-a[]' \
'--asc[]' \
'--pretty-json[]' \
'-i[]' \
'--ignore-case[]' \
'--no-stats[]' \
'--no-nulls[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(geocode)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(countryinfonow)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-check)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-load)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-reset)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-update)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(iplookup)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(iplookupnow)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reversenow)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(suggest)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(suggestnow)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--min-score=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--admin1=[]: :_default' \
'--invalid-result=[]: :_default' \
'--cache-dir=[]: :_default' \
'--languages=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'--timeout=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--force[]' \
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
'-g+[]: :_default' \
'--geometry=[]: :_default' \
'-l+[]: :_default' \
'--max-length=[]: :_default' \
'-y+[]: :_default' \
'--latitude=[]: :_default' \
'-x+[]: :_default' \
'--longitude=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(headers)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--intersect[]' \
'-j[]' \
'--just-names[]' \
'--trim[]' \
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
'--quote-style=[]: :_default' \
'--escape=[]: :_default' \
'--skip-lastlines=[]: :_default' \
'--comment=[]: :_default' \
'--quote=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--skip-lines=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--encoding-errors=[]: :_default' \
'--no-quoting[]' \
'--auto-skip[]' \
'--trim-fields[]' \
'--trim-headers[]' \
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
'--right[]' \
'--left-semi[]' \
'--right-anti[]' \
'-z[]' \
'--ignore-leading-zeros[]' \
'-n[]' \
'--no-headers[]' \
'--left[]' \
'--left-anti[]' \
'--nulls[]' \
'--right-semi[]' \
'--cross[]' \
'-i[]' \
'--ignore-case[]' \
'--full[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(joinp)
_arguments "${_arguments_options[@]}" : \
'--right_by=[]: :_default' \
'--strategy=[]: :_default' \
'--filter-left=[]: :_default' \
'--validate=[]: :_default' \
'--sql-filter=[]: :_default' \
'--time-format=[]: :_default' \
'--non-equi=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--datetime-format=[]: :_default' \
'--date-format=[]: :_default' \
'--infer-len=[]: :_default' \
'--float-precision=[]: :_default' \
'--left_by=[]: :_default' \
'-N+[]: :_default' \
'--norm-unicode=[]: :_default' \
'--maintain-order=[]: :_default' \
'--cache-schema=[]: :_default' \
'--null-value=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--tolerance=[]: :_default' \
'--filter-right=[]: :_default' \
'-q[]' \
'--quiet[]' \
'--ignore-errors[]' \
'-X[]' \
'--allow-exact-matches[]' \
'--full[]' \
'--decimal-comma[]' \
'--left-semi[]' \
'--nulls[]' \
'--low-memory[]' \
'--no-sort[]' \
'--streaming[]' \
'--no-optimizations[]' \
'-i[]' \
'--ignore-case[]' \
'--coalesce[]' \
'--right[]' \
'--try-parsedates[]' \
'--cross[]' \
'--right-anti[]' \
'--left-anti[]' \
'--left[]' \
'--asof[]' \
'-z[]' \
'--ignore-leading-zeros[]' \
'--right-semi[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(json)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--jaq=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(jsonl)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
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
'-f+[]: :_default' \
'--freeze-columns=[]: :_default' \
'--find=[]: :_default' \
'-W+[]: :_default' \
'--wrap-mode=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-t[]' \
'--tab-separated[]' \
'-i[]' \
'--ignore-case[]' \
'--debug[]' \
'-A[]' \
'--auto-reload[]' \
'--no-headers[]' \
'-m[]' \
'--monochrome[]' \
'-S[]' \
'--streaming-stdin[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(log)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(luau)
_arguments "${_arguments_options[@]}" : \
'--ckan-api=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--max-errors=[]: :_default' \
'--timeout=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-token=[]: :_default' \
'-B+[]: :_default' \
'--begin=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-r[]' \
'--remap[]' \
'-g[]' \
'--no-globals[]' \
'-n[]' \
'--no-headers[]' \
'--colindex[]' \
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
'-o+[]: :_default' \
'--output=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--max-errors=[]: :_default' \
'--timeout=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-token=[]: :_default' \
'-B+[]: :_default' \
'--begin=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-r[]' \
'--remap[]' \
'-g[]' \
'--no-globals[]' \
'-n[]' \
'--no-headers[]' \
'--colindex[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(map)
_arguments "${_arguments_options[@]}" : \
'--ckan-api=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--max-errors=[]: :_default' \
'--timeout=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-token=[]: :_default' \
'-B+[]: :_default' \
'--begin=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-r[]' \
'--remap[]' \
'-g[]' \
'--no-globals[]' \
'-n[]' \
'--no-headers[]' \
'--colindex[]' \
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
'-K+[]: :_default' \
'--join-keys=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-J+[]: :_default' \
'--join-inputs=[]: :_default' \
'--pct-thresholds=[]: :_default' \
'--stats-options=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--xsd-gdate-scan=[]: :_default' \
'--round=[]: :_default' \
'-S+[]: :_default' \
'--bivariate-stats=[]: :_default' \
'-e+[]: :_default' \
'--epsilon=[]: :_default' \
'-C+[]: :_default' \
'--cardinality-threshold=[]: :_default' \
'-T+[]: :_default' \
'--join-type=[]: :_default' \
'-B[]' \
'--bivariate[]' \
'--advanced[]' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'--use-percentiles[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(partition)
_arguments "${_arguments_options[@]}" : \
'--filename=[]: :_default' \
'--limit=[]: :_default' \
'-p+[]: :_default' \
'--prefix-length=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--drop[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pivotp)
_arguments "${_arguments_options[@]}" : \
'--infer-len=[]: :_default' \
'-a+[]: :_default' \
'--agg=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-i+[]: :_default' \
'--index=[]: :_default' \
'-v+[]: :_default' \
'--values=[]: :_default' \
'--col-separator=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-q[]' \
'--quiet[]' \
'--sort-columns[]' \
'--try-parsedates[]' \
'--maintain-order[]' \
'--decimal-comma[]' \
'--validate[]' \
'--ignore-errors[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pragmastat)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-m+[]: :_default' \
'--misrate=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--memcheck[]' \
'-t[]' \
'--twosample[]' \
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
'--base-delay-ms=[]: :_default' \
'--save-fname=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-m+[]: :_default' \
'--msg=[]: :_default' \
'-d+[]: :_default' \
'--workdir=[]: :_default' \
'-q[]' \
'--quiet[]' \
'-f[]' \
'--fd-output[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pseudo)
_arguments "${_arguments_options[@]}" : \
'--formatstr=[]: :_default' \
'--start=[]: :_default' \
'--increment=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--helper=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--helper=[]: :_default' \
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
(map)
_arguments "${_arguments_options[@]}" : \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--helper=[]: :_default' \
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
'-o+[]: :_default' \
'--output=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--size-limit=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-q[]' \
'--quiet[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-i[]' \
'--ignore-case[]' \
'--literal[]' \
'--exact[]' \
'-u[]' \
'--unicode[]' \
'--not-one[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--memcheck[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(safenames)
_arguments "${_arguments_options[@]}" : \
'--reserved=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--prefix=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--mode=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sample)
_arguments "${_arguments_options[@]}" : \
'--rng=[]: :_default' \
'--timeout=[]: :_default' \
'--timeseries=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--ts-start=[]: :_default' \
'--ts-adaptive=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--weighted=[]: :_default' \
'--cluster=[]: :_default' \
'--ts-interval=[]: :_default' \
'--user-agent=[]: :_default' \
'--max-size=[]: :_default' \
'--ts-input-tz=[]: :_default' \
'--ts-aggregate=[]: :_default' \
'--systematic=[]: :_default' \
'--stratified=[]: :_default' \
'--seed=[]: :_default' \
'--force[]' \
'-n[]' \
'--no-headers[]' \
'--ts-prefer-dmy[]' \
'--bernoulli[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(schema)
_arguments "${_arguments_options[@]}" : \
'--dates-whitelist=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--enum-threshold=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--pattern-columns=[]: :_default' \
'--stdout[]' \
'--strict-dates[]' \
'--force[]' \
'--strict-formats[]' \
'--memcheck[]' \
'-i[]' \
'--ignore-case[]' \
'--polars[]' \
'-n[]' \
'--no-headers[]' \
'--prefer-dmy[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(search)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-f+[]: :_default' \
'--flag=[]: :_default' \
'--preview-match=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'--size-limit=[]: :_default' \
'--exact[]' \
'-c[]' \
'--count[]' \
'-q[]' \
'--quiet[]' \
'--literal[]' \
'-p[]' \
'--progressbar[]' \
'-v[]' \
'--invert-match[]' \
'-i[]' \
'--ignore-case[]' \
'-n[]' \
'--no-headers[]' \
'--not-one[]' \
'-Q[]' \
'--quick[]' \
'-u[]' \
'--unicode[]' \
'--json[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(searchset)
_arguments "${_arguments_options[@]}" : \
'--jobs=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'--size-limit=[]: :_default' \
'--unmatched-output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-f+[]: :_default' \
'--flag=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-u[]' \
'--unicode[]' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
'-i[]' \
'--ignore-case[]' \
'-v[]' \
'--invert-match[]' \
'-j[]' \
'--json[]' \
'--not-one[]' \
'-c[]' \
'--count[]' \
'--flag-matches-only[]' \
'-Q[]' \
'--quick[]' \
'-n[]' \
'--no-headers[]' \
'--exact[]' \
'--literal[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(select)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--seed=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-R[]' \
'--random[]' \
'-S[]' \
'--sort[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(slice)
_arguments "${_arguments_options[@]}" : \
'-l+[]: :_default' \
'--len=[]: :_default' \
'-e+[]: :_default' \
'--end=[]: :_default' \
'-s+[]: :_default' \
'--start=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-i+[]: :_default' \
'--index=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--json[]' \
'--invert[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(snappy)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--user-agent=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
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
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--user-agent=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(compress)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--user-agent=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(decompress)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--user-agent=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--user-agent=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
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
'--user-agent=[]: :_default' \
'--quote=[]: :_default' \
'--timeout=[]: :_default' \
'--sample=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--save-urlsample=[]: :_default' \
'--prefer-dmy[]' \
'-Q[]' \
'--quick[]' \
'--stats-types[]' \
'--json[]' \
'--no-infer[]' \
'--just-mime[]' \
'--harvest-mode[]' \
'-p[]' \
'--progressbar[]' \
'--pretty-json[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sort)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--rng=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--seed=[]: :_default' \
'-i[]' \
'--ignore-case[]' \
'-n[]' \
'--no-headers[]' \
'--faster[]' \
'--memcheck[]' \
'--natural[]' \
'-N[]' \
'--numeric[]' \
'-R[]' \
'--reverse[]' \
'--random[]' \
'-u[]' \
'--unique[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sortcheck)
_arguments "${_arguments_options[@]}" : \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--pretty-json[]' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'--all[]' \
'-i[]' \
'--ignore-case[]' \
'--json[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(split)
_arguments "${_arguments_options[@]}" : \
'--filter=[]: :_default' \
'--pad=[]: :_default' \
'-k+[]: :_default' \
'--kb-size=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-c+[]: :_default' \
'--chunks=[]: :_default' \
'--filename=[]: :_default' \
'-s+[]: :_default' \
'--size=[]: :_default' \
'--filter-ignore-errors[]' \
'-q[]' \
'--quiet[]' \
'-n[]' \
'--no-headers[]' \
'--filter-cleanup[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sqlp)
_arguments "${_arguments_options[@]}" : \
'--wnull-value=[]: :_default' \
'--datetime-format=[]: :_default' \
'--rnull-values=[]: :_default' \
'--date-format=[]: :_default' \
'--float-precision=[]: :_default' \
'--compression=[]: :_default' \
'--format=[]: :_default' \
'--infer-len=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--time-format=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--compress-level=[]: :_default' \
'--truncate-ragged-lines[]' \
'--cache-schema[]' \
'--ignore-errors[]' \
'--statistics[]' \
'--decimal-comma[]' \
'--streaming[]' \
'--try-parsedates[]' \
'--low-memory[]' \
'--no-optimizations[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stats)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--boolean-patterns=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--dates-whitelist=[]: :_default' \
'-c+[]: :_default' \
'--cache-threshold=[]: :_default' \
'--round=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--percentile-list=[]: :_default' \
'--weight=[]: :_default' \
'--infer-boolean[]' \
'--cardinality[]' \
'--vis-whitespace[]' \
'--median[]' \
'-E[]' \
'--everything[]' \
'--infer-dates[]' \
'--prefer-dmy[]' \
'--memcheck[]' \
'--mode[]' \
'--mad[]' \
'--percentiles[]' \
'--force[]' \
'-n[]' \
'--no-headers[]' \
'--typesonly[]' \
'--quartiles[]' \
'--nulls[]' \
'--stats-jsonl[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(table)
_arguments "${_arguments_options[@]}" : \
'-p+[]: :_default' \
'--pad=[]: :_default' \
'-c+[]: :_default' \
'--condense=[]: :_default' \
'-w+[]: :_default' \
'--width=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-a+[]: :_default' \
'--align=[]: :_default' \
'--memcheck[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(template)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--delimiter=[]: :_default' \
'--outsubdir-size=[]: :_default' \
'--template=[]: :_default' \
'--ckan-token=[]: :_default' \
'-j+[]: :_default' \
'--globals-json=[]: :_default' \
'--outfilename=[]: :_default' \
'--jobs=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--timeout=[]: :_default' \
'-t+[]: :_default' \
'--template-file=[]: :_default' \
'--customfilter-error=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(to)
_arguments "${_arguments_options[@]}" : \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'--delimiter=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d[]' \
'--drop[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-q[]' \
'--quiet[]' \
'-A[]' \
'--all-strings[]' \
'-k[]' \
'--print-package[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
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
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'--delimiter=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d[]' \
'--drop[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-q[]' \
'--quiet[]' \
'-A[]' \
'--all-strings[]' \
'-k[]' \
'--print-package[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(ods)
_arguments "${_arguments_options[@]}" : \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'--delimiter=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d[]' \
'--drop[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-q[]' \
'--quiet[]' \
'-A[]' \
'--all-strings[]' \
'-k[]' \
'--print-package[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(postgres)
_arguments "${_arguments_options[@]}" : \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'--delimiter=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d[]' \
'--drop[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-q[]' \
'--quiet[]' \
'-A[]' \
'--all-strings[]' \
'-k[]' \
'--print-package[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sqlite)
_arguments "${_arguments_options[@]}" : \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'--delimiter=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d[]' \
'--drop[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-q[]' \
'--quiet[]' \
'-A[]' \
'--all-strings[]' \
'-k[]' \
'--print-package[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(xlsx)
_arguments "${_arguments_options[@]}" : \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'--delimiter=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d[]' \
'--drop[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-q[]' \
'--quiet[]' \
'-A[]' \
'--all-strings[]' \
'-k[]' \
'--print-package[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
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
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
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
'--long=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--memcheck[]' \
'-m[]' \
'--multipass[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
'--email-min-subdomains=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'--invalid=[]: :_default' \
'--ckan-api=[]: :_default' \
'--cache-dir=[]: :_default' \
'--valid=[]: :_default' \
'--backtrack-limit=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--valid-output=[]: :_default' \
'--ckan-token=[]: :_default' \
'--size-limit=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'--email-required-tld[]' \
'--json[]' \
'--email-domain-literal[]' \
'-q[]' \
'--quiet[]' \
'--no-format-validation[]' \
'--pretty-json[]' \
'--trim[]' \
'--fancy-regex[]' \
'--email-display-text[]' \
'--fail-fast[]' \
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
'--email-min-subdomains=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'--invalid=[]: :_default' \
'--ckan-api=[]: :_default' \
'--cache-dir=[]: :_default' \
'--valid=[]: :_default' \
'--backtrack-limit=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--valid-output=[]: :_default' \
'--ckan-token=[]: :_default' \
'--size-limit=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--timeout=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'--email-required-tld[]' \
'--json[]' \
'--email-domain-literal[]' \
'-q[]' \
'--quiet[]' \
'--no-format-validation[]' \
'--pretty-json[]' \
'--trim[]' \
'--fancy-regex[]' \
'--email-display-text[]' \
'--fail-fast[]' \
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
(log)
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
(pragmastat)
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
'log:' \
'luau:' \
'moarstats:' \
'partition:' \
'pivotp:' \
'pragmastat:' \
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
'log:' \
'luau:' \
'moarstats:' \
'partition:' \
'pivotp:' \
'pragmastat:' \
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
(( $+functions[_qsv__help__log_commands] )) ||
_qsv__help__log_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help log commands' commands "$@"
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
(( $+functions[_qsv__help__pragmastat_commands] )) ||
_qsv__help__pragmastat_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help pragmastat commands' commands "$@"
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
(( $+functions[_qsv__log_commands] )) ||
_qsv__log_commands() {
    local commands; commands=()
    _describe -t commands 'qsv log commands' commands "$@"
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
(( $+functions[_qsv__pragmastat_commands] )) ||
_qsv__pragmastat_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pragmastat commands' commands "$@"
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
