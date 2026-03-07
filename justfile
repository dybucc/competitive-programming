set shell := ["fish", "-c"]
set quiet := true

alias r := run
alias t := test

cargo := require("cargo")
delta := require("delta")
moor := require("moor")
current_problem := "bitsequalizer"
current_case := "test.1.in"
current_sol := "test.1.ans"
current_dir := home_directory() / "Downloads"

[default]
[private]
default:
    {{ just_executable() }} --list --unsorted --justfile {{ justfile() }}

# runs the program without testing it against any sample solution
run host_dir=current_dir problem=current_problem case=current_case:
    {{ cargo }} r --release <{{ host_dir / problem / case }}

# runs the selected test case for the selected problem in the selected directory
test host_dir=current_dir problem=current_problem case=current_case case_sol=current_sol:
    {{ delta }} ({{ cargo }} r --release 2> /dev/null -- <{{ host_dir / problem / case }} | psub) ({{ moor }} {{ host_dir / problem / case_sol }} | psub)
