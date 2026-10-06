# First source commit review — 2026-10-02

Scope: the EOIE source workspace and its supported single-flight compiler
integration. This is separate from certifying a distributable EOIE release.

## Strict preflight — October 6

`compiler-contracts/test-strict-preflight.ps1 -RequireReady` still rejects the
staged distribution. **The source port has not reached a strict release.** This
session added the missing renewal paths and registered real coverage evidence. Preflight errors went from 4 to 3. Latest report: `.cache/strict-preflight/830d7a99cb1c44f29feb2c28622d313a/report.json` (the same 3 errors, after the differential catalog and family contract renewals). Evidence currentness now stops at `src/Cargo.toml`, which only closeout renews, and semantic closeout and ColdProofV4 are unchanged.

| Blocker (October 5) | Result |
| --- | --- |
| Coverage durability and the four missing `evidence/coverage/*` payloads | Renewed with real evidence and cleared. `proxy coverage-register` imported two Windows native coverage runs, both run with `-CompilerContracts`. The current checkpoint is the final tree: **614/1000 over 29,388 lines**, LCOV SHA `5165deed…`. The replay seed is the earlier run in this session: 600/1000 over 29,365 lines, SHA `eeb8e24e…`. Both have workspace-relative `SF:` paths and `lcov-union` receipts. This is test-run coverage that includes tests. It is not release-smoke coverage over the historical 17,303-line author-product universe, so the narrative rows of `state/coverage.spi` (804/1000) remain historical. Runs: `.cache/native-coverage/3478194ab96444cba408e369a07f8a92` and `2770c45c242344fca30327ead9847cc0`. |
| ColdProofV4 (`state/cold_proof.spi`) | Renewal path added. `proxy release-closeout apply` now renews all five gates. The binary gate records the root binary actually present (`eoie.exe` on Windows) and the workspace gate records the current `src/Cargo.lock`. The renewal is kept only if formal authority verifies and the binary equals the ColdRebuildV1 product; otherwise it rolls back. A probe on a disposable staged copy renewed all five gates, including `eoie.exe` and the current lock. After the differential catalog and family contract renewals, apply rolls back at `cold rebuild owner inventory drift receipt=94 declared=106` (live checkout and staged copy) and leaves zero state files changed. |
| Semantic closeout (`state/release_closeout.spi`) | The closeout evidence refresh records `eoie` or `eoie.exe`, whichever single binary the root carries, instead of a hardcoded `eoie`. Closeout still cannot be applied, because its cold-proof step rolls back as above. |
| Evidence identity (`state/evidence.spi`) | Only `proxy release-closeout apply` renews the remaining rows. It cannot run until the cold proof can be renewed. |

The cold proof cannot be renewed honestly yet. Formal authority depends on three
more receipts. Two of them now have producers and are current:

- **DifferentialCatalogV1** (`state/differential_catalog.spi`): renewed by
  `proxy formal-receipts-renew .` and also by `release-closeout apply`. Before it
  re-hashes anything, the renewal executes the root binary's `proxy differential-compare`
  route through five scenarios: parity, a deterministic receipt, a mismatch, a
  missing input and an escaping receipt. It proceeds only with all five observed
  (flags 31), then runs the full verifier (settlement 10/9 shape and markers,
  catalog 36/36, contract and route markers). Only the route gate changed
  (`b4e6b206…` to `d8ba4ffc…`).
- **FamilyContractsV1** (`state/family_contracts.spi`): renewed the same way. The full
  verifier re-checks 36 families against 36 decisions, with no unsettled family and
  every prune or port decision matching its disposition. Only the runtime gate changed
  (`64d75eed…` to `fd9b5643…`). `runtime_drift.spi` declares its baseline
  historical-only, and no other EOIE check validates it.
- After both renewals, closeout apply (live checkout and a staged copy) rolls back at
  the next unrenewable receipt, with zero state files changed:
  `cold rebuild owner inventory drift receipt=94 declared=106`.
- **ColdRebuildV1** (still no producer) (`state/cold_rebuild.spi`): it needs two independent cold rebuilds
  of the current `src` tree with byte-identical products, owner compiles under 15 s,
  a global gate under 30 s and current gate hashes. Its rustc and compiler identities
  must appear in `state/toolchain_identity.spi`, which still records the Linux payload.
  The cold-rebuild matrix replays owners but writes no receipt.

