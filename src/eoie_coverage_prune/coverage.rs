#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
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
    }
    args.extend(["--jobs".to_owned(), jobs.to_owned()]);
    Ok(args)
}

fn coverage_clear_profraw(path: &Path) -> Result<usize, String> {
    fs::create_dir_all(path).map_err(|error| format!("create {}: {error}", path.display()))?;
    let mut removed = 0usize;
    for entry in fs::read_dir(path).map_err(|error| format!("read {}: {error}", path.display()))? {
        let child = entry.map_err(|error| error.to_string())?.path();
        if child.is_file() && child.extension().and_then(|value| value.to_str()) == Some("profraw") {
            fs::remove_file(&child).map_err(|error| format!("remove {}: {error}", child.display()))?;
            removed += 1;
        }
    }
    Ok(removed)
}

fn coverage_profile_count(path: &Path) -> Result<usize, String> {
    let mut count = 0;
    for entry in fs::read_dir(path).map_err(|error| format!("read {}: {error}", path.display()))? {
        let entry = entry.map_err(|error| error.to_string())?;
        if entry.path().extension().and_then(|value| value.to_str()) != Some("profraw") { continue; }
        if entry.file_type().map_err(|error| error.to_string())?.is_file() && entry.metadata().map_err(|error| error.to_string())?.len() > 0 { count += 1; }
    }
    Ok(count)
}

struct CoverageRunContext<'a> { cargo: &'a Path, source: &'a Path, target: &'a Path, profile_pattern: &'a str, compiler: &'a Path, binary: &'a Path, jobs: &'a str, timeout_ms: u64 }

fn coverage_run_phase(context: &CoverageRunContext<'_>, command_args: &[String], phase: &str) -> Result<(), String> {
    let started = std::time::Instant::now();
    let mut command = Command::new(context.cargo);
    command.current_dir(context.source).args(command_args).env("CARGO_TARGET_DIR", context.target).env("CARGO_BUILD_JOBS", context.jobs).env("RUST_TEST_THREADS", env::var("RUST_TEST_THREADS").unwrap_or_else(|_| context.jobs.to_owned())).env("CARGO_INCREMENTAL", "1").env_remove("CARGO_ENCODED_RUSTFLAGS").env("RUSTFLAGS", "-C instrument-coverage --cfg eoie_coverage").env("LLVM_PROFILE_FILE", context.profile_pattern).env("EOIE_SPIRAL_COMPILE", context.compiler).env("EOIE_BIN_UNDER_TEST", context.binary).env("RAYON_NUM_THREADS", context.jobs);
    let _observation = coverage_run_bounded_streaming_checked(&mut command, context.timeout_ms, phase, None)?;
    println!("eoie coverage phase={phase} status=passed elapsed_ms={}", started.elapsed().as_millis());
    Ok(())
}

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
    }
    let profiles = coverage_profile_count(profraw)?;
    if profiles == 0 { return Err("coverage run produced no profraw files".to_owned()); }
    println!("eoie proxy coverage-run ok scope={scope} profiles={profiles} cleared={removed} target={}", target.display());
    Ok(())
}

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
    }
}
fn coverage_profraw_files(path: &Path) -> Result<Vec<PathBuf>, String> {
    let metadata = fs::symlink_metadata(path).map_err(|error| format!("metadata {}: {error}", path.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() { return Err("coverage profraw path must be a real directory".to_owned()); }
    let mut profiles = fs::read_dir(path).map_err(|error| error.to_string())?.try_fold(Vec::new(), |mut profiles, entry| -> Result<Vec<PathBuf>, String> { let child = entry.map_err(|error| error.to_string())?.path(); let child_metadata = fs::symlink_metadata(&child).map_err(|error| error.to_string())?; if child_metadata.file_type().is_symlink() { return Err(format!("coverage profile cannot be a symlink: {}", child.display())); } if child_metadata.is_file() && child.extension().and_then(|value| value.to_str()) == Some("profraw") { profiles.push(child); } Ok(profiles) })?;
    profiles.sort();
    if profiles.is_empty() { return Err("coverage export found no profraw profiles".to_owned()); }
    Ok(profiles)
}

fn coverage_export_command(grcov: &Path, profraw: &Path, binary_path: &Path, llvm_path: &Path, source: &Path, output: &Path) -> Command {
    let mut command = Command::new(grcov);
    command.arg(profraw).arg("-s").arg(source).arg("-b").arg(binary_path).arg("--llvm-path").arg(llvm_path).args(["-t", "lcov", "-o"]).arg(output).args(["--llvm", "--ignore-not-existing", "--ignore", "*/vendor/*", "--ignore", "vendor/*", "--ignore", "*/rustc/*", "--ignore", "rustc/*"]);
    command
}

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
}

mod coverage_export_tests {
    #[test] fn coverage_export_requires_profiles() { let root = std::env::temp_dir().join(format!("eoie-coverage-export-{}", std::process::id())); let _ = std::fs::remove_dir_all(&root); std::fs::create_dir_all(&root).expect("root"); assert!(super::coverage_profraw_files(&root).is_err()); std::fs::write(root.join("b.profraw"), b"b").expect("b"); std::fs::write(root.join("a.profraw"), b"a").expect("a"); let profiles = super::coverage_profraw_files(&root).expect("profiles"); assert!(profiles[0].ends_with("a.profraw")); assert!(profiles[1].ends_with("b.profraw")); std::fs::remove_dir_all(root).expect("cleanup"); }
}
fn coverage_union_records(path: &Path) -> Result<BTreeMap<String, BTreeMap<u32, u64>>, String> {
    let text = coverage_read_regular_text_limited(path, 64 * 1024 * 1024).map_err(|error| format!("read {}: {error}", path.display()))?;
    let error = eoie_lcov_error(&text);
    if !error.is_empty() { return Err(error.to_string()); }
    let mut records: BTreeMap<String, BTreeMap<u32, u64>> = BTreeMap::new();
    for row in eoie_lcov_rows(&text).lines() {
        let mut fields = row.split('\t');
        let source = fields.next().unwrap_or_default().to_owned();
        let lines = records.entry(source).or_default();
        if let (Some(line), Some(hits)) = (fields.next(), fields.next()) {
            let (line, hits) = (line.parse::<u32>().unwrap_or(0), hits.parse::<u64>().unwrap_or(0));
            lines.entry(line).and_modify(|seen| *seen = (*seen).max(hits)).or_insert(hits);
        }
    }
    Ok(records)
}

fn coverage_union_merge(target: &mut BTreeMap<String, BTreeMap<u32, u64>>, source: BTreeMap<String, BTreeMap<u32, u64>>) {
    for (file, lines) in source { let target_lines = target.entry(file).or_default(); for (line, hits) in lines { target_lines.entry(line).and_modify(|seen| *seen = (*seen).max(hits)).or_insert(hits); } }
}

fn coverage_union_payload(records: &BTreeMap<String, BTreeMap<u32, u64>>) -> String {
    let mut payload = String::new();
    for (source, lines) in records { payload.push_str("SF:"); payload.push_str(source); payload.push(char::from(10)); for (line, hits) in lines { payload.push_str(&format!("DA:{line},{hits}\n")); } payload.push_str("end_of_record\n"); }
    payload
}

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
}

