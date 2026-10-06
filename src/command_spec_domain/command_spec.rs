#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn method0(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-h|--help"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v1.split("|").any(|item| item == &*v0);
    if v2 {
        5u64
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status|patch|agile|bundle|proxy|help"); } LIT.with(|lit| lit.clone()) };
        let mut v4: u64 = v3.split("|").position(|item| item == &*v0).map(|index| index as u64).unwrap_or(u64::MAX);
        v4
    }
}
fn method1(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch|agile|proxy"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v1.split("|").any(|item| item == &*v0);
    if v2 {
        2u64
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bundle"); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v0 == v3 ;
        if v4 {
            1u64
        } else {
            0u64
        }
    }
}
fn method2(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch:check|agile:list|agile:check|agile:lease|agile:status|agile:handoff|proxy:source-topology/1|proxy:plan-ir-inspect|proxy:plan-ir-check|proxy:legacy-surface|proxy:self-upgrade-check|proxy:fs-list|proxy:fs-read|proxy:fs-slice|proxy:fs-context|proxy:fs-search|proxy:parallel-search|proxy:parallel-manifest|proxy:product-diff|proxy:hash|proxy:hash-tree|proxy:source-stats|proxy:ingest-map|proxy:ingest-trim-check|proxy:ingest-extracted-manifest|proxy:ingest-targets|proxy:ingest-targets-derive|proxy:frontier-map|proxy:spi-map|proxy:spi-diff|proxy:source-recovery-diff|proxy:coverage-assess"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v1.split("|").any(|item| item == &*v0);
    if v2 {
        0u64
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile:record|proxy:release-closeout|proxy:spiral-session-shutdown"); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v3.split("|").any(|item| item == &*v0);
        if v4 {
            1u64
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile:begin"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v0 == v5 ;
            if v6 {
                3u64
            } else {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bundle:retention-apply"); } LIT.with(|lit| lit.clone()) };
                let mut v8: bool = v0 == v7 ;
                if v8 {
                    2u64
                } else {
                    4u64
                }
            }
        }
    }
}
fn method3(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0|1|2|3|4|5"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status\npatch\nagile\nbundle\nproxy\nhelp"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = v1.split("|").zip(v2.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v3 == v4 ;
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown public verb"); } LIT.with(|lit| lit.clone()) };
        (v6.clone(), v7.clone(), v6.clone(), v8.clone(), v6.clone())
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect\nmutate\nmutate\nwrap\nmutate\ninspect"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = v1.split("|").zip(v9.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [Planned|Active|Blocked|Paused|Done] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|eoie] | create <root> <output.zip> [auto|generic|eoie] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|eoie] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|eoie] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered ...\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = v1.split("|").zip(v11.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect migration evidence toolchain and authority state\nrehearse gate and apply typed patch plans\nmanage typed backlog leases and turn receipts\ncheck create verify rehydrate and retain bundles\nexecute bounded filesystem process archive toolchain and evidence capabilities\nrender the public command catalog"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = v1.split("|").zip(v13.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        (v3.clone(), v10.clone(), v12.clone(), v14.clone(), v3.clone())
    }
}
fn method4(mut v0: Rc<str>) -> Rc<str> {
    let (mut v1, mut v2, mut v3, mut v4, mut v5): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = method3(v0.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"name\":\""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"effect\":\""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v1, &*v7, &*v2, &*v8].concat());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"handler\":\""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"summary\":\""); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v5, &*v11, &*v4, &*v8].concat());
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"usage\":\""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"}"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v3, &*v14, &*v15, &*v15].concat());
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v12, &*v16, &*v15, &*v15].concat());
    v17.clone()
}
fn method5() -> i32 {
    6i32
}
fn method6() -> i32 {
    let mut v0: i32 = method5();
    let mut v1: bool = 6i32 == v0;
    if v1 {
        1i32
    } else {
        0i32
    }
}
fn method7(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cargo-lock|cargo-fmt-check|cargo-check|cargo-test|cargo-clippy|cargo-build|cargo-version|spiral-version"); } LIT.with(|lit| lit.clone()) };
    let mut v2: u64 = v1.split("|").position(|item| item == &*v0).map(|index| index as u64).unwrap_or(u64::MAX);
    v2
}
fn method8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan-ir-inspect|plan-ir-check|install-self|self-upgrade-check|fs-chmod|fs-symlink|fs-copy-tree|command-capture|command-capture-env|fs-list|fs-read|fs-context|fs-search|parallel-search|hash|hash-tree|fs-write|text-replace|fs-remove|fs-remove-tree|run|archive-extract|zip-extract|toolchain|portable-toolchain|legacy-surface|restart-baton|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-slice|parallel-manifest|product-diff|fs-copy|spiral-session-shutdown|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|source-map|spi-diff|source-diff|source-recovery-diff|cold-rebuild-matrix|cold-rebuild-owner|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proxy plan-ir-inspect <index>\nproxy plan-ir-check\nproxy install-self <source-binary> <installed-binary> <canonical-link>\nproxy self-upgrade-check <current-root> <candidate-root>\nproxy fs-chmod <root> <relative-path> <octal-mode>\nproxy fs-symlink <root> <target-relative> <link-relative>\nproxy fs-copy-tree <source> <destination>\nproxy command-capture <root> <receipt-relative> <timeout-ms> <program> [args...]\nproxy command-capture-env <root> <receipt-relative> <timeout-ms> <cwd-relative> <env-cell> <program> [args...]\nproxy fs-list <absolute-directory>\nproxy fs-read <root> <relative>\nproxy fs-context <root> <relative> <needle> <radius-bytes>\nproxy fs-search <root> <relative> <needle>\nproxy parallel-search <root> <relative> <needle> <max-hits>\nproxy hash <root> <relative>\nproxy hash-tree <root> <relative>\nproxy fs-write <root> <relative> [--replace-existing] [--expect-hash <sha256>] [--expect-nonempty] [--] <text>\nproxy text-replace <check|apply> <root> <relative> <old> <new>\nproxy fs-remove <root> <relative>\nproxy fs-remove-tree <root> <relative> [--missing-ok]\nproxy run --cwd <directory> --program <program> [--timeout-ms <ms>] [--env KEY=VALUE]... [--unset KEY]... [-- <args...>]\nproxy archive-extract <archive> <destination>\nproxy zip-extract <archive> <destination>\nproxy toolchain <root> <action> ...\nproxy portable-toolchain <root> ...\nproxy legacy-surface\nproxy restart-baton <root> <receipt-relative> <bundle-absolute> <expected-migration> <active-workstream> <touched-sources> <next-gate>\nproxy command-capture-matrix <root> <spec-relative> <timeout-ms> <report-relative> [parallel-chains [auto | start-chain chain-count]]\nproxy command-capture-matrix-gate <root> <report-relative>\nproxy batch-plan <root> <spec-relative> <timeout-ms> <report-relative> [parallel-chains [auto | start-chain chain-count]]\nproxy batch-plan-resume <root> <report-relative> <resume-relative>\nproxy fs-slice <root> <relative> <start> <count>\nproxy parallel-manifest <root> <relative> <max-entries> [preview|apply <output-relative>]\nproxy product-diff <left-root> <right-root>\nproxy fs-copy <root> <source> <target>\nproxy spiral-session-shutdown <compiler> [timeout-ms]\nproxy prune-build-cache <root> <relative-target>\nproxy prune-compiler-sidecars <root> [--dry-run]\nproxy incident-recovery snapshot <root> <source-relative> <snapshot-parent-relative> <receipt-relative> | restore <root> <target-relative> <snapshot-relative> <receipt-relative>\nproxy external-payload <register|hydrate|dehydrate> <root> <payload-relative> <cache-relative> <receipt-relative>\nproxy differential-compare <root> <oracle> <direct> <receipt>\nproxy coverage-assess <test.lcov> <smoke.lcov> <test-floor> <smoke-floor> <combined-floor>\nproxy coverage-export <profraw-dir> <binary-path> <grcov> <llvm-bin-dir> <source-root> <output-lcov> <timeout-ms>\nproxy coverage-union <output-lcov> <input-lcov> <input-lcov> [input-lcov ...]\nproxy coverage-run <root> <cargo> <compiler> <target-dir> <profraw-dir> <public|workspace|smoke|owner:package> <timeout-ms>\nproxy source-stats <root>\nproxy source-topology <root> [state/authority_census.spi]\nproxy ingest-map <root>\nproxy ingest-trim-check <root>\nproxy ingest-extracted-manifest <root>\nproxy ingest-targets <root>\nproxy ingest-targets-derive <root>\nproxy frontier-map <root>\nproxy spi-map <root>\nproxy source-map <root>\nproxy spi-diff <left-root> <right-root>\nproxy source-diff <left-root> <right-root>\nproxy source-recovery-diff <current-root> <candidate-root> [candidate-root ...]\nproxy cold-rebuild-matrix <root> <matrix-relative> <eoie> <compiler> <dotnet> <rustfmt> [compiler-timeout-ms]\nproxy cold-rebuild-owner <root> <snapshot-relative> <input-relative> <output-relative> <compiler> <dotnet> <rustfmt-or-empty> <timeout-ms> <workspace-root-or-empty>\nproxy source-author <preview|apply> <root> <target-relative> <template-relative> <expected-target-sha256|missing> <expected-template-sha256> KEY=VALUE...\nproxy fs-batch-write <preview|apply> <root> <plan.spi>\nproxy fs-actions <preview|apply> <root> <plan.spi>\nproxy binary-install <preview|apply> <root> <source-relative> <destination-relative> <provenance-relative> <expected-source-sha256> <expected-destination-sha256|missing> <expected-provenance-sha256|missing> <mode-octal>\nproxy release-closeout <preview|apply> <root> <receipt-relative> <expected-migration> <expected-receipt-sha256|missing> <name|relative|sha256>... (six gates)\nproxy shim install <root> <relative-dir> <block|warn|pass> | uninstall <root> <relative-dir> | status <root> <relative-dir>\nproxy prune-uncovered <root> <relative> <test.lcov> <smoke.lcov> -- <validation-program> [args...]"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = v1.split("|").zip(v2.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    v3.clone()
}
fn method9(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan-ir-inspect|plan-ir-check|prune-compiler-sidecars|incident-recovery|external-payload|coverage-export|coverage-union|legacy-surface|install-self|self-upgrade-check|restart-baton|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|fs-chmod|fs-symlink|fs-copy-tree|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|portable-toolchain|product-diff|source-recovery-diff"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v1.split("|").any(|item| item == &*v0);
    if v2 {
        let mut v3: u64 = v1.split("|").position(|item| item == &*v0).map(|index| index as u64).unwrap_or(u64::MAX);
        v3
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|hash|hash-tree|fs-copy"); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v4.split("|").any(|item| item == &*v0);
        if v5 {
            29u64
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("archive-extract"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v0 == v6 ;
            if v7 {
                30u64
            } else {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zip-extract"); } LIT.with(|lit| lit.clone()) };
                let mut v9: bool = v0 == v8 ;
                if v9 {
                    31u64
                } else {
                    32u64
                }
            }
        }
    }
}
fn method10() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan-ir-inspect"); } LIT.with(|lit| lit.clone()) };
    let mut v1: u64 = method9(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("source-recovery-diff"); } LIT.with(|lit| lit.clone()) };
    let mut v3: u64 = method9(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash-tree"); } LIT.with(|lit| lit.clone()) };
    let mut v5: u64 = method9(v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("archive-extract"); } LIT.with(|lit| lit.clone()) };
    let mut v7: u64 = method9(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zip-extract"); } LIT.with(|lit| lit.clone()) };
    let mut v9: u64 = method9(v8.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-write"); } LIT.with(|lit| lit.clone()) };
    let mut v11: u64 = method9(v10.clone());
    let mut v12: bool = 0u64 == v1;
    if v12 {
        let mut v13: bool = 28u64 == v3;
        if v13 {
            let mut v14: bool = 29u64 == v5;
            if v14 {
                let mut v15: bool = 30u64 == v7;
                if v15 {
                    let mut v16: bool = 31u64 == v9;
                    if v16 {
                        let mut v17: bool = 32u64 == v11;
                        if v17 {
                            1i32
                        } else {
                            0i32
                        }
                    } else {
                        0i32
                    }
                } else {
                    0i32
                }
            } else {
                0i32
            }
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn method11(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--completion"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        0u64
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v0 == v3 ;
        if v4 {
            1u64
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--schema"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v0 == v5 ;
            if v6 {
                2u64
            } else {
                3u64
            }
        }
    }
}
fn method12() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--completion"); } LIT.with(|lit| lit.clone()) };
    let mut v1: u64 = method11(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
    let mut v3: u64 = method11(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--schema"); } LIT.with(|lit| lit.clone()) };
    let mut v5: u64 = method11(v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let mut v7: u64 = method11(v6.clone());
    let mut v8: bool = 0u64 == v1;
    if v8 {
        let mut v9: bool = 1u64 == v3;
        if v9 {
            let mut v10: bool = 2u64 == v5;
            if v10 {
                let mut v11: bool = 3u64 == v7;
                if v11 {
                    1i32
                } else {
                    0i32
                }
            } else {
                0i32
            }
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method0(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method1(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method2(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        method3(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        method4(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        method5()
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        method6()
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method7(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        method8(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method9(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure10() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        method10()
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure11() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method11(v0.clone())
    }); }
    CLOSURE.with(|closure| closure.clone())
}
fn closure12() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        method12()
    }); }
    CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_command_code(v0: &str) -> u64 {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_command_effect_code(v0: &str) -> u64 {
    closure1()(Rc::<str>::from(v0))
}
pub fn eoie_command_subcommand_effect_code(v0: &str) -> u64 {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_command_descriptor(v0: &str) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure3()(Rc::<str>::from(v0))
}
pub fn eoie_command_descriptor_json(v0: &str) -> Rc<str> {
    closure4()(Rc::<str>::from(v0))
}
pub fn eoie_command_count() -> i32 {
    closure5()()
}
pub fn eoie_command_schema_witness() -> i32 {
    closure6()()
}
pub fn eoie_toolchain_action_name_code(v0: &str) -> u64 {
    closure7()(Rc::<str>::from(v0))
}
pub fn eoie_command_proxy_usage(v0: &str) -> Rc<str> {
    closure8()(Rc::<str>::from(v0))
}
pub fn eoie_proxy_route_code(v0: &str) -> u64 {
    closure9()(Rc::<str>::from(v0))
}
pub fn eoie_proxy_route_witness() -> i32 {
    closure10()()
}
pub fn eoie_help_flag_code(v0: &str) -> u64 {
    closure11()(Rc::<str>::from(v0))
}
pub fn eoie_help_flag_witness() -> i32 {
    closure12()()
}
