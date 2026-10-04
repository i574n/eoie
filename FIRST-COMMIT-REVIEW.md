# First source commit review — 2026-10-02

Scope: the EOIE source workspace and its supported single-flight compiler
integration. This is separate from certifying a distributable EOIE release.

## Native Spiral correction — October 2, evening

The predicate codemod now implements lexing, matching and rewriting in native
Spiral, with 14 native checks. Its typed predicate library uses GADTs,
existential projections and an HKT registry. Three production helpers were
mechanically replaced, removing 335 bytes of embedded Rust; differential tests
cover all ASCII bytes at four lengths plus empty, mixed and Unicode inputs.
Two compiler fixtures reject incorrect predicate inputs and registry carriers.

Cold-proof binary test scenarios now originate in Spiral. The native `eoie-dev`
driver owns build/test ordering and selected-package metadata policy; its
selection contract and all twelve transitional regeneration contracts passed.
Regeneration and binary publication still use PowerShell adapters. The current
inventory contains 94 primary and 12 auxiliary generation declarations,
37 Rust files without generation mappings, and 14 PowerShell scripts. Complete
Spiral ownership remains unfinished; wrapped Rust algorithms do not count as
native Spiral migration. The prior coverage and source-package snapshots below
predate this correction and are not evidence for the new files.

Validation of this correction: **184 Windows tests passed, zero failed or
skipped**, including all 17 optional compiler contracts, plus the 14 checks in
the native codemod's separate test runner. Locked offline Linux all-target
compilation passed from Windows; Linux runtime execution was not performed.
Source topology passes at 998 lines for the largest crate and 988 for the
largest Rust file, under the unchanged 1,000-line limits. The generated outputs
were regenerated and validated in isolated workspaces before publication.
Evidence logs are under ignored `.cache/mechanical-migration/`:
`full-native-workspace.log`, `linux-all-targets.log`,
`workflow-compatibility.log`, and `type-rejection-results.tsv`.

A unit-returning tail-recursive compiler loop was observed to hang. The minimal
`compiler-contracts/unit-tail-return` fixture and `COMPILER-UNIT-TAIL-RETURN`
agile task retain that unresolved compiler issue. The migrated tests use
value-returning loops; do not run the failing fixture without a timeout.

## Earlier Windows checkpoint — October 2

The source milestone has native Windows build and runtime evidence. A certified
EOIE distribution remains a separate, unfinished milestone.

| Check | Observed result |
| --- | --- |
| Full regeneration | All 98 outputs (89 primary owners and nine auxiliary outputs), all-target Cargo checks and 173 tests passed, with no failures or skips. Later changed owners were regenerated and tested separately. |
| Native release | Final workspace run: 182 tests passed, zero failed or skipped, including all 17 compiler contracts. The optimized candidate built, but its installation was blocked by the wrap-time topology classification issue below; the previous executable is preserved. |
| Instrumented snapshot | 180 tests passed: 163 native tests plus all 17 compiler contracts. All 102 executables exported without LLVM diagnostics; all 533 runtime profiles matched. |
| Workspace line coverage | 595/1000 over 25,594 executable lines, including tests. This is test-run coverage of the frozen snapshot, not release-smoke evidence. The earlier baseline was 543/1000 over 24,131 lines. |
| Linux compilation | Locked offline all-target checks passed for x86_64-unknown-linux-gnu from Windows. No WSL or Linux runtime execution was used. |
| Developer workflow | Twelve development scenarios, five selected-build scenarios and twelve binary-publication scenarios passed. |
| Native examples | All 20 Cargo-declared examples rebuilt and passed in the verified 982-file snapshot. |
| Source packaging | A 983-file snapshot passed archive, rehydration, every SHA-256 and topology checks. The earlier 982-file snapshot also rebuilt and passed 163 native tests, with 17 optional compiler tests skipped there and passed separately. Vendor/build products were absent. |
| Source limits | 89 Cargo crates; largest crate 998 lines and largest Rust file 982 lines, below the 1,000-line limits. |

The coverage snapshot predates the final Linux-only FIFO guard, handoff wording
and owner-growth receipt publication fix. These changes have separate validation;
the bounded FIFO regression still awaits runtime validation on a Linux host.

Key fixes and development improvements:

- Agile attestations bind compiler dependencies and package graphs. Task changes
  preserve unrelated source drift, and concurrent input changes block receipts.
