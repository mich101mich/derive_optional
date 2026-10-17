# Agent Instructions

## Project Structure

- This is a Rust procedural-macro crate. Derive parsing and expansion start in `src/lib.rs`; generated method groups live under `src/sections/`, with trait and conversion impls under `src/external/`.
- The integration tests under `tests/generic/` and `tests/specific/` mirror those source groups. Keep new behavior and tests close to the matching group.
- `src/utils` and `submodules/test_script` are Git submodules. Adapt project code to utility API changes instead of editing submodule files for project-specific fixes.

## Testing

- `try_op` is nightly-only; run all-features tests on nightly, not stable.
- Compile-fail tests use `err_span_check`. Keep their Rust source cases under `tests/fail/`; expected diagnostics are inline annotations, not `.stderr` files. These fixtures must be tracked by Git before the checker can run.
- `test.sh` is a wrapper for the shared `submodules/test_script` harness. It enables procedural-macro error checks and disables coverage for this crate. Use `bash ./test.sh frozen` for the full non-interactive verification workflow.
- On Windows, run the shared harness from WSL; its isolated test directories require Unix symlink behavior.
- The shared harness runs the MSRV and minimal-version tests in isolated target directories. Do not replace the checked-in `Cargo.lock` with a minimal-version lockfile.
- Keep dependency lower bounds as small as the code and minimal-version tests allow.

## Change Scope

- Preserve the existing module boundaries and public API unless the requested behavior requires a change.
- When changing an error diagnostic or its span, update the matching `tests/fail` case and verify it with `err_span_check`.
