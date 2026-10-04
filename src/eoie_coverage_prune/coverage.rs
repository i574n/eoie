#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use eoie_process::{typecheck_spiral_file as coverage_typecheck_spiral_file, resolve_spiral_compiler as coverage_resolve_spiral_compiler};
use eoie_process_observation::{run_bounded_receipted_observed as coverage_run_bounded_receipted_observed, run_bounded_streaming_checked as coverage_run_bounded_streaming_checked};
use eoie_coverage_smoke::{coverage_smoke_preflight, coverage_smoke_run, CoverageSmokeContext};
use eoie_proxy_search::inspection_file_sha256;
use eoie_rust_std_fs::{atomic_write as coverage_atomic_write, read_regular_text_limited as coverage_read_regular_text_limited, rooted_path as coverage_rooted};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn coverage_arg<'a>(args: &'a [String], index: usize, name: &str) -> Result<&'a str, String> {
    args.get(index).map(String::as_str).ok_or_else(|| format!("missing argument: {name}"))
}

fn coverage_typecheck(path: &Path) -> Result<(), String> {
    let compiler = coverage_resolve_spiral_compiler(None)?.ok_or_else(|| "Spiral compiler is required for prune-uncovered; configure EOIE_SPIRAL_COMPILE, EOIE_CONFIG_HOME/XDG/HOME, or PATH".to_owned())?;
    coverage_typecheck_spiral_file(&compiler, path)
}

fn coverage_lcov_state(lcov: &Path, root: &Path, source: &Path) -> Result<Option<bool>, String> {
    let canonical = fs::canonicalize(source).map_err(|error| format!("canonicalize {}: {error}", source.display()))?;
    let mut found = false;
    let mut covered = false;
    for (record, lines) in coverage_union_records(lcov)? {
        let evidence = Path::new(&record);
        let evidence = if evidence.is_absolute() { evidence.to_path_buf() } else { root.join(evidence) };
        if !lines.is_empty() && fs::canonicalize(&evidence).is_ok_and(|path| path == canonical) {
            found = true;
            covered |= lines.values().any(|hits| *hits > 0);
        }
    }
    Ok(if found { Some(covered) } else { None })
}

fn coverage_escape_spi(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\"', "\\\"").replace('\n', "\\n").replace('\r', "\\r")
}

fn coverage_lcov_lines(path: &Path) -> Result<BTreeMap<String, u64>, String> {
    let mut origins: BTreeMap<String, String> = BTreeMap::new();
    let mut flat: BTreeMap<String, u64> = BTreeMap::new();
    for (source, lines) in coverage_union_records(path)? {
        let relative = source.rsplit_once("/src/").map_or_else(|| source.strip_prefix("src/").unwrap_or(&source).to_owned(), |(_, tail)| tail.to_owned());
        if let Some(previous) = origins.insert(relative.clone(), source.clone()) {
            if previous != source { return Err(format!("coverage has ambiguous source identity: {relative}")); }
        }
        for (number, hits) in lines { flat.insert(format!("{relative}:{number}"), hits); }
    }
    Ok(flat)
}

fn coverage_ratio(lines: &BTreeMap<String, u64>) -> u32 {
    if lines.is_empty() { 0 } else { (lines.values().filter(|hits| **hits > 0).count() * 1000 / lines.len()) as u32 }
}

pub fn coverage_assess(args: &[String]) -> Result<(), String> {
    if args.len() != 6 { return Err("coverage-assess expects test.lcov smoke.lcov test-floor smoke-floor combined-floor".to_owned()); }
    let test = coverage_lcov_lines(Path::new(coverage_arg(args, 1, "test.lcov")?))?;
    let smoke = coverage_lcov_lines(Path::new(coverage_arg(args, 2, "smoke.lcov")?))?;
    let test_floor = coverage_arg(args, 3, "test-floor")?.parse::<u32>().map_err(|_| "coverage floors must be permille integers".to_owned())?;
    let smoke_floor = coverage_arg(args, 4, "smoke-floor")?.parse::<u32>().map_err(|_| "coverage floors must be permille integers".to_owned())?;
    let combined_floor = coverage_arg(args, 5, "combined-floor")?.parse::<u32>().map_err(|_| "coverage floors must be permille integers".to_owned())?;
    if [test_floor, smoke_floor, combined_floor].iter().any(|floor| *floor > 1000) { return Err("coverage floors must be within 0..=1000".to_owned()); }
    let mut combined = test.clone();
    for (key, hits) in &smoke { combined.entry(key.clone()).and_modify(|seen| *seen = (*seen).max(*hits)).or_insert(*hits); }
    let test_ratio = coverage_ratio(&test);
    let smoke_ratio = coverage_ratio(&smoke);
    let combined_ratio = coverage_ratio(&combined);
    println!("eoie coverage assess test={test_ratio}/1000 smoke={smoke_ratio}/1000 combined={combined_ratio}/1000 lines={}", combined.len());
    if test_ratio < test_floor || smoke_ratio < smoke_floor || combined_ratio < combined_floor {
        return Err(format!("coverage floor failed test={test_ratio}/{test_floor} smoke={smoke_ratio}/{smoke_floor} combined={combined_ratio}/{combined_floor}"));
    }
    Ok(())
}