- Generated owners declare their source/output mapping in Cargo metadata.
  Isolated development preserves unchanged timestamps, contains compiler sidecars,
  rejects source drift and builds the CLI for tests that require it.
- Windows copies stream. Tree copies reject overlap and linked ancestors, enforce
  bounded Unicode enumeration, stage complete output and preserve competing
  destinations. Traversal admission and remaining-entry policy are authored in Spiral.
- Chmod uses confined handles/descriptors. Nested symlinks retain root-relative
  meaning. Filesystem-action copies and links preserve failed preparation and
  detected concurrent edits; rollback restores originals, including read-only
  Windows output. Sharing locks and long paths have regressions.
- Shared hashing uses bounded nofollow reads. Linux leaf opens use nonblocking
  mode so regular-file checks can reject FIFOs without waiting for a writer.
- Self-upgrade discovers exactly one supported executable name and preserves all
  six strict compatibility checks. Its verifier fixtures test that protocol;
  they do not certify a release.
- Cleanup rejects junction escapes and preserves ignored caches. ZIP publication
  validates a stage before replacement, bounds expansion and applies strict checks
  to extracted release contents.
- Owner-growth receipts use confined atomic writes, preserving unrelated fixed
  temporary files and rejecting linked state directories. Both regressions failed
  before the fix and pass after it. Predecessor ZIP staging remains tracked separately.
- Agile handoff labels saved evidence as recorded state and explicitly states that
  rendering does not revalidate freshness. Its guidance uses registry dependencies
  and the actual lease timestamps; Linux runtime work now explicitly excludes WSL.
- Batch fixtures run from their temporary directory and clear inherited lease
  settings, so an expired checkout lease does not invalidate isolated tests.

Closeout limitation: after the 19:50 wrap boundary, candidate publication rejected
the read-only `source-topology` check as a mutation. All 182 tests and the optimized
build passed; `src/target/dev-validation/release/eoie.exe` contains the latest fix,
while root `eoie.exe` preserves the earlier handoff build. Agile task
`DOGFOOD-INSPECTION-LEASE-CLASSIFICATION` tracks correcting read versus write
classification before rerunning publication. The final archive snapshot predates
the one-line batch-fixture isolation fix and these closeout notes.
The same classification issue also blocks `agile handoff` during wrap. Agile
history append and check still passed at closeout (sequence 332); the failed
handoff command is retained in `.cache/dogfood-20261002/handoff-wrap-rejection.txt`.

The original Linux executable is preserved under ignored diagnostics, leaving
only the native executable at the workspace root. Vendor remains ignored.
Tracked deletions remove the superseded hygiene policy (now a package-only owner)
and a stale local Linux candidate-install receipt. No changes were staged or committed.

Local evidence is intentionally ignored:

- Full regeneration: `.cache/dogfood-20261002/full-regeneration-checkpoint.txt`.
- Native tests/build: `.cache/dogfood-20261002/full-native-copy-upgrade.txt`.
- Final tests/build and blocked publication: `.cache/dogfood-20261002/full-native-lease-isolated.txt`.
- Read-only rollback and installation: `.cache/dogfood-20261002/readonly-copy-locks.txt`
  and `release-readonly-fix.txt` in the same directory.
- Coverage: `.cache/native-coverage/a488eb78c9bd4ed9abfffe73aaf43103/evidence.json`.
- Linux check: `.cache/dogfood-20261002/linux-cross-check-closeout.txt`.
- Source rehearsal: `.cache/source-package/4a133ff576474a63a0f55c84a998536e/manifest.json`.
- Closeout archive checks: `.cache/source-package/09dfbef2154b4eb7a1bc22afd8a3e0e6/manifest.json`.
- Native examples: `.cache/native-probes/5e3734c18cf04fc996d23da8fc2f9689/evidence.json`.
- Strict audit: `.cache/strict-preflight/5981e05ec8e14dd6aee8bde9f0bfbae9/report.json`.
- Receipt publication: `.cache/dogfood-20261002/growth-publication-after.txt`.
- Handoff regeneration: `.cache/dogfood-20261002/handoff-scope-regeneration.txt`
  and `handoff-contract-regeneration.txt` in the same directory.

Strict preflight still reports five evidence blockers: incomplete owner-growth
receipts, missing coverage evidence, stale semantic closeout, stale cold proof
and absent durable coverage. Historical Linux receipts remain historical.
Generic source-archive success and the new Windows coverage snapshot do not
renew those receipts or provide strict certification.

