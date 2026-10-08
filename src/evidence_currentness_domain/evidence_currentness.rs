#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use eoie_patch_control_domain::{patch_quoted_fields, patch_safe_relative};
use eoie_proxy_search::{inspection_file_sha256, inspection_sha256, inspection_tree_sha256};
use eoie_rust_std_fs::atomic_write;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct EvidenceCurrentnessSummary { pub declared: usize, pub observed: usize, pub verified: usize }

#[derive(Clone, Debug)]
struct EvidenceSpec { relative: String, sha256: String, bytes: u64 }

#[derive(Clone, Debug)]
enum EvidenceTreePolicy { Exact, StateWithoutMetaManifests }

#[derive(Clone, Debug)]
struct EvidenceTreeSpec { relative: String, sha256: String, policy: EvidenceTreePolicy }

fn evidence_parse_tree_specs(source: &str) -> Result<Vec<EvidenceTreeSpec>, String> {
    let mut specs = Vec::new();
    let mut seen = BTreeSet::new();
    for (number, line) in source.lines().enumerate() {
        if !line.contains("EvidenceTreeRef (") { continue; }
        let fields = patch_quoted_fields(line)?;
        if fields.len() != 2 { return Err(format!("EvidenceTreeRef on line {} expects path and sha256 strings", number + 1)); }
        let relative = patch_safe_relative(&fields[0])?;
        let relative_text = relative.to_string_lossy().replace('\\', "/");
        if !seen.insert(relative_text.clone()) { return Err(format!("duplicate EvidenceTreeRef path: {relative_text}")); }
        let sha256 = fields[1].to_ascii_lowercase();
        if !evidence_sha256_text(&sha256) { return Err(format!("EvidenceTreeRef sha256 is malformed on line {}", number + 1)); }
        let policy = if line.contains("StateTreeWithoutMetaManifests") {
            if relative_text != "state" { return Err("StateTreeWithoutMetaManifests is valid only for state".to_owned()); }
            EvidenceTreePolicy::StateWithoutMetaManifests
        } else if line.contains("ExactTree") { EvidenceTreePolicy::Exact } else { return Err(format!("EvidenceTreeRef policy missing on line {}", number + 1)); };
        specs.push(EvidenceTreeSpec { relative: relative_text, sha256, policy });
    }
    Ok(specs)
}

