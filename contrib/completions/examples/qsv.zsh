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
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
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
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
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
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
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
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
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
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-C+[]: :_default' \
'--comparand=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--replacement=[]: :_default' \
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
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-l+[]: :_default' \
'--length=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-c[]' \
'--check[]' \
'--keyed[]' \
'--no-mmap[]' \
'--no-names[]' \
'-q[]' \
'--quiet[]' \
'--raw[]' \
'--tag[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cat)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-g+[]: :_default' \
'--group=[]: :_default' \
'-N+[]: :_default' \
'--group-name=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-t+[]: :_default' \
'--title=[]: :_default' \
'-C[]' \
'--color[]' \
'--memcheck[]' \
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
'-f[]' \
'--flexible[]' \
'-H[]' \
'--human-readable[]' \
'--json[]' \
'--low-memory[]' \
'-n[]' \
'--no-headers[]' \
'--no-polars[]' \
'--width[]' \
'--width-no-delims[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(datefmt)
_arguments "${_arguments_options[@]}" : \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--default-tz=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--formatstr=[]: :_default' \
'--input-tz=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--output-tz=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-R+[]: :_default' \
'--ts-resolution=[]: :_default' \
'--keep-zero-time[]' \
'-n[]' \
'--no-headers[]' \
'--prefer-dmy[]' \
'-p[]' \
'--progressbar[]' \
'--utc[]' \
'--zulu[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(dedup)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-D+[]: :_default' \
'--dupes-output=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'-H[]' \
'--human-readable[]' \
'-i[]' \
'--ignore-case[]' \
'--memcheck[]' \
'-n[]' \
'--no-headers[]' \
'-N[]' \
'--numeric[]' \
'-q[]' \
'--quiet[]' \
'--sorted[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(describegpt)
_arguments "${_arguments_options[@]}" : \
'--addl-cols-list=[]: :_default' \
'--addl-props=[]: :_default' \
'-k+[]: :_default' \
'--api-key=[]: :_default' \
'-u+[]: :_default' \
'--base-url=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'--ds-license=[]: :_default' \
'--ds-source=[]: :_default' \
'--ds-updated=[]: :_default' \
'--enum-threshold=[]: :_default' \
'--export-prompt=[]: :_default' \
'--format=[]: :_default' \
'--freq-options=[]: :_default' \
'--language=[]: :_default' \
'--markdown-template=[]: :_default' \
'-t+[]: :_default' \
'--max-tokens=[]: :_default' \
'-m+[]: :_default' \
'--model=[]: :_default' \
'--num-examples=[]: :_default' \
'--num-tags=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-p+[]: :_default' \
'--prompt=[]: :_default' \
'--prompt-file=[]: :_default' \
'--sample-size=[]: :_default' \
'--score-max-retries=[]: :_default' \
'--score-threshold=[]: :_default' \
'--session=[]: :_default' \
'--session-len=[]: :_default' \
'--sql-results=[]: :_default' \
'--stats-options=[]: :_default' \
'--tag-vocab=[]: :_default' \
'--timeout=[]: :_default' \
'--truncate-str=[]: :_default' \
'--user-agent=[]: :_default' \
'--addl-cols[]' \
'-A[]' \
'--all[]' \
'--allow-extra-cols[]' \
'--description[]' \
'--dictionary[]' \
'--fewshot-examples[]' \
'--flush-cache[]' \
'--forget[]' \
'--fresh[]' \
'--infer-content-type[]' \
'--no-cache[]' \
'--no-score-sql[]' \
'--prepare-context[]' \
'--process-response[]' \
'-q[]' \
'--quiet[]' \
'--redis-cache[]' \
'--strict-dates[]' \
'--tags[]' \
'--two-pass[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(diff)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--delimiter-left=[]: :_default' \
'--delimiter-output=[]: :_default' \
'--delimiter-right=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--key=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--sort-columns=[]: :_default' \
'--drop-equal-fields[]' \
'--no-headers-left[]' \
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
'--constant=[]: :_default' \
'--copy=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--hash=[]: :_default' \
'--increment=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--start=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--uuid4[]' \
'--uuid7[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(excel)
_arguments "${_arguments_options[@]}" : \
'--cell=[]: :_default' \
'--date-format=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--error-format=[]: :_default' \
'--header-row=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--metadata=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--range=[]: :_default' \
'-s+[]: :_default' \
'--sheet=[]: :_default' \
'--table=[]: :_default' \
'--flexible[]' \
'--keep-zero-time[]' \
'-q[]' \
'--quiet[]' \
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
'-v[]' \
'--invert[]' \
'--memcheck[]' \
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
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(extdedup)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-D+[]: :_default' \
'--dupes-output=[]: :_default' \
'--memory-limit=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--temp-dir=[]: :_default' \
'-H[]' \
'--human-readable[]' \
'-n[]' \
'--no-headers[]' \
'--no-output[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(extsort)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--memory-limit=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--tmp-dir=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'-H+[]: :_default' \
'--http-header=[]: :_default' \
'--jaq=[]: :_default' \
'--jaqfile=[]: :_default' \
'--max-errors=[]: :_default' \
'--max-retries=[]: :_default' \
'--mem-cache-size=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'--report=[]: :_default' \
'--timeout=[]: :_default' \
'--url-template=[]: :_default' \
'--user-agent=[]: :_default' \
'--cache-error[]' \
'--cookies[]' \
'--disk-cache[]' \
'--flush-cache[]' \
'--no-cache[]' \
'-n[]' \
'--no-headers[]' \
'--pretty[]' \
'-p[]' \
'--progressbar[]' \
'--redis-cache[]' \
'--store-error[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fetchpost)
_arguments "${_arguments_options[@]}" : \
'--content-type=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--disk-cache-dir=[]: :_default' \
'-j+[]: :_default' \
'--globals-json=[]: :_default' \
'-H+[]: :_default' \
'--http-header=[]: :_default' \
'--jaq=[]: :_default' \
'--jaqfile=[]: :_default' \
'--max-errors=[]: :_default' \
'--max-retries=[]: :_default' \
'--mem-cache-size=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-t+[]: :_default' \
'--payload-tpl=[]: :_default' \
'--rate-limit=[]: :_default' \
'--report=[]: :_default' \
'--timeout=[]: :_default' \
'--user-agent=[]: :_default' \
'--cache-error[]' \
'--compress[]' \
'--cookies[]' \
'--disk-cache[]' \
'--flush-cache[]' \
'--no-cache[]' \
'-n[]' \
'--no-headers[]' \
'--pretty[]' \
'-p[]' \
'--progressbar[]' \
'--redis-cache[]' \
'--store-error[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fill)
_arguments "${_arguments_options[@]}" : \
'-v+[]: :_default' \
'--default=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-g+[]: :_default' \
'--groupby=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
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
'--escape=[]: :_default' \
'-i+[]: :_default' \
'--insert=[]: :_default' \
'-l+[]: :_default' \
'--length=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--quote=[]: :_default' \
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
'-c+[]: :_default' \
'--condense=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--field-separator=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--escape=[]: :_default' \
'-t+[]: :_default' \
'--out-delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--quote=[]: :_default' \
'--ascii[]' \
'--crlf[]' \
'--no-final-newline[]' \
'--quote-always[]' \
'--quote-never[]' \
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
'--all-unique-text=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--high-card-pct=[]: :_default' \
'--high-card-threshold=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-l+[]: :_default' \
'--limit=[]: :_default' \
'--lmt-threshold=[]: :_default' \
'--no-float=[]: :_default' \
'--null-text=[]: :_default' \
'--other-text=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--pct-dec-places=[]: :_default' \
'-r+[]: :_default' \
'--rank-strategy=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--sketch-map-size=[]: :_default' \
'--sketch-method=[]: :_default' \
'--stats-filter=[]: :_default' \
'-u+[]: :_default' \
'--unq-limit=[]: :_default' \
'--weight=[]: :_default' \
'-a[]' \
'--asc[]' \
'--force[]' \
'--frequency-jsonl[]' \
'-i[]' \
'--ignore-case[]' \
'--json[]' \
'--memcheck[]' \
'-n[]' \
'--no-headers[]' \
'--no-nulls[]' \
'--no-other[]' \
'--no-stats[]' \
'--no-trim[]' \
'--null-sorted[]' \
'--other-sorted[]' \
'--pct-nulls[]' \
'--pretty-json[]' \
'--toon[]' \
'--vis-whitespace[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(geocode)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
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
            (cache-clear)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cache-info)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cache-prune)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(countryinfo)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(countryinfonow)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-check)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-load)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-reset)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(index-update)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(iplookup)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(iplookupnow)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(opencage)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(opencagenow)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reversenow)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(suggest)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(suggestnow)
