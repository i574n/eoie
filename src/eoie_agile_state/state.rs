#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use agile_state_decision_domain::{eoie_agile_state_series_sequence_binding, eoie_agile_state_series_source_binding, eoie_agile_state_series_pair_binding, eoie_agile_state_series_monotonic_binding, eoie_agile_state_series_fingerprint_binding, eoie_agile_state_entry_kind_binding, eoie_agile_state_transient_name_code, eoie_agile_state_record_required_name_bit, eoie_agile_state_record_required_catalog, eoie_agile_state_record_preflight_binding, eoie_agile_state_attestation_decision_binding};
use eoie_process::{resolve_spiral_compiler as agile_resolve_spiral_compiler, typecheck_spiral_file as agile_typecheck_spiral, SpiralCompilerDiscovery};
use eoie_rust_std_fs::{atomic_write as agile_atomic_write, read_regular_limited as agile_read_regular_limited, read_regular_text_limited as agile_read_regular_text_limited};
use eoie_agile_policy::{eoie_agile_attestation_normalize_binding, eoie_agile_attestation_value_binding, eoie_agile_set_decision_binding, eoie_agile_task_identity_binding, eoie_agile_receipt_content_binding, eoie_agile_receipt_snapshot_binding};
use agile_state_runtime_policy_domain::{eoie_agile_lease_budget_decision_binding, eoie_agile_lease_readback_decision_binding, eoie_agile_prompt_title_code, eoie_agile_receipt_attestation_kind_code, eoie_agile_receipt_attestation_text, eoie_agile_receipt_row_kind_code, eoie_agile_receipt_row_kind_text, eoie_agile_state_mutation_target_decision_binding, eoie_agile_status_text, eoie_agile_status_name_code, eoie_agile_task_line_candidate_code};
use state_receipt_fingerprint_domain::{eoie_state_receipt_high_step, eoie_state_receipt_low_step, eoie_state_receipt_mix_low};
use state_receipt_parser_domain::eoie_state_receipt_parse;
use state_receipt_tuple_domain::eoie_state_receipt_row_tuple;
use state_receipt_serializer_domain::eoie_state_receipt_serialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use eoie_agile_lease::{agile_escape_spi, agile_now_unix_ms, agile_parse_u64_decimal, agile_package_modules_and_anchor, agile_prompt_lease_clock, agile_tuple_fields};


fn agile_finish(result: Result<(), String>) -> i32 {
    if let Err(error) = result { eprintln!("eoie error: {error}"); 2 } else { 0 }
}

fn agile_arg<'a>(args: &'a [String], index: usize, name: &str) -> Result<&'a str, String> {
    args.get(index).map(String::as_str).ok_or_else(|| format!("missing argument: {name}"))
}

fn agile_task_id_matches(line: &str, id: &str) -> bool {
    let Some(start) = line.find("Task (") else { return false; };
    let Some(end) = line.rfind(')') else { return false; };
    let Ok(fields) = agile_tuple_fields(&line[start + 6..end]) else { return false; };
    let expected = format!("\"{}\"", agile_escape_spi(id));
    let id_equal = fields.first().map(String::as_str) == Some(expected.as_str());
    eoie_agile_task_identity_binding(fields.len() as i32, i32::from(id_equal)) == 1
}

const AGILE_FNV_OFFSET: u64 = 14_695_981_039_346_656_037;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AgileModuleReceipt {
    bytes: u64,
    fingerprint: u64,
    build_fingerprint: Option<u64>,
}

fn agile_hash_extend(hash: u64, bytes: &[u8]) -> u64 {
    let mut high = (hash >> 32) as u32 as i32;
    let mut low = hash as u32 as i32;
    for byte in bytes {
        let mixed_low = eoie_state_receipt_mix_low(low, i32::from(*byte));
        high = eoie_state_receipt_high_step(high, mixed_low);
        low = eoie_state_receipt_low_step(mixed_low, 0);
    }
    ((high as u32 as u64) << 32) | (low as u32 as u64)
}

fn agile_module_identity(path: &Path) -> Result<AgileModuleReceipt, String> {
    let bytes = agile_read_regular_limited(path, usize::MAX)?;
    Ok(AgileModuleReceipt { bytes: bytes.len() as u64, fingerprint: agile_hash_extend(AGILE_FNV_OFFSET, &bytes), build_fingerprint: None })
}