pub fn prune_uncovered(args: &[String]) -> Result<(), String> {
    let root_path = std::path::absolute(coverage_arg(args, 1, "root")?).map_err(|error| error.to_string())?;
    let root = root_path.as_path();
    let relative = coverage_arg(args, 2, "relative")?;
    let test_lcov = Path::new(coverage_arg(args, 3, "test.lcov")?);
    let smoke_lcov = Path::new(coverage_arg(args, 4, "smoke.lcov")?);
    let separator = args.iter().position(|value| value == "--").ok_or_else(|| "prune-uncovered requires -- command args".to_owned())?;
    if separator < 5 || separator + 1 >= args.len() { return Err("prune-uncovered requires a validation command".to_owned()); }
    let source = coverage_rooted(root, relative)?;
    let test_state = coverage_lcov_state(test_lcov, root, &source)?;
    let smoke_state = coverage_lcov_state(smoke_lcov, root, &source)?;
    if test_state != Some(false) || smoke_state != Some(false) {
        return Err(format!("prune refused: coverage evidence must exist and be zero in both profiles, test={test_state:?} smoke={smoke_state:?}"));
    }
    let receipt = coverage_rooted(root, "state/prune_receipts.spi")?;
    let original_receipts = coverage_read_regular_text_limited(&receipt, 8 * 1024 * 1024).map_err(|error| format!("read {}: {error}", receipt.display()))?;
    if fs::canonicalize(&source).map_err(|error| error.to_string())? == fs::canonicalize(&receipt).map_err(|error| error.to_string())? {
        return Err("cannot prune the receipt ledger".to_owned());
    }
    let marker = "inl main () : i32 = 0i32";
    if original_receipts.matches(marker).count() != 1 { return Err("prune receipt ledger must have exactly one main marker".to_owned()); }
    let hash = inspection_file_sha256(&source)?;
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|error| error.to_string())?.as_nanos();
    let command_text = args[separator + 1..].join(" ");
    let bundle_id = env::var("EOIE_BUNDLE_ID").unwrap_or_else(|_| "external".to_owned());
    let entry = format!("inl receipt_{}_{stamp} () : prune_receipt = PruneReceipt (\"{}\", \"{}\", \"{}\", \"{}\")\n", std::process::id(), coverage_escape_spi(relative), coverage_escape_spi(&hash), coverage_escape_spi(&command_text), coverage_escape_spi(&bundle_id));
    let updated = original_receipts.replacen(marker, &format!("{entry}{marker}"), 1);
    let stage = source.parent().ok_or("prune source has no parent")?.join(format!(".eoie-prune-{}-{stamp}", std::process::id()));
    fs::create_dir(&stage).map_err(|error| format!("claim prune stage: {error}"))?;
    let capability = stage.join("rollback-capability");
    if let Err(error) = eoie_rust_std_fs_mutation::rooted_create_hardlink_within(root, &source, &capability) {
        let _ = fs::remove_dir(&stage);
        return Err(format!("prune requires exclusive rollback support; source unchanged: {error}"));
    }
    if let Err(error) = fs::remove_file(&capability) {
        return Err(format!("prune rollback preflight cleanup failed; source unchanged; link retained at {}: {error}", capability.display()));
    }
    let backup = stage.join("original");
    if let Err(error) = fs::rename(&source, &backup) {
        let _ = fs::remove_dir(&stage);
        return Err(format!("stage prune: {error}"));
    }
    let mut ledger_written = false;
    let result = (|| -> Result<(), String> {
        if inspection_file_sha256(&backup)? != hash { return Err("prune source changed during staging".to_owned()); }
        let mut validation = Command::new(&args[separator + 1]);
        validation.current_dir(root).args(&args[separator + 2..]);
        let observation = coverage_run_bounded_receipted_observed(&mut validation, 300_000, "coverage-prune-validation");
        if !observation.as_ref().is_ok_and(|value| value.status.success()) { return Err(format!("prune validation failed: {observation:?}")); }
        if eoie_rust_std_fs::rooted_entry_exists(&source)? { return Err("prune validation recreated the source".to_owned()); }
        if coverage_read_regular_text_limited(&receipt, 8 * 1024 * 1024)? != original_receipts { return Err("prune receipt ledger changed during validation".to_owned()); }
        coverage_atomic_write(&receipt, updated.as_bytes()).map_err(|error| format!("write prune receipt: {error}"))?;
        ledger_written = true;
        coverage_typecheck(&receipt).map_err(|error| format!("prune receipt failed: {error}"))?;
        if eoie_rust_std_fs::rooted_entry_exists(&source)? { return Err("prune source reappeared before commit".to_owned()); }
        if coverage_read_regular_text_limited(&receipt, 8 * 1024 * 1024)? != updated { return Err("prune receipt ledger changed before commit".to_owned()); }
        fs::remove_file(&backup).map_err(|error| format!("commit prune: {error}"))?;
        Ok(())
    })();
    if let Err(error) = result {
        let mut failures = Vec::new();
        if ledger_written {
            match coverage_read_regular_text_limited(&receipt, 8 * 1024 * 1024) {
                Ok(current) if current == updated => {
                    if let Err(error) = coverage_atomic_write(&receipt, original_receipts.as_bytes()) { failures.push(format!("receipt restore: {error}")); }
                }
                _ => failures.push("receipt changed externally; preserved for review".to_owned()),
            }
        }
        // An exclusive hard link restores the original metadata without replacing a concurrent file.
        match eoie_rust_std_fs_mutation::rooted_create_hardlink_within(root, &backup, &source) {
            Ok(()) => { if let Err(error) = fs::remove_file(&backup) { failures.push(format!("backup cleanup: {error}")); } }
            Err(error) => failures.push(format!("source restore: {error}; original retained at {}", backup.display())),
        }
        if failures.is_empty() {
            let _ = fs::remove_dir(&stage);
            return Err(format!("{error}; rolled back"));
        }
        return Err(format!("{error}; rollback incomplete: {}", failures.join("; ")));
    }
    if let Err(error) = fs::remove_dir(&stage) { eprintln!("prune stage cleanup failed: {error}"); }
    println!("eoie proxy prune-uncovered ok relative={relative} sha256={hash}");
    Ok(())
}

