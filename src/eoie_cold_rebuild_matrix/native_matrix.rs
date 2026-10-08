use eoie_cold_rebuild_matrix::{cold_rebuild_matrix_run, cold_rebuild_owner_run};
use std::{fs, path::PathBuf};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("eoie cold ü {} {stamp}", std::process::id()));
        let owner = root.join("src/owner with spaces");
        fs::create_dir_all(&owner).unwrap();
        fs::write(owner.join("Cargo.toml"), "[package]\nname = 'owner'\n[lib]\npath = 'generated.rs'\n[package.metadata.spiral]\nentry = 'chosen.spi'\noutput = 'generated.rs'\n").unwrap();
        fs::write(owner.join("package.spiproj"), "modules:\n    chosen\n").unwrap();
        fs::write(owner.join("chosen.spi"), "inl main () : i32 = 0i32\n").unwrap();
        fs::write(owner.join("generated.rs"), "// last validated output\n").unwrap();
        Self(root)
    }
    fn arguments(&self, compiler: &str) -> Vec<String> {
        ["cold-rebuild-matrix", self.0.to_str().unwrap(), ".cache/native matrix.tsv", "C:/tools with spaces/eoie.exe", compiler, "C:/dotnet SDK/dotnet.exe", "rustfmt", "45000"]
            .map(str::to_owned).to_vec()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}