fn agile_snapshot_identity(state_dir: &Path, paths: &[PathBuf]) -> Result<(u64, bool), String> {
    let mut hash = agile_hash_extend(AGILE_FNV_OFFSET, b"state-topology-v3");
    for path in paths.iter().filter(|path| !agile_transient_receipt(path)) {
        let name = path.file_name().and_then(|value| value.to_str()).ok_or_else(|| format!("state module name is not UTF-8: {}", path.display()))?;
        hash = agile_hash_extend(hash, name.as_bytes());
        hash = agile_hash_extend(hash, &[0]);
        hash = agile_hash_extend(hash, &[255]);
    }
    let graph = eoie_agile_lease::agile_package_input_fingerprint(state_dir, false);
    hash = agile_hash_extend(hash, &graph.unwrap_or(0).to_le_bytes());
    Ok((hash, graph.is_some()))
}

fn agile_load_receipts(path: &Path, expected_snapshot: u64) -> Result<BTreeMap<String, AgileModuleReceipt>, String> {
    if !path.exists() { return Ok(BTreeMap::new()); }
    let source = agile_read_regular_text_limited(path, 8 * 1024 * 1024)?;
    let canonical = eoie_state_receipt_parse(&source);
    if source.replace("\r\n", "\n").trim_end() != eoie_state_receipt_serialize(&canonical).trim_end() { return Ok(BTreeMap::new()); }
    let mut lines = canonical.lines();
    let Some(snapshot_line) = lines.next() else { return Ok(BTreeMap::new()); };
    let (snapshot_kind, snapshot_value, snapshot_extra0, snapshot_extra1, snapshot_extra2) = eoie_state_receipt_row_tuple(snapshot_line);
    if eoie_agile_receipt_row_kind_code(snapshot_kind.as_ref()) != 1 || !snapshot_extra0.is_empty() || !snapshot_extra1.is_empty() || !snapshot_extra2.is_empty() { return Ok(BTreeMap::new()); }
    let persisted_snapshot = agile_parse_u64_decimal(snapshot_value.as_ref(), "snapshot")?;
    let snapshot_equal = i32::from(persisted_snapshot == expected_snapshot);
    if eoie_agile_receipt_snapshot_binding(snapshot_equal, 0) != 1 { return Ok(BTreeMap::new()); }
    let mut receipts = BTreeMap::new();
    for line in lines {
        let (kind, module, bytes_text, fingerprint_text, attestation) = eoie_state_receipt_row_tuple(line);
        if eoie_agile_receipt_row_kind_code(kind.as_ref()) != 2 { return Err("unknown receipt row".to_owned()); }
        let bytes = agile_parse_u64_decimal(bytes_text.as_ref(), "module receipt bytes")?;
        let fingerprint = agile_parse_u64_decimal(fingerprint_text.as_ref(), "module receipt fingerprint")?;
        let normalized_attestation = eoie_agile_attestation_normalize_binding(attestation.as_ref());
        if normalized_attestation.is_empty() { return Err(format!("invalid module receipt attestation: {}", attestation)); }
        let attestation_kind = eoie_agile_receipt_attestation_kind_code(normalized_attestation.as_ref());
        let build_fingerprint = match attestation_kind {
            1 => None,
            2 => Some(eoie_agile_attestation_value_binding(attestation.as_ref())),
            _ => return Err(format!("typed Spiral receipt attestation kind rejected: {}", attestation)),
        };
        receipts.insert(module.to_string(), AgileModuleReceipt { bytes, fingerprint, build_fingerprint });
    }
    Ok(receipts)
}

fn agile_write_receipts(path: &Path, snapshot: u64, receipts: &BTreeMap<String, AgileModuleReceipt>) -> Result<(), String> {
    let snapshot_kind = eoie_agile_receipt_row_kind_text("1");
    let module_kind = eoie_agile_receipt_row_kind_text("2");
    if snapshot_kind.is_empty() || module_kind.is_empty() { return Err("typed Spiral receipt row-kind serialization rejected".to_owned()); }
    let mut canonical = format!("{}\t{snapshot}\n", snapshot_kind);
    for (module, receipt) in receipts {
        let build_fingerprint = receipt.build_fingerprint.map(|value| value.to_string()).unwrap_or_default();
        let attestation = eoie_agile_receipt_attestation_text(&build_fingerprint);
        if attestation.is_empty() { return Err("typed Spiral receipt attestation serialization rejected".to_owned()); }
        canonical.push_str(&format!("{}\t{module}\t{}\t{}\t{}\n", module_kind, receipt.bytes, receipt.fingerprint, attestation));
    }
    let source = eoie_state_receipt_serialize(&canonical);
    agile_atomic_write(path, source.as_bytes())
}

