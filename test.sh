#!/bin/bash -e

base_dir="$(realpath "$(dirname "$0")")"
is_proc_macro=1
disable_coverage=1 # Coverage is currently broken due to build directory changes
msrv_overrides="cargo-platform@0.3.2" # 0.3.3 requires Rust 1.91

export base_dir is_proc_macro disable_coverage msrv_overrides

"${base_dir}/submodules/test_script/test.sh" "$@"
