set quiet
set shell := ["fish", "-c"]

alias r := run
alias t := test
alias s := show
alias d := debug

cargo := require("cargo")
delta := require("delta")
moor := require("moor")
current-problem := "bread"
current-case := "bread.1.in"
current-sol := "bread.1.ans"
problem-dir := home_directory() / "Downloads"

# FIXME(msrv): update whenever kattis updates

rust-version := "1.94.0"
opts := append(prepend(replace_regex('''
    -C target-cpu=native -C opt-level=3''', '\s', '", "'), '"'), '"')
cargo-opts := trim(f'''
  +$RV --config {{"'build.rustflags=[" + opts + "]'"}}
''')

[default]
[private]
default:
    {{ just_executable() }} --list --unsorted --justfile {{ justfile() }}

# runs the program without testing it against sample cases with `debug_assertions` on
[arg("err", pattern='(-|\+)e')]
[arg("nightly", pattern='(-|\+)n')]
[no-cd]
debug nightly='-n' err='-e':
    {{ if nightly == "+n" { "RV=nightly" } else { "RV=" + rust-version } }} {{ cargo }} +$RV r -- {{ if err == "+e" { "" } else { "2> /dev/null" } }} <{{ problem-dir / current-problem / current-case }}

# runs the program without testing it against sample cases
[arg("err", pattern='(-|\+)e')]
[arg("nightly", pattern='(-|\+)n')]
[no-cd]
run nightly='-n' err='-e':
    {{ if nightly == "+n" { "RV=nightly" } else { "RV=" + rust-version } }} {{ cargo }} {{ cargo-opts }} r "--release" {{ if err == "+e" { "" } else { "2> /dev/null" } }} -- <{{ problem-dir / current-problem / current-case }}

# runs the current test case for the current problem
[no-cd]
test:
    {{ delta }} ({{ cargo }} {{ cargo-opts }} r --release 2> /dev/null -- <{{ problem-dir / current-problem / current-case }} | psub) ({{ moor }} {{ problem-dir / current-problem / current-sol }} | psub)

# outputs to stdout a slightly formatted sample test case and its solution
[no-cd]
show:
    echo -e '--- end test sample ---' > ./.newline
    -{{ moor }} (cat {{ problem-dir / current-problem / current-case }} .newline {{ problem-dir / current-problem / current-sol }} | psub)
    rm ./.newline
