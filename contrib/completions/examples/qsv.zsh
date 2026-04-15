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
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__subcmd__apply_commands" \
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
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
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
(dynfmt)
_arguments "${_arguments_options[@]}" : \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
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
(emptyreplace)
_arguments "${_arguments_options[@]}" : \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
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
(operations)
_arguments "${_arguments_options[@]}" : \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
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
":: :_qsv__subcmd__apply__subcmd__help_commands" \
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
(blake3)
_arguments "${_arguments_options[@]}" : \
'--derive-key=[]: :_default' \
'-l+[]: :_default' \
'--length=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--no-names[]' \
'--keyed[]' \
'-q[]' \
'--quiet[]' \
'-c[]' \
'--check[]' \
'--tag[]' \
'--raw[]' \
'--no-mmap[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cat)
_arguments "${_arguments_options[@]}" : \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--flexible[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--pad[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__subcmd__cat_commands" \
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
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
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
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
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
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
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
":: :_qsv__subcmd__cat__subcmd__help_commands" \
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
'-f[]' \
'--flexible[]' \
'--width[]' \
'--json[]' \
'--width-no-delims[]' \
'--low-memory[]' \
'--no-polars[]' \
'-n[]' \
'--no-headers[]' \
'-H[]' \
'--human-readable[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(datefmt)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--output-tz=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--input-tz=[]: :_default' \
'--formatstr=[]: :_default' \
'--default-tz=[]: :_default' \
'-R+[]: :_default' \
'--ts-resolution=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--keep-zero-time[]' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'--prefer-dmy[]' \
'--zulu[]' \
'--utc[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(dedup)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-D+[]: :_default' \
'--dupes-output=[]: :_default' \
'-i[]' \
'--ignore-case[]' \
'-N[]' \
'--numeric[]' \
'--sorted[]' \
'-n[]' \
'--no-headers[]' \
'--memcheck[]' \
'-q[]' \
'--quiet[]' \
'-H[]' \
'--human-readable[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(describegpt)
_arguments "${_arguments_options[@]}" : \
'--sample-size=[]: :_default' \
'--user-agent=[]: :_default' \
'--freq-options=[]: :_default' \
'-u+[]: :_default' \
'--base-url=[]: :_default' \
'--session-len=[]: :_default' \
'--cache-dir=[]: :_default' \
'--score-threshold=[]: :_default' \
'-m+[]: :_default' \
'--model=[]: :_default' \
'--addl-props=[]: :_default' \
'--tag-vocab=[]: :_default' \
'--format=[]: :_default' \
'--truncate-str=[]: :_default' \
'--prompt-file=[]: :_default' \
'--enum-threshold=[]: :_default' \
'--ckan-token=[]: :_default' \
'--language=[]: :_default' \
'-k+[]: :_default' \
'--api-key=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--num-examples=[]: :_default' \
'--num-tags=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'-p+[]: :_default' \
'--prompt=[]: :_default' \
'-t+[]: :_default' \
'--max-tokens=[]: :_default' \
'--export-prompt=[]: :_default' \
'--sql-results=[]: :_default' \
'--timeout=[]: :_default' \
'--addl-cols-list=[]: :_default' \
'--score-max-retries=[]: :_default' \
'--ckan-api=[]: :_default' \
'--stats-options=[]: :_default' \
'--session=[]: :_default' \
'--no-score-sql[]' \
'--dictionary[]' \
'--no-cache[]' \
'-A[]' \
'--all[]' \
'--process-response[]' \
'--flush-cache[]' \
'--fewshot-examples[]' \
'--tags[]' \
'--forget[]' \
'--description[]' \
'--prepare-context[]' \
'-q[]' \
'--quiet[]' \
'--redis-cache[]' \
'--fresh[]' \
'--addl-cols[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(diff)
_arguments "${_arguments_options[@]}" : \
'--delimiter-right=[]: :_default' \
'--delimiter-left=[]: :_default' \
'-k+[]: :_default' \
'--key=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--delimiter-output=[]: :_default' \
'--sort-columns=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--no-headers-left[]' \
'--drop-equal-fields[]' \
'--no-headers-output[]' \
'--no-headers-right[]' \
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
'--increment=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--start=[]: :_default' \
'--constant=[]: :_default' \
'--copy=[]: :_default' \
'--hash=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
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
'--metadata=[]: :_default' \
'--range=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--table=[]: :_default' \
'--header-row=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--error-format=[]: :_default' \
'--cell=[]: :_default' \
'--date-format=[]: :_default' \
'-s+[]: :_default' \
'--sheet=[]: :_default' \
'-q[]' \
'--quiet[]' \
'--flexible[]' \
'--trim[]' \
'--keep-zero-time[]' \
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
'-v[]' \
'--invert[]' \
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
'-D+[]: :_default' \
'--dupes-output=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--memory-limit=[]: :_default' \
'--temp-dir=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-H[]' \
'--human-readable[]' \
'-q[]' \
'--quiet[]' \
'-n[]' \
'--no-headers[]' \
'--no-output[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(extsort)
_arguments "${_arguments_options[@]}" : \
'--memory-limit=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--tmp-dir=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
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
'--jaq=[]: :_default' \
'--mem-cache-size=[]: :_default' \
'--jaqfile=[]: :_default' \
'-H+[]: :_default' \
'--http-header=[]: :_default' \
'--url-template=[]: :_default' \
'--max-retries=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--report=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'--rate-limit=[]: :_default' \
'--max-errors=[]: :_default' \
'--user-agent=[]: :_default' \
'--timeout=[]: :_default' \
'--store-error[]' \
'-p[]' \
'--progressbar[]' \
'--pretty[]' \
'--disk-cache[]' \
'--flush-cache[]' \
'--cache-error[]' \
'--no-cache[]' \
'-n[]' \
'--no-headers[]' \
'--redis-cache[]' \
'--cookies[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fetchpost)
_arguments "${_arguments_options[@]}" : \
'-H+[]: :_default' \
'--http-header=[]: :_default' \
'-j+[]: :_default' \
'--globals-json=[]: :_default' \
'--user-agent=[]: :_default' \
'--max-errors=[]: :_default' \
'-t+[]: :_default' \
'--payload-tpl=[]: :_default' \
'--content-type=[]: :_default' \
'--report=[]: :_default' \
'--mem-cache-size=[]: :_default' \
'--jaq=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'--max-retries=[]: :_default' \
'--timeout=[]: :_default' \
'--rate-limit=[]: :_default' \
'--jaqfile=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'--no-cache[]' \
'--redis-cache[]' \
'--cache-error[]' \
'-n[]' \
'--no-headers[]' \
'--pretty[]' \
'--cookies[]' \
'--compress[]' \
'--store-error[]' \
'--disk-cache[]' \
'--flush-cache[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fill)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--groupby=[]: :_default' \
'-v+[]: :_default' \
'--default=[]: :_default' \
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
'-i+[]: :_default' \
'--insert=[]: :_default' \
'--escape=[]: :_default' \
'--quote=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--length=[]: :_default' \
'-q[]' \
'--quiet[]' \
'-r[]' \
'--remove-empty[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(flatten)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-s+[]: :_default' \
'--separator=[]: :_default' \
'-c+[]: :_default' \
'--condense=[]: :_default' \
'-f+[]: :_default' \
'--field-separator=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fmt)
_arguments "${_arguments_options[@]}" : \
'--quote=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--escape=[]: :_default' \
'-t+[]: :_default' \
'--out-delimiter=[]: :_default' \
'--ascii[]' \
'--crlf[]' \
'--quote-always[]' \
'--quote-never[]' \
'--no-final-newline[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(foreach)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--dry-run=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-u[]' \
'--unify[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(frequency)
_arguments "${_arguments_options[@]}" : \
'--high-card-threshold=[]: :_default' \
'--no-float=[]: :_default' \
'-u+[]: :_default' \
'--unq-limit=[]: :_default' \
'-l+[]: :_default' \
'--limit=[]: :_default' \
'--null-text=[]: :_default' \
'--pct-dec-places=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--weight=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--high-card-pct=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--stats-filter=[]: :_default' \
'--lmt-threshold=[]: :_default' \
'-r+[]: :_default' \
'--rank-strategy=[]: :_default' \
'--other-text=[]: :_default' \
'--all-unique-text=[]: :_default' \
'-a[]' \
'--asc[]' \
'--no-stats[]' \
'--pct-nulls[]' \
'--json[]' \
'-i[]' \
'--ignore-case[]' \
'--vis-whitespace[]' \
'--frequency-jsonl[]' \
'--force[]' \
'--no-nulls[]' \
'--pretty-json[]' \
'-n[]' \
'--no-headers[]' \
'--memcheck[]' \
'--no-other[]' \
'--toon[]' \
'--no-trim[]' \
'--other-sorted[]' \
'--null-sorted[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(geocode)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__subcmd__geocode_commands" \
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
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(countryinfonow)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-check)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-load)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-reset)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-update)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(iplookup)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(iplookupnow)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reversenow)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(suggest)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(suggestnow)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--languages=[]: :_default' \
'--invalid-result=[]: :_default' \
'--min-score=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--timeout=[]: :_default' \
'--admin1=[]: :_default' \
'--cache-dir=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'--force[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__subcmd__geocode__subcmd__help_commands" \
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
'-o+[]: :_default' \
'--output=[]: :_default' \
'-g+[]: :_default' \
'--geometry=[]: :_default' \
'-y+[]: :_default' \
'--latitude=[]: :_default' \
'-l+[]: :_default' \
'--max-length=[]: :_default' \
'-x+[]: :_default' \
'--longitude=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(headers)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-J[]' \
'--just-count[]' \
'--trim[]' \
'-j[]' \
'--just-names[]' \
'--intersect[]' \
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
'--escape=[]: :_default' \
'--skip-lastlines=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--encoding-errors=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--comment=[]: :_default' \
'--quote=[]: :_default' \
'--skip-lines=[]: :_default' \
'--quote-style=[]: :_default' \
'--auto-skip[]' \
'--trim-headers[]' \
'--no-quoting[]' \
'--trim-fields[]' \
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
'-z[]' \
'--ignore-leading-zeros[]' \
'--right-anti[]' \
'--cross[]' \
'--left-anti[]' \
'--left-semi[]' \
'--right[]' \
'--full[]' \
'-i[]' \
'--ignore-case[]' \
'-n[]' \
'--no-headers[]' \
'--nulls[]' \
'--right-semi[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(joinp)
_arguments "${_arguments_options[@]}" : \
'--non-equi=[]: :_default' \
'--date-format=[]: :_default' \
'--infer-len=[]: :_default' \
'--left_by=[]: :_default' \
'--right_by=[]: :_default' \
'--cache-schema=[]: :_default' \
'--time-format=[]: :_default' \
'--strategy=[]: :_default' \
'--filter-right=[]: :_default' \
'--sql-filter=[]: :_default' \
'--maintain-order=[]: :_default' \
'--float-precision=[]: :_default' \
'-N+[]: :_default' \
'--norm-unicode=[]: :_default' \
'--tolerance=[]: :_default' \
'--datetime-format=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--validate=[]: :_default' \
'--null-value=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--filter-left=[]: :_default' \
'--coalesce[]' \
'-z[]' \
'--ignore-leading-zeros[]' \
'--asof[]' \
'--left-anti[]' \
'--ignore-errors[]' \
'--left[]' \
'--decimal-comma[]' \
'--full[]' \
'--right[]' \
'-q[]' \
'--quiet[]' \
'--left-semi[]' \
'--cross[]' \
'--try-parsedates[]' \
'--no-sort[]' \
'-X[]' \
'--allow-exact-matches[]' \
'--right-semi[]' \
'-i[]' \
'--ignore-case[]' \
'--nulls[]' \
'--low-memory[]' \
'--no-optimizations[]' \
'--streaming[]' \
'--right-anti[]' \
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
'--echo-column=[]: :_default' \
'--columns=[]: :_default' \
'-W+[]: :_default' \
'--wrap-mode=[]: :_default' \
'-P+[]: :_default' \
'--prompt=[]: :_default' \
'--find=[]: :_default' \
'--filter=[]: :_default' \
'-f+[]: :_default' \
'--freeze-columns=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-A[]' \
'--auto-reload[]' \
'--no-headers[]' \
'-i[]' \
'--ignore-case[]' \
'-S[]' \
'--streaming-stdin[]' \
'--debug[]' \
'-m[]' \
'--monochrome[]' \
'-t[]' \
'--tab-separated[]' \
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
'-B+[]: :_default' \
'--begin=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--timeout=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--max-errors=[]: :_default' \
'--ckan-token=[]: :_default' \
'--cache-dir=[]: :_default' \
'--colindex[]' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-r[]' \
'--remap[]' \
'-g[]' \
'--no-globals[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__subcmd__luau_commands" \
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
'-B+[]: :_default' \
'--begin=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--timeout=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--max-errors=[]: :_default' \
'--ckan-token=[]: :_default' \
'--cache-dir=[]: :_default' \
'--colindex[]' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-r[]' \
'--remap[]' \
'-g[]' \
'--no-globals[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(map)
_arguments "${_arguments_options[@]}" : \
'--ckan-api=[]: :_default' \
'-B+[]: :_default' \
'--begin=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--timeout=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--max-errors=[]: :_default' \
'--ckan-token=[]: :_default' \
'--cache-dir=[]: :_default' \
'--colindex[]' \
'-p[]' \
'--progressbar[]' \
'-n[]' \
'--no-headers[]' \
'-r[]' \
'--remap[]' \
'-g[]' \
'--no-globals[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__subcmd__luau__subcmd__help_commands" \
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
'-S+[]: :_default' \
'--bivariate-stats=[]: :_default' \
'-T+[]: :_default' \
'--join-type=[]: :_default' \
'-C+[]: :_default' \
'--cardinality-threshold=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-e+[]: :_default' \
'--epsilon=[]: :_default' \
'--pct-thresholds=[]: :_default' \
'--xsd-gdate-scan=[]: :_default' \
'-J+[]: :_default' \
'--join-inputs=[]: :_default' \
'-K+[]: :_default' \
'--join-keys=[]: :_default' \
'--round=[]: :_default' \
'--force[]' \
'--advanced[]' \
'--use-percentiles[]' \
'-p[]' \
'--progressbar[]' \
'-B[]' \
'--bivariate[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(partition)
_arguments "${_arguments_options[@]}" : \
'-p+[]: :_default' \
'--prefix-length=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--filename=[]: :_default' \
'--limit=[]: :_default' \
'--drop[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pivotp)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-a+[]: :_default' \
'--agg=[]: :_default' \
'--infer-len=[]: :_default' \
'--total-label=[]: :_default' \
'-v+[]: :_default' \
'--values=[]: :_default' \
'--col-separator=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-i+[]: :_default' \
'--index=[]: :_default' \
'--sort-columns[]' \
'--try-parsedates[]' \
'--validate[]' \
'--subtotal[]' \
'--maintain-order[]' \
'--decimal-comma[]' \
'--grand-total[]' \
'-q[]' \
'--quiet[]' \
'--ignore-errors[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pragmastat)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--compare1=[]: :_default' \
'--stats-options=[]: :_default' \
'--seed=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--subsample=[]: :_default' \
'--round=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-m+[]: :_default' \
'--misrate=[]: :_default' \
'--compare2=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--memcheck[]' \
'--force[]' \
'--no-bounds[]' \
'--standalone[]' \
'-t[]' \
'--twosample[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pro)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__subcmd__pro_commands" \
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
":: :_qsv__subcmd__pro__subcmd__help_commands" \
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
'-o+[]: :_default' \
'--output=[]: :_default' \
'-F+[]: :_default' \
'--filters=[]: :_default' \
'-m+[]: :_default' \
'--msg=[]: :_default' \
'--base-delay-ms=[]: :_default' \
'--save-fname=[]: :_default' \
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
'--increment=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--start=[]: :_default' \
'--formatstr=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(py)
_arguments "${_arguments_options[@]}" : \
'-f+[]: :_default' \
'--helper=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__subcmd__py_commands" \
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
'-f+[]: :_default' \
'--helper=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
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
'-f+[]: :_default' \
'--helper=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
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
":: :_qsv__subcmd__py__subcmd__help_commands" \
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
'--dfa-size-limit=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--size-limit=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-u[]' \
'--unicode[]' \
'--literal[]' \
'--not-one[]' \
'-q[]' \
'--quiet[]' \
'--exact[]' \
'-i[]' \
'--ignore-case[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
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
'--prefix=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--mode=[]: :_default' \
'--reserved=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sample)
_arguments "${_arguments_options[@]}" : \
'--ts-interval=[]: :_default' \
'--ts-start=[]: :_default' \
'--systematic=[]: :_default' \
'--ts-adaptive=[]: :_default' \
'--seed=[]: :_default' \
'--stratified=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
'--rng=[]: :_default' \
'--ts-input-tz=[]: :_default' \
'--user-agent=[]: :_default' \
'--max-size=[]: :_default' \
'--cluster=[]: :_default' \
'--ts-aggregate=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--timeseries=[]: :_default' \
'--weighted=[]: :_default' \
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
'--pattern-columns=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--enum-threshold=[]: :_default' \
'--dates-whitelist=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--memcheck[]' \
'--strict-dates[]' \
'-n[]' \
'--no-headers[]' \
'--stdout[]' \
'--force[]' \
'--strict-formats[]' \
'-i[]' \
'--ignore-case[]' \
'--prefer-dmy[]' \
'--polars[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(scoresql)
_arguments "${_arguments_options[@]}" : \
'--infer-len=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-q[]' \
'--quiet[]' \
'--truncate-ragged-lines[]' \
'--json[]' \
'--try-parsedates[]' \
'--duckdb[]' \
'--ignore-errors[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(search)
_arguments "${_arguments_options[@]}" : \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--size-limit=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--preview-match=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-f+[]: :_default' \
'--flag=[]: :_default' \
'--exact[]' \
'--json[]' \
'--not-one[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-c[]' \
'--count[]' \
'-q[]' \
'--quiet[]' \
'-i[]' \
'--ignore-case[]' \
'--literal[]' \
'-v[]' \
'--invert-match[]' \
'-u[]' \
'--unicode[]' \
'-Q[]' \
'--quick[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(searchset)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--jobs=[]: :_default' \
'--unmatched-output=[]: :_default' \
'--size-limit=[]: :_default' \
'-f+[]: :_default' \
'--flag=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-c[]' \
'--count[]' \
'-q[]' \
'--quiet[]' \
'-i[]' \
'--ignore-case[]' \
'-j[]' \
'--json[]' \
'-p[]' \
'--progressbar[]' \
'--flag-matches-only[]' \
'--exact[]' \
'-n[]' \
'--no-headers[]' \
'-Q[]' \
'--quick[]' \
'--literal[]' \
'--not-one[]' \
'-v[]' \
'--invert-match[]' \
'-u[]' \
'--unicode[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(select)
_arguments "${_arguments_options[@]}" : \
'--seed=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
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
'-s+[]: :_default' \
'--start=[]: :_default' \
'-i+[]: :_default' \
'--index=[]: :_default' \
'-l+[]: :_default' \
'--len=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-e+[]: :_default' \
'--end=[]: :_default' \
'--invert[]' \
'-n[]' \
'--no-headers[]' \
'--json[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(snappy)
_arguments "${_arguments_options[@]}" : \
'--timeout=[]: :_default' \
'--user-agent=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__subcmd__snappy_commands" \
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
'--timeout=[]: :_default' \
'--user-agent=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
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
'--timeout=[]: :_default' \
'--user-agent=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
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
'--timeout=[]: :_default' \
'--user-agent=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
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
'--timeout=[]: :_default' \
'--user-agent=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
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
":: :_qsv__subcmd__snappy__subcmd__help_commands" \
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
'--timeout=[]: :_default' \
'--sample=[]: :_default' \
'--save-urlsample=[]: :_default' \
'--user-agent=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--quote=[]: :_default' \
'--json[]' \
'--just-mime[]' \
'--stats-types[]' \
'--prefer-dmy[]' \
'-Q[]' \
'--quick[]' \
'--pretty-json[]' \
'--no-infer[]' \
'-p[]' \
'--progressbar[]' \
'--harvest-mode[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sort)
_arguments "${_arguments_options[@]}" : \
'--rng=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--seed=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--natural[]' \
'--faster[]' \
'-N[]' \
'--numeric[]' \
'-R[]' \
'--reverse[]' \
'--random[]' \
'--memcheck[]' \
'-u[]' \
'--unique[]' \
'-i[]' \
'--ignore-case[]' \
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
'--all[]' \
'-n[]' \
'--no-headers[]' \
'-i[]' \
'--ignore-case[]' \
'-p[]' \
'--progressbar[]' \
'--json[]' \
'--pretty-json[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(split)
_arguments "${_arguments_options[@]}" : \
'--filter=[]: :_default' \
'--pad=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-c+[]: :_default' \
'--chunks=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--kb-size=[]: :_default' \
'--filename=[]: :_default' \
'-s+[]: :_default' \
'--size=[]: :_default' \
'-q[]' \
'--quiet[]' \
'--filter-cleanup[]' \
'--filter-ignore-errors[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sqlp)
_arguments "${_arguments_options[@]}" : \
'--date-format=[]: :_default' \
'--wnull-value=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--time-format=[]: :_default' \
'--compression=[]: :_default' \
'--compress-level=[]: :_default' \
'--format=[]: :_default' \
'--datetime-format=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--infer-len=[]: :_default' \
'--float-precision=[]: :_default' \
'--rnull-values=[]: :_default' \
'--no-optimizations[]' \
'--decimal-comma[]' \
'--streaming[]' \
'--truncate-ragged-lines[]' \
'--try-parsedates[]' \
'--cache-schema[]' \
'--statistics[]' \
'--ignore-errors[]' \
'-q[]' \
'--quiet[]' \
'--low-memory[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stats)
_arguments "${_arguments_options[@]}" : \
'--boolean-patterns=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--round=[]: :_default' \
'--percentile-list=[]: :_default' \
'--dates-whitelist=[]: :_default' \
'--weight=[]: :_default' \
'-c+[]: :_default' \
'--cache-threshold=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--mad[]' \
'--infer-boolean[]' \
'--typesonly[]' \
'--percentiles[]' \
'--vis-whitespace[]' \
'-n[]' \
'--no-headers[]' \
'--stats-jsonl[]' \
'-E[]' \
'--everything[]' \
'--mode[]' \
'--infer-dates[]' \
'--nulls[]' \
'--median[]' \
'--cardinality[]' \
'--quartiles[]' \
'--prefer-dmy[]' \
'--memcheck[]' \
'--force[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(table)
_arguments "${_arguments_options[@]}" : \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-a+[]: :_default' \
'--align=[]: :_default' \
'-c+[]: :_default' \
'--condense=[]: :_default' \
'-w+[]: :_default' \
'--width=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-p+[]: :_default' \
'--pad=[]: :_default' \
'--memcheck[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(template)
_arguments "${_arguments_options[@]}" : \
'--outsubdir-size=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-token=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--ckan-api=[]: :_default' \
'-j+[]: :_default' \
'--globals-json=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--delimiter=[]: :_default' \
'--timeout=[]: :_default' \
'--outfilename=[]: :_default' \
'--customfilter-error=[]: :_default' \
'--template=[]: :_default' \
'--jobs=[]: :_default' \
'-t+[]: :_default' \
'--template-file=[]: :_default' \
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
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'--compression=[]: :_default' \
'--compress-level=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-i[]' \
'--pipe[]' \
'-A[]' \
'--all-strings[]' \
'--drop[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-q[]' \
'--quiet[]' \
'-e[]' \
'--evolve[]' \
'--try-parse-dates[]' \
'-k[]' \
'--print-package[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__subcmd__to_commands" \
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
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'--compression=[]: :_default' \
'--compress-level=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-i[]' \
'--pipe[]' \
'-A[]' \
'--all-strings[]' \
'--drop[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-q[]' \
'--quiet[]' \
'-e[]' \
'--evolve[]' \
'--try-parse-dates[]' \
'-k[]' \
'--print-package[]' \
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
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'--compression=[]: :_default' \
'--compress-level=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-i[]' \
'--pipe[]' \
'-A[]' \
'--all-strings[]' \
'--drop[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-q[]' \
'--quiet[]' \
'-e[]' \
'--evolve[]' \
'--try-parse-dates[]' \
'-k[]' \
'--print-package[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(parquet)
_arguments "${_arguments_options[@]}" : \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'--compression=[]: :_default' \
'--compress-level=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-i[]' \
'--pipe[]' \
'-A[]' \
'--all-strings[]' \
'--drop[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-q[]' \
'--quiet[]' \
'-e[]' \
'--evolve[]' \
'--try-parse-dates[]' \
'-k[]' \
'--print-package[]' \
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
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'--compression=[]: :_default' \
'--compress-level=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-i[]' \
'--pipe[]' \
'-A[]' \
'--all-strings[]' \
'--drop[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-q[]' \
'--quiet[]' \
'-e[]' \
'--evolve[]' \
'--try-parse-dates[]' \
'-k[]' \
'--print-package[]' \
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
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'--compression=[]: :_default' \
'--compress-level=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-i[]' \
'--pipe[]' \
'-A[]' \
'--all-strings[]' \
'--drop[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-q[]' \
'--quiet[]' \
'-e[]' \
'--evolve[]' \
'--try-parse-dates[]' \
'-k[]' \
'--print-package[]' \
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
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'--compression=[]: :_default' \
'--compress-level=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-i[]' \
'--pipe[]' \
'-A[]' \
'--all-strings[]' \
'--drop[]' \
'-a[]' \
'--stats[]' \
'-u[]' \
'--dump[]' \
'-q[]' \
'--quiet[]' \
'-e[]' \
'--evolve[]' \
'--try-parse-dates[]' \
'-k[]' \
'--print-package[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__subcmd__to__subcmd__help_commands" \
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
(parquet)
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
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--trim[]' \
'--memcheck[]' \
'-q[]' \
'--quiet[]' \
'--no-boolean[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(transpose)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--long=[]: :_default' \
'--memcheck[]' \
'-m[]' \
'--multipass[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--ckan-api=[]: :_default' \
'--cache-dir=[]: :_default' \
'--email-min-subdomains=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--backtrack-limit=[]: :_default' \
'--timeout=[]: :_default' \
'--size-limit=[]: :_default' \
'--valid-output=[]: :_default' \
'--ckan-token=[]: :_default' \
'--valid=[]: :_default' \
'--invalid=[]: :_default' \
'--no-format-validation[]' \
'--trim[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
'--email-display-text[]' \
'--email-domain-literal[]' \
'--fail-fast[]' \
'--pretty-json[]' \
'--fancy-regex[]' \
'--json[]' \
'--email-required-tld[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__subcmd__validate_commands" \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--ckan-api=[]: :_default' \
'--cache-dir=[]: :_default' \
'--email-min-subdomains=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--backtrack-limit=[]: :_default' \
'--timeout=[]: :_default' \
'--size-limit=[]: :_default' \
'--valid-output=[]: :_default' \
'--ckan-token=[]: :_default' \
'--valid=[]: :_default' \
'--invalid=[]: :_default' \
'--no-format-validation[]' \
'--trim[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
'--email-display-text[]' \
'--email-domain-literal[]' \
'--fail-fast[]' \
'--pretty-json[]' \
'--fancy-regex[]' \
'--json[]' \
'--email-required-tld[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__subcmd__validate__subcmd__help_commands" \
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
":: :_qsv__subcmd__help_commands" \
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
":: :_qsv__subcmd__help__subcmd__apply_commands" \
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
(blake3)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cat)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__subcmd__help__subcmd__cat_commands" \
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
":: :_qsv__subcmd__help__subcmd__geocode_commands" \
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
":: :_qsv__subcmd__help__subcmd__luau_commands" \
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
":: :_qsv__subcmd__help__subcmd__pro_commands" \
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
":: :_qsv__subcmd__help__subcmd__py_commands" \
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
(scoresql)
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
":: :_qsv__subcmd__help__subcmd__snappy_commands" \
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
":: :_qsv__subcmd__help__subcmd__to_commands" \
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
(parquet)
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
":: :_qsv__subcmd__help__subcmd__validate_commands" \
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
'blake3:' \
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
'scoresql:' \
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
(( $+functions[_qsv__subcmd__apply_commands] )) ||
_qsv__subcmd__apply_commands() {
    local commands; commands=(
'calcconv:' \
'dynfmt:' \
'emptyreplace:' \
'operations:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv apply commands' commands "$@"
}
(( $+functions[_qsv__subcmd__apply__subcmd__calcconv_commands] )) ||
_qsv__subcmd__apply__subcmd__calcconv_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply calcconv commands' commands "$@"
}
(( $+functions[_qsv__subcmd__apply__subcmd__dynfmt_commands] )) ||
_qsv__subcmd__apply__subcmd__dynfmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply dynfmt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__apply__subcmd__emptyreplace_commands] )) ||
_qsv__subcmd__apply__subcmd__emptyreplace_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply emptyreplace commands' commands "$@"
}
(( $+functions[_qsv__subcmd__apply__subcmd__help_commands] )) ||
_qsv__subcmd__apply__subcmd__help_commands() {
    local commands; commands=(
'calcconv:' \
'dynfmt:' \
'emptyreplace:' \
'operations:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv apply help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__apply__subcmd__help__subcmd__calcconv_commands] )) ||
_qsv__subcmd__apply__subcmd__help__subcmd__calcconv_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply help calcconv commands' commands "$@"
}
(( $+functions[_qsv__subcmd__apply__subcmd__help__subcmd__dynfmt_commands] )) ||
_qsv__subcmd__apply__subcmd__help__subcmd__dynfmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply help dynfmt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__apply__subcmd__help__subcmd__emptyreplace_commands] )) ||
_qsv__subcmd__apply__subcmd__help__subcmd__emptyreplace_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply help emptyreplace commands' commands "$@"
}
(( $+functions[_qsv__subcmd__apply__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__apply__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply help help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__apply__subcmd__help__subcmd__operations_commands] )) ||
_qsv__subcmd__apply__subcmd__help__subcmd__operations_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply help operations commands' commands "$@"
}
(( $+functions[_qsv__subcmd__apply__subcmd__operations_commands] )) ||
_qsv__subcmd__apply__subcmd__operations_commands() {
    local commands; commands=()
    _describe -t commands 'qsv apply operations commands' commands "$@"
}
(( $+functions[_qsv__subcmd__behead_commands] )) ||
_qsv__subcmd__behead_commands() {
    local commands; commands=()
    _describe -t commands 'qsv behead commands' commands "$@"
}
(( $+functions[_qsv__subcmd__blake3_commands] )) ||
_qsv__subcmd__blake3_commands() {
    local commands; commands=()
    _describe -t commands 'qsv blake3 commands' commands "$@"
}
(( $+functions[_qsv__subcmd__cat_commands] )) ||
_qsv__subcmd__cat_commands() {
    local commands; commands=(
'columns:' \
'rows:' \
'rowskey:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv cat commands' commands "$@"
}
(( $+functions[_qsv__subcmd__cat__subcmd__columns_commands] )) ||
_qsv__subcmd__cat__subcmd__columns_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat columns commands' commands "$@"
}
(( $+functions[_qsv__subcmd__cat__subcmd__help_commands] )) ||
_qsv__subcmd__cat__subcmd__help_commands() {
    local commands; commands=(
'columns:' \
'rows:' \
'rowskey:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv cat help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__cat__subcmd__help__subcmd__columns_commands] )) ||
_qsv__subcmd__cat__subcmd__help__subcmd__columns_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat help columns commands' commands "$@"
}
(( $+functions[_qsv__subcmd__cat__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__cat__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat help help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__cat__subcmd__help__subcmd__rows_commands] )) ||
_qsv__subcmd__cat__subcmd__help__subcmd__rows_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat help rows commands' commands "$@"
}
(( $+functions[_qsv__subcmd__cat__subcmd__help__subcmd__rowskey_commands] )) ||
_qsv__subcmd__cat__subcmd__help__subcmd__rowskey_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat help rowskey commands' commands "$@"
}
(( $+functions[_qsv__subcmd__cat__subcmd__rows_commands] )) ||
_qsv__subcmd__cat__subcmd__rows_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat rows commands' commands "$@"
}
(( $+functions[_qsv__subcmd__cat__subcmd__rowskey_commands] )) ||
_qsv__subcmd__cat__subcmd__rowskey_commands() {
    local commands; commands=()
    _describe -t commands 'qsv cat rowskey commands' commands "$@"
}
(( $+functions[_qsv__subcmd__clipboard_commands] )) ||
_qsv__subcmd__clipboard_commands() {
    local commands; commands=()
    _describe -t commands 'qsv clipboard commands' commands "$@"
}
(( $+functions[_qsv__subcmd__color_commands] )) ||
_qsv__subcmd__color_commands() {
    local commands; commands=()
    _describe -t commands 'qsv color commands' commands "$@"
}
(( $+functions[_qsv__subcmd__count_commands] )) ||
_qsv__subcmd__count_commands() {
    local commands; commands=()
    _describe -t commands 'qsv count commands' commands "$@"
}
(( $+functions[_qsv__subcmd__datefmt_commands] )) ||
_qsv__subcmd__datefmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv datefmt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__dedup_commands] )) ||
_qsv__subcmd__dedup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv dedup commands' commands "$@"
}
(( $+functions[_qsv__subcmd__describegpt_commands] )) ||
_qsv__subcmd__describegpt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv describegpt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__diff_commands] )) ||
_qsv__subcmd__diff_commands() {
    local commands; commands=()
    _describe -t commands 'qsv diff commands' commands "$@"
}
(( $+functions[_qsv__subcmd__edit_commands] )) ||
_qsv__subcmd__edit_commands() {
    local commands; commands=()
    _describe -t commands 'qsv edit commands' commands "$@"
}
(( $+functions[_qsv__subcmd__enum_commands] )) ||
_qsv__subcmd__enum_commands() {
    local commands; commands=()
    _describe -t commands 'qsv enum commands' commands "$@"
}
(( $+functions[_qsv__subcmd__excel_commands] )) ||
_qsv__subcmd__excel_commands() {
    local commands; commands=()
    _describe -t commands 'qsv excel commands' commands "$@"
}
(( $+functions[_qsv__subcmd__exclude_commands] )) ||
_qsv__subcmd__exclude_commands() {
    local commands; commands=()
    _describe -t commands 'qsv exclude commands' commands "$@"
}
(( $+functions[_qsv__subcmd__explode_commands] )) ||
_qsv__subcmd__explode_commands() {
    local commands; commands=()
    _describe -t commands 'qsv explode commands' commands "$@"
}
(( $+functions[_qsv__subcmd__extdedup_commands] )) ||
_qsv__subcmd__extdedup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv extdedup commands' commands "$@"
}
(( $+functions[_qsv__subcmd__extsort_commands] )) ||
_qsv__subcmd__extsort_commands() {
    local commands; commands=()
    _describe -t commands 'qsv extsort commands' commands "$@"
}
(( $+functions[_qsv__subcmd__fetch_commands] )) ||
_qsv__subcmd__fetch_commands() {
    local commands; commands=()
    _describe -t commands 'qsv fetch commands' commands "$@"
}
(( $+functions[_qsv__subcmd__fetchpost_commands] )) ||
_qsv__subcmd__fetchpost_commands() {
    local commands; commands=()
    _describe -t commands 'qsv fetchpost commands' commands "$@"
}
(( $+functions[_qsv__subcmd__fill_commands] )) ||
_qsv__subcmd__fill_commands() {
    local commands; commands=()
    _describe -t commands 'qsv fill commands' commands "$@"
}
(( $+functions[_qsv__subcmd__fixlengths_commands] )) ||
_qsv__subcmd__fixlengths_commands() {
    local commands; commands=()
    _describe -t commands 'qsv fixlengths commands' commands "$@"
}
(( $+functions[_qsv__subcmd__flatten_commands] )) ||
_qsv__subcmd__flatten_commands() {
    local commands; commands=()
    _describe -t commands 'qsv flatten commands' commands "$@"
}
(( $+functions[_qsv__subcmd__fmt_commands] )) ||
_qsv__subcmd__fmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv fmt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__foreach_commands] )) ||
_qsv__subcmd__foreach_commands() {
    local commands; commands=()
    _describe -t commands 'qsv foreach commands' commands "$@"
}
(( $+functions[_qsv__subcmd__frequency_commands] )) ||
_qsv__subcmd__frequency_commands() {
    local commands; commands=()
    _describe -t commands 'qsv frequency commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode_commands] )) ||
_qsv__subcmd__geocode_commands() {
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
(( $+functions[_qsv__subcmd__geocode__subcmd__countryinfo_commands] )) ||
_qsv__subcmd__geocode__subcmd__countryinfo_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode countryinfo commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__countryinfonow_commands] )) ||
_qsv__subcmd__geocode__subcmd__countryinfonow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode countryinfonow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help_commands] )) ||
_qsv__subcmd__geocode__subcmd__help_commands() {
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
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__countryinfo_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__countryinfo_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help countryinfo commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__countryinfonow_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__countryinfonow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help countryinfonow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__index-check_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__index-check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help index-check commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__index-load_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__index-load_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help index-load commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__index-reset_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__index-reset_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help index-reset commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__index-update_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__index-update_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help index-update commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__iplookup_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__iplookup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help iplookup commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__iplookupnow_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__iplookupnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help iplookupnow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__reverse_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help reverse commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__reversenow_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__reversenow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help reversenow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__suggest_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__suggest_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help suggest commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__suggestnow_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__suggestnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help suggestnow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__index-check_commands] )) ||
_qsv__subcmd__geocode__subcmd__index-check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode index-check commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__index-load_commands] )) ||
_qsv__subcmd__geocode__subcmd__index-load_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode index-load commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__index-reset_commands] )) ||
_qsv__subcmd__geocode__subcmd__index-reset_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode index-reset commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__index-update_commands] )) ||
_qsv__subcmd__geocode__subcmd__index-update_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode index-update commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__iplookup_commands] )) ||
_qsv__subcmd__geocode__subcmd__iplookup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode iplookup commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__iplookupnow_commands] )) ||
_qsv__subcmd__geocode__subcmd__iplookupnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode iplookupnow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__reverse_commands] )) ||
_qsv__subcmd__geocode__subcmd__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode reverse commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__reversenow_commands] )) ||
_qsv__subcmd__geocode__subcmd__reversenow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode reversenow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__suggest_commands] )) ||
_qsv__subcmd__geocode__subcmd__suggest_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode suggest commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__suggestnow_commands] )) ||
_qsv__subcmd__geocode__subcmd__suggestnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode suggestnow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geoconvert_commands] )) ||
_qsv__subcmd__geoconvert_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geoconvert commands' commands "$@"
}
(( $+functions[_qsv__subcmd__headers_commands] )) ||
_qsv__subcmd__headers_commands() {
    local commands; commands=()
    _describe -t commands 'qsv headers commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help_commands] )) ||
_qsv__subcmd__help_commands() {
    local commands; commands=(
'apply:' \
'behead:' \
'blake3:' \
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
'scoresql:' \
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
(( $+functions[_qsv__subcmd__help__subcmd__apply_commands] )) ||
_qsv__subcmd__help__subcmd__apply_commands() {
    local commands; commands=(
'calcconv:' \
'dynfmt:' \
'emptyreplace:' \
'operations:' \
    )
    _describe -t commands 'qsv help apply commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__apply__subcmd__calcconv_commands] )) ||
_qsv__subcmd__help__subcmd__apply__subcmd__calcconv_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help apply calcconv commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__apply__subcmd__dynfmt_commands] )) ||
_qsv__subcmd__help__subcmd__apply__subcmd__dynfmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help apply dynfmt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__apply__subcmd__emptyreplace_commands] )) ||
_qsv__subcmd__help__subcmd__apply__subcmd__emptyreplace_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help apply emptyreplace commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__apply__subcmd__operations_commands] )) ||
_qsv__subcmd__help__subcmd__apply__subcmd__operations_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help apply operations commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__behead_commands] )) ||
_qsv__subcmd__help__subcmd__behead_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help behead commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__blake3_commands] )) ||
_qsv__subcmd__help__subcmd__blake3_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help blake3 commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__cat_commands] )) ||
_qsv__subcmd__help__subcmd__cat_commands() {
    local commands; commands=(
'columns:' \
'rows:' \
'rowskey:' \
    )
    _describe -t commands 'qsv help cat commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__cat__subcmd__columns_commands] )) ||
_qsv__subcmd__help__subcmd__cat__subcmd__columns_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help cat columns commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__cat__subcmd__rows_commands] )) ||
_qsv__subcmd__help__subcmd__cat__subcmd__rows_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help cat rows commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__cat__subcmd__rowskey_commands] )) ||
_qsv__subcmd__help__subcmd__cat__subcmd__rowskey_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help cat rowskey commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__clipboard_commands] )) ||
_qsv__subcmd__help__subcmd__clipboard_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help clipboard commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__color_commands] )) ||
_qsv__subcmd__help__subcmd__color_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help color commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__count_commands] )) ||
_qsv__subcmd__help__subcmd__count_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help count commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__datefmt_commands] )) ||
_qsv__subcmd__help__subcmd__datefmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help datefmt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__dedup_commands] )) ||
_qsv__subcmd__help__subcmd__dedup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help dedup commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__describegpt_commands] )) ||
_qsv__subcmd__help__subcmd__describegpt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help describegpt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__diff_commands] )) ||
_qsv__subcmd__help__subcmd__diff_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help diff commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__edit_commands] )) ||
_qsv__subcmd__help__subcmd__edit_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help edit commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__enum_commands] )) ||
_qsv__subcmd__help__subcmd__enum_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help enum commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__excel_commands] )) ||
_qsv__subcmd__help__subcmd__excel_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help excel commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__exclude_commands] )) ||
_qsv__subcmd__help__subcmd__exclude_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help exclude commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__explode_commands] )) ||
_qsv__subcmd__help__subcmd__explode_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help explode commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__extdedup_commands] )) ||
_qsv__subcmd__help__subcmd__extdedup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help extdedup commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__extsort_commands] )) ||
_qsv__subcmd__help__subcmd__extsort_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help extsort commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__fetch_commands] )) ||
_qsv__subcmd__help__subcmd__fetch_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help fetch commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__fetchpost_commands] )) ||
_qsv__subcmd__help__subcmd__fetchpost_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help fetchpost commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__fill_commands] )) ||
_qsv__subcmd__help__subcmd__fill_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help fill commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__fixlengths_commands] )) ||
_qsv__subcmd__help__subcmd__fixlengths_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help fixlengths commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__flatten_commands] )) ||
_qsv__subcmd__help__subcmd__flatten_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help flatten commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__fmt_commands] )) ||
_qsv__subcmd__help__subcmd__fmt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help fmt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__foreach_commands] )) ||
_qsv__subcmd__help__subcmd__foreach_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help foreach commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__frequency_commands] )) ||
_qsv__subcmd__help__subcmd__frequency_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help frequency commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode_commands] )) ||
_qsv__subcmd__help__subcmd__geocode_commands() {
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
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__countryinfo_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__countryinfo_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode countryinfo commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__countryinfonow_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__countryinfonow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode countryinfonow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__index-check_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__index-check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode index-check commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__index-load_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__index-load_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode index-load commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__index-reset_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__index-reset_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode index-reset commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__index-update_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__index-update_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode index-update commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__iplookup_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__iplookup_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode iplookup commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__iplookupnow_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__iplookupnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode iplookupnow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__reverse_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode reverse commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__reversenow_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__reversenow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode reversenow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__suggest_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__suggest_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode suggest commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__suggestnow_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__suggestnow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode suggestnow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geoconvert_commands] )) ||
_qsv__subcmd__help__subcmd__geoconvert_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geoconvert commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__headers_commands] )) ||
_qsv__subcmd__help__subcmd__headers_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help headers commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__index_commands] )) ||
_qsv__subcmd__help__subcmd__index_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help index commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__input_commands] )) ||
_qsv__subcmd__help__subcmd__input_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help input commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__join_commands] )) ||
_qsv__subcmd__help__subcmd__join_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help join commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__joinp_commands] )) ||
_qsv__subcmd__help__subcmd__joinp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help joinp commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__json_commands] )) ||
_qsv__subcmd__help__subcmd__json_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help json commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__jsonl_commands] )) ||
_qsv__subcmd__help__subcmd__jsonl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help jsonl commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__lens_commands] )) ||
_qsv__subcmd__help__subcmd__lens_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help lens commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__log_commands] )) ||
_qsv__subcmd__help__subcmd__log_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help log commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__luau_commands] )) ||
_qsv__subcmd__help__subcmd__luau_commands() {
    local commands; commands=(
'filter:' \
'map:' \
    )
    _describe -t commands 'qsv help luau commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__luau__subcmd__filter_commands] )) ||
_qsv__subcmd__help__subcmd__luau__subcmd__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help luau filter commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__luau__subcmd__map_commands] )) ||
_qsv__subcmd__help__subcmd__luau__subcmd__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help luau map commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__moarstats_commands] )) ||
_qsv__subcmd__help__subcmd__moarstats_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help moarstats commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__partition_commands] )) ||
_qsv__subcmd__help__subcmd__partition_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help partition commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__pivotp_commands] )) ||
_qsv__subcmd__help__subcmd__pivotp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help pivotp commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__pragmastat_commands] )) ||
_qsv__subcmd__help__subcmd__pragmastat_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help pragmastat commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__pro_commands] )) ||
_qsv__subcmd__help__subcmd__pro_commands() {
    local commands; commands=(
'lens:' \
'workflow:' \
    )
    _describe -t commands 'qsv help pro commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__pro__subcmd__lens_commands] )) ||
_qsv__subcmd__help__subcmd__pro__subcmd__lens_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help pro lens commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__pro__subcmd__workflow_commands] )) ||
_qsv__subcmd__help__subcmd__pro__subcmd__workflow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help pro workflow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__prompt_commands] )) ||
_qsv__subcmd__help__subcmd__prompt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help prompt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__pseudo_commands] )) ||
_qsv__subcmd__help__subcmd__pseudo_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help pseudo commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__py_commands] )) ||
_qsv__subcmd__help__subcmd__py_commands() {
    local commands; commands=(
'filter:' \
'map:' \
    )
    _describe -t commands 'qsv help py commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__py__subcmd__filter_commands] )) ||
_qsv__subcmd__help__subcmd__py__subcmd__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help py filter commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__py__subcmd__map_commands] )) ||
_qsv__subcmd__help__subcmd__py__subcmd__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help py map commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__rename_commands] )) ||
_qsv__subcmd__help__subcmd__rename_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help rename commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__replace_commands] )) ||
_qsv__subcmd__help__subcmd__replace_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help replace commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__reverse_commands] )) ||
_qsv__subcmd__help__subcmd__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help reverse commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__safenames_commands] )) ||
_qsv__subcmd__help__subcmd__safenames_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help safenames commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__sample_commands] )) ||
_qsv__subcmd__help__subcmd__sample_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help sample commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__schema_commands] )) ||
_qsv__subcmd__help__subcmd__schema_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help schema commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__scoresql_commands] )) ||
_qsv__subcmd__help__subcmd__scoresql_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help scoresql commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__search_commands] )) ||
_qsv__subcmd__help__subcmd__search_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help search commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__searchset_commands] )) ||
_qsv__subcmd__help__subcmd__searchset_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help searchset commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__select_commands] )) ||
_qsv__subcmd__help__subcmd__select_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help select commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__slice_commands] )) ||
_qsv__subcmd__help__subcmd__slice_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help slice commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__snappy_commands] )) ||
_qsv__subcmd__help__subcmd__snappy_commands() {
    local commands; commands=(
'check:' \
'compress:' \
'decompress:' \
'validate:' \
    )
    _describe -t commands 'qsv help snappy commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__snappy__subcmd__check_commands] )) ||
_qsv__subcmd__help__subcmd__snappy__subcmd__check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help snappy check commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__snappy__subcmd__compress_commands] )) ||
_qsv__subcmd__help__subcmd__snappy__subcmd__compress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help snappy compress commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__snappy__subcmd__decompress_commands] )) ||
_qsv__subcmd__help__subcmd__snappy__subcmd__decompress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help snappy decompress commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__snappy__subcmd__validate_commands] )) ||
_qsv__subcmd__help__subcmd__snappy__subcmd__validate_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help snappy validate commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__sniff_commands] )) ||
_qsv__subcmd__help__subcmd__sniff_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help sniff commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__sort_commands] )) ||
_qsv__subcmd__help__subcmd__sort_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help sort commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__sortcheck_commands] )) ||
_qsv__subcmd__help__subcmd__sortcheck_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help sortcheck commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__split_commands] )) ||
_qsv__subcmd__help__subcmd__split_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help split commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__sqlp_commands] )) ||
_qsv__subcmd__help__subcmd__sqlp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help sqlp commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__stats_commands] )) ||
_qsv__subcmd__help__subcmd__stats_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help stats commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__table_commands] )) ||
_qsv__subcmd__help__subcmd__table_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help table commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__template_commands] )) ||
_qsv__subcmd__help__subcmd__template_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help template commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__to_commands] )) ||
_qsv__subcmd__help__subcmd__to_commands() {
    local commands; commands=(
'datapackage:' \
'ods:' \
'parquet:' \
'postgres:' \
'sqlite:' \
'xlsx:' \
    )
    _describe -t commands 'qsv help to commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__to__subcmd__datapackage_commands] )) ||
_qsv__subcmd__help__subcmd__to__subcmd__datapackage_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to datapackage commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__to__subcmd__ods_commands] )) ||
_qsv__subcmd__help__subcmd__to__subcmd__ods_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to ods commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__to__subcmd__parquet_commands] )) ||
_qsv__subcmd__help__subcmd__to__subcmd__parquet_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to parquet commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__to__subcmd__postgres_commands] )) ||
_qsv__subcmd__help__subcmd__to__subcmd__postgres_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to postgres commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__to__subcmd__sqlite_commands] )) ||
_qsv__subcmd__help__subcmd__to__subcmd__sqlite_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to sqlite commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__to__subcmd__xlsx_commands] )) ||
_qsv__subcmd__help__subcmd__to__subcmd__xlsx_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help to xlsx commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__tojsonl_commands] )) ||
_qsv__subcmd__help__subcmd__tojsonl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help tojsonl commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__transpose_commands] )) ||
_qsv__subcmd__help__subcmd__transpose_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help transpose commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__validate_commands] )) ||
_qsv__subcmd__help__subcmd__validate_commands() {
    local commands; commands=(
'schema:' \
    )
    _describe -t commands 'qsv help validate commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__validate__subcmd__schema_commands] )) ||
_qsv__subcmd__help__subcmd__validate__subcmd__schema_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help validate schema commands' commands "$@"
}
(( $+functions[_qsv__subcmd__index_commands] )) ||
_qsv__subcmd__index_commands() {
    local commands; commands=()
    _describe -t commands 'qsv index commands' commands "$@"
}
(( $+functions[_qsv__subcmd__input_commands] )) ||
_qsv__subcmd__input_commands() {
    local commands; commands=()
    _describe -t commands 'qsv input commands' commands "$@"
}
(( $+functions[_qsv__subcmd__join_commands] )) ||
_qsv__subcmd__join_commands() {
    local commands; commands=()
    _describe -t commands 'qsv join commands' commands "$@"
}
(( $+functions[_qsv__subcmd__joinp_commands] )) ||
_qsv__subcmd__joinp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv joinp commands' commands "$@"
}
(( $+functions[_qsv__subcmd__json_commands] )) ||
_qsv__subcmd__json_commands() {
    local commands; commands=()
    _describe -t commands 'qsv json commands' commands "$@"
}
(( $+functions[_qsv__subcmd__jsonl_commands] )) ||
_qsv__subcmd__jsonl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv jsonl commands' commands "$@"
}
(( $+functions[_qsv__subcmd__lens_commands] )) ||
_qsv__subcmd__lens_commands() {
    local commands; commands=()
    _describe -t commands 'qsv lens commands' commands "$@"
}
(( $+functions[_qsv__subcmd__log_commands] )) ||
_qsv__subcmd__log_commands() {
    local commands; commands=()
    _describe -t commands 'qsv log commands' commands "$@"
}
(( $+functions[_qsv__subcmd__luau_commands] )) ||
_qsv__subcmd__luau_commands() {
    local commands; commands=(
'filter:' \
'map:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv luau commands' commands "$@"
}
(( $+functions[_qsv__subcmd__luau__subcmd__filter_commands] )) ||
_qsv__subcmd__luau__subcmd__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv luau filter commands' commands "$@"
}
(( $+functions[_qsv__subcmd__luau__subcmd__help_commands] )) ||
_qsv__subcmd__luau__subcmd__help_commands() {
    local commands; commands=(
'filter:' \
'map:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv luau help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__luau__subcmd__help__subcmd__filter_commands] )) ||
_qsv__subcmd__luau__subcmd__help__subcmd__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv luau help filter commands' commands "$@"
}
(( $+functions[_qsv__subcmd__luau__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__luau__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv luau help help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__luau__subcmd__help__subcmd__map_commands] )) ||
_qsv__subcmd__luau__subcmd__help__subcmd__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv luau help map commands' commands "$@"
}
(( $+functions[_qsv__subcmd__luau__subcmd__map_commands] )) ||
_qsv__subcmd__luau__subcmd__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv luau map commands' commands "$@"
}
(( $+functions[_qsv__subcmd__moarstats_commands] )) ||
_qsv__subcmd__moarstats_commands() {
    local commands; commands=()
    _describe -t commands 'qsv moarstats commands' commands "$@"
}
(( $+functions[_qsv__subcmd__partition_commands] )) ||
_qsv__subcmd__partition_commands() {
    local commands; commands=()
    _describe -t commands 'qsv partition commands' commands "$@"
}
(( $+functions[_qsv__subcmd__pivotp_commands] )) ||
_qsv__subcmd__pivotp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pivotp commands' commands "$@"
}
(( $+functions[_qsv__subcmd__pragmastat_commands] )) ||
_qsv__subcmd__pragmastat_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pragmastat commands' commands "$@"
}
(( $+functions[_qsv__subcmd__pro_commands] )) ||
_qsv__subcmd__pro_commands() {
    local commands; commands=(
'lens:' \
'workflow:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv pro commands' commands "$@"
}
(( $+functions[_qsv__subcmd__pro__subcmd__help_commands] )) ||
_qsv__subcmd__pro__subcmd__help_commands() {
    local commands; commands=(
'lens:' \
'workflow:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv pro help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__pro__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__pro__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pro help help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__pro__subcmd__help__subcmd__lens_commands] )) ||
_qsv__subcmd__pro__subcmd__help__subcmd__lens_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pro help lens commands' commands "$@"
}
(( $+functions[_qsv__subcmd__pro__subcmd__help__subcmd__workflow_commands] )) ||
_qsv__subcmd__pro__subcmd__help__subcmd__workflow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pro help workflow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__pro__subcmd__lens_commands] )) ||
_qsv__subcmd__pro__subcmd__lens_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pro lens commands' commands "$@"
}
(( $+functions[_qsv__subcmd__pro__subcmd__workflow_commands] )) ||
_qsv__subcmd__pro__subcmd__workflow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pro workflow commands' commands "$@"
}
(( $+functions[_qsv__subcmd__prompt_commands] )) ||
_qsv__subcmd__prompt_commands() {
    local commands; commands=()
    _describe -t commands 'qsv prompt commands' commands "$@"
}
(( $+functions[_qsv__subcmd__pseudo_commands] )) ||
_qsv__subcmd__pseudo_commands() {
    local commands; commands=()
    _describe -t commands 'qsv pseudo commands' commands "$@"
}
(( $+functions[_qsv__subcmd__py_commands] )) ||
_qsv__subcmd__py_commands() {
    local commands; commands=(
'filter:' \
'map:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv py commands' commands "$@"
}
(( $+functions[_qsv__subcmd__py__subcmd__filter_commands] )) ||
_qsv__subcmd__py__subcmd__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv py filter commands' commands "$@"
}
(( $+functions[_qsv__subcmd__py__subcmd__help_commands] )) ||
_qsv__subcmd__py__subcmd__help_commands() {
    local commands; commands=(
'filter:' \
'map:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv py help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__py__subcmd__help__subcmd__filter_commands] )) ||
_qsv__subcmd__py__subcmd__help__subcmd__filter_commands() {
    local commands; commands=()
    _describe -t commands 'qsv py help filter commands' commands "$@"
}
(( $+functions[_qsv__subcmd__py__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__py__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv py help help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__py__subcmd__help__subcmd__map_commands] )) ||
_qsv__subcmd__py__subcmd__help__subcmd__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv py help map commands' commands "$@"
}
(( $+functions[_qsv__subcmd__py__subcmd__map_commands] )) ||
_qsv__subcmd__py__subcmd__map_commands() {
    local commands; commands=()
    _describe -t commands 'qsv py map commands' commands "$@"
}
(( $+functions[_qsv__subcmd__rename_commands] )) ||
_qsv__subcmd__rename_commands() {
    local commands; commands=()
    _describe -t commands 'qsv rename commands' commands "$@"
}
(( $+functions[_qsv__subcmd__replace_commands] )) ||
_qsv__subcmd__replace_commands() {
    local commands; commands=()
    _describe -t commands 'qsv replace commands' commands "$@"
}
(( $+functions[_qsv__subcmd__reverse_commands] )) ||
_qsv__subcmd__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'qsv reverse commands' commands "$@"
}
(( $+functions[_qsv__subcmd__safenames_commands] )) ||
_qsv__subcmd__safenames_commands() {
    local commands; commands=()
    _describe -t commands 'qsv safenames commands' commands "$@"
}
(( $+functions[_qsv__subcmd__sample_commands] )) ||
_qsv__subcmd__sample_commands() {
    local commands; commands=()
    _describe -t commands 'qsv sample commands' commands "$@"
}
(( $+functions[_qsv__subcmd__schema_commands] )) ||
_qsv__subcmd__schema_commands() {
    local commands; commands=()
    _describe -t commands 'qsv schema commands' commands "$@"
}
(( $+functions[_qsv__subcmd__scoresql_commands] )) ||
_qsv__subcmd__scoresql_commands() {
    local commands; commands=()
    _describe -t commands 'qsv scoresql commands' commands "$@"
}
(( $+functions[_qsv__subcmd__search_commands] )) ||
_qsv__subcmd__search_commands() {
    local commands; commands=()
    _describe -t commands 'qsv search commands' commands "$@"
}
(( $+functions[_qsv__subcmd__searchset_commands] )) ||
_qsv__subcmd__searchset_commands() {
    local commands; commands=()
    _describe -t commands 'qsv searchset commands' commands "$@"
}
(( $+functions[_qsv__subcmd__select_commands] )) ||
_qsv__subcmd__select_commands() {
    local commands; commands=()
    _describe -t commands 'qsv select commands' commands "$@"
}
(( $+functions[_qsv__subcmd__slice_commands] )) ||
_qsv__subcmd__slice_commands() {
    local commands; commands=()
    _describe -t commands 'qsv slice commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy_commands] )) ||
_qsv__subcmd__snappy_commands() {
    local commands; commands=(
'check:' \
'compress:' \
'decompress:' \
'validate:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv snappy commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy__subcmd__check_commands] )) ||
_qsv__subcmd__snappy__subcmd__check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy check commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy__subcmd__compress_commands] )) ||
_qsv__subcmd__snappy__subcmd__compress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy compress commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy__subcmd__decompress_commands] )) ||
_qsv__subcmd__snappy__subcmd__decompress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy decompress commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy__subcmd__help_commands] )) ||
_qsv__subcmd__snappy__subcmd__help_commands() {
    local commands; commands=(
'check:' \
'compress:' \
'decompress:' \
'validate:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv snappy help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy__subcmd__help__subcmd__check_commands] )) ||
_qsv__subcmd__snappy__subcmd__help__subcmd__check_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy help check commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy__subcmd__help__subcmd__compress_commands] )) ||
_qsv__subcmd__snappy__subcmd__help__subcmd__compress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy help compress commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy__subcmd__help__subcmd__decompress_commands] )) ||
_qsv__subcmd__snappy__subcmd__help__subcmd__decompress_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy help decompress commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__snappy__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy help help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy__subcmd__help__subcmd__validate_commands] )) ||
_qsv__subcmd__snappy__subcmd__help__subcmd__validate_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy help validate commands' commands "$@"
}
(( $+functions[_qsv__subcmd__snappy__subcmd__validate_commands] )) ||
_qsv__subcmd__snappy__subcmd__validate_commands() {
    local commands; commands=()
    _describe -t commands 'qsv snappy validate commands' commands "$@"
}
(( $+functions[_qsv__subcmd__sniff_commands] )) ||
_qsv__subcmd__sniff_commands() {
    local commands; commands=()
    _describe -t commands 'qsv sniff commands' commands "$@"
}
(( $+functions[_qsv__subcmd__sort_commands] )) ||
_qsv__subcmd__sort_commands() {
    local commands; commands=()
    _describe -t commands 'qsv sort commands' commands "$@"
}
(( $+functions[_qsv__subcmd__sortcheck_commands] )) ||
_qsv__subcmd__sortcheck_commands() {
    local commands; commands=()
    _describe -t commands 'qsv sortcheck commands' commands "$@"
}
(( $+functions[_qsv__subcmd__split_commands] )) ||
_qsv__subcmd__split_commands() {
    local commands; commands=()
    _describe -t commands 'qsv split commands' commands "$@"
}
(( $+functions[_qsv__subcmd__sqlp_commands] )) ||
_qsv__subcmd__sqlp_commands() {
    local commands; commands=()
    _describe -t commands 'qsv sqlp commands' commands "$@"
}
(( $+functions[_qsv__subcmd__stats_commands] )) ||
_qsv__subcmd__stats_commands() {
    local commands; commands=()
    _describe -t commands 'qsv stats commands' commands "$@"
}
(( $+functions[_qsv__subcmd__table_commands] )) ||
_qsv__subcmd__table_commands() {
    local commands; commands=()
    _describe -t commands 'qsv table commands' commands "$@"
}
(( $+functions[_qsv__subcmd__template_commands] )) ||
_qsv__subcmd__template_commands() {
    local commands; commands=()
    _describe -t commands 'qsv template commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to_commands] )) ||
_qsv__subcmd__to_commands() {
    local commands; commands=(
'datapackage:' \
'ods:' \
'parquet:' \
'postgres:' \
'sqlite:' \
'xlsx:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv to commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__datapackage_commands] )) ||
_qsv__subcmd__to__subcmd__datapackage_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to datapackage commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__help_commands] )) ||
_qsv__subcmd__to__subcmd__help_commands() {
    local commands; commands=(
'datapackage:' \
'ods:' \
'parquet:' \
'postgres:' \
'sqlite:' \
'xlsx:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv to help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__help__subcmd__datapackage_commands] )) ||
_qsv__subcmd__to__subcmd__help__subcmd__datapackage_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help datapackage commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__to__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__help__subcmd__ods_commands] )) ||
_qsv__subcmd__to__subcmd__help__subcmd__ods_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help ods commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__help__subcmd__parquet_commands] )) ||
_qsv__subcmd__to__subcmd__help__subcmd__parquet_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help parquet commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__help__subcmd__postgres_commands] )) ||
_qsv__subcmd__to__subcmd__help__subcmd__postgres_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help postgres commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__help__subcmd__sqlite_commands] )) ||
_qsv__subcmd__to__subcmd__help__subcmd__sqlite_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help sqlite commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__help__subcmd__xlsx_commands] )) ||
_qsv__subcmd__to__subcmd__help__subcmd__xlsx_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to help xlsx commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__ods_commands] )) ||
_qsv__subcmd__to__subcmd__ods_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to ods commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__parquet_commands] )) ||
_qsv__subcmd__to__subcmd__parquet_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to parquet commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__postgres_commands] )) ||
_qsv__subcmd__to__subcmd__postgres_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to postgres commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__sqlite_commands] )) ||
_qsv__subcmd__to__subcmd__sqlite_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to sqlite commands' commands "$@"
}
(( $+functions[_qsv__subcmd__to__subcmd__xlsx_commands] )) ||
_qsv__subcmd__to__subcmd__xlsx_commands() {
    local commands; commands=()
    _describe -t commands 'qsv to xlsx commands' commands "$@"
}
(( $+functions[_qsv__subcmd__tojsonl_commands] )) ||
_qsv__subcmd__tojsonl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv tojsonl commands' commands "$@"
}
(( $+functions[_qsv__subcmd__transpose_commands] )) ||
_qsv__subcmd__transpose_commands() {
    local commands; commands=()
    _describe -t commands 'qsv transpose commands' commands "$@"
}
(( $+functions[_qsv__subcmd__validate_commands] )) ||
_qsv__subcmd__validate_commands() {
    local commands; commands=(
'schema:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv validate commands' commands "$@"
}
(( $+functions[_qsv__subcmd__validate__subcmd__help_commands] )) ||
_qsv__subcmd__validate__subcmd__help_commands() {
    local commands; commands=(
'schema:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv validate help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__validate__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__validate__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv validate help help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__validate__subcmd__help__subcmd__schema_commands] )) ||
_qsv__subcmd__validate__subcmd__help__subcmd__schema_commands() {
    local commands; commands=()
    _describe -t commands 'qsv validate help schema commands' commands "$@"
}
(( $+functions[_qsv__subcmd__validate__subcmd__schema_commands] )) ||
_qsv__subcmd__validate__subcmd__schema_commands() {
    local commands; commands=()
    _describe -t commands 'qsv validate schema commands' commands "$@"
}

if [ "$funcstack[1]" = "_qsv" ]; then
    _qsv "$@"
else
    compdef _qsv qsv
fi
