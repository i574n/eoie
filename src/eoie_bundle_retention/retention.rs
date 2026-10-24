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
fn retention_read_receipt(path: &Path) -> Result<RetentionReceipt, String> { let text = fs::read_to_string(path).map_err(|error| format!("read {}: {error}", path.display()))?; let error = eoie_retention_receipt_error(&text); if !error.is_empty() { return Err(error.to_string()); } let row = eoie_retention_receipt_row(&text); let parts = row.split(char::from(9u8)).collect::<Vec<_>>(); let field = |index: usize| parts.get(index).copied().unwrap_or_default(); Ok(RetentionReceipt { fingerprint: field(0).to_owned(), current_hash: field(1).to_owned(), candidates: field(2).parse().unwrap_or(0), deletes: field(3).parse().unwrap_or(0), remaining: field(4).parse().unwrap_or(0), status: field(5).to_owned() }) }
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
    US2_ReceiptRead(Rc<str>),
    US2_ReceiptRejected(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Absent,
    US3_Present(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_FieldTaken(US3, US3, US3, US3, US3, US3, US3),
    US4_FieldRejected(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_PlannedReceipt,
    US5_AppliedReceipt,
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
                let mut v13: bool = v12.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v13 {
                    let mut v14: Rc<str> = std::rc::Rc::<str>::from(&(&*v3)[..v9 as usize]);
                    let mut v15: u64 = v10.parse::<u64>().unwrap_or(0);
                    US0::US0_SeriesMember(v14.clone(), v15)
                } else {
                    let mut v17: US1 = US1::US1_AlphaOutOfRange(v12.clone());
                    US0::US0_SeriesRejected(v17.clone())
                }
            } else {
                let mut v20: US1 = US1::US1_InvalidAlphaSuffix;
                US0::US0_SeriesRejected(v20.clone())
            }
        }
    } else {
        let mut v24: US1 = US1::US1_MissingZipSuffix;
        US0::US0_SeriesRejected(v24.clone())
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
                US1::US1_AlphaOutOfRange(v12) => {
                    let mut v12: Rc<str> = v12.clone();
                    v12.clone()
                }
                US1::US1_InvalidAlphaSuffix => {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("retention current has invalid alpha suffix: "); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v0));
                    v11.clone()
                }
                US1::US1_MissingAlphaSuffix => {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("retention current has no alpha suffix: "); } LIT.with(|lit| lit.clone()) };
                    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v0));
                    v9.clone()
                }
                US1::US1_MissingZipSuffix => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("retention current must end in .zip: "); } LIT.with(|lit| lit.clone()) };
                    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v0));
                    v7.clone()
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
fn field_applied_7(mut v0: US3, mut v1: US3, mut v2: US3, mut v3: US3, mut v4: US3, mut v5: US3, mut v6: US3, mut v7: Rc<str>) -> US4 {
    let mut v8: bool = v7.contains(char::from(61u8));
    if v8 {
        let mut v9: Rc<str> = std::rc::Rc::<str>::from(v7.split_once(char::from(61u8)).map(|pair| pair.0).unwrap_or(""));
        let mut v10: Rc<str> = std::rc::Rc::<str>::from(v7.split_once(char::from(61u8)).map(|pair| pair.1).unwrap_or(""));
        let mut v11: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("schema"); } LIT.with(|lit| lit.clone()) };
        if v11 {
            let mut v12: US3 = US3::US3_Present(v10.clone());
            US4::US4_FieldTaken(v12.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
        } else {
            let mut v14: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fingerprint"); } LIT.with(|lit| lit.clone()) };
            if v14 {
                let mut v15: US3 = US3::US3_Present(v10.clone());
                US4::US4_FieldTaken(v0.clone(), v15.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
            } else {
                let mut v17: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("current"); } LIT.with(|lit| lit.clone()) };
                if v17 {
                    let mut v18: US3 = US3::US3_Present(v10.clone());
                    US4::US4_FieldTaken(v0.clone(), v1.clone(), v18.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
                } else {
                    let mut v20: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("candidates"); } LIT.with(|lit| lit.clone()) };
                    let mut v22: bool = if v20 {
                        true
                    } else {
                        let mut v21: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deletes"); } LIT.with(|lit| lit.clone()) };
                        v21
                    };
                    let mut v24: bool = if v22 {
                        true
                    } else {
                        let mut v23: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("remaining"); } LIT.with(|lit| lit.clone()) };
                        v23
                    };
                    if v24 {
                        let mut v25: Rc<str> = std::rc::Rc::<str>::from(v10.parse::<usize>().err().map(|error| error.to_string()).unwrap_or_default());
                        let mut v26: bool = v25.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        if v26 {
                            US4::US4_FieldRejected(v25.clone())
                        } else {
                            if v20 {
                                let mut v28: Rc<str> = std::rc::Rc::<str>::from(v10.parse::<usize>().map(|count| count.to_string()).unwrap_or_default());
                                let mut v29: US3 = US3::US3_Present(v28.clone());
                                US4::US4_FieldTaken(v0.clone(), v1.clone(), v2.clone(), v29.clone(), v4.clone(), v5.clone(), v6.clone())
                            } else {
                                let mut v31: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deletes"); } LIT.with(|lit| lit.clone()) };
                                if v31 {
                                    let mut v32: Rc<str> = std::rc::Rc::<str>::from(v10.parse::<usize>().map(|count| count.to_string()).unwrap_or_default());
                                    let mut v33: US3 = US3::US3_Present(v32.clone());
                                    US4::US4_FieldTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v33.clone(), v5.clone(), v6.clone())
                                } else {
                                    let mut v35: Rc<str> = std::rc::Rc::<str>::from(v10.parse::<usize>().map(|count| count.to_string()).unwrap_or_default());
                                    let mut v36: US3 = US3::US3_Present(v35.clone());
                                    US4::US4_FieldTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v36.clone(), v6.clone())
                                }
                            }
                        }
                    } else {
                        let mut v41: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                        if v41 {
                            let mut v42: US3 = US3::US3_Present(v10.clone());
                            US4::US4_FieldTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v42.clone())
                        } else {
                            let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown retention receipt field: "); } LIT.with(|lit| lit.clone()) };
                            let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v9));
                            US4::US4_FieldRejected(v45.clone())
                        }
                    }
                }
            }
        }
    } else {
        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid retention receipt field: "); } LIT.with(|lit| lit.clone()) };
        let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v7));
        US4::US4_FieldRejected(v53.clone())
    }
}
fn fields_from_6(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: US3, mut v4: US3, mut v5: US3, mut v6: US3, mut v7: US3, mut v8: US3, mut v9: US3) -> US4 {
    loop {
        let mut v10: bool = v1 < v2;
        if v10 {
            let mut v11: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(124u8)).nth(v1 as usize).unwrap_or(""));
            let mut v12: US4 = field_applied_7(v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v11.clone());
            match &v12 {
                US4::US4_FieldRejected(v22) => {
                    let mut v22: Rc<str> = v22.clone();
                    return US4::US4_FieldRejected(v22.clone());
                }
                US4::US4_FieldTaken(v13, v14, v15, v16, v17, v18, v19) => {
                    let mut v13: US3 = v13.clone();
                    let mut v14: US3 = v14.clone();
                    let mut v15: US3 = v15.clone();
                    let mut v16: US3 = v16.clone();
                    let mut v17: US3 = v17.clone();
                    let mut v18: US3 = v18.clone();
                    let mut v19: US3 = v19.clone();
                    let mut v20: u64 = v1.wrapping_add(1u64);
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v0.clone(), v20, v2, v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone());
                    continue;
                }
            }
        } else {
            return US4::US4_FieldTaken(v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone());
        }
    }
}
fn receipt_reading_of_5(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().find(|line| line.starts_with("// retention-receipt|")).unwrap_or(""));
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("retention receipt marker missing"); } LIT.with(|lit| lit.clone()) };
        US2::US2_ReceiptRejected(v3.clone())
    } else {
        let mut v5: u64 = 1u64;
        let mut v6: u64 = v1.split(char::from(124u8)).count() as u64;
        let mut v7: US3 = US3::US3_Absent;
        let mut v8: US3 = US3::US3_Absent;
        let mut v9: US3 = US3::US3_Absent;
        let mut v10: US3 = US3::US3_Absent;
        let mut v11: US3 = US3::US3_Absent;
        let mut v12: US3 = US3::US3_Absent;
        let mut v13: US3 = US3::US3_Absent;
        let mut v14: US4 = fields_from_6(v1.clone(), v5, v6, v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone());
        match &v14 {
            US4::US4_FieldRejected(v15) => {
                let mut v15: Rc<str> = v15.clone();
                US2::US2_ReceiptRejected(v15.clone())
            }
            US4::US4_FieldTaken(v17, v18, v19, v20, v21, v22, v23) => {
                let mut v17: US3 = v17.clone();
                let mut v18: US3 = v18.clone();
                let mut v19: US3 = v19.clone();
                let mut v20: US3 = v20.clone();
                let mut v21: US3 = v21.clone();
                let mut v22: US3 = v22.clone();
                let mut v23: US3 = v23.clone();
                let mut v27: bool = match &v17 {
                    US3::US3_Absent => {
                        false
                    }
                    US3::US3_Present(v24) => {
                        let mut v24: Rc<str> = v24.clone();
                        let mut v25: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                        v25
                    }
                };
                let mut v28: bool = v27 == false;
                if v28 {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unsupported retention receipt schema"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ReceiptRejected(v29.clone())
                } else {
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing receipt fingerprint"); } LIT.with(|lit| lit.clone()) };
                    let mut v36: US2 = match &v18 {
                        US3::US3_Absent => {
                            US2::US2_ReceiptRejected(v31.clone())
                        }
                        US3::US3_Present(v32) => {
                            let mut v32: Rc<str> = v32.clone();
                            US2::US2_ReceiptRead(v32.clone())
                        }
                    };
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing receipt current hash"); } LIT.with(|lit| lit.clone()) };
                    let mut v42: US2 = match &v19 {
                        US3::US3_Absent => {
                            US2::US2_ReceiptRejected(v37.clone())
                        }
                        US3::US3_Present(v38) => {
                            let mut v38: Rc<str> = v38.clone();
                            US2::US2_ReceiptRead(v38.clone())
                        }
                    };
                    match &v36 {
                        US2::US2_ReceiptRejected(v43) => {
                            let mut v43: Rc<str> = v43.clone();
                            US2::US2_ReceiptRejected(v43.clone())
                        }
                        _ => {
                            match &v42 {
                                US2::US2_ReceiptRejected(v45) => {
                                    let mut v45: Rc<str> = v45.clone();
                                    US2::US2_ReceiptRejected(v45.clone())
                                }
                                _ => {
                                    match &v36 {
                                        US2::US2_ReceiptRead(v47) => {
                                            let mut v47: Rc<str> = v47.clone();
                                            match &v42 {
                                                US2::US2_ReceiptRead(v48) => {
                                                    let mut v48: Rc<str> = v48.clone();
                                                    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing receipt candidate count"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v54: US2 = match &v20 {
                                                        US3::US3_Absent => {
                                                            US2::US2_ReceiptRejected(v49.clone())
                                                        }
                                                        US3::US3_Present(v50) => {
                                                            let mut v50: Rc<str> = v50.clone();
                                                            US2::US2_ReceiptRead(v50.clone())
                                                        }
                                                    };
                                                    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing receipt delete count"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v60: US2 = match &v21 {
                                                        US3::US3_Absent => {
                                                            US2::US2_ReceiptRejected(v55.clone())
                                                        }
                                                        US3::US3_Present(v56) => {
                                                            let mut v56: Rc<str> = v56.clone();
                                                            US2::US2_ReceiptRead(v56.clone())
                                                        }
                                                    };
                                                    match &v54 {
                                                        US2::US2_ReceiptRejected(v61) => {
                                                            let mut v61: Rc<str> = v61.clone();
                                                            US2::US2_ReceiptRejected(v61.clone())
                                                        }
                                                        _ => {
                                                            match &v60 {
                                                                US2::US2_ReceiptRejected(v63) => {
                                                                    let mut v63: Rc<str> = v63.clone();
                                                                    US2::US2_ReceiptRejected(v63.clone())
                                                                }
                                                                _ => {
                                                                    match &v54 {
                                                                        US2::US2_ReceiptRead(v65) => {
                                                                            let mut v65: Rc<str> = v65.clone();
                                                                            match &v60 {
                                                                                US2::US2_ReceiptRead(v66) => {
                                                                                    let mut v66: Rc<str> = v66.clone();
                                                                                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing receipt remaining count"); } LIT.with(|lit| lit.clone()) };
                                                                                    let mut v72: US2 = match &v22 {
                                                                                        US3::US3_Absent => {
                                                                                            US2::US2_ReceiptRejected(v67.clone())
                                                                                        }
                                                                                        US3::US3_Present(v68) => {
                                                                                            let mut v68: Rc<str> = v68.clone();
                                                                                            US2::US2_ReceiptRead(v68.clone())
                                                                                        }
                                                                                    };
                                                                                    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing receipt status"); } LIT.with(|lit| lit.clone()) };
                                                                                    let mut v78: US2 = match &v23 {
                                                                                        US3::US3_Absent => {
                                                                                            US2::US2_ReceiptRejected(v73.clone())
                                                                                        }
                                                                                        US3::US3_Present(v74) => {
                                                                                            let mut v74: Rc<str> = v74.clone();
                                                                                            US2::US2_ReceiptRead(v74.clone())
                                                                                        }
                                                                                    };
                                                                                    match &v72 {
                                                                                        US2::US2_ReceiptRejected(v79) => {
                                                                                            let mut v79: Rc<str> = v79.clone();
                                                                                            US2::US2_ReceiptRejected(v79.clone())
                                                                                        }
                                                                                        _ => {
                                                                                            match &v78 {
                                                                                                US2::US2_ReceiptRejected(v81) => {
                                                                                                    let mut v81: Rc<str> = v81.clone();
                                                                                                    US2::US2_ReceiptRejected(v81.clone())
                                                                                                }
                                                                                                _ => {
                                                                                                    match &v72 {
                                                                                                        US2::US2_ReceiptRead(v83) => {
                                                                                                            let mut v83: Rc<str> = v83.clone();
                                                                                                            match &v78 {
                                                                                                                US2::US2_ReceiptRead(v84) => {
                                                                                                                    let mut v84: Rc<str> = v84.clone();
                                                                                                                    let mut v85: bool = v84.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planned"); } LIT.with(|lit| lit.clone()) };
                                                                                                                    let mut v87: bool = if v85 {
                                                                                                                        true
                                                                                                                    } else {
                                                                                                                        let mut v86: bool = v84.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("applied"); } LIT.with(|lit| lit.clone()) };
                                                                                                                        v86
                                                                                                                    };
                                                                                                                    if v87 {
                                                                                                                        let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                                                                                                        let mut v89: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v88));
                                                                                                                        let mut v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v48));
                                                                                                                        let mut v91: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v88));
                                                                                                                        let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v65));
                                                                                                                        let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v88));
                                                                                                                        let mut v94: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v88));
                                                                                                                        let mut v95: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v83));
                                                                                                                        let mut v96: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v88));
                                                                                                                        let mut v97: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v84));
                                                                                                                        let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v97));
                                                                                                                        US2::US2_ReceiptRead(v98.clone())
                                                                                                                    } else {
                                                                                                                        let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid retention receipt status: "); } LIT.with(|lit| lit.clone()) };
                                                                                                                        let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v84));
                                                                                                                        US2::US2_ReceiptRejected(v101.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                                _ => unreachable!(),
                                                                                                            }
                                                                                                        }
                                                                                                        _ => unreachable!(),
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                _ => unreachable!(),
                                                                            }
                                                                        }
                                                                        _ => unreachable!(),
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => unreachable!(),
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
fn receipt_error_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US2 = receipt_reading_of_5(v0.clone());
    match &v1 {
        US2::US2_ReceiptRead(v2) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US2::US2_ReceiptRejected(v4) => {
            let mut v4: Rc<str> = v4.clone();
            v4.clone()
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        receipt_error_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn receipt_row_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US2 = receipt_reading_of_5(v0.clone());
    match &v1 {
        US2::US2_ReceiptRead(v2) => {
            let mut v2: Rc<str> = v2.clone();
            v2.clone()
        }
        US2::US2_ReceiptRejected(v3) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        receipt_row_8(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn receipt_source_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: Rc<str>) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("// retention-receipt|schema=1|fingerprint="); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v0));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|current="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v1));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|candidates="); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let mut v13: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|deletes="); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    let mut v17: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let mut v19: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|remaining="); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|status="); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v5));
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v26));
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union retention_receipt_status =\n    | RetentionDryRun\n    | RetentionApplied\n\nunion retention_receipt =\n    | RetentionReceipt :: string * string * i32 * i32 * i32 * retention_receipt_status -> retention_receipt\n\ninl latest_retention_receipt () : retention_receipt =\n    RetentionReceipt (\""); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
    let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v0));
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
    let mut v33: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planned"); } LIT.with(|lit| lit.clone()) };
    let mut v36: US5 = if v33 {
        US5::US5_PlannedReceipt
    } else {
        US5::US5_AppliedReceipt
    };
    let mut v37: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v37));
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32, "); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
    let mut v42: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
    let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v40));
    let mut v45: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
    let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v40));
    let mut v51: Rc<str> = match &v36 {
        US5::US5_AppliedReceipt => {
            let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RetentionApplied"); } LIT.with(|lit| lit.clone()) };
            v49.clone()
        }
        US5::US5_PlannedReceipt => {
            let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RetentionDryRun"); } LIT.with(|lit| lit.clone()) };
            v48.clone()
        }
    };
    let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v51));
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v53));
    let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v54));
    let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v55));
    v56.clone()
}
fn closure5() -> Rc<dyn Fn(Rc<str>, Rc<str>, u64, u64, u64, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, u64, u64, u64, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: Rc<str>| -> Rc<str> {
        receipt_source_9(v0.clone(), v1.clone(), v2, v3, v4, v5.clone())
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
pub fn eoie_retention_receipt_error(v0: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0))
}
pub fn eoie_retention_receipt_row(v0: &str) -> Rc<str> {
    closure4()(Rc::<str>::from(v0))
}
pub fn eoie_retention_receipt_source(v0: &str, v1: &str, v2: u64, v3: u64, v4: u64, v5: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, v3, v4, Rc::<str>::from(v5))
}
