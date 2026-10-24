#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use command_spec_domain::{eoie_agile_subcommand_known, eoie_command_code, eoie_command_count, eoie_command_more_usages, eoie_command_descriptor, eoie_command_descriptor_named, eoie_command_descriptor_json, eoie_command_schema_witness, eoie_command_proxy_usage, eoie_proxy_route_code, eoie_proxy_route_witness, eoie_help_flag_code, eoie_help_flag_witness};
use canonical_plan_ir_domain::{eoie_plan_ir_count, eoie_plan_ir_decode_code, eoie_plan_ir_descriptor, eoie_plan_ir_index_valid, eoie_plan_ir_projection_witness, eoie_plan_ir_schema, eoie_plan_ir_stage_witness};
use eoie_agile_lease::lease_authority_code;

#[must_use]
fn EoieProcessVerbCode() -> i32 {
    if !command_spec_domain::eoie_help_request_text().is_empty() { return eoie_command_code("help") as i32; }
    if lease_authority_code() != 0 { return 7; }
    let verb = std::env::args().nth(1).unwrap_or_else(|| "status".to_owned());
    let code = eoie_command_code(&verb);
    let unknown = eoie_command_count(); if code < unknown as u64 { code as i32 } else { unknown }
}

fn eoie_authority_summary_for_mask(mask: i32) -> Result<(i32, i32, i32, String), String> {
    let state = authority_state_domain::eoie_authority_state(mask, 0);
    if mask < 0 || state < 0 { return Err("typed authority state rejected".to_owned()); }
    let labels = authority_state_domain::eoie_authority_blocker_labels("v1");
    let labels = [labels.0, labels.1, labels.2, labels.3, labels.4];
    let mut reasons = Vec::new();
    for (index, label) in labels.into_iter().enumerate() {
        let bit = authority_state_domain::eoie_authority_blocker_bit(index as i32, 0);
        if bit <= 0 { return Err("typed authority blocker catalog rejected".to_owned()); }
        if mask & bit != 0 { reasons.push(label.to_string()); }
    }
    Ok((mask, state, 1, if reasons.is_empty() { "none".to_owned() } else { reasons.join(",") }))
}

fn eoie_authority_summary() -> Result<(i32, i32, i32, String), String> {
    eoie_authority_summary_for_mask(authority_state_domain::eoie_authority_declared_mask(0, 0))
}

#[must_use]
fn EoieStatusOk(migration: i32) -> i32 {
    let (mask, state, operational_minus_fallback, reasons) = match eoie_authority_summary() {
        Ok(value) => value,
        Err(error) => { eprintln!("eoie error: authority state: {error}"); return 2; }
    };
    let authority = if state == 1 { "exclusive" } else { "blocked" };
    let operational = i32::from(operational_minus_fallback == 1);
    println!(
        "eoie status ok migration={migration}/1000 public_verbs=5/5 spiral_state=true spiral_first=true authority_formal={authority} authority_blocker_mask={mask} authority_blockers={reasons} operational_replacement={operational} legacy_fallback_execution=false"
    );
    0
}

#[must_use]
fn EoieStatusLive(migration: i32) -> i32 {
    let root_text = std::env::args().nth(2).unwrap_or_default();
    let root = std::path::Path::new(&root_text);
    let evidence = match evidence_currentness_domain::evidence_status(root) {
        Ok(value) => value,
        Err(error) => { eprintln!("eoie error: live status evidence: {error}"); return 2; }
    };
    let semantic_current = match semantic_evidence_domain::verify_closeout_registry(root, migration) {
        Ok(_) => 1i32,
        Err(error) => { eprintln!("eoie error: live status semantic closeout: {error}"); return 2; }
    };
    let cold_current = match cold_proof_domain::verify_formal_authority(root) {
        Ok(_) => 30i32,
        Err(error) => { eprintln!("eoie error: live status ColdProofV4, differential catalog or family contracts: {error}"); return 2; }
    };
    let proof_mask = authority_state_domain::eoie_authority_live_mask(semantic_current + cold_current, 0); let declared_mask = authority_state_domain::eoie_authority_declared_mask(0, 0); let live_mask = if proof_mask < 0 || declared_mask < 0 { -1 } else { proof_mask | declared_mask };
    let (authority_mask, authority_state, operational_minus_fallback, authority_reasons) = match eoie_authority_summary_for_mask(live_mask) {
        Ok(value) => value,
        Err(error) => { eprintln!("eoie error: live status authority: {error}"); return 2; }
    };
    let authority_mode = if authority_state == 1 { "exclusive" } else { "blocked" };
    let operational_replacement = i32::from(operational_minus_fallback == 1);
    let toolchain_text = match evidence_currentness_domain::toolchain_status(root) {
        Ok(Some(value)) => format!("toolchain=current toolchain_declared={} toolchain_observed={} toolchain_verified={}", value.declared, value.observed, value.verified),
        Ok(None) => "toolchain=absent".to_owned(),
        Err(error) => { eprintln!("eoie error: live status Rust toolchain: {error}"); return 2; }
    };
    match eoie_compiler_discovery::resolve_spiral_compiler(None) {
        Ok(Some(compiler)) => println!("eoie status live migration={migration}/1000 spiral_state=true spiral_first=true evidence_declared={} evidence_observed={} evidence_verified={} evidence_current=true {} compiler=available compiler_source={} compiler_sha256={} compiler_entry_sha256={} compiler_facade_sha256={} authority_formal={} authority_blocker_mask={} authority_blockers={} operational_replacement={} legacy_fallback_execution=false", evidence.declared, evidence.observed, evidence.verified, toolchain_text, compiler.source, compiler.sha256, compiler.entry_sha256, compiler.facade_sha256.as_deref().unwrap_or("absent"), authority_mode, authority_mask, authority_reasons, operational_replacement),
        Ok(None) => println!("eoie status live migration={migration}/1000 spiral_state=true spiral_first=true evidence_declared={} evidence_observed={} evidence_verified={} evidence_current=true {} compiler=absent authority_formal={} authority_blocker_mask={} authority_blockers={} operational_replacement={} legacy_fallback_execution=false", evidence.declared, evidence.observed, evidence.verified, toolchain_text, authority_mode, authority_mask, authority_reasons, operational_replacement),
        Err(error) => { eprintln!("eoie error: live status compiler discovery: {error}"); return 2; }
    }
    0
}

