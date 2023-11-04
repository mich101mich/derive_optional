#!/bin/bash

function try_silent {
    echo "Running $*"
    unbuffer "$@" > /tmp/derive_optional_test_out.txt && return 0
    # if we get here, the command failed
    cat /tmp/derive_optional_test_out.txt
    echo "Failed to run $*"
    exit 1
}

# main tests
pushd ~/projects/derive_optional || (echo "Failed to cd to ~/projects/derive_optional"; exit 1)
try_silent rustup update
try_silent cargo update
try_silent cargo +stable test
try_silent cargo +nightly test
try_silent cargo +nightly doc --no-deps
try_silent cargo +nightly clippy -- -D warnings
try_silent cargo +stable fmt --check
# shellcheck disable=SC2164
popd

# old rustc version
pushd ~/projects/derive_optional_old_rustc || (echo "Failed to cd to ~/projects/derive_optional_old_rustc"; exit 1)
try_silent cargo +1.56.0 test -- --skip failing_tests
# shellcheck disable=SC2164
popd

# minimum version
pushd ~/projects/derive_optional_min_version || (echo "Failed to cd to ~/projects/derive_optional_min_version"; exit 1)
try_silent cargo +nightly -Z minimal-versions update

try_silent cargo +stable test -- --skip failing_tests
try_silent cargo +nightly test -- --skip failing_tests
# shellcheck disable=SC2164
popd

echo "All tests passed!"
