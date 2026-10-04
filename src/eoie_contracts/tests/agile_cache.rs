use std::{env, fs, path::PathBuf, process::{Command, Output}, time::{SystemTime, UNIX_EPOCH}};
struct Fixture { root: PathBuf, compiler: PathBuf }
impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie agile cache ü {} {nonce}", std::process::id()));
        fs::create_dir_all(root.join("state")).unwrap();
        fs::create_dir_all(root.join("src/dep")).unwrap();
        fs::write(root.join("state/package.spiproj"), "packageDir: ../src\npackages:\n    dep\nmodules:\n    core\n    prompt\n").unwrap();
        fs::write(root.join("state/core.spi"), "inl main () : i32 = 0i32\n").unwrap();
        fs::write(root.join("src/dep/package.spiproj"), "modules:\n    model\n").unwrap();
        fs::write(root.join("src/dep/model.spi"), "inl main () = 0i32\n").unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        fs::write(root.join("state/prompt.spi"), format!("union lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\ninl current () : prompt_lease = PromptLease (\"cache-test\", \"native cache graph check\", 3600000u64, 600000u64, {now}u64, {}u64, LeaseActive)\ninl main () : i32 = 0i32\n", now + 4200000)).unwrap();
        let source = root.join("compiler.rs");
        fs::write(&source, r#"
fn main() {
    use std::{fs, io::Write};
    assert_eq!(std::env::args().nth(1).as_deref(), Some("--check"));
    let anchor = std::path::PathBuf::from(std::env::args().nth(2).unwrap());
    let state = anchor.parent().unwrap();
    let root = state.parent().unwrap();
    let mut calls = fs::OpenOptions::new().create(true).append(true).open(root.join("calls.txt")).unwrap();
    writeln!(calls, "check").unwrap();
    if fs::read_to_string(state.join("package.spiproj")).unwrap().contains("missing") { std::process::exit(2); }
    if fs::read_to_string(state.join("typecheck_receipts.spi")).unwrap_or_default().contains("invalid receipt") { std::process::exit(2); }
    if root.join("mutate").exists() { fs::write(state.join("core.spi"), "inl main () = 99i32\n").unwrap(); }
}
"#).unwrap();
        let compiler = root.join(format!("fixture-compiler{}", env::consts::EXE_SUFFIX));
        let output = Command::new(env::var_os("RUSTC").unwrap_or_else(|| "rustc".into())).arg(&source).arg("-o").arg(&compiler).output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        Self { root, compiler }
    }
    fn check(&self, compiler: bool) -> Output {
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX)));
        let mut command = Command::new(binary);
        command.args(["agile", "check"]).arg(&self.root).env_remove("EOIE_SPIRAL_COMPILE").env("EOIE_CONFIG_HOME", &self.root).env("PATH", "");
        if compiler { command.arg("--compiler").arg(&self.compiler); }
        command.output().unwrap()
    }
    fn successful(&self) -> String {
        let output = self.check(true);
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        String::from_utf8(output.stdout).unwrap()
    }
    fn calls(&self) -> usize { fs::read_to_string(self.root.join("calls.txt")).unwrap().lines().count() }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.root); } }
#[test]
fn warm_receipts_recheck_manifest_and_imported_sources_and_reject_missing_packages() {
    let fixture = Fixture::new();
    assert!(fixture.successful().contains("build_batches=1"));
    assert!(fixture.successful().contains("build_batches=0"));
    assert_eq!(fixture.calls(), 1);
    fs::write(fixture.root.join("src/dep/model.spi"), "inl main () = 1i32\n").unwrap();
    assert!(fixture.successful().contains("build_batches=1"));
    assert_eq!(fixture.calls(), 2);
    let manifest = fixture.root.join("state/package.spiproj");
    fs::write(&manifest, "packageDir: ../src\npackages:\n    missing\nmodules:\n    core\n    prompt\n").unwrap();
    let before = fs::read(fixture.root.join("state/typecheck_receipts.spi")).unwrap();
    assert!(!fixture.check(true).status.success());
    assert_eq!(fixture.calls(), 3);
    assert_eq!(before, fs::read(fixture.root.join("state/typecheck_receipts.spi")).unwrap());
    let absent = fixture.check(false);
    assert!(!absent.status.success());
    assert!(String::from_utf8_lossy(&absent.stderr).contains("cannot reuse cached receipts"));
}
#[test]
fn unsupported_graphs_require_fresh_attestation_every_time() {
    let fixture = Fixture::new();
    let manifest = fixture.root.join("state/package.spiproj");
    let source = fs::read_to_string(&manifest).unwrap();
    fs::write(manifest, format!("// Compiler syntax beyond the cache parser\n{source}")).unwrap();
    assert!(fixture.successful().contains("build_batches=1"));
    assert!(fixture.successful().contains("build_batches=1"));
    assert_eq!(fixture.calls(), 2);
    assert!(!fixture.check(false).status.success());
}
#[test]
fn concurrent_state_mutation_does_not_publish_receipts() {
    let fixture = Fixture::new();
    fs::write(fixture.root.join("mutate"), "").unwrap();
    let output = fixture.check(true);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("changed during attestation"));
    assert!(!fixture.root.join("state/typecheck_receipts.spi").exists());
}

