# EOIE source workspace

This repository is the source workspace extracted from the EOIE bundle. Generated
Rust is checked in so a normal build does not require a Spiral compiler. New
application policy, migration tools and test scenarios belong in native Spiral.
Existing handwritten Rust adapters and tests remain migration debt; embedding
their algorithms in `rust_global` does not complete that migration.

## Build

Install Rust 1.88 or newer. Windows also needs the MSVC C++ build tools; Linux
needs a native C linker. From `src`, bootstrap the Spiral-authored driver
into a separate target directory so Windows can rebuild the workspace safely:

```text
cargo build --locked --package eoie-dev --target-dir target/workflow-driver
./target/workflow-driver/debug/eoie-dev --root .. --test --skip-release
```

On Windows, the executable is `eoie-dev.exe`. Add `--offline` to both commands
after caching dependencies. Omit `--skip-release` to also build a release
candidate. The native driver does not install it. `--package NAME` selects tests,
`--target-dir PATH` selects artifacts, and `--timeout-ms N` bounds each Cargo
process. `--dry-run` prints commands; selected runs still read Cargo metadata.

PowerShell 7 remains required for regeneration and the transitional publication
adapter. From the repository root:

```powershell
pwsh -NoProfile -File build.ps1 -Test
pwsh -NoProfile -File eoie.ps1 help
```

Cargo downloads registry dependencies using the committed `src/Cargo.lock`.
`src/vendor/` stays ignored and is not part of the build. After one successful
fetch/build, `build.ps1 -Test -Offline` uses the Cargo cache. When invoking Cargo
directly, change to `src` first so it discovers `.cargo/config.toml`.

`-Test` runs every workspace suite, including native regression tests. Tests that
require the optional `--check`/`--plan-ir` compiler are explicitly marked ignored.
Run them with `-Test -CompilerContracts -SpiralCompiler <compiler-path>` using
the patched single-flight host described below. Ordinary runtime builds remain
compiler-independent.

For a Spiral edit, use the targeted development loop:

```powershell
pwsh dev.ps1 -List
pwsh dev.ps1 -Package eoie-fs-actions -Offline
pwsh dev.ps1 -Package eoie-fs-actions -Offline -Publish
```

The command regenerates the selected Cargo packages and their declared probes in an isolated copy, checks
all their targets and runs their tests without building a release executable.
Cargo artifacts are reused under the ignored `src/target/dev-validation` directory;
the full regeneration gate uses a fresh target directory unless explicitly configured.
`-CheckOnly` skips tests. `-Publish` copies the generated Rust back only after
validation succeeds and the source file inventory and original contents still
match the staged snapshot. Added source files block publication; ignored cache
activity does not.
Generated outputs use LF line endings; unchanged outputs retain their timestamps.
Managed compiler assemblies are copied and hashed once per run so concurrent
compiler rebuilds cannot replace them between owners. `-CompilerBundle` and `-TimeoutSec`
select the compiler and bound each owner compilation; no compiler is needed for
`-List`. An existing Cargo build can be tested without regeneration using
`build.ps1 -Test -Package eoie-fs-actions -SkipRelease`.
Packages whose tests launch the CLI declare `test-requires-cli = true` under
`[package.metadata.eoie]`. Selected runs build that executable first, including
when the target directory is empty; pure library tests remain isolated. Run
`cargo test --locked -p eoie-dev --test selected_workflow` from `src` to check
this behavior independently of the Spiral compiler. Scenarios are authored in
`src/eoie_dev/selected_tests.spi`.

Each Cargo owner declares its Spiral `entry` and Rust `output` under
`[package.metadata.spiral]` in its existing `Cargo.toml`. Add this metadata when
adding an owner. The development loop and full regeneration gate use the same
mapping. Workflow failure/timeout/publication contracts run independently of a
Spiral installation with `compiler-contracts/test-dev-workflow.ps1`.
Generated examples also declare `entry` and `output` in
`[[package.metadata.spiral.auxiliary]]` tables. Handwritten adapters and tests
still lack generation declarations and must be migrated. EOIE's source statistics and cold rebuild
planner use these mappings too.

## Mechanical native Spiral migration

