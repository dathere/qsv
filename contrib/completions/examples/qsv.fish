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
complete -c qsv -n "__fish_qsv_needs_command" -f -a "cat"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "clipboard"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "color"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "count"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "datefmt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "dedup"
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
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fixlengths"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "flatten"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fmt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "foreach"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "frequency"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "geocode"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "geoconvert"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "headers"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "index"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "input"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "join"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "joinp"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "json"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "jsonl"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "lens"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "luau"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "moarstats"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "partition"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "pivotp"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "pro"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "prompt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "pseudo"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "py"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "rename"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "replace"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "reverse"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "safenames"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sample"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "schema"
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
complete -c qsv -n "__fish_qsv_needs_command" -f -a "table"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "template"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "to"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "tojsonl"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "transpose"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "validate"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -f -a "calcconv"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -f -a "dynfmt"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -f -a "emptyreplace"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -f -a "operations"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from calcconv dynfmt emptyreplace operations help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s R -l replacement -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s C -l comparand -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "calcconv"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "dynfmt"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "emptyreplace"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "operations"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand behead" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand behead" -s f -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand behead" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s g -l group -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s N -l group-name -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s p -l pad
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -f -a "columns"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -f -a "rows"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -f -a "rowskey"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from columns rows rowskey help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s g -l group -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s N -l group-name -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s p -l pad
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s g -l group -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s N -l group-name -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s p -l pad
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s g -l group -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s N -l group-name -r
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s p -l pad
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "columns"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "rows"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "rowskey"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
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
complete -c qsv -n "__fish_qsv_using_subcommand count" -l width
complete -c qsv -n "__fish_qsv_using_subcommand count" -s H -l human-readable
complete -c qsv -n "__fish_qsv_using_subcommand count" -l low-memory
complete -c qsv -n "__fish_qsv_using_subcommand count" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand count" -s f -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand count" -l json
complete -c qsv -n "__fish_qsv_using_subcommand count" -l no-polars
complete -c qsv -n "__fish_qsv_using_subcommand count" -l width-no-delims
complete -c qsv -n "__fish_qsv_using_subcommand count" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l output-tz -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s R -l ts-resolution -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l input-tz -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l default-tz -r
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l zulu
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l keep-zero-time
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l utc
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s D -l dupes-output -r
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s N -l numeric
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s H -l human-readable
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l sorted
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l format -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l addl-props -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l num-tags -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s t -l max-tokens -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l session -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s m -l model -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l tag-vocab -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l sql-results -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l prompt-file -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l sample-size -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s p -l prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s u -l base-url -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s k -l api-key -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l export-prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l freq-options -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l session-len -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l enum-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l truncate-str -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l disk-cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l addl-cols-list -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l stats-options -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l num-examples -r
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l flush-cache
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l fresh
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l forget
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l dictionary
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l tags
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l fewshot-examples
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l addl-cols
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l description
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s A -l all
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l redis-cache
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l delimiter-output -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s k -l key -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l sort-columns -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l delimiter-right -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l delimiter-left -r
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l no-headers-right
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l no-headers-left
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l drop-equal-fields
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l no-headers-output
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand edit" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand edit" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand edit" -s i -l in-place
complete -c qsv -n "__fish_qsv_using_subcommand edit" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l start -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l constant -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l copy -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l increment -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l hash -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l uuid7
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l uuid4
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l metadata -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l table -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l cell -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l range -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l header-row -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s s -l sheet -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l date-format -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l error-format -r
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l keep-zero-time
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s D -l dupes-output -r
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l memory-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l temp-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s H -l human-readable
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l no-output
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l memory-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l tmp-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s R -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l max-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l jaq -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l mem-cache-size -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l max-retries -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s H -l http-header -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l disk-cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l jaqfile -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l url-template -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l report -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l disk-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l store-error
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l cookies
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l pretty
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l cache-error
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l flush-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l redis-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l max-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l report -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s j -l globals-json -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l disk-cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l max-retries -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l rate-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l content-type -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l mem-cache-size -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s H -l http-header -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l jaq -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s t -l payload-tpl -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l jaqfile -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l pretty
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l disk-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l redis-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l cookies
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l compress
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l store-error
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l cache-error
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l flush-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s g -l groupby -r
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s v -l default -r
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s b -l backfill
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s f -l first
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s l -l length -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s i -l insert -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -l quote -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -l escape -r
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s r -l remove-empty
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s s -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s f -l field-separator -r
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s c -l condense -r
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -s t -l out-delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l quote -r
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l escape -r
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l crlf
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l quote-never
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l quote-always
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l ascii
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l no-final-newline
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -l dry-run -r
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s u -l unify
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l all-unique-text -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s r -l rank-strategy -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l pct-dec-places -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-float -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l other-text -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l stats-filter -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l null-text -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s l -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s u -l unq-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l lmt-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l weight -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l pct-nulls
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-trim
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l toon
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s a -l asc
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l json
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l vis-whitespace
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l null-sorted
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l other-sorted
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-other
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-stats
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "countryinfo"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "countryinfonow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "index-check"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "index-load"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "index-reset"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "index-update"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "iplookup"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "iplookupnow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "reversenow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "suggest"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "suggestnow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from countryinfo countryinfonow index-check index-load index-reset index-update iplookup iplookupnow reverse reversenow suggest suggestnow help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookup" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from iplookupnow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s k -l k_weight -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s r -l rename -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l cities-url -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l country -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s l -l language -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l languages -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l min-score -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s c -l new-column -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l invalid-result -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s f -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l admin1 -r
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "countryinfo"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "countryinfonow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-check"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-load"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-reset"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-update"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "iplookup"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "iplookupnow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "reversenow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "suggest"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "suggestnow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s l -l max-length -r
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s y -l latitude -r
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s g -l geometry -r
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s x -l longitude -r
complete -c qsv -n "__fish_qsv_using_subcommand geoconvert" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand headers" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand headers" -l intersect
complete -c qsv -n "__fish_qsv_using_subcommand headers" -s J -l just-count
complete -c qsv -n "__fish_qsv_using_subcommand headers" -s j -l just-names
complete -c qsv -n "__fish_qsv_using_subcommand headers" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand headers" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand index" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand index" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand input" -l encoding-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l quote-style -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l skip-lastlines -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l skip-lines -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l comment -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l quote -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l escape -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand input" -l no-quoting
complete -c qsv -n "__fish_qsv_using_subcommand input" -l trim-headers
complete -c qsv -n "__fish_qsv_using_subcommand input" -l trim-fields
complete -c qsv -n "__fish_qsv_using_subcommand input" -l auto-skip
complete -c qsv -n "__fish_qsv_using_subcommand input" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand join" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand join" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand join" -l keys-output -r
complete -c qsv -n "__fish_qsv_using_subcommand join" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand join" -s z -l ignore-leading-zeros
complete -c qsv -n "__fish_qsv_using_subcommand join" -l left-anti
complete -c qsv -n "__fish_qsv_using_subcommand join" -l left
complete -c qsv -n "__fish_qsv_using_subcommand join" -l full
complete -c qsv -n "__fish_qsv_using_subcommand join" -l right-anti
complete -c qsv -n "__fish_qsv_using_subcommand join" -l right-semi
complete -c qsv -n "__fish_qsv_using_subcommand join" -l cross
complete -c qsv -n "__fish_qsv_using_subcommand join" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand join" -l nulls
complete -c qsv -n "__fish_qsv_using_subcommand join" -l left-semi
complete -c qsv -n "__fish_qsv_using_subcommand join" -l right
complete -c qsv -n "__fish_qsv_using_subcommand join" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l right_by -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s N -l norm-unicode -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l time-format -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l cache-schema -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l tolerance -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l sql-filter -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left_by -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l null-value -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l validate -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l date-format -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l non-equi -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l filter-left -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l float-precision -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l datetime-format -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l filter-right -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l maintain-order -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l strategy -r
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left-semi
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l no-optimizations
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l full
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l right
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s X -l allow-exact-matches
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l right-semi
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l asof
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s z -l ignore-leading-zeros
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l cross
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l streaming
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l try-parsedates
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l right-anti
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l coalesce
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l nulls
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l low-memory
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l decimal-comma
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l no-sort
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left-anti
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand json" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand json" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand json" -l jaq -r
complete -c qsv -n "__fish_qsv_using_subcommand json" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s W -l wrap-mode -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l echo-column -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l filter -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l find -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s P -l prompt -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s f -l freeze-columns -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l columns -r
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s t -l tab-separated
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l debug
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s A -l auto-reload
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s m -l monochrome
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s S -l streaming-stdin
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l max-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s B -l begin -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s E -l end -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s r -l remap
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s g -l no-globals
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -l colindex
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from filter map help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l max-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s B -l begin -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s E -l end -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s r -l remap
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s g -l no-globals
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l colindex
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l max-errors -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s B -l begin -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s E -l end -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s r -l remap
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s g -l no-globals
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l colindex
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s C -l cardinality-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s e -l epsilon -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l xsd-gdate-scan -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s J -l join-inputs -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s S -l bivariate-stats -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l stats-options -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s T -l join-type -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s K -l join-keys -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l round -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l pct-thresholds -r
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l use-percentiles
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l advanced
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s B -l bivariate
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -l force
complete -c qsv -n "__fish_qsv_using_subcommand moarstats" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l filename -r
complete -c qsv -n "__fish_qsv_using_subcommand partition" -s p -l prefix-length -r
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l limit -r
complete -c qsv -n "__fish_qsv_using_subcommand partition" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand partition" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l drop
complete -c qsv -n "__fish_qsv_using_subcommand partition" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s a -l agg -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s i -l index -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s v -l values -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l col-separator -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l try-parsedates
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l maintain-order
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l validate
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l decimal-comma
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -l sort-columns
complete -c qsv -n "__fish_qsv_using_subcommand pivotp" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -f -a "workflow"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from lens" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from workflow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from help" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from help" -f -a "workflow"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l save-fname -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l base-delay-ms -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s d -l workdir -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s F -l filters -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s m -l msg -r
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s f -l fd-output
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l start -r
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l increment -r
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l formatstr -r
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s f -l helper -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from filter map help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s f -l helper -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s f -l helper -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand rename" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand rename" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand rename" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand rename" -l pairwise
complete -c qsv -n "__fish_qsv_using_subcommand rename" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l dfa-size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s u -l unicode
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l not-one
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l exact
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l literal
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l prefix -r
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l reserved -r
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l mode -r
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l max-size -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-aggregate -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l seed -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l weighted -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l rng -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l cluster -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-interval -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-input-tz -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l stratified -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l timeseries -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-adaptive -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l systematic -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-start -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l bernoulli
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l force
complete -c qsv -n "__fish_qsv_using_subcommand sample" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l ts-prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand sample" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l enum-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l pattern-columns -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l dates-whitelist -r
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l strict-dates
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l stdout
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l force
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l strict-formats
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l polars
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand search" -l preview-match -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -s f -l flag -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -l size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -l dfa-size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand search" -l literal
complete -c qsv -n "__fish_qsv_using_subcommand search" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand search" -s v -l invert-match
complete -c qsv -n "__fish_qsv_using_subcommand search" -s Q -l quick
complete -c qsv -n "__fish_qsv_using_subcommand search" -l not-one
complete -c qsv -n "__fish_qsv_using_subcommand search" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand search" -s u -l unicode
complete -c qsv -n "__fish_qsv_using_subcommand search" -l exact
complete -c qsv -n "__fish_qsv_using_subcommand search" -l json
complete -c qsv -n "__fish_qsv_using_subcommand search" -s c -l count
complete -c qsv -n "__fish_qsv_using_subcommand search" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand search" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand search" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s f -l flag -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l unmatched-output -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l dfa-size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s j -l json
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l literal
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s v -l invert-match
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l not-one
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l flag-matches-only
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s u -l unicode
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s Q -l quick
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l exact
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s c -l count
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand select" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand select" -l seed -r
complete -c qsv -n "__fish_qsv_using_subcommand select" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand select" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand select" -s R -l random
complete -c qsv -n "__fish_qsv_using_subcommand select" -s S -l sort
complete -c qsv -n "__fish_qsv_using_subcommand select" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s l -l len -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s e -l end -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s s -l start -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s i -l index -r
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l json
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l invert
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -f -a "check"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -f -a "compress"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -f -a "decompress"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -f -a "validate"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from check compress decompress validate help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -s j -l jobs -r
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
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l save-urlsample -r
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l user-agent -r
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l sample -r
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l no-infer
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -s Q -l quick
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l harvest-mode
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l stats-types
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l json
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l just-mime
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l seed -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l rng -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l natural
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s N -l numeric
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s u -l unique
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s R -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l faster
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l random
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l all
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l json
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s i -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand split" -s k -l kb-size -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -s c -l chunks -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -s s -l size -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -l filename -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -l filter -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -l pad -r
complete -c qsv -n "__fish_qsv_using_subcommand split" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand split" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand split" -l filter-cleanup
complete -c qsv -n "__fish_qsv_using_subcommand split" -l filter-ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand split" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l compression -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l wnull-value -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l rnull-values -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l datetime-format -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l compress-level -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l float-precision -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l infer-len -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l format -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l date-format -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l time-format -r
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l decimal-comma
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l statistics
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l try-parsedates
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l no-optimizations
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l streaming
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l low-memory
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l truncate-ragged-lines
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l cache-schema
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l round -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l percentile-list -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l dates-whitelist -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l boolean-patterns -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s c -l cache-threshold -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l weight -r
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s E -l everything
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l stats-jsonl
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l quartiles
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l vis-whitespace
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l force
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l median
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l mad
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l nulls
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l cardinality
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l percentiles
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l infer-boolean
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l mode
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l infer-dates
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l typesonly
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand table" -s a -l align -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -s c -l condense -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -s w -l width -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -s p -l pad -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand table" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand table" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand template" -l outsubdir-size -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -s t -l template-file -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l template -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l outfilename -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -s j -l globals-json -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l customfilter-error -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand template" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand template" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand template" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -f -a "datapackage"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -f -a "ods"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -f -a "postgres"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -f -a "sqlite"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -f -a "xlsx"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from datapackage ods postgres sqlite xlsx help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from ods" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s c -l stats-csv -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s s -l schema -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s p -l separator -r
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s d -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s u -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s A -l all-strings
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s a -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s e -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s k -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s i -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "datapackage"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "ods"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "postgres"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "sqlite"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "xlsx"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l no-boolean
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s o -l output -r
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s s -l select -r
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -l long -r
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s m -l multipass
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l valid -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l valid-output -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l backtrack-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l invalid -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l email-min-subdomains -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l dfa-size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l email-domain-literal
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l json
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l email-display-text
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l fail-fast
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l email-required-tld
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l no-format-validation
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -l fancy-regex
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -f -a "schema"
complete -c qsv -n "__fish_qsv_using_subcommand validate; and not __fish_seen_subcommand_from schema help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l valid -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s b -l batch -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l valid-output -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l timeout -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s d -l delimiter -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l backtrack-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l invalid -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s j -l jobs -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l ckan-token -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l ckan-api -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l cache-dir -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l email-min-subdomains -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l dfa-size-limit -r
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l email-domain-literal
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l json
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l email-display-text
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l fail-fast
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s n -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l email-required-tld
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l no-format-validation
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -l fancy-regex
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s p -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s q -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from schema" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from help" -f -a "schema"
complete -c qsv -n "__fish_qsv_using_subcommand validate; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "apply"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "behead"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "cat"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "clipboard"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "color"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "count"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "datefmt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "dedup"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "describegpt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "diff"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "edit"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "enum"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "excel"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "exclude"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "explode"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "extdedup"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "extsort"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "fetch"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "fetchpost"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "fill"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "fixlengths"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "flatten"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "fmt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "foreach"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "frequency"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "geocode"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "geoconvert"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "headers"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "index"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "input"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "join"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "joinp"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "json"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "jsonl"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "luau"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "moarstats"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "partition"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "pivotp"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "pro"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "prompt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "pseudo"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "py"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "rename"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "replace"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "safenames"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "sample"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "schema"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "search"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "searchset"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "select"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "slice"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "snappy"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "sniff"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "sort"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "sortcheck"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "split"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "sqlp"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "stats"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "table"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "template"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "to"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "tojsonl"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "transpose"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "validate"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard color count datefmt dedup describegpt diff edit enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode geoconvert headers index input join joinp json jsonl lens luau moarstats partition pivotp pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "calcconv"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "dynfmt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "emptyreplace"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "operations"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from cat" -f -a "columns"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from cat" -f -a "rows"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from cat" -f -a "rowskey"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "countryinfo"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "countryinfonow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-check"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-load"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-reset"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-update"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "iplookup"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "iplookupnow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "reversenow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "suggest"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "suggestnow"
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
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "postgres"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "sqlite"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "xlsx"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from validate" -f -a "schema"