mod coverage_union_tests {
    #[test] fn coverage_union_merges_max_hits_and_writes_receipt() { let root = std::env::temp_dir().join(format!("eoie-coverage-union-{}", std::process::id())); let _ = std::fs::remove_dir_all(&root); std::fs::create_dir_all(&root).expect("root"); let first = root.join("first.lcov"); let second = root.join("second.lcov"); let output = root.join("union.lcov"); std::fs::write(&first, b"SF:src/a.rs\nDA:1,1\nDA:2,0\nend_of_record\n").expect("first"); std::fs::write(&second, b"SF:src/a.rs\nDA:1,3\nDA:2,1\nend_of_record\n").expect("second"); let args = vec!["coverage-union".to_owned(), output.to_string_lossy().into_owned(), first.to_string_lossy().into_owned(), second.to_string_lossy().into_owned()]; super::coverage_union(&args).expect("union"); let text = std::fs::read_to_string(&output).expect("output"); assert!(text.contains("DA:1,3")); assert!(text.contains("DA:2,1")); let receipt = std::fs::read_to_string(format!("{}.receipt", output.display())).expect("receipt"); assert!(receipt.contains("inputs=2")); std::fs::remove_dir_all(root).expect("cleanup"); }
}
#[derive(Clone)]
enum US1 {
    US1_SourceBeforeEnd,
    US1_EmptySource,
    US1_HitBeforeSource,
    US1_InvalidHitFields,
    US1_InvalidHitLine,
    US1_InvalidHitCount,
    US1_EndBeforeSource,
    US1_MalformedRecord,
    US1_UnterminatedSource,
    US1_NoLineRecords,
}
#[derive(Clone)]
enum US0 {
    US0_LcovRecords(Rc<str>),
    US0_LcovRejected(US1),
}
#[derive(Clone)]
enum US2 {
    US2_BetweenSources,
    US2_InSource(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_SourceLine(Rc<str>),
    US3_HitLine(Rc<str>),
    US3_EndLine,
    US3_MalformedLine,
    US3_OtherLine,
}
#[derive(Clone)]
enum US4 {
    US4_HitRead(Rc<str>),
    US4_HitRejected(US1),
}
fn coverage_jobs_binding_0(mut v0: i32, mut v1: i32) -> i32 {
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
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        coverage_jobs_binding_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn hit_reading_of_4(mut v0: Rc<str>) -> US4 {
    let mut v1: bool = { let parts = v0.split(char::from(44u8)).collect::<Vec<_>>(); (2..=3).contains(&parts.len()) && parts[..2].iter().all(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())) };
    if v1 {
        let mut v2: u64 = v0.split(char::from(44u8)).next().and_then(|line| line.parse::<u32>().ok()).filter(|line| *line > 0).map(|line| line as u64).unwrap_or(0);
        let mut v3: bool = v2 == 0u64;
        if v3 {
            let mut v4: US1 = US1::US1_InvalidHitLine;
            US4::US4_HitRejected(v4.clone())
        } else {
            let mut v6: bool = v0.split(char::from(44u8)).nth(1).map(|hits| hits.parse::<u64>().is_ok()).unwrap_or(false);
            if v6 {
                let mut v7: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(44u8)).nth(1).unwrap_or(""));
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v8, &*v9, &*v10, &*v10].concat());
                US4::US4_HitRead(v11.clone())
            } else {
                let mut v13: US1 = US1::US1_InvalidHitCount;
                US4::US4_HitRejected(v13.clone())
            }
        }
    } else {
        let mut v17: US1 = US1::US1_InvalidHitFields;
        US4::US4_HitRejected(v17.clone())
    }
}
fn lcov_from_3(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: US2, mut v4: Rc<str>) -> US0 {
    loop {
        let mut v5: bool = v1 < v2;
        if v5 {
            let mut v6: u64 = v1.wrapping_add(1u64);
            let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.lines().nth(v1 as usize).unwrap_or(""));
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SF:"); } LIT.with(|lit| lit.clone()) };
            let mut v9: bool = v7.starts_with(&*v8);
            let mut v28: US3 = if v9 {
                let mut v10: Rc<str> = std::rc::Rc::<str>::from(v7.strip_prefix(&*v8).unwrap_or(""));
                US3::US3_SourceLine(v10.clone())
            } else {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DA:"); } LIT.with(|lit| lit.clone()) };
                let mut v13: bool = v7.starts_with(&*v12);
                if v13 {
                    let mut v14: Rc<str> = std::rc::Rc::<str>::from(v7.strip_prefix(&*v12).unwrap_or(""));
                    US3::US3_HitLine(v14.clone())
                } else {
                    let mut v16: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("end_of_record"); } LIT.with(|lit| lit.clone()) };
                    if v16 {
                        US3::US3_EndLine
                    } else {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DA"); } LIT.with(|lit| lit.clone()) };
                        let mut v19: bool = v7.starts_with(&*v18);
                        let mut v22: bool = if v19 {
                            true
                        } else {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SF"); } LIT.with(|lit| lit.clone()) };
                            let mut v21: bool = v7.starts_with(&*v20);
                            v21
                        };
                        if v22 {
                            US3::US3_MalformedLine
                        } else {
                            US3::US3_OtherLine
                        }
                    }
                }
            };
            match &v28 {
                US3::US3_EndLine => {
                    match &v3 {
                        US2::US2_BetweenSources => {
                            let mut v68: US1 = US1::US1_EndBeforeSource;
                            return US0::US0_LcovRejected(v68.clone());
                        }
                        US2::US2_InSource(v70) => {
                            let mut v70: Rc<str> = v70.clone();
                            let mut v71: US2 = US2::US2_BetweenSources;
                            (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v71.clone(), v4.clone());
                            continue;
                        }
                    }
                }
                US3::US3_HitLine(v48) => {
                    let mut v48: Rc<str> = v48.clone();
                    match &v3 {
                        US2::US2_BetweenSources => {
                            let mut v49: US1 = US1::US1_HitBeforeSource;
                            return US0::US0_LcovRejected(v49.clone());
                        }
                        US2::US2_InSource(v51) => {
                            let mut v51: Rc<str> = v51.clone();
                            let mut v52: US4 = hit_reading_of_4(v48.clone());
                            match &v52 {
                                US4::US4_HitRead(v53) => {
                                    let mut v53: Rc<str> = v53.clone();
                                    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v51, &*v54, &*v53, &*v55, &*v55].concat());
                                    let mut v57: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v60: Rc<str> = if v57 {
                                        v56.clone()
                                    } else {
                                        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                        let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v58, &*v56, &*v55, &*v55].concat());
                                        v59.clone()
                                    };
                                    (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v3.clone(), v60.clone());
                                    continue;
                                }
                                US4::US4_HitRejected(v62) => {
                                    let mut v62: US1 = v62.clone();
                                    return US0::US0_LcovRejected(v62.clone());
                                }
                            }
                        }
                    }
                }
                US3::US3_MalformedLine => {
                    let mut v75: US1 = US1::US1_MalformedRecord;
                    return US0::US0_LcovRejected(v75.clone());
                }
                US3::US3_OtherLine => {
                    (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v3.clone(), v4.clone());
                    continue;
                }
                US3::US3_SourceLine(v29) => {
                    let mut v29: Rc<str> = v29.clone();
                    match &v3 {
                        US2::US2_BetweenSources => {
                            let mut v33: Rc<str> = std::rc::Rc::<str>::from(v29.trim());
                            let mut v34: bool = v33.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            if v34 {
                                let mut v35: US1 = US1::US1_EmptySource;
                                return US0::US0_LcovRejected(v35.clone());
                            } else {
                                let mut v37: Rc<str> = std::rc::Rc::<str>::from(v29.replace(char::from(92u8), "/"));
                                let mut v38: US2 = US2::US2_InSource(v37.clone());
                                let mut v39: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v43: Rc<str> = if v39 {
                                    v37.clone()
                                } else {
                                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v40, &*v37, &*v41, &*v41].concat());
                                    v42.clone()
                                };
                                (v0, v1, v2, v3, v4) = (v0.clone(), v6, v2, v38.clone(), v43.clone());
                                continue;
                            }
                        }
                        US2::US2_InSource(v30) => {
                            let mut v30: Rc<str> = v30.clone();
                            let mut v31: US1 = US1::US1_SourceBeforeEnd;
                            return US0::US0_LcovRejected(v31.clone());
                        }
                    }
                }
            }
        } else {
            match &v3 {
                US2::US2_BetweenSources => {
                    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v87: bool = v4.contains(&*v86);
                    if v87 {
                        return US0::US0_LcovRecords(v4.clone());
                    } else {
                        let mut v89: US1 = US1::US1_NoLineRecords;
                        return US0::US0_LcovRejected(v89.clone());
                    }
                }
                US2::US2_InSource(v83) => {
                    let mut v83: Rc<str> = v83.clone();
                    let mut v84: US1 = US1::US1_UnterminatedSource;
                    return US0::US0_LcovRejected(v84.clone());
                }
            }
        }
    }
}
fn lcov_reading_of_2(mut v0: Rc<str>) -> US0 {
    let mut v1: u64 = 0u64;
    let mut v2: u64 = v0.lines().count() as u64;
    let mut v3: US2 = US2::US2_BetweenSources;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    lcov_from_3(v0.clone(), v1, v2, v3.clone(), v4.clone())
}
fn lcov_error_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = lcov_reading_of_2(v0.clone());
    match &v1 {
        US0::US0_LcovRecords(v2) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US0::US0_LcovRejected(v4) => {
            let mut v4: US1 = v4.clone();
            match &v4 {
                US1::US1_EmptySource => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has an empty SF record"); } LIT.with(|lit| lit.clone()) };
                    v6.clone()
                }
                US1::US1_EndBeforeSource => {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has end_of_record before SF"); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                }
                US1::US1_HitBeforeSource => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has DA before SF"); } LIT.with(|lit| lit.clone()) };
                    v7.clone()
                }
                US1::US1_InvalidHitCount => {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has invalid DA hits"); } LIT.with(|lit| lit.clone()) };
                    v10.clone()
                }
                US1::US1_InvalidHitFields => {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has invalid DA fields"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                }
                US1::US1_InvalidHitLine => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has invalid DA line"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                }
                US1::US1_MalformedRecord => {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has a malformed source or line record"); } LIT.with(|lit| lit.clone()) };
                    v12.clone()
                }
                US1::US1_NoLineRecords => {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage input has no line records"); } LIT.with(|lit| lit.clone()) };
                    v14.clone()
                }
                US1::US1_SourceBeforeEnd => {
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has SF before end_of_record"); } LIT.with(|lit| lit.clone()) };
                    v5.clone()
                }
                US1::US1_UnterminatedSource => {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("coverage has an unterminated source record"); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                }
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        lcov_error_1(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn lcov_rows_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = lcov_reading_of_2(v0.clone());
    match &v1 {
        US0::US0_LcovRecords(v2) => {
            let mut v2: Rc<str> = v2.clone();
            v2.clone()
        }
        US0::US0_LcovRejected(v3) => {
            let mut v3: US1 = v3.clone();
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        lcov_rows_5(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_coverage_jobs(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn eoie_lcov_error(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
pub fn eoie_lcov_rows(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