_arguments "${_arguments_options[@]}" : \
'--admin1=[]: :_default' \
'--api-key=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--cache-ttl=[]: :_default' \
'--cities-url=[]: :_default' \
'--country=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-f+[]: :_default' \
'--formatstr=[]: :_default' \
'--invalid-result=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--k_weight=[]: :_default' \
'-l+[]: :_default' \
'--language=[]: :_default' \
'--languages=[]: :_default' \
'--min-score=[]: :_default' \
'-c+[]: :_default' \
'--new-column=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rate-limit=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'--timeout=[]: :_default' \
'--force[]' \
'--no-annotations[]' \
'--no-cache[]' \
'-p[]' \
'--progressbar[]' \
'--reverse[]' \
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
            (cache-clear)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-prune)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
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
(opencage)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(opencagenow)
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
'-y+[]: :_default' \
'--latitude=[]: :_default' \
'-x+[]: :_default' \
'--longitude=[]: :_default' \
'-l+[]: :_default' \
'--max-length=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(get)
_arguments "${_arguments_options[@]}" : \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'--cloud-opt=[]: :_default' \
'--compress=[]: :_default' \
'--name=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--refresh=[]: :_default' \
'--timeout=[]: :_default' \
'--ttl=[]: :_default' \
'--force[]' \
'--json[]' \
'-q[]' \
'--quiet[]' \
'--verify[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_qsv__subcmd__get_commands" \
"*::: :->get" \
&& ret=0

    case $state in
    (get)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-get-command-$line[1]:"
        case $line[1] in
            (cache-clear)
