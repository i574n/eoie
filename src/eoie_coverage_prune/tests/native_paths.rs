use std::{env, fs, path::PathBuf, process::{Command, Output}, time::{SystemTime, UNIX_EPOCH}};

struct Fixture { root: PathBuf, binary: PathBuf, cargo: String }
impl Fixture {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie coverage paths ü {} {stamp}", std::process::id()));
        fs::create_dir_all(root.join("project/src")).unwrap();
        fs::create_dir_all(root.join("tools")).unwrap();
        fs::create_dir_all(root.join("profiles ü")).unwrap();
        fs::write(root.join("project/src/Cargo.toml"), "[workspace]\n").unwrap();
        fs::write(root.join("tools/compiler.dll"), "unused fake compiler").unwrap();
        fs::write(root.join("profiles ü/previous.profraw"), "previous profile").unwrap();
        let source = root.join("fake_cargo.rs");
        fs::write(&source, r#"use std::{env,fs,path::Path};
fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args[1] == "help" {
        if Path::new("skip-profile").exists() { return; }
        let pattern = env::var("LLVM_PROFILE_FILE").unwrap();
        fs::write(Path::new(&pattern).parent().unwrap().join("current.profraw"), "new profile").unwrap();
        return;
    }
    let target = env::var("CARGO_TARGET_DIR").unwrap();
    for key in ["CARGO_TARGET_DIR", "EOIE_SPIRAL_COMPILE", "EOIE_BIN_UNDER_TEST", "LLVM_PROFILE_FILE"] {
        assert!(Path::new(&env::var(key).unwrap()).is_absolute(), "relative environment: {key}");
    }
    assert!(env::var_os("CARGO_ENCODED_RUSTFLAGS").is_none());
    assert_eq!(env::var("RUSTFLAGS").unwrap(), "-C instrument-coverage --cfg eoie_coverage");
    let jobs = &args[args.iter().position(|arg| arg == "--jobs").unwrap()+1];
    assert_eq!(&env::var("CARGO_BUILD_JOBS").unwrap(), jobs);
    assert_eq!(&env::var("RUST_TEST_THREADS").unwrap(), jobs);
    let release = Path::new(&target).join("release");
    fs::create_dir_all(&release).unwrap();
    if args[1] == "build" {
        fs::copy(env::current_exe().unwrap(), release.join(format!("eoie{}", env::consts::EXE_SUFFIX))).unwrap();
    } else {
        assert_eq!(args[1], "test");
        let pattern = env::var("LLVM_PROFILE_FILE").unwrap();
        fs::write(Path::new(&pattern).parent().unwrap().join("current.profraw"), "new profile").unwrap();
        fs::write(Path::new(&target).join("observed.txt"), format!("jobs={jobs}\ncwd={}\n", env::current_dir().unwrap().display())).unwrap();
    }
}"#).unwrap();
        let cargo = format!("tools/fake-cargo{}", env::consts::EXE_SUFFIX);
        let compiled = Command::new("rustc").arg(&source).arg("-o").arg(root.join(&cargo)).output().unwrap();
        assert!(compiled.status.success(), "{}", String::from_utf8_lossy(&compiled.stderr));
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
        });
        Self { root, binary, cargo }
    }
    fn run(&self, jobs: Option<&str>, scope: &str) -> Output {
        let mut command = Command::new(&self.binary);
        command.current_dir(&self.root).args(["proxy", "coverage-run", "project", &self.cargo, "tools/compiler.dll", "output with spaces", "profiles ü", scope, "5000"])
            .env_remove("EOIE_COVERAGE_JOBS").env_remove("CARGO_BUILD_JOBS").env_remove("RUST_TEST_THREADS")
            .env_remove("EOIE_COVERAGE_SMOKE_CHILD").env("CARGO_ENCODED_RUSTFLAGS", "must-not-override-coverage");
        if let Some(jobs) = jobs { command.env("EOIE_COVERAGE_JOBS", jobs); }
        command.output().unwrap()
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.root); } }

#[test]
fn native_coverage_resolves_relative_paths_before_changing_directory() {
    let fixture = Fixture::new();
    let output = fixture.run(Some("2"), "workspace");
    assert!(output.status.success(), "{output:?}");
    assert!(fixture.root.join("profiles ü/current.profraw").is_file());
    assert!(fixture.root.join("profiles ü/preflight/current.profraw").is_file());
    assert!(!fixture.root.join("profiles ü/previous.profraw").exists());
    assert!(!fixture.root.join("project/src/output with spaces").exists());
    let observed = fs::read_to_string(fixture.root.join("output with spaces/observed.txt")).unwrap();
    assert!(observed.starts_with("jobs=2\n"));
}

#[test]
fn native_coverage_rejects_bad_configuration_before_clearing_profiles() {
    let fixture = Fixture::new();
    for (jobs, scope) in [("0", "workspace"), ("-1", "workspace"), ("257", "workspace"), ("many", "workspace"), ("2", "invalid-scope")] {
        let output = fixture.run(Some(jobs), scope);
        assert!(!output.status.success(), "{output:?}");
        assert_eq!(fs::read_to_string(fixture.root.join("profiles ü/previous.profraw")).unwrap(), "previous profile");
        assert!(!fixture.root.join("output with spaces").exists());
    }
}

#[test]
fn native_coverage_default_is_bounded_and_explicit_jobs_are_validated() {
    use eoie_coverage_prune::eoie_coverage_jobs;
    assert_eq!(eoie_coverage_jobs(0, 1), 1);
    assert_eq!(eoie_coverage_jobs(0, 64), 2);
    assert_eq!(eoie_coverage_jobs(1, 64), 1);
    assert_eq!(eoie_coverage_jobs(56, 64), 56);
    assert_eq!(eoie_coverage_jobs(256, 64), 256);
    assert_eq!(eoie_coverage_jobs(257, 64), -1);
    assert_eq!(eoie_coverage_jobs(-1, 64), -1);
}

#[test]
fn native_coverage_rejects_a_binary_that_does_not_write_profiles() {
    let fixture = Fixture::new();
    fs::write(fixture.root.join("project/skip-profile"), "simulate missing instrumentation").unwrap();
    let output = fixture.run(Some("2"), "workspace");
    assert!(!output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("no nonempty profile"));
    assert!(!fixture.root.join("output with spaces/observed.txt").exists());
}

#[test]
fn native_smoke_rejects_uncertified_sources_before_mutating_outputs() {
    let fixture = Fixture::new();
    let output = fixture.run(Some("2"), "smoke");
    assert!(!output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("valid strict EOIE release"), "{output:?}");
    assert_eq!(fs::read_to_string(fixture.root.join("profiles ü/previous.profraw")).unwrap(), "previous profile");
    assert!(!fixture.root.join("output with spaces").exists());
}