`typed_predicate` implements byte/text predicates as a GADT, existential
projections and an HKT registry. The first consumers replace three cold-proof
and handoff helpers, removing 335 bytes of embedded production Rust. This is
semantic migration credit, not a claim that three entire RustGlobal sites vanished.

The `eoie-predicate-lift` tool lexes, matches and rewrites source in native Spiral.
It accepts only the supported SHA-256-text and receipt-text helper forms and
rejects ambiguity, attributes, public functions and near matches. From `src`:

```text
cargo build --locked -p eoie-predicate-lift
./target/debug/eoie-lift-predicate sha256 helper input.spi candidate.spi
cargo test --locked -p eoie-predicate-lift --test native_codemod
```

Use `.exe` on Windows. The output must be a new file; the input is preserved.
Add `typed_predicate` to the owning Spiral package, review the candidate, then
regenerate and test the affected owner before adopting it. The codemod uses an
exact token grammar, not a general Rust AST or semantic equivalence proof.
Its lexer and rewrite models live in `predicate_lex_core` and
`predicate_rewrite_model`; their generated libraries are separate Cargo owners.
The native developer workflow uses `eoie_dev_model` and
`developer_metadata_domain` for argument, ordering and package-selection policy.

Process matrix `args` cells can use `json:["argument with spaces", "", "next"]`
to preserve exact arguments, including Windows paths and empty strings. Ordinary
whitespace-separated cells retain their existing behavior. These arguments go
directly to the child process; they are not shell commands.

`proxy cold-rebuild-matrix <root> <matrix-relative> <eoie> <compiler> <dotnet>
<rustfmt> [compiler-timeout-ms]` plans one bounded compiler process per declared
output. The optional budget defaults to 180,000 ms. Use absolute tool paths or
names available on PATH, and set `EOIE_SPIRAL_BUNDLE` when the compiler needs its
bundle for package resolution. Planning records a source snapshot under
`.cache/cold-rebuild/`; owner commands compile there so compiler sidecars never
touch the replay root. Each owner checks source/output drift, optionally formats
its candidate and atomically publishes nonempty Rust after successful compilation.
Identical outputs retain their timestamps. Run cold replay in an isolated source
copy: its commands replace generated outputs there, and Cargo/runtime validation
remains a separate gate. Generating the matrix does not execute it. Keep source
inputs unchanged between planning and replay. Pass a row timeout larger than
the compiler budget plus formatting time to `proxy batch-plan` and
choose its parallel-chain count explicitly when sharing the machine.

Build products (`eoie`, `eoie.exe`, `src/target`, compiler C residuals and sidecars)
are ignored. Keep `.spi`, `.spiproj`, generated `.rs`, adapters, tests, Cargo
manifests and the lockfile. Files under `state/` include historical Linux release
evidence; they do not certify the current Windows build.

## Native Spiral compiler