fn evidence_tree_relative(root: &Path, path: &Path) -> Result<String, String> {
    let relative = path.strip_prefix(root).map_err(|_| format!("tree path escaped root: {}", path.display()))?;
    Ok(relative.components().map(|component| component.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("/"))
}

fn evidence_tree_collect_filtered(root: &Path, path: &Path, excluded_a: &Path, excluded_b: &Path, manifest: &mut Vec<String>, total: &mut u64, limit: u64) -> Result<(), String> {
    if path == excluded_a || path == excluded_b { return Ok(()); }
    let metadata = fs::symlink_metadata(path).map_err(|error| format!("metadata {}: {error}", path.display()))?;
    if metadata.file_type().is_symlink() { return Err(format!("tree hash refuses symlink: {}", path.display())); }
    let relative = evidence_tree_relative(root, path)?;
    if metadata.is_dir() {
        manifest.push(format!("d\t{relative}"));
        let mut children = fs::read_dir(path).map_err(|error| format!("read {}: {error}", path.display()))?.map(|entry| entry.map(|entry| entry.path()).map_err(|error| error.to_string())).collect::<Result<Vec<_>, _>>()?;
        children.sort();
        for child in children { evidence_tree_collect_filtered(root, &child, excluded_a, excluded_b, manifest, total, limit)?; }
    } else if metadata.is_file() {
        *total = total.checked_add(metadata.len()).ok_or_else(|| "tree byte count overflow".to_owned())?;
        if *total > limit { return Err(format!("tree hash input exceeds limit: bytes={} limit={limit}", *total)); }
        manifest.push(format!("f\t{relative}\t{}\t{}", metadata.len(), inspection_file_sha256(path)?));
    } else { return Err(format!("unsupported tree entry: {}", path.display())); }
    Ok(())
}

pub fn evidence_state_tree_sha256(root: &Path) -> Result<String, String> {
    let path = root.join("state");
    let excluded_evidence = root.join("state/evidence.spi");
    let excluded_attestation = root.join("state/typecheck_receipts.spi");
    let limit = env::var("EOIE_TREE_HASH_LIMIT_BYTES").ok().and_then(|value| value.parse::<u64>().ok()).unwrap_or(1024 * 1024 * 1024);
    let mut manifest = Vec::new();
    let mut total = 0u64;
    evidence_tree_collect_filtered(root, &path, &excluded_evidence, &excluded_attestation, &mut manifest, &mut total, limit)?;
    manifest.sort();
    Ok(inspection_sha256(manifest.join("\n").into_bytes()))
}

pub fn refresh_release_closeout_evidence(root: &Path, closeout_sha256: &str, closeout_bytes: u64) -> Result<i32, String> {
    if !evidence_sha256_text(closeout_sha256) { return Err("closeout evidence sha256 is malformed".to_owned()); }
    let manifest_path = root.join("state/evidence.spi");
    let original = fs::read_to_string(&manifest_path).map_err(|error| format!("read {}: {error}", manifest_path.display()))?;
    let state_sha256 = evidence_state_tree_sha256(root)?;
    let cold_proof_path = root.join("state/cold_proof.spi");
    let cold_proof_metadata = fs::symlink_metadata(&cold_proof_path).map_err(|error| format!("metadata {}: {error}", cold_proof_path.display()))?;
    if !cold_proof_metadata.is_file() || cold_proof_metadata.file_type().is_symlink() { return Err("cold proof evidence target must be a regular file".to_owned()); }
    let cold_proof_sha256 = inspection_file_sha256(&cold_proof_path)?;
    let cold_proof_bytes = cold_proof_metadata.len();
    let cargo_lock_path = root.join("src/Cargo.lock");
    let cargo_lock_metadata = fs::symlink_metadata(&cargo_lock_path).map_err(|error| format!("metadata {}: {error}", cargo_lock_path.display()))?;
    if !cargo_lock_metadata.is_file() || cargo_lock_metadata.file_type().is_symlink() { return Err("Cargo.lock evidence target must be a regular file".to_owned()); }
    let cargo_lock_sha256 = inspection_file_sha256(&cargo_lock_path)?;
    let cargo_lock_bytes = cargo_lock_metadata.len();
    let eoie_name = evidence_public_binary(root)?;
    let eoie_path = root.join(&eoie_name);
    let eoie_metadata = fs::symlink_metadata(&eoie_path).map_err(|error| format!("metadata {}: {error}", eoie_path.display()))?;
    if !eoie_metadata.is_file() || eoie_metadata.file_type().is_symlink() { return Err("EOIE binary evidence target must be a regular file".to_owned()); }
    let eoie_sha256 = inspection_file_sha256(&eoie_path)?;
    let eoie_bytes = eoie_metadata.len();
    let source_sha256 = inspection_tree_sha256(root, "src")?;
    let mut release_hits = 0usize;
    let mut cold_hits = 0usize;
    let mut cargo_hits = 0usize;
    let mut attested_hits = 0usize;
    let mut binary_hits = 0usize;
    let mut source_hits = 0usize;
    let mut state_hits = 0usize;
    let mut output = Vec::new();
    for line in original.lines() {
        let mut updated = line.to_owned();
        if line.contains("EvidenceRef (") {
            let fields = patch_quoted_fields(line)?;
            if fields.len() == 2 && fields[0] == "state/release_closeout.spi" {
                release_hits += 1;
                updated = updated.replacen(&fields[1], closeout_sha256, 1);
                let old_bytes = evidence_parse_u64(line)?;
                updated = updated.replacen(&format!("{old_bytes}u64"), &format!("{closeout_bytes}u64"), 1);
            }
            if fields.len() == 2 && fields[0] == "state/cold_proof.spi" {
                cold_hits += 1;
                updated = updated.replacen(&fields[1], &cold_proof_sha256, 1);
                let old_bytes = evidence_parse_u64(line)?;
                updated = updated.replacen(&format!("{old_bytes}u64"), &format!("{cold_proof_bytes}u64"), 1);
            }
            if fields.len() == 2 && fields[0] == "src/Cargo.lock" {
                cargo_hits += 1;
                updated = updated.replacen(&fields[1], &cargo_lock_sha256, 1);
                let old_bytes = evidence_parse_u64(line)?;
                updated = updated.replacen(&format!("{old_bytes}u64"), &format!("{cargo_lock_bytes}u64"), 1);
            }
            if fields.len() == 2 && line.contains("core.Utf8TextMedia") && (fields[0].starts_with("state/") || fields[0].starts_with("src/")) && !["state/release_closeout.spi", "state/cold_proof.spi", "src/Cargo.lock"].contains(&fields[0].as_str()) {
                attested_hits += 1;
                let attested_path = root.join(patch_safe_relative(&fields[0])?);
                let attested_metadata = fs::symlink_metadata(&attested_path).map_err(|error| format!("metadata {}: {error}", attested_path.display()))?;
                if !attested_metadata.is_file() || attested_metadata.file_type().is_symlink() { return Err(format!("closeout evidence target must be a regular file: {}", fields[0])); }
                updated = updated.replacen(&fields[1], &inspection_file_sha256(&attested_path)?, 1);
                let old_bytes = evidence_parse_u64(line)?;
                updated = updated.replacen(&format!("{old_bytes}u64"), &format!("{}u64", attested_metadata.len()), 1);
            }
            if fields.len() == 2 && (fields[0] == "eoie" || fields[0] == "eoie.exe") {
                binary_hits += 1;
                updated = updated.replacen(&format!("({:?}", fields[0]), &format!("({eoie_name:?}"), 1);
                updated = updated.replacen(&fields[1], &eoie_sha256, 1);
                let old_bytes = evidence_parse_u64(line)?;
                updated = updated.replacen(&format!("{old_bytes}u64"), &format!("{eoie_bytes}u64"), 1);
            }
        }
        if line.contains("EvidenceTreeRef (") && line.contains("ExactTree") {
            let fields = patch_quoted_fields(line)?;
            if fields.len() == 2 && fields[0] == "src" {
                source_hits += 1;
                updated = updated.replacen(&fields[1], &source_sha256, 1);
            }
        }
        if line.contains("EvidenceTreeRef (") && line.contains("StateTreeWithoutMetaManifests") {
            let fields = patch_quoted_fields(line)?;
            if fields.len() == 2 && fields[0] == "state" {
                state_hits += 1;
                updated = updated.replacen(&fields[1], &state_sha256, 1);
            }
        }
        output.push(updated);
    }
    if release_hits == 0 && cold_hits == 0 && cargo_hits == 0 && binary_hits == 0 && source_hits == 0 && state_hits == 0 { let summary = check_evidence_tree(root)?; return if summary.declared == summary.verified { Ok(127) } else { Err(format!("closeout evidence currentness incomplete declared={} verified={}", summary.declared, summary.verified)) }; }
    if release_hits != 1 || cold_hits != 1 || cargo_hits != 1 || binary_hits != 1 || source_hits != 1 || state_hits != 1 { return Err(format!("closeout evidence anchors rejected release_hits={release_hits} cold_hits={cold_hits} cargo_hits={cargo_hits} attested_hits={attested_hits} binary_hits={binary_hits} source_hits={source_hits} state_hits={state_hits}")); }
    let next = output.join("\n") + "\n";
    atomic_write(&manifest_path, next.as_bytes())?;
    match check_evidence_tree(root) {
        Ok(summary) if summary.declared == summary.verified => Ok(127),
        Ok(summary) => { let _ = atomic_write(&manifest_path, original.as_bytes()); Err(format!("closeout evidence refresh incomplete declared={} verified={}", summary.declared, summary.verified)) },
        Err(error) => { let _ = atomic_write(&manifest_path, original.as_bytes()); Err(format!("closeout evidence refresh rolled back: {error}")) },
    }
}

fn evidence_parse_u64(line: &str) -> Result<u64, String> {
    let marker = line.find("u64").ok_or_else(|| "EvidenceRef bytes must use u64".to_owned())?;
    let prefix = &line[..marker];
    let digits_rev = prefix.chars().rev().skip_while(|c| c.is_whitespace()).take_while(|c| c.is_ascii_digit()).collect::<String>();
    if digits_rev.is_empty() { return Err("EvidenceRef bytes are missing".to_owned()); }
    digits_rev.chars().rev().collect::<String>().parse::<u64>().map_err(|_| "EvidenceRef bytes are invalid".to_owned())
}

fn evidence_parse_specs(source: &str) -> Result<Vec<EvidenceSpec>, String> {
    let mut specs = Vec::new();
    let mut seen = BTreeSet::new();
    for (number, line) in source.lines().enumerate() {
        if !line.contains("EvidenceRef (") { continue; }
        let fields = patch_quoted_fields(line)?;
        if fields.len() != 2 { return Err(format!("EvidenceRef on line {} expects path and sha256 strings", number + 1)); }
        let relative = patch_safe_relative(&fields[0])?;
        if relative == Path::new("state/evidence.spi") { return Err(format!("EvidenceRef escaped evidence root on line {}: {}", number + 1, fields[0])); }
        let relative_text = relative.to_string_lossy().replace('\\', "/");
        if !seen.insert(relative_text.clone()) { return Err(format!("duplicate EvidenceRef path: {relative_text}")); }
        let sha256 = fields[1].to_ascii_lowercase();
        if !evidence_sha256_text(&sha256) { return Err(format!("EvidenceRef sha256 is malformed on line {}", number + 1)); }
        specs.push(EvidenceSpec { relative: relative_text, sha256, bytes: evidence_parse_u64(line)? });
    }
    if specs.is_empty() { return Err("state/evidence.spi contains no EvidenceRef rows".to_owned()); }
    Ok(specs)
}

fn evidence_collect_files(bundle_root: &Path, current: &Path, out: &mut BTreeSet<String>) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|error| format!("read evidence tree {}: {error}", current.display()))? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(|error| format!("metadata {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() { return Err(format!("evidence symlink is forbidden: {}", path.display())); }
        if metadata.is_dir() { evidence_collect_files(bundle_root, &path, out)?; continue; }
        if !metadata.is_file() { return Err(format!("unsupported evidence entry: {}", path.display())); }
        let relative = path.strip_prefix(bundle_root).map_err(|error| error.to_string())?.to_string_lossy().replace('\\', "/");
        out.insert(relative);
    }
    Ok(())
}