#[test]
fn native_dll_plan_preserves_spaced_paths_and_uses_bounded_single_flight() {
    let fixture = Fixture::new();
    let arguments = fixture.arguments("C:/compiler cache/SpiralCompiler.dll");
    cold_rebuild_matrix_run(&arguments).unwrap();
    let text = fs::read_to_string(fixture.0.join(".cache/native matrix.tsv")).unwrap();
    let rows = text.lines().collect::<Vec<_>>();
    assert_eq!(rows.len(), 2, "one command for one declared owner");
    let cells = rows[1].split('\t').collect::<Vec<_>>();
    assert_eq!(cells.len(), 9);
    assert_eq!(cells[1], arguments[3]);
    let argv = process_batch_plan_domain::decode_matrix_arguments(cells[2]).unwrap();
    assert_eq!(&argv[..2], ["proxy", "cold-rebuild-owner"]);
    assert_eq!(argv[6], arguments[4]);
    assert_eq!(argv[7], arguments[5]);
    assert_eq!(argv[9], "45000");
    assert_eq!(argv[8], "");
    let staged = fixture.0.join(&argv[3]).join("workspace").join(&argv[4]);
    assert_eq!(fs::read(&staged).unwrap(), fs::read(fixture.0.join("src/owner with spaces/chosen.spi")).unwrap());
    #[cfg(windows)]
    assert!(!argv[2].starts_with(r"\\?\"), "verbatim paths are not compiler URI inputs");
    assert_eq!(fs::read_to_string(fixture.0.join("src/owner with spaces/generated.rs")).unwrap(), "// last validated output\n");
    assert!(cells[8].starts_with(".cache/"));
}

#[test]
fn invalid_budget_or_escaping_plan_path_is_rejected_without_publication() {
    let fixture = Fixture::new();
    for invalid in ["0", "-1", "600001", "invalid"] {
        let mut arguments = fixture.arguments("SpiralCompiler.exe");
        arguments[7] = invalid.to_owned();
        assert!(cold_rebuild_matrix_run(&arguments).is_err());
    }
    let mut arguments = fixture.arguments("SpiralCompiler.exe");
    arguments[2] = "../escape.tsv".to_owned();
    assert!(cold_rebuild_matrix_run(&arguments).is_err());
    assert!(!fixture.0.join(".cache/native matrix.tsv").exists());
}

#[test]
fn cold_plan_does_not_require_a_previous_generated_file() {
    let fixture = Fixture::new();
    let output = fixture.0.join("src/owner with spaces/generated.rs");
    fs::remove_file(&output).unwrap();
    cold_rebuild_matrix_run(&fixture.arguments("SpiralCompiler.dll")).unwrap();
    assert!(!output.exists(), "planning must leave missing outputs absent");
    let plan = fs::read_to_string(fixture.0.join(".cache/native matrix.tsv")).unwrap();
    assert_eq!(plan.lines().count(), 2);
}

fn fake_compiler() -> &'static PathBuf {
    static COMPILER: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    COMPILER.get_or_init(|| {
        let root = std::env::temp_dir().join(format!("eoie cold compiler {}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let source = root.join("compiler.rs");
        fs::write(&source, r#"
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input = std::path::Path::new(&args[args.len() - 2]);
    let output = std::path::Path::new(&args[args.len() - 1]);
    let mode = std::fs::read_to_string(input).unwrap();
    std::fs::write(input.with_extension("rs"), "// partial compiler sidecar").unwrap();
    if mode.contains("TIMEOUT") { std::thread::sleep(std::time::Duration::from_secs(5)); }
    std::fs::write(output, if mode.contains("MOD") { "mod child;\r\nfn main() {}\r\n" } else { "fn main() {}\r\n" }).unwrap();
    if mode.contains("FAIL") { std::process::exit(5); }
}
"#).unwrap();
        let executable = root.join(format!("SpiralCompiler{}", std::env::consts::EXE_SUFFIX));
        let output = std::process::Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
            .arg(source).arg("-o").arg(&executable).output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        executable
    })
}

fn planned_owner(fixture: &Fixture) -> Vec<String> {
    cold_rebuild_matrix_run(&fixture.arguments(fake_compiler().to_str().unwrap())).unwrap();
    let plan = fs::read_to_string(fixture.0.join(".cache/native matrix.tsv")).unwrap();
    let cell = plan.lines().nth(1).unwrap().split('\t').nth(2).unwrap();
    process_batch_plan_domain::decode_matrix_arguments(cell).unwrap()[1..].to_vec()
}

#[test]
fn cold_owner_contains_sidecars_and_publishes_normalized_output() {
    for existing_adapter in [false, true] {
        let fixture = Fixture::new();
        let sidecar = fixture.0.join("src/owner with spaces/chosen.rs");
        if existing_adapter { fs::write(&sidecar, "// handwritten adapter\n").unwrap(); }
        let args = planned_owner(&fixture);
        cold_rebuild_owner_run(&args).unwrap();
        let output = fixture.0.join("src/owner with spaces/generated.rs");
        assert_eq!(fs::read_to_string(&output).unwrap(), "fn main() {}\n");
        if existing_adapter { assert_eq!(fs::read_to_string(&sidecar).unwrap(), "// handwritten adapter\n"); }
        else { assert!(!sidecar.exists()); }
        let modified = fs::metadata(&output).unwrap().modified().unwrap();
        cold_rebuild_owner_run(&args).unwrap();
        assert_eq!(fs::metadata(output).unwrap().modified().unwrap(), modified, "identical replay changed output timestamp");
    }
}

#[test]
fn cold_owner_formats_output_with_out_of_line_modules() {
    let fixture = Fixture::new();
    fs::write(fixture.0.join("src/owner with spaces/chosen.spi"), "MOD").unwrap();
    fs::write(fixture.0.join("src/rustfmt.toml"), "edition = \"2024\"\n").unwrap();
    let mut args = planned_owner(&fixture);
    args[7] = "rustfmt".to_owned();
    cold_rebuild_owner_run(&args).unwrap();
    assert_eq!(fs::read_to_string(fixture.0.join("src/owner with spaces/generated.rs")).unwrap(), "mod child;\nfn main() {}\n");
}

#[test]
fn failed_or_timed_out_cold_owner_preserves_previous_output() {
    for mode in ["FAIL", "TIMEOUT"] {
        let fixture = Fixture::new();
        fs::write(fixture.0.join("src/owner with spaces/chosen.spi"), mode).unwrap();
        let mut args = planned_owner(&fixture);
        if mode == "TIMEOUT" { args[8] = "100".to_owned(); }
        assert!(cold_rebuild_owner_run(&args).is_err());
        assert_eq!(fs::read_to_string(fixture.0.join("src/owner with spaces/generated.rs")).unwrap(), "// last validated output\n");
        assert!(!fixture.0.join("src/owner with spaces/chosen.rs").exists());
    }
}

#[test]
fn cold_owner_rejects_source_and_output_changes_after_planning() {
    for name in ["chosen.spi", "generated.rs"] {
        let fixture = Fixture::new();
        let args = planned_owner(&fixture);
        let changed = fixture.0.join("src/owner with spaces").join(name);
        fs::write(&changed, "concurrent edit").unwrap();
        assert!(cold_rebuild_owner_run(&args).unwrap_err().contains("changed since cold planning"));
        assert_eq!(fs::read_to_string(changed).unwrap(), "concurrent edit");
        assert!(!fixture.0.join("src/owner with spaces/chosen.rs").exists());
    }
}
