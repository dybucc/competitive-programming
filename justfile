set quiet := true
set shell := ["fish", "-c"]

alias r := run
alias t := test
alias s := show

cargo := require("cargo")
delta := require("delta")
moor := require("moor")
current-problem := "tictactoe2"
current-case := "2.in"
current-sol := "2.ans"
problem-dir := justfile_directory()
rust-version := "1.91.0"
opts := append(prepend(replace_regex('''
    -C target-cpu=native -C opt-level=3''', '\s', '", "'), '"'), '"')
cargo-opts := trim(f'''
  +$RV --config {{"'build.rustflags=[" + opts + "]'"}}
''')

[default]
[private]
default:
    {{ just_executable() }} --list --unsorted --justfile {{ justfile() }}

# runs the program without testing it against sample cases
[arg("case", pattern='[[:ascii:]]+')]
[arg("host_dir", pattern='(/[[:ascii:]]+/?)+')]
[arg("nightly", pattern='0|1')]
[arg("problem", pattern='[[:ascii:]]+')]
[no-cd]
run host_dir=problem-dir problem=current-problem case=current-case nightly='0':
    {{ if nightly == "1" { "RV=nightly" } else { "RV=" + rust-version } }} {{ cargo }} {{ cargo-opts }} r --release 2> /dev/null -- <{{ host_dir / problem / case }}

# runs the selected test case for the selected problem in the selected directory
[arg("case", pattern='[[:ascii:]]+')]
[arg("case_sol", pattern='[[:ascii:]]+')]
[arg("host_dir", pattern='(/[[:ascii:]]+/?)+')]
[arg("problem", pattern='[[:ascii:]]+')]
[no-cd]
test host_dir=problem-dir problem=current-problem case=current-case case_sol=current-sol:
    {{ delta }} ({{ cargo }} {{ cargo-opts }} r --release 2> /dev/null -- <{{ host_dir / problem / case }} | psub) ({{ moor }} {{ host_dir / problem / case_sol }} | psub)

# outputs to stdout a slightly formatted sample test case and its solution
[arg("case", pattern='[[:ascii:]]+')]
[arg("case_sol", pattern='[[:ascii:]]+')]
[arg("host_dir", pattern='(/[[:ascii:]]+/?)+')]
[arg("problem", pattern='[[:ascii:]]+')]
[no-cd]
show host_dir=problem-dir problem=current-problem case=current-case case_sol=current-sol:
    echo -e '--- end test sample ---' > ./.newline
    -{{ moor }} (cat {{ host_dir / problem / case }} .newline {{ host_dir / problem / case_sol }} | psub)
    rm ./.newline