pub fn check_evidence_tree(root: &Path) -> Result<EvidenceCurrentnessSummary, String> {
    if !root.is_dir() { return Err(format!("bundle root is not a directory: {}", root.display())); }
    let evidence_root = root.join("evidence");
    if !evidence_root.is_dir() { return Err("missing evidence root".to_owned()); }
    let manifest_path = root.join("state/evidence.spi");
    let source = fs::read_to_string(&manifest_path).map_err(|error| format!("read {}: {error}", manifest_path.display()))?;
    let specs = evidence_parse_specs(&source)?;
    let tree_specs = evidence_parse_tree_specs(&source)?;
    let declared_evidence = specs.iter().filter(|spec| spec.relative.starts_with("evidence/")).map(|spec| spec.relative.clone()).collect::<BTreeSet<_>>();
    let mut observed = BTreeSet::new();
    evidence_collect_files(root, &evidence_root, &mut observed)?;
    let missing = declared_evidence.difference(&observed).cloned().collect::<Vec<_>>();
    let unexpected = observed.difference(&declared_evidence).cloned().collect::<Vec<_>>();
    if !missing.is_empty() || !unexpected.is_empty() { return Err(format!("evidence closure mismatch missing={missing:?} unexpected={unexpected:?}")); }
    let mut verified = 0usize;
    let mut observed_specs = 0usize;
    for spec in &specs {
        let path = root.join(&spec.relative);
        let metadata = fs::symlink_metadata(&path).map_err(|error| format!("metadata {}: {error}", path.display()))?;
        let regular = metadata.is_file() && !metadata.file_type().is_symlink();
        let bytes_match = regular && metadata.len() == spec.bytes;
        let observed_sha = if regular { inspection_file_sha256(&path)? } else { String::new() };
        let sha_match = observed_sha == spec.sha256;
        let stage = eoie_evidence_row_stage(i32::from(regular), i32::from(bytes_match && sha_match));
        if stage > 0 { observed_specs += 1; }
        let in_scope = path.starts_with(root) && spec.relative != "state/evidence.spi";
        let mask = i32::from(in_scope) + 2 * i32::from(regular) + 4 * i32::from(bytes_match) + 8 * i32::from(sha_match);
        if !in_scope || stage != 2 {
            return Err(format!("evidence identity mismatch path={} expected_bytes={} observed_bytes={} expected_sha256={} observed_sha256={} mask={}", path.display(), spec.bytes, metadata.len(), spec.sha256, observed_sha, mask));
        }
        verified += 1;
    }
    let mut tree_verified = 0usize;
    for spec in &tree_specs {
        let observed_sha = match spec.policy {
            EvidenceTreePolicy::Exact => inspection_tree_sha256(root, &spec.relative)?,
            EvidenceTreePolicy::StateWithoutMetaManifests => evidence_state_tree_sha256(root)?,
        };
        if observed_sha != spec.sha256 { return Err(format!("evidence tree identity mismatch path={} expected_sha256={} observed_sha256={}", spec.relative, spec.sha256, observed_sha)); }
        tree_verified += 1;
    }
    if specs.len() != verified { return Err(format!("typed evidence file set rejected declared={} verified={}", specs.len(), verified)); }
    if tree_specs.len() != tree_verified { return Err(format!("typed evidence tree set rejected declared={} verified={}", tree_specs.len(), tree_verified)); }
    Ok(EvidenceCurrentnessSummary { declared: specs.len() + tree_specs.len(), observed: observed_specs + tree_verified, verified: verified + tree_verified })
}