pub fn coverage_finish(code: i32) -> i32 {
    #[cfg(all(windows, eoie_coverage))]
    {
        unsafe extern "C" { fn __llvm_profile_write_file() -> i32; }
        // The generated launcher uses process::exit; Windows does not finish the
        // profiler's buffered writes through that path.
        if unsafe { __llvm_profile_write_file() } != 0 {
            eprintln!("eoie error: coverage profile flush failed");
            return if code == 0 { 1 } else { code };
        }
    }
    code
}

fn coverage_absolute(args: &[String], index: usize, name: &str) -> Result<PathBuf, String> {
    std::path::absolute(coverage_arg(args, index, name)?).map_err(|error| format!("resolve coverage {name}: {error}"))
}

fn coverage_jobs() -> Result<String, String> {
    let requested = env::var("EOIE_COVERAGE_JOBS").or_else(|_| env::var("CARGO_BUILD_JOBS")).ok();
    let requested = match requested {
        Some(value) => value.parse::<i32>().ok().filter(|value| *value > 0).unwrap_or(-1),
        None => 0,
    };
    let available = std::thread::available_parallelism().map(|value| i32::try_from(value.get()).unwrap_or(i32::MAX)).unwrap_or(1);
    let jobs = eoie_coverage_jobs(requested, available);
    if jobs < 1 { return Err("EOIE_COVERAGE_JOBS/CARGO_BUILD_JOBS must be within 1..=256".to_owned()); }
    Ok(jobs.to_string())
}

