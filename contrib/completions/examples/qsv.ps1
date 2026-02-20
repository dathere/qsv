
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'qsv' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'qsv'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'qsv' {
            [CompletionResult]::new('--list', '--list', [CompletionResultType]::ParameterName, 'list')
            [CompletionResult]::new('--envlist', '--envlist', [CompletionResultType]::ParameterName, 'envlist')
            [CompletionResult]::new('--update', '--update', [CompletionResultType]::ParameterName, 'update')
            [CompletionResult]::new('--updatenow', '--updatenow', [CompletionResultType]::ParameterName, 'updatenow')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'V')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'version')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('apply', 'apply', [CompletionResultType]::ParameterValue, 'apply')
            [CompletionResult]::new('behead', 'behead', [CompletionResultType]::ParameterValue, 'behead')
            [CompletionResult]::new('cat', 'cat', [CompletionResultType]::ParameterValue, 'cat')
            [CompletionResult]::new('clipboard', 'clipboard', [CompletionResultType]::ParameterValue, 'clipboard')
            [CompletionResult]::new('color', 'color', [CompletionResultType]::ParameterValue, 'color')
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'count')
            [CompletionResult]::new('datefmt', 'datefmt', [CompletionResultType]::ParameterValue, 'datefmt')
            [CompletionResult]::new('dedup', 'dedup', [CompletionResultType]::ParameterValue, 'dedup')
            [CompletionResult]::new('describegpt', 'describegpt', [CompletionResultType]::ParameterValue, 'describegpt')
            [CompletionResult]::new('diff', 'diff', [CompletionResultType]::ParameterValue, 'diff')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'edit')
            [CompletionResult]::new('enum', 'enum', [CompletionResultType]::ParameterValue, 'enum')
            [CompletionResult]::new('excel', 'excel', [CompletionResultType]::ParameterValue, 'excel')
            [CompletionResult]::new('exclude', 'exclude', [CompletionResultType]::ParameterValue, 'exclude')
            [CompletionResult]::new('explode', 'explode', [CompletionResultType]::ParameterValue, 'explode')
            [CompletionResult]::new('extdedup', 'extdedup', [CompletionResultType]::ParameterValue, 'extdedup')
            [CompletionResult]::new('extsort', 'extsort', [CompletionResultType]::ParameterValue, 'extsort')
            [CompletionResult]::new('fetch', 'fetch', [CompletionResultType]::ParameterValue, 'fetch')
            [CompletionResult]::new('fetchpost', 'fetchpost', [CompletionResultType]::ParameterValue, 'fetchpost')
            [CompletionResult]::new('fill', 'fill', [CompletionResultType]::ParameterValue, 'fill')
            [CompletionResult]::new('fixlengths', 'fixlengths', [CompletionResultType]::ParameterValue, 'fixlengths')
            [CompletionResult]::new('flatten', 'flatten', [CompletionResultType]::ParameterValue, 'flatten')
            [CompletionResult]::new('fmt', 'fmt', [CompletionResultType]::ParameterValue, 'fmt')
            [CompletionResult]::new('foreach', 'foreach', [CompletionResultType]::ParameterValue, 'foreach')
            [CompletionResult]::new('frequency', 'frequency', [CompletionResultType]::ParameterValue, 'frequency')
            [CompletionResult]::new('geocode', 'geocode', [CompletionResultType]::ParameterValue, 'geocode')
            [CompletionResult]::new('geoconvert', 'geoconvert', [CompletionResultType]::ParameterValue, 'geoconvert')
            [CompletionResult]::new('headers', 'headers', [CompletionResultType]::ParameterValue, 'headers')
            [CompletionResult]::new('index', 'index', [CompletionResultType]::ParameterValue, 'index')
            [CompletionResult]::new('input', 'input', [CompletionResultType]::ParameterValue, 'input')
            [CompletionResult]::new('join', 'join', [CompletionResultType]::ParameterValue, 'join')
            [CompletionResult]::new('joinp', 'joinp', [CompletionResultType]::ParameterValue, 'joinp')
            [CompletionResult]::new('json', 'json', [CompletionResultType]::ParameterValue, 'json')
            [CompletionResult]::new('jsonl', 'jsonl', [CompletionResultType]::ParameterValue, 'jsonl')
            [CompletionResult]::new('lens', 'lens', [CompletionResultType]::ParameterValue, 'lens')
            [CompletionResult]::new('luau', 'luau', [CompletionResultType]::ParameterValue, 'luau')
            [CompletionResult]::new('moarstats', 'moarstats', [CompletionResultType]::ParameterValue, 'moarstats')
            [CompletionResult]::new('partition', 'partition', [CompletionResultType]::ParameterValue, 'partition')
            [CompletionResult]::new('pivotp', 'pivotp', [CompletionResultType]::ParameterValue, 'pivotp')
            [CompletionResult]::new('pragmastat', 'pragmastat', [CompletionResultType]::ParameterValue, 'pragmastat')
            [CompletionResult]::new('pro', 'pro', [CompletionResultType]::ParameterValue, 'pro')
            [CompletionResult]::new('prompt', 'prompt', [CompletionResultType]::ParameterValue, 'prompt')
            [CompletionResult]::new('pseudo', 'pseudo', [CompletionResultType]::ParameterValue, 'pseudo')
            [CompletionResult]::new('py', 'py', [CompletionResultType]::ParameterValue, 'py')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'rename')
            [CompletionResult]::new('replace', 'replace', [CompletionResultType]::ParameterValue, 'replace')
            [CompletionResult]::new('reverse', 'reverse', [CompletionResultType]::ParameterValue, 'reverse')
            [CompletionResult]::new('safenames', 'safenames', [CompletionResultType]::ParameterValue, 'safenames')
            [CompletionResult]::new('sample', 'sample', [CompletionResultType]::ParameterValue, 'sample')
            [CompletionResult]::new('schema', 'schema', [CompletionResultType]::ParameterValue, 'schema')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'search')
            [CompletionResult]::new('searchset', 'searchset', [CompletionResultType]::ParameterValue, 'searchset')
            [CompletionResult]::new('select', 'select', [CompletionResultType]::ParameterValue, 'select')
            [CompletionResult]::new('slice', 'slice', [CompletionResultType]::ParameterValue, 'slice')
            [CompletionResult]::new('snappy', 'snappy', [CompletionResultType]::ParameterValue, 'snappy')
            [CompletionResult]::new('sniff', 'sniff', [CompletionResultType]::ParameterValue, 'sniff')
            [CompletionResult]::new('sort', 'sort', [CompletionResultType]::ParameterValue, 'sort')
            [CompletionResult]::new('sortcheck', 'sortcheck', [CompletionResultType]::ParameterValue, 'sortcheck')
            [CompletionResult]::new('split', 'split', [CompletionResultType]::ParameterValue, 'split')
            [CompletionResult]::new('sqlp', 'sqlp', [CompletionResultType]::ParameterValue, 'sqlp')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'stats')
            [CompletionResult]::new('table', 'table', [CompletionResultType]::ParameterValue, 'table')
            [CompletionResult]::new('template', 'template', [CompletionResultType]::ParameterValue, 'template')
            [CompletionResult]::new('to', 'to', [CompletionResultType]::ParameterValue, 'to')
            [CompletionResult]::new('tojsonl', 'tojsonl', [CompletionResultType]::ParameterValue, 'tojsonl')
            [CompletionResult]::new('transpose', 'transpose', [CompletionResultType]::ParameterValue, 'transpose')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'validate')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;apply' {
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--comparand', '--comparand', [CompletionResultType]::ParameterName, 'comparand')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--replacement', '--replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('calcconv', 'calcconv', [CompletionResultType]::ParameterValue, 'calcconv')
            [CompletionResult]::new('dynfmt', 'dynfmt', [CompletionResultType]::ParameterValue, 'dynfmt')
            [CompletionResult]::new('emptyreplace', 'emptyreplace', [CompletionResultType]::ParameterValue, 'emptyreplace')
            [CompletionResult]::new('operations', 'operations', [CompletionResultType]::ParameterValue, 'operations')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;apply;calcconv' {
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--comparand', '--comparand', [CompletionResultType]::ParameterName, 'comparand')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--replacement', '--replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;dynfmt' {
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--comparand', '--comparand', [CompletionResultType]::ParameterName, 'comparand')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--replacement', '--replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;emptyreplace' {
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--comparand', '--comparand', [CompletionResultType]::ParameterName, 'comparand')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--replacement', '--replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;operations' {
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--comparand', '--comparand', [CompletionResultType]::ParameterName, 'comparand')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--replacement', '--replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;help' {
            [CompletionResult]::new('calcconv', 'calcconv', [CompletionResultType]::ParameterValue, 'calcconv')
            [CompletionResult]::new('dynfmt', 'dynfmt', [CompletionResultType]::ParameterValue, 'dynfmt')
            [CompletionResult]::new('emptyreplace', 'emptyreplace', [CompletionResultType]::ParameterValue, 'emptyreplace')
            [CompletionResult]::new('operations', 'operations', [CompletionResultType]::ParameterValue, 'operations')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;apply;help;calcconv' {
            break
        }
        'qsv;apply;help;dynfmt' {
            break
        }
        'qsv;apply;help;emptyreplace' {
            break
        }
        'qsv;apply;help;operations' {
            break
        }
        'qsv;apply;help;help' {
            break
        }
        'qsv;behead' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--flexible', '--flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;cat' {
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--group-name', '--group-name', [CompletionResultType]::ParameterName, 'group-name')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--group', '--group', [CompletionResultType]::ParameterName, 'group')
            [CompletionResult]::new('--flexible', '--flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--pad', '--pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('columns', 'columns', [CompletionResultType]::ParameterValue, 'columns')
            [CompletionResult]::new('rows', 'rows', [CompletionResultType]::ParameterValue, 'rows')
            [CompletionResult]::new('rowskey', 'rowskey', [CompletionResultType]::ParameterValue, 'rowskey')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;cat;columns' {
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--group-name', '--group-name', [CompletionResultType]::ParameterName, 'group-name')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--group', '--group', [CompletionResultType]::ParameterName, 'group')
            [CompletionResult]::new('--flexible', '--flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--pad', '--pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;cat;rows' {
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--group-name', '--group-name', [CompletionResultType]::ParameterName, 'group-name')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--group', '--group', [CompletionResultType]::ParameterName, 'group')
            [CompletionResult]::new('--flexible', '--flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--pad', '--pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;cat;rowskey' {
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--group-name', '--group-name', [CompletionResultType]::ParameterName, 'group-name')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--group', '--group', [CompletionResultType]::ParameterName, 'group')
            [CompletionResult]::new('--flexible', '--flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--pad', '--pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;cat;help' {
            [CompletionResult]::new('columns', 'columns', [CompletionResultType]::ParameterValue, 'columns')
            [CompletionResult]::new('rows', 'rows', [CompletionResultType]::ParameterValue, 'rows')
            [CompletionResult]::new('rowskey', 'rowskey', [CompletionResultType]::ParameterValue, 'rowskey')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;cat;help;columns' {
            break
        }
        'qsv;cat;help;rows' {
            break
        }
        'qsv;cat;help;rowskey' {
            break
        }
        'qsv;cat;help;help' {
            break
        }
        'qsv;clipboard' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--save', '--save', [CompletionResultType]::ParameterName, 'save')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;color' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--title', '--title', [CompletionResultType]::ParameterName, 'title')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--color', '--color', [CompletionResultType]::ParameterName, 'color')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--row-numbers', '--row-numbers', [CompletionResultType]::ParameterName, 'row-numbers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;count' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--human-readable', '--human-readable', [CompletionResultType]::ParameterName, 'human-readable')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--flexible', '--flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--width', '--width', [CompletionResultType]::ParameterName, 'width')
            [CompletionResult]::new('--no-polars', '--no-polars', [CompletionResultType]::ParameterName, 'no-polars')
            [CompletionResult]::new('--width-no-delims', '--width-no-delims', [CompletionResultType]::ParameterName, 'width-no-delims')
            [CompletionResult]::new('--low-memory', '--low-memory', [CompletionResultType]::ParameterName, 'low-memory')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;datefmt' {
            [CompletionResult]::new('--output-tz', '--output-tz', [CompletionResultType]::ParameterName, 'output-tz')
            [CompletionResult]::new('--default-tz', '--default-tz', [CompletionResultType]::ParameterName, 'default-tz')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--input-tz', '--input-tz', [CompletionResultType]::ParameterName, 'input-tz')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--ts-resolution', '--ts-resolution', [CompletionResultType]::ParameterName, 'ts-resolution')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--keep-zero-time', '--keep-zero-time', [CompletionResultType]::ParameterName, 'keep-zero-time')
            [CompletionResult]::new('--zulu', '--zulu', [CompletionResultType]::ParameterName, 'zulu')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--prefer-dmy', '--prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--utc', '--utc', [CompletionResultType]::ParameterName, 'utc')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;dedup' {
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'D')
            [CompletionResult]::new('--dupes-output', '--dupes-output', [CompletionResultType]::ParameterName, 'dupes-output')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--sorted', '--sorted', [CompletionResultType]::ParameterName, 'sorted')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--human-readable', '--human-readable', [CompletionResultType]::ParameterName, 'human-readable')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--numeric', '--numeric', [CompletionResultType]::ParameterName, 'numeric')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;describegpt' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--num-examples', '--num-examples', [CompletionResultType]::ParameterName, 'num-examples')
            [CompletionResult]::new('--truncate-str', '--truncate-str', [CompletionResultType]::ParameterName, 'truncate-str')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--base-url', '--base-url', [CompletionResultType]::ParameterName, 'base-url')
            [CompletionResult]::new('--session', '--session', [CompletionResultType]::ParameterName, 'session')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--export-prompt', '--export-prompt', [CompletionResultType]::ParameterName, 'export-prompt')
            [CompletionResult]::new('--num-tags', '--num-tags', [CompletionResultType]::ParameterName, 'num-tags')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--prompt', '--prompt', [CompletionResultType]::ParameterName, 'prompt')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--api-key', '--api-key', [CompletionResultType]::ParameterName, 'api-key')
            [CompletionResult]::new('--enum-threshold', '--enum-threshold', [CompletionResultType]::ParameterName, 'enum-threshold')
            [CompletionResult]::new('--tag-vocab', '--tag-vocab', [CompletionResultType]::ParameterName, 'tag-vocab')
            [CompletionResult]::new('--prompt-file', '--prompt-file', [CompletionResultType]::ParameterName, 'prompt-file')
            [CompletionResult]::new('--addl-props', '--addl-props', [CompletionResultType]::ParameterName, 'addl-props')
            [CompletionResult]::new('--addl-cols-list', '--addl-cols-list', [CompletionResultType]::ParameterName, 'addl-cols-list')
            [CompletionResult]::new('--freq-options', '--freq-options', [CompletionResultType]::ParameterName, 'freq-options')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'format')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--disk-cache-dir', '--disk-cache-dir', [CompletionResultType]::ParameterName, 'disk-cache-dir')
            [CompletionResult]::new('--session-len', '--session-len', [CompletionResultType]::ParameterName, 'session-len')
            [CompletionResult]::new('--stats-options', '--stats-options', [CompletionResultType]::ParameterName, 'stats-options')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--max-tokens', '--max-tokens', [CompletionResultType]::ParameterName, 'max-tokens')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'm')
            [CompletionResult]::new('--model', '--model', [CompletionResultType]::ParameterName, 'model')
            [CompletionResult]::new('--sample-size', '--sample-size', [CompletionResultType]::ParameterName, 'sample-size')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--sql-results', '--sql-results', [CompletionResultType]::ParameterName, 'sql-results')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all', '--all', [CompletionResultType]::ParameterName, 'all')
            [CompletionResult]::new('--flush-cache', '--flush-cache', [CompletionResultType]::ParameterName, 'flush-cache')
            [CompletionResult]::new('--redis-cache', '--redis-cache', [CompletionResultType]::ParameterName, 'redis-cache')
            [CompletionResult]::new('--fewshot-examples', '--fewshot-examples', [CompletionResultType]::ParameterName, 'fewshot-examples')
            [CompletionResult]::new('--tags', '--tags', [CompletionResultType]::ParameterName, 'tags')
            [CompletionResult]::new('--dictionary', '--dictionary', [CompletionResultType]::ParameterName, 'dictionary')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'no-cache')
            [CompletionResult]::new('--fresh', '--fresh', [CompletionResultType]::ParameterName, 'fresh')
            [CompletionResult]::new('--forget', '--forget', [CompletionResultType]::ParameterName, 'forget')
            [CompletionResult]::new('--addl-cols', '--addl-cols', [CompletionResultType]::ParameterName, 'addl-cols')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--description', '--description', [CompletionResultType]::ParameterName, 'description')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;diff' {
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--key', '--key', [CompletionResultType]::ParameterName, 'key')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter-output', '--delimiter-output', [CompletionResultType]::ParameterName, 'delimiter-output')
            [CompletionResult]::new('--delimiter-left', '--delimiter-left', [CompletionResultType]::ParameterName, 'delimiter-left')
            [CompletionResult]::new('--delimiter-right', '--delimiter-right', [CompletionResultType]::ParameterName, 'delimiter-right')
            [CompletionResult]::new('--sort-columns', '--sort-columns', [CompletionResultType]::ParameterName, 'sort-columns')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--no-headers-output', '--no-headers-output', [CompletionResultType]::ParameterName, 'no-headers-output')
            [CompletionResult]::new('--no-headers-left', '--no-headers-left', [CompletionResultType]::ParameterName, 'no-headers-left')
            [CompletionResult]::new('--no-headers-right', '--no-headers-right', [CompletionResultType]::ParameterName, 'no-headers-right')
            [CompletionResult]::new('--drop-equal-fields', '--drop-equal-fields', [CompletionResultType]::ParameterName, 'drop-equal-fields')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;edit' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--in-place', '--in-place', [CompletionResultType]::ParameterName, 'in-place')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;enum' {
            [CompletionResult]::new('--hash', '--hash', [CompletionResultType]::ParameterName, 'hash')
            [CompletionResult]::new('--start', '--start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--constant', '--constant', [CompletionResultType]::ParameterName, 'constant')
            [CompletionResult]::new('--copy', '--copy', [CompletionResultType]::ParameterName, 'copy')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--increment', '--increment', [CompletionResultType]::ParameterName, 'increment')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--uuid7', '--uuid7', [CompletionResultType]::ParameterName, 'uuid7')
            [CompletionResult]::new('--uuid4', '--uuid4', [CompletionResultType]::ParameterName, 'uuid4')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;excel' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--date-format', '--date-format', [CompletionResultType]::ParameterName, 'date-format')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--error-format', '--error-format', [CompletionResultType]::ParameterName, 'error-format')
            [CompletionResult]::new('--metadata', '--metadata', [CompletionResultType]::ParameterName, 'metadata')
            [CompletionResult]::new('--cell', '--cell', [CompletionResultType]::ParameterName, 'cell')
            [CompletionResult]::new('--range', '--range', [CompletionResultType]::ParameterName, 'range')
            [CompletionResult]::new('--table', '--table', [CompletionResultType]::ParameterName, 'table')
            [CompletionResult]::new('--header-row', '--header-row', [CompletionResultType]::ParameterName, 'header-row')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--sheet', '--sheet', [CompletionResultType]::ParameterName, 'sheet')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--flexible', '--flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('--keep-zero-time', '--keep-zero-time', [CompletionResultType]::ParameterName, 'keep-zero-time')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--trim', '--trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;exclude' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--invert', '--invert', [CompletionResultType]::ParameterName, 'invert')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;explode' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;extdedup' {
            [CompletionResult]::new('--memory-limit', '--memory-limit', [CompletionResultType]::ParameterName, 'memory-limit')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--temp-dir', '--temp-dir', [CompletionResultType]::ParameterName, 'temp-dir')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'D')
            [CompletionResult]::new('--dupes-output', '--dupes-output', [CompletionResultType]::ParameterName, 'dupes-output')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--human-readable', '--human-readable', [CompletionResultType]::ParameterName, 'human-readable')
            [CompletionResult]::new('--no-output', '--no-output', [CompletionResultType]::ParameterName, 'no-output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;extsort' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memory-limit', '--memory-limit', [CompletionResultType]::ParameterName, 'memory-limit')
            [CompletionResult]::new('--tmp-dir', '--tmp-dir', [CompletionResultType]::ParameterName, 'tmp-dir')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--reverse', '--reverse', [CompletionResultType]::ParameterName, 'reverse')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fetch' {
            [CompletionResult]::new('--disk-cache-dir', '--disk-cache-dir', [CompletionResultType]::ParameterName, 'disk-cache-dir')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--jaq', '--jaq', [CompletionResultType]::ParameterName, 'jaq')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--rate-limit', '--rate-limit', [CompletionResultType]::ParameterName, 'rate-limit')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--max-errors', '--max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('--url-template', '--url-template', [CompletionResultType]::ParameterName, 'url-template')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--http-header', '--http-header', [CompletionResultType]::ParameterName, 'http-header')
            [CompletionResult]::new('--report', '--report', [CompletionResultType]::ParameterName, 'report')
            [CompletionResult]::new('--max-retries', '--max-retries', [CompletionResultType]::ParameterName, 'max-retries')
            [CompletionResult]::new('--mem-cache-size', '--mem-cache-size', [CompletionResultType]::ParameterName, 'mem-cache-size')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--jaqfile', '--jaqfile', [CompletionResultType]::ParameterName, 'jaqfile')
            [CompletionResult]::new('--redis-cache', '--redis-cache', [CompletionResultType]::ParameterName, 'redis-cache')
            [CompletionResult]::new('--pretty', '--pretty', [CompletionResultType]::ParameterName, 'pretty')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--cache-error', '--cache-error', [CompletionResultType]::ParameterName, 'cache-error')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--store-error', '--store-error', [CompletionResultType]::ParameterName, 'store-error')
            [CompletionResult]::new('--cookies', '--cookies', [CompletionResultType]::ParameterName, 'cookies')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'no-cache')
            [CompletionResult]::new('--disk-cache', '--disk-cache', [CompletionResultType]::ParameterName, 'disk-cache')
            [CompletionResult]::new('--flush-cache', '--flush-cache', [CompletionResultType]::ParameterName, 'flush-cache')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fetchpost' {
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--jaq', '--jaq', [CompletionResultType]::ParameterName, 'jaq')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--max-retries', '--max-retries', [CompletionResultType]::ParameterName, 'max-retries')
            [CompletionResult]::new('--max-errors', '--max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--globals-json', '--globals-json', [CompletionResultType]::ParameterName, 'globals-json')
            [CompletionResult]::new('--jaqfile', '--jaqfile', [CompletionResultType]::ParameterName, 'jaqfile')
            [CompletionResult]::new('--content-type', '--content-type', [CompletionResultType]::ParameterName, 'content-type')
            [CompletionResult]::new('--disk-cache-dir', '--disk-cache-dir', [CompletionResultType]::ParameterName, 'disk-cache-dir')
            [CompletionResult]::new('--rate-limit', '--rate-limit', [CompletionResultType]::ParameterName, 'rate-limit')
            [CompletionResult]::new('--mem-cache-size', '--mem-cache-size', [CompletionResultType]::ParameterName, 'mem-cache-size')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--report', '--report', [CompletionResultType]::ParameterName, 'report')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--http-header', '--http-header', [CompletionResultType]::ParameterName, 'http-header')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--payload-tpl', '--payload-tpl', [CompletionResultType]::ParameterName, 'payload-tpl')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--pretty', '--pretty', [CompletionResultType]::ParameterName, 'pretty')
            [CompletionResult]::new('--flush-cache', '--flush-cache', [CompletionResultType]::ParameterName, 'flush-cache')
            [CompletionResult]::new('--cache-error', '--cache-error', [CompletionResultType]::ParameterName, 'cache-error')
            [CompletionResult]::new('--cookies', '--cookies', [CompletionResultType]::ParameterName, 'cookies')
            [CompletionResult]::new('--redis-cache', '--redis-cache', [CompletionResultType]::ParameterName, 'redis-cache')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--disk-cache', '--disk-cache', [CompletionResultType]::ParameterName, 'disk-cache')
            [CompletionResult]::new('--compress', '--compress', [CompletionResultType]::ParameterName, 'compress')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'no-cache')
            [CompletionResult]::new('--store-error', '--store-error', [CompletionResultType]::ParameterName, 'store-error')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fill' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--default', '--default', [CompletionResultType]::ParameterName, 'default')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--groupby', '--groupby', [CompletionResultType]::ParameterName, 'groupby')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--backfill', '--backfill', [CompletionResultType]::ParameterName, 'backfill')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--first', '--first', [CompletionResultType]::ParameterName, 'first')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fixlengths' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--quote', '--quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--length', '--length', [CompletionResultType]::ParameterName, 'length')
            [CompletionResult]::new('--escape', '--escape', [CompletionResultType]::ParameterName, 'escape')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--insert', '--insert', [CompletionResultType]::ParameterName, 'insert')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--remove-empty', '--remove-empty', [CompletionResultType]::ParameterName, 'remove-empty')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;flatten' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--field-separator', '--field-separator', [CompletionResultType]::ParameterName, 'field-separator')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--condense', '--condense', [CompletionResultType]::ParameterName, 'condense')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fmt' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--out-delimiter', '--out-delimiter', [CompletionResultType]::ParameterName, 'out-delimiter')
            [CompletionResult]::new('--quote', '--quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--escape', '--escape', [CompletionResultType]::ParameterName, 'escape')
            [CompletionResult]::new('--quote-never', '--quote-never', [CompletionResultType]::ParameterName, 'quote-never')
            [CompletionResult]::new('--ascii', '--ascii', [CompletionResultType]::ParameterName, 'ascii')
            [CompletionResult]::new('--no-final-newline', '--no-final-newline', [CompletionResultType]::ParameterName, 'no-final-newline')
            [CompletionResult]::new('--crlf', '--crlf', [CompletionResultType]::ParameterName, 'crlf')
            [CompletionResult]::new('--quote-always', '--quote-always', [CompletionResultType]::ParameterName, 'quote-always')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;foreach' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--dry-run', '--dry-run', [CompletionResultType]::ParameterName, 'dry-run')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unify', '--unify', [CompletionResultType]::ParameterName, 'unify')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;frequency' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--stats-filter', '--stats-filter', [CompletionResultType]::ParameterName, 'stats-filter')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unq-limit', '--unq-limit', [CompletionResultType]::ParameterName, 'unq-limit')
            [CompletionResult]::new('--null-text', '--null-text', [CompletionResultType]::ParameterName, 'null-text')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--lmt-threshold', '--lmt-threshold', [CompletionResultType]::ParameterName, 'lmt-threshold')
            [CompletionResult]::new('--high-card-threshold', '--high-card-threshold', [CompletionResultType]::ParameterName, 'high-card-threshold')
            [CompletionResult]::new('--all-unique-text', '--all-unique-text', [CompletionResultType]::ParameterName, 'all-unique-text')
            [CompletionResult]::new('--high-card-pct', '--high-card-pct', [CompletionResultType]::ParameterName, 'high-card-pct')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'limit')
            [CompletionResult]::new('--other-text', '--other-text', [CompletionResultType]::ParameterName, 'other-text')
            [CompletionResult]::new('--no-float', '--no-float', [CompletionResultType]::ParameterName, 'no-float')
            [CompletionResult]::new('--weight', '--weight', [CompletionResultType]::ParameterName, 'weight')
            [CompletionResult]::new('--pct-dec-places', '--pct-dec-places', [CompletionResultType]::ParameterName, 'pct-dec-places')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rank-strategy', '--rank-strategy', [CompletionResultType]::ParameterName, 'rank-strategy')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--no-nulls', '--no-nulls', [CompletionResultType]::ParameterName, 'no-nulls')
            [CompletionResult]::new('--other-sorted', '--other-sorted', [CompletionResultType]::ParameterName, 'other-sorted')
            [CompletionResult]::new('--pretty-json', '--pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--pct-nulls', '--pct-nulls', [CompletionResultType]::ParameterName, 'pct-nulls')
            [CompletionResult]::new('--toon', '--toon', [CompletionResultType]::ParameterName, 'toon')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('--no-stats', '--no-stats', [CompletionResultType]::ParameterName, 'no-stats')
            [CompletionResult]::new('--vis-whitespace', '--vis-whitespace', [CompletionResultType]::ParameterName, 'vis-whitespace')
            [CompletionResult]::new('--no-trim', '--no-trim', [CompletionResultType]::ParameterName, 'no-trim')
            [CompletionResult]::new('--no-other', '--no-other', [CompletionResultType]::ParameterName, 'no-other')
            [CompletionResult]::new('--frequency-jsonl', '--frequency-jsonl', [CompletionResultType]::ParameterName, 'frequency-jsonl')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--null-sorted', '--null-sorted', [CompletionResultType]::ParameterName, 'null-sorted')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--asc', '--asc', [CompletionResultType]::ParameterName, 'asc')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('countryinfo', 'countryinfo', [CompletionResultType]::ParameterValue, 'countryinfo')
            [CompletionResult]::new('countryinfonow', 'countryinfonow', [CompletionResultType]::ParameterValue, 'countryinfonow')
            [CompletionResult]::new('index-check', 'index-check', [CompletionResultType]::ParameterValue, 'index-check')
            [CompletionResult]::new('index-load', 'index-load', [CompletionResultType]::ParameterValue, 'index-load')
            [CompletionResult]::new('index-reset', 'index-reset', [CompletionResultType]::ParameterValue, 'index-reset')
            [CompletionResult]::new('index-update', 'index-update', [CompletionResultType]::ParameterValue, 'index-update')
            [CompletionResult]::new('iplookup', 'iplookup', [CompletionResultType]::ParameterValue, 'iplookup')
            [CompletionResult]::new('iplookupnow', 'iplookupnow', [CompletionResultType]::ParameterValue, 'iplookupnow')
            [CompletionResult]::new('reverse', 'reverse', [CompletionResultType]::ParameterValue, 'reverse')
            [CompletionResult]::new('reversenow', 'reversenow', [CompletionResultType]::ParameterValue, 'reversenow')
            [CompletionResult]::new('suggest', 'suggest', [CompletionResultType]::ParameterValue, 'suggest')
            [CompletionResult]::new('suggestnow', 'suggestnow', [CompletionResultType]::ParameterValue, 'suggestnow')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;geocode;countryinfo' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;countryinfonow' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;index-check' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;index-load' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;index-reset' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;index-update' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;iplookup' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;iplookupnow' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;reverse' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;reversenow' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;suggest' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;suggestnow' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;help' {
            [CompletionResult]::new('countryinfo', 'countryinfo', [CompletionResultType]::ParameterValue, 'countryinfo')
            [CompletionResult]::new('countryinfonow', 'countryinfonow', [CompletionResultType]::ParameterValue, 'countryinfonow')
            [CompletionResult]::new('index-check', 'index-check', [CompletionResultType]::ParameterValue, 'index-check')
            [CompletionResult]::new('index-load', 'index-load', [CompletionResultType]::ParameterValue, 'index-load')
            [CompletionResult]::new('index-reset', 'index-reset', [CompletionResultType]::ParameterValue, 'index-reset')
            [CompletionResult]::new('index-update', 'index-update', [CompletionResultType]::ParameterValue, 'index-update')
            [CompletionResult]::new('iplookup', 'iplookup', [CompletionResultType]::ParameterValue, 'iplookup')
            [CompletionResult]::new('iplookupnow', 'iplookupnow', [CompletionResultType]::ParameterValue, 'iplookupnow')
            [CompletionResult]::new('reverse', 'reverse', [CompletionResultType]::ParameterValue, 'reverse')
            [CompletionResult]::new('reversenow', 'reversenow', [CompletionResultType]::ParameterValue, 'reversenow')
            [CompletionResult]::new('suggest', 'suggest', [CompletionResultType]::ParameterValue, 'suggest')
            [CompletionResult]::new('suggestnow', 'suggestnow', [CompletionResultType]::ParameterValue, 'suggestnow')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;geocode;help;countryinfo' {
            break
        }
        'qsv;geocode;help;countryinfonow' {
            break
        }
        'qsv;geocode;help;index-check' {
            break
        }
        'qsv;geocode;help;index-load' {
            break
        }
        'qsv;geocode;help;index-reset' {
            break
        }
        'qsv;geocode;help;index-update' {
            break
        }
        'qsv;geocode;help;iplookup' {
            break
        }
        'qsv;geocode;help;iplookupnow' {
            break
        }
        'qsv;geocode;help;reverse' {
            break
        }
        'qsv;geocode;help;reversenow' {
            break
        }
        'qsv;geocode;help;suggest' {
            break
        }
        'qsv;geocode;help;suggestnow' {
            break
        }
        'qsv;geocode;help;help' {
            break
        }
        'qsv;geoconvert' {
            [CompletionResult]::new('-x', '-x', [CompletionResultType]::ParameterName, 'x')
            [CompletionResult]::new('--longitude', '--longitude', [CompletionResultType]::ParameterName, 'longitude')
            [CompletionResult]::new('-y', '-y', [CompletionResultType]::ParameterName, 'y')
            [CompletionResult]::new('--latitude', '--latitude', [CompletionResultType]::ParameterName, 'latitude')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--geometry', '--geometry', [CompletionResultType]::ParameterName, 'geometry')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--max-length', '--max-length', [CompletionResultType]::ParameterName, 'max-length')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;headers' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--intersect', '--intersect', [CompletionResultType]::ParameterName, 'intersect')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--just-names', '--just-names', [CompletionResultType]::ParameterName, 'just-names')
            [CompletionResult]::new('-J', '-J ', [CompletionResultType]::ParameterName, 'J')
            [CompletionResult]::new('--just-count', '--just-count', [CompletionResultType]::ParameterName, 'just-count')
            [CompletionResult]::new('--trim', '--trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;index' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;input' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--escape', '--escape', [CompletionResultType]::ParameterName, 'escape')
            [CompletionResult]::new('--quote-style', '--quote-style', [CompletionResultType]::ParameterName, 'quote-style')
            [CompletionResult]::new('--skip-lastlines', '--skip-lastlines', [CompletionResultType]::ParameterName, 'skip-lastlines')
            [CompletionResult]::new('--encoding-errors', '--encoding-errors', [CompletionResultType]::ParameterName, 'encoding-errors')
            [CompletionResult]::new('--comment', '--comment', [CompletionResultType]::ParameterName, 'comment')
            [CompletionResult]::new('--quote', '--quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('--skip-lines', '--skip-lines', [CompletionResultType]::ParameterName, 'skip-lines')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-quoting', '--no-quoting', [CompletionResultType]::ParameterName, 'no-quoting')
            [CompletionResult]::new('--auto-skip', '--auto-skip', [CompletionResultType]::ParameterName, 'auto-skip')
            [CompletionResult]::new('--trim-fields', '--trim-fields', [CompletionResultType]::ParameterName, 'trim-fields')
            [CompletionResult]::new('--trim-headers', '--trim-headers', [CompletionResultType]::ParameterName, 'trim-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;join' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--keys-output', '--keys-output', [CompletionResultType]::ParameterName, 'keys-output')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--nulls', '--nulls', [CompletionResultType]::ParameterName, 'nulls')
            [CompletionResult]::new('--full', '--full', [CompletionResultType]::ParameterName, 'full')
            [CompletionResult]::new('--right-anti', '--right-anti', [CompletionResultType]::ParameterName, 'right-anti')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--right', '--right', [CompletionResultType]::ParameterName, 'right')
            [CompletionResult]::new('--cross', '--cross', [CompletionResultType]::ParameterName, 'cross')
            [CompletionResult]::new('--left', '--left', [CompletionResultType]::ParameterName, 'left')
            [CompletionResult]::new('--left-anti', '--left-anti', [CompletionResultType]::ParameterName, 'left-anti')
            [CompletionResult]::new('--right-semi', '--right-semi', [CompletionResultType]::ParameterName, 'right-semi')
            [CompletionResult]::new('--left-semi', '--left-semi', [CompletionResultType]::ParameterName, 'left-semi')
            [CompletionResult]::new('-z', '-z', [CompletionResultType]::ParameterName, 'z')
            [CompletionResult]::new('--ignore-leading-zeros', '--ignore-leading-zeros', [CompletionResultType]::ParameterName, 'ignore-leading-zeros')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;joinp' {
            [CompletionResult]::new('--time-format', '--time-format', [CompletionResultType]::ParameterName, 'time-format')
            [CompletionResult]::new('--tolerance', '--tolerance', [CompletionResultType]::ParameterName, 'tolerance')
            [CompletionResult]::new('--filter-left', '--filter-left', [CompletionResultType]::ParameterName, 'filter-left')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--norm-unicode', '--norm-unicode', [CompletionResultType]::ParameterName, 'norm-unicode')
            [CompletionResult]::new('--date-format', '--date-format', [CompletionResultType]::ParameterName, 'date-format')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--non-equi', '--non-equi', [CompletionResultType]::ParameterName, 'non-equi')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--left_by', '--left_by', [CompletionResultType]::ParameterName, 'left_by')
            [CompletionResult]::new('--validate', '--validate', [CompletionResultType]::ParameterName, 'validate')
            [CompletionResult]::new('--null-value', '--null-value', [CompletionResultType]::ParameterName, 'null-value')
            [CompletionResult]::new('--filter-right', '--filter-right', [CompletionResultType]::ParameterName, 'filter-right')
            [CompletionResult]::new('--sql-filter', '--sql-filter', [CompletionResultType]::ParameterName, 'sql-filter')
            [CompletionResult]::new('--float-precision', '--float-precision', [CompletionResultType]::ParameterName, 'float-precision')
            [CompletionResult]::new('--strategy', '--strategy', [CompletionResultType]::ParameterName, 'strategy')
            [CompletionResult]::new('--cache-schema', '--cache-schema', [CompletionResultType]::ParameterName, 'cache-schema')
            [CompletionResult]::new('--right_by', '--right_by', [CompletionResultType]::ParameterName, 'right_by')
            [CompletionResult]::new('--infer-len', '--infer-len', [CompletionResultType]::ParameterName, 'infer-len')
            [CompletionResult]::new('--datetime-format', '--datetime-format', [CompletionResultType]::ParameterName, 'datetime-format')
            [CompletionResult]::new('--maintain-order', '--maintain-order', [CompletionResultType]::ParameterName, 'maintain-order')
            [CompletionResult]::new('--no-optimizations', '--no-optimizations', [CompletionResultType]::ParameterName, 'no-optimizations')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--right', '--right', [CompletionResultType]::ParameterName, 'right')
            [CompletionResult]::new('--ignore-errors', '--ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('-z', '-z', [CompletionResultType]::ParameterName, 'z')
            [CompletionResult]::new('--ignore-leading-zeros', '--ignore-leading-zeros', [CompletionResultType]::ParameterName, 'ignore-leading-zeros')
            [CompletionResult]::new('--coalesce', '--coalesce', [CompletionResultType]::ParameterName, 'coalesce')
            [CompletionResult]::new('--try-parsedates', '--try-parsedates', [CompletionResultType]::ParameterName, 'try-parsedates')
            [CompletionResult]::new('--cross', '--cross', [CompletionResultType]::ParameterName, 'cross')
            [CompletionResult]::new('--no-sort', '--no-sort', [CompletionResultType]::ParameterName, 'no-sort')
            [CompletionResult]::new('--streaming', '--streaming', [CompletionResultType]::ParameterName, 'streaming')
            [CompletionResult]::new('--full', '--full', [CompletionResultType]::ParameterName, 'full')
            [CompletionResult]::new('--left-anti', '--left-anti', [CompletionResultType]::ParameterName, 'left-anti')
            [CompletionResult]::new('-X', '-X ', [CompletionResultType]::ParameterName, 'X')
            [CompletionResult]::new('--allow-exact-matches', '--allow-exact-matches', [CompletionResultType]::ParameterName, 'allow-exact-matches')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--low-memory', '--low-memory', [CompletionResultType]::ParameterName, 'low-memory')
            [CompletionResult]::new('--asof', '--asof', [CompletionResultType]::ParameterName, 'asof')
            [CompletionResult]::new('--right-semi', '--right-semi', [CompletionResultType]::ParameterName, 'right-semi')
            [CompletionResult]::new('--left-semi', '--left-semi', [CompletionResultType]::ParameterName, 'left-semi')
            [CompletionResult]::new('--decimal-comma', '--decimal-comma', [CompletionResultType]::ParameterName, 'decimal-comma')
            [CompletionResult]::new('--nulls', '--nulls', [CompletionResultType]::ParameterName, 'nulls')
            [CompletionResult]::new('--right-anti', '--right-anti', [CompletionResultType]::ParameterName, 'right-anti')
            [CompletionResult]::new('--left', '--left', [CompletionResultType]::ParameterName, 'left')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;json' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--jaq', '--jaq', [CompletionResultType]::ParameterName, 'jaq')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;jsonl' {
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--ignore-errors', '--ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;lens' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-P', '-P ', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--prompt', '--prompt', [CompletionResultType]::ParameterName, 'prompt')
            [CompletionResult]::new('--filter', '--filter', [CompletionResultType]::ParameterName, 'filter')
            [CompletionResult]::new('--find', '--find', [CompletionResultType]::ParameterName, 'find')
            [CompletionResult]::new('--echo-column', '--echo-column', [CompletionResultType]::ParameterName, 'echo-column')
            [CompletionResult]::new('--columns', '--columns', [CompletionResultType]::ParameterName, 'columns')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--freeze-columns', '--freeze-columns', [CompletionResultType]::ParameterName, 'freeze-columns')
            [CompletionResult]::new('-W', '-W ', [CompletionResultType]::ParameterName, 'W')
            [CompletionResult]::new('--wrap-mode', '--wrap-mode', [CompletionResultType]::ParameterName, 'wrap-mode')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'm')
            [CompletionResult]::new('--monochrome', '--monochrome', [CompletionResultType]::ParameterName, 'monochrome')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--auto-reload', '--auto-reload', [CompletionResultType]::ParameterName, 'auto-reload')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--tab-separated', '--tab-separated', [CompletionResultType]::ParameterName, 'tab-separated')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--debug', '--debug', [CompletionResultType]::ParameterName, 'debug')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'S')
            [CompletionResult]::new('--streaming-stdin', '--streaming-stdin', [CompletionResultType]::ParameterName, 'streaming-stdin')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;luau' {
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--max-errors', '--max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('-E', '-E ', [CompletionResultType]::ParameterName, 'E')
            [CompletionResult]::new('--end', '--end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('-B', '-B ', [CompletionResultType]::ParameterName, 'B')
            [CompletionResult]::new('--begin', '--begin', [CompletionResultType]::ParameterName, 'begin')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--colindex', '--colindex', [CompletionResultType]::ParameterName, 'colindex')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--no-globals', '--no-globals', [CompletionResultType]::ParameterName, 'no-globals')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--remap', '--remap', [CompletionResultType]::ParameterName, 'remap')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('filter', 'filter', [CompletionResultType]::ParameterValue, 'filter')
            [CompletionResult]::new('map', 'map', [CompletionResultType]::ParameterValue, 'map')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;luau;filter' {
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--max-errors', '--max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('-E', '-E ', [CompletionResultType]::ParameterName, 'E')
            [CompletionResult]::new('--end', '--end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('-B', '-B ', [CompletionResultType]::ParameterName, 'B')
            [CompletionResult]::new('--begin', '--begin', [CompletionResultType]::ParameterName, 'begin')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--colindex', '--colindex', [CompletionResultType]::ParameterName, 'colindex')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--no-globals', '--no-globals', [CompletionResultType]::ParameterName, 'no-globals')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--remap', '--remap', [CompletionResultType]::ParameterName, 'remap')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;luau;map' {
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--max-errors', '--max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('-E', '-E ', [CompletionResultType]::ParameterName, 'E')
            [CompletionResult]::new('--end', '--end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('-B', '-B ', [CompletionResultType]::ParameterName, 'B')
            [CompletionResult]::new('--begin', '--begin', [CompletionResultType]::ParameterName, 'begin')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--colindex', '--colindex', [CompletionResultType]::ParameterName, 'colindex')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--no-globals', '--no-globals', [CompletionResultType]::ParameterName, 'no-globals')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--remap', '--remap', [CompletionResultType]::ParameterName, 'remap')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;luau;help' {
            [CompletionResult]::new('filter', 'filter', [CompletionResultType]::ParameterValue, 'filter')
            [CompletionResult]::new('map', 'map', [CompletionResultType]::ParameterValue, 'map')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;luau;help;filter' {
            break
        }
        'qsv;luau;help;map' {
            break
        }
        'qsv;luau;help;help' {
            break
        }
        'qsv;moarstats' {
            [CompletionResult]::new('-K', '-K ', [CompletionResultType]::ParameterName, 'K')
            [CompletionResult]::new('--join-keys', '--join-keys', [CompletionResultType]::ParameterName, 'join-keys')
            [CompletionResult]::new('--pct-thresholds', '--pct-thresholds', [CompletionResultType]::ParameterName, 'pct-thresholds')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'T')
            [CompletionResult]::new('--join-type', '--join-type', [CompletionResultType]::ParameterName, 'join-type')
            [CompletionResult]::new('--round', '--round', [CompletionResultType]::ParameterName, 'round')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--cardinality-threshold', '--cardinality-threshold', [CompletionResultType]::ParameterName, 'cardinality-threshold')
            [CompletionResult]::new('-J', '-J ', [CompletionResultType]::ParameterName, 'J')
            [CompletionResult]::new('--join-inputs', '--join-inputs', [CompletionResultType]::ParameterName, 'join-inputs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--epsilon', '--epsilon', [CompletionResultType]::ParameterName, 'epsilon')
            [CompletionResult]::new('--stats-options', '--stats-options', [CompletionResultType]::ParameterName, 'stats-options')
            [CompletionResult]::new('--xsd-gdate-scan', '--xsd-gdate-scan', [CompletionResultType]::ParameterName, 'xsd-gdate-scan')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'S')
            [CompletionResult]::new('--bivariate-stats', '--bivariate-stats', [CompletionResultType]::ParameterName, 'bivariate-stats')
            [CompletionResult]::new('--use-percentiles', '--use-percentiles', [CompletionResultType]::ParameterName, 'use-percentiles')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('--advanced', '--advanced', [CompletionResultType]::ParameterName, 'advanced')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-B', '-B ', [CompletionResultType]::ParameterName, 'B')
            [CompletionResult]::new('--bivariate', '--bivariate', [CompletionResultType]::ParameterName, 'bivariate')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;partition' {
            [CompletionResult]::new('--filename', '--filename', [CompletionResultType]::ParameterName, 'filename')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--prefix-length', '--prefix-length', [CompletionResultType]::ParameterName, 'prefix-length')
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'limit')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;pivotp' {
            [CompletionResult]::new('--infer-len', '--infer-len', [CompletionResultType]::ParameterName, 'infer-len')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--col-separator', '--col-separator', [CompletionResultType]::ParameterName, 'col-separator')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--agg', '--agg', [CompletionResultType]::ParameterName, 'agg')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--index', '--index', [CompletionResultType]::ParameterName, 'index')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--values', '--values', [CompletionResultType]::ParameterName, 'values')
            [CompletionResult]::new('--sort-columns', '--sort-columns', [CompletionResultType]::ParameterName, 'sort-columns')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--maintain-order', '--maintain-order', [CompletionResultType]::ParameterName, 'maintain-order')
            [CompletionResult]::new('--ignore-errors', '--ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('--try-parsedates', '--try-parsedates', [CompletionResultType]::ParameterName, 'try-parsedates')
            [CompletionResult]::new('--validate', '--validate', [CompletionResultType]::ParameterName, 'validate')
            [CompletionResult]::new('--decimal-comma', '--decimal-comma', [CompletionResultType]::ParameterName, 'decimal-comma')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;pragmastat' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'm')
            [CompletionResult]::new('--misrate', '--misrate', [CompletionResultType]::ParameterName, 'misrate')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--twosample', '--twosample', [CompletionResultType]::ParameterName, 'twosample')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;pro' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('lens', 'lens', [CompletionResultType]::ParameterValue, 'lens')
            [CompletionResult]::new('workflow', 'workflow', [CompletionResultType]::ParameterValue, 'workflow')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;pro;lens' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;pro;workflow' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;pro;help' {
            [CompletionResult]::new('lens', 'lens', [CompletionResultType]::ParameterValue, 'lens')
            [CompletionResult]::new('workflow', 'workflow', [CompletionResultType]::ParameterValue, 'workflow')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;pro;help;lens' {
            break
        }
        'qsv;pro;help;workflow' {
            break
        }
        'qsv;pro;help;help' {
            break
        }
        'qsv;prompt' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'm')
            [CompletionResult]::new('--msg', '--msg', [CompletionResultType]::ParameterName, 'msg')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--base-delay-ms', '--base-delay-ms', [CompletionResultType]::ParameterName, 'base-delay-ms')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--workdir', '--workdir', [CompletionResultType]::ParameterName, 'workdir')
            [CompletionResult]::new('--save-fname', '--save-fname', [CompletionResultType]::ParameterName, 'save-fname')
            [CompletionResult]::new('-F', '-F ', [CompletionResultType]::ParameterName, 'F')
            [CompletionResult]::new('--filters', '--filters', [CompletionResultType]::ParameterName, 'filters')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--fd-output', '--fd-output', [CompletionResultType]::ParameterName, 'fd-output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;pseudo' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--start', '--start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('--increment', '--increment', [CompletionResultType]::ParameterName, 'increment')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;py' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--helper', '--helper', [CompletionResultType]::ParameterName, 'helper')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('filter', 'filter', [CompletionResultType]::ParameterValue, 'filter')
            [CompletionResult]::new('map', 'map', [CompletionResultType]::ParameterValue, 'map')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;py;filter' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--helper', '--helper', [CompletionResultType]::ParameterName, 'helper')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;py;map' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--helper', '--helper', [CompletionResultType]::ParameterName, 'helper')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;py;help' {
            [CompletionResult]::new('filter', 'filter', [CompletionResultType]::ParameterValue, 'filter')
            [CompletionResult]::new('map', 'map', [CompletionResultType]::ParameterValue, 'map')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;py;help;filter' {
            break
        }
        'qsv;py;help;map' {
            break
        }
        'qsv;py;help;help' {
            break
        }
        'qsv;rename' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--pairwise', '--pairwise', [CompletionResultType]::ParameterName, 'pairwise')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;replace' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--size-limit', '--size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--dfa-size-limit', '--dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--exact', '--exact', [CompletionResultType]::ParameterName, 'exact')
            [CompletionResult]::new('--literal', '--literal', [CompletionResultType]::ParameterName, 'literal')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unicode', '--unicode', [CompletionResultType]::ParameterName, 'unicode')
            [CompletionResult]::new('--not-one', '--not-one', [CompletionResultType]::ParameterName, 'not-one')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;reverse' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;safenames' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--mode', '--mode', [CompletionResultType]::ParameterName, 'mode')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--prefix', '--prefix', [CompletionResultType]::ParameterName, 'prefix')
            [CompletionResult]::new('--reserved', '--reserved', [CompletionResultType]::ParameterName, 'reserved')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sample' {
            [CompletionResult]::new('--cluster', '--cluster', [CompletionResultType]::ParameterName, 'cluster')
            [CompletionResult]::new('--stratified', '--stratified', [CompletionResultType]::ParameterName, 'stratified')
            [CompletionResult]::new('--ts-input-tz', '--ts-input-tz', [CompletionResultType]::ParameterName, 'ts-input-tz')
            [CompletionResult]::new('--systematic', '--systematic', [CompletionResultType]::ParameterName, 'systematic')
            [CompletionResult]::new('--max-size', '--max-size', [CompletionResultType]::ParameterName, 'max-size')
            [CompletionResult]::new('--rng', '--rng', [CompletionResultType]::ParameterName, 'rng')
            [CompletionResult]::new('--ts-adaptive', '--ts-adaptive', [CompletionResultType]::ParameterName, 'ts-adaptive')
            [CompletionResult]::new('--seed', '--seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('--ts-aggregate', '--ts-aggregate', [CompletionResultType]::ParameterName, 'ts-aggregate')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--weighted', '--weighted', [CompletionResultType]::ParameterName, 'weighted')
            [CompletionResult]::new('--timeseries', '--timeseries', [CompletionResultType]::ParameterName, 'timeseries')
            [CompletionResult]::new('--ts-start', '--ts-start', [CompletionResultType]::ParameterName, 'ts-start')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--ts-interval', '--ts-interval', [CompletionResultType]::ParameterName, 'ts-interval')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--ts-prefer-dmy', '--ts-prefer-dmy', [CompletionResultType]::ParameterName, 'ts-prefer-dmy')
            [CompletionResult]::new('--bernoulli', '--bernoulli', [CompletionResultType]::ParameterName, 'bernoulli')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;schema' {
            [CompletionResult]::new('--pattern-columns', '--pattern-columns', [CompletionResultType]::ParameterName, 'pattern-columns')
            [CompletionResult]::new('--enum-threshold', '--enum-threshold', [CompletionResultType]::ParameterName, 'enum-threshold')
            [CompletionResult]::new('--dates-whitelist', '--dates-whitelist', [CompletionResultType]::ParameterName, 'dates-whitelist')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--strict-dates', '--strict-dates', [CompletionResultType]::ParameterName, 'strict-dates')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--strict-formats', '--strict-formats', [CompletionResultType]::ParameterName, 'strict-formats')
            [CompletionResult]::new('--polars', '--polars', [CompletionResultType]::ParameterName, 'polars')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('--prefer-dmy', '--prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--stdout', '--stdout', [CompletionResultType]::ParameterName, 'stdout')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;search' {
            [CompletionResult]::new('--preview-match', '--preview-match', [CompletionResultType]::ParameterName, 'preview-match')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--size-limit', '--size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--flag', '--flag', [CompletionResultType]::ParameterName, 'flag')
            [CompletionResult]::new('--dfa-size-limit', '--dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unicode', '--unicode', [CompletionResultType]::ParameterName, 'unicode')
            [CompletionResult]::new('--not-one', '--not-one', [CompletionResultType]::ParameterName, 'not-one')
            [CompletionResult]::new('-Q', '-Q ', [CompletionResultType]::ParameterName, 'Q')
            [CompletionResult]::new('--quick', '--quick', [CompletionResultType]::ParameterName, 'quick')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--count', '--count', [CompletionResultType]::ParameterName, 'count')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--literal', '--literal', [CompletionResultType]::ParameterName, 'literal')
            [CompletionResult]::new('--exact', '--exact', [CompletionResultType]::ParameterName, 'exact')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--invert-match', '--invert-match', [CompletionResultType]::ParameterName, 'invert-match')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;searchset' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--unmatched-output', '--unmatched-output', [CompletionResultType]::ParameterName, 'unmatched-output')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--flag', '--flag', [CompletionResultType]::ParameterName, 'flag')
            [CompletionResult]::new('--size-limit', '--size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--dfa-size-limit', '--dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('-Q', '-Q ', [CompletionResultType]::ParameterName, 'Q')
            [CompletionResult]::new('--quick', '--quick', [CompletionResultType]::ParameterName, 'quick')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--count', '--count', [CompletionResultType]::ParameterName, 'count')
            [CompletionResult]::new('--flag-matches-only', '--flag-matches-only', [CompletionResultType]::ParameterName, 'flag-matches-only')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unicode', '--unicode', [CompletionResultType]::ParameterName, 'unicode')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--not-one', '--not-one', [CompletionResultType]::ParameterName, 'not-one')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--invert-match', '--invert-match', [CompletionResultType]::ParameterName, 'invert-match')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--literal', '--literal', [CompletionResultType]::ParameterName, 'literal')
            [CompletionResult]::new('--exact', '--exact', [CompletionResultType]::ParameterName, 'exact')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;select' {
            [CompletionResult]::new('--seed', '--seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'S')
            [CompletionResult]::new('--sort', '--sort', [CompletionResultType]::ParameterName, 'sort')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--random', '--random', [CompletionResultType]::ParameterName, 'random')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;slice' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--start', '--start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--end', '--end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--len', '--len', [CompletionResultType]::ParameterName, 'len')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--index', '--index', [CompletionResultType]::ParameterName, 'index')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--invert', '--invert', [CompletionResultType]::ParameterName, 'invert')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;snappy' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'check')
            [CompletionResult]::new('compress', 'compress', [CompletionResultType]::ParameterValue, 'compress')
            [CompletionResult]::new('decompress', 'decompress', [CompletionResultType]::ParameterValue, 'decompress')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'validate')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;snappy;check' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;snappy;compress' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;snappy;decompress' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;snappy;validate' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;snappy;help' {
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'check')
            [CompletionResult]::new('compress', 'compress', [CompletionResultType]::ParameterValue, 'compress')
            [CompletionResult]::new('decompress', 'decompress', [CompletionResultType]::ParameterValue, 'decompress')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'validate')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;snappy;help;check' {
            break
        }
        'qsv;snappy;help;compress' {
            break
        }
        'qsv;snappy;help;decompress' {
            break
        }
        'qsv;snappy;help;validate' {
            break
        }
        'qsv;snappy;help;help' {
            break
        }
        'qsv;sniff' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--sample', '--sample', [CompletionResultType]::ParameterName, 'sample')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--save-urlsample', '--save-urlsample', [CompletionResultType]::ParameterName, 'save-urlsample')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--quote', '--quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('--no-infer', '--no-infer', [CompletionResultType]::ParameterName, 'no-infer')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--prefer-dmy', '--prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--stats-types', '--stats-types', [CompletionResultType]::ParameterName, 'stats-types')
            [CompletionResult]::new('-Q', '-Q ', [CompletionResultType]::ParameterName, 'Q')
            [CompletionResult]::new('--quick', '--quick', [CompletionResultType]::ParameterName, 'quick')
            [CompletionResult]::new('--pretty-json', '--pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--harvest-mode', '--harvest-mode', [CompletionResultType]::ParameterName, 'harvest-mode')
            [CompletionResult]::new('--just-mime', '--just-mime', [CompletionResultType]::ParameterName, 'just-mime')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sort' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--seed', '--seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('--rng', '--rng', [CompletionResultType]::ParameterName, 'rng')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--natural', '--natural', [CompletionResultType]::ParameterName, 'natural')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unique', '--unique', [CompletionResultType]::ParameterName, 'unique')
            [CompletionResult]::new('--random', '--random', [CompletionResultType]::ParameterName, 'random')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--reverse', '--reverse', [CompletionResultType]::ParameterName, 'reverse')
            [CompletionResult]::new('--faster', '--faster', [CompletionResultType]::ParameterName, 'faster')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--numeric', '--numeric', [CompletionResultType]::ParameterName, 'numeric')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sortcheck' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--all', '--all', [CompletionResultType]::ParameterName, 'all')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--pretty-json', '--pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;split' {
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--kb-size', '--kb-size', [CompletionResultType]::ParameterName, 'kb-size')
            [CompletionResult]::new('--filter', '--filter', [CompletionResultType]::ParameterName, 'filter')
            [CompletionResult]::new('--filename', '--filename', [CompletionResultType]::ParameterName, 'filename')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--size', '--size', [CompletionResultType]::ParameterName, 'size')
            [CompletionResult]::new('--pad', '--pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--chunks', '--chunks', [CompletionResultType]::ParameterName, 'chunks')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--filter-ignore-errors', '--filter-ignore-errors', [CompletionResultType]::ParameterName, 'filter-ignore-errors')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--filter-cleanup', '--filter-cleanup', [CompletionResultType]::ParameterName, 'filter-cleanup')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sqlp' {
            [CompletionResult]::new('--compression', '--compression', [CompletionResultType]::ParameterName, 'compression')
            [CompletionResult]::new('--compress-level', '--compress-level', [CompletionResultType]::ParameterName, 'compress-level')
            [CompletionResult]::new('--wnull-value', '--wnull-value', [CompletionResultType]::ParameterName, 'wnull-value')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--datetime-format', '--datetime-format', [CompletionResultType]::ParameterName, 'datetime-format')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'format')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--date-format', '--date-format', [CompletionResultType]::ParameterName, 'date-format')
            [CompletionResult]::new('--time-format', '--time-format', [CompletionResultType]::ParameterName, 'time-format')
            [CompletionResult]::new('--rnull-values', '--rnull-values', [CompletionResultType]::ParameterName, 'rnull-values')
            [CompletionResult]::new('--float-precision', '--float-precision', [CompletionResultType]::ParameterName, 'float-precision')
            [CompletionResult]::new('--infer-len', '--infer-len', [CompletionResultType]::ParameterName, 'infer-len')
            [CompletionResult]::new('--streaming', '--streaming', [CompletionResultType]::ParameterName, 'streaming')
            [CompletionResult]::new('--truncate-ragged-lines', '--truncate-ragged-lines', [CompletionResultType]::ParameterName, 'truncate-ragged-lines')
            [CompletionResult]::new('--try-parsedates', '--try-parsedates', [CompletionResultType]::ParameterName, 'try-parsedates')
            [CompletionResult]::new('--cache-schema', '--cache-schema', [CompletionResultType]::ParameterName, 'cache-schema')
            [CompletionResult]::new('--statistics', '--statistics', [CompletionResultType]::ParameterName, 'statistics')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--low-memory', '--low-memory', [CompletionResultType]::ParameterName, 'low-memory')
            [CompletionResult]::new('--ignore-errors', '--ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('--no-optimizations', '--no-optimizations', [CompletionResultType]::ParameterName, 'no-optimizations')
            [CompletionResult]::new('--decimal-comma', '--decimal-comma', [CompletionResultType]::ParameterName, 'decimal-comma')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;stats' {
            [CompletionResult]::new('--round', '--round', [CompletionResultType]::ParameterName, 'round')
            [CompletionResult]::new('--dates-whitelist', '--dates-whitelist', [CompletionResultType]::ParameterName, 'dates-whitelist')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--boolean-patterns', '--boolean-patterns', [CompletionResultType]::ParameterName, 'boolean-patterns')
            [CompletionResult]::new('--weight', '--weight', [CompletionResultType]::ParameterName, 'weight')
            [CompletionResult]::new('--percentile-list', '--percentile-list', [CompletionResultType]::ParameterName, 'percentile-list')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--cache-threshold', '--cache-threshold', [CompletionResultType]::ParameterName, 'cache-threshold')
            [CompletionResult]::new('--nulls', '--nulls', [CompletionResultType]::ParameterName, 'nulls')
            [CompletionResult]::new('--typesonly', '--typesonly', [CompletionResultType]::ParameterName, 'typesonly')
            [CompletionResult]::new('--percentiles', '--percentiles', [CompletionResultType]::ParameterName, 'percentiles')
            [CompletionResult]::new('--mode', '--mode', [CompletionResultType]::ParameterName, 'mode')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-E', '-E ', [CompletionResultType]::ParameterName, 'E')
            [CompletionResult]::new('--everything', '--everything', [CompletionResultType]::ParameterName, 'everything')
            [CompletionResult]::new('--mad', '--mad', [CompletionResultType]::ParameterName, 'mad')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--infer-boolean', '--infer-boolean', [CompletionResultType]::ParameterName, 'infer-boolean')
            [CompletionResult]::new('--quartiles', '--quartiles', [CompletionResultType]::ParameterName, 'quartiles')
            [CompletionResult]::new('--median', '--median', [CompletionResultType]::ParameterName, 'median')
            [CompletionResult]::new('--prefer-dmy', '--prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--vis-whitespace', '--vis-whitespace', [CompletionResultType]::ParameterName, 'vis-whitespace')
            [CompletionResult]::new('--cardinality', '--cardinality', [CompletionResultType]::ParameterName, 'cardinality')
            [CompletionResult]::new('--infer-dates', '--infer-dates', [CompletionResultType]::ParameterName, 'infer-dates')
            [CompletionResult]::new('--stats-jsonl', '--stats-jsonl', [CompletionResultType]::ParameterName, 'stats-jsonl')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;table' {
            [CompletionResult]::new('-w', '-w', [CompletionResultType]::ParameterName, 'w')
            [CompletionResult]::new('--width', '--width', [CompletionResultType]::ParameterName, 'width')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--condense', '--condense', [CompletionResultType]::ParameterName, 'condense')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--pad', '--pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--align', '--align', [CompletionResultType]::ParameterName, 'align')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;template' {
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--outfilename', '--outfilename', [CompletionResultType]::ParameterName, 'outfilename')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--template-file', '--template-file', [CompletionResultType]::ParameterName, 'template-file')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--template', '--template', [CompletionResultType]::ParameterName, 'template')
            [CompletionResult]::new('--outsubdir-size', '--outsubdir-size', [CompletionResultType]::ParameterName, 'outsubdir-size')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--customfilter-error', '--customfilter-error', [CompletionResultType]::ParameterName, 'customfilter-error')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--globals-json', '--globals-json', [CompletionResultType]::ParameterName, 'globals-json')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('datapackage', 'datapackage', [CompletionResultType]::ParameterValue, 'datapackage')
            [CompletionResult]::new('ods', 'ods', [CompletionResultType]::ParameterValue, 'ods')
            [CompletionResult]::new('postgres', 'postgres', [CompletionResultType]::ParameterValue, 'postgres')
            [CompletionResult]::new('sqlite', 'sqlite', [CompletionResultType]::ParameterValue, 'sqlite')
            [CompletionResult]::new('xlsx', 'xlsx', [CompletionResultType]::ParameterValue, 'xlsx')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;to;datapackage' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to;ods' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to;postgres' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to;sqlite' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to;xlsx' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to;help' {
            [CompletionResult]::new('datapackage', 'datapackage', [CompletionResultType]::ParameterValue, 'datapackage')
            [CompletionResult]::new('ods', 'ods', [CompletionResultType]::ParameterValue, 'ods')
            [CompletionResult]::new('postgres', 'postgres', [CompletionResultType]::ParameterValue, 'postgres')
            [CompletionResult]::new('sqlite', 'sqlite', [CompletionResultType]::ParameterValue, 'sqlite')
            [CompletionResult]::new('xlsx', 'xlsx', [CompletionResultType]::ParameterValue, 'xlsx')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;to;help;datapackage' {
            break
        }
        'qsv;to;help;ods' {
            break
        }
        'qsv;to;help;postgres' {
            break
        }
        'qsv;to;help;sqlite' {
            break
        }
        'qsv;to;help;xlsx' {
            break
        }
        'qsv;to;help;help' {
            break
        }
        'qsv;tojsonl' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--no-boolean', '--no-boolean', [CompletionResultType]::ParameterName, 'no-boolean')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('--trim', '--trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;transpose' {
            [CompletionResult]::new('--long', '--long', [CompletionResultType]::ParameterName, 'long')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'm')
            [CompletionResult]::new('--multipass', '--multipass', [CompletionResultType]::ParameterName, 'multipass')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;validate' {
            [CompletionResult]::new('--dfa-size-limit', '--dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('--valid-output', '--valid-output', [CompletionResultType]::ParameterName, 'valid-output')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--backtrack-limit', '--backtrack-limit', [CompletionResultType]::ParameterName, 'backtrack-limit')
            [CompletionResult]::new('--email-min-subdomains', '--email-min-subdomains', [CompletionResultType]::ParameterName, 'email-min-subdomains')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--valid', '--valid', [CompletionResultType]::ParameterName, 'valid')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--invalid', '--invalid', [CompletionResultType]::ParameterName, 'invalid')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--size-limit', '--size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--email-required-tld', '--email-required-tld', [CompletionResultType]::ParameterName, 'email-required-tld')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--pretty-json', '--pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('--no-format-validation', '--no-format-validation', [CompletionResultType]::ParameterName, 'no-format-validation')
            [CompletionResult]::new('--fail-fast', '--fail-fast', [CompletionResultType]::ParameterName, 'fail-fast')
            [CompletionResult]::new('--email-domain-literal', '--email-domain-literal', [CompletionResultType]::ParameterName, 'email-domain-literal')
            [CompletionResult]::new('--email-display-text', '--email-display-text', [CompletionResultType]::ParameterName, 'email-display-text')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--fancy-regex', '--fancy-regex', [CompletionResultType]::ParameterName, 'fancy-regex')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--trim', '--trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('schema', 'schema', [CompletionResultType]::ParameterValue, 'schema')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;validate;schema' {
            [CompletionResult]::new('--dfa-size-limit', '--dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('--valid-output', '--valid-output', [CompletionResultType]::ParameterName, 'valid-output')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--backtrack-limit', '--backtrack-limit', [CompletionResultType]::ParameterName, 'backtrack-limit')
            [CompletionResult]::new('--email-min-subdomains', '--email-min-subdomains', [CompletionResultType]::ParameterName, 'email-min-subdomains')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--valid', '--valid', [CompletionResultType]::ParameterName, 'valid')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--invalid', '--invalid', [CompletionResultType]::ParameterName, 'invalid')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--size-limit', '--size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--email-required-tld', '--email-required-tld', [CompletionResultType]::ParameterName, 'email-required-tld')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--pretty-json', '--pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('--no-format-validation', '--no-format-validation', [CompletionResultType]::ParameterName, 'no-format-validation')
            [CompletionResult]::new('--fail-fast', '--fail-fast', [CompletionResultType]::ParameterName, 'fail-fast')
            [CompletionResult]::new('--email-domain-literal', '--email-domain-literal', [CompletionResultType]::ParameterName, 'email-domain-literal')
            [CompletionResult]::new('--email-display-text', '--email-display-text', [CompletionResultType]::ParameterName, 'email-display-text')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--fancy-regex', '--fancy-regex', [CompletionResultType]::ParameterName, 'fancy-regex')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--trim', '--trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;validate;help' {
            [CompletionResult]::new('schema', 'schema', [CompletionResultType]::ParameterValue, 'schema')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;validate;help;schema' {
            break
        }
        'qsv;validate;help;help' {
            break
        }
        'qsv;help' {
            [CompletionResult]::new('apply', 'apply', [CompletionResultType]::ParameterValue, 'apply')
            [CompletionResult]::new('behead', 'behead', [CompletionResultType]::ParameterValue, 'behead')
            [CompletionResult]::new('cat', 'cat', [CompletionResultType]::ParameterValue, 'cat')
            [CompletionResult]::new('clipboard', 'clipboard', [CompletionResultType]::ParameterValue, 'clipboard')
            [CompletionResult]::new('color', 'color', [CompletionResultType]::ParameterValue, 'color')
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'count')
            [CompletionResult]::new('datefmt', 'datefmt', [CompletionResultType]::ParameterValue, 'datefmt')
            [CompletionResult]::new('dedup', 'dedup', [CompletionResultType]::ParameterValue, 'dedup')
            [CompletionResult]::new('describegpt', 'describegpt', [CompletionResultType]::ParameterValue, 'describegpt')
            [CompletionResult]::new('diff', 'diff', [CompletionResultType]::ParameterValue, 'diff')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'edit')
            [CompletionResult]::new('enum', 'enum', [CompletionResultType]::ParameterValue, 'enum')
            [CompletionResult]::new('excel', 'excel', [CompletionResultType]::ParameterValue, 'excel')
            [CompletionResult]::new('exclude', 'exclude', [CompletionResultType]::ParameterValue, 'exclude')
            [CompletionResult]::new('explode', 'explode', [CompletionResultType]::ParameterValue, 'explode')
            [CompletionResult]::new('extdedup', 'extdedup', [CompletionResultType]::ParameterValue, 'extdedup')
            [CompletionResult]::new('extsort', 'extsort', [CompletionResultType]::ParameterValue, 'extsort')
            [CompletionResult]::new('fetch', 'fetch', [CompletionResultType]::ParameterValue, 'fetch')
            [CompletionResult]::new('fetchpost', 'fetchpost', [CompletionResultType]::ParameterValue, 'fetchpost')
            [CompletionResult]::new('fill', 'fill', [CompletionResultType]::ParameterValue, 'fill')
            [CompletionResult]::new('fixlengths', 'fixlengths', [CompletionResultType]::ParameterValue, 'fixlengths')
            [CompletionResult]::new('flatten', 'flatten', [CompletionResultType]::ParameterValue, 'flatten')
            [CompletionResult]::new('fmt', 'fmt', [CompletionResultType]::ParameterValue, 'fmt')
            [CompletionResult]::new('foreach', 'foreach', [CompletionResultType]::ParameterValue, 'foreach')
            [CompletionResult]::new('frequency', 'frequency', [CompletionResultType]::ParameterValue, 'frequency')
            [CompletionResult]::new('geocode', 'geocode', [CompletionResultType]::ParameterValue, 'geocode')
            [CompletionResult]::new('geoconvert', 'geoconvert', [CompletionResultType]::ParameterValue, 'geoconvert')
            [CompletionResult]::new('headers', 'headers', [CompletionResultType]::ParameterValue, 'headers')
            [CompletionResult]::new('index', 'index', [CompletionResultType]::ParameterValue, 'index')
            [CompletionResult]::new('input', 'input', [CompletionResultType]::ParameterValue, 'input')
            [CompletionResult]::new('join', 'join', [CompletionResultType]::ParameterValue, 'join')
            [CompletionResult]::new('joinp', 'joinp', [CompletionResultType]::ParameterValue, 'joinp')
            [CompletionResult]::new('json', 'json', [CompletionResultType]::ParameterValue, 'json')
            [CompletionResult]::new('jsonl', 'jsonl', [CompletionResultType]::ParameterValue, 'jsonl')
            [CompletionResult]::new('lens', 'lens', [CompletionResultType]::ParameterValue, 'lens')
            [CompletionResult]::new('luau', 'luau', [CompletionResultType]::ParameterValue, 'luau')
            [CompletionResult]::new('moarstats', 'moarstats', [CompletionResultType]::ParameterValue, 'moarstats')
            [CompletionResult]::new('partition', 'partition', [CompletionResultType]::ParameterValue, 'partition')
            [CompletionResult]::new('pivotp', 'pivotp', [CompletionResultType]::ParameterValue, 'pivotp')
            [CompletionResult]::new('pragmastat', 'pragmastat', [CompletionResultType]::ParameterValue, 'pragmastat')
            [CompletionResult]::new('pro', 'pro', [CompletionResultType]::ParameterValue, 'pro')
            [CompletionResult]::new('prompt', 'prompt', [CompletionResultType]::ParameterValue, 'prompt')
            [CompletionResult]::new('pseudo', 'pseudo', [CompletionResultType]::ParameterValue, 'pseudo')
            [CompletionResult]::new('py', 'py', [CompletionResultType]::ParameterValue, 'py')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'rename')
            [CompletionResult]::new('replace', 'replace', [CompletionResultType]::ParameterValue, 'replace')
            [CompletionResult]::new('reverse', 'reverse', [CompletionResultType]::ParameterValue, 'reverse')
            [CompletionResult]::new('safenames', 'safenames', [CompletionResultType]::ParameterValue, 'safenames')
            [CompletionResult]::new('sample', 'sample', [CompletionResultType]::ParameterValue, 'sample')
            [CompletionResult]::new('schema', 'schema', [CompletionResultType]::ParameterValue, 'schema')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'search')
            [CompletionResult]::new('searchset', 'searchset', [CompletionResultType]::ParameterValue, 'searchset')
            [CompletionResult]::new('select', 'select', [CompletionResultType]::ParameterValue, 'select')
            [CompletionResult]::new('slice', 'slice', [CompletionResultType]::ParameterValue, 'slice')
            [CompletionResult]::new('snappy', 'snappy', [CompletionResultType]::ParameterValue, 'snappy')
            [CompletionResult]::new('sniff', 'sniff', [CompletionResultType]::ParameterValue, 'sniff')
            [CompletionResult]::new('sort', 'sort', [CompletionResultType]::ParameterValue, 'sort')
            [CompletionResult]::new('sortcheck', 'sortcheck', [CompletionResultType]::ParameterValue, 'sortcheck')
            [CompletionResult]::new('split', 'split', [CompletionResultType]::ParameterValue, 'split')
            [CompletionResult]::new('sqlp', 'sqlp', [CompletionResultType]::ParameterValue, 'sqlp')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'stats')
            [CompletionResult]::new('table', 'table', [CompletionResultType]::ParameterValue, 'table')
            [CompletionResult]::new('template', 'template', [CompletionResultType]::ParameterValue, 'template')
            [CompletionResult]::new('to', 'to', [CompletionResultType]::ParameterValue, 'to')
            [CompletionResult]::new('tojsonl', 'tojsonl', [CompletionResultType]::ParameterValue, 'tojsonl')
            [CompletionResult]::new('transpose', 'transpose', [CompletionResultType]::ParameterValue, 'transpose')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'validate')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;help;apply' {
            [CompletionResult]::new('calcconv', 'calcconv', [CompletionResultType]::ParameterValue, 'calcconv')
            [CompletionResult]::new('dynfmt', 'dynfmt', [CompletionResultType]::ParameterValue, 'dynfmt')
            [CompletionResult]::new('emptyreplace', 'emptyreplace', [CompletionResultType]::ParameterValue, 'emptyreplace')
            [CompletionResult]::new('operations', 'operations', [CompletionResultType]::ParameterValue, 'operations')
            break
        }
        'qsv;help;apply;calcconv' {
            break
        }
        'qsv;help;apply;dynfmt' {
            break
        }
        'qsv;help;apply;emptyreplace' {
            break
        }
        'qsv;help;apply;operations' {
            break
        }
        'qsv;help;behead' {
            break
        }
        'qsv;help;cat' {
            [CompletionResult]::new('columns', 'columns', [CompletionResultType]::ParameterValue, 'columns')
            [CompletionResult]::new('rows', 'rows', [CompletionResultType]::ParameterValue, 'rows')
            [CompletionResult]::new('rowskey', 'rowskey', [CompletionResultType]::ParameterValue, 'rowskey')
            break
        }
        'qsv;help;cat;columns' {
            break
        }
        'qsv;help;cat;rows' {
            break
        }
        'qsv;help;cat;rowskey' {
            break
        }
        'qsv;help;clipboard' {
            break
        }
        'qsv;help;color' {
            break
        }
        'qsv;help;count' {
            break
        }
        'qsv;help;datefmt' {
            break
        }
        'qsv;help;dedup' {
            break
        }
        'qsv;help;describegpt' {
            break
        }
        'qsv;help;diff' {
            break
        }
        'qsv;help;edit' {
            break
        }
        'qsv;help;enum' {
            break
        }
        'qsv;help;excel' {
            break
        }
        'qsv;help;exclude' {
            break
        }
        'qsv;help;explode' {
            break
        }
        'qsv;help;extdedup' {
            break
        }
        'qsv;help;extsort' {
            break
        }
        'qsv;help;fetch' {
            break
        }
        'qsv;help;fetchpost' {
            break
        }
        'qsv;help;fill' {
            break
        }
        'qsv;help;fixlengths' {
            break
        }
        'qsv;help;flatten' {
            break
        }
        'qsv;help;fmt' {
            break
        }
        'qsv;help;foreach' {
            break
        }
        'qsv;help;frequency' {
            break
        }
        'qsv;help;geocode' {
            [CompletionResult]::new('countryinfo', 'countryinfo', [CompletionResultType]::ParameterValue, 'countryinfo')
            [CompletionResult]::new('countryinfonow', 'countryinfonow', [CompletionResultType]::ParameterValue, 'countryinfonow')
            [CompletionResult]::new('index-check', 'index-check', [CompletionResultType]::ParameterValue, 'index-check')
            [CompletionResult]::new('index-load', 'index-load', [CompletionResultType]::ParameterValue, 'index-load')
            [CompletionResult]::new('index-reset', 'index-reset', [CompletionResultType]::ParameterValue, 'index-reset')
            [CompletionResult]::new('index-update', 'index-update', [CompletionResultType]::ParameterValue, 'index-update')
            [CompletionResult]::new('iplookup', 'iplookup', [CompletionResultType]::ParameterValue, 'iplookup')
            [CompletionResult]::new('iplookupnow', 'iplookupnow', [CompletionResultType]::ParameterValue, 'iplookupnow')
            [CompletionResult]::new('reverse', 'reverse', [CompletionResultType]::ParameterValue, 'reverse')
            [CompletionResult]::new('reversenow', 'reversenow', [CompletionResultType]::ParameterValue, 'reversenow')
            [CompletionResult]::new('suggest', 'suggest', [CompletionResultType]::ParameterValue, 'suggest')
            [CompletionResult]::new('suggestnow', 'suggestnow', [CompletionResultType]::ParameterValue, 'suggestnow')
            break
        }
        'qsv;help;geocode;countryinfo' {
            break
        }
        'qsv;help;geocode;countryinfonow' {
            break
        }
        'qsv;help;geocode;index-check' {
            break
        }
        'qsv;help;geocode;index-load' {
            break
        }
        'qsv;help;geocode;index-reset' {
            break
        }
        'qsv;help;geocode;index-update' {
            break
        }
        'qsv;help;geocode;iplookup' {
            break
        }
        'qsv;help;geocode;iplookupnow' {
            break
        }
        'qsv;help;geocode;reverse' {
            break
        }
        'qsv;help;geocode;reversenow' {
            break
        }
        'qsv;help;geocode;suggest' {
            break
        }
        'qsv;help;geocode;suggestnow' {
            break
        }
        'qsv;help;geoconvert' {
            break
        }
        'qsv;help;headers' {
            break
        }
        'qsv;help;index' {
            break
        }
        'qsv;help;input' {
            break
        }
        'qsv;help;join' {
            break
        }
        'qsv;help;joinp' {
            break
        }
        'qsv;help;json' {
            break
        }
        'qsv;help;jsonl' {
            break
        }
        'qsv;help;lens' {
            break
        }
        'qsv;help;luau' {
            [CompletionResult]::new('filter', 'filter', [CompletionResultType]::ParameterValue, 'filter')
            [CompletionResult]::new('map', 'map', [CompletionResultType]::ParameterValue, 'map')
            break
        }
        'qsv;help;luau;filter' {
            break
        }
        'qsv;help;luau;map' {
            break
        }
        'qsv;help;moarstats' {
            break
        }
        'qsv;help;partition' {
            break
        }
        'qsv;help;pivotp' {
            break
        }
        'qsv;help;pragmastat' {
            break
        }
        'qsv;help;pro' {
            [CompletionResult]::new('lens', 'lens', [CompletionResultType]::ParameterValue, 'lens')
            [CompletionResult]::new('workflow', 'workflow', [CompletionResultType]::ParameterValue, 'workflow')
            break
        }
        'qsv;help;pro;lens' {
            break
        }
        'qsv;help;pro;workflow' {
            break
        }
        'qsv;help;prompt' {
            break
        }
        'qsv;help;pseudo' {
            break
        }
        'qsv;help;py' {
            [CompletionResult]::new('filter', 'filter', [CompletionResultType]::ParameterValue, 'filter')
            [CompletionResult]::new('map', 'map', [CompletionResultType]::ParameterValue, 'map')
            break
        }
        'qsv;help;py;filter' {
            break
        }
        'qsv;help;py;map' {
            break
        }
        'qsv;help;rename' {
            break
        }
        'qsv;help;replace' {
            break
        }
        'qsv;help;reverse' {
            break
        }
        'qsv;help;safenames' {
            break
        }
        'qsv;help;sample' {
            break
        }
        'qsv;help;schema' {
            break
        }
        'qsv;help;search' {
            break
        }
        'qsv;help;searchset' {
            break
        }
        'qsv;help;select' {
            break
        }
        'qsv;help;slice' {
            break
        }
        'qsv;help;snappy' {
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'check')
            [CompletionResult]::new('compress', 'compress', [CompletionResultType]::ParameterValue, 'compress')
            [CompletionResult]::new('decompress', 'decompress', [CompletionResultType]::ParameterValue, 'decompress')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'validate')
            break
        }
        'qsv;help;snappy;check' {
            break
        }
        'qsv;help;snappy;compress' {
            break
        }
        'qsv;help;snappy;decompress' {
            break
        }
        'qsv;help;snappy;validate' {
            break
        }
        'qsv;help;sniff' {
            break
        }
        'qsv;help;sort' {
            break
        }
        'qsv;help;sortcheck' {
            break
        }
        'qsv;help;split' {
            break
        }
        'qsv;help;sqlp' {
            break
        }
        'qsv;help;stats' {
            break
        }
        'qsv;help;table' {
            break
        }
        'qsv;help;template' {
            break
        }
        'qsv;help;to' {
            [CompletionResult]::new('datapackage', 'datapackage', [CompletionResultType]::ParameterValue, 'datapackage')
            [CompletionResult]::new('ods', 'ods', [CompletionResultType]::ParameterValue, 'ods')
            [CompletionResult]::new('postgres', 'postgres', [CompletionResultType]::ParameterValue, 'postgres')
            [CompletionResult]::new('sqlite', 'sqlite', [CompletionResultType]::ParameterValue, 'sqlite')
            [CompletionResult]::new('xlsx', 'xlsx', [CompletionResultType]::ParameterValue, 'xlsx')
            break
        }
        'qsv;help;to;datapackage' {
            break
        }
        'qsv;help;to;ods' {
            break
        }
        'qsv;help;to;postgres' {
            break
        }
        'qsv;help;to;sqlite' {
            break
        }
        'qsv;help;to;xlsx' {
            break
        }
        'qsv;help;tojsonl' {
            break
        }
        'qsv;help;transpose' {
            break
        }
        'qsv;help;validate' {
            [CompletionResult]::new('schema', 'schema', [CompletionResultType]::ParameterValue, 'schema')
            break
        }
        'qsv;help;validate;schema' {
            break
        }
        'qsv;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