#[test]
fn generated_receipts_in_package_reuse_cache_but_extra_source_requires_attestation() {
    let fixture = Fixture::new();
    fixture.successful();
    let manifest = fixture.root.join("state/package.spiproj");
    let source = fs::read_to_string(&manifest).unwrap();
    fs::write(manifest, format!("{source}    typecheck_receipts\n")).unwrap();
    assert!(fixture.successful().contains("build_batches=1"));
    assert!(fixture.successful().contains("build_batches=0"));
    let receipt = fixture.root.join("state/typecheck_receipts.spi");
    let source = fs::read_to_string(&receipt).unwrap();
    fs::write(&receipt, format!("{source}inl injected () = invalid receipt\n")).unwrap();
    assert!(!fixture.check(true).status.success());
    assert_eq!(fixture.calls(), 3);
    assert!(fs::read_to_string(receipt).unwrap().contains("invalid receipt"));
}
#[test]
fn runtime_mutation_cannot_refresh_a_changed_dependency_graph() {
    let fixture = Fixture::new();
    // A one-module state package exposes accidental replacement of the graph receipt.
    fs::remove_file(fixture.root.join("state/core.spi")).unwrap();
    fs::write(fixture.root.join("state/package.spiproj"), "packageDir: ../src\npackages:\n    dep\nmodules:\n    prompt\n").unwrap();
    fixture.successful();
    let receipt = fixture.root.join("state/typecheck_receipts.spi");
    let before = fs::read(&receipt).unwrap();
    fs::write(fixture.root.join("src/dep/model.spi"), "inl changed () = unknown_identifier\n").unwrap();
    let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX)));
    let output = Command::new(binary).args(["agile", "begin"]).arg(&fixture.root).arg("Continue native cache validation").output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(before == fs::read(&receipt).unwrap(), "runtime mutation renewed a stale package graph");
    let output = fixture.check(false);
    assert!(!output.status.success(), "changed imports incorrectly passed without compiler attestation");
}

#[test]
fn task_updates_preserve_unrelated_drift_but_clean_updates_remain_compiler_free() {
    let fixture = Fixture::new();
    let manifest = fixture.root.join("state/package.spiproj");
    let source = fs::read_to_string(&manifest).unwrap();
    fs::write(manifest, format!("{source}    agile\n")).unwrap();
    let agile = fixture.root.join("state/agile.spi");
    fs::write(&agile, "union task_kind = | TaskKind :: task_kind\nunion priority = | P0 :: priority\nunion difficulty = | D1 :: difficulty\nunion task_status = | Active :: task_status\nunion agile_item = | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\ninl task () = Task (\"ID\", \"Task fixture\", TaskKind, P0, D1, 0u32, Active, \"\", \"\", \"\", \"\")\ninl main () : i32 = 0i32\n").unwrap();
    fixture.successful();
    let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX)));
    let update = || Command::new(&binary).args(["agile", "set"]).arg(&fixture.root).args(["ID", "50", "Active"]).output().unwrap();
    assert!(update().status.success());
    assert!(fixture.check(false).status.success(), "clean runtime mutation stopped working without a compiler");
    let source = fs::read_to_string(&agile).unwrap();
    fs::write(&agile, format!("{source}inl unrelated () = unknown_identifier\n")).unwrap();
    let receipt = fixture.root.join("state/typecheck_receipts.spi");
    let before = fs::read(&receipt).unwrap();
    assert!(update().status.success());
    assert!(before == fs::read(&receipt).unwrap(), "task update certified an unrelated source edit");
    assert!(!fixture.check(false).status.success(), "unrelated drift passed without compiler attestation");
}
#[test]
fn history_appends_preserve_unrelated_drift_but_clean_appends_remain_compiler_free() {
    let fixture = Fixture::new();
    let manifest = fixture.root.join("state/package.spiproj");
    let source = fs::read_to_string(&manifest).unwrap();
    fs::write(manifest, format!("{source}    history\n    ratings\n    bench\n    migration\n")).unwrap();
    for name in ["ratings", "bench"] { fs::write(fixture.root.join(format!("state/{name}.spi")), "inl main () : i32 = 0i32\n").unwrap(); }
    fs::write(fixture.root.join("state/migration.spi"), "union migration_denominator = | MigrationDenominator :: u32 * u32 * u32 * u32 * string -> migration_denominator\ninl denominator () : migration_denominator = MigrationDenominator (1u32, 1u32, 0u32, 1000u32, \"fixture\")\ninl main () : i32 = 0i32\n").unwrap();
    let history = fixture.root.join("state/history.spi");
    fs::write(&history, "union series_record = | SeriesRecord :: u32 * u32 * u32 * string * string * u64 * string * string * u64 * string -> series_record\ninl retired_module_count () : u32 = 0u32\ninl main () : i32 = 0i32\n").unwrap();
    fixture.successful();
    let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX)));
    let append = || Command::new(&binary).args(["agile", "record"]).arg(&fixture.root).output().unwrap();
    let output = append();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(fixture.check(false).status.success(), "clean history append stopped working without a compiler");
    let source = fs::read_to_string(&history).unwrap();
    fs::write(&history, format!("{source}inl unrelated () = unknown_identifier\n")).unwrap();
    let receipt = fixture.root.join("state/typecheck_receipts.spi");
    let before = fs::read(&receipt).unwrap();
    let output = append();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(before == fs::read(&receipt).unwrap(), "history append certified an unrelated source edit");
    assert!(!fixture.check(false).status.success(), "history drift passed without compiler attestation");
}
