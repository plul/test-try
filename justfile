_default:
    @just --unsorted --list

# Check project
check:
    bc-check

# Format project
fmt:
    bc-fmt

watch-test:
    watchexec --clear --restart -- cargo test
