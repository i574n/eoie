#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_return, clippy::needless_late_init)]
use bundle_retention_domain::{eoie_retention_apply_binding, eoie_retention_complete_binding, eoie_retention_current_binding, eoie_retention_dry_run_binding, eoie_retention_window_binding};
use rayon::prelude::*;
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Component, Path};
#[derive(Clone, Debug, Eq, PartialEq)] struct RetentionCandidate { name: String, alpha: u64, size: u64, hash: u64 }
#[derive(Clone, Debug, Eq, PartialEq)] pub struct RetentionPlan { pub fingerprint: String, pub current_hash: String, pub candidate_count: usize, pub delete_count: usize, pub keep: Vec<String>, pub delete: Vec<String> }
#[derive(Clone, Copy, Debug, Eq, PartialEq)] pub enum RetentionFault { None, AfterFirstDelete, AfterAllDeletes }
#[derive(Clone, Debug, Eq, PartialEq)] struct RetentionReceipt { fingerprint: String, current_hash: String, candidates: usize, deletes: usize, remaining: usize, status: String }
fn retention_arg<'a>(args: &'a [String], index: usize, name: &str) -> Result<&'a str, String> { args.get(index).map(String::as_str).ok_or_else(|| format!("missing argument: {name}")) }
fn retention_filename(value: &str) -> Result<String, String> { let path = Path::new(value); let mut components = path.components(); let first = components.next(); if value.is_empty() || components.next().is_some() || !matches!(first, Some(Component::Normal(_))) { return Err(format!("retention current must be a filename: {value}")); } Ok(value.to_owned()) }
fn retention_series(current: &str) -> Result<(String, u64), String> { let current = retention_filename(current)?; let error = eoie_retention_series_error(&current); if !error.is_empty() { return Err(error.to_string()); } Ok((eoie_retention_series_prefix(&current).to_string(), eoie_retention_series_alpha(&current))) }
fn retention_fnv_update(mut hash: u64, bytes: &[u8]) -> u64 { for byte in bytes { hash ^= u64::from(*byte); hash = hash.wrapping_mul(1099511628211); } hash }
fn retention_file_hash(path: &Path) -> Result<u64, String> { let mut file = fs::File::open(path).map_err(|error| format!("open {}: {error}", path.display()))?; let mut hash = 14695981039346656037u64; let mut buffer = [0u8; 131072]; loop { let count = file.read(&mut buffer).map_err(|error| format!("read {}: {error}", path.display()))?; if count == 0 { break; } hash = retention_fnv_update(hash, &buffer[..count]); } Ok(hash) }
fn retention_inventory(directory: &Path, current: &str) -> Result<(String, Vec<RetentionCandidate>), String> { if !directory.is_dir() { return Err(format!("retention directory is not a directory: {}", directory.display())); } let current = retention_filename(current)?; let (prefix, current_alpha) = retention_series(&current)?; let mut pending = Vec::new(); for entry in fs::read_dir(directory).map_err(|error| format!("read {}: {error}", directory.display()))? { let entry = entry.map_err(|error| error.to_string())?; let name = entry.file_name().into_string().map_err(|_| "retention candidate name is not UTF-8".to_owned())?; if !name.starts_with(&prefix) || !name.ends_with(".zip") { continue; } let digits = &name[prefix.len()..name.len() - 4]; if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) { return Err(format!("malformed retention candidate: {name}")); } let alpha = digits.parse::<u64>().map_err(|error| error.to_string())?; let path = entry.path(); let metadata = fs::symlink_metadata(&path).map_err(|error| format!("metadata {}: {error}", path.display()))?; if metadata.file_type().is_symlink() || !metadata.is_file() { return Err(format!("retention candidate must be a regular file: {}", path.display())); } pending.push((name, alpha, metadata.len(), path)); } if pending.is_empty() { return Err(format!("no retention candidates for series {prefix}")); } let hashed = pending.into_par_iter().map(|(name, alpha, size, path)| retention_file_hash(&path).map(|hash| RetentionCandidate { name, alpha, size, hash })).collect::<Result<Vec<_>, _>>()?; let mut candidates = hashed; candidates.sort_by(|left, right| left.alpha.cmp(&right.alpha).then_with(|| left.name.cmp(&right.name))); if candidates.windows(2).any(|window| window[0].alpha == window[1].alpha) { return Err("duplicate retention alpha in series".to_owned()); } let latest = candidates.last().ok_or_else(|| "empty retention inventory".to_owned())?; let current_matches = candidates.iter().filter(|candidate| candidate.name == current && candidate.alpha == current_alpha).count() == 1; let current_is_latest = latest.name == current && latest.alpha == current_alpha; if eoie_retention_current_binding(i32::from(current_is_latest), i32::from(current_matches)) != 1 { return Err(format!("retention current is not the latest series candidate: current={current} latest={}", latest.name)); } Ok((prefix, candidates)) }
fn retention_build_plan(directory: &Path, current: &str) -> Result<RetentionPlan, String> { let current = retention_filename(current)?; let (_prefix, candidates) = retention_inventory(directory, &current)?; let delete_count = eoie_retention_window_binding(i32::try_from(candidates.len()).unwrap_or(i32::MAX), 3); if delete_count < 0 { return Err("typed retention window rejected candidate inventory".to_owned()); } let split = usize::try_from(delete_count).map_err(|error| error.to_string())?; let delete = candidates[..split].iter().map(|candidate| candidate.name.clone()).collect::<Vec<_>>(); let keep = candidates[split..].iter().map(|candidate| candidate.name.clone()).collect::<Vec<_>>(); if eoie_retention_dry_run_binding(delete_count, i32::try_from(delete.len()).unwrap_or(i32::MAX)) != 1 { return Err("typed retention dry-run count mismatch".to_owned()); } let current_candidate = candidates.iter().find(|candidate| candidate.name == current).ok_or_else(|| "current candidate disappeared".to_owned())?; let mut hash = 14695981039346656037u64; for candidate in &candidates { let line = format!("{}|{}|{}|{:016x}\n", candidate.name, candidate.alpha, candidate.size, candidate.hash); hash = retention_fnv_update(hash, line.as_bytes()); } for name in &keep { hash = retention_fnv_update(hash, format!("keep|{name}\n").as_bytes()); } for name in &delete { hash = retention_fnv_update(hash, format!("delete|{name}\n").as_bytes()); } Ok(RetentionPlan { fingerprint: format!("{hash:016x}"), current_hash: format!("{:016x}", current_candidate.hash), candidate_count: candidates.len(), delete_count: delete.len(), keep, delete }) }
fn retention_atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> { let parent = path.parent().ok_or_else(|| format!("path has no parent: {}", path.display()))?; fs::create_dir_all(parent).map_err(|error| format!("create {}: {error}", parent.display()))?; let name = path.file_name().and_then(|value| value.to_str()).unwrap_or("retention.spi"); let temporary = parent.join(format!(".{name}.eoie-retention-{}", std::process::id())); fs::write(&temporary, bytes).map_err(|error| format!("write {}: {error}", temporary.display()))?; fs::rename(&temporary, path).map_err(|error| format!("commit {}: {error}", path.display()))?; Ok(()) }
fn retention_receipt_source(plan: &RetentionPlan, status: &str, remaining: usize) -> String { eoie_retention_receipt_source(&plan.fingerprint, &plan.current_hash, plan.candidate_count as u64, plan.delete_count as u64, remaining as u64, status).to_string() }
fn retention_write_receipt(path: &Path, plan: &RetentionPlan, status: &str, remaining: usize) -> Result<(), String> { let previous = fs::read(path).ok(); let source = retention_receipt_source(plan, status, remaining); retention_atomic_write(path, source.as_bytes())?; let verified = retention_read_receipt(path).and_then(|receipt| { let ok = receipt.fingerprint == plan.fingerprint && receipt.current_hash == plan.current_hash && receipt.candidates == plan.candidate_count && receipt.deletes == plan.delete_count && receipt.remaining == remaining && receipt.status == status; if ok { Ok(()) } else { Err("retention receipt runtime readback mismatch".to_owned()) } }); if let Err(error) = verified { if let Some(bytes) = previous { let _ = retention_atomic_write(path, &bytes); } else { let _ = fs::remove_file(path); } return Err(error); } Ok(()) }
fn retention_read_receipt(path: &Path) -> Result<RetentionReceipt, String> { let text = fs::read_to_string(path).map_err(|error| format!("read {}: {error}", path.display()))?; let line = text.lines().find(|line| line.starts_with("// retention-receipt|")).ok_or_else(|| "retention receipt marker missing".to_owned())?; let mut schema = None; let mut fingerprint = None; let mut current_hash = None; let mut candidates = None; let mut deletes = None; let mut remaining = None; let mut status = None; for field in line.split('|').skip(1) { let (key, value) = field.split_once('=').ok_or_else(|| format!("invalid retention receipt field: {field}"))?; match key { "schema" => schema = Some(value.to_owned()), "fingerprint" => fingerprint = Some(value.to_owned()), "current" => current_hash = Some(value.to_owned()), "candidates" => candidates = Some(value.parse::<usize>().map_err(|error| error.to_string())?), "deletes" => deletes = Some(value.parse::<usize>().map_err(|error| error.to_string())?), "remaining" => remaining = Some(value.parse::<usize>().map_err(|error| error.to_string())?), "status" => status = Some(value.to_owned()), _ => return Err(format!("unknown retention receipt field: {key}")), } } if schema.as_deref() != Some("1") { return Err("unsupported retention receipt schema".to_owned()); } let receipt = RetentionReceipt { fingerprint: fingerprint.ok_or_else(|| "missing receipt fingerprint".to_owned())?, current_hash: current_hash.ok_or_else(|| "missing receipt current hash".to_owned())?, candidates: candidates.ok_or_else(|| "missing receipt candidate count".to_owned())?, deletes: deletes.ok_or_else(|| "missing receipt delete count".to_owned())?, remaining: remaining.ok_or_else(|| "missing receipt remaining count".to_owned())?, status: status.ok_or_else(|| "missing receipt status".to_owned())? }; if receipt.status != "planned" && receipt.status != "applied" { return Err(format!("invalid retention receipt status: {}", receipt.status)); } Ok(receipt) }
fn retention_series_names(directory: &Path, prefix: &str) -> Result<BTreeSet<String>, String> { let mut names = BTreeSet::new(); for entry in fs::read_dir(directory).map_err(|error| format!("read {}: {error}", directory.display()))? { let entry = entry.map_err(|error| error.to_string())?; let name = entry.file_name().into_string().map_err(|_| "retention candidate name is not UTF-8".to_owned())?; if !name.starts_with(prefix) || !name.ends_with(".zip") { continue; } let digits = &name[prefix.len()..name.len() - 4]; if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) { return Err(format!("malformed retention candidate: {name}")); } let metadata = fs::symlink_metadata(entry.path()).map_err(|error| error.to_string())?; if metadata.file_type().is_symlink() || !metadata.is_file() { return Err(format!("retention candidate must be regular: {name}")); } names.insert(name); } Ok(names) }
fn retention_stage_copy(source: &Path, staged: &Path) -> Result<(), String> { if let Some(parent) = staged.parent() { fs::create_dir_all(parent).map_err(|error| format!("create {}: {error}", parent.display()))?; } if let Err(link_error) = fs::hard_link(source, staged) { fs::copy(source, staged).map_err(|copy_error| format!("stage {}: hardlink={link_error}; copy={copy_error}", source.display()))?; } if retention_file_hash(source)? != retention_file_hash(staged)? { return Err(format!("staged retention hash mismatch: {}", source.display())); } Ok(()) }
fn retention_restore(directory: &Path, stage: &Path, names: &[String]) -> Result<(), String> { let mut errors = Vec::new(); for name in names { let source = directory.join(name); let staged = stage.join(name); if !source.exists() && staged.is_file() { if let Err(error) = fs::hard_link(&staged, &source).or_else(|_| fs::copy(&staged, &source).map(|_| ())) { errors.push(format!("restore {name}: {error}")); } } } let _ = fs::remove_dir_all(stage); if errors.is_empty() { Ok(()) } else { Err(errors.join("; ")) } }
pub fn plan_retention(directory: &Path, current: &str, receipt: &Path) -> Result<RetentionPlan, String> { let plan = retention_build_plan(directory, current)?; retention_write_receipt(receipt, &plan, "planned", plan.candidate_count)?; for name in &plan.keep { println!("retention keep {name}"); } for name in &plan.delete { println!("retention delete {name}"); } Ok(plan) }
pub fn apply_retention_with_fault(directory: &Path, current: &str, receipt_path: &Path, token: &str, fault: RetentionFault) -> Result<RetentionPlan, String> { let receipt = retention_read_receipt(receipt_path)?; if receipt.status != "planned" { return Err("retention apply requires a planned receipt".to_owned()); } let plan = retention_build_plan(directory, current)?; if receipt.fingerprint != plan.fingerprint || receipt.current_hash != plan.current_hash || receipt.candidates != plan.candidate_count || receipt.deletes != plan.delete_count || receipt.remaining != plan.candidate_count { return Err("retention inventory changed after dry-run".to_owned()); } let explicit_apply = i32::from(token == plan.fingerprint); if eoie_retention_apply_binding(1, explicit_apply) != 1 { return Err(format!("retention apply token mismatch; expected {}", plan.fingerprint)); } let (prefix, _) = retention_series(current)?; let stage = directory.join(format!(".eoie-retention-stage-{}", plan.fingerprint)); if stage.exists() { return Err(format!("retention stage already exists: {}", stage.display())); } fs::create_dir(&stage).map_err(|error| format!("create stage {}: {error}", stage.display()))?; for name in &plan.delete { if let Err(error) = retention_stage_copy(&directory.join(name), &stage.join(name)) { let _ = fs::remove_dir_all(&stage); return Err(error); } } let mut removed = 0usize; for name in &plan.delete { if let Err(error) = fs::remove_file(directory.join(name)) { let rollback = retention_restore(directory, &stage, &plan.delete); return Err(format!("remove {name}: {error}; rollback={rollback:?}")); } removed += 1; if removed == 1 && fault == RetentionFault::AfterFirstDelete { let rollback = retention_restore(directory, &stage, &plan.delete); return Err(format!("injected retention fault after first delete; rollback={rollback:?}")); } } if fault == RetentionFault::AfterAllDeletes { let rollback = retention_restore(directory, &stage, &plan.delete); return Err(format!("injected retention fault after all deletes; rollback={rollback:?}")); } let observed = retention_series_names(directory, &prefix)?; let expected = plan.keep.iter().cloned().collect::<BTreeSet<_>>(); let expected_remaining = plan.candidate_count.min(3); if observed != expected || eoie_retention_complete_binding(i32::try_from(expected_remaining).unwrap_or(i32::MAX), i32::try_from(observed.len()).unwrap_or(i32::MAX)) != 1 { let rollback = retention_restore(directory, &stage, &plan.delete); return Err(format!("retention visible window mismatch; rollback={rollback:?}")); } if let Err(error) = retention_write_receipt(receipt_path, &plan, "applied", observed.len()) { let rollback = retention_restore(directory, &stage, &plan.delete); return Err(format!("write applied receipt: {error}; rollback={rollback:?}")); } if let Err(error) = fs::remove_dir_all(&stage) { eprintln!("eoie retention warning: committed copies remain in {}: {error}", stage.display()); } Ok(plan) }
pub fn apply_retention(directory: &Path, current: &str, receipt_path: &Path, token: &str) -> Result<RetentionPlan, String> { apply_retention_with_fault(directory, current, receipt_path, token, RetentionFault::None) }
pub fn retention_plan_command(args: &[String]) -> Result<(), String> { if args.len() != 4 { return Err("retention-plan expects directory current.zip receipt.spi".to_owned()); } let plan = plan_retention(Path::new(retention_arg(args, 1, "directory")?), retention_arg(args, 2, "current.zip")?, Path::new(retention_arg(args, 3, "receipt.spi")?))?; println!("eoie bundle retention-plan ok candidates={} keep={} delete={} fingerprint={}", plan.candidate_count, plan.keep.len(), plan.delete_count, plan.fingerprint); Ok(()) }
pub fn retention_apply_command(args: &[String]) -> Result<(), String> { if args.len() != 5 { return Err("retention-apply expects directory current.zip receipt.spi fingerprint".to_owned()); } let plan = apply_retention(Path::new(retention_arg(args, 1, "directory")?), retention_arg(args, 2, "current.zip")?, Path::new(retention_arg(args, 3, "receipt.spi")?), retention_arg(args, 4, "fingerprint")?)?; println!("eoie bundle retention-apply ok removed={} remaining={} fingerprint={}", plan.delete_count, plan.keep.len(), plan.fingerprint); Ok(()) }
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_MissingZipSuffix,
    US1_MissingAlphaSuffix,
    US1_InvalidAlphaSuffix,
    US1_AlphaOutOfRange(Rc<str>),
}
#[derive(Clone)]
enum US0 {
    US0_SeriesMember(Rc<str>, u64),
    US0_SeriesRejected(US1),
}
#[derive(Clone)]
enum US2 {
    US2_PlannedReceipt,
    US2_AppliedReceipt,
}
fn read_series_1(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".zip"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.ends_with(&*v1);
    if v2 {
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.strip_suffix(&*v1).unwrap_or(&*v0));
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alpha"); } LIT.with(|lit| lit.clone()) };
        let mut v5: i64 = v3.rfind(&*v4).map(|index| index as i64).unwrap_or(-1);
        let mut v6: bool = v5 < 0i64;
        if v6 {
            let mut v7: US1 = US1::US1_MissingAlphaSuffix;
            US0::US0_SeriesRejected(v7.clone())
        } else {
            let mut v9: i64 = v5.wrapping_add(5i64);
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(&(&*v3)[v9 as usize..]);
            let mut v11: bool = !v10.is_empty() && v10.bytes().all(|byte| byte.is_ascii_digit());
            if v11 {
                let mut v12: Rc<str> = std::rc::Rc::<str>::from(v10.parse::<u64>().err().map(|error| error.to_string()).unwrap_or_default());
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v14: bool = v12 == v13 ;
                if v14 {
                    let mut v15: Rc<str> = std::rc::Rc::<str>::from(&(&*v3)[..v9 as usize]);
                    let mut v16: u64 = v10.parse::<u64>().unwrap_or(0);
                    US0::US0_SeriesMember(v15.clone(), v16)
                } else {
                    let mut v18: US1 = US1::US1_AlphaOutOfRange(v12.clone());
                    US0::US0_SeriesRejected(v18.clone())
                }
            } else {
                let mut v21: US1 = US1::US1_InvalidAlphaSuffix;
                US0::US0_SeriesRejected(v21.clone())
            }
        }
    } else {
        let mut v25: US1 = US1::US1_MissingZipSuffix;
        US0::US0_SeriesRejected(v25.clone())
    }
}
fn series_error_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = read_series_1(v0.clone());
    match &v1 {
        US0::US0_SeriesMember(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: u64 = *v3;
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_SeriesRejected(v5) => {
            let mut v5: US1 = v5.clone();
            match &v5 {
                US1::US1_AlphaOutOfRange(v15) => {
                    let mut v15: Rc<str> = v15.clone();
                    v15.clone()
                }
                US1::US1_InvalidAlphaSuffix => {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("retention current has invalid alpha suffix: "); } LIT.with(|lit| lit.clone()) };
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v0, &*v13, &*v13, &*v13].concat());
                    v14.clone()
                }
                US1::US1_MissingAlphaSuffix => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("retention current has no alpha suffix: "); } LIT.with(|lit| lit.clone()) };
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v0, &*v10, &*v10, &*v10].concat());
                    v11.clone()
                }
                US1::US1_MissingZipSuffix => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("retention current must end in .zip: "); } LIT.with(|lit| lit.clone()) };
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v0, &*v7, &*v7, &*v7].concat());
                    v8.clone()
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        series_error_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn series_prefix_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = read_series_1(v0.clone());
    match &v1 {
        US0::US0_SeriesMember(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: u64 = *v3;
            v2.clone()
        }
        US0::US0_SeriesRejected(v4) => {
            let mut v4: US1 = v4.clone();
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        series_prefix_2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn series_alpha_3(mut v0: Rc<str>) -> u64 {
    let mut v1: US0 = read_series_1(v0.clone());
    match &v1 {
        US0::US0_SeriesMember(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: u64 = *v3;
            v3
        }
        US0::US0_SeriesRejected(v4) => {
            let mut v4: US1 = v4.clone();
            0u64
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        series_alpha_3(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn receipt_source_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: Rc<str>) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// retention-receipt|schema=1|fingerprint="); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|current="); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|candidates="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v0, &*v7, &*v1, &*v8].concat());
    let mut v10: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|deletes="); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|remaining="); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|status="); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v5, &*v16].concat());
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v12, &*v17].concat());
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union retention_receipt_status =\n    | RetentionDryRun\n    | RetentionApplied\n\nunion retention_receipt =\n    | RetentionReceipt :: string * string * i32 * i32 * i32 * retention_receipt_status -> retention_receipt\n\ninl latest_retention_receipt () : retention_receipt =\n    RetentionReceipt (\""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planned"); } LIT.with(|lit| lit.clone()) };
    let mut v22: bool = v5 == v21 ;
    let mut v25: US2 = if v22 {
        US2::US2_PlannedReceipt
    } else {
        US2::US2_AppliedReceipt
    };
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32, "); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v27, &*v28, &*v29, &*v28].concat());
    let mut v31: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v35: Rc<str> = match &v25 {
        US2::US2_AppliedReceipt => {
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RetentionApplied"); } LIT.with(|lit| lit.clone()) };
            v33.clone()
        }
        US2::US2_PlannedReceipt => {
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RetentionDryRun"); } LIT.with(|lit| lit.clone()) };
            v32.clone()
        }
    };
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v28, &*v35, &*v36].concat());
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v37, &*v38, &*v38, &*v38].concat());
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v0, &*v20, &*v39].concat());
    v40.clone()
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>, u64, u64, u64, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, u64, u64, u64, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: Rc<str>| -> Rc<str> {
        receipt_source_4(v0.clone(), v1.clone(), v2, v3, v4, v5.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_retention_series_error(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_retention_series_prefix(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
pub fn eoie_retention_series_alpha(v0: &str) -> u64 {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_retention_receipt_source(v0: &str, v1: &str, v2: u64, v3: u64, v4: u64, v5: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, v3, v4, Rc::<str>::from(v5))
}