The current supported lane is **single-flight**. Hopac remains experimental.
The compiler lives in the [spiral](https://github.com/i574n/spiral) repository (`apps/compiler/tmp`);
clone it beside this repository. Build alpha418 once using its own portable scripts:

```powershell
$compiler = '../spiral/apps/compiler/tmp'
pwsh "$compiler/scripts/install-dotnet.ps1" # only if .NET 11 is absent
pwsh "$compiler/scripts/build.ps1" -Mode single-flight
pwsh spiral.ps1 -CompilerBundle $compiler `
  -InputPath src/eoie_fs_actions/main.spi `
  -OutputPath src/eoie_fs_actions/fs_actions.rs
```

`spiral.ps1` supports `-Backend Rust|C|Fsharp|Delphi`, `-TimeoutSec`, and
`-SourceRoot`. It copies Spiral sources into a unique compiler cache directory
before compilation, preventing the core from leaving C residuals in the checkout.
It publishes the requested output only after a successful bounded compile and
preserves the output timestamp when its content is unchanged.

Set `EOIE_SPIRAL_BUNDLE` or pass `-CompilerBundle` for a compiler elsewhere.
Without either, the scripts use `../spiral/apps/compiler/tmp` (a spiral checkout beside this repository).
The compiler's `SPIRAL_BIN_CACHE_DIR` and `SPIRAL_DOTNET` select its cache and SDK.
The same scripts run on Windows and Linux; CI is configured to test runtime builds
on both. This review was executed on Windows.

EOIE's process library also accepts a native `SpiralCompiler.dll` through
`EOIE_SPIRAL_COMPILE`. Set `EOIE_DOTNET` (or `SPIRAL_DOTNET`) to the matching .NET
host, and `EOIE_SPIRAL_BUNDLE` for core-package resolution. Native code generation
omits the legacy timeout flag and is supervised by EOIE's process timeout.
The patched native host implements package-wide `--check` without requiring
`main`, and bounded `--plan-ir` for unconditional compiled `RustPlanOp` markers.
Runtime-dependent plans are rejected. All seventeen optional EOIE integration
contracts can run against this host; Hopac package attestation remains unsupported.
Run `compiler-contracts/test-attestation.ps1` for package rejection,
plan payload, timeout, and native GADT regression coverage.

The compiler host validates six Rust export ABIs: `RustExportI32`,
`RustExportI32Binary`, `RustExportStringUnary`, `RustExportU64Unary`,
`RustExportStringTuple5Unary`, and `RustExportU64String5`. Unary string and u64
exports accept a borrowed string; tuple exports return five owned `Rc<str>` values.
Exports require exactly one `RustLibrary` marker and a retained join point with
the exact argument and result types. Unknown markers, duplicate names, missing
targets and signatures changed by optimization are rejected.

Run the compiler's `scripts/test-rust-exports.ps1` (in the spiral repository's `apps/compiler/tmp`) for an external Rust
consumer and negative ABI contracts. `compiler-contracts/test-regeneration.ps1`
regenerates every declared primary Cargo target in an isolated
cache copy. Use `-CargoCheck -Test -CompilerContracts` to check all targets, run
the workspace contracts, check source topology with the rebuilt EOIE, and build a release executable from those regenerated
owners. Use `-EoieRoot` after relocating EOIE, `-Filter` for selected members,
and `-Offline` with a populated registry cache. Auxiliary probes and operating
system adapters remain consumers by default; `-IncludeAuxiliary` also regenerates
the declared probes. Checkout Rust changes only with explicit `-Publish`, after
successful Cargo validation and a checkout snapshot check.

## Generic versus strict bundles

| Profile | What passing means |
| --- | --- |
| `generic` | A nonempty tree can be represented by the archive policy: normalized unique paths, no symlinks/special entries, canonical manifest and bounded extraction. It does not attest EOIE source, executable identity or release evidence. |
| `eoie` (strict) | The EOIE release layout, source/package census, growth limits, current evidence, ratings, receipts and executable/archive contracts also pass. `verify` extracts a bounded temporary copy and applies the release checks before reporting success. |
| `auto` | Selects `eoie` when `eoie` or `eoie.exe` accompanies `state/package.spiproj`, `state/core.spi` and `src/Cargo.toml`; otherwise selects `generic`. A strict distribution must contain exactly one root binary. Auto-selection is not certification. |

Use explicit `eoie` for release gates. A source checkout has docs, scripts and
tests and lacks a certified release payload, so it is not expected to pass that
packaging gate. Generic success must never be substituted for strict release
approval. Do not run `bundle create` over a live checkout: it prunes release
transients and includes files that Git ignores. Stage a distribution separately.

Archive creation validates a sibling staging file before replacing an existing
output. Rejected size/parity checks preserve the previous archive. Output paths
inside the source tree are rejected, including Windows extended-path aliases.
ZIP verification bounds total expanded payload to 256 MiB; strict verification
removes its temporary extraction after either acceptance or rejection.

The first source commit is checked through locked builds, tests and Git hygiene.
`pwsh compiler-contracts/test-source-package.ps1 -Offline` stages the working-tree
contents of Git-eligible files, archives and rehydrates them through EOIE, checks
every SHA-256 and runs the native workspace tests from the restored tree. It
leaves the source archive and manifest under ignored `.cache/source-package/`.
Use `-SkipBuild` for archive/hash/topology checks only, or `-EoieBinary` to test a
specific executable. Omit `-Offline` before dependencies have been cached.
Producing a newly certified EOIE release still requires compatible compiler
attestation, fresh release evidence and cold rebuilds. These remain tracked in
the `PORT-*` agile tasks, alongside the fixes completed in this review.

## Native coverage