fn eoie_public_valid_values() -> String { (0..eoie_command_count()).map(|index| eoie_command_descriptor(&index.to_string()).0.to_string()).collect::<Vec<_>>().join("|") }
fn eoie_actionable_error(reason: &str, intent: &str, usage: &str, valid: &str, corrected: &str) -> i32 { eprintln!("eoie error: {reason}\nintent: {intent}\nusage: eoie {usage}\nvalid: {valid}\ncorrected: {corrected}"); 2 }

#[must_use]
fn EoieStatusArityError() -> i32 { let descriptor = eoie_command_descriptor_named("status"); eoie_actionable_error("status accepts zero arguments or one bundle root", "status", descriptor.2.as_ref(), "zero arguments|one bundle root", "eoie status") }

#[must_use]
fn EoieStatusAuthorityError() -> i32 {
    eprintln!("eoie error: workspace-state authority rejected");
    2
}

#[must_use]
fn eoie_reenter_with_compiler(verb: &str, forwarded: &[&str], compiler: &str) -> i32 {
    let compiler = std::path::Path::new(compiler);
    if !compiler.is_absolute() || !compiler.is_file() {
        eprintln!("eoie error: Spiral compiler must be an absolute regular file: {}", compiler.display());
        return 2;
    }
    let executable = match std::env::current_exe() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("eoie error: locate current executable: {error}");
            return 2;
        }
    };
    let fallback_timeout_ms = std::env::var("EOIE_REENTER_TIMEOUT_MS").ok().and_then(|value| value.parse::<u64>().ok()).filter(|value| *value > 0).unwrap_or(300_000);
    let timeout_ms = forwarded.get(1).and_then(|root| {
        let root = std::path::Path::new(root);
        let (_, _, wrap_at_unix_ms, _, should_wrap, _) = eoie_agile_lease::agile_prompt_lease_clock(root).ok()?;
        if should_wrap { return Some(1); }
        let now_unix_ms = eoie_agile_lease::agile_now_unix_ms().ok()?;
        Some(wrap_at_unix_ms.saturating_sub(now_unix_ms).max(1))
    }).unwrap_or(fallback_timeout_ms);
    let mut command = std::process::Command::new(executable);
    command.arg(verb)
        .args(forwarded)
        .env("EOIE_SPIRAL_COMPILE", compiler)
        .env("EOIE_PATCH_COMPILE_REQUIRED", "1");
    match eoie_process_observation::run_bounded_streaming_receipted_observed(&mut command, timeout_ms, "cli-reenter-with-compiler", None) {
        Ok(observation) => observation.status.code().unwrap_or(2),
        Err(error) => {
            eprintln!("eoie error: reenter {verb}: {error}");
            2
        }
    }
}

#[must_use]
fn EoiePatchAdapter() -> i32 {
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    let subcommand = args.first().map(String::as_str); if !matches!(subcommand, Some("apply" | "check" | "rehearse" | "gated")) { let descriptor = eoie_command_descriptor_named("patch"); let reason = subcommand.map(|value| format!("unknown patch subcommand: {value}")).unwrap_or_else(|| "patch subcommand required".to_owned()); return eoie_actionable_error(&reason, "patch", descriptor.2.as_ref(), descriptor.2.as_ref(), "eoie help patch"); }
    if args.len() == 5 && args[0] == "apply" && args[3] == "--compiler" {
        return eoie_reenter_with_compiler(
            "patch",
            &[args[0].as_str(), args[1].as_str(), args[2].as_str()],
            args[4].as_str(),
        );
    }
    if args.first().map(String::as_str) == Some("rehearse") {
        return eoie_patch_rehearsal::eoie_patch_rehearse_run();
    }
    if args.first().map(String::as_str) == Some("gated") {
        return eoie_patch_rehearsal::eoie_patch_gated_run();
    }
    eoie_patch_mutation::eoie_patch_run()
}

#[must_use]
fn EoieAgileAdapter() -> i32 {
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    let subcommand = args.first().map(String::as_str); if subcommand.map(|value| eoie_agile_subcommand_known(value) == 0).unwrap_or(true) { let descriptor = eoie_command_descriptor_named("agile"); let reason = subcommand.map(|value| format!("unknown agile subcommand: {value}")).unwrap_or_else(|| "agile subcommand required".to_owned()); return eoie_actionable_error(&reason, "agile", descriptor.2.as_ref(), descriptor.2.as_ref(), "eoie help agile"); }
    if args.len() == 4 && args[0] == "check" && args[2] == "--compiler" {
        return eoie_agile_state::agile_check_with_compiler_owned(args[1].as_str(), args[3].as_str());
    }
    eoie_agile_mutation::eoie_agile_run()
}

#[must_use]
fn EoieBundleAdapter() -> i32 {
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    let subcommand = args.first().map(String::as_str); if !matches!(subcommand, Some("growth-baseline" | "growth-receipt" | "check" | "create" | "create-flat" | "verify" | "verify-flat" | "rehydrate" | "retention-plan" | "retention-apply")) { let descriptor = eoie_command_descriptor_named("bundle"); let reason = subcommand.map(|value| format!("unknown bundle subcommand: {value}")).unwrap_or_else(|| "bundle subcommand required".to_owned()); return eoie_actionable_error(&reason, "bundle", descriptor.2.as_ref(), descriptor.2.as_ref(), "eoie help bundle"); }
    match subcommand {
        Some("retention-plan") => eoie_proxy_result(eoie_bundle_retention::retention_plan_command(&args)),
        Some("retention-apply") => eoie_proxy_result(eoie_bundle_retention::retention_apply_command(&args)),
        _ => eoie_legacy_operations::eoie_bundle_run(),
    }
}

fn eoie_proxy_result(result: Result<(), String>) -> i32 {
    if let Err(error) = result {
        eprintln!("eoie error: {error}");
        2
    } else {
        0
    }
}