fn evidence_public_binary(root: &Path) -> Result<String, String> {
    let unix = fs::symlink_metadata(root.join("eoie")).is_ok();
    let windows = fs::symlink_metadata(root.join("eoie.exe")).is_ok();
    let name = eoie_evidence_public_binary(i32::from(unix), i32::from(windows));
    if name.is_empty() { return Err("closeout evidence requires exactly one root binary: eoie or eoie.exe".to_owned()); }
    Ok(name.to_string())
}

fn coverage_receipt_field<'a>(receipt: &'a str, name: &str) -> Result<&'a str, String> {
    receipt.lines().find_map(|line| line.strip_prefix(name).and_then(|rest| rest.strip_prefix('='))).ok_or_else(|| format!("coverage receipt field missing: {name}"))
}

fn coverage_registration_input(lcov: &Path) -> Result<(Vec<u8>, Vec<u8>, String, u32, u64), String> {
    let receipt_path = std::path::PathBuf::from(format!("{}.receipt", lcov.display()));
    let payload = eoie_rust_std_fs::read_regular_limited(lcov, 256 * 1024 * 1024)?;
    let receipt = eoie_rust_std_fs::read_regular_limited(&receipt_path, 64 * 1024)?;
    let receipt_text = std::str::from_utf8(&receipt).map_err(|_| format!("coverage receipt must be UTF-8: {}", receipt_path.display()))?;
    let text = std::str::from_utf8(&payload).map_err(|_| format!("coverage LCOV must be UTF-8: {}", lcov.display()))?;
    let route = coverage_receipt_field(receipt_text, "route")?;
    if route != "lcov-union" && route != "grcov-lcov" { return Err(format!("coverage receipt route rejected: {route}")); }
    let sha256 = inspection_sha256(payload.clone());
    if coverage_receipt_field(receipt_text, "sha256")? != sha256 { return Err(format!("coverage receipt sha256 does not match {}", lcov.display())); }
    let lines = coverage_receipt_field(receipt_text, "lines")?.parse::<u64>().map_err(|_| "coverage receipt lines are malformed".to_owned())?;
    let permille = coverage_receipt_field(receipt_text, "covered_permille")?.parse::<u32>().map_err(|_| "coverage receipt permille is malformed".to_owned())?;
    if lines == 0 || lines > u64::from(u32::MAX) || permille > 1000 || !text.lines().any(|line| line.starts_with("DA:")) { return Err(format!("coverage LCOV has no bounded line evidence: {}", lcov.display())); }
    for source in text.lines().filter_map(|line| line.strip_prefix("SF:")) {
        if source.is_empty() || source.contains(':') || source.starts_with('/') || source.starts_with('\\') || source.split(['/', '\\']).any(|part| part == "..") {
            return Err(format!("coverage LCOV source path must be relative to the workspace: {source}"));
        }
    }
    Ok((payload, receipt, sha256, permille, lines))
}