## Historical Windows checkpoint — September 27

- Locked Cargo workspace tests, including all 17 optional compiler contracts:
  **76 passed, 0 failed, 0 ignored**. Registry dependencies are used; `vendor/`
  remains ignored. Build products and compiler scratch output remain ignored.
- The process Rust owner regenerates from its Spiral source through `spiral.ps1`.
  Earlier filesystem-action and process-budget policy migrations remain in Spiral.
- The native host typechecks the complete owning package, including modules after
  the input, without requiring `main`. EOIE's own Agile package check produced
  61 build attestations in one batch.
- Canonical Plan IR preserves string payload braces, rejects runtime-dependent
  operations, and enforces positive timeouts. The dedicated regression script
  exercises these cases and compiles and runs a specialized GADT through Rust.
- Git review excludes executables, archives, vendor, caches and build directories.
- The single-flight compiler builds cleanly. The changed Hopac host also builds
  cleanly against its cached core with output copying disabled because another
  process holds its published DLL. Hopac remains experimental.

The Windows/Linux CI matrix now builds the optional compiler and runs the
attestation regression script plus all EOIE contracts. Linux execution has not
been verified locally; the installed WSL probe did not finish starting.

## Remaining scope

`PORT-COMPILER-EXPORT-SURFACE` now has regeneration evidence for all 89 primary
Cargo owners: every source emits Rust and the isolated workspace passes locked,
offline `cargo check --workspace --all-targets`. Generated Rust remains committed
so ordinary builds do not depend on the compiler. The October 2 checkpoint also
regenerated the nine declared auxiliary probes; handwritten platform adapters
remain tested consumers. The earlier native cold replay passed **106 tests**, including
the 17 optional compiler contracts. The earlier full regeneration also built a
release executable and passed its schema smoke check.
Six export ABIs (I32, I32Binary, StringUnary, U64Unary, StringTuple5Unary and
U64String5) pass an external Rust consumer with three runtime tests and eight
rejection cases. The contracts cover UTF-8, input lifetime, tuple field order,
u64 width, duplicate markers and optimized-away parameters.

Regeneration exposed and fixed incorrect codec package imports, u64 string
lengths emitted as i32, and parameter names lost when used only by embedded
Rust expressions. The regeneration harness stages sources and products outside
the checkout and is included in the Windows/Linux CI matrix.
The review also fixed C line splitting that changed quoted `}new{` plan payloads;
adjacent block splitting now preserves strings and comments.

`PORT-STRICT-RELEASE` tracks a separately staged distribution, renewed hashes and
receipts, cold rebuild evidence and strict release validation. Historical Linux
evidence under `state/` does not certify this Windows source port. `generic`
archive success only verifies archive policy and cannot close this task.

The broader Rust compiler sweep after the return and closure fixes ran 136
fixtures: 113 emitted and ran natively, 23 rejected compilation and none timed
out. The five native build failures found before the return fix (fixed-array
getter and four SCC tail-return cases) now compile and run in focused C/Rust
tests; all agree with the C oracle. The fixed-array test also runs a negative
index to prove the final element fallback. Both managed-closure fixtures now
compile and return 42 after correcting their malformed package manifests; the
nested-closure output agrees with C. The managed tuple fixture also agrees with
C when both backends are selected. The seven-case C/Rust regression pass had no
build failures or oracle disagreements. Baselines remain unchanged, including
the newly verified native outputs. Twenty-six native examples recover compared
with the imported patch.

## Reproduction

Build the compiler with `../spiral/apps/compiler/tmp/scripts/build.ps1 -Mode single-flight` (a spiral checkout beside this repository), then run
`compiler-contracts/test-attestation.ps1` and the compiler's `scripts/test-rust-exports.ps1`.
Use `compiler-contracts/test-regeneration.ps1 -IncludeAuxiliary -CargoCheck -Test -CompilerContracts`
to reproduce the isolated 98-output regeneration and runtime validation.
Set `EOIE_DOTNET` to that compiler's .NET host
and `EOIE_SPIRAL_COMPILE` to its `SpiralCompiler.dll`, then run:

```powershell
pwsh build.ps1 -Test -CompilerContracts
./eoie.exe agile check . --compiler $env:EOIE_SPIRAL_COMPILE
```

Use `-Offline` only with a populated Cargo registry cache. See the README for
compiler relocation settings and generic versus strict bundle behavior.