fn coverage_run_test_args(scope: &str, jobs: &str) -> Result<Vec<String>, String> {
    let mut args = vec!["test".to_owned(), "--offline".to_owned(), "--locked".to_owned(), "--release".to_owned()];
    match scope {
        "public" => args.extend(["-p", "eoie-contracts", "--lib"].into_iter().map(str::to_owned)),
        "workspace" => args.extend(["--workspace", "--lib", "--bins", "--tests"].into_iter().map(str::to_owned)),
        value if value.starts_with("owner:") => { let package = &value[6..]; if package.is_empty() || !package.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_') { return Err("coverage owner scope is invalid".to_owned()); } args.extend(["-p".to_owned(), package.to_owned(), "--all-targets".to_owned()]); },
        _ => return Err("coverage scope must be public workspace smoke or owner:<package>".to_owned()),
    } // coverage_run_test_args match
    args.extend(["--jobs".to_owned(), jobs.to_owned()]);
    Ok(args)
} // coverage_run_test_args

fn coverage_clear_profraw(path: &Path) -> Result<usize, String> {
    fs::create_dir_all(path).map_err(|error| format!("create {}: {error}", path.display()))?;
    let mut removed = 0usize;
    for entry in fs::read_dir(path).map_err(|error| format!("read {}: {error}", path.display()))? {
        let child = entry.map_err(|error| error.to_string())?.path();
        if child.is_file() && child.extension().and_then(|value| value.to_str()) == Some("profraw") {
            fs::remove_file(&child).map_err(|error| format!("remove {}: {error}", child.display()))?;
            removed += 1;
        } // coverage_clear_profraw if
    } // coverage_clear_profraw loop
    Ok(removed)
} // coverage_clear_profraw

fn coverage_profile_count(path: &Path) -> Result<usize, String> {
    let mut count = 0;
    for entry in fs::read_dir(path).map_err(|error| format!("read {}: {error}", path.display()))? {
        let entry = entry.map_err(|error| error.to_string())?;
        if entry.path().extension().and_then(|value| value.to_str()) != Some("profraw") { continue; }
        if entry.file_type().map_err(|error| error.to_string())?.is_file() && entry.metadata().map_err(|error| error.to_string())?.len() > 0 { count += 1; }
    }
    Ok(count)
} // coverage_profile_count

struct CoverageRunContext<'a> { cargo: &'a Path, source: &'a Path, target: &'a Path, profile_pattern: &'a str, compiler: &'a Path, binary: &'a Path, jobs: &'a str, timeout_ms: u64 }

fn coverage_run_phase(context: &CoverageRunContext<'_>, command_args: &[String], phase: &str) -> Result<(), String> {
    let started = std::time::Instant::now();
    let mut command = Command::new(context.cargo);
    command.current_dir(context.source).args(command_args).env("CARGO_TARGET_DIR", context.target).env("CARGO_BUILD_JOBS", context.jobs).env("RUST_TEST_THREADS", env::var("RUST_TEST_THREADS").unwrap_or_else(|_| context.jobs.to_owned())).env("CARGO_INCREMENTAL", "1").env_remove("CARGO_ENCODED_RUSTFLAGS").env("RUSTFLAGS", "-C instrument-coverage --cfg eoie_coverage").env("LLVM_PROFILE_FILE", context.profile_pattern).env("EOIE_SPIRAL_COMPILE", context.compiler).env("EOIE_BIN_UNDER_TEST", context.binary).env("RAYON_NUM_THREADS", context.jobs);
    let _observation = coverage_run_bounded_streaming_checked(&mut command, context.timeout_ms, phase, None)?;
    println!("eoie coverage phase={phase} status=passed elapsed_ms={}", started.elapsed().as_millis());
    Ok(()) // coverage_run_phase
} // coverage_run_phase