fn eoie_plan_ir_inspect(args: &[String]) -> Result<(), String> {
    if eoie_plan_ir_stage_witness() != 1 { return Err("typed Plan IR stage witness rejected".to_owned()); }
    if eoie_plan_ir_count() <= 0 { return Err("typed Plan IR provider is empty".to_owned()); }
    if args.len() != 2 { return Err("plan-ir-inspect expects one index".to_owned()); }
    let index = &args[1];
    if eoie_plan_ir_index_valid(index) != 1 { return Err(format!("invalid Plan IR index: {index}")); }
    let numeric_index = index.parse::<u64>().map_err(|_| "Plan IR index must be integer".to_owned())?;
    let descriptor = eoie_plan_ir_descriptor(index);
    if eoie_plan_ir_decode_code(descriptor.0.as_ref(), descriptor.1.as_ref(), descriptor.2.as_ref(), descriptor.3.as_ref(), descriptor.4.as_ref()) != numeric_index { return Err(format!("canonical Plan IR descriptor rejected: {index}")); }
    println!("schema={} index={} op={} target={} expected={} payload={} effect={}", eoie_plan_ir_schema("schema"), index, descriptor.0, descriptor.1, descriptor.2, descriptor.3, descriptor.4);
    Ok(())
}

fn eoie_plan_ir_check(args: &[String]) -> Result<(), String> {
    if args.len() != 1 { return Err("plan-ir-check accepts no extra arguments".to_owned()); }
    if eoie_plan_ir_stage_witness() != 1 { return Err("typed Plan IR stage witness rejected".to_owned()); }
    if eoie_plan_ir_projection_witness() != 1 { return Err("typed Plan IR manifest projection witness rejected".to_owned()); }
    let count = eoie_plan_ir_count();
    if count <= 0 { return Err("typed Plan IR provider is empty".to_owned()); }
    for index in 0..count {
        let key = index.to_string();
        let descriptor = eoie_plan_ir_descriptor(&key);
        if eoie_plan_ir_decode_code(descriptor.0.as_ref(), descriptor.1.as_ref(), descriptor.2.as_ref(), descriptor.3.as_ref(), descriptor.4.as_ref()) != index as u64 { return Err(format!("canonical Plan IR descriptor rejected: {key}")); }
    }
    println!("eoie plan-ir check ok schema={} count={} stage_witness=1 projection_witness=1", eoie_plan_ir_schema("schema"), count);
    Ok(())
}

#[must_use]
fn EoieProxyAdapter() -> i32 {
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    if args.is_empty() { let descriptor = eoie_command_descriptor_named("proxy"); return eoie_actionable_error("proxy capability required", "proxy capability", descriptor.2.as_ref(), descriptor.2.as_ref(), "eoie help proxy"); }
    if eoie_proxy_route_witness() != 1 { return eoie_actionable_error("typed proxy route witness rejected", "proxy capability", "proxy <capability>", "typed proxy route catalog", "eoie help proxy"); }
    let route = eoie_proxy_route_code(args[0].as_str());
    match route {
        0 => eoie_proxy_result(eoie_plan_ir_inspect(&args)),
        1 => eoie_proxy_result(eoie_plan_ir_check(&args)),
        2 => eoie_proxy_result(eoie_release_hygiene::prune_compiler_sidecars(&args)),
        3 => eoie_proxy_result(eoie_release_hygiene::incident_recovery_run(&args)),
        4 => eoie_proxy_result(bundle_retention_domain::external_payload_run(&args)),
        5 => eoie_proxy_result(eoie_coverage_prune::coverage_export(&args)),
        6 => eoie_proxy_result(eoie_coverage_prune::coverage_union(&args)),
        7 => eoie_proxy_result(legacy_capability_domain::legacy_surface_run(&args)),
        8 => eoie_proxy_result(legacy_capability_domain::legacy_install_self_run(&args)),
        9 => eoie_proxy_result(legacy_capability_domain::legacy_self_upgrade_check_run(&args)),
        10 => eoie_proxy_result(eoie_legacy_operations::legacy_restart_baton_run(&args)),
        11 => eoie_proxy_result(eoie_patch_driver_domain::source_authoring_run(&args)),
        12 => eoie_proxy_result(eoie_batch_write::filesystem_batch_write_run(&args)),
        13 => eoie_proxy_result(eoie_fs_actions::filesystem_actions_run(&args)),
        14 => eoie_proxy_result(eoie_handoff::binary_install_run(&args)),
        15 => eoie_proxy_result(eoie_handoff::release_closeout_run(&args)),
        16 => eoie_proxy_result(eoie_legacy_operations::dogfood_shim_proxy(&args)),
        17 => eoie_proxy_result(legacy_capability_domain::legacy_chmod_run(&args)),
        18 => eoie_proxy_result(legacy_capability_domain::legacy_symlink_run(&args)),
        19 => eoie_proxy_result(legacy_capability_domain::legacy_copy_tree_run(&args)),
        20 => eoie_proxy_result(legacy_capability_domain::legacy_command_capture_run(&args)),
        21 => eoie_proxy_result(process_batch_plan_domain::command_capture_env_run(&args)),
        22 => eoie_proxy_result(process_batch_plan_domain::command_capture_matrix_run(&args)),
        23 => eoie_proxy_result(process_batch_plan_domain::command_capture_matrix_gate_run(&args)),
        24 => eoie_proxy_result(process_batch_plan_domain::batch_plan_run(&args)),
        25 => eoie_proxy_result(process_batch_plan_domain::batch_plan_resume_run(&args)),
        26 => eoie_proxy_result(eoie_toolchain_proxy::portable_toolchain_run(&args)),
        27 => eoie_proxy_result(eoie_product_diff::product_diff_run(&args)),
        28 => eoie_proxy_result(eoie_product_diff::source_recovery_run(&args)),
        29 => eoie_proxy_result(eoie_proxy_search::proxy_search(&args)),
        30 => eoie_proxy_result(eoie_bundle_manifest::archive_extract_generic_run(&args)),
        31 => eoie_proxy_result(archive_deflate_domain::archive_extract_run(&args)),
        _ => eoie_legacy_operations::eoie_proxy_run(),
    }
}