pub fn register_coverage_evidence(root: &Path, current: &Path, seed: &Path) -> Result<String, String> {
    let (current_lcov, current_receipt, current_sha256, current_permille, current_lines) = coverage_registration_input(current)?;
    let (seed_lcov, seed_receipt, seed_sha256, seed_permille, seed_lines) = coverage_registration_input(seed)?;
    let evidence_path = root.join("state/evidence.spi");
    let coverage_path = root.join("state/coverage.spi");
    let evidence_original = eoie_rust_std_fs::read_regular_text_limited(&evidence_path, 1024 * 1024)?;
    let coverage_original = eoie_rust_std_fs::read_regular_text_limited(&coverage_path, 1024 * 1024)?;
    let payloads = [("evidence/coverage/current.lcov", current_lcov), ("evidence/coverage/current.lcov.receipt", current_receipt), ("evidence/coverage/replay-seed.lcov", seed_lcov), ("evidence/coverage/replay-seed.receipt", seed_receipt)];
    let mut hits = 0usize;
    let mut evidence_lines = Vec::new();
    for line in evidence_original.lines() {
        let mut updated = line.to_owned();
        if line.contains("EvidenceRef (") {
            let fields = patch_quoted_fields(line)?;
            if let Some((_, bytes)) = payloads.iter().find(|(relative, _)| fields.len() == 2 && fields[0] == *relative) {
                hits += 1;
                updated = updated.replacen(&fields[1], &inspection_sha256(bytes.clone()), 1);
                let old_bytes = evidence_parse_u64(line)?;
                updated = updated.replacen(&format!("{old_bytes}u64"), &format!("{}u64", bytes.len()), 1);
            }
        }
        evidence_lines.push(updated);
    }
    if hits != payloads.len() { return Err(format!("coverage evidence rows rejected hits={hits} expected={}", payloads.len())); }
    let durability = "inl current_checkpoint_durability () : coverage_checkpoint_durability = ";
    let seed_row = "inl current_replay_seed () : coverage_replay_seed = ";
    let (mut durable_hits, mut seed_hits) = (0usize, 0usize);
    let mut coverage_lines = Vec::new();
    for line in coverage_original.lines() {
        if line.starts_with(durability) { durable_hits += 1; coverage_lines.push(format!("{durability}CoverageCheckpointDurable ({current_permille}u32, {current_sha256:?})")); }
        else if line.starts_with(seed_row) { seed_hits += 1; coverage_lines.push(format!("{seed_row}CoverageReplaySeed ({seed_permille}u32, {seed_lines}u32, {seed_sha256:?}, \"evidence/coverage/replay-seed.lcov\")")); }
        else { coverage_lines.push(line.to_owned()); }
    }
    if durable_hits != 1 || seed_hits != 1 { return Err(format!("coverage checkpoint rows rejected durable={durable_hits} seed={seed_hits}")); }
    let mut originals: Vec<(std::path::PathBuf, Option<Vec<u8>>)> = Vec::new();
    let mut publish = || -> Result<(), String> {
        fs::create_dir_all(root.join("evidence/coverage")).map_err(|error| format!("create evidence/coverage: {error}"))?;
        for (relative, bytes) in &payloads {
            let path = root.join(patch_safe_relative(relative)?);
            originals.push((path.clone(), fs::read(&path).ok()));
            atomic_write(&path, bytes)?;
        }
        originals.push((evidence_path.clone(), Some(evidence_original.clone().into_bytes())));
        atomic_write(&evidence_path, (evidence_lines.join("\n") + "\n").as_bytes())?;
        originals.push((coverage_path.clone(), Some(coverage_original.clone().into_bytes())));
        atomic_write(&coverage_path, (coverage_lines.join("\n") + "\n").as_bytes())?;
        for (relative, bytes) in &payloads {
            if fs::read(root.join(relative)).map_err(|error| format!("readback {relative}: {error}"))? != *bytes { return Err(format!("coverage evidence readback mismatch: {relative}")); }
        }
        Ok(())
    };
    if let Err(error) = publish() {
        for (path, original) in originals.into_iter().rev() {
            match original { Some(bytes) => { let _ = atomic_write(&path, &bytes); } None => { let _ = fs::remove_file(&path); } }
        }
        return Err(format!("coverage evidence registration rolled back: {error}"));
    }
    Ok(format!("eoie proxy coverage-register ok current_sha256={current_sha256} current_permille={current_permille} current_lines={current_lines} seed_sha256={seed_sha256} seed_permille={seed_permille} seed_lines={seed_lines} mutation=true"))
}

