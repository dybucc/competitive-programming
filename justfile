set quiet := true
set shell := ["fish", "-c"]

alias r := run
alias t := test
alias s := show

cargo := require("cargo")
delta := require("delta")
moor := require("moor")
current-problem := "battleship"
current-case := "sample-case.in"
current-sol := "sample-case.ans"
problem-dir := justfile_directory()
rust-version := "1.91.0"
opts := append(prepend(replace_regex('''
    -C target-cpu=native -C opt-level=3''', '\s', '", "'), '"'), '"')
cargo-opts := trim(f'''
  +{{rust-version}} --config {{"'build.rustflags=[" + opts + "]'"}}
''')

[default]
[private]
default:
    {{ just_executable() }} --list --unsorted --justfile {{ justfile() }}

# runs the program without testing it against sample cases
[arg("case", pattern='[[:alnum:][:punct:]]+')]
[arg("host_dir", pattern='(/[[:alnum:][:punct:][:blank:]]+/?)+')]
[arg("problem", pattern='[[:alpha:]]+')]
run host_dir=problem-dir problem=current-problem case=current-case:
    {{ cargo }} {{ cargo-opts }} r --release 2> /dev/null -- <{{ host_dir / problem / case }}

# runs the selected test case for the selected problem in the selected directory
[arg("case", pattern='[[:alnum:][:punct:]]+')]
[arg("case_sol", pattern='[[:alnum:][:punct:]]+')]
[arg("host_dir", pattern='(/[[:alnum:][:punct:][:blank:]]*)+')]
[arg("problem", pattern='[[:alpha:]]+')]
test host_dir=problem-dir problem=current-problem case=current-case case_sol=current-sol:
    {{ delta }} ({{ cargo }} {{ cargo-opts }} r --release 2> /dev/null -- <{{ host_dir / problem / case }} | psub) ({{ moor }} {{ host_dir / problem / case_sol }} | psub)

# outputs to stdout a slightly formatted sample test case and its solution
[arg("case", pattern='[[:alnum:][:punct:]]+')]
[arg("case_sol", pattern='[[:alnum:][:punct:]]+')]
[arg("host_dir", pattern='(/[[:alnum:][:punct:][:blank:]]*)+')]
[arg("problem", pattern='[[:alpha:]]+')]
show host_dir=problem-dir problem=current-problem case=current-case case_sol=current-sol:
    echo -e '--- end test sample ---' > ./.newline
    -{{ moor }} (cat {{ host_dir / problem / case }} .newline {{ host_dir / problem / case_sol }} | psub)
    rm ./.newline