_arguments "${_arguments_options[@]}" : \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'--cloud-opt=[]: :_default' \
'--compress=[]: :_default' \
'--name=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--refresh=[]: :_default' \
'--timeout=[]: :_default' \
'--ttl=[]: :_default' \
'--force[]' \
'--json[]' \
'-q[]' \
'--quiet[]' \
'--verify[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cache-info)
_arguments "${_arguments_options[@]}" : \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'--cloud-opt=[]: :_default' \
'--compress=[]: :_default' \
'--name=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--refresh=[]: :_default' \
'--timeout=[]: :_default' \
'--ttl=[]: :_default' \
'--force[]' \
'--json[]' \
'-q[]' \
'--quiet[]' \
'--verify[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cache-list)
_arguments "${_arguments_options[@]}" : \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'--cloud-opt=[]: :_default' \
'--compress=[]: :_default' \
'--name=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--refresh=[]: :_default' \
'--timeout=[]: :_default' \
'--ttl=[]: :_default' \
'--force[]' \
'--json[]' \
'-q[]' \
'--quiet[]' \
'--verify[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cache-prune)
_arguments "${_arguments_options[@]}" : \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'--cloud-opt=[]: :_default' \
'--compress=[]: :_default' \
'--name=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--refresh=[]: :_default' \
'--timeout=[]: :_default' \
'--ttl=[]: :_default' \
'--force[]' \
'--json[]' \
'-q[]' \
'--quiet[]' \
'--verify[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cache-set-policy)
_arguments "${_arguments_options[@]}" : \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'--cloud-opt=[]: :_default' \
'--compress=[]: :_default' \
'--name=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--refresh=[]: :_default' \
'--timeout=[]: :_default' \
'--ttl=[]: :_default' \
'--force[]' \
'--json[]' \
'-q[]' \
'--quiet[]' \
'--verify[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cache-set-ttl)
_arguments "${_arguments_options[@]}" : \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'--cloud-opt=[]: :_default' \
'--compress=[]: :_default' \
'--name=[]: :_default' \
'--older-than=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--refresh=[]: :_default' \
'--timeout=[]: :_default' \
'--ttl=[]: :_default' \
'--force[]' \
'--json[]' \
'-q[]' \
'--quiet[]' \
'--verify[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__subcmd__get__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-get-help-command-$line[1]:"
        case $line[1] in
            (cache-clear)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-prune)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-set-policy)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-set-ttl)
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
(headers)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-J[]' \
'--just-count[]' \
'-j[]' \
'--just-names[]' \
'--trim[]' \
'--union[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(implode)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-k+[]: :_default' \
'--keys=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-r+[]: :_default' \
'--rename=[]: :_default' \
'-v+[]: :_default' \
'--value=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--skip-empty[]' \
'--sorted[]' \
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
'--comment=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--encoding-errors=[]: :_default' \
'--escape=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--quote=[]: :_default' \
'--quote-style=[]: :_default' \
'--skip-lastlines=[]: :_default' \
'--skip-lines=[]: :_default' \
'--auto-skip[]' \
'--no-quoting[]' \
'--trim-fields[]' \
'--trim-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(join)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--keys-output=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--cross[]' \
'--full[]' \
'-i[]' \
'--ignore-case[]' \
'-z[]' \
'--ignore-leading-zeros[]' \
'--left[]' \
'--left-anti[]' \
'--left-semi[]' \
'-n[]' \
'--no-headers[]' \
'--nulls[]' \
'--right[]' \
'--right-anti[]' \
'--right-semi[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(joinp)
_arguments "${_arguments_options[@]}" : \
'--cache-schema=[]: :_default' \
'--date-format=[]: :_default' \
'--datetime-format=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--filter-left=[]: :_default' \
'--filter-right=[]: :_default' \
'--float-precision=[]: :_default' \
'--infer-len=[]: :_default' \
'--left_by=[]: :_default' \
'--maintain-order=[]: :_default' \
'--non-equi=[]: :_default' \
'-N+[]: :_default' \
'--norm-unicode=[]: :_default' \
'--null-value=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--right_by=[]: :_default' \
'--sql-filter=[]: :_default' \
'--strategy=[]: :_default' \
'--time-format=[]: :_default' \
'--tolerance=[]: :_default' \
'--validate=[]: :_default' \
'-X[]' \
'--allow-exact-matches[]' \
'--asof[]' \
'--coalesce[]' \
'--cross[]' \
'--decimal-comma[]' \
'--full[]' \
'-i[]' \
'--ignore-case[]' \
'--ignore-errors[]' \
'-z[]' \
'--ignore-leading-zeros[]' \
'--left[]' \
'--left-anti[]' \
'--left-semi[]' \
'--low-memory[]' \
'--no-optimizations[]' \
'--no-sort[]' \
'--nulls[]' \
'-q[]' \
'--quiet[]' \
'--right[]' \
'--right-anti[]' \
'--right-semi[]' \
'--streaming[]' \
'--try-parsedates[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(json)
_arguments "${_arguments_options[@]}" : \
'--jaq=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--ignore-errors[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(lens)
_arguments "${_arguments_options[@]}" : \
'--columns=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--echo-column=[]: :_default' \
'--filter=[]: :_default' \
'--find=[]: :_default' \
'-f+[]: :_default' \
'--freeze-columns=[]: :_default' \
'-P+[]: :_default' \
'--prompt=[]: :_default' \
'-W+[]: :_default' \
'--wrap-mode=[]: :_default' \
'-A[]' \
'--auto-reload[]' \
'--debug[]' \
'-i[]' \
'--ignore-case[]' \
'-m[]' \
'--monochrome[]' \
'--no-headers[]' \
'-S[]' \
'--streaming-stdin[]' \
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
'-B+[]: :_default' \
'--begin=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'--max-errors=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
'--colindex[]' \
'-g[]' \
'--no-globals[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-r[]' \
'--remap[]' \
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
'-B+[]: :_default' \
'--begin=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'--max-errors=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
'--colindex[]' \
'-g[]' \
'--no-globals[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-r[]' \
'--remap[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(map)
_arguments "${_arguments_options[@]}" : \
'-B+[]: :_default' \
'--begin=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-E+[]: :_default' \
'--end=[]: :_default' \
'--max-errors=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
'--colindex[]' \
'-g[]' \
'--no-globals[]' \
'-n[]' \
'--no-headers[]' \
'-p[]' \
'--progressbar[]' \
'-r[]' \
'--remap[]' \
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
'-S+[]: :_default' \
'--bivariate-stats=[]: :_default' \
'-C+[]: :_default' \
'--cardinality-threshold=[]: :_default' \
'-e+[]: :_default' \
'--epsilon=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-J+[]: :_default' \
'--join-inputs=[]: :_default' \
'-K+[]: :_default' \
'--join-keys=[]: :_default' \
'-T+[]: :_default' \
'--join-type=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--pct-thresholds=[]: :_default' \
'--round=[]: :_default' \
'--stats-options=[]: :_default' \
'--xsd-gdate-scan=[]: :_default' \
'--advanced[]' \
'-B[]' \
'--bivariate[]' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--filename=[]: :_default' \
'--limit=[]: :_default' \
'-p+[]: :_default' \
'--prefix-length=[]: :_default' \
'--drop[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pivotp)
_arguments "${_arguments_options[@]}" : \
'-a+[]: :_default' \
'--agg=[]: :_default' \
'--col-separator=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-i+[]: :_default' \
'--index=[]: :_default' \
'--infer-len=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--total-label=[]: :_default' \
'-v+[]: :_default' \
'--values=[]: :_default' \
'--decimal-comma[]' \
'--grand-total[]' \
'--ignore-errors[]' \
'--maintain-order[]' \
'-q[]' \
'--quiet[]' \
'--sort-columns[]' \
'--subtotal[]' \
'--try-parsedates[]' \
'--validate[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pragmastat)
_arguments "${_arguments_options[@]}" : \
'--compare1=[]: :_default' \
'--compare2=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-m+[]: :_default' \
'--misrate=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--round=[]: :_default' \
'--seed=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--stats-options=[]: :_default' \
'--subsample=[]: :_default' \
'--force[]' \
'--memcheck[]' \
'--no-bounds[]' \
'-n[]' \
'--no-headers[]' \
'--standalone[]' \
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
(profile)
_arguments "${_arguments_options[@]}" : \
'--dcat-discovery-timeout=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--initial-context=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--profile=[]: :_default' \
'--spec=[]: :_default' \
'--allow-external-validator[]' \
'--catalog[]' \
'--croissant-frequency[]' \
'--dcat-legacy-license[]' \
'--force[]' \
'--memcheck[]' \
'--no-ckan[]' \
'--no-dcat-discovery[]' \
'-n[]' \
'--no-headers[]' \
'--no-projection[]' \
'--strict[]' \
'--validate[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(prompt)
_arguments "${_arguments_options[@]}" : \
'--base-delay-ms=[]: :_default' \
'-F+[]: :_default' \
'--filters=[]: :_default' \
'-m+[]: :_default' \
'--msg=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--save-fname=[]: :_default' \
'-d+[]: :_default' \
'--workdir=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--formatstr=[]: :_default' \
'--increment=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--start=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'--pairwise[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(replace)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--size-limit=[]: :_default' \
'--exact[]' \
'-i[]' \
'--ignore-case[]' \
'--literal[]' \
'-n[]' \
'--no-headers[]' \
'--not-one[]' \
'-p[]' \
'--progressbar[]' \
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
'--memcheck[]' \
'-n[]' \
'--no-headers[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(safenames)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--mode=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--prefix=[]: :_default' \
'--reserved=[]: :_default' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sample)
_arguments "${_arguments_options[@]}" : \
'--cluster=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--max-size=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rng=[]: :_default' \
'--seed=[]: :_default' \
'--sketch-in=[]: :_default' \
'--sketch-out=[]: :_default' \
'--stratified=[]: :_default' \
'--systematic=[]: :_default' \
'--timeout=[]: :_default' \
'--timeseries=[]: :_default' \
'--ts-adaptive=[]: :_default' \
'--ts-aggregate=[]: :_default' \
'--ts-input-tz=[]: :_default' \
'--ts-interval=[]: :_default' \
'--ts-start=[]: :_default' \
'--user-agent=[]: :_default' \
'--varopt=[]: :_default' \
'--weighted=[]: :_default' \
'--bernoulli[]' \
'--force[]' \
'--mergeable-reservoir[]' \
'-n[]' \
'--no-headers[]' \
'--ts-prefer-dmy[]' \
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
'--force[]' \
'-i[]' \
'--ignore-case[]' \
'--memcheck[]' \
'-n[]' \
'--no-headers[]' \
'--polars[]' \
'--prefer-dmy[]' \
'--stdout[]' \
'--strict-dates[]' \
'--strict-formats[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(scoresql)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--duckdb[]' \
'--ignore-errors[]' \
'--json[]' \
'-q[]' \
'--quiet[]' \
'--truncate-ragged-lines[]' \
'--try-parsedates[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(search)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-f+[]: :_default' \
'--flag=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--preview-match=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--size-limit=[]: :_default' \
'-c[]' \
'--count[]' \
'--exact[]' \
'-i[]' \
'--ignore-case[]' \
'-v[]' \
'--invert-match[]' \
'--json[]' \
'--literal[]' \
'-n[]' \
'--no-headers[]' \
'--not-one[]' \
'-p[]' \
'--progressbar[]' \
'-Q[]' \
'--quick[]' \
'-q[]' \
'--quiet[]' \
'-u[]' \
'--unicode[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(searchset)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'-f+[]: :_default' \
'--flag=[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--size-limit=[]: :_default' \
'--unmatched-output=[]: :_default' \
'-c[]' \
'--count[]' \
'--exact[]' \
'--flag-matches-only[]' \
'-i[]' \
'--ignore-case[]' \
'-v[]' \
'--invert-match[]' \
'-j[]' \
'--json[]' \
'--literal[]' \
'-n[]' \
'--no-headers[]' \
'--not-one[]' \
'-p[]' \
'--progressbar[]' \
'-Q[]' \
'--quick[]' \
'-q[]' \
'--quiet[]' \
'-u[]' \
'--unicode[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(select)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--seed=[]: :_default' \
'-n[]' \
'--no-headers[]' \
'-R[]' \
'--random[]' \
'-S[]' \
'--sort[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(slice)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-e+[]: :_default' \
'--end=[]: :_default' \
'-i+[]: :_default' \
'--index=[]: :_default' \
'-l+[]: :_default' \
'--len=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-s+[]: :_default' \
'--start=[]: :_default' \
'--invert[]' \
'--json[]' \
'-n[]' \
'--no-headers[]' \
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
'--timeout=[]: :_default' \
'--user-agent=[]: :_default' \
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
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
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
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
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
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
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
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--timeout=[]: :_default' \
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
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--quote=[]: :_default' \
'--sample=[]: :_default' \
'--save-urlsample=[]: :_default' \
'--timeout=[]: :_default' \
'--user-agent=[]: :_default' \
'--harvest-mode[]' \
'--json[]' \
'--just-mime[]' \
'--no-infer[]' \
'--prefer-dmy[]' \
'--pretty-json[]' \
'-p[]' \
'--progressbar[]' \
'-Q[]' \
'--quick[]' \
'--stats-types[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sort)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rng=[]: :_default' \
'--seed=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--faster[]' \
'-i[]' \
'--ignore-case[]' \
'--memcheck[]' \
'--natural[]' \
'-n[]' \
'--no-headers[]' \
'-N[]' \
'--numeric[]' \
'--random[]' \
'-R[]' \
'--reverse[]' \
'-u[]' \
'--unique[]' \
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
'-i[]' \
'--ignore-case[]' \
'--json[]' \
'--natural[]' \
'-n[]' \
'--no-headers[]' \
'-N[]' \
'--numeric[]' \
'--pretty-json[]' \
'-p[]' \
'--progressbar[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(split)
_arguments "${_arguments_options[@]}" : \
'-c+[]: :_default' \
'--chunks=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--filename=[]: :_default' \
'--filter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-k+[]: :_default' \
'--kb-size=[]: :_default' \
'--pad=[]: :_default' \
'-s+[]: :_default' \
'--size=[]: :_default' \
'--filter-cleanup[]' \
'--filter-ignore-errors[]' \
'-n[]' \
'--no-headers[]' \
'-q[]' \
'--quiet[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sqlp)
_arguments "${_arguments_options[@]}" : \
'--compress-level=[]: :_default' \
'--compression=[]: :_default' \
'--date-format=[]: :_default' \
'--datetime-format=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--float-precision=[]: :_default' \
'--format=[]: :_default' \
'--infer-len=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--rnull-values=[]: :_default' \
'--time-format=[]: :_default' \
'--wnull-value=[]: :_default' \
'--cache-schema[]' \
'--decimal-comma[]' \
'--ignore-errors[]' \
'--low-memory[]' \
'--no-optimizations[]' \
'-q[]' \
'--quiet[]' \
'--statistics[]' \
'--streaming[]' \
'--truncate-ragged-lines[]' \
'--try-parsedates[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stats)
_arguments "${_arguments_options[@]}" : \
'--boolean-patterns=[]: :_default' \
'-c+[]: :_default' \
'--cache-threshold=[]: :_default' \
'--cardinality-method=[]: :_default' \
'--dates-whitelist=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--mode-cardinality-cap=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--percentile-list=[]: :_default' \
'--quantile-method=[]: :_default' \
'--round=[]: :_default' \
'-s+[]: :_default' \
'--select=[]: :_default' \
'--weight=[]: :_default' \
'--cardinality[]' \
'-E[]' \
'--everything[]' \
'--force[]' \
'--infer-boolean[]' \
'--infer-dates[]' \
'--mad[]' \
'--median[]' \
'--memcheck[]' \
'--mode[]' \
'-n[]' \
'--no-headers[]' \
'--nulls[]' \
'--percentiles[]' \
'--prefer-dmy[]' \
'--quartiles[]' \
'--stats-jsonl[]' \
'--typesonly[]' \
'--vis-whitespace[]' \
'--zero-padded-numeric[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(synthesize)
_arguments "${_arguments_options[@]}" : \
'--correlation-threshold=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--dictionary=[]: :_default' \
'--freq-limit=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--joint-cardinality-cap=[]: :_default' \
'--locale=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-n+[]: :_default' \
'--rows=[]: :_default' \
'--seed=[]: :_default' \
'--stats-options=[]: :_default' \
'--consistent-fakes[]' \
'--infer-content-type[]' \
'--no-relationships[]' \
'--strict-relationships[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(table)
_arguments "${_arguments_options[@]}" : \
'-a+[]: :_default' \
'--align=[]: :_default' \
'-c+[]: :_default' \
'--condense=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'-p+[]: :_default' \
'--pad=[]: :_default' \
'-w+[]: :_default' \
'--width=[]: :_default' \
'--memcheck[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(template)
_arguments "${_arguments_options[@]}" : \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'--customfilter-error=[]: :_default' \
'--delimiter=[]: :_default' \
'-J+[]: :_default' \
'--globals-json=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--outfilename=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--outsubdir-size=[]: :_default' \
'--template=[]: :_default' \
'-t+[]: :_default' \
'--template-file=[]: :_default' \
'--timeout=[]: :_default' \
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
'--compress-level=[]: :_default' \
'--compression=[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'-A[]' \
'--all-strings[]' \
'-d[]' \
'--drop[]' \
'-u[]' \
'--dump[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-q[]' \
'--quiet[]' \
'-a[]' \
'--stats[]' \
'--try-parse-dates[]' \
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
'--compress-level=[]: :_default' \
'--compression=[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'-A[]' \
'--all-strings[]' \
'-d[]' \
'--drop[]' \
'-u[]' \
'--dump[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-q[]' \
'--quiet[]' \
'-a[]' \
'--stats[]' \
'--try-parse-dates[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(ods)
_arguments "${_arguments_options[@]}" : \
'--compress-level=[]: :_default' \
'--compression=[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'-A[]' \
'--all-strings[]' \
'-d[]' \
'--drop[]' \
'-u[]' \
'--dump[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-q[]' \
'--quiet[]' \
'-a[]' \
'--stats[]' \
'--try-parse-dates[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(parquet)
_arguments "${_arguments_options[@]}" : \
'--compress-level=[]: :_default' \
'--compression=[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'-A[]' \
'--all-strings[]' \
'-d[]' \
'--drop[]' \
'-u[]' \
'--dump[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-q[]' \
'--quiet[]' \
'-a[]' \
'--stats[]' \
'--try-parse-dates[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(postgres)
_arguments "${_arguments_options[@]}" : \
'--compress-level=[]: :_default' \
'--compression=[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'-A[]' \
'--all-strings[]' \
'-d[]' \
'--drop[]' \
'-u[]' \
'--dump[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-q[]' \
'--quiet[]' \
'-a[]' \
'--stats[]' \
'--try-parse-dates[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sqlite)
_arguments "${_arguments_options[@]}" : \
'--compress-level=[]: :_default' \
'--compression=[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'-A[]' \
'--all-strings[]' \
'-d[]' \
'--drop[]' \
'-u[]' \
'--dump[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-q[]' \
'--quiet[]' \
'-a[]' \
'--stats[]' \
'--try-parse-dates[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(xlsx)
_arguments "${_arguments_options[@]}" : \
'--compress-level=[]: :_default' \
'--compression=[]: :_default' \
'--delimiter=[]: :_default' \
'--infer-len=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-s+[]: :_default' \
'--schema=[]: :_default' \
'-p+[]: :_default' \
'--separator=[]: :_default' \
'-c+[]: :_default' \
'--stats-csv=[]: :_default' \
'-t+[]: :_default' \
'--table=[]: :_default' \
'-A[]' \
'--all-strings[]' \
'-d[]' \
'--drop[]' \
'-u[]' \
'--dump[]' \
'-e[]' \
'--evolve[]' \
'-i[]' \
'--pipe[]' \
'-k[]' \
'--print-package[]' \
'-q[]' \
'--quiet[]' \
'-a[]' \
'--stats[]' \
'--try-parse-dates[]' \
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
'-b+[]: :_default' \
'--batch=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'-o+[]: :_default' \
'--output=[]: :_default' \
'--memcheck[]' \
'--no-boolean[]' \
'-q[]' \
'--quiet[]' \
'--trim[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(transpose)
_arguments "${_arguments_options[@]}" : \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--long=[]: :_default' \
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
'--backtrack-limit=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'--email-min-subdomains=[]: :_default' \
'--invalid=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--size-limit=[]: :_default' \
'--timeout=[]: :_default' \
'--valid=[]: :_default' \
'--valid-output=[]: :_default' \
'--email-display-text[]' \
'--email-domain-literal[]' \
'--email-required-tld[]' \
'--fail-fast[]' \
'--fancy-regex[]' \
'--json[]' \
'--no-format-validation[]' \
'-n[]' \
'--no-headers[]' \
'--pretty-json[]' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
'--trim[]' \
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
'--backtrack-limit=[]: :_default' \
'-b+[]: :_default' \
'--batch=[]: :_default' \
'--cache-dir=[]: :_default' \
'--ckan-api=[]: :_default' \
'--ckan-token=[]: :_default' \
'-d+[]: :_default' \
'--delimiter=[]: :_default' \
'--dfa-size-limit=[]: :_default' \
'--email-min-subdomains=[]: :_default' \
'--invalid=[]: :_default' \
'-j+[]: :_default' \
'--jobs=[]: :_default' \
'--size-limit=[]: :_default' \
'--timeout=[]: :_default' \
'--valid=[]: :_default' \
'--valid-output=[]: :_default' \
'--email-display-text[]' \
'--email-domain-literal[]' \
'--email-required-tld[]' \
'--fail-fast[]' \
'--fancy-regex[]' \
'--json[]' \
'--no-format-validation[]' \
'-n[]' \
'--no-headers[]' \
'--pretty-json[]' \
'-p[]' \
'--progressbar[]' \
'-q[]' \
'--quiet[]' \
'--trim[]' \
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
            (cache-clear)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-prune)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
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
(opencage)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(opencagenow)
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
(get)
_arguments "${_arguments_options[@]}" : \
":: :_qsv__subcmd__help__subcmd__get_commands" \
"*::: :->get" \
&& ret=0

    case $state in
    (get)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:qsv-help-get-command-$line[1]:"
        case $line[1] in
            (cache-clear)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-prune)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-set-policy)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cache-set-ttl)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(headers)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(implode)
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
(profile)
_arguments "${_arguments_options[@]}" : \
&& ret=0
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
(synthesize)
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
'get:' \
'headers:' \
'implode:' \
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
'profile:' \
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
'synthesize:' \
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
'cache-clear:' \
'cache-info:' \
'cache-prune:' \
'countryinfo:' \
'countryinfonow:' \
'index-check:' \
'index-load:' \
'index-reset:' \
'index-update:' \
'iplookup:' \
'iplookupnow:' \
'opencage:' \
'opencagenow:' \
'reverse:' \
'reversenow:' \
'suggest:' \
'suggestnow:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv geocode commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__cache-clear_commands] )) ||
_qsv__subcmd__geocode__subcmd__cache-clear_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode cache-clear commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__cache-info_commands] )) ||
_qsv__subcmd__geocode__subcmd__cache-info_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode cache-info commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__cache-prune_commands] )) ||
_qsv__subcmd__geocode__subcmd__cache-prune_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode cache-prune commands' commands "$@"
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
'cache-clear:' \
'cache-info:' \
'cache-prune:' \
'countryinfo:' \
'countryinfonow:' \
'index-check:' \
'index-load:' \
'index-reset:' \
'index-update:' \
'iplookup:' \
'iplookupnow:' \
'opencage:' \
'opencagenow:' \
'reverse:' \
'reversenow:' \
'suggest:' \
'suggestnow:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv geocode help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__cache-clear_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__cache-clear_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help cache-clear commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__cache-info_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__cache-info_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help cache-info commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__cache-prune_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__cache-prune_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help cache-prune commands' commands "$@"
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
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__opencage_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__opencage_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help opencage commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__help__subcmd__opencagenow_commands] )) ||
_qsv__subcmd__geocode__subcmd__help__subcmd__opencagenow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode help opencagenow commands' commands "$@"
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
(( $+functions[_qsv__subcmd__geocode__subcmd__opencage_commands] )) ||
_qsv__subcmd__geocode__subcmd__opencage_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode opencage commands' commands "$@"
}
(( $+functions[_qsv__subcmd__geocode__subcmd__opencagenow_commands] )) ||
_qsv__subcmd__geocode__subcmd__opencagenow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv geocode opencagenow commands' commands "$@"
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
(( $+functions[_qsv__subcmd__get_commands] )) ||
_qsv__subcmd__get_commands() {
    local commands; commands=(
'cache-clear:' \
'cache-info:' \
'cache-list:' \
'cache-prune:' \
'cache-set-policy:' \
'cache-set-ttl:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv get commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__cache-clear_commands] )) ||
_qsv__subcmd__get__subcmd__cache-clear_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get cache-clear commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__cache-info_commands] )) ||
_qsv__subcmd__get__subcmd__cache-info_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get cache-info commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__cache-list_commands] )) ||
_qsv__subcmd__get__subcmd__cache-list_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get cache-list commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__cache-prune_commands] )) ||
_qsv__subcmd__get__subcmd__cache-prune_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get cache-prune commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__cache-set-policy_commands] )) ||
_qsv__subcmd__get__subcmd__cache-set-policy_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get cache-set-policy commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__cache-set-ttl_commands] )) ||
_qsv__subcmd__get__subcmd__cache-set-ttl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get cache-set-ttl commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__help_commands] )) ||
_qsv__subcmd__get__subcmd__help_commands() {
    local commands; commands=(
'cache-clear:' \
'cache-info:' \
'cache-list:' \
'cache-prune:' \
'cache-set-policy:' \
'cache-set-ttl:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'qsv get help commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__help__subcmd__cache-clear_commands] )) ||
_qsv__subcmd__get__subcmd__help__subcmd__cache-clear_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get help cache-clear commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__help__subcmd__cache-info_commands] )) ||
_qsv__subcmd__get__subcmd__help__subcmd__cache-info_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get help cache-info commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__help__subcmd__cache-list_commands] )) ||
_qsv__subcmd__get__subcmd__help__subcmd__cache-list_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get help cache-list commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__help__subcmd__cache-prune_commands] )) ||
_qsv__subcmd__get__subcmd__help__subcmd__cache-prune_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get help cache-prune commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__help__subcmd__cache-set-policy_commands] )) ||
_qsv__subcmd__get__subcmd__help__subcmd__cache-set-policy_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get help cache-set-policy commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__help__subcmd__cache-set-ttl_commands] )) ||
_qsv__subcmd__get__subcmd__help__subcmd__cache-set-ttl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get help cache-set-ttl commands' commands "$@"
}
(( $+functions[_qsv__subcmd__get__subcmd__help__subcmd__help_commands] )) ||
_qsv__subcmd__get__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'qsv get help help commands' commands "$@"
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
'get:' \
'headers:' \
'implode:' \
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
'profile:' \
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
'synthesize:' \
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
'cache-clear:' \
'cache-info:' \
'cache-prune:' \
'countryinfo:' \
'countryinfonow:' \
'index-check:' \
'index-load:' \
'index-reset:' \
'index-update:' \
'iplookup:' \
'iplookupnow:' \
'opencage:' \
'opencagenow:' \
'reverse:' \
'reversenow:' \
'suggest:' \
'suggestnow:' \
    )
    _describe -t commands 'qsv help geocode commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__cache-clear_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__cache-clear_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode cache-clear commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__cache-info_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__cache-info_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode cache-info commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__cache-prune_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__cache-prune_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode cache-prune commands' commands "$@"
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
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__opencage_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__opencage_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode opencage commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__geocode__subcmd__opencagenow_commands] )) ||
_qsv__subcmd__help__subcmd__geocode__subcmd__opencagenow_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help geocode opencagenow commands' commands "$@"
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
(( $+functions[_qsv__subcmd__help__subcmd__get_commands] )) ||
_qsv__subcmd__help__subcmd__get_commands() {
    local commands; commands=(
'cache-clear:' \
'cache-info:' \
'cache-list:' \
'cache-prune:' \
'cache-set-policy:' \
'cache-set-ttl:' \
    )
    _describe -t commands 'qsv help get commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__get__subcmd__cache-clear_commands] )) ||
_qsv__subcmd__help__subcmd__get__subcmd__cache-clear_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help get cache-clear commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__get__subcmd__cache-info_commands] )) ||
_qsv__subcmd__help__subcmd__get__subcmd__cache-info_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help get cache-info commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__get__subcmd__cache-list_commands] )) ||
_qsv__subcmd__help__subcmd__get__subcmd__cache-list_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help get cache-list commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__get__subcmd__cache-prune_commands] )) ||
_qsv__subcmd__help__subcmd__get__subcmd__cache-prune_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help get cache-prune commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__get__subcmd__cache-set-policy_commands] )) ||
_qsv__subcmd__help__subcmd__get__subcmd__cache-set-policy_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help get cache-set-policy commands' commands "$@"
}
(( $+functions[_qsv__subcmd__help__subcmd__get__subcmd__cache-set-ttl_commands] )) ||
_qsv__subcmd__help__subcmd__get__subcmd__cache-set-ttl_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help get cache-set-ttl commands' commands "$@"
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
(( $+functions[_qsv__subcmd__help__subcmd__implode_commands] )) ||
_qsv__subcmd__help__subcmd__implode_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help implode commands' commands "$@"
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
(( $+functions[_qsv__subcmd__help__subcmd__profile_commands] )) ||
_qsv__subcmd__help__subcmd__profile_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help profile commands' commands "$@"
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
(( $+functions[_qsv__subcmd__help__subcmd__synthesize_commands] )) ||
_qsv__subcmd__help__subcmd__synthesize_commands() {
    local commands; commands=()
    _describe -t commands 'qsv help synthesize commands' commands "$@"
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
(( $+functions[_qsv__subcmd__implode_commands] )) ||
_qsv__subcmd__implode_commands() {
    local commands; commands=()
    _describe -t commands 'qsv implode commands' commands "$@"
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
(( $+functions[_qsv__subcmd__profile_commands] )) ||
_qsv__subcmd__profile_commands() {
    local commands; commands=()
    _describe -t commands 'qsv profile commands' commands "$@"
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
(( $+functions[_qsv__subcmd__synthesize_commands] )) ||
_qsv__subcmd__synthesize_commands() {
    local commands; commands=()
    _describe -t commands 'qsv synthesize commands' commands "$@"
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