pub fn coverage_run(args: &[String]) -> Result<(), String> {
    if args.len() != 8 { return Err("coverage-run expects root cargo compiler target-dir profraw-dir public|workspace|smoke|owner:<package> timeout-ms".to_owned()); }
    let root_path = coverage_absolute(args, 1, "root")?;
    let root = root_path.as_path();
    let source = root.join("src");
    let cargo_path = coverage_absolute(args, 2, "cargo")?;
    let cargo = cargo_path.as_path();
    let compiler_path = coverage_absolute(args, 3, "compiler")?;
    let compiler = compiler_path.as_path();
    let target_path = coverage_absolute(args, 4, "target-dir")?;
    let target = target_path.as_path();
    let profraw_path = coverage_absolute(args, 5, "profraw-dir")?;
    let profraw = profraw_path.as_path();
    let scope = coverage_arg(args, 6, "scope")?;
    let timeout_ms = coverage_arg(args, 7, "timeout-ms")?.parse::<u64>().map_err(|_| "coverage timeout must be a positive integer".to_owned())?;
    if timeout_ms == 0 { return Err("coverage timeout must be positive".to_owned()); }
    if !source.join("Cargo.toml").is_file() { return Err(format!("coverage root has no src/Cargo.toml: {}", root.display())); }
    if !cargo.is_file() || !compiler.is_file() { return Err("coverage cargo and compiler must be files".to_owned()); }
    let jobs = coverage_jobs()?;
    if scope != "smoke" { coverage_run_test_args(scope, &jobs)?; }
    else { coverage_smoke_preflight(&env::current_exe().map_err(|error| error.to_string())?, root, timeout_ms)?; }
    fs::create_dir_all(target).map_err(|error| format!("create {}: {error}", target.display()))?;
    let profile_pattern = profraw.join("%p-%m.profraw").to_string_lossy().into_owned();
    let binary = target.join("release").join(if cfg!(windows) { "eoie.exe" } else { "eoie" });
    let smoke_child = env::var("EOIE_COVERAGE_SMOKE_CHILD").ok().as_deref() == Some("1");
    if smoke_child {
        if scope != "smoke" { return Err("coverage smoke child requires smoke scope".to_owned()); }
        if !binary.is_file() { return Err(format!("instrumented EOIE binary missing: {}", binary.display())); }
        let smoke_context = CoverageSmokeContext { root, cargo, compiler, binary: &binary, target, profile_pattern: &profile_pattern, timeout_ms, jobs: &jobs };
        coverage_smoke_run(&smoke_context)?;
        println!("eoie coverage smoke child ok target={}", target.display());
        return Ok(());
    }
    let removed = coverage_clear_profraw(profraw)?;
    let build = ["build", "--offline", "--locked", "--release", "-p", "eoie-cli", "--bin", "eoie", "--jobs", jobs.as_str()].into_iter().map(str::to_owned).collect::<Vec<_>>();
    let context = CoverageRunContext { cargo, source: &source, target, profile_pattern: &profile_pattern, compiler, binary: &binary, jobs: &jobs, timeout_ms };
    coverage_run_phase(&context, &build, "build-binary")?;
    if !binary.is_file() { return Err(format!("instrumented EOIE binary missing: {}", binary.display())); }
    // Cargo test may relink this binary; its early probe is not test-run evidence.
    let preflight = profraw.join("preflight");
    coverage_clear_profraw(&preflight)?;
    let mut probe = Command::new(&binary);
    probe.current_dir(root).args(["help", "--schema"]).env("LLVM_PROFILE_FILE", preflight.join("%p-%m.profraw"));
    coverage_run_bounded_streaming_checked(&mut probe, timeout_ms, "coverage-binary-probe", None)?;
    if coverage_profile_count(&preflight)? == 0 { return Err("instrumented EOIE produced no nonempty profile; check coverage flags and Windows profile flushing".to_owned()); }

    if scope == "smoke" {
        let mut command = Command::new(&binary);
        command.current_dir(root).arg("proxy").args(args).env("EOIE_COVERAGE_SMOKE_CHILD", "1").env("LLVM_PROFILE_FILE", &profile_pattern).env("EOIE_SPIRAL_COMPILE", compiler).env("EOIE_BIN_UNDER_TEST", &binary).env("CARGO_TARGET_DIR", target).env("RAYON_NUM_THREADS", &jobs);
        let _observation = coverage_run_bounded_streaming_checked(&mut command, timeout_ms, "smoke-reentry", None)?;
    } else {
        let tests = coverage_run_test_args(scope, &jobs)?;
        coverage_run_phase(&context, &tests, "contracts")?;
    } // coverage_run smoke branch
    let profiles = coverage_profile_count(profraw)?;
    if profiles == 0 { return Err("coverage run produced no profraw files".to_owned()); }
    println!("eoie proxy coverage-run ok scope={scope} profiles={profiles} cleared={removed} target={}", target.display());
    Ok(()) // coverage_run
} // coverage_run