pub fn evidence_status(root: &Path) -> Result<EvidenceCurrentnessSummary, String> { check_evidence_tree(root) }

#[cfg(test)]
#[path = "registration_tests.rs"]
mod registration_tests;

#[derive(Clone, Debug)]
pub struct ToolchainCurrentnessSummary { pub declared: usize, pub observed: usize, pub verified: usize }

fn toolchain_env_name(relative: &str) -> Option<String> { Some(eoie_rust_tool_environment(relative).to_string()).filter(|name| !name.is_empty()) }

pub fn toolchain_status(root: &Path) -> Result<Option<ToolchainCurrentnessSummary>, String> {
    let state = root.join("state/toolchain_identity.spi");
    let source = fs::read_to_string(&state).map_err(|error| format!("read {}: {error}", state.display()))?;
    let specs = evidence_parse_specs(&source)?;
    let provided = specs.iter().filter(|spec| toolchain_env_name(&spec.relative).and_then(env::var_os).is_some()).count();
    if provided == 0 { return Ok(None); }
    if provided != specs.len() { return Err(format!("partial Rust toolchain environment observed={provided} declared={}", specs.len())); }
    let mut observed = 0usize;
    let mut verified = 0usize;
    for spec in &specs {
        let env_name = toolchain_env_name(&spec.relative).ok_or_else(|| format!("unknown Rust tool identity: {}", spec.relative))?;
        let value = env::var_os(&env_name).ok_or_else(|| format!("missing Rust tool environment: {env_name}"))?;
        let path = Path::new(&value);
        if !path.is_absolute() { return Err(format!("Rust tool path must be absolute: {env_name}")); }
        let metadata = fs::symlink_metadata(path).map_err(|error| format!("metadata {}: {error}", path.display()))?;
        let regular = metadata.is_file() && !metadata.file_type().is_symlink();
        let bytes_match = regular && metadata.len() == spec.bytes;
        let sha256 = if regular { inspection_file_sha256(path)? } else { String::new() };
        let stage = eoie_evidence_row_stage(i32::from(regular), i32::from(bytes_match && sha256 == spec.sha256));
        if stage > 0 { observed += 1; }
        if stage != 2 { return Err(format!("Rust tool identity mismatch tool={} expected_bytes={} observed_bytes={} expected_sha256={} observed_sha256={}", spec.relative, spec.bytes, metadata.len(), spec.sha256, sha256)); }
        verified += 1;
    }
    Ok(Some(ToolchainCurrentnessSummary { declared: specs.len(), observed, verified }))
}

