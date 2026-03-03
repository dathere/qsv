
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
            [CompletionResult]::new('log', 'log', [CompletionResultType]::ParameterValue, 'log')
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
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--replacement', '--replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
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
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--replacement', '--replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;dynfmt' {
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--comparand', '--comparand', [CompletionResultType]::ParameterName, 'comparand')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--replacement', '--replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;emptyreplace' {
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--comparand', '--comparand', [CompletionResultType]::ParameterName, 'comparand')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--replacement', '--replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;operations' {
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--comparand', '--comparand', [CompletionResultType]::ParameterName, 'comparand')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--replacement', '--replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
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
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--group', '--group', [CompletionResultType]::ParameterName, 'group')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--group-name', '--group-name', [CompletionResultType]::ParameterName, 'group-name')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
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
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--group', '--group', [CompletionResultType]::ParameterName, 'group')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--group-name', '--group-name', [CompletionResultType]::ParameterName, 'group-name')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
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
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--group', '--group', [CompletionResultType]::ParameterName, 'group')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--group-name', '--group-name', [CompletionResultType]::ParameterName, 'group-name')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
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
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--group', '--group', [CompletionResultType]::ParameterName, 'group')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--group-name', '--group-name', [CompletionResultType]::ParameterName, 'group-name')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
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
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--title', '--title', [CompletionResultType]::ParameterName, 'title')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
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
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--low-memory', '--low-memory', [CompletionResultType]::ParameterName, 'low-memory')
            [CompletionResult]::new('--no-polars', '--no-polars', [CompletionResultType]::ParameterName, 'no-polars')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--width-no-delims', '--width-no-delims', [CompletionResultType]::ParameterName, 'width-no-delims')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--human-readable', '--human-readable', [CompletionResultType]::ParameterName, 'human-readable')
            [CompletionResult]::new('--width', '--width', [CompletionResultType]::ParameterName, 'width')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--flexible', '--flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;datefmt' {
            [CompletionResult]::new('--default-tz', '--default-tz', [CompletionResultType]::ParameterName, 'default-tz')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--output-tz', '--output-tz', [CompletionResultType]::ParameterName, 'output-tz')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--input-tz', '--input-tz', [CompletionResultType]::ParameterName, 'input-tz')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--ts-resolution', '--ts-resolution', [CompletionResultType]::ParameterName, 'ts-resolution')
            [CompletionResult]::new('--prefer-dmy', '--prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--keep-zero-time', '--keep-zero-time', [CompletionResultType]::ParameterName, 'keep-zero-time')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--zulu', '--zulu', [CompletionResultType]::ParameterName, 'zulu')
            [CompletionResult]::new('--utc', '--utc', [CompletionResultType]::ParameterName, 'utc')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;dedup' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'D')
            [CompletionResult]::new('--dupes-output', '--dupes-output', [CompletionResultType]::ParameterName, 'dupes-output')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--sorted', '--sorted', [CompletionResultType]::ParameterName, 'sorted')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--numeric', '--numeric', [CompletionResultType]::ParameterName, 'numeric')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--human-readable', '--human-readable', [CompletionResultType]::ParameterName, 'human-readable')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;describegpt' {
            [CompletionResult]::new('--num-examples', '--num-examples', [CompletionResultType]::ParameterName, 'num-examples')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--enum-threshold', '--enum-threshold', [CompletionResultType]::ParameterName, 'enum-threshold')
            [CompletionResult]::new('--prompt-file', '--prompt-file', [CompletionResultType]::ParameterName, 'prompt-file')
            [CompletionResult]::new('--addl-cols-list', '--addl-cols-list', [CompletionResultType]::ParameterName, 'addl-cols-list')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--base-url', '--base-url', [CompletionResultType]::ParameterName, 'base-url')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'm')
            [CompletionResult]::new('--model', '--model', [CompletionResultType]::ParameterName, 'model')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--max-tokens', '--max-tokens', [CompletionResultType]::ParameterName, 'max-tokens')
            [CompletionResult]::new('--export-prompt', '--export-prompt', [CompletionResultType]::ParameterName, 'export-prompt')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--addl-props', '--addl-props', [CompletionResultType]::ParameterName, 'addl-props')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--api-key', '--api-key', [CompletionResultType]::ParameterName, 'api-key')
            [CompletionResult]::new('--num-tags', '--num-tags', [CompletionResultType]::ParameterName, 'num-tags')
            [CompletionResult]::new('--sql-results', '--sql-results', [CompletionResultType]::ParameterName, 'sql-results')
            [CompletionResult]::new('--truncate-str', '--truncate-str', [CompletionResultType]::ParameterName, 'truncate-str')
            [CompletionResult]::new('--stats-options', '--stats-options', [CompletionResultType]::ParameterName, 'stats-options')
            [CompletionResult]::new('--freq-options', '--freq-options', [CompletionResultType]::ParameterName, 'freq-options')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--sample-size', '--sample-size', [CompletionResultType]::ParameterName, 'sample-size')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'format')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('--disk-cache-dir', '--disk-cache-dir', [CompletionResultType]::ParameterName, 'disk-cache-dir')
            [CompletionResult]::new('--session', '--session', [CompletionResultType]::ParameterName, 'session')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--prompt', '--prompt', [CompletionResultType]::ParameterName, 'prompt')
            [CompletionResult]::new('--session-len', '--session-len', [CompletionResultType]::ParameterName, 'session-len')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--tag-vocab', '--tag-vocab', [CompletionResultType]::ParameterName, 'tag-vocab')
            [CompletionResult]::new('--addl-cols', '--addl-cols', [CompletionResultType]::ParameterName, 'addl-cols')
            [CompletionResult]::new('--flush-cache', '--flush-cache', [CompletionResultType]::ParameterName, 'flush-cache')
            [CompletionResult]::new('--fresh', '--fresh', [CompletionResultType]::ParameterName, 'fresh')
            [CompletionResult]::new('--fewshot-examples', '--fewshot-examples', [CompletionResultType]::ParameterName, 'fewshot-examples')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all', '--all', [CompletionResultType]::ParameterName, 'all')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--description', '--description', [CompletionResultType]::ParameterName, 'description')
            [CompletionResult]::new('--forget', '--forget', [CompletionResultType]::ParameterName, 'forget')
            [CompletionResult]::new('--tags', '--tags', [CompletionResultType]::ParameterName, 'tags')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'no-cache')
            [CompletionResult]::new('--redis-cache', '--redis-cache', [CompletionResultType]::ParameterName, 'redis-cache')
            [CompletionResult]::new('--dictionary', '--dictionary', [CompletionResultType]::ParameterName, 'dictionary')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;diff' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter-right', '--delimiter-right', [CompletionResultType]::ParameterName, 'delimiter-right')
            [CompletionResult]::new('--sort-columns', '--sort-columns', [CompletionResultType]::ParameterName, 'sort-columns')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--key', '--key', [CompletionResultType]::ParameterName, 'key')
            [CompletionResult]::new('--delimiter-output', '--delimiter-output', [CompletionResultType]::ParameterName, 'delimiter-output')
            [CompletionResult]::new('--delimiter-left', '--delimiter-left', [CompletionResultType]::ParameterName, 'delimiter-left')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--no-headers-output', '--no-headers-output', [CompletionResultType]::ParameterName, 'no-headers-output')
            [CompletionResult]::new('--drop-equal-fields', '--drop-equal-fields', [CompletionResultType]::ParameterName, 'drop-equal-fields')
            [CompletionResult]::new('--no-headers-left', '--no-headers-left', [CompletionResultType]::ParameterName, 'no-headers-left')
            [CompletionResult]::new('--no-headers-right', '--no-headers-right', [CompletionResultType]::ParameterName, 'no-headers-right')
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
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--constant', '--constant', [CompletionResultType]::ParameterName, 'constant')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--increment', '--increment', [CompletionResultType]::ParameterName, 'increment')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--copy', '--copy', [CompletionResultType]::ParameterName, 'copy')
            [CompletionResult]::new('--start', '--start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--uuid4', '--uuid4', [CompletionResultType]::ParameterName, 'uuid4')
            [CompletionResult]::new('--uuid7', '--uuid7', [CompletionResultType]::ParameterName, 'uuid7')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;excel' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--table', '--table', [CompletionResultType]::ParameterName, 'table')
            [CompletionResult]::new('--date-format', '--date-format', [CompletionResultType]::ParameterName, 'date-format')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--cell', '--cell', [CompletionResultType]::ParameterName, 'cell')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--sheet', '--sheet', [CompletionResultType]::ParameterName, 'sheet')
            [CompletionResult]::new('--header-row', '--header-row', [CompletionResultType]::ParameterName, 'header-row')
            [CompletionResult]::new('--metadata', '--metadata', [CompletionResultType]::ParameterName, 'metadata')
            [CompletionResult]::new('--range', '--range', [CompletionResultType]::ParameterName, 'range')
            [CompletionResult]::new('--error-format', '--error-format', [CompletionResultType]::ParameterName, 'error-format')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--flexible', '--flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('--keep-zero-time', '--keep-zero-time', [CompletionResultType]::ParameterName, 'keep-zero-time')
            [CompletionResult]::new('--trim', '--trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;exclude' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--invert', '--invert', [CompletionResultType]::ParameterName, 'invert')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;explode' {
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;extdedup' {
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'D')
            [CompletionResult]::new('--dupes-output', '--dupes-output', [CompletionResultType]::ParameterName, 'dupes-output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--memory-limit', '--memory-limit', [CompletionResultType]::ParameterName, 'memory-limit')
            [CompletionResult]::new('--temp-dir', '--temp-dir', [CompletionResultType]::ParameterName, 'temp-dir')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--no-output', '--no-output', [CompletionResultType]::ParameterName, 'no-output')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--human-readable', '--human-readable', [CompletionResultType]::ParameterName, 'human-readable')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;extsort' {
            [CompletionResult]::new('--tmp-dir', '--tmp-dir', [CompletionResultType]::ParameterName, 'tmp-dir')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memory-limit', '--memory-limit', [CompletionResultType]::ParameterName, 'memory-limit')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--reverse', '--reverse', [CompletionResultType]::ParameterName, 'reverse')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fetch' {
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--url-template', '--url-template', [CompletionResultType]::ParameterName, 'url-template')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--jaqfile', '--jaqfile', [CompletionResultType]::ParameterName, 'jaqfile')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--http-header', '--http-header', [CompletionResultType]::ParameterName, 'http-header')
            [CompletionResult]::new('--mem-cache-size', '--mem-cache-size', [CompletionResultType]::ParameterName, 'mem-cache-size')
            [CompletionResult]::new('--max-errors', '--max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('--disk-cache-dir', '--disk-cache-dir', [CompletionResultType]::ParameterName, 'disk-cache-dir')
            [CompletionResult]::new('--jaq', '--jaq', [CompletionResultType]::ParameterName, 'jaq')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--report', '--report', [CompletionResultType]::ParameterName, 'report')
            [CompletionResult]::new('--rate-limit', '--rate-limit', [CompletionResultType]::ParameterName, 'rate-limit')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--max-retries', '--max-retries', [CompletionResultType]::ParameterName, 'max-retries')
            [CompletionResult]::new('--flush-cache', '--flush-cache', [CompletionResultType]::ParameterName, 'flush-cache')
            [CompletionResult]::new('--cache-error', '--cache-error', [CompletionResultType]::ParameterName, 'cache-error')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--pretty', '--pretty', [CompletionResultType]::ParameterName, 'pretty')
            [CompletionResult]::new('--disk-cache', '--disk-cache', [CompletionResultType]::ParameterName, 'disk-cache')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'no-cache')
            [CompletionResult]::new('--cookies', '--cookies', [CompletionResultType]::ParameterName, 'cookies')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--redis-cache', '--redis-cache', [CompletionResultType]::ParameterName, 'redis-cache')
            [CompletionResult]::new('--store-error', '--store-error', [CompletionResultType]::ParameterName, 'store-error')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fetchpost' {
            [CompletionResult]::new('--content-type', '--content-type', [CompletionResultType]::ParameterName, 'content-type')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--payload-tpl', '--payload-tpl', [CompletionResultType]::ParameterName, 'payload-tpl')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--jaqfile', '--jaqfile', [CompletionResultType]::ParameterName, 'jaqfile')
            [CompletionResult]::new('--report', '--report', [CompletionResultType]::ParameterName, 'report')
            [CompletionResult]::new('--max-retries', '--max-retries', [CompletionResultType]::ParameterName, 'max-retries')
            [CompletionResult]::new('--jaq', '--jaq', [CompletionResultType]::ParameterName, 'jaq')
            [CompletionResult]::new('--max-errors', '--max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--globals-json', '--globals-json', [CompletionResultType]::ParameterName, 'globals-json')
            [CompletionResult]::new('--rate-limit', '--rate-limit', [CompletionResultType]::ParameterName, 'rate-limit')
            [CompletionResult]::new('--disk-cache-dir', '--disk-cache-dir', [CompletionResultType]::ParameterName, 'disk-cache-dir')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--http-header', '--http-header', [CompletionResultType]::ParameterName, 'http-header')
            [CompletionResult]::new('--mem-cache-size', '--mem-cache-size', [CompletionResultType]::ParameterName, 'mem-cache-size')
            [CompletionResult]::new('--cache-error', '--cache-error', [CompletionResultType]::ParameterName, 'cache-error')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--redis-cache', '--redis-cache', [CompletionResultType]::ParameterName, 'redis-cache')
            [CompletionResult]::new('--disk-cache', '--disk-cache', [CompletionResultType]::ParameterName, 'disk-cache')
            [CompletionResult]::new('--cookies', '--cookies', [CompletionResultType]::ParameterName, 'cookies')
            [CompletionResult]::new('--flush-cache', '--flush-cache', [CompletionResultType]::ParameterName, 'flush-cache')
            [CompletionResult]::new('--compress', '--compress', [CompletionResultType]::ParameterName, 'compress')
            [CompletionResult]::new('--pretty', '--pretty', [CompletionResultType]::ParameterName, 'pretty')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'no-cache')
            [CompletionResult]::new('--store-error', '--store-error', [CompletionResultType]::ParameterName, 'store-error')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fill' {
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--default', '--default', [CompletionResultType]::ParameterName, 'default')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--groupby', '--groupby', [CompletionResultType]::ParameterName, 'groupby')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--first', '--first', [CompletionResultType]::ParameterName, 'first')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--backfill', '--backfill', [CompletionResultType]::ParameterName, 'backfill')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fixlengths' {
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--length', '--length', [CompletionResultType]::ParameterName, 'length')
            [CompletionResult]::new('--quote', '--quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--escape', '--escape', [CompletionResultType]::ParameterName, 'escape')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--insert', '--insert', [CompletionResultType]::ParameterName, 'insert')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--remove-empty', '--remove-empty', [CompletionResultType]::ParameterName, 'remove-empty')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;flatten' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--condense', '--condense', [CompletionResultType]::ParameterName, 'condense')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--field-separator', '--field-separator', [CompletionResultType]::ParameterName, 'field-separator')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fmt' {
            [CompletionResult]::new('--quote', '--quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--out-delimiter', '--out-delimiter', [CompletionResultType]::ParameterName, 'out-delimiter')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--escape', '--escape', [CompletionResultType]::ParameterName, 'escape')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--crlf', '--crlf', [CompletionResultType]::ParameterName, 'crlf')
            [CompletionResult]::new('--quote-never', '--quote-never', [CompletionResultType]::ParameterName, 'quote-never')
            [CompletionResult]::new('--ascii', '--ascii', [CompletionResultType]::ParameterName, 'ascii')
            [CompletionResult]::new('--quote-always', '--quote-always', [CompletionResultType]::ParameterName, 'quote-always')
            [CompletionResult]::new('--no-final-newline', '--no-final-newline', [CompletionResultType]::ParameterName, 'no-final-newline')
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
            [CompletionResult]::new('--high-card-pct', '--high-card-pct', [CompletionResultType]::ParameterName, 'high-card-pct')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--no-float', '--no-float', [CompletionResultType]::ParameterName, 'no-float')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rank-strategy', '--rank-strategy', [CompletionResultType]::ParameterName, 'rank-strategy')
            [CompletionResult]::new('--lmt-threshold', '--lmt-threshold', [CompletionResultType]::ParameterName, 'lmt-threshold')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--other-text', '--other-text', [CompletionResultType]::ParameterName, 'other-text')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unq-limit', '--unq-limit', [CompletionResultType]::ParameterName, 'unq-limit')
            [CompletionResult]::new('--all-unique-text', '--all-unique-text', [CompletionResultType]::ParameterName, 'all-unique-text')
            [CompletionResult]::new('--weight', '--weight', [CompletionResultType]::ParameterName, 'weight')
            [CompletionResult]::new('--stats-filter', '--stats-filter', [CompletionResultType]::ParameterName, 'stats-filter')
            [CompletionResult]::new('--pct-dec-places', '--pct-dec-places', [CompletionResultType]::ParameterName, 'pct-dec-places')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'limit')
            [CompletionResult]::new('--null-text', '--null-text', [CompletionResultType]::ParameterName, 'null-text')
            [CompletionResult]::new('--high-card-threshold', '--high-card-threshold', [CompletionResultType]::ParameterName, 'high-card-threshold')
            [CompletionResult]::new('--vis-whitespace', '--vis-whitespace', [CompletionResultType]::ParameterName, 'vis-whitespace')
            [CompletionResult]::new('--pretty-json', '--pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('--frequency-jsonl', '--frequency-jsonl', [CompletionResultType]::ParameterName, 'frequency-jsonl')
            [CompletionResult]::new('--pct-nulls', '--pct-nulls', [CompletionResultType]::ParameterName, 'pct-nulls')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--other-sorted', '--other-sorted', [CompletionResultType]::ParameterName, 'other-sorted')
            [CompletionResult]::new('--no-stats', '--no-stats', [CompletionResultType]::ParameterName, 'no-stats')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('--null-sorted', '--null-sorted', [CompletionResultType]::ParameterName, 'null-sorted')
            [CompletionResult]::new('--no-trim', '--no-trim', [CompletionResultType]::ParameterName, 'no-trim')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--asc', '--asc', [CompletionResultType]::ParameterName, 'asc')
            [CompletionResult]::new('--no-nulls', '--no-nulls', [CompletionResultType]::ParameterName, 'no-nulls')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--no-other', '--no-other', [CompletionResultType]::ParameterName, 'no-other')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--toon', '--toon', [CompletionResultType]::ParameterName, 'toon')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
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
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;countryinfonow' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;index-check' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;index-load' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;index-reset' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;index-update' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;iplookup' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;iplookupnow' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;reverse' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;reversenow' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;suggest' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode;suggestnow' {
            [CompletionResult]::new('--country', '--country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--admin1', '--admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--language', '--language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--k_weight', '--k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--new-column', '--new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--languages', '--languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--invalid-result', '--invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--min-score', '--min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--rename', '--rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--cities-url', '--cities-url', [CompletionResultType]::ParameterName, 'cities-url')
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
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--geometry', '--geometry', [CompletionResultType]::ParameterName, 'geometry')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--max-length', '--max-length', [CompletionResultType]::ParameterName, 'max-length')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-x', '-x', [CompletionResultType]::ParameterName, 'x')
            [CompletionResult]::new('--longitude', '--longitude', [CompletionResultType]::ParameterName, 'longitude')
            [CompletionResult]::new('-y', '-y', [CompletionResultType]::ParameterName, 'y')
            [CompletionResult]::new('--latitude', '--latitude', [CompletionResultType]::ParameterName, 'latitude')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;headers' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--trim', '--trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('--intersect', '--intersect', [CompletionResultType]::ParameterName, 'intersect')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--just-names', '--just-names', [CompletionResultType]::ParameterName, 'just-names')
            [CompletionResult]::new('-J', '-J ', [CompletionResultType]::ParameterName, 'J')
            [CompletionResult]::new('--just-count', '--just-count', [CompletionResultType]::ParameterName, 'just-count')
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
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--quote-style', '--quote-style', [CompletionResultType]::ParameterName, 'quote-style')
            [CompletionResult]::new('--escape', '--escape', [CompletionResultType]::ParameterName, 'escape')
            [CompletionResult]::new('--skip-lastlines', '--skip-lastlines', [CompletionResultType]::ParameterName, 'skip-lastlines')
            [CompletionResult]::new('--skip-lines', '--skip-lines', [CompletionResultType]::ParameterName, 'skip-lines')
            [CompletionResult]::new('--encoding-errors', '--encoding-errors', [CompletionResultType]::ParameterName, 'encoding-errors')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--comment', '--comment', [CompletionResultType]::ParameterName, 'comment')
            [CompletionResult]::new('--quote', '--quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('--auto-skip', '--auto-skip', [CompletionResultType]::ParameterName, 'auto-skip')
            [CompletionResult]::new('--trim-headers', '--trim-headers', [CompletionResultType]::ParameterName, 'trim-headers')
            [CompletionResult]::new('--trim-fields', '--trim-fields', [CompletionResultType]::ParameterName, 'trim-fields')
            [CompletionResult]::new('--no-quoting', '--no-quoting', [CompletionResultType]::ParameterName, 'no-quoting')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;join' {
            [CompletionResult]::new('--keys-output', '--keys-output', [CompletionResultType]::ParameterName, 'keys-output')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--left-semi', '--left-semi', [CompletionResultType]::ParameterName, 'left-semi')
            [CompletionResult]::new('--right-semi', '--right-semi', [CompletionResultType]::ParameterName, 'right-semi')
            [CompletionResult]::new('--left-anti', '--left-anti', [CompletionResultType]::ParameterName, 'left-anti')
            [CompletionResult]::new('--cross', '--cross', [CompletionResultType]::ParameterName, 'cross')
            [CompletionResult]::new('--left', '--left', [CompletionResultType]::ParameterName, 'left')
            [CompletionResult]::new('--right', '--right', [CompletionResultType]::ParameterName, 'right')
            [CompletionResult]::new('-z', '-z', [CompletionResultType]::ParameterName, 'z')
            [CompletionResult]::new('--ignore-leading-zeros', '--ignore-leading-zeros', [CompletionResultType]::ParameterName, 'ignore-leading-zeros')
            [CompletionResult]::new('--full', '--full', [CompletionResultType]::ParameterName, 'full')
            [CompletionResult]::new('--nulls', '--nulls', [CompletionResultType]::ParameterName, 'nulls')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--right-anti', '--right-anti', [CompletionResultType]::ParameterName, 'right-anti')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;joinp' {
            [CompletionResult]::new('--non-equi', '--non-equi', [CompletionResultType]::ParameterName, 'non-equi')
            [CompletionResult]::new('--datetime-format', '--datetime-format', [CompletionResultType]::ParameterName, 'datetime-format')
            [CompletionResult]::new('--right_by', '--right_by', [CompletionResultType]::ParameterName, 'right_by')
            [CompletionResult]::new('--filter-right', '--filter-right', [CompletionResultType]::ParameterName, 'filter-right')
            [CompletionResult]::new('--left_by', '--left_by', [CompletionResultType]::ParameterName, 'left_by')
            [CompletionResult]::new('--null-value', '--null-value', [CompletionResultType]::ParameterName, 'null-value')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--filter-left', '--filter-left', [CompletionResultType]::ParameterName, 'filter-left')
            [CompletionResult]::new('--tolerance', '--tolerance', [CompletionResultType]::ParameterName, 'tolerance')
            [CompletionResult]::new('--date-format', '--date-format', [CompletionResultType]::ParameterName, 'date-format')
            [CompletionResult]::new('--strategy', '--strategy', [CompletionResultType]::ParameterName, 'strategy')
            [CompletionResult]::new('--cache-schema', '--cache-schema', [CompletionResultType]::ParameterName, 'cache-schema')
            [CompletionResult]::new('--infer-len', '--infer-len', [CompletionResultType]::ParameterName, 'infer-len')
            [CompletionResult]::new('--time-format', '--time-format', [CompletionResultType]::ParameterName, 'time-format')
            [CompletionResult]::new('--maintain-order', '--maintain-order', [CompletionResultType]::ParameterName, 'maintain-order')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--float-precision', '--float-precision', [CompletionResultType]::ParameterName, 'float-precision')
            [CompletionResult]::new('--validate', '--validate', [CompletionResultType]::ParameterName, 'validate')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--norm-unicode', '--norm-unicode', [CompletionResultType]::ParameterName, 'norm-unicode')
            [CompletionResult]::new('--sql-filter', '--sql-filter', [CompletionResultType]::ParameterName, 'sql-filter')
            [CompletionResult]::new('--ignore-errors', '--ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('--right', '--right', [CompletionResultType]::ParameterName, 'right')
            [CompletionResult]::new('--left-semi', '--left-semi', [CompletionResultType]::ParameterName, 'left-semi')
            [CompletionResult]::new('--left-anti', '--left-anti', [CompletionResultType]::ParameterName, 'left-anti')
            [CompletionResult]::new('--streaming', '--streaming', [CompletionResultType]::ParameterName, 'streaming')
            [CompletionResult]::new('--right-anti', '--right-anti', [CompletionResultType]::ParameterName, 'right-anti')
            [CompletionResult]::new('--coalesce', '--coalesce', [CompletionResultType]::ParameterName, 'coalesce')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--decimal-comma', '--decimal-comma', [CompletionResultType]::ParameterName, 'decimal-comma')
            [CompletionResult]::new('--full', '--full', [CompletionResultType]::ParameterName, 'full')
            [CompletionResult]::new('--no-optimizations', '--no-optimizations', [CompletionResultType]::ParameterName, 'no-optimizations')
            [CompletionResult]::new('--asof', '--asof', [CompletionResultType]::ParameterName, 'asof')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--cross', '--cross', [CompletionResultType]::ParameterName, 'cross')
            [CompletionResult]::new('--try-parsedates', '--try-parsedates', [CompletionResultType]::ParameterName, 'try-parsedates')
            [CompletionResult]::new('-X', '-X ', [CompletionResultType]::ParameterName, 'X')
            [CompletionResult]::new('--allow-exact-matches', '--allow-exact-matches', [CompletionResultType]::ParameterName, 'allow-exact-matches')
            [CompletionResult]::new('--low-memory', '--low-memory', [CompletionResultType]::ParameterName, 'low-memory')
            [CompletionResult]::new('--nulls', '--nulls', [CompletionResultType]::ParameterName, 'nulls')
            [CompletionResult]::new('--left', '--left', [CompletionResultType]::ParameterName, 'left')
            [CompletionResult]::new('--no-sort', '--no-sort', [CompletionResultType]::ParameterName, 'no-sort')
            [CompletionResult]::new('-z', '-z', [CompletionResultType]::ParameterName, 'z')
            [CompletionResult]::new('--ignore-leading-zeros', '--ignore-leading-zeros', [CompletionResultType]::ParameterName, 'ignore-leading-zeros')
            [CompletionResult]::new('--right-semi', '--right-semi', [CompletionResultType]::ParameterName, 'right-semi')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;json' {
            [CompletionResult]::new('--jaq', '--jaq', [CompletionResultType]::ParameterName, 'jaq')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;jsonl' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--ignore-errors', '--ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;lens' {
            [CompletionResult]::new('--find', '--find', [CompletionResultType]::ParameterName, 'find')
            [CompletionResult]::new('-W', '-W ', [CompletionResultType]::ParameterName, 'W')
            [CompletionResult]::new('--wrap-mode', '--wrap-mode', [CompletionResultType]::ParameterName, 'wrap-mode')
            [CompletionResult]::new('-P', '-P ', [CompletionResultType]::ParameterName, 'P')
            [CompletionResult]::new('--prompt', '--prompt', [CompletionResultType]::ParameterName, 'prompt')
            [CompletionResult]::new('--filter', '--filter', [CompletionResultType]::ParameterName, 'filter')
            [CompletionResult]::new('--echo-column', '--echo-column', [CompletionResultType]::ParameterName, 'echo-column')
            [CompletionResult]::new('--columns', '--columns', [CompletionResultType]::ParameterName, 'columns')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--freeze-columns', '--freeze-columns', [CompletionResultType]::ParameterName, 'freeze-columns')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--auto-reload', '--auto-reload', [CompletionResultType]::ParameterName, 'auto-reload')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--tab-separated', '--tab-separated', [CompletionResultType]::ParameterName, 'tab-separated')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'S')
            [CompletionResult]::new('--streaming-stdin', '--streaming-stdin', [CompletionResultType]::ParameterName, 'streaming-stdin')
            [CompletionResult]::new('--debug', '--debug', [CompletionResultType]::ParameterName, 'debug')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'm')
            [CompletionResult]::new('--monochrome', '--monochrome', [CompletionResultType]::ParameterName, 'monochrome')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;log' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;luau' {
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-B', '-B ', [CompletionResultType]::ParameterName, 'B')
            [CompletionResult]::new('--begin', '--begin', [CompletionResultType]::ParameterName, 'begin')
            [CompletionResult]::new('-E', '-E ', [CompletionResultType]::ParameterName, 'E')
            [CompletionResult]::new('--end', '--end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('--max-errors', '--max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--colindex', '--colindex', [CompletionResultType]::ParameterName, 'colindex')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--remap', '--remap', [CompletionResultType]::ParameterName, 'remap')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--no-globals', '--no-globals', [CompletionResultType]::ParameterName, 'no-globals')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('filter', 'filter', [CompletionResultType]::ParameterValue, 'filter')
            [CompletionResult]::new('map', 'map', [CompletionResultType]::ParameterValue, 'map')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;luau;filter' {
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-B', '-B ', [CompletionResultType]::ParameterName, 'B')
            [CompletionResult]::new('--begin', '--begin', [CompletionResultType]::ParameterName, 'begin')
            [CompletionResult]::new('-E', '-E ', [CompletionResultType]::ParameterName, 'E')
            [CompletionResult]::new('--end', '--end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('--max-errors', '--max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--colindex', '--colindex', [CompletionResultType]::ParameterName, 'colindex')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--remap', '--remap', [CompletionResultType]::ParameterName, 'remap')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--no-globals', '--no-globals', [CompletionResultType]::ParameterName, 'no-globals')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;luau;map' {
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-B', '-B ', [CompletionResultType]::ParameterName, 'B')
            [CompletionResult]::new('--begin', '--begin', [CompletionResultType]::ParameterName, 'begin')
            [CompletionResult]::new('-E', '-E ', [CompletionResultType]::ParameterName, 'E')
            [CompletionResult]::new('--end', '--end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('--max-errors', '--max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--colindex', '--colindex', [CompletionResultType]::ParameterName, 'colindex')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'r')
            [CompletionResult]::new('--remap', '--remap', [CompletionResultType]::ParameterName, 'remap')
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'g')
            [CompletionResult]::new('--no-globals', '--no-globals', [CompletionResultType]::ParameterName, 'no-globals')
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
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--epsilon', '--epsilon', [CompletionResultType]::ParameterName, 'epsilon')
            [CompletionResult]::new('--round', '--round', [CompletionResultType]::ParameterName, 'round')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--pct-thresholds', '--pct-thresholds', [CompletionResultType]::ParameterName, 'pct-thresholds')
            [CompletionResult]::new('--xsd-gdate-scan', '--xsd-gdate-scan', [CompletionResultType]::ParameterName, 'xsd-gdate-scan')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'C')
            [CompletionResult]::new('--cardinality-threshold', '--cardinality-threshold', [CompletionResultType]::ParameterName, 'cardinality-threshold')
            [CompletionResult]::new('--stats-options', '--stats-options', [CompletionResultType]::ParameterName, 'stats-options')
            [CompletionResult]::new('-J', '-J ', [CompletionResultType]::ParameterName, 'J')
            [CompletionResult]::new('--join-inputs', '--join-inputs', [CompletionResultType]::ParameterName, 'join-inputs')
            [CompletionResult]::new('-K', '-K ', [CompletionResultType]::ParameterName, 'K')
            [CompletionResult]::new('--join-keys', '--join-keys', [CompletionResultType]::ParameterName, 'join-keys')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'T')
            [CompletionResult]::new('--join-type', '--join-type', [CompletionResultType]::ParameterName, 'join-type')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'S')
            [CompletionResult]::new('--bivariate-stats', '--bivariate-stats', [CompletionResultType]::ParameterName, 'bivariate-stats')
            [CompletionResult]::new('-B', '-B ', [CompletionResultType]::ParameterName, 'B')
            [CompletionResult]::new('--bivariate', '--bivariate', [CompletionResultType]::ParameterName, 'bivariate')
            [CompletionResult]::new('--advanced', '--advanced', [CompletionResultType]::ParameterName, 'advanced')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--use-percentiles', '--use-percentiles', [CompletionResultType]::ParameterName, 'use-percentiles')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;partition' {
            [CompletionResult]::new('--filename', '--filename', [CompletionResultType]::ParameterName, 'filename')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--prefix-length', '--prefix-length', [CompletionResultType]::ParameterName, 'prefix-length')
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'limit')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;pivotp' {
            [CompletionResult]::new('--infer-len', '--infer-len', [CompletionResultType]::ParameterName, 'infer-len')
            [CompletionResult]::new('--col-separator', '--col-separator', [CompletionResultType]::ParameterName, 'col-separator')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--index', '--index', [CompletionResultType]::ParameterName, 'index')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--agg', '--agg', [CompletionResultType]::ParameterName, 'agg')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--values', '--values', [CompletionResultType]::ParameterName, 'values')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--validate', '--validate', [CompletionResultType]::ParameterName, 'validate')
            [CompletionResult]::new('--decimal-comma', '--decimal-comma', [CompletionResultType]::ParameterName, 'decimal-comma')
            [CompletionResult]::new('--maintain-order', '--maintain-order', [CompletionResultType]::ParameterName, 'maintain-order')
            [CompletionResult]::new('--sort-columns', '--sort-columns', [CompletionResultType]::ParameterName, 'sort-columns')
            [CompletionResult]::new('--try-parsedates', '--try-parsedates', [CompletionResultType]::ParameterName, 'try-parsedates')
            [CompletionResult]::new('--ignore-errors', '--ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;pragmastat' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'm')
            [CompletionResult]::new('--misrate', '--misrate', [CompletionResultType]::ParameterName, 'misrate')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
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
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--workdir', '--workdir', [CompletionResultType]::ParameterName, 'workdir')
            [CompletionResult]::new('--base-delay-ms', '--base-delay-ms', [CompletionResultType]::ParameterName, 'base-delay-ms')
            [CompletionResult]::new('-F', '-F ', [CompletionResultType]::ParameterName, 'F')
            [CompletionResult]::new('--filters', '--filters', [CompletionResultType]::ParameterName, 'filters')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'm')
            [CompletionResult]::new('--msg', '--msg', [CompletionResultType]::ParameterName, 'msg')
            [CompletionResult]::new('--save-fname', '--save-fname', [CompletionResultType]::ParameterName, 'save-fname')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--fd-output', '--fd-output', [CompletionResultType]::ParameterName, 'fd-output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;pseudo' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--start', '--start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('--increment', '--increment', [CompletionResultType]::ParameterName, 'increment')
            [CompletionResult]::new('--formatstr', '--formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;py' {
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--helper', '--helper', [CompletionResultType]::ParameterName, 'helper')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
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
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--helper', '--helper', [CompletionResultType]::ParameterName, 'helper')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;py;map' {
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--helper', '--helper', [CompletionResultType]::ParameterName, 'helper')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
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
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--dfa-size-limit', '--dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('--size-limit', '--size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--exact', '--exact', [CompletionResultType]::ParameterName, 'exact')
            [CompletionResult]::new('--literal', '--literal', [CompletionResultType]::ParameterName, 'literal')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--not-one', '--not-one', [CompletionResultType]::ParameterName, 'not-one')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unicode', '--unicode', [CompletionResultType]::ParameterName, 'unicode')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;reverse' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;safenames' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--reserved', '--reserved', [CompletionResultType]::ParameterName, 'reserved')
            [CompletionResult]::new('--prefix', '--prefix', [CompletionResultType]::ParameterName, 'prefix')
            [CompletionResult]::new('--mode', '--mode', [CompletionResultType]::ParameterName, 'mode')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sample' {
            [CompletionResult]::new('--weighted', '--weighted', [CompletionResultType]::ParameterName, 'weighted')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--max-size', '--max-size', [CompletionResultType]::ParameterName, 'max-size')
            [CompletionResult]::new('--ts-adaptive', '--ts-adaptive', [CompletionResultType]::ParameterName, 'ts-adaptive')
            [CompletionResult]::new('--ts-input-tz', '--ts-input-tz', [CompletionResultType]::ParameterName, 'ts-input-tz')
            [CompletionResult]::new('--timeseries', '--timeseries', [CompletionResultType]::ParameterName, 'timeseries')
            [CompletionResult]::new('--seed', '--seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('--systematic', '--systematic', [CompletionResultType]::ParameterName, 'systematic')
            [CompletionResult]::new('--ts-interval', '--ts-interval', [CompletionResultType]::ParameterName, 'ts-interval')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--ts-aggregate', '--ts-aggregate', [CompletionResultType]::ParameterName, 'ts-aggregate')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--ts-start', '--ts-start', [CompletionResultType]::ParameterName, 'ts-start')
            [CompletionResult]::new('--cluster', '--cluster', [CompletionResultType]::ParameterName, 'cluster')
            [CompletionResult]::new('--rng', '--rng', [CompletionResultType]::ParameterName, 'rng')
            [CompletionResult]::new('--stratified', '--stratified', [CompletionResultType]::ParameterName, 'stratified')
            [CompletionResult]::new('--ts-prefer-dmy', '--ts-prefer-dmy', [CompletionResultType]::ParameterName, 'ts-prefer-dmy')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('--bernoulli', '--bernoulli', [CompletionResultType]::ParameterName, 'bernoulli')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;schema' {
            [CompletionResult]::new('--enum-threshold', '--enum-threshold', [CompletionResultType]::ParameterName, 'enum-threshold')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--dates-whitelist', '--dates-whitelist', [CompletionResultType]::ParameterName, 'dates-whitelist')
            [CompletionResult]::new('--pattern-columns', '--pattern-columns', [CompletionResultType]::ParameterName, 'pattern-columns')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--prefer-dmy', '--prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--polars', '--polars', [CompletionResultType]::ParameterName, 'polars')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--stdout', '--stdout', [CompletionResultType]::ParameterName, 'stdout')
            [CompletionResult]::new('--strict-dates', '--strict-dates', [CompletionResultType]::ParameterName, 'strict-dates')
            [CompletionResult]::new('--strict-formats', '--strict-formats', [CompletionResultType]::ParameterName, 'strict-formats')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;search' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--dfa-size-limit', '--dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('--preview-match', '--preview-match', [CompletionResultType]::ParameterName, 'preview-match')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--flag', '--flag', [CompletionResultType]::ParameterName, 'flag')
            [CompletionResult]::new('--size-limit', '--size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unicode', '--unicode', [CompletionResultType]::ParameterName, 'unicode')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--count', '--count', [CompletionResultType]::ParameterName, 'count')
            [CompletionResult]::new('--literal', '--literal', [CompletionResultType]::ParameterName, 'literal')
            [CompletionResult]::new('-Q', '-Q ', [CompletionResultType]::ParameterName, 'Q')
            [CompletionResult]::new('--quick', '--quick', [CompletionResultType]::ParameterName, 'quick')
            [CompletionResult]::new('--exact', '--exact', [CompletionResultType]::ParameterName, 'exact')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--invert-match', '--invert-match', [CompletionResultType]::ParameterName, 'invert-match')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--not-one', '--not-one', [CompletionResultType]::ParameterName, 'not-one')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;searchset' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--flag', '--flag', [CompletionResultType]::ParameterName, 'flag')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--size-limit', '--size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('--dfa-size-limit', '--dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--unmatched-output', '--unmatched-output', [CompletionResultType]::ParameterName, 'unmatched-output')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unicode', '--unicode', [CompletionResultType]::ParameterName, 'unicode')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--invert-match', '--invert-match', [CompletionResultType]::ParameterName, 'invert-match')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--count', '--count', [CompletionResultType]::ParameterName, 'count')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--not-one', '--not-one', [CompletionResultType]::ParameterName, 'not-one')
            [CompletionResult]::new('--exact', '--exact', [CompletionResultType]::ParameterName, 'exact')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--literal', '--literal', [CompletionResultType]::ParameterName, 'literal')
            [CompletionResult]::new('-Q', '-Q ', [CompletionResultType]::ParameterName, 'Q')
            [CompletionResult]::new('--quick', '--quick', [CompletionResultType]::ParameterName, 'quick')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--flag-matches-only', '--flag-matches-only', [CompletionResultType]::ParameterName, 'flag-matches-only')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;select' {
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--seed', '--seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--random', '--random', [CompletionResultType]::ParameterName, 'random')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'S')
            [CompletionResult]::new('--sort', '--sort', [CompletionResultType]::ParameterName, 'sort')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;slice' {
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--end', '--end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--len', '--len', [CompletionResultType]::ParameterName, 'len')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--start', '--start', [CompletionResultType]::ParameterName, 'start')
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
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
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
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
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
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
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
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
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
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
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
            [CompletionResult]::new('--quote', '--quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('--user-agent', '--user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--sample', '--sample', [CompletionResultType]::ParameterName, 'sample')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--save-urlsample', '--save-urlsample', [CompletionResultType]::ParameterName, 'save-urlsample')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--no-infer', '--no-infer', [CompletionResultType]::ParameterName, 'no-infer')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--just-mime', '--just-mime', [CompletionResultType]::ParameterName, 'just-mime')
            [CompletionResult]::new('--prefer-dmy', '--prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--pretty-json', '--pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('--stats-types', '--stats-types', [CompletionResultType]::ParameterName, 'stats-types')
            [CompletionResult]::new('--harvest-mode', '--harvest-mode', [CompletionResultType]::ParameterName, 'harvest-mode')
            [CompletionResult]::new('-Q', '-Q ', [CompletionResultType]::ParameterName, 'Q')
            [CompletionResult]::new('--quick', '--quick', [CompletionResultType]::ParameterName, 'quick')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sort' {
            [CompletionResult]::new('--rng', '--rng', [CompletionResultType]::ParameterName, 'rng')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--seed', '--seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--unique', '--unique', [CompletionResultType]::ParameterName, 'unique')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'N')
            [CompletionResult]::new('--numeric', '--numeric', [CompletionResultType]::ParameterName, 'numeric')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'R')
            [CompletionResult]::new('--reverse', '--reverse', [CompletionResultType]::ParameterName, 'reverse')
            [CompletionResult]::new('--faster', '--faster', [CompletionResultType]::ParameterName, 'faster')
            [CompletionResult]::new('--random', '--random', [CompletionResultType]::ParameterName, 'random')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('--natural', '--natural', [CompletionResultType]::ParameterName, 'natural')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sortcheck' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--ignore-case', '--ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--pretty-json', '--pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('--all', '--all', [CompletionResultType]::ParameterName, 'all')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;split' {
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--kb-size', '--kb-size', [CompletionResultType]::ParameterName, 'kb-size')
            [CompletionResult]::new('--filter', '--filter', [CompletionResultType]::ParameterName, 'filter')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--size', '--size', [CompletionResultType]::ParameterName, 'size')
            [CompletionResult]::new('--pad', '--pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('--filename', '--filename', [CompletionResultType]::ParameterName, 'filename')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--chunks', '--chunks', [CompletionResultType]::ParameterName, 'chunks')
            [CompletionResult]::new('--filter-cleanup', '--filter-cleanup', [CompletionResultType]::ParameterName, 'filter-cleanup')
            [CompletionResult]::new('--filter-ignore-errors', '--filter-ignore-errors', [CompletionResultType]::ParameterName, 'filter-ignore-errors')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sqlp' {
            [CompletionResult]::new('--compression', '--compression', [CompletionResultType]::ParameterName, 'compression')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--time-format', '--time-format', [CompletionResultType]::ParameterName, 'time-format')
            [CompletionResult]::new('--wnull-value', '--wnull-value', [CompletionResultType]::ParameterName, 'wnull-value')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--compress-level', '--compress-level', [CompletionResultType]::ParameterName, 'compress-level')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'format')
            [CompletionResult]::new('--infer-len', '--infer-len', [CompletionResultType]::ParameterName, 'infer-len')
            [CompletionResult]::new('--rnull-values', '--rnull-values', [CompletionResultType]::ParameterName, 'rnull-values')
            [CompletionResult]::new('--datetime-format', '--datetime-format', [CompletionResultType]::ParameterName, 'datetime-format')
            [CompletionResult]::new('--date-format', '--date-format', [CompletionResultType]::ParameterName, 'date-format')
            [CompletionResult]::new('--float-precision', '--float-precision', [CompletionResultType]::ParameterName, 'float-precision')
            [CompletionResult]::new('--low-memory', '--low-memory', [CompletionResultType]::ParameterName, 'low-memory')
            [CompletionResult]::new('--ignore-errors', '--ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('--cache-schema', '--cache-schema', [CompletionResultType]::ParameterName, 'cache-schema')
            [CompletionResult]::new('--truncate-ragged-lines', '--truncate-ragged-lines', [CompletionResultType]::ParameterName, 'truncate-ragged-lines')
            [CompletionResult]::new('--no-optimizations', '--no-optimizations', [CompletionResultType]::ParameterName, 'no-optimizations')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--streaming', '--streaming', [CompletionResultType]::ParameterName, 'streaming')
            [CompletionResult]::new('--statistics', '--statistics', [CompletionResultType]::ParameterName, 'statistics')
            [CompletionResult]::new('--try-parsedates', '--try-parsedates', [CompletionResultType]::ParameterName, 'try-parsedates')
            [CompletionResult]::new('--decimal-comma', '--decimal-comma', [CompletionResultType]::ParameterName, 'decimal-comma')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;stats' {
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--round', '--round', [CompletionResultType]::ParameterName, 'round')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--weight', '--weight', [CompletionResultType]::ParameterName, 'weight')
            [CompletionResult]::new('--percentile-list', '--percentile-list', [CompletionResultType]::ParameterName, 'percentile-list')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--cache-threshold', '--cache-threshold', [CompletionResultType]::ParameterName, 'cache-threshold')
            [CompletionResult]::new('--boolean-patterns', '--boolean-patterns', [CompletionResultType]::ParameterName, 'boolean-patterns')
            [CompletionResult]::new('--dates-whitelist', '--dates-whitelist', [CompletionResultType]::ParameterName, 'dates-whitelist')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--infer-boolean', '--infer-boolean', [CompletionResultType]::ParameterName, 'infer-boolean')
            [CompletionResult]::new('--mode', '--mode', [CompletionResultType]::ParameterName, 'mode')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('--infer-dates', '--infer-dates', [CompletionResultType]::ParameterName, 'infer-dates')
            [CompletionResult]::new('--prefer-dmy', '--prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('--stats-jsonl', '--stats-jsonl', [CompletionResultType]::ParameterName, 'stats-jsonl')
            [CompletionResult]::new('--typesonly', '--typesonly', [CompletionResultType]::ParameterName, 'typesonly')
            [CompletionResult]::new('--percentiles', '--percentiles', [CompletionResultType]::ParameterName, 'percentiles')
            [CompletionResult]::new('--mad', '--mad', [CompletionResultType]::ParameterName, 'mad')
            [CompletionResult]::new('--vis-whitespace', '--vis-whitespace', [CompletionResultType]::ParameterName, 'vis-whitespace')
            [CompletionResult]::new('--cardinality', '--cardinality', [CompletionResultType]::ParameterName, 'cardinality')
            [CompletionResult]::new('-E', '-E ', [CompletionResultType]::ParameterName, 'E')
            [CompletionResult]::new('--everything', '--everything', [CompletionResultType]::ParameterName, 'everything')
            [CompletionResult]::new('--median', '--median', [CompletionResultType]::ParameterName, 'median')
            [CompletionResult]::new('--quartiles', '--quartiles', [CompletionResultType]::ParameterName, 'quartiles')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--nulls', '--nulls', [CompletionResultType]::ParameterName, 'nulls')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;table' {
            [CompletionResult]::new('-w', '-w', [CompletionResultType]::ParameterName, 'w')
            [CompletionResult]::new('--width', '--width', [CompletionResultType]::ParameterName, 'width')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--align', '--align', [CompletionResultType]::ParameterName, 'align')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--condense', '--condense', [CompletionResultType]::ParameterName, 'condense')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--pad', '--pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;template' {
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--template-file', '--template-file', [CompletionResultType]::ParameterName, 'template-file')
            [CompletionResult]::new('--outfilename', '--outfilename', [CompletionResultType]::ParameterName, 'outfilename')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--outsubdir-size', '--outsubdir-size', [CompletionResultType]::ParameterName, 'outsubdir-size')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--customfilter-error', '--customfilter-error', [CompletionResultType]::ParameterName, 'customfilter-error')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--globals-json', '--globals-json', [CompletionResultType]::ParameterName, 'globals-json')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--template', '--template', [CompletionResultType]::ParameterName, 'template')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
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
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to;ods' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to;postgres' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to;sqlite' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to;xlsx' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--schema', '--schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--stats-csv', '--stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--pipe', '--pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('-A', '-A ', [CompletionResultType]::ParameterName, 'A')
            [CompletionResult]::new('--all-strings', '--all-strings', [CompletionResultType]::ParameterName, 'all-strings')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--dump', '--dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--drop', '--drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--evolve', '--evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'k')
            [CompletionResult]::new('--print-package', '--print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--stats', '--stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
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
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--trim', '--trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('--no-boolean', '--no-boolean', [CompletionResultType]::ParameterName, 'no-boolean')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;transpose' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--select', '--select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--long', '--long', [CompletionResultType]::ParameterName, 'long')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'm')
            [CompletionResult]::new('--multipass', '--multipass', [CompletionResultType]::ParameterName, 'multipass')
            [CompletionResult]::new('--memcheck', '--memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;validate' {
            [CompletionResult]::new('--dfa-size-limit', '--dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--size-limit', '--size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('--backtrack-limit', '--backtrack-limit', [CompletionResultType]::ParameterName, 'backtrack-limit')
            [CompletionResult]::new('--valid-output', '--valid-output', [CompletionResultType]::ParameterName, 'valid-output')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--valid', '--valid', [CompletionResultType]::ParameterName, 'valid')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--email-min-subdomains', '--email-min-subdomains', [CompletionResultType]::ParameterName, 'email-min-subdomains')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('--invalid', '--invalid', [CompletionResultType]::ParameterName, 'invalid')
            [CompletionResult]::new('--email-domain-literal', '--email-domain-literal', [CompletionResultType]::ParameterName, 'email-domain-literal')
            [CompletionResult]::new('--trim', '--trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('--email-display-text', '--email-display-text', [CompletionResultType]::ParameterName, 'email-display-text')
            [CompletionResult]::new('--email-required-tld', '--email-required-tld', [CompletionResultType]::ParameterName, 'email-required-tld')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--fancy-regex', '--fancy-regex', [CompletionResultType]::ParameterName, 'fancy-regex')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--pretty-json', '--pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--fail-fast', '--fail-fast', [CompletionResultType]::ParameterName, 'fail-fast')
            [CompletionResult]::new('--no-format-validation', '--no-format-validation', [CompletionResultType]::ParameterName, 'no-format-validation')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('schema', 'schema', [CompletionResultType]::ParameterValue, 'schema')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;validate;schema' {
            [CompletionResult]::new('--dfa-size-limit', '--dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('-j', '-j', [CompletionResultType]::ParameterName, 'j')
            [CompletionResult]::new('--jobs', '--jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--delimiter', '--delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--batch', '--batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--size-limit', '--size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('--backtrack-limit', '--backtrack-limit', [CompletionResultType]::ParameterName, 'backtrack-limit')
            [CompletionResult]::new('--valid-output', '--valid-output', [CompletionResultType]::ParameterName, 'valid-output')
            [CompletionResult]::new('--ckan-api', '--ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--valid', '--valid', [CompletionResultType]::ParameterName, 'valid')
            [CompletionResult]::new('--cache-dir', '--cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--email-min-subdomains', '--email-min-subdomains', [CompletionResultType]::ParameterName, 'email-min-subdomains')
            [CompletionResult]::new('--ckan-token', '--ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('--invalid', '--invalid', [CompletionResultType]::ParameterName, 'invalid')
            [CompletionResult]::new('--email-domain-literal', '--email-domain-literal', [CompletionResultType]::ParameterName, 'email-domain-literal')
            [CompletionResult]::new('--trim', '--trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('--email-display-text', '--email-display-text', [CompletionResultType]::ParameterName, 'email-display-text')
            [CompletionResult]::new('--email-required-tld', '--email-required-tld', [CompletionResultType]::ParameterName, 'email-required-tld')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--progressbar', '--progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--fancy-regex', '--fancy-regex', [CompletionResultType]::ParameterName, 'fancy-regex')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'q')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--json', '--json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--pretty-json', '--pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--no-headers', '--no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--fail-fast', '--fail-fast', [CompletionResultType]::ParameterName, 'fail-fast')
            [CompletionResult]::new('--no-format-validation', '--no-format-validation', [CompletionResultType]::ParameterName, 'no-format-validation')
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
            [CompletionResult]::new('log', 'log', [CompletionResultType]::ParameterValue, 'log')
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
        'qsv;help;log' {
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