Release builds smoke-test a temporary executable before atomically replacing the
installed binary. Failed candidates and locked Windows destinations preserve the
previous executable; identical binaries preserve its timestamp. Run
`pwsh compiler-contracts/test-binary-publication.ps1` to verify these behaviors.

Run `pwsh compiler-contracts/test-native-probes.ps1` from this directory to build
and execute every Cargo example in a verified source snapshot. The runner checks
Cargo's declared inventory, uses two build jobs, runs probes sequentially and
keeps process receipts and source/binary hashes under ignored `.cache/native-probes/`.
`-EoieBinary`, `-TargetDirectory`, `-Jobs` and `-TimeoutMs` configure the run.

`compiler-contracts/test-native-coverage.ps1` archives and rehydrates Git-eligible
sources, freezes the EOIE executable and managed compiler, and runs instrumented
workspace tests through EOIE. It uses two Cargo jobs and one test thread by
default. LLVM tools must match the installed Rust compiler; use its
`llvm-tools-preview` component or provide a matching tools directory.

```powershell
pwsh compiler-contracts/test-native-coverage.ps1 -SpiralCompiler $env:EOIE_SPIRAL_COMPILE -LlvmDirectory '<LLVM bin directory>' -CompilerContracts
```

The optional compiler contracts also use the configured `EOIE_DOTNET` and
`EOIE_SPIRAL_BUNDLE`. Omit `-CompilerContracts` to collect native tests only.
All dependencies must already be cached. Results, source/tool hashes and process
receipts stay under ignored `.cache/native-coverage/`; the reusable instrumented
Cargo target stays under `src/target/native-coverage`. Evidence from this workflow
does not certify a strict release. `-EoieBinary` selects the supervising build.

The driver checks that the instrumented CLI writes a nonempty profile before
running tests. Instrumented Windows builds explicitly flush profiling data before
the generated launcher exits; ordinary builds do not link the profiler. Runtime
profiles are matched to each executable's module signature before LLVM export,
then combined by maximum hits per source line. Compilation-only profiles remain
in a separate directory, as does the early CLI probe because Cargo can relink
the executable before tests. Unexpected runtime profiles or LLVM diagnostics reject
the collection instead of producing validated evidence.

The lower-level `proxy coverage-run` resolves relative paths before changing
directories. `EOIE_COVERAGE_JOBS`, then `CARGO_BUILD_JOBS`, overrides its default
of at most two jobs; explicit values must be 1–256. Invalid settings are rejected
before clearing profiles. LCOV assessment rejects malformed or ambiguous line
evidence, pruning requires the exact rooted source file, and failed exports
preserve existing LCOV data and receipts.

The legacy `smoke` scope requires a separately staged strict EOIE release,
`EOIE_GRCOV` as an absolute executable path, and `EOIE_LLVM_BIN` containing
`llvm-profdata` and `llvm-cov`. It checks these prerequisites before creating
build output or clearing profiles. Mutable phases use a bounded temporary copy,
reject links and Windows junctions, and leave shared compiler sessions running.
The legacy phase inventory still needs a complete Windows rehearsal; use the
native workspace coverage and probe drivers above for current source validation.

Pruning validates the receipt ledger and rollback capability before moving a
source file. Failed validation restores it. Concurrent replacements are preserved,
with the original recovery path reported if restoration cannot complete. Receipt
verification uses compiler `--check` and preserves adjacent C/Rust files.

## Work planning

`pwsh compiler-contracts/test-strict-preflight.ps1` stages a native distribution
from a verified source archive and records the strict bundle diagnostics under
ignored `.cache/strict-preflight/`. Repository-only `src/.gitignore` stays out of
the distribution. The audit preserves all existing evidence and reports blockers;
it does not renew receipts or certify a release. Add `-RequireReady` to return a
failure when the strict check rejects the staged tree.

```powershell
pwsh eoie.ps1 agile begin . 'Continue portable source review'
pwsh eoie.ps1 agile list .
pwsh eoie.ps1 agile list . Active
```

The optional list filter accepts `Planned`, `Active`, `Blocked`, `Paused` or
`Done` and matches the task status, including when those words appear elsewhere
in a task description. Listing does not mutate state.

See [README.windows.md](README.windows.md) for Windows filesystem and process details.
See [FIRST-COMMIT-REVIEW.md](FIRST-COMMIT-REVIEW.md) for validation and remaining release scope.