The renewal re-hashes the five gates. It carries the recorded replay flags (`15`:
rehydrate, bundle-check, status-live, byte-identical rebundle) forward without
re-observing them. Instead of re-running those replays, it requires formal authority:
a current ColdRebuildV1 whose product equals the binary. No renewal is committed
today. A producer that re-observes the four replays on the staged candidate before
promoting it is still missing.

The host already meets some of the ColdRebuildV1 conditions. Owner compiles in this
session took 1.0–4.5 s each (`dev.ps1`, single-flight, 15 compiles across 10 owners),
well under the 15 s budget.

Two release builds of `eoie-cli` used the receipt's normalization (`CARGO_INCREMENTAL=0`,
`--remap-path-prefix`, `-Cstrip=symbols`). They differed in 20 bytes: the PE
TimeDateStamp, three debug-directory timestamps and the CodeView PDB GUID. No path
differed. Adding `-Clink-arg=/Brepro` made the two builds byte-identical (SHA
`309310a5…`). A Windows ColdRebuildV1 needs that flag in its normalization.

What is missing is the receipt producer, a Windows `toolchain_identity`, and the
global-gate measurement.

Registering new coverage leaves some `state/` rows stating the old checkpoint as
current, and EOIE has no command to rewrite them:
- `coverage.spi`: `current_coverage_target`, `grcov_status`, `native_lcov_export` (804/1000, SHA `4c9e577c…`)
- `coverage_refresh.spi` (lines 56–76)
- `bench.spi` `canonical_checkpoint_durability`
- `agile.spi` `COVERAGE-CHECKPOINT-DURABILITY`

They are historical and are not the registered checkpoint. The typed
`CoverageCheckpointDurable` and `CoverageReplaySeed` rows are.

`src/evidence_currentness_domain/registration_tests.rs` is a new handwritten Rust test.
That adds one to the Rust files without a generation mapping (migration debt). The
cold-proof renewal scenario is authored in Spiral.

ColdProofV4 and the binary gate support `eoie.exe`, so a platform-specific strict
release is within the design. Its missing pieces are the three producers above plus
a Windows toolchain identity, not a Linux host.

The coverage workflow needed three fixes before it could pass on this host:

- The Spiral-generated `native_codemod` runner (`eoie-predicate-lift`, `harness = false`)
  exits through `process::exit` and wrote no profile on Windows. It now flushes under
  `cfg(all(windows, eoie_coverage))`, as the CLI does.
- The `selected_workflow` fixture's nested `eoie-dev`/Cargo processes inherited
  `RUSTFLAGS=-C instrument-coverage` and `LLVM_PROFILE_FILE`. That left five profiles
  outside the Cargo inventory, which the workflow rejects. The fixture process now
  drops both variables.
- Exported `SF:` paths are now workspace-relative, so registered LCOV does not embed
  a snapshot path.

Other fixes:

- `eoie agile handoff . | Select-Object -First 5` no longer panics when the reader
  closes the pipe (os error 232). The report is written once, and a broken pipe counts
  as success. A CLI regression test failed before the fix.
- The compiler's literal-interning fix landed: it now emits `std::rc::Rc::<str>` inside
  the cached literal. `rust_std_string` is back to its original
  `unwrap_or_else(|| std::rc::Rc::<str>::from(""))`. `command_spec_domain` and
  `authority_state_domain` regenerate, compile and pass with it.

Validation:

- Tests came first. The new cold-proof renewal scenario (Spiral, `native_binary_tests.spi`) the two differential/family renewal scenarios (`family_contract_renewal_rechecks_the_catalog_settlement_and_rolls_back_on_drift`, `differential_catalog_renewal_requires_the_public_route_and_rolls_back`) and three evidence tests (`registration_tests.rs`) failed to build before the implementation and pass after it. The handoff pipe test failed with the os error 232 panic before its fix.
- `build.ps1 -Test -CompilerContracts -Offline`: **191 passed, 0 failed or skipped**. The published `eoie.exe` is SHA `9956e08f…`.
- `flat_bundle_roundtrip_is_self_verified`, which runs closeout apply on a complete fixture, passes through the new renewal.
- The renewed `state/` package passes `eoie agile check --compiler <single-flight dll>`: 63 files, 61 build-attested. It ran on a staged copy and then on the checkout, which renewed `state/typecheck_receipts.spi`.
- `dev.ps1 -Publish` regenerated `cold_proof_domain`, `evidence_currentness_domain`, `eoie_legacy_operations`, `eoie_predicate_lift`, `eoie_dev`, `eoie_handoff`, `eoie_cli`, `command_spec_domain` and `authority_state_domain` with the current compiler (3CCCBD5D8346). The regeneration also re-emits unchanged literals in its cached form.