fn method1(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> bool {
    loop {
        let mut v3: bool = v2 == v1;
        if v3 {
            return true;
        } else {
            let mut v4: u64 = v0.as_bytes()[v2 as usize] as u64;
            let mut v5: bool = v4 < 48u64;
            let mut v7: bool = if v5 {
                false
            } else {
                let mut v6: bool = v4 <= 57u64;
                v6
            };
            let mut v15: bool = if v7 {
                true
            } else {
                let mut v8: bool = v4 < 65u64;
                let mut v10: bool = if v8 {
                    false
                } else {
                    let mut v9: bool = v4 <= 70u64;
                    v9
                };
                if v10 {
                    true
                } else {
                    let mut v11: bool = v4 < 97u64;
                    if v11 {
                        false
                    } else {
                        let mut v12: bool = v4 <= 102u64;
                        v12
                    }
                }
            };
            if v15 {
                let mut v16: u64 = v2.wrapping_add(1u64);
                (v0, v1, v2) = (v0.clone(), v1, v16);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn method0(mut v0: Rc<str>) -> bool {
    let mut v1: u64 = (v0.clone().len() as u64);
    let mut v2: bool = v1 < 64u64;
    let mut v4: bool = if v2 {
        false
    } else {
        let mut v3: bool = v1 <= 64u64;
        v3
    };
    if v4 {
        let mut v5: u64 = 0u64;
        method1(v0.clone(), v1, v5)
    } else {
        false
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        method0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method2(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 1i32;
    let mut v3: bool = v1 == 1i32;
    let (mut v4, mut v5): (bool, i32) = if v2 {
        (true, 1i32)
    } else {
        (false, 0i32)
    };
    let (mut v8, mut v9): (bool, i32) = if v4 {
        if v3 {
            (true, 2i32)
        } else {
            (false, v5)
        }
    } else {
        (false, v5)
    };
    v9
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustfmt"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RUSTFMT"); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    };
    let mut v5: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustdoc"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RUSTDOC"); } LIT.with(|lit| lit.clone()) };
        v6.clone()
    } else {
        v4.clone()
    };
    let mut v8: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cargo"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CARGO"); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        v7.clone()
    };
    let mut v11: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustc"); } LIT.with(|lit| lit.clone()) };
    if v11 {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RUSTC"); } LIT.with(|lit| lit.clone()) };
        v12.clone()
    } else {
        v10.clone()
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        method3(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method4(mut v0: i32, mut v1: i32) -> Rc<str> {
    let mut v2: bool = v0 == 1i32;
    let mut v4: bool = if v2 {
        let mut v3: bool = v1 == 0i32;
        v3
    } else {
        false
    };
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie"); } LIT.with(|lit| lit.clone()) };
        v5.clone()
    } else {
        let mut v6: bool = v0 == 0i32;
        let mut v8: bool = if v6 {
            let mut v7: bool = v1 == 1i32;
            v7
        } else {
            false
        };
        if v8 {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie.exe"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        } else {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
    }
}
fn closure3() -> Rc<dyn Fn(i32, i32) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: i32| -> Rc<str> {
        method4(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn evidence_sha256_text(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_evidence_row_stage(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn eoie_rust_tool_environment(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_evidence_public_binary(v0: i32, v1: i32) -> Rc<str> {
    closure3()(v0, v1)
}