#[must_use]
fn EoieHelpRejected() -> i32 {
    let request = command_spec_domain::eoie_help_request_text();
    if !request.is_empty() { println!("{request}"); return 0; }
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    let count = eoie_command_count();
    if count <= 0 { eprintln!("eoie error: typed command catalog is empty"); return 2; }
    if args.len() == 2 && eoie_command_code(&args[0]) == 4 {
        let usage = eoie_command_proxy_usage(&args[1]);
        if usage.is_empty() { let valid = eoie_command_descriptor_named("proxy").2; return eoie_actionable_error(&format!("unknown proxy help target: {}", args[1]), "proxy help", "help proxy <capability>", valid.as_ref(), "eoie help"); }
        println!("usage: eoie {}", usage);
        return 0;
    }
    if args.len() > 1 { let descriptor = eoie_command_descriptor_named("help"); let valid = format!("{}|proxy <capability>|--completion|--json|--schema", eoie_public_valid_values()); return eoie_actionable_error("help accepts zero arguments, one verb, proxy <capability>, --completion, --json, or --schema", "help", descriptor.2.as_ref(), &valid, "eoie help"); }
    if let Some(target) = args.first() {
        if eoie_help_flag_witness() != 1 { eprintln!("eoie error: typed help flag witness rejected"); return 2; }
        let help_flag = eoie_help_flag_code(target);
        if help_flag == 0 {
            for index in 0..count { let descriptor = eoie_command_descriptor(&index.to_string()); println!("{}", descriptor.0); }
            return 0;
        }
        if help_flag == 1 {
            if eoie_command_schema_witness() != 1 { eprintln!("eoie error: typed command schema witness rejected"); return 2; }
            print!("[");
            for index in 0..count { if index > 0 { print!(","); } print!("{}", eoie_command_descriptor_json(&index.to_string())); }
            println!("]");
            return 0;
        }
        if help_flag == 2 {
            if eoie_command_schema_witness() != 1 { eprintln!("eoie error: typed command schema witness rejected"); return 2; }
            println!("command-spec");
            return 0;
        }
        let code = eoie_command_code(target);
        if code >= count as u64 { let proxy_usage = eoie_command_proxy_usage(target); if !proxy_usage.is_empty() { let corrected = format!("eoie help proxy {target}"); return eoie_actionable_error(&format!("help target is a proxy capability: {target}"), "proxy help", "help proxy <capability>", proxy_usage.as_ref(), &corrected); } let valid = eoie_public_valid_values(); return eoie_actionable_error(&format!("unknown help target: {target}"), "help", "help [verb|--completion|--json|--schema]", &valid, "eoie help"); }
        let descriptor = eoie_command_descriptor(&code.to_string());
        println!("name={} effect={} handler={}", descriptor.0, descriptor.1, descriptor.4);
        println!("summary={}", descriptor.3);
        println!("usage: eoie {}", descriptor.2);
        return 0;
    }
    println!("usage: eoie <status|patch|agile|bundle|proxy|help> ...");
    for index in 0..count { let descriptor = eoie_command_descriptor(&index.to_string()); println!("{}", descriptor.2); }
    println!("{}", eoie_command_more_usages());
    println!("deployment: runtime operations are compilerless; source-author and prune-uncovered are build-authoring operations requiring EOIE_SPIRAL_COMPILE");
    0
}

#[must_use]
fn EoieUnknownRejected() -> i32 { let verb = std::env::args().nth(1).unwrap_or_default(); let proxy_usage = eoie_command_proxy_usage(&verb); if !proxy_usage.is_empty() { let corrected = format!("eoie {proxy_usage}"); return eoie_actionable_error(&format!("proxy capability used as public verb: {verb}"), "proxy capability", proxy_usage.as_ref(), "proxy <capability>", &corrected); } let valid = eoie_public_valid_values(); eoie_actionable_error(&format!("unknown verb: {verb}"), "public command", "<status|patch|agile|bundle|proxy|help> ...", &valid, "eoie help") }