## Strict preflight — October 5

`compiler-contracts/test-strict-preflight.ps1 -RequireReady` still rejects the
staged distribution. **The source port has not reached a strict release.** Two
blockers were code defects and are fixed. Owner-growth receipts were renewed
through EOIE. Four evidence blockers remain, and none has a producer that can run
on this Windows host.

| Blocker | Result |
| --- | --- |
| Expired lease blocked read-only `source-topology`, so the preflight stopped before the strict check | Fixed. `proxy source-topology <root>` without a census path and `agile handoff` are now inspection effects; the census write stays a mutation. Agile task `DOGFOOD-INSPECTION-LEASE-CLASSIFICATION` is Done. |
| `expected 1 public binary, found 3` | Fixed. `eoie-dev` and `eoie-lift-predicate` declare `[package.metadata.eoie] developer-tool = true` and are excluded from the public-binary count. An undeclared second `[[bin]]` is still rejected. |
| Owner-growth receipts missing for 86 owners | Renewed with `eoie bundle growth-receipt . windows-source-port-and-native-spiral-migration PORT-STRICT-RELEASE`, which recorded 87 changed owners. The policy remains report-only. That command rewrites every change row with one reason, so the earlier `eoie_release_hygiene` reason now lives only in Git history. |
| ColdProofV4 (`state/cold_proof.spi`) | Not renewable. No EOIE command writes this receipt; contracts write it by hand only inside test fixtures. The one refresh path, `refresh_cold_proof_census_gate`, rewrites the census hash and then re-verifies all five gates. It fails on the `src/Cargo.lock` gate (`d4e2a00a…` recorded, `c06a98ea…` current). The binary gate records `eoie` with the historical Linux hash, while Windows staging carries `eoie.exe`. |
| Semantic closeout (`state/release_closeout.spi`) | Not renewable while the cold proof is stale. `proxy release-closeout preview` accepts all six resealed gates. `apply`, run on a disposable staged copy, fails at the cold-proof refresh above and rolls back every state file. Its evidence refresh also reads a root binary named `eoie`, so a Windows closeout needs that path made platform-aware. |
| Evidence closure and durable coverage | Not renewable. `state/evidence.spi` declares the durable 166,535-byte LCOV (SHA `4c9e577c…`), its receipt and the replay seed under `evidence/coverage/`. Those payloads were never imported into this repository and exist nowhere locally. No EOIE command registers new coverage evidence or durability. `coverage-export` needs grcov and LLVM tools, and neither is installed. Native workspace coverage is explicitly not release evidence. |

Linux runtime evidence was not attempted, because WSL is excluded by `PORT-LINUX-LOCAL`.
The cold-rebuild matrix was not run: its replay replaces generated outputs in a
copy, but nothing converts the result into `state/cold_rebuild.spi` or ColdProofV4
receipts. `state/cold_rebuild.spi` would be checked next. It records 94 owners and
the historical Linux product hash.

Closing `PORT-STRICT-RELEASE` therefore requires:

- a platform-aware cold-replay producer for ColdProofV4 and ColdRebuildV1;
- a closeout evidence refresh that accepts `eoie.exe`;
- a durable-coverage producer or a deliberate retirement of that evidence.

The final compiler's literal interning rewrote the `Rc::<str>::from("")` suffix of
`std::rc::Rc::<str>::from("")` and produced invalid Rust. `rust_std_string`
therefore now uses the equivalent `unwrap_or_default()`.

Validation:

- `build.ps1 -Test -CompilerContracts -Offline`: **184 passed, 0 failed or skipped**, with the release candidate published.
- The new lease and public-binary contracts failed before the fix and pass after it.
- `dev.ps1 -Package eoie-contracts,command-spec-domain,eoie-agile-lease,eoie-bundle-zip -Publish` regenerated and published the four changed owners.

The final strict report is
`.cache/strict-preflight/bb8690b24185449d80cd4b9ea3470121/report.json`.

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
