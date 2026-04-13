set quiet := true
set shell := ["fish", "-c"]

alias r := run
alias t := test
alias s := show

cargo := require("cargo")
delta := require("delta")
moor := require("moor")
current-problem := "classy"
current-case := "classy-01.in"
current-sol := "classy-01.ans"
problem-dir := home_directory() / "Downloads"
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

# runs the program without testing it against sample cases
[arg("err", pattern='0|1')]
[arg("nightly", pattern='0|1')]
[no-cd]
run nightly='0' err='0':
    {{ if nightly == "1" { "RV=nightly" } else { "RV=" + rust-version } }} {{ cargo }} {{ cargo-opts }} r --release {{ if err == "1" { "" } else { "2> /dev/null" } }} -- <{{ problem-dir / current-problem / current-case }}

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