mod coverage_run_tests {
    #[test] fn coverage_scope_builds_bounded_test_surfaces() {
        let public = super::coverage_run_test_args("public", "56").expect("public");
        assert!(public.iter().any(|arg| arg == "eoie-contracts"));
        let workspace = super::coverage_run_test_args("workspace", "56").expect("workspace");
        assert!(workspace.iter().any(|arg| arg == "--workspace"));
        let owner = super::coverage_run_test_args("owner:eoie-patch-transaction-contracts", "56").expect("owner");
        assert!(owner.windows(2).any(|pair| pair == ["-p", "eoie-patch-transaction-contracts"]));
        assert!(super::coverage_run_test_args("owner:../bad", "56").is_err());
        assert!(super::coverage_run_test_args("unknown", "56").is_err());
    } // coverage_scope_builds_bounded_test_surfaces
} // coverage_run_tests
fn coverage_profraw_files(path: &Path) -> Result<Vec<PathBuf>, String> {
    let metadata = fs::symlink_metadata(path).map_err(|error| format!("metadata {}: {error}", path.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() { return Err("coverage profraw path must be a real directory".to_owned()); }
    let mut profiles = fs::read_dir(path).map_err(|error| error.to_string())?.try_fold(Vec::new(), |mut profiles, entry| -> Result<Vec<PathBuf>, String> { let child = entry.map_err(|error| error.to_string())?.path(); let child_metadata = fs::symlink_metadata(&child).map_err(|error| error.to_string())?; if child_metadata.file_type().is_symlink() { return Err(format!("coverage profile cannot be a symlink: {}", child.display())); } if child_metadata.is_file() && child.extension().and_then(|value| value.to_str()) == Some("profraw") { profiles.push(child); } Ok(profiles) })?;
    profiles.sort();
    if profiles.is_empty() { return Err("coverage export found no profraw profiles".to_owned()); }
    Ok(profiles)
} // coverage_profraw_files

fn coverage_export_command(grcov: &Path, profraw: &Path, binary_path: &Path, llvm_path: &Path, source: &Path, output: &Path) -> Command {
    let mut command = Command::new(grcov);
    command.arg(profraw).arg("-s").arg(source).arg("-b").arg(binary_path).arg("--llvm-path").arg(llvm_path).args(["-t", "lcov", "-o"]).arg(output).args(["--llvm", "--ignore-not-existing", "--ignore", "*/vendor/*", "--ignore", "vendor/*", "--ignore", "*/rustc/*", "--ignore", "rustc/*"]);
    command
} // coverage_export_command

pub fn coverage_export(args: &[String]) -> Result<(), String> {
    if args.len() != 8 { return Err("coverage-export expects profraw-dir binary-path grcov llvm-bin-dir source-root output-lcov timeout-ms".to_owned()); }
    let profraw = Path::new(coverage_arg(args, 1, "profraw-dir")?);
    let binary_path = Path::new(coverage_arg(args, 2, "binary-path")?);
    let grcov = Path::new(coverage_arg(args, 3, "grcov")?);
    let llvm_path = Path::new(coverage_arg(args, 4, "llvm-bin-dir")?);
    let source = Path::new(coverage_arg(args, 5, "source-root")?);
    let output = Path::new(coverage_arg(args, 6, "output-lcov")?);
    let timeout_ms = coverage_arg(args, 7, "timeout-ms")?.parse::<u64>().map_err(|_| "coverage export timeout must be a positive integer".to_owned())?;
    if timeout_ms == 0 { return Err("coverage export timeout must be positive".to_owned()); }
    let profiles = coverage_profraw_files(profraw)?;
    for (label, path, directory) in [("binary path", binary_path, false), ("llvm path", llvm_path, true), ("source root", source, true), ("grcov", grcov, false)] {
        let metadata = fs::symlink_metadata(path).map_err(|error| format!("metadata {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() || (directory && !metadata.is_dir()) || (!directory && !metadata.is_file()) { return Err(format!("coverage export {label} has invalid type: {}", path.display())); }
    }
    let parent = output.parent().filter(|path| !path.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|error| format!("create {}: {error}", parent.display()))?;
    let receipt = PathBuf::from(format!("{}.receipt", output.display()));
    for destination in [output, receipt.as_path()] {
        match fs::symlink_metadata(destination) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => return Err("coverage output and receipt must be regular file paths".to_owned()),
            Ok(_) => {},
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {},
            Err(error) => return Err(format!("metadata {}: {error}", destination.display())),
        }
    }
    if let Ok(destination) = fs::canonicalize(output) {
        for input in [binary_path, grcov].into_iter().chain(profiles.iter().map(PathBuf::as_path)) {
            if fs::canonicalize(input).is_ok_and(|input| input == destination) { return Err("coverage output cannot replace an input".to_owned()); }
        }
    }
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|error| error.to_string())?.as_nanos();
    let stage = parent.join(format!(".eoie-coverage-{}-{stamp}", std::process::id()));
    fs::create_dir(&stage).map_err(|error| format!("create coverage stage: {error}"))?;
    let candidate = stage.join("candidate.lcov");
    let result = (|| -> Result<(), String> {
        let mut command = coverage_export_command(grcov, profraw, binary_path, llvm_path, source, &candidate);
        let _observation = coverage_run_bounded_streaming_checked(&mut command, timeout_ms, "coverage-export", None)?;

        let lines = coverage_lcov_lines(&candidate)?;
        let payload = coverage_read_regular_text_limited(&candidate, 64 * 1024 * 1024)?;
        let ratio = coverage_ratio(&lines);
        let hash = inspection_file_sha256(&candidate)?;
        let receipt_text = format!("route=grcov-lcov\nprofiles={}\nlines={}\ncovered_permille={ratio}\nsha256={hash}\n", profiles.len(), lines.len());
        coverage_atomic_write(output, payload.as_bytes())?;
        coverage_atomic_write(&receipt, receipt_text.as_bytes())?;
        println!("eoie proxy coverage-export ok profiles={} lines={} covered={ratio}/1000 sha256={hash}", profiles.len(), lines.len());
        Ok(())
    })();
    if let Err(error) = fs::remove_dir_all(&stage) { eprintln!("coverage stage cleanup failed: {error}"); }
    result
} // coverage_export

mod coverage_export_tests {
    #[test] fn coverage_export_requires_profiles() { let root = std::env::temp_dir().join(format!("eoie-coverage-export-{}", std::process::id())); let _ = std::fs::remove_dir_all(&root); std::fs::create_dir_all(&root).expect("root"); assert!(super::coverage_profraw_files(&root).is_err()); std::fs::write(root.join("b.profraw"), b"b").expect("b"); std::fs::write(root.join("a.profraw"), b"a").expect("a"); let profiles = super::coverage_profraw_files(&root).expect("profiles"); assert!(profiles[0].ends_with("a.profraw")); assert!(profiles[1].ends_with("b.profraw")); std::fs::remove_dir_all(root).expect("cleanup"); }
} // coverage_export_tests
fn coverage_union_records(path: &Path) -> Result<BTreeMap<String, BTreeMap<u32, u64>>, String> {
    let text = coverage_read_regular_text_limited(path, 64 * 1024 * 1024).map_err(|error| format!("read {}: {error}", path.display()))?;
    let mut active: Option<String> = None;
    let mut records: BTreeMap<String, BTreeMap<u32, u64>> = BTreeMap::new();
    for raw in text.lines() {
        if let Some(source) = raw.strip_prefix("SF:") {
            if active.is_some() { return Err("coverage has SF before end_of_record".to_owned()); }
            if source.trim().is_empty() { return Err("coverage has an empty SF record".to_owned()); }
            let source = source.replace('\\', "/");
            records.entry(source.clone()).or_default();
            active = Some(source);
        } else if let Some(value) = raw.strip_prefix("DA:") {
            let source = active.as_ref().ok_or_else(|| "coverage has DA before SF".to_owned())?;
            let parts = value.split(',').collect::<Vec<_>>();
            if !(2..=3).contains(&parts.len()) || parts[..2].iter().any(|value| value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit())) {
                return Err("coverage has invalid DA fields".to_owned());
            }
            let line = parts[0].parse::<u32>().ok().filter(|line| *line > 0).ok_or_else(|| "coverage has invalid DA line".to_owned())?;
            let hits = parts[1].parse::<u64>().map_err(|_| "coverage has invalid DA hits".to_owned())?;
            records.get_mut(source).unwrap().entry(line).and_modify(|seen| *seen = (*seen).max(hits)).or_insert(hits);
        } else if raw == "end_of_record" {
            if active.take().is_none() { return Err("coverage has end_of_record before SF".to_owned()); }
        } else if raw.starts_with("DA") || raw.starts_with("SF") {
            return Err("coverage has a malformed source or line record".to_owned());
        }
    }
    if active.is_some() { return Err("coverage has an unterminated source record".to_owned()); }
    if !records.values().any(|lines| !lines.is_empty()) { return Err("coverage input has no line records".to_owned()); }
    Ok(records)
} // coverage_union_records

fn coverage_union_merge(target: &mut BTreeMap<String, BTreeMap<u32, u64>>, source: BTreeMap<String, BTreeMap<u32, u64>>) {
    for (file, lines) in source { let target_lines = target.entry(file).or_default(); for (line, hits) in lines { target_lines.entry(line).and_modify(|seen| *seen = (*seen).max(hits)).or_insert(hits); } }
} // coverage_union_merge

fn coverage_union_payload(records: &BTreeMap<String, BTreeMap<u32, u64>>) -> String {
    let mut payload = String::new();
    for (source, lines) in records { payload.push_str("SF:"); payload.push_str(source); payload.push(char::from(10)); for (line, hits) in lines { payload.push_str(&format!("DA:{line},{hits}\n")); } payload.push_str("end_of_record\n"); }
    payload
} // coverage_union_payload

pub fn coverage_union(args: &[String]) -> Result<(), String> {
    if args.len() < 4 { return Err("coverage-union expects output-lcov and at least two input LCOV files".to_owned()); }
    let output = Path::new(coverage_arg(args, 1, "output-lcov")?);
    let mut records: BTreeMap<String, BTreeMap<u32, u64>> = BTreeMap::new();
    for input in &args[2..] { let path = Path::new(input); if path == output { return Err("coverage union output cannot also be an input".to_owned()); } coverage_union_merge(&mut records, coverage_union_records(path)?); }
    if output.exists() { let metadata = fs::symlink_metadata(output).map_err(|error| error.to_string())?; if metadata.file_type().is_symlink() || !metadata.is_file() { return Err("coverage union output must be a regular file path".to_owned()); } }
    let payload = coverage_union_payload(&records);
    coverage_atomic_write(output, payload.as_bytes())?;
    let flat = coverage_lcov_lines(output)?;
    let ratio = coverage_ratio(&flat);
    let hash = inspection_file_sha256(output)?;
    let receipt = PathBuf::from(format!("{}.receipt", output.display()));
    let receipt_text = format!("route=lcov-union\ninputs={}\nlines={}\ncovered_permille={ratio}\nsha256={hash}\n", args.len() - 2, flat.len());
    coverage_atomic_write(&receipt, receipt_text.as_bytes())?;
    println!("eoie proxy coverage-union ok inputs={} lines={} covered={ratio}/1000 sha256={hash}", args.len() - 2, flat.len());
    Ok(())
} // coverage_union

mod coverage_union_tests {
    #[test] fn coverage_union_merges_max_hits_and_writes_receipt() { let root = std::env::temp_dir().join(format!("eoie-coverage-union-{}", std::process::id())); let _ = std::fs::remove_dir_all(&root); std::fs::create_dir_all(&root).expect("root"); let first = root.join("first.lcov"); let second = root.join("second.lcov"); let output = root.join("union.lcov"); std::fs::write(&first, b"SF:src/a.rs\nDA:1,1\nDA:2,0\nend_of_record\n").expect("first"); std::fs::write(&second, b"SF:src/a.rs\nDA:1,3\nDA:2,1\nend_of_record\n").expect("second"); let args = vec!["coverage-union".to_owned(), output.to_string_lossy().into_owned(), first.to_string_lossy().into_owned(), second.to_string_lossy().into_owned()]; super::coverage_union(&args).expect("union"); let text = std::fs::read_to_string(&output).expect("output"); assert!(text.contains("DA:1,3")); assert!(text.contains("DA:2,1")); let receipt = std::fs::read_to_string(format!("{}.receipt", output.display())).expect("receipt"); assert!(receipt.contains("inputs=2")); std::fs::remove_dir_all(root).expect("cleanup"); }
} // coverage_union_tests
fn method0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 0i32;
    if v2 {
        let mut v3: bool = v1 < 2i32;
        if v3 {
            1i32
        } else {
            2i32
        }
    } else {
        let mut v5: bool = v0 < 1i32;
        let mut v7: bool = if v5 {
            true
        } else {
            let mut v6: bool = 256i32 < v0;
            v6
        };
        if v7 {
            -1i32
        } else {
            v0
        }
    }
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method0(v0, v1)
    })
}
pub fn eoie_coverage_jobs(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
