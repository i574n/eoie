#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use command_spec_domain::{eoie_agile_subcommand_known, eoie_command_code, eoie_command_count, eoie_command_descriptor, eoie_command_descriptor_json, eoie_command_schema_witness, eoie_command_proxy_usage, eoie_proxy_route_code, eoie_proxy_route_witness, eoie_help_flag_code, eoie_help_flag_witness};
use canonical_plan_ir_domain::{eoie_plan_ir_count, eoie_plan_ir_decode_code, eoie_plan_ir_descriptor, eoie_plan_ir_index_valid, eoie_plan_ir_projection_witness, eoie_plan_ir_schema, eoie_plan_ir_stage_witness};
use eoie_agile_lease::lease_authority_code;

#[must_use]
fn EoieProcessVerbCode() -> i32 {
    if !command_spec_domain::eoie_help_request_text().is_empty() { return 5; }
    if lease_authority_code() != 0 { return 7; }
    let verb = std::env::args().nth(1).unwrap_or_else(|| "status".to_owned());
    let code = eoie_command_code(&verb);
    if code <= 6 { code as i32 } else { 6 }
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
fn EoieStatusArityError() -> i32 { let descriptor = eoie_command_descriptor("0"); eoie_actionable_error("status accepts zero arguments or one bundle root", "status", descriptor.2.as_ref(), "zero arguments|one bundle root", "eoie status") }

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
    let subcommand = args.first().map(String::as_str); if !matches!(subcommand, Some("apply" | "check" | "rehearse" | "gated")) { let descriptor = eoie_command_descriptor("1"); let reason = subcommand.map(|value| format!("unknown patch subcommand: {value}")).unwrap_or_else(|| "patch subcommand required".to_owned()); return eoie_actionable_error(&reason, "patch", descriptor.2.as_ref(), descriptor.2.as_ref(), "eoie help patch"); }
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
    let subcommand = args.first().map(String::as_str); if subcommand.map(|value| eoie_agile_subcommand_known(value) == 0).unwrap_or(true) { let descriptor = eoie_command_descriptor("2"); let reason = subcommand.map(|value| format!("unknown agile subcommand: {value}")).unwrap_or_else(|| "agile subcommand required".to_owned()); return eoie_actionable_error(&reason, "agile", descriptor.2.as_ref(), descriptor.2.as_ref(), "eoie help agile"); }
    if args.len() == 4 && args[0] == "check" && args[2] == "--compiler" {
        return eoie_agile_state::agile_check_with_compiler_owned(args[1].as_str(), args[3].as_str());
    }
    eoie_agile_mutation::eoie_agile_run()
}