#[derive(Clone)]
enum US0 {
    US0_DirectArguments,
    US0_FileArguments(Rc<str>),
    US0_StdinArguments,
}
#[derive(Clone)]
enum US2 {
    US2_VerbJob,
    US2_VerbRepo,
    US2_VerbTable,
    US2_VerbRead,
    US2_VerbLines,
    US2_VerbGrep,
    US2_VerbDo,
    US2_VerbPs,
    US2_VerbKill,
    US2_VerbStandards,
    US2_VerbWindow,
    US2_VerbExperiment,
    US2_VerbGuide,
    US2_VerbMachine,
    US2_VerbAdoption,
    US2_VerbMemory,
    US2_VerbResume,
}
#[derive(Clone)]
enum US1 {
    US1_DirectKnown(US2),
    US1_DirectUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_StatusHandler,
    US3_PatchHandler,
    US3_AgileHandler,
    US3_BundleHandler,
    US3_ProxyHandler,
    US3_HelpHandler,
    US3_UnknownHandler,
    US3_LeaseDeniedHandler,
}
#[derive(Clone)]
enum US4 {
    US4_StatusSpec,
    US4_PatchSpec,
    US4_AgileSpec,
    US4_BundleSpec,
    US4_ProxySpec,
    US4_HelpSpec,
    US4_UnknownSpec,
}
fn argument_source_0() -> US0 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--args-from"); } LIT.with(|lit| lit.clone()) };
    if v2 {
        let mut v3: i32 = 2i32;
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v3 as usize).unwrap_or_default());
        let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        if v5 {
            US0::US0_StdinArguments
        } else {
            US0::US0_FileArguments(v4.clone())
        }
    } else {
        US0::US0_DirectArguments
    }
}
fn command_more_usages_2() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status [root]\npatch: apply <root> <plan.spi> [--compiler <absolute>] | check <root> <plan.spi> | rehearse <root> <plan.spi> | gated <root> <plan.spi> <gate-program>\nagile: init <root> | begin <root> <title> | lease <root> | status <root> | handoff <root> | list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json] | check <root> [--compiler <absolute>] | record <root> | set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done> | add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>] | edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value> | show <root> <id> [--json] | plan <root> [--json] | migrate-tags preview|apply <root> | note <root> <id> <text> [--as <author>] | note-edit <root> <id> <note-number> <text> [--as <author>] | notes from-title <root> <id> | rename <root> <old-id> <new-id> | todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record | next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--open] [--ready] [--blocked] [count] | roots <root> [add|drop <path>...] | links <root> --memory <dir> [--json] | views <root> | view <root> <name> [extra flags...] | view-add <root> <name> <list|next> [flags...]\nbundle: growth-baseline <predecessor.zip> <previous-root> <root> | growth-receipt <root> <reason> <reference> | check <root> [auto|generic|eoie] | create <root> <output.zip> [auto|generic|eoie] | create-flat <root> <output.zip> <max-bytes> | verify <archive.zip> [auto|generic|eoie] | verify-flat <archive.zip> <max-bytes> | rehydrate <archive.zip> <destination> [auto|generic|eoie] | retention-plan <directory> <current.zip> <receipt.spi> | retention-apply <directory> <current.zip> <receipt.spi> <fingerprint>\nproxy: plan-ir-inspect|plan-ir-check|legacy-surface|install-self|self-upgrade-check|restart-baton|fs-chmod|fs-symlink|fs-copy-tree|command-output|command-capture|command-capture-env|command-capture-matrix|command-capture-matrix-gate|batch-plan|batch-plan-resume|fs-list|fs-read|fs-slice|fs-context|fs-search|parallel-search|parallel-manifest|product-diff|fs-write|text-replace|fs-copy|fs-remove|fs-remove-tree|run|spiral-session-shutdown|archive-extract|zip-extract|hash|hash-tree|prune-build-cache|prune-compiler-sidecars|incident-recovery|external-payload|differential-compare|coverage-assess|coverage-export|coverage-union|coverage-run|toolchain|portable-toolchain|source-stats|source-topology|ingest-map|ingest-trim-check|ingest-extracted-manifest|ingest-targets|ingest-targets-derive|frontier-map|spi-map|spi-diff|source-recovery-diff|source-author|fs-batch-write|fs-actions|binary-install|release-closeout|shim|prune-uncovered|cold-rebuild-matrix|cold-rebuild-owner|source-map|source-diff|coverage-register|formal-receipts-renew\nhelp [verb|--completion|--json|--schema] | proxy <capability>\nrepo: state <root> [<root>...] | affected <src> <package>... (Cargo crates whose package.spiproj graph reaches a changed package)\nread <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]\ntable: query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone [--out file.tsv] [--quiet] | test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] [--prediction x] [--confidence 0.95] [--where ...]...\nstandards: <package-dir>... [--json] (rules from the package's standards.spi rows, else no-comment-lines, line-width 120, lf-only, test-targets; exit 1 on findings) | --census <dir>... | --baseline <file.spi> <dir>... | --ratchet <file.spi> <dir>... (typefeatureset and drift counts per package; the ratchet fails when a package gets worse than its baseline or a new package misses the floor)\nwindow: request <class> <minutes> [--label <text>] [--wait <minutes>] | release <id> | list | check <class> | run <class> <minutes> [--label <text>] [--wait <minutes>] -- <command> [args...] (machine windows shared by every session; live-gui and compiler-install exclude all other classes; exit 1 when blocked)\nexperiment: register <ledger> <ID> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <col> --runs <n> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | analyze <ledger> <ID> <table.tsv> [--where col=value]... [--interim <reason>] | amend <ledger> <ID> --field <name> --value <v> --reason <text> | addendum <ledger> <ID> <text> | show <ledger> <ID> (pre-registered; one analysis at the planned count)\ndo <root> '<program>' | do <root> - (stdin); edit <path|glob> [dry-run] [expect files n replacements n] / replace [pattern] once|all|<n> times \"old\" with \"new\" / patch <<<hunks>>> / show <path>; search [literal] \"re\" in <glob> [include a,b] [exclude glob] [context n[,m]] [first n per file] [paths] / read <path> around \"text\" [context n[,m]] / files <glob> / head <glob> n / tail <glob> n / wait <file> ~ \"re\" [timeout 5s]; hash <a,b,...> / env <A,B> / exists <a,b> / nearest \"re\" before lines n,m in <file> / json <file> get a.b,c.length / lines <file> count by \"(re)\" / files <glob> count by extension / table <file.tsv> [where col = \"v\"|col <> \"v\"] count by a,b / dirs <dir/*> having a and b + tag <name> = <file> ~ \"re\" [and|or|not ...] + count by <name> / diff <a.tsv> <b.tsv> by k1,k2 [on c1,c2]; run <cmd> [timeout 10s] [until output ~ \"re\" then stop] [background as name] / tail output n / search \"re\" in output / errors in output by \"(re)\" / processes where command ~ \"re\" / stop <name>; append <file> \"line {clock:HH:mm}\" / copy <glob> to <dir> [except *.tmp,...] / move <path> to <path> (no overwrite; absolute paths reach other roots), then read statements; git init|commit <repo> all \"msg\" [at <iso>] / git history <repo> \"text\" in <file> / git status a, b group by state / git log <repo> [since <date>] count by day / search \"re\" in <glob> with git status; job start <cmd> as <name> timeout 30s [notify] / job wait <name> [timeout 5m] / job stop <name>; lab audit <dir> against <backlog>\nps [regex] [--tree] [--mine] | kill <pid>... [--tree] (never this eoie or its calling shells)\nguide <spec/one-call.md> [--out <file.md>]\nmemory import <memory-dir> <data.spi> | render <data.spi> <out-dir> | check <data.spi> | list <data.spi> [--kind user|feedback|project|reference] | recall <data.spi> <words> [--limit n]\nresume <backlog-root> [--team <team>] [--as <author>]\nmachine watch [--sample-seconds n]\nadoption [--days n] [--out <file.tsv>]\njob: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(6).collect::<Vec<_>>().join("
"));
    v1.clone()
}
fn direct_verb_usage_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = command_more_usages_2();
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v1).lines().find(|line| line.split(char::from(124u8)).any(|part| part.trim().split(|c: char| c == char::from(32u8) || c == char::from(58u8)).next() == Some(&*v0))).unwrap_or("").trim());
    v2.clone()
}
fn direct_help_3(mut v0: Rc<str>) -> i32 {
    println!("usage: eoie {}", &*v0);
    0i32
}
fn lowercase_named_5(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (v1).to_ascii_lowercase() == &*v0;
    v2
}
fn direct_of_4(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Job"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = lowercase_named_5(v0.clone(), v1.clone());
    if v2 {
        let mut v3: US2 = US2::US2_VerbJob;
        US1::US1_DirectKnown(v3.clone())
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Repo"); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = lowercase_named_5(v0.clone(), v5.clone());
        if v6 {
            let mut v7: US2 = US2::US2_VerbRepo;
            US1::US1_DirectKnown(v7.clone())
        } else {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Table"); } LIT.with(|lit| lit.clone()) };
            let mut v10: bool = lowercase_named_5(v0.clone(), v9.clone());
            if v10 {
                let mut v11: US2 = US2::US2_VerbTable;
                US1::US1_DirectKnown(v11.clone())
            } else {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Read"); } LIT.with(|lit| lit.clone()) };
                let mut v14: bool = lowercase_named_5(v0.clone(), v13.clone());
                if v14 {
                    let mut v15: US2 = US2::US2_VerbRead;
                    US1::US1_DirectKnown(v15.clone())
                } else {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Lines"); } LIT.with(|lit| lit.clone()) };
                    let mut v18: bool = lowercase_named_5(v0.clone(), v17.clone());
                    if v18 {
                        let mut v19: US2 = US2::US2_VerbLines;
                        US1::US1_DirectKnown(v19.clone())
                    } else {
                        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Grep"); } LIT.with(|lit| lit.clone()) };
                        let mut v22: bool = lowercase_named_5(v0.clone(), v21.clone());
                        if v22 {
                            let mut v23: US2 = US2::US2_VerbGrep;
                            US1::US1_DirectKnown(v23.clone())
                        } else {
                            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Do"); } LIT.with(|lit| lit.clone()) };
                            let mut v26: bool = lowercase_named_5(v0.clone(), v25.clone());
                            if v26 {
                                let mut v27: US2 = US2::US2_VerbDo;
                                US1::US1_DirectKnown(v27.clone())
                            } else {
                                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Ps"); } LIT.with(|lit| lit.clone()) };
                                let mut v30: bool = lowercase_named_5(v0.clone(), v29.clone());
                                if v30 {
                                    let mut v31: US2 = US2::US2_VerbPs;
                                    US1::US1_DirectKnown(v31.clone())
                                } else {
                                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Kill"); } LIT.with(|lit| lit.clone()) };
                                    let mut v34: bool = lowercase_named_5(v0.clone(), v33.clone());
                                    if v34 {
                                        let mut v35: US2 = US2::US2_VerbKill;
                                        US1::US1_DirectKnown(v35.clone())
                                    } else {
                                        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Standards"); } LIT.with(|lit| lit.clone()) };
                                        let mut v38: bool = lowercase_named_5(v0.clone(), v37.clone());
                                        if v38 {
                                            let mut v39: US2 = US2::US2_VerbStandards;
                                            US1::US1_DirectKnown(v39.clone())
                                        } else {
                                            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Window"); } LIT.with(|lit| lit.clone()) };
                                            let mut v42: bool = lowercase_named_5(v0.clone(), v41.clone());
                                            if v42 {
                                                let mut v43: US2 = US2::US2_VerbWindow;
                                                US1::US1_DirectKnown(v43.clone())
                                            } else {
                                                let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Experiment"); } LIT.with(|lit| lit.clone()) };
                                                let mut v46: bool = lowercase_named_5(v0.clone(), v45.clone());
                                                if v46 {
                                                    let mut v47: US2 = US2::US2_VerbExperiment;
                                                    US1::US1_DirectKnown(v47.clone())
                                                } else {
                                                    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Guide"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v50: bool = lowercase_named_5(v0.clone(), v49.clone());
                                                    if v50 {
                                                        let mut v51: US2 = US2::US2_VerbGuide;
                                                        US1::US1_DirectKnown(v51.clone())
                                                    } else {
                                                        let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Machine"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v54: bool = lowercase_named_5(v0.clone(), v53.clone());
                                                        if v54 {
                                                            let mut v55: US2 = US2::US2_VerbMachine;
                                                            US1::US1_DirectKnown(v55.clone())
                                                        } else {
                                                            let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Adoption"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v58: bool = lowercase_named_5(v0.clone(), v57.clone());
                                                            if v58 {
                                                                let mut v59: US2 = US2::US2_VerbAdoption;
                                                                US1::US1_DirectKnown(v59.clone())
                                                            } else {
                                                                let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Memory"); } LIT.with(|lit| lit.clone()) };
                                                                let mut v62: bool = lowercase_named_5(v0.clone(), v61.clone());
                                                                if v62 {
                                                                    let mut v63: US2 = US2::US2_VerbMemory;
                                                                    US1::US1_DirectKnown(v63.clone())
                                                                } else {
                                                                    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Resume"); } LIT.with(|lit| lit.clone()) };
                                                                    let mut v66: bool = lowercase_named_5(v0.clone(), v65.clone());
                                                                    if v66 {
                                                                        let mut v67: US2 = US2::US2_VerbResume;
                                                                        US1::US1_DirectKnown(v67.clone())
                                                                    } else {
                                                                        US1::US1_DirectUnknown
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
                            }
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 1i32;
        let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("help"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = if v2 {
            let mut v3: i32 = 2i32;
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v3 as usize).unwrap_or_default());
            direct_verb_usage_1(v4.clone())
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        };
        let mut v8: bool = v7.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v8 {
            direct_help_3(v7.clone())
        } else {
            let mut v10: US1 = direct_of_4(v1.clone());
            match &v10 {
                US1::US1_DirectKnown(v11) => {
                    let mut v11: US2 = v11.clone();
                    match &v11 {
                        US2::US2_VerbAdoption => {
                            let mut v41: i32 = eoie_adoption::eoie_adoption_run();
                            v41
                        }
                        US2::US2_VerbDo => {
                            let mut v33: i32 = eoie_do::eoie_do_run();
                            v33
                        }
                        US2::US2_VerbExperiment => {
                            let mut v38: i32 = eoie_experiments::eoie_experiment_run();
                            v38
                        }
                        US2::US2_VerbGrep => {
                            let mut v32: i32 = eoie_grep_query::eoie_grep_run();
                            v32
                        }
                        US2::US2_VerbGuide => {
                            let mut v39: i32 = eoie_guide::eoie_guide_run();
                            v39
                        }
                        US2::US2_VerbJob => {
                            let mut v12: i32 = eoie_jobs::eoie_job_run();
                            v12
                        }
                        US2::US2_VerbKill => {
                            let mut v35: i32 = eoie_process_table::eoie_process_run();
                            v35
                        }
                        US2::US2_VerbLines => {
                            let mut v31: i32 = eoie_read_query::eoie_read_run();
                            v31
                        }
                        US2::US2_VerbMachine => {
                            let mut v40: i32 = eoie_machine_watch::eoie_machine_watch_run();
                            v40
                        }
                        US2::US2_VerbMemory => {
                            let mut v42: i32 = eoie_memory::eoie_memory_run();
                            v42
                        }
                        US2::US2_VerbPs => {
                            let mut v34: i32 = eoie_process_table::eoie_process_run();
                            v34
                        }
                        US2::US2_VerbRead => {
                            let mut v30: i32 = eoie_read_query::eoie_read_run();
                            v30
                        }
                        US2::US2_VerbRepo => {
                            let mut v13: i32 = 2i32;
                            let mut v14: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v13 as usize).unwrap_or_default());
                            let mut v15: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("affected"); } LIT.with(|lit| lit.clone()) };
                            if v15 {
                                let mut v16: i32 = eoie_repo_affected::eoie_repo_affected_run();
                                v16
                            } else {
                                let mut v17: i32 = eoie_repo_state::eoie_repo_run();
                                v17
                            }
                        }
                        US2::US2_VerbResume => {
                            let mut v43: i32 = eoie_resume::eoie_resume_run();
                            v43
                        }
                        US2::US2_VerbStandards => {
                            let mut v36: i32 = eoie_standards::eoie_standards_run();
                            v36
                        }
                        US2::US2_VerbTable => {
                            let mut v19: i32 = 2i32;
                            let mut v20: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v19 as usize).unwrap_or_default());
                            let mut v21: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("logs"); } LIT.with(|lit| lit.clone()) };
                            if v21 {
                                let mut v22: i32 = eoie_table_logs::eoie_table_logs_run();
                                v22
                            } else {
                                let mut v23: i32 = 2i32;
                                let mut v24: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v23 as usize).unwrap_or_default());
                                let mut v25: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
                                if v25 {
                                    let mut v26: i32 = eoie_table_test::eoie_table_test_run();
                                    v26
                                } else {
                                    let mut v27: i32 = eoie_table_query::eoie_table_run();
                                    v27
                                }
                            }
                        }
                        US2::US2_VerbWindow => {
                            let mut v37: i32 = eoie_machine_windows::eoie_window_run();
                            v37
                        }
                    }
                }
                US1::US1_DirectUnknown => {
                    let mut v61: i32 = eoie_legacy_operations::dogfood_shim_argv0_code();
                    let mut v62: bool = 0i32 == v61;
                    if v62 {
                        let mut v63: i32 = EoieProcessVerbCode();
                        let mut v64: bool = v63 == 7i32;
                        let mut v103: US3 = if v64 {
                            US3::US3_LeaseDeniedHandler
                        } else {
                            let mut v66: i32 = v63.wrapping_add(1i32);
                            let mut v67: bool = v66 == 1i32;
                            let mut v88: US4 = if v67 {
                                US4::US4_StatusSpec
                            } else {
                                let mut v69: bool = v66 == 2i32;
                                if v69 {
                                    US4::US4_PatchSpec
                                } else {
                                    let mut v71: bool = v66 == 3i32;
                                    if v71 {
                                        US4::US4_AgileSpec
                                    } else {
                                        let mut v73: bool = v66 == 4i32;
                                        if v73 {
                                            US4::US4_BundleSpec
                                        } else {
                                            let mut v75: bool = v66 == 5i32;
                                            if v75 {
                                                US4::US4_ProxySpec
                                            } else {
                                                let mut v77: bool = v66 == 6i32;
                                                if v77 {
                                                    US4::US4_HelpSpec
                                                } else {
                                                    US4::US4_UnknownSpec
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            match &v88 {
                                US4::US4_AgileSpec => {
                                    US3::US3_AgileHandler
                                }
                                US4::US4_BundleSpec => {
                                    US3::US3_BundleHandler
                                }
                                US4::US4_HelpSpec => {
                                    US3::US3_HelpHandler
                                }
                                US4::US4_PatchSpec => {
                                    US3::US3_PatchHandler
                                }
                                US4::US4_ProxySpec => {
                                    US3::US3_ProxyHandler
                                }
                                US4::US4_StatusSpec => {
                                    US3::US3_StatusHandler
                                }
                                US4::US4_UnknownSpec => {
                                    US3::US3_UnknownHandler
                                }
                            }
                        };
                        match &v103 {
                            US3::US3_AgileHandler => {
                                let mut v135: i32 = EoieAgileAdapter();
                                v135
                            }
                            US3::US3_BundleHandler => {
                                let mut v136: i32 = EoieBundleAdapter();
                                v136
                            }
                            US3::US3_HelpHandler => {
                                let mut v138: i32 = EoieHelpRejected();
                                v138
                            }
                            US3::US3_LeaseDeniedHandler => {
                                2i32
                            }
                            US3::US3_PatchHandler => {
                                let mut v134: i32 = EoiePatchAdapter();
                                v134
                            }
                            US3::US3_ProxyHandler => {
                                let mut v137: i32 = EoieProxyAdapter();
                                v137
                            }
                            US3::US3_StatusHandler => {
                                let mut v104: i32 = std::env::args().skip(2).count() as i32;
                                let mut v105: bool = 0i32 == v104;
                                if v105 {
                                    let mut v106: i32 = bootstrap_authority_status::eoie_bootstrap_workspace_state_status();
                                    let mut v107: bool = 1i32 == v106;
                                    if v107 {
                                        let mut v108: i32 = bootstrap_authority_status::eoie_bootstrap_compiler_session_status();
                                        let mut v109: bool = 1i32 == v108;
                                        if v109 {
                                            let mut v110: i32 = bootstrap_authority_status::eoie_bootstrap_legacy_bundle_writer_status();
                                            let mut v111: bool = 1i32 == v110;
                                            if v111 {
                                                let mut v112: i32 = bootstrap_authority_status::eoie_bootstrap_ingest_frontier_status();
                                                let mut v113: bool = 1i32 == v112;
                                                if v113 {
                                                    let mut v114: i32 = process_batch_plan_domain::eoie_batch_matrix_auto_chain_budget(60000, 16000);
                                                    let mut v115: bool = 1i32 == v114;
                                                    if v115 {
                                                        let mut v116: i32 = bootstrap_authority_status::eoie_bootstrap_settled_migration_status();
                                                        let mut v117: i32 = EoieStatusOk(v116);
                                                        v117
                                                    } else {
                                                        let mut v118: i32 = EoieStatusAuthorityError();
                                                        v118
                                                    }
                                                } else {
                                                    let mut v120: i32 = EoieStatusAuthorityError();
                                                    v120
                                                }
                                            } else {
                                                let mut v122: i32 = EoieStatusAuthorityError();
                                                v122
                                            }
                                        } else {
                                            let mut v124: i32 = EoieStatusAuthorityError();
                                            v124
                                        }
                                    } else {
                                        let mut v126: i32 = EoieStatusAuthorityError();
                                        v126
                                    }
                                } else {
                                    let mut v128: bool = 1i32 == v104;
                                    if v128 {
                                        let mut v129: i32 = bootstrap_authority_status::eoie_bootstrap_settled_migration_status();
                                        let mut v130: i32 = EoieStatusLive(v129);
                                        v130
                                    } else {
                                        let mut v131: i32 = EoieStatusArityError();
                                        v131
                                    }
                                }
                            }
                            US3::US3_UnknownHandler => {
                                let mut v139: i32 = EoieUnknownRejected();
                                v139
                            }
                        }
                    } else {
                        let mut v148: i32 = eoie_legacy_operations::dogfood_shim_dispatch();
                        v148
                    }
                }
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn run_from_6(mut v0: US0, mut v1: Rc<dyn Fn() -> i32>) -> i32 {
    #[cfg(windows)] { unsafe extern "system" { fn SetHandleInformation(handle: *mut std::ffi::c_void, mask: u32, flags: u32) -> i32; } use std::os::windows::io::AsRawHandle; unsafe { SetHandleInformation(std::io::stdin().as_raw_handle(), 1, 0); SetHandleInformation(std::io::stdout().as_raw_handle(), 1, 0); SetHandleInformation(std::io::stderr().as_raw_handle(), 1, 0); } };
    match &v0 {
        US0::US0_DirectArguments => {
            v1()
        }
        US0::US0_FileArguments(v3) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Result<std::rc::Rc<str>, std::rc::Rc<str>> = std::fs::read_to_string(&*v3).map(std::rc::Rc::<str>::from).map_err(|error| std::rc::Rc::<str>::from(format!("--args-from {}: {error}", &*v3)));
            let mut v5: bool = v4.is_ok();
            if v5 {
                let mut v6: Rc<str> = v4.clone().unwrap_or_else(|error| error);
                let mut v7: i32 = { let arguments = v6.lines().map(|line| line.trim_end_matches(char::from(13u8)).to_owned()).chain(std::env::args().skip(3)).collect::<Vec<_>>(); match std::env::current_exe().and_then(|exe| std::process::Command::new(exe).args(&arguments).status()) { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("eoie error: --args-from relaunch: {error}"); 2 } } };
                v7
            } else {
                let mut v8: Rc<str> = v4.clone().unwrap_or_else(|error| error);
                let mut v9: i32 = { eprintln!("eoie error: {}", &*v8); 2 };
                v9
            }
        }
        US0::US0_StdinArguments => {
            let mut v11: Result<std::rc::Rc<str>, std::rc::Rc<str>> = { let mut text = String::new(); std::io::Read::read_to_string(&mut std::io::stdin(), &mut text).map(|_| std::rc::Rc::<str>::from(text)).map_err(|error| std::rc::Rc::<str>::from(format!("--args-from -: {error}"))) };
            let mut v12: bool = v11.is_ok();
            if v12 {
                let mut v13: Rc<str> = v11.clone().unwrap_or_else(|error| error);
                let mut v14: i32 = { let arguments = v13.lines().map(|line| line.trim_end_matches(char::from(13u8)).to_owned()).chain(std::env::args().skip(3)).collect::<Vec<_>>(); match std::env::current_exe().and_then(|exe| std::process::Command::new(exe).args(&arguments).status()) { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("eoie error: --args-from relaunch: {error}"); 2 } } };
                v14
            } else {
                let mut v15: Rc<str> = v11.clone().unwrap_or_else(|error| error);
                let mut v16: i32 = { eprintln!("eoie error: {}", &*v15); 2 };
                v16
            }
        }
    }
}
fn spiral_main() -> i32 {
    let mut v0: US0 = argument_source_0();
    let mut v1: Rc<dyn Fn() -> i32> = closure0();
    let mut v2: i32 = run_from_6(v0.clone(), v1.clone());
    let mut v3: i32 = eoie_coverage_prune::coverage_finish(v2);
    v3
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
