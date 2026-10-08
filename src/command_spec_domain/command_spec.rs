#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_0,
    US0_1,
    US0_2,
    US0_3,
    US0_4,
    US0_5,
    US0_6,
    US0_7,
    US0_8,
    US0_9,
    US0_10,
    US0_11,
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_0 => 0,
            US0::US0_1 => 1,
            US0::US0_2 => 2,
            US0::US0_3 => 3,
            US0::US0_4 => 4,
            US0::US0_5 => 5,
            US0::US0_6 => 6,
            US0::US0_7 => 7,
            US0::US0_8 => 8,
            US0::US0_9 => 9,
            US0::US0_10 => 10,
            US0::US0_11 => 11,
        }
    }
}
#[derive(Clone)]
enum US1 {
    US1_0,
    US1_1(Rc<str>),
    US1_2(Rc<str>),
}
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_0 => 0,
            US1::US1_1(..) => 1,
            US1::US1_2(..) => 2,
        }
    }
}
fn command_code_0(mut v0: Rc<str>) -> u64 {
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
fn command_effect_code_1(mut v0: Rc<str>) -> u64 {
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
fn command_subcommand_effect_code_2(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch:check|proxy:text-replace:check|proxy:fs-batch-write:preview|proxy:fs-actions:preview|proxy:source-author:preview|agile:list|agile:check|agile:lease|agile:status|agile:handoff|proxy:source-topology/1|proxy:plan-ir-inspect|proxy:plan-ir-check|proxy:legacy-surface|proxy:self-upgrade-check|proxy:fs-list|proxy:fs-read|proxy:fs-slice|proxy:fs-context|proxy:fs-search|proxy:parallel-search|proxy:parallel-manifest|proxy:product-diff|proxy:hash|proxy:hash-tree|proxy:source-stats|proxy:ingest-map|proxy:ingest-trim-check|proxy:ingest-extracted-manifest|proxy:ingest-targets|proxy:ingest-targets-derive|proxy:frontier-map|proxy:spi-map|proxy:spi-diff|proxy:source-recovery-diff|proxy:coverage-assess"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v1.split("|").any(|item| item == &*v0);
    if v2 {
        0u64
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile:record|proxy:release-closeout|proxy:spiral-session-shutdown"); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v3.split("|").any(|item| item == &*v0);
        if v4 {
            1u64
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile:begin|agile:init"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v5.split("|").any(|item| item == &*v0);
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
fn command_descriptor_by_index_3(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
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
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [Planned|Active|Blocked|Paused|Done] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|eoie] | create <root> <output.zip> [auto|generic|eoie] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|eoie] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|eoie] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered ...\nhelp [verb|--completion|--json|--schema] | proxy <capability>"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = v1.split("|").zip(v11.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inspect migration evidence toolchain and authority state\nrehearse gate and apply typed patch plans\nmanage typed backlog leases and turn receipts\ncheck create verify rehydrate and retain bundles\nexecute bounded filesystem process archive toolchain and evidence capabilities\nrender the public command catalog"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = v1.split("|").zip(v13.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        (v3.clone(), v10.clone(), v12.clone(), v14.clone(), v3.clone())
    }
}
fn command_descriptor_json_by_index_4(mut v0: Rc<str>) -> Rc<str> {
    let (mut v1, mut v2, mut v3, mut v4, mut v5): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = command_descriptor_by_index_3(v0.clone());
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
fn command_count_5() -> i32 {
    6i32
}
fn command_schema_witness_6() -> i32 {
    let mut v0: i32 = command_count_5();
    let mut v1: bool = 6i32 == v0;
    if v1 {
        1i32
    } else {
        0i32
    }
}
fn toolchain_action_code_7(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cargo-lock|cargo-fmt-check|cargo-check|cargo-test|cargo-clippy|cargo-build|cargo-version|spiral-version"); } LIT.with(|lit| lit.clone()) };
    let mut v2: u64 = v1.split("|").position(|item| item == &*v0).map(|index| index as u64).unwrap_or(u64::MAX);
    v2
}
fn proxy_usage_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan-ir-inspect|plan-ir-check|install-self|self-upgrade-check|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|fs-list|fs-read|fs-context|fs-search|parallel-search|hash|hash-tree|fs-write|text-replace|fs-remove|fs-remove-tree|run|archive-extract|zip-extract|toolchain|portable-toolchain|legacy-surface|restart-baton|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-slice|parallel-manifest|product-diff|fs-copy|spiral-session-shutdown|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|source-map|spi-diff|source-diff|source-recovery-diff|cold-rebuild-matrix|cold-rebuild-owner|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proxy plan-ir-inspect <index>\nproxy plan-ir-check\nproxy install-self <source-binary> <installed-binary> <canonical-link>\nproxy self-upgrade-check <current-root> <candidate-root>\nproxy fs-chmod <root> <relative-path> <octal-mode>\nproxy fs-symlink <root> <target-relative> <link-relative>\nproxy fs-copy-tree <source> <destination>\nproxy command-output <root> <timeout-ms> <program> [args...]\nproxy command-capture <root> <receipt-relative> <timeout-ms> <program> [args...]\nproxy command-capture-env <root> <receipt-relative> <timeout-ms> <cwd-relative> <env-cell> <program> [args...]\nproxy fs-list <absolute-directory>\nproxy fs-read <root> <relative>\nproxy fs-context <root> <relative> <needle> <radius-bytes>\nproxy fs-search <root> <relative> <needle>\nproxy parallel-search <root> <relative> <needle> <max-hits>\nproxy hash <root> <relative>\nproxy hash-tree <root> <relative>\nproxy fs-write <root> <relative> [--replace-existing] [--expect-hash <sha256>] [--expect-nonempty] [--] <text>\nproxy text-replace <check|apply> <root> <relative> <old> <new>\nproxy fs-remove <root> <relative>\nproxy fs-remove-tree <root> <relative> [--missing-ok]\nproxy run --cwd <directory> --program <program> [--timeout-ms <ms>] [--env KEY=VALUE]... [--unset KEY]... [-- <args...>]\nproxy archive-extract <archive> <destination>\nproxy zip-extract <archive> <destination>\nproxy toolchain <root> <action> ...\nproxy portable-toolchain <root> ...\nproxy legacy-surface\nproxy restart-baton <root> <receipt-relative> <bundle-absolute> <expected-migration> <active-workstream> <touched-sources> <next-gate>\nproxy command-capture-matrix <root> <spec-relative> <timeout-ms> <report-relative> [parallel-chains [auto | start-chain chain-count]]\nproxy command-capture-matrix-gate <root> <report-relative>\nproxy batch-plan <root> <spec-relative> <timeout-ms> <report-relative> [parallel-chains [auto | start-chain chain-count]]\nproxy batch-plan-resume <root> <report-relative> <resume-relative>\nproxy fs-slice <root> <relative> <start> <count>\nproxy parallel-manifest <root> <relative> <max-entries> [preview|apply <output-relative>]\nproxy product-diff <left-root> <right-root>\nproxy fs-copy <root> <source> <target>\nproxy spiral-session-shutdown <compiler> [timeout-ms]\nproxy prune-build-cache <root> <relative-target>\nproxy prune-compiler-sidecars <root> [--dry-run]\nproxy incident-recovery snapshot <root> <source-relative> <snapshot-parent-relative> <receipt-relative> | restore <root> <target-relative> <snapshot-relative> <receipt-relative>\nproxy external-payload <register|hydrate|dehydrate> <root> <payload-relative> <cache-relative> <receipt-relative>\nproxy differential-compare <root> <oracle> <direct> <receipt>\nproxy coverage-assess <test.lcov> <smoke.lcov> <test-floor> <smoke-floor> <combined-floor>\nproxy coverage-export <profraw-dir> <binary-path> <grcov> <llvm-bin-dir> <source-root> <output-lcov> <timeout-ms>\nproxy coverage-union <output-lcov> <input-lcov> <input-lcov> [input-lcov ...]\nproxy coverage-run <root> <cargo> <compiler> <target-dir> <profraw-dir> <public|workspace|smoke|owner:package> <timeout-ms>\nproxy source-stats <root>\nproxy source-topology <root> [state/authority_census.spi]\nproxy ingest-map <root>\nproxy ingest-trim-check <root>\nproxy ingest-extracted-manifest <root>\nproxy ingest-targets <root>\nproxy ingest-targets-derive <root>\nproxy frontier-map <root>\nproxy spi-map <root>\nproxy source-map <root>\nproxy spi-diff <left-root> <right-root>\nproxy source-diff <left-root> <right-root>\nproxy source-recovery-diff <current-root> <candidate-root> [candidate-root ...]\nproxy cold-rebuild-matrix <root> <matrix-relative> <eoie> <compiler> <dotnet> <rustfmt> [compiler-timeout-ms]\nproxy cold-rebuild-owner <root> <snapshot-relative> <input-relative> <output-relative> <compiler> <dotnet> <rustfmt-or-empty> <timeout-ms> <workspace-root-or-empty>\nproxy source-author <preview|apply> <root> <target-relative> <template-relative> <expected-target-sha256|missing> <expected-template-sha256> KEY=VALUE...\nproxy fs-batch-write <preview|apply> <root> <plan.spi>\nproxy fs-actions <preview|apply> <root> <plan.spi>\nproxy binary-install <preview|apply> <root> <source-relative> <destination-relative> <provenance-relative> <expected-source-sha256> <expected-destination-sha256|missing> <expected-provenance-sha256|missing> <mode-octal>\nproxy release-closeout <preview|apply> <root> <receipt-relative> <expected-migration> <expected-receipt-sha256|missing> <name|relative|sha256>... (six gates)\nproxy shim install <root> <relative-dir> <block|warn|pass> | uninstall <root> <relative-dir> | status <root> <relative-dir>\nproxy prune-uncovered <root> <relative> <test.lcov> <smoke.lcov> -- <validation-program> [args...]"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = v1.split("|").zip(v2.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    v3.clone()
}
fn proxy_route_code_9(mut v0: Rc<str>) -> u64 {
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
fn proxy_route_witness_10() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan-ir-inspect"); } LIT.with(|lit| lit.clone()) };
    let mut v1: u64 = proxy_route_code_9(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("source-recovery-diff"); } LIT.with(|lit| lit.clone()) };
    let mut v3: u64 = proxy_route_code_9(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash-tree"); } LIT.with(|lit| lit.clone()) };
    let mut v5: u64 = proxy_route_code_9(v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("archive-extract"); } LIT.with(|lit| lit.clone()) };
    let mut v7: u64 = proxy_route_code_9(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zip-extract"); } LIT.with(|lit| lit.clone()) };
    let mut v9: u64 = proxy_route_code_9(v8.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-write"); } LIT.with(|lit| lit.clone()) };
    let mut v11: u64 = proxy_route_code_9(v10.clone());
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
fn help_flag_code_11(mut v0: Rc<str>) -> u64 {
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
fn help_flag_witness_12() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--completion"); } LIT.with(|lit| lit.clone()) };
    let mut v1: u64 = help_flag_code_11(v0.clone());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
    let mut v3: u64 = help_flag_code_11(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--schema"); } LIT.with(|lit| lit.clone()) };
    let mut v5: u64 = help_flag_code_11(v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let mut v7: u64 = help_flag_code_11(v6.clone());
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
fn agile_subcommand_known_13(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin|lease|status|handoff|list|check|record|set|add|edit|init"); } LIT.with(|lit| lit.clone()) };
    let mut v2: u64 = v1.split("|").position(|item| item == &*v0).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v3: bool = 0u64 == v2;
    let mut v36: US0 = if v3 {
        US0::US0_0
    } else {
        let mut v5: bool = 1u64 == v2;
        if v5 {
            US0::US0_1
        } else {
            let mut v7: bool = 2u64 == v2;
            if v7 {
                US0::US0_2
            } else {
                let mut v9: bool = 3u64 == v2;
                if v9 {
                    US0::US0_3
                } else {
                    let mut v11: bool = 4u64 == v2;
                    if v11 {
                        US0::US0_4
                    } else {
                        let mut v13: bool = 5u64 == v2;
                        if v13 {
                            US0::US0_5
                        } else {
                            let mut v15: bool = 6u64 == v2;
                            if v15 {
                                US0::US0_6
                            } else {
                                let mut v17: bool = 7u64 == v2;
                                if v17 {
                                    US0::US0_7
                                } else {
                                    let mut v19: bool = 8u64 == v2;
                                    if v19 {
                                        US0::US0_8
                                    } else {
                                        let mut v21: bool = 9u64 == v2;
                                        if v21 {
                                            US0::US0_9
                                        } else {
                                            let mut v23: bool = 10u64 == v2;
                                            if v23 {
                                                US0::US0_10
                                            } else {
                                                US0::US0_11
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    match &v36 {
        US0::US0_11 => {
            0i32
        }
        _ => {
            1i32
        }
    }
}
fn help_flag_from_15(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let mut v2: bool = v0 >= v1;
        if v2 {
            return false;
        } else {
            let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--help|-h"); } LIT.with(|lit| lit.clone()) };
            let mut v5: bool = v4.split("|").any(|item| item == &*v3);
            if v5 {
                return true;
            } else {
                let mut v6: i32 = v0 + 1;
                (v0, v1) = (v6, v1);
                continue;
            }
        }
    }
}
fn help_request_text_14() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(1i32 as usize).unwrap_or_default());
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2i32 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("help"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v0 == v2 ;
    let mut v31: US1 = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v1 == v4 ;
        let mut v9: bool = if v5 {
            false
        } else {
            let mut v6: Rc<str> = proxy_usage_8(v1.clone());
            let mut v7: bool = v6 == v4 ;
            let mut v8: bool = v7 == false;
            v8
        };
        if v9 {
            US1::US1_2(v1.clone())
        } else {
            US1::US1_0
        }
    } else {
        let mut v13: i32 = 2i32;
        let mut v14: i32 = std::env::args().count() as i32;
        let mut v15: bool = help_flag_from_15(v13, v14);
        if v15 {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proxy"); } LIT.with(|lit| lit.clone()) };
            let mut v17: bool = v0 == v16 ;
            if v17 {
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v19: bool = v1 == v18 ;
                let mut v23: bool = if v19 {
                    false
                } else {
                    let mut v20: Rc<str> = proxy_usage_8(v1.clone());
                    let mut v21: bool = v20 == v18 ;
                    let mut v22: bool = v21 == false;
                    v22
                };
                if v23 {
                    US1::US1_2(v1.clone())
                } else {
                    US1::US1_1(v0.clone())
                }
            } else {
                US1::US1_1(v0.clone())
            }
        } else {
            US1::US1_0
        }
    };
    match &v31 {
        US1::US1_2(v50) => {
            let mut v50: Rc<str> = v50.clone();
            let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie "); } LIT.with(|lit| lit.clone()) };
            let mut v52: Rc<str> = proxy_usage_8(v50.clone());
            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v51, &*v52, &*v53, &*v53, &*v53].concat());
            v54.clone()
        }
        US1::US1_0 => {
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v32.clone()
        }
        US1::US1_1(v33) => {
            let mut v33: Rc<str> = v33.clone();
            let mut v34: u64 = command_code_0(v33.clone());
            let mut v35: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
            let (mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = command_descriptor_by_index_3(v35.clone());
            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("name="); } LIT.with(|lit| lit.clone()) };
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" effect="); } LIT.with(|lit| lit.clone()) };
            let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" handler="); } LIT.with(|lit| lit.clone()) };
            let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v36, &*v42, &*v37, &*v43].concat());
            let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nsummary="); } LIT.with(|lit| lit.clone()) };
            let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nusage: eoie "); } LIT.with(|lit| lit.clone()) };
            let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v45, &*v39, &*v46, &*v38].concat());
            let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v47, &*v48, &*v48, &*v48].concat());
            v49.clone()
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        command_code_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        command_effect_code_1(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        command_subcommand_effect_code_2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        command_descriptor_by_index_3(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        command_descriptor_json_by_index_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        command_count_5()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        command_schema_witness_6()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        toolchain_action_code_7(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        proxy_usage_8(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        proxy_route_code_9(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure10() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        proxy_route_witness_10()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure11() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        help_flag_code_11(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure12() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        help_flag_witness_12()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure13() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        agile_subcommand_known_13(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure14() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        help_request_text_14()
    }); } CLOSURE.with(|closure| closure.clone())
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
pub fn eoie_agile_subcommand_known(v0: &str) -> i32 {
    closure13()(Rc::<str>::from(v0))
}
pub fn eoie_help_request_text() -> Rc<str> {
    closure14()()
}