#[must_use]
fn EoieBundleAdapter() -> i32 {
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    let subcommand = args.first().map(String::as_str); if !matches!(subcommand, Some("growth-baseline" | "growth-receipt" | "check" | "create" | "create-flat" | "verify" | "verify-flat" | "rehydrate" | "retention-plan" | "retention-apply")) { let descriptor = eoie_command_descriptor("3"); let reason = subcommand.map(|value| format!("unknown bundle subcommand: {value}")).unwrap_or_else(|| "bundle subcommand required".to_owned()); return eoie_actionable_error(&reason, "bundle", descriptor.2.as_ref(), descriptor.2.as_ref(), "eoie help bundle"); }
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
    if args.is_empty() { let descriptor = eoie_command_descriptor("4"); return eoie_actionable_error("proxy capability required", "proxy capability", descriptor.2.as_ref(), descriptor.2.as_ref(), "eoie help proxy"); }
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
        if usage.is_empty() { let valid = eoie_command_descriptor("4").2; return eoie_actionable_error(&format!("unknown proxy help target: {}", args[1]), "proxy help", "help proxy <capability>", valid.as_ref(), "eoie help"); }
        println!("usage: eoie {}", usage);
        return 0;
    }
    if args.len() > 1 { let descriptor = eoie_command_descriptor("5"); let valid = format!("{}|proxy <capability>|--completion|--json|--schema", eoie_public_valid_values()); return eoie_actionable_error("help accepts zero arguments, one verb, proxy <capability>, --completion, --json, or --schema", "help", descriptor.2.as_ref(), &valid, "eoie help"); }
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
enum US1 {
    US1_StatusHandler,
    US1_PatchHandler,
    US1_AgileHandler,
    US1_BundleHandler,
    US1_ProxyHandler,
    US1_HelpHandler,
    US1_UnknownHandler,
    US1_LeaseDeniedHandler,
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
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = 1i32;
        let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
        if v2 {
            let mut v3: i32 = eoie_jobs::eoie_job_run();
            v3
        } else {
            let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repo"); } LIT.with(|lit| lit.clone()) };
            if v4 {
                let mut v5: i32 = eoie_repo_state::eoie_repo_run();
                v5
            } else {
                let mut v6: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table"); } LIT.with(|lit| lit.clone()) };
                if v6 {
                    let mut v7: i32 = eoie_table_query::eoie_table_run();
                    v7
                } else {
                    let mut v8: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read"); } LIT.with(|lit| lit.clone()) };
                    let mut v10: bool = if v8 {
                        true
                    } else {
                        let mut v9: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
                        v9
                    };
                    let mut v12: bool = if v10 {
                        true
                    } else {
                        let mut v11: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("grep"); } LIT.with(|lit| lit.clone()) };
                        v11
                    };
                    if v12 {
                        let mut v13: i32 = eoie_read_query::eoie_read_run();
                        v13
                    } else {
                        let mut v14: i32 = eoie_legacy_operations::dogfood_shim_argv0_code();
                        let mut v15: bool = 0i32 == v14;
                        if v15 {
                            let mut v16: i32 = EoieProcessVerbCode();
                            let mut v17: bool = 0i32 == v16;
                            let mut v38: US1 = if v17 {
                                US1::US1_StatusHandler
                            } else {
                                let mut v19: bool = 1i32 == v16;
                                if v19 {
                                    US1::US1_PatchHandler
                                } else {
                                    let mut v21: bool = 2i32 == v16;
                                    if v21 {
                                        US1::US1_AgileHandler
                                    } else {
                                        let mut v23: bool = 3i32 == v16;
                                        if v23 {
                                            US1::US1_BundleHandler
                                        } else {
                                            let mut v25: bool = 4i32 == v16;
                                            if v25 {
                                                US1::US1_ProxyHandler
                                            } else {
                                                let mut v27: bool = 5i32 == v16;
                                                if v27 {
                                                    US1::US1_HelpHandler
                                                } else {
                                                    let mut v29: bool = 7i32 == v16;
                                                    if v29 {
                                                        US1::US1_LeaseDeniedHandler
                                                    } else {
                                                        US1::US1_UnknownHandler
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            match &v38 {
                                US1::US1_AgileHandler => {
                                    let mut v70: i32 = EoieAgileAdapter();
                                    v70
                                }
                                US1::US1_BundleHandler => {
                                    let mut v71: i32 = EoieBundleAdapter();
                                    v71
                                }
                                US1::US1_HelpHandler => {
                                    let mut v73: i32 = EoieHelpRejected();
                                    v73
                                }
                                US1::US1_LeaseDeniedHandler => {
                                    2i32
                                }
                                US1::US1_PatchHandler => {
                                    let mut v69: i32 = EoiePatchAdapter();
                                    v69
                                }
                                US1::US1_ProxyHandler => {
                                    let mut v72: i32 = EoieProxyAdapter();
                                    v72
                                }
                                US1::US1_StatusHandler => {
                                    let mut v39: i32 = std::env::args().skip(2).count() as i32;
                                    let mut v40: bool = 0i32 == v39;
                                    if v40 {
                                        let mut v41: i32 = bootstrap_authority_status::eoie_bootstrap_workspace_state_status();
                                        let mut v42: bool = 1i32 == v41;
                                        if v42 {
                                            let mut v43: i32 = bootstrap_authority_status::eoie_bootstrap_compiler_session_status();
                                            let mut v44: bool = 1i32 == v43;
                                            if v44 {
                                                let mut v45: i32 = bootstrap_authority_status::eoie_bootstrap_legacy_bundle_writer_status();
                                                let mut v46: bool = 1i32 == v45;
                                                if v46 {
                                                    let mut v47: i32 = bootstrap_authority_status::eoie_bootstrap_ingest_frontier_status();
                                                    let mut v48: bool = 1i32 == v47;
                                                    if v48 {
                                                        let mut v49: i32 = process_batch_plan_domain::eoie_batch_matrix_auto_chain_budget(60000, 16000);
                                                        let mut v50: bool = 1i32 == v49;
                                                        if v50 {
                                                            let mut v51: i32 = bootstrap_authority_status::eoie_bootstrap_settled_migration_status();
                                                            let mut v52: i32 = EoieStatusOk(v51);
                                                            v52
                                                        } else {
                                                            let mut v53: i32 = EoieStatusAuthorityError();
                                                            v53
                                                        }
                                                    } else {
                                                        let mut v55: i32 = EoieStatusAuthorityError();
                                                        v55
                                                    }
                                                } else {
                                                    let mut v57: i32 = EoieStatusAuthorityError();
                                                    v57
                                                }
                                            } else {
                                                let mut v59: i32 = EoieStatusAuthorityError();
                                                v59
                                            }
                                        } else {
                                            let mut v61: i32 = EoieStatusAuthorityError();
                                            v61
                                        }
                                    } else {
                                        let mut v63: bool = 1i32 == v39;
                                        if v63 {
                                            let mut v64: i32 = bootstrap_authority_status::eoie_bootstrap_settled_migration_status();
                                            let mut v65: i32 = EoieStatusLive(v64);
                                            v65
                                        } else {
                                            let mut v66: i32 = EoieStatusArityError();
                                            v66
                                        }
                                    }
                                }
                                US1::US1_UnknownHandler => {
                                    let mut v74: i32 = EoieUnknownRejected();
                                    v74
                                }
                            }
                        } else {
                            let mut v83: i32 = eoie_legacy_operations::dogfood_shim_dispatch();
                            v83
                        }
                    }
                }
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn run_from_1(mut v0: US0, mut v1: Rc<dyn Fn() -> i32>) -> i32 {
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
    let mut v2: i32 = run_from_1(v0.clone(), v1.clone());
    let mut v3: i32 = eoie_coverage_prune::coverage_finish(v2);
    v3
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