fn agile_state_paths(state_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = fs::read_dir(state_dir).map_err(|error| format!("read {}: {error}", state_dir.display()))?
        .map(|entry| entry.map(|entry| entry.path()).map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.retain(|path| path.file_name().and_then(|value| value.to_str()).map(|name| agile_state_transient_schema_domain::eoie_agile_state_path_candidate_code(name) == 1).unwrap_or(false));
    paths.sort();
    Ok(paths)
}

fn agile_attest_known_mutation(root: &Path, path: &Path, previous: Option<&[u8]>) -> Result<(), String> {
    let state_dir = root.join("state");
    let parent_is_state = i32::from(path.parent() == Some(state_dir.as_path()));
    if eoie_agile_state_mutation_target_decision_binding(parent_is_state, 0) == 0 { return Err(format!("agile mutation escaped state root: {}", path.display())); }
    let receipt_path = state_dir.join("typecheck_receipts.spi");
    let paths = agile_state_paths(&state_dir)?;
    let module = path.file_name().and_then(|value| value.to_str()).ok_or_else(|| format!("state module name is not UTF-8: {}", path.display()))?.to_owned();
    let (static_modules, _) = agile_package_modules_and_anchor(&state_dir)?;
    let mutation_decision = eoie_agile_state_mutation_target_decision_binding(parent_is_state, i32::from(static_modules.contains(&module)));
    if mutation_decision != 2 { return Err(format!("agile mutation target is not a package-owned state module: {module}")); }
    let snapshot = agile_snapshot_identity(&state_dir, &paths)?.0;
    let mut receipts = agile_load_receipts(&receipt_path, snapshot)?;
    if receipt_path.exists() && receipts.is_empty() { return Ok(()); }
    if let Some(previous) = previous {
        let unchanged = receipts.get(&module).is_some_and(|receipt| receipt.bytes == previous.len() as u64 && receipt.fingerprint == agile_hash_extend(AGILE_FNV_OFFSET, previous));
        if !unchanged { return Ok(()); }
    }
    receipts.insert(module, agile_module_identity(path)?);
    agile_write_receipts(&receipt_path, snapshot, &receipts)
}

fn agile_typecheck_external_drift(path: &Path, compiler: &SpiralCompilerDiscovery) -> Result<(), String> { agile_typecheck_spiral(compiler, path) }

fn agile_list_from_args(args: Vec<String>) -> Result<(), String> {
    if !(2..=3).contains(&args.len()) || agile_arg(&args, 0, "subcommand")? != "list" { return Err("agile list expects list <root> [Planned|Active|Blocked|Paused|Done]".to_owned()); }
    let filter = args.get(2).map(|status| eoie_agile_status_name_code(status));
    if filter.is_some_and(|code| code >= 5) { return Err("invalid agile status filter; expected Planned|Active|Blocked|Paused|Done".to_owned()); }
    let state = Path::new(agile_arg(&args, 1, "root")?).join("state/agile.spi");
    let source = agile_read_regular_text_limited(&state, 8 * 1024 * 1024)?;
    for line in source.lines().filter(|line| eoie_agile_task_line_candidate_code(line) == 1) {
        if let Some(filter) = filter {
            let start = line.find("Task (").ok_or("malformed Task")? + 6;
            let end = line.rfind(')').ok_or("malformed Task terminator")?;
            let fields = agile_tuple_fields(line.get(start..end).ok_or("malformed Task range")?)?;
            if eoie_agile_task_identity_binding(fields.len() as i32, 1) != 1 { return Err("malformed Task fields".to_owned()); }
            let status = eoie_agile_status_name_code(&fields[6]);
            if status >= 5 { return Err("invalid Task status".to_owned()); }
            if status != filter { continue; }
        }
        println!("{}", line.trim());
    }
    Ok(())
}


fn agile_begin_from_args(args: Vec<String>) -> Result<(), String> {
    if args.len() != 3 || agile_arg(&args, 0, "subcommand")? != "begin" { return Err("agile begin expects begin <root> <title>".to_owned()); }
    let root = Path::new(agile_arg(&args, 1, "root")?);
    if !root.is_dir() { return Err(format!("agile root is not a directory: {}", root.display())); }
    let title = agile_arg(&args, 2, "title")?;
    if eoie_agile_prompt_title_code(title) != 1 { return Err("typed prompt title is empty".to_owned()); }
    let prompt_id = env::var("EOIE_PROMPT_ID").unwrap_or_else(|_| "current".to_owned());
    let budget_ms = env::var("EOIE_BUDGET_MS").ok().map(|value| agile_parse_u64_decimal(&value, "EOIE_BUDGET_MS")).transpose()?.unwrap_or(4_500_000);
    let guard_ms = env::var("EOIE_GUARD_MS").ok().map(|value| agile_parse_u64_decimal(&value, "EOIE_GUARD_MS")).transpose()?.unwrap_or(600_000);
    if eoie_agile_lease_budget_decision_binding(i32::from(budget_ms > 0), i32::from(guard_ms < budget_ms)) != 1 { return Err("prompt budget must be positive and larger than guard".to_owned()); }
    let started_unix_ms = agile_now_unix_ms()?;
    let deadline_unix_ms = started_unix_ms.checked_add(budget_ms).and_then(|value| value.checked_add(guard_ms)).ok_or_else(|| "prompt lease deadline overflow".to_owned())?;
    let state = root.join("state/prompt.spi");
    let original = agile_read_regular_limited(&state, 1024 * 1024).ok();
    let source = format!("union lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\ninl current () : prompt_lease = PromptLease (\"{}\", \"{}\", {}u64, {}u64, {}u64, {}u64, LeaseActive)\ninl main () : i32 = 0i32\n", agile_escape_spi(&prompt_id), agile_escape_spi(title), budget_ms, guard_ms, started_unix_ms, deadline_unix_ms);
    agile_atomic_write(&state, source.as_bytes())?;
    let validate = (|| -> Result<(), String> {
        let (observed_started, observed_deadline, _, _, _, _) = agile_prompt_lease_clock(root)?;
        if eoie_agile_lease_readback_decision_binding(i32::from(observed_started == started_unix_ms), i32::from(observed_deadline == deadline_unix_ms)) != 1 { return Err("prompt lease runtime validation mismatch".to_owned()); }
        agile_attest_known_mutation(root, &state, None)
    })();
    if let Err(error) = validate {
        if let Some(original) = original { agile_atomic_write(&state, &original)?; } else { let _ = fs::remove_file(&state); }
        return Err(format!("prompt lease rolled back: {error}"));
    }
    println!("eoie agile begin ok prompt={prompt_id} budget_ms={budget_ms} guard_ms={guard_ms} started_unix_ms={started_unix_ms} deadline_unix_ms={deadline_unix_ms} validation=runtime-attested compiler=not-required");
    Ok(())
}

fn agile_set_from_args(args: Vec<String>, progress: i32, status_code: i32) -> Result<(), String> {
    let started = agile_update_receipt::eoie_agile_clock_ms();
    if args.len() != 5 || agile_arg(&args, 0, "subcommand")? != "set" { return Err("agile set expects root, id, progress, status".to_owned()); }
    if eoie_agile_set_decision_binding(progress, status_code) != 1 { return Err("typed Spiral agile mutation policy rejected progress/status".to_owned()); }
    let status_code_text = status_code.to_string();
    let status_value = eoie_agile_status_text(&status_code_text);
    if status_value.is_empty() { return Err("typed Spiral agile status serialization rejected code".to_owned()); }
    let status = status_value.as_ref();
    let root = Path::new(agile_arg(&args, 1, "root")?);
    let state = root.join("state/agile.spi");
    let id = agile_arg(&args, 2, "id")?;
    let original = agile_read_regular_text_limited(&state, 8 * 1024 * 1024)?;
    let mut found = false;
    let mut lines = Vec::new();
    for line in original.lines() {
        if eoie_agile_task_line_candidate_code(line) == 1 && agile_task_id_matches(line, id) {
                let start = line.find("Task (").ok_or_else(|| "malformed Task".to_owned())? + 6;
                let end = line.rfind(')').ok_or_else(|| "malformed Task terminator".to_owned())?;
                let mut parts = agile_tuple_fields(&line[start..end])?;
                if eoie_agile_task_identity_binding(parts.len() as i32, 1) != 1 { return Err(format!("Task {id} has a shape rejected by the typed Spiral policy: fields={}", parts.len())); }
                parts[5] = format!("{progress}u32");
                parts[6] = status.to_owned();
                let prompt_id = env::var("EOIE_PROMPT_ID").unwrap_or_else(|_| "current".to_owned());
                parts[10] = format!("\"{}\"", agile_escape_spi(&prompt_id));
                lines.push(format!("{}{}{}", &line[..start], parts.join(", "), &line[end..]));
                found = true;
            continue;
        }
        lines.push(line.to_owned());
    }
    if !found { return Err(format!("agile task not found: {id}")); }
    let updated = format!("{}\n", lines.join("\n"));
    agile_atomic_write(&state, updated.as_bytes())?;
    let validate = (|| -> Result<(), String> {
        let observed = agile_read_regular_text_limited(&state, 8 * 1024 * 1024)?;
        if observed != updated { return Err("agile update readback mismatch".to_owned()); }
        let line = observed.lines().find(|line| agile_task_id_matches(line, id)).ok_or_else(|| format!("updated agile task disappeared: {id}"))?;
        let start = line.find("Task (").ok_or_else(|| "updated Task malformed".to_owned())? + 6;
        let end = line.rfind(')').ok_or_else(|| "updated Task terminator missing".to_owned())?;
        let parts = agile_tuple_fields(&line[start..end])?;
        if eoie_agile_task_identity_binding(parts.len() as i32, 1) != 1 { return Err("updated Task shape rejected by typed Spiral policy".to_owned()); }
        let expected_progress = format!("{progress}u32");
        if parts.get(5).map(String::as_str) != Some(expected_progress.as_str()) || parts.get(6).map(String::as_str) != Some(status) { return Err("agile update runtime validation mismatch".to_owned()); }
        agile_attest_known_mutation(root, &state, Some(original.as_bytes()))
    })();
    if let Err(error) = validate { agile_atomic_write(&state, original.as_bytes())?; return Err(format!("agile update rolled back: {error}")); }
    println!("{}", agile_update_receipt::eoie_agile_set_receipt_text(id, progress, status, original.len() as u64, updated.len() as u64, started));
    Ok(())
}

fn agile_transient_receipt(path: &Path) -> bool {
    path.file_name().and_then(|value| value.to_str()).map(|name| eoie_agile_state_transient_name_code(name) == 1).unwrap_or(false)
}

fn agile_validate_transient(path: &Path) -> Result<(), String> {
    let source = agile_read_regular_text_limited(path, 8 * 1024 * 1024)?;
    let name = path.file_name().and_then(|value| value.to_str()).ok_or_else(|| format!("transient state name is not UTF-8: {}", path.display()))?;
    let projection = format!("{name}\n{source}");
    if agile_state_transient_schema_domain::eoie_agile_state_transient_schema_code(&projection) == 1 { Ok(()) } else { Err(format!("known transient state receipt failed runtime schema validation: {name}")) }
}

fn agile_check_classified_from_args(args: Vec<String>) -> Result<(), String> {
    if agile_arg(&args, 0, "subcommand")? != "check" || !(args.len() == 2 || (args.len() == 4 && agile_arg(&args, 2, "flag")? == "--compiler")) { return Err("agile check expects check <root> [--compiler <absolute>]".to_owned()); }
    let explicit_compiler = if args.len() == 4 { Some(agile_arg(&args, 3, "compiler")?) } else { None };
    let compiler = agile_resolve_spiral_compiler(explicit_compiler)?;
    let root = Path::new(agile_arg(&args, 1, "root")?);
    if !root.is_dir() { return Err(format!("agile root is not a directory: {}", root.display())); }
    let state_dir = root.join("state");
    let receipt_path = state_dir.join("typecheck_receipts.spi");
    let (static_modules, batch_anchor) = agile_package_modules_and_anchor(&state_dir)?;
    let paths = agile_state_paths(&state_dir)?;
    let (snapshot, cacheable) = agile_snapshot_identity(&state_dir, &paths)?;
    let graph_before = eoie_agile_lease::agile_package_input_fingerprint(&state_dir, true);
    let mut receipts = agile_load_receipts(&receipt_path, snapshot)?;
    let mut batch_attested = false;
    let mut build_batches = 0usize;
    let identities = paths.iter().map(|path| agile_module_identity(path)).collect::<Result<Vec<_>, _>>()?;
    if !cacheable {
        agile_typecheck_external_drift(&batch_anchor, compiler.as_ref().ok_or("package graph cannot reuse cached receipts; configure a Spiral compiler")?)?;
        batch_attested = true; build_batches = 1;
    }
    let mut skipped = 0usize;
    let mut transient = 0usize;
    let mut runtime_attested = 0usize;
    let mut build_attested = 0usize;
    for (path, initial_identity) in paths.iter().zip(&identities) {
        let module = path.file_name().and_then(|value| value.to_str()).ok_or_else(|| format!("state module name is not UTF-8: {}", path.display()))?.to_owned();
        let package_owned = i32::from(static_modules.contains(&module));
        let transient_known = i32::from(agile_transient_receipt(path));
        let kind = eoie_agile_state_entry_kind_binding(package_owned, transient_known);
        if kind == 2 { transient += 1; }
        let mut identity = *initial_identity;
        let same_content = receipts.get(&module).map(|receipt| eoie_agile_receipt_content_binding(i32::from(receipt.bytes == identity.bytes), i32::from(receipt.fingerprint == identity.fingerprint)) == 1).unwrap_or(false);
        let build_equal = compiler.as_ref().and_then(|compiler| receipts.get(&module).map(|receipt| receipt.build_fingerprint == Some(compiler.fingerprint))).unwrap_or(false);
        let observation_mask = i32::from(same_content) | (i32::from(compiler.is_some()) << 1) | (i32::from(build_equal) << 2);
        match eoie_agile_state_attestation_decision_binding(kind, observation_mask) {
            0 => return Err(format!("state entry is neither one static package module nor one known transient receipt: {module}")),
            1 => { skipped += 1; continue; }
            2 => {
                agile_validate_transient(path)?;
                receipts.insert(module, identity);
                runtime_attested += 1;
            }
            3 => {
                let compiler = compiler.as_ref().ok_or_else(|| "typed attestation decision requires compiler".to_owned())?;
                if !batch_attested {
                    agile_typecheck_external_drift(&batch_anchor, compiler).map_err(|error| format!("{}: {error}", batch_anchor.display()))?;
                    batch_attested = true;
                    build_batches += 1;
                }
                identity.build_fingerprint = Some(compiler.fingerprint);
                receipts.insert(module, identity);
                build_attested += 1;
            }
            4 => return Err(format!("{}: state drift requires build-time Spiral attestation; configure compiler by --compiler, EOIE_SPIRAL_COMPILE, EOIE_CONFIG_HOME/XDG/HOME, PATH, or the deployed compiler {}", path.display(), eoie_compiler_discovery::deployed_spiral_compiler_path().map(|path| path.display().to_string()).unwrap_or_else(|| "(no cache directory)".to_owned()))),
            other => return Err(format!("typed attestation decision returned unsupported code: {other}")),
        }
    }
    if graph_before != eoie_agile_lease::agile_package_input_fingerprint(&state_dir, true) || paths != agile_state_paths(&state_dir)? || paths.iter().zip(&identities).any(|(path, identity)| agile_module_identity(path).ok().as_ref() != Some(identity)) { return Err("state or package inputs changed during attestation; retry agile check".to_owned()); }
    agile_write_receipts(&receipt_path, snapshot, &receipts)?;
    let (started_unix_ms, deadline_unix_ms, wrap_at_unix_ms, remaining_ms, should_wrap, should_yield) = agile_prompt_lease_clock(root)?;
    println!("eoie agile lease started_unix_ms={started_unix_ms} deadline_unix_ms={deadline_unix_ms} wrap_at_unix_ms={wrap_at_unix_ms} remaining_ms={remaining_ms} should_wrap={} should_yield={}", i32::from(should_wrap), i32::from(should_yield));
    let record_catalog = eoie_agile_state_record_required_catalog("v1");
    let record_required = record_catalog.as_ref().split('|').filter(|name| !name.is_empty()).collect::<Vec<_>>();
    let record_missing = record_required.iter().filter(|name| !state_dir.join(name).is_file()).copied().collect::<Vec<_>>();
    let mut record_mask = 0i32;
    for path in &paths { if let Some(name) = path.file_name().and_then(|value| value.to_str()) { record_mask |= eoie_agile_state_record_required_name_bit(name) as i32; } }
    let record_ready = eoie_agile_state_record_preflight_binding(record_mask, 0);
    if record_ready < 0 { return Err(format!("typed agile record preflight rejected mask={record_mask}")); }
    println!("{}", agile_record_catalog::eoie_agile_record_catalog_text("v1", (record_required.len() - record_missing.len()) as i32, record_required.len() as i32, &record_missing.join(",")));
    let compiler_status = compiler.as_ref().map(|value| format!("available source={} fingerprint={} sha256={} path={}", value.source, value.fingerprint, value.sha256, value.path.display())).unwrap_or_else(|| "absent".to_owned());
    println!("eoie agile check ok files={} skipped={skipped} transient={transient} runtime_attested={runtime_attested} build_attested={build_attested} build_batches={build_batches} validation=receipt-first compiler={compiler_status}", paths.len() + 1);
    Ok(())
}

use eoie_proxy_search::mnemonic_path_tag;
use state_receipt_codec_core::{eoie_state_receipt_dash_shape, eoie_state_receipt_last_series_record_payload, eoie_state_receipt_migration_denominator_payload, eoie_state_receipt_series_records_valid, eoie_state_receipt_u32_spi_literal_valid, eoie_state_receipt_u32_spi_literal_value};

fn agile_series_u32(field: &str, name: &str) -> Result<u32, String> {
    let value = field.trim();
    if eoie_state_receipt_u32_spi_literal_valid(value) != 1 { return Err(format!("invalid {name}: expected u32 decimal")); }
    Ok(eoie_state_receipt_u32_spi_literal_value(value) as u32)
}

fn agile_series_migration(source: &str) -> Result<(u32, u32), String> {
    let payload = eoie_state_receipt_migration_denominator_payload(source);
    if payload.is_empty() { return Err("migration denominator is missing or malformed".to_owned()); }
    let parts = agile_tuple_fields(payload.as_ref())?;
    if parts.len() < 4 { return Err("migration denominator shape is unsupported".to_owned()); }
    Ok((agile_series_u32(&parts[1], "settled units")?, agile_series_u32(&parts[3], "migration score")?))
}

fn agile_series_last(source: &str) -> Result<(u32, u32, u32), String> {
    if eoie_state_receipt_series_records_valid(source) != 1 { return Err("series records are malformed or not append-only".to_owned()); }
    let payload = eoie_state_receipt_last_series_record_payload(source);
    if payload.is_empty() { return Ok((0u32, 0u32, 0u32)); }
    let parts = agile_tuple_fields(payload.as_ref())?;
    if parts.len() != 10 { return Err(format!("series record has {} fields instead of ten", parts.len())); }
    Ok((agile_series_u32(&parts[0], "series sequence")?, agile_series_u32(&parts[1], "series settled units")?, agile_series_u32(&parts[2], "series migration score")?))
}

fn agile_record_from_args(args: Vec<String>) -> Result<(), String> {
    if args.len() != 2 || agile_arg(&args, 0, "subcommand")? != "record" { return Err("agile record expects record <root>".to_owned()); }
    let root = Path::new(agile_arg(&args, 1, "root")?);
    if !root.is_dir() { return Err(format!("agile root is not a directory: {}", root.display())); }
    let history_path = root.join("state/history.spi");
    let ratings_path = root.join("state/ratings.spi");
    let bench_path = root.join("state/bench.spi");
    let migration_path = root.join("state/migration.spi");
    let original = agile_read_regular_text_limited(&history_path, 8 * 1024 * 1024)?;
    let migration = agile_read_regular_text_limited(&migration_path, 8 * 1024 * 1024)?;
    let (settled_units, migration_score) = agile_series_migration(&migration)?;
    let (previous_sequence, previous_units, previous_score) = agile_series_last(&original)?;
    let sequence = previous_sequence.checked_add(1).ok_or_else(|| "series sequence overflow".to_owned())?;
    if eoie_agile_state_series_sequence_binding(previous_sequence as i32, sequence as i32) != 1 { return Err("typed series sequence rejected".to_owned()); }
    if eoie_agile_state_series_monotonic_binding(previous_units as i32, settled_units as i32) != 1 || eoie_agile_state_series_monotonic_binding(previous_score as i32, migration_score as i32) != 1 { return Err("typed series settlement regression rejected".to_owned()); }
    let ratings_identity = agile_module_identity(&ratings_path)?;
    let bench_identity = agile_module_identity(&bench_path)?;
    let ratings_relative = "state/ratings.spi";
    let bench_relative = "state/bench.spi";
    let ratings_tag = mnemonic_path_tag(ratings_relative)?;
    let bench_tag = mnemonic_path_tag(bench_relative)?;
    if eoie_agile_state_series_source_binding(ratings_relative.len() as i32, eoie_state_receipt_dash_shape(&ratings_tag) as i32) != 1 || eoie_agile_state_series_source_binding(bench_relative.len() as i32, eoie_state_receipt_dash_shape(&bench_tag) as i32) != 1 { return Err("typed series source rejected".to_owned()); }
    if eoie_agile_state_series_pair_binding(1, 1) != 1 || eoie_agile_state_series_fingerprint_binding(i32::from(ratings_identity.fingerprint != 0), i32::from(bench_identity.fingerprint != 0)) != 1 { return Err("typed series evidence pair rejected".to_owned()); }
    let prompt_id = env::var("EOIE_PROMPT_ID").unwrap_or_else(|_| "current".to_owned());
    let record = format!("inl series_record_{sequence:06} () : series_record = SeriesRecord ({sequence}u32, {settled_units}u32, {migration_score}u32, \"{}\", \"{}\", {}u64, \"{}\", \"{}\", {}u64, \"{}\")\n", ratings_relative, agile_escape_spi(&ratings_tag), ratings_identity.fingerprint, bench_relative, agile_escape_spi(&bench_tag), bench_identity.fingerprint, agile_escape_spi(&prompt_id));
    let marker = "inl retired_module_count () : u32";
    if !original.contains(marker) { return Err("history insertion marker is missing".to_owned()); }
    let updated = original.replacen(marker, &format!("{record}\n{marker}"), 1);
    agile_atomic_write(&history_path, updated.as_bytes())?;
    let validate = (|| -> Result<(), String> {
        let observed = agile_read_regular_text_limited(&history_path, 8 * 1024 * 1024)?;
        let (observed_sequence, observed_units, observed_score) = agile_series_last(&observed)?;
        if observed_sequence != sequence || observed_units != settled_units || observed_score != migration_score { return Err("series runtime validation mismatch".to_owned()); }
        agile_attest_known_mutation(root, &history_path, Some(original.as_bytes()))
    })();
    if let Err(error) = validate { agile_atomic_write(&history_path, original.as_bytes())?; return Err(format!("series append rolled back: {error}")); }
    println!("eoie agile record ok sequence={sequence} units={settled_units} migration={migration_score}/1000 ratings_tag={ratings_tag} bench_tag={bench_tag} validation=runtime-attested compiler=not-required");
    Ok(())
}

#[must_use]
pub fn agile_list_owned(root: &str) -> i32 {
    let args = vec!["list".to_owned(), root.to_owned()];
    agile_finish(agile_list_from_args(args))
}

#[must_use]
pub fn agile_list_filtered_owned(root: &str, status: &str) -> i32 {
    agile_finish(agile_list_from_args(vec!["list".to_owned(), root.to_owned(), status.to_owned()]))
}

#[must_use]
pub fn agile_check_owned(root: &str) -> i32 {
    let args = vec!["check".to_owned(), root.to_owned()];
    agile_finish(agile_check_classified_from_args(args))
}

#[must_use]
pub fn agile_check_with_compiler_owned(root: &str, compiler: &str) -> i32 {
    let args = vec!["check".to_owned(), root.to_owned(), "--compiler".to_owned(), compiler.to_owned()];
    agile_finish(agile_check_classified_from_args(args))
}

#[must_use]
pub fn agile_lease_status_owned(root: &str) -> i32 {
    let root = std::path::Path::new(root);
    let result = agile_prompt_lease_clock(root).map(|(started, deadline, wrap_at, remaining, should_wrap, should_yield)| {
        println!("{}", agile_lease_terms::eoie_lease_status_text(&root.display().to_string(), started, wrap_at, deadline, remaining, should_wrap, should_yield));
    });
    agile_finish(result)
}

#[must_use]
pub fn agile_begin_owned(root: &str, title: &str) -> i32 {
    let args = vec!["begin".to_owned(), root.to_owned(), title.to_owned()];
    agile_finish(agile_begin_from_args(args))
}

#[must_use]
pub fn agile_record_owned(root: &str) -> i32 {
    let args = vec!["record".to_owned(), root.to_owned()];
    agile_finish(agile_record_from_args(args))
}

#[must_use]
pub fn agile_set_validated_owned(root: &str, id: &str, progress: i32, status_code: i32) -> i32 {
    let args = vec!["set".to_owned(), root.to_owned(), id.to_owned(), progress.to_string(), status_code.to_string()];
    agile_finish(agile_set_from_args(args, progress, status_code))
}

fn spiral_main() -> i32 {
    0i32
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
