# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_qsv_global_optspecs
	string join \n list envlist update updatenow V/version h/help
end

function __fish_qsv_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_qsv_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_qsv_using_subcommand
	set -l cmd (__fish_qsv_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c qsv -n "__fish_qsv_needs_command" -l list
complete -c qsv -n "__fish_qsv_needs_command" -l envlist
complete -c qsv -n "__fish_qsv_needs_command" -l update
complete -c qsv -n "__fish_qsv_needs_command" -l updatenow
complete -c qsv -n "__fish_qsv_needs_command" -s V -l version
complete -c qsv -n "__fish_qsv_needs_command" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_needs_command" -f -a "apply"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "behead"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "blake3"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "cat"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "clean"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "clipboard"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "color"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "count"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "datefmt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "dedup"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "denull"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "describegpt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "diff"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "edit"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "enum"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "excel"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "exclude"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "explode"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "extdedup"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "extsort"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fetch"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fetchpost"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fill"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fixedwidth"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fixlengths"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "flatten"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fmt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "foreach"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "frequency"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "geocode"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "geoconvert"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "get"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "headers"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "implode"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "index"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "input"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "join"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "joinp"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "json"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "jsonl"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "lens"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "log"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "luau"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "moarstats"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "partition"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "pivotp"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "pragmastat"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "pro"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "profile"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "prompt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "pseudo"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "py"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "readstat"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "rename"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "replace"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "reverse"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "safenames"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sample"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "schema"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "scoresql"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "search"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "searchset"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "select"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "slice"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "snappy"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sniff"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sort"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sortcheck"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "split"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sqlp"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "stats"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "synthesize"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "table"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "template"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "to"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "tojsonl"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "transpose"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "validate"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "viz"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l addl-props -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s k -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s u -l base-url -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s t -l max-tokens -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s m -l model -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l on-error -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l prompt-file -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l fresh
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -f -a "calcconv"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -f -a "dynfmt"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -f -a "emptyreplace"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -f -a "operations"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -f -a "summarize"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations summarize help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l addl-props -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s k -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s u -l base-url -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s t -l max-tokens -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s m -l model -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l on-error -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l prompt-file -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l fresh
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l addl-props -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s k -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s u -l base-url -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s t -l max-tokens -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s m -l model -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l on-error -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l prompt-file -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l fresh
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l addl-props -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s k -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s u -l base-url -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s t -l max-tokens -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s m -l model -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l on-error -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l prompt-file -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l fresh
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l addl-props -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s k -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s u -l base-url -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s t -l max-tokens -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s m -l model -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l on-error -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l prompt-file -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l fresh
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l addl-props -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s k -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s u -l base-url -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s t -l max-tokens -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s m -l model -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l on-error -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l prompt-file -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l fresh
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from summarize" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "calcconv"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "dynfmt"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "emptyreplace"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "operations"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "summarize"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand behead" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand behead" -s f -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand behead" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -l derive-key -r
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -s l -l length -r
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -s c -l check
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -l keyed
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -l no-mmap
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -l no-names
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -l raw
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -l tag
complete -c qsv -n "__fish_qsv_using_subcommand blake3" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s g -l group -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s N -l group-name -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s p -l pad
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -f -a "columns"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -f -a "rows"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -f -a "rowskey"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s g -l group -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s N -l group-name -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s p -l pad
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s g -l group -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s N -l group-name -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s p -l pad
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s g -l group -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s N -l group-name -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s p -l pad
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "columns"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "rows"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "rowskey"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand clean" -l all
complete -c qsv -n "__fish_qsv_using_subcommand clean" -s n -l dry-run
complete -c qsv -n "__fish_qsv_using_subcommand clean" -s f -l force
complete -c qsv -n "__fish_qsv_using_subcommand clean" -l frequency
complete -c qsv -n "__fish_qsv_using_subcommand clean" -l index
complete -c qsv -n "__fish_qsv_using_subcommand clean" -l moarstats
complete -c qsv -n "__fish_qsv_using_subcommand clean" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand clean" -s r -l recursive
complete -c qsv -n "__fish_qsv_using_subcommand clean" -l schema
complete -c qsv -n "__fish_qsv_using_subcommand clean" -l stale
complete -c qsv -n "__fish_qsv_using_subcommand clean" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand clean" -l validate
complete -c qsv -n "__fish_qsv_using_subcommand clean" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand clipboard" -s s -l save
complete -c qsv -n "__fish_qsv_using_subcommand clipboard" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand color" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand color" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand color" -s t -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand color" -s C -l color
complete -c qsv -n "__fish_qsv_using_subcommand color" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand color" -s n -l row-numbers
complete -c qsv -n "__fish_qsv_using_subcommand color" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand count" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand count" -s f -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand count" -s H -l human-readable
complete -c qsv -n "__fish_qsv_using_subcommand count" -l json
complete -c qsv -n "__fish_qsv_using_subcommand count" -l low-memory
complete -c qsv -n "__fish_qsv_using_subcommand count" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand count" -l no-polars
complete -c qsv -n "__fish_qsv_using_subcommand count" -l width
complete -c qsv -n "__fish_qsv_using_subcommand count" -l width-no-delims
complete -c qsv -n "__fish_qsv_using_subcommand count" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l default-tz -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l input-tz -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l output-tz -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s R -l ts-resolution -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l keep-zero-time
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l utc
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l zulu
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s D -l dupes-output -r
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s H -l human-readable
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s N -l numeric
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l sorted
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand denull" -l add-vocab -r
complete -c qsv -n "__fish_qsv_using_subcommand denull" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand denull" -l max-distinct -r
complete -c qsv -n "__fish_qsv_using_subcommand denull" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand denull" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand denull" -l vocab -r
complete -c qsv -n "__fish_qsv_using_subcommand denull" -l all-columns
complete -c qsv -n "__fish_qsv_using_subcommand denull" -l apply
complete -c qsv -n "__fish_qsv_using_subcommand denull" -l json
complete -c qsv -n "__fish_qsv_using_subcommand denull" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand denull" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l addl-cols-list -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l addl-props -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s k -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s u -l base-url -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l context-file -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l disk-cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l ds-license -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l ds-source -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l ds-updated -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l enum-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l export-prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l format -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l freq-options -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l markdown-template -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s t -l max-tokens -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s m -l model -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l num-examples -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l num-tags -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l okf-type -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s p -l prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l prompt-file -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l sample-size -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l score-max-retries -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l score-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l session -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l session-len -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l sql-results -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l stats-options -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l tag-vocab -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l truncate-str -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l addl-cols
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s A -l all
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l allow-extra-cols
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l description
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l dictionary
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l fewshot-examples
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l flush-cache
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l forget
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l fresh
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l infer-content-type
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l infer-null-values
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l no-score-sql
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l prepare-context
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l process-response
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l redis-cache
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l strict-dates
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l tags
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l two-pass
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l delimiter-left -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l delimiter-output -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l delimiter-right -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s k -l key -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l sort-columns -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l drop-equal-columns
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l drop-equal-fields
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l no-headers-left
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l no-headers-output
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l no-headers-right
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand edit" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand edit" -s i -l in-place
complete -c qsv -n "__fish_qsv_using_subcommand edit" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand edit" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l constant -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l copy -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l hash -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l increment -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l start -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l uuid4
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l uuid7
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l cell -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l date-format -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l error-format -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l header-row -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l metadata -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l range -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s s -l sheet -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l table -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l keep-zero-time
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s v -l invert
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s D -l dupes-output -r
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l memory-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l temp-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s H -l human-readable
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l no-output
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l memory-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l tmp-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s R -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l disk-cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s H -l http-header -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l jaq -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l jaqfile -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l max-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l max-retries -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l mem-cache-size -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l report -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l url-template -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l cache-error
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l cookies
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l disk-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l flush-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l pretty
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l redis-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l store-error
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l content-type -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l disk-cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s j -l globals-json -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s H -l http-header -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l jaq -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l jaqfile -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l max-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l max-retries -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l mem-cache-size -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s t -l payload-tpl -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l report -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l cache-error
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l compress
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l cookies
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l disk-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l flush-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l pretty
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l redis-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l store-error
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s v -l default -r
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s g -l groupby -r
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s b -l backfill
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s f -l first
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fixedwidth" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fixedwidth" -l positions -r
complete -c qsv -n "__fish_qsv_using_subcommand fixedwidth" -l widths -r
complete -c qsv -n "__fish_qsv_using_subcommand fixedwidth" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -l escape -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s i -l insert -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s l -l length -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -l quote -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s r -l remove-empty
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s c -l condense -r
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s f -l field-separator -r
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s s -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l escape -r
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -s t -l out-delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l quote -r
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l ascii
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l crlf
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l no-final-newline
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l quote-always
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l quote-never
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -l dry-run -r
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s u -l unify
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l all-unique-text -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l high-card-pct -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l high-card-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s l -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l lmt-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-float -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l null-text -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l other-text -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l pct-dec-places -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s r -l rank-strategy -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l sketch-map-size -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l sketch-method -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l stats-filter -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s u -l unq-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l weight -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s a -l asc
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l force
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l frequency-jsonl
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l json
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-stats
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-trim
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l null-sorted
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l other-sorted
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l pct-nulls
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l toon
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l vis-whitespace
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "cache-clear"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "cache-info"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "cache-prune"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "countryinfo"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "countryinfonow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "index-check"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "index-load"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "index-reset"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "index-update"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "iplookup"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "iplookupnow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "opencage"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "opencagenow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "reversenow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "suggest"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "suggestnow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from cache-clear cache-info cache-prune countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow opencage opencagenow reverse reversenow suggest suggestnow help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-clear" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-info" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from cache-prune" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencage" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from opencagenow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l cache-ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l no-annotations
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "cache-clear"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "cache-info"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "cache-prune"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "countryinfo"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "countryinfonow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-check"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-load"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-reset"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-update"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "iplookup"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "iplookupnow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "opencage"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "opencagenow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "reversenow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "suggest"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "suggestnow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s g -l geometry -r
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s y -l latitude -r
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s x -l longitude -r
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s l -l max-length -r
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l cloud-opt -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l compress -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l name -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l offset -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l refresh -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l sample -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l force
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l json
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l random
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -l verify
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -f -a "cache-clear"
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -f -a "cache-fetch"
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -f -a "cache-info"
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -f -a "cache-list"
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -f -a "cache-prune"
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -f -a "cache-set-policy"
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -f -a "cache-set-ttl"
complete -c qsv -n "__fish_qsv_using_subcommand get; and not __fish_seen_subcommand_from cache-clear cache-fetch cache-info cache-list cache-prune cache-set-policy cache-set-ttl help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l cloud-opt -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l compress -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l name -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l offset -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l refresh -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l sample -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l force
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l json
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l random
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -l verify
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-clear" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l cloud-opt -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l compress -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l name -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l offset -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l refresh -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l sample -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l force
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l json
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l random
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -l verify
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-fetch" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l cloud-opt -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l compress -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l name -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l offset -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l refresh -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l sample -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l force
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l json
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l random
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -l verify
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-info" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l cloud-opt -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l compress -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l name -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l offset -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l refresh -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l sample -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l force
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l json
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l random
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -l verify
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-list" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l cloud-opt -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l compress -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l name -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l offset -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l refresh -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l sample -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l force
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l json
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l random
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -l verify
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-prune" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l cloud-opt -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l compress -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l name -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l offset -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l refresh -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l sample -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l force
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l json
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l random
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -l verify
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-policy" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l cloud-opt -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l compress -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l name -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l offset -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l older-than -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l refresh -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l sample -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l ttl -r
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l force
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l json
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l random
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -l verify
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from cache-set-ttl" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from help" -f -a "cache-clear"
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from help" -f -a "cache-fetch"
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from help" -f -a "cache-info"
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from help" -f -a "cache-list"
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from help" -f -a "cache-prune"
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from help" -f -a "cache-set-policy"
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from help" -f -a "cache-set-ttl"
complete -c qsv -n "__fish_qsv_using_subcommand get; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand headers" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand headers" -s J -l just-count
complete -c qsv -n "__fish_qsv_using_subcommand headers" -s j -l just-names
complete -c qsv -n "__fish_qsv_using_subcommand headers" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand headers" -l union
complete -c qsv -n "__fish_qsv_using_subcommand headers" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand implode" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand implode" -s k -l keys -r
complete -c qsv -n "__fish_qsv_using_subcommand implode" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand implode" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand implode" -s v -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand implode" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand implode" -l skip-empty
complete -c qsv -n "__fish_qsv_using_subcommand implode" -l sorted
complete -c qsv -n "__fish_qsv_using_subcommand implode" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand index" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand index" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand input" -l comment -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l encoding-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l escape -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l quote -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l quote-style -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l skip-lastlines -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l skip-lines -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l auto-skip
complete -c qsv -n "__fish_qsv_using_subcommand input" -l no-quoting
complete -c qsv -n "__fish_qsv_using_subcommand input" -l trim-fields
complete -c qsv -n "__fish_qsv_using_subcommand input" -l trim-headers
complete -c qsv -n "__fish_qsv_using_subcommand input" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand join" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand join" -l keys-output -r
complete -c qsv -n "__fish_qsv_using_subcommand join" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand join" -l cross
complete -c qsv -n "__fish_qsv_using_subcommand join" -l full
complete -c qsv -n "__fish_qsv_using_subcommand join" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand join" -s z -l ignore-leading-zeros
complete -c qsv -n "__fish_qsv_using_subcommand join" -l left
complete -c qsv -n "__fish_qsv_using_subcommand join" -l left-anti
complete -c qsv -n "__fish_qsv_using_subcommand join" -l left-semi
complete -c qsv -n "__fish_qsv_using_subcommand join" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand join" -l nulls
complete -c qsv -n "__fish_qsv_using_subcommand join" -l right
complete -c qsv -n "__fish_qsv_using_subcommand join" -l right-anti
complete -c qsv -n "__fish_qsv_using_subcommand join" -l right-semi
complete -c qsv -n "__fish_qsv_using_subcommand join" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l cache-schema -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l date-format -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l datetime-format -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l filter-left -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l filter-right -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l float-precision -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left_by -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l maintain-order -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l non-equi -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s N -l norm-unicode -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l null-value -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l right_by -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l sql-filter -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l strategy -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l time-format -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l tolerance -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l validate -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s X -l allow-exact-matches
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l asof
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l coalesce
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l cross
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l decimal-comma
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l full
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s z -l ignore-leading-zeros
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left-anti
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left-semi
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l low-memory
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l no-optimizations
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l no-sort
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l nulls
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l right
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l right-anti
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l right-semi
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l streaming
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l try-parsedates
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand json" -l jaq -r
complete -c qsv -n "__fish_qsv_using_subcommand json" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand json" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand json" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l columns -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l echo-column -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l filter -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l find -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s f -l freeze-columns -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s P -l prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s W -l wrap-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s A -l auto-reload
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l debug
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s m -l monochrome
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s S -l streaming-stdin
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s t -l tab-separated
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand log" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s B -l begin -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s E -l end -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l max-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l colindex
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s g -l no-globals
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s r -l remap
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s B -l begin -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s E -l end -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l max-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l colindex
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s g -l no-globals
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s r -l remap
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s B -l begin -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s E -l end -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l max-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l colindex
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s g -l no-globals
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s r -l remap
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l bivariate-batch -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s S -l bivariate-stats -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s C -l cardinality-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s e -l epsilon -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s J -l join-inputs -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s K -l join-keys -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s T -l join-type -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l pct-thresholds -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l round -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l stats-options -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l xsd-gdate-scan -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l advanced
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s B -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l force
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l use-percentiles
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand partition" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l filename -r
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand partition" -s p -l prefix-length -r
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l drop
complete -c qsv -n "__fish_qsv_using_subcommand partition" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand partition" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s a -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l col-separator -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s i -l index -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l max-columns -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l total-label -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s v -l values -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l decimal-comma
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l grand-total
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l maintain-order
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l sort-columns
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l subtotal
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l try-parsedates
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l validate
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -l compare1 -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -l compare2 -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -s m -l misrate -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -l round -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -l seed -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -l stats-options -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -l subsample -r
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -l force
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -l no-bounds
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -l standalone
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -s t -l twosample
complete -c qsv -n "__fish_qsv_using_subcommand pragmastat" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -f -a "workflow"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from lens" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from workflow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from help" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from help" -f -a "workflow"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l dcat-discovery-timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand profile" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l initial-context -r
complete -c qsv -n "__fish_qsv_using_subcommand profile" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand profile" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l profile -r
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l spec -r
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l allow-external-validator
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l catalog
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l croissant-frequency
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l dcat-legacy-license
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l force
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l no-ckan
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l no-dcat-discovery
complete -c qsv -n "__fish_qsv_using_subcommand profile" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l no-projection
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l strict
complete -c qsv -n "__fish_qsv_using_subcommand profile" -l validate
complete -c qsv -n "__fish_qsv_using_subcommand profile" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l base-delay-ms -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s F -l filters -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s m -l msg -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l save-fname -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s d -l workdir -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s f -l fd-output
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l increment -r
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l start -r
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s f -l helper -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s f -l helper -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s f -l helper -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand readstat" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand readstat" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand readstat" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand readstat" -l metadata -r
complete -c qsv -n "__fish_qsv_using_subcommand readstat" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand readstat" -l value-labels
complete -c qsv -n "__fish_qsv_using_subcommand readstat" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand rename" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand rename" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand rename" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand rename" -l pairwise
complete -c qsv -n "__fish_qsv_using_subcommand rename" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l dfa-size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l exact
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l literal
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l not-one
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s u -l unicode
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l mode -r
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l prefix -r
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l reserved -r
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l collapse
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l unicode
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l max-size -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l rng -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l seed -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l sketch-in -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l sketch-out -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l stratified -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l systematic -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l timeseries -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-adaptive -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-aggregate -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-input-tz -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-interval -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-start -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l varopt -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l weighted -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l bernoulli
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l force
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l mergeable-reservoir
complete -c qsv -n "__fish_qsv_using_subcommand sample" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand sample" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l dates-whitelist -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l enum-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l pattern-columns -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l force
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l polars
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l stdout
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l strict-dates
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l strict-formats
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand scoresql" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand scoresql" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand scoresql" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand scoresql" -l duckdb
complete -c qsv -n "__fish_qsv_using_subcommand scoresql" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand scoresql" -l json
complete -c qsv -n "__fish_qsv_using_subcommand scoresql" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand scoresql" -l truncate-ragged-lines
complete -c qsv -n "__fish_qsv_using_subcommand scoresql" -l try-parsedates
complete -c qsv -n "__fish_qsv_using_subcommand scoresql" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand search" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -l dfa-size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -s f -l flag -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -l preview-match -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -l size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -s c -l count
complete -c qsv -n "__fish_qsv_using_subcommand search" -l exact
complete -c qsv -n "__fish_qsv_using_subcommand search" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand search" -s v -l invert-match
complete -c qsv -n "__fish_qsv_using_subcommand search" -l json
complete -c qsv -n "__fish_qsv_using_subcommand search" -l literal
complete -c qsv -n "__fish_qsv_using_subcommand search" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand search" -l not-one
complete -c qsv -n "__fish_qsv_using_subcommand search" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand search" -s Q -l quick
complete -c qsv -n "__fish_qsv_using_subcommand search" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand search" -s u -l unicode
complete -c qsv -n "__fish_qsv_using_subcommand search" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l dfa-size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s f -l flag -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l unmatched-output -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s c -l count
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l exact
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l flag-matches-only
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s v -l invert-match
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s j -l json
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l literal
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l not-one
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s Q -l quick
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s u -l unicode
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand select" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand select" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand select" -l seed -r
complete -c qsv -n "__fish_qsv_using_subcommand select" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand select" -s R -l random
complete -c qsv -n "__fish_qsv_using_subcommand select" -s S -l sort
complete -c qsv -n "__fish_qsv_using_subcommand select" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s e -l end -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s i -l index -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s l -l len -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s s -l start -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l invert
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l json
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -f -a "check"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -f -a "compress"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -f -a "decompress"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -f -a "validate"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from help" -f -a "check"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from help" -f -a "compress"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from help" -f -a "decompress"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from help" -f -a "validate"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l quote -r
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l sample -r
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l save-urlsample -r
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l harvest-mode
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l json
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l just-mime
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l no-infer
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -s Q -l quick
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l stats-types
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l rng -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l seed -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l faster
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l natural
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s N -l numeric
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l random
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s R -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s u -l unique
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l all
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l json
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l natural
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s N -l numeric
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand split" -s c -l chunks -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -l filename -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -l filter -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -s k -l kb-size -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -l pad -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -s s -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -l filter-cleanup
complete -c qsv -n "__fish_qsv_using_subcommand split" -l filter-ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand split" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand split" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand split" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l compress-level -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l compression -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l date-format -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l datetime-format -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l float-precision -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l format -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l rnull-values -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l time-format -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l wnull-value -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l cache-schema
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l decimal-comma
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l low-memory
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l no-optimizations
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l statistics
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l streaming
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l truncate-ragged-lines
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l try-parsedates
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l boolean-patterns -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s c -l cache-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l cardinality-method -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l dates-whitelist -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l mode-cardinality-cap -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l percentile-list -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l quantile-method -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l round -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l weight -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l cardinality
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s E -l everything
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l force
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l infer-boolean
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l infer-dates
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l jsonl
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l mad
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l median
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l mode
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l nulls
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l percentiles
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l quartiles
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l stats-jsonl
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l typesonly
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l vis-whitespace
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l zero-padded-numeric
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l correlation-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l freq-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l joint-cardinality-cap -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l locale -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -s n -l rows -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l seed -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l stats-options -r
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l consistent-fakes
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l infer-content-type
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l no-relationships
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -l strict-relationships
complete -c qsv -n "__fish_qsv_using_subcommand synthesize" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand table" -s a -l align -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -s c -l condense -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -s p -l pad -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -s w -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand table" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand template" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l customfilter-error -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -s J -l globals-json -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l outfilename -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l outsubdir-size -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l template -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -s t -l template-file -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand template" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand template" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -l compress-level -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -l compression -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s t -l table -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -l try-parse-dates
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -f -a "datapackage"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -f -a "ods"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -f -a "parquet"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -f -a "postgres"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -f -a "sqlite"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -f -a "xlsx"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods parquet postgres sqlite xlsx help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l compress-level -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l compression -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s t -l table -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l try-parse-dates
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -l compress-level -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -l compression -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s t -l table -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -l try-parse-dates
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -l compress-level -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -l compression -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s t -l table -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -l try-parse-dates
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from parquet" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l compress-level -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l compression -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s t -l table -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l try-parse-dates
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l compress-level -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l compression -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s t -l table -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l try-parse-dates
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l compress-level -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l compression -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s t -l table -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l try-parse-dates
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "datapackage"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "ods"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "parquet"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "postgres"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "sqlite"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "xlsx"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l no-boolean
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -l long -r
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s m -l multipass
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l backtrack-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l dfa-size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l email-min-subdomains -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l invalid -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l valid -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l valid-output -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l email-display-text
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l email-domain-literal
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l email-required-tld
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l fail-fast
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l fancy-regex
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l json
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l no-format-validation
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l split-ragged
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -f -a "schema"
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l backtrack-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l dfa-size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l email-min-subdomains -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l invalid -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l valid -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l valid-output -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l email-display-text
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l email-domain-literal
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l email-required-tld
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l fail-fast
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l fancy-regex
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l json
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l no-format-validation
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l split-ragged
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from help" -f -a "schema"
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "bar"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "box"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "candlestick"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "choropleth"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "contour"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "funnel"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "geo"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "heatmap"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "histogram"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "icicle"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "line"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "ohlc"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "parcats"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "pie"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "radar"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "sankey"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "scatter"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "scatter3d"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "smart"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "splom"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "sunburst"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "treemap"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "violin"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and not __fish_seen_subcommand_from bar box candlestick choropleth contour funnel geo heatmap histogram icicle line map ohlc parcats pie radar sankey scatter scatter3d smart splom sunburst treemap violin help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from bar" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from box" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from candlestick" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from choropleth" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from contour" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from funnel" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from geo" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from heatmap" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from histogram" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from icicle" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from line" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from map" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from ohlc" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from parcats" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from pie" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from radar" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sankey" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from scatter3d" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from smart" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from splom" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from sunburst" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from treemap" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l annotation -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l bins -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l box-points -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l close -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l color -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l color-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l dataset-pid -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l denominator -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l denominator-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l denominator-unit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l dictionary -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l dictionary-context -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l feature-id-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l feature-name-key -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l geocode-admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l geocode-country -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l geojson -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l grid-cols -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l heatmap-density -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l height -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l hierarchy-style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l high -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l lat -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l location-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l locations -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l log-scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l lon -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l low -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l max-charts -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l ohlc-open -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l preview-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l projection -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l region-state -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l scale -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l series -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l slider -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l slider-speed -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l snap-max-dist -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l source -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l style -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l target -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l text -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l theme -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l tour-audience -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l tour-steps -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l value -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l violin -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -s x -l x -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l x-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -s y -l y -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l y-range -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l y-title -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -s z -l z -r
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l check-geojson-key
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l density
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l dict-info
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l donut
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l geocode
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l map
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l no-snap
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l open
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l photos
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l rangeslider
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l sankey-value-order
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l slider-cumulative
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -l smarter
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from violin" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "bar"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "box"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "candlestick"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "choropleth"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "contour"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "funnel"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "geo"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "heatmap"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "histogram"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "icicle"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "line"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "ohlc"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "parcats"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "pie"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "radar"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "sankey"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "scatter"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "scatter3d"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "smart"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "splom"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "sunburst"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "treemap"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "violin"
complete -c qsv -n "__fish_qsv_using_subcommand viz; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "apply"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "behead"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "blake3"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "cat"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "clean"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "clipboard"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "color"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "count"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "datefmt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "dedup"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "denull"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "describegpt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "diff"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "edit"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "enum"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "excel"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "exclude"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "explode"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "extdedup"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "extsort"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "fetch"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "fetchpost"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "fill"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "fixedwidth"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "fixlengths"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "flatten"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "fmt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "foreach"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "frequency"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "geocode"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "geoconvert"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "get"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "headers"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "implode"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "index"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "input"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "join"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "joinp"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "json"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "jsonl"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "log"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "luau"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "moarstats"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "partition"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "pivotp"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "pragmastat"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "pro"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "profile"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "prompt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "pseudo"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "py"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "readstat"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "rename"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "replace"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "safenames"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "sample"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "schema"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "scoresql"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "search"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "searchset"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "select"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "slice"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "snappy"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "sniff"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "sort"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "sortcheck"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "split"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "sqlp"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "stats"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "synthesize"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "table"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "template"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "to"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "tojsonl"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "transpose"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "validate"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "viz"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead blake3 cat clean clipboard color count datefmt dedup denull describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixedwidth fixlengths flatten fmt foreach frequency geocode geoconvert get headers implode index input join joinp json jsonl lens log luau moarstats partition pivotp pragmastat pro profile prompt pseudo py readstat rename replace reverse safenames sample schema scoresql search searchset select slice snappy sniff sort sortcheck split sqlp stats synthesize table template to tojsonl transpose validate viz help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "calcconv"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "dynfmt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "emptyreplace"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "operations"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "summarize"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from cat" -f -a "columns"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from cat" -f -a "rows"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from cat" -f -a "rowskey"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "cache-clear"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "cache-info"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "cache-prune"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "countryinfo"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "countryinfonow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-check"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-load"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-reset"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-update"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "iplookup"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "iplookupnow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "opencage"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "opencagenow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "reversenow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "suggest"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "suggestnow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from get" -f -a "cache-clear"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from get" -f -a "cache-fetch"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from get" -f -a "cache-info"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from get" -f -a "cache-list"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from get" -f -a "cache-prune"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from get" -f -a "cache-set-policy"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from get" -f -a "cache-set-ttl"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from luau" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from luau" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from pro" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from pro" -f -a "workflow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from py" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from py" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from snappy" -f -a "check"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from snappy" -f -a "compress"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from snappy" -f -a "decompress"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from snappy" -f -a "validate"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "datapackage"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "ods"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "parquet"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "postgres"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "sqlite"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "xlsx"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from validate" -f -a "schema"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "bar"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "box"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "candlestick"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "choropleth"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "contour"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "funnel"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "geo"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "heatmap"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "histogram"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "icicle"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "line"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "ohlc"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "parcats"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "pie"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "radar"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "sankey"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "scatter"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "scatter3d"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "smart"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "splom"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "sunburst"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "treemap"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from viz" -f -a "violin"
