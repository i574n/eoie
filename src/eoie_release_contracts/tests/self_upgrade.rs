use std::{env, fs, path::PathBuf, process::{Command, Output}, time::{SystemTime, UNIX_EPOCH}};

struct Fixture { root: PathBuf, binary: PathBuf, native_name: String }
impl Fixture {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie upgrade ü {} {stamp}", std::process::id()));
        for name in ["current", "candidate"] { fs::create_dir_all(root.join(name)).unwrap(); }
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
        });
        let native_name = format!("eoie{}", env::consts::EXE_SUFFIX);
        fs::write(root.join("verifier.rs"), r#"
use std::{env, fs::OpenOptions, io::Write};
fn main() {
    assert_eq!(env::var("EOIE_CONTROL_READ_ONLY").unwrap(), "1");
    let exe = env::current_exe().unwrap();
    let role = exe.parent().unwrap().file_name().unwrap().to_str().unwrap();
    let args: Vec<_> = env::args().skip(1).collect();
    let mut log = OpenOptions::new().create(true).append(true).open(env::var_os("EOIE_UPGRADE_LOG").unwrap()).unwrap();
    writeln!(log, "{}|{}", role, args.join("|")).unwrap();
    if role == "candidate" && args[0] == "status" && env::var_os("EOIE_UPGRADE_FAIL").is_some() { std::process::exit(7); }
}
"#).unwrap();
        let output = Command::new("rustc").arg(root.join("verifier.rs")).arg("-o").arg(root.join("current").join(&native_name)).output().unwrap();
        assert!(output.status.success(), "{output:?}");
        fs::copy(root.join("current").join(&native_name), root.join("candidate").join(&native_name)).unwrap();
        Self { root, binary, native_name }
    }
    fn run(&self, fail: bool) -> Output {
        let mut command = Command::new(&self.binary);
        command.args(["proxy", "self-upgrade-check"]).arg(self.root.join("current")).arg(self.root.join("candidate"))
            .env("EOIE_UPGRADE_LOG", self.root.join("gates.txt")).env_remove("EOIE_UPGRADE_FAIL");
        if fail { command.env("EOIE_UPGRADE_FAIL", "1"); }
        command.output().unwrap()
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.root); } }

#[test]
fn native_upgrade_preserves_all_six_strict_gates_and_propagates_failure() {
    let fixture = Fixture::new();
    let output = fixture.run(false);
    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("gates=6 compatibility=strict"));
    let mut expected = Vec::new();
    for (role, root) in [("current", "current"), ("current", "candidate"), ("candidate", "candidate")] {
        let path = fs::canonicalize(fixture.root.join(root)).unwrap();
        expected.push(format!("{role}|bundle|check|{}|eoie", path.display()));
        expected.push(format!("{role}|status|{}", path.display()));
    }
    assert_eq!(fs::read_to_string(fixture.root.join("gates.txt")).unwrap().lines().collect::<Vec<_>>(), expected);
    fs::remove_file(fixture.root.join("gates.txt")).unwrap();
    let output = fixture.run(true);
    assert!(!output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("candidate-self-verifier gate failed"));
    assert_eq!(fs::read_to_string(fixture.root.join("gates.txt")).unwrap().lines().count(), 6);
}

#[test]
fn native_upgrade_rejects_ambiguous_missing_directory_and_linked_binaries_before_launch() {
    let fixture = Fixture::new();
    let native = fixture.root.join("candidate").join(&fixture.native_name);
    let alternate = fixture.root.join("candidate").join(if fixture.native_name == "eoie" { "eoie.exe" } else { "eoie" });
    fs::copy(&native, &alternate).unwrap();
    let output = fixture.run(false);
    assert!(!output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("exactly one"));
    fs::remove_file(&alternate).unwrap();
    fs::remove_file(&native).unwrap();
    assert!(!fixture.run(false).status.success());
    fs::create_dir(&native).unwrap();
    assert!(!fixture.run(false).status.success());
    fs::remove_dir(&native).unwrap();
    let source = fixture.root.join("current").join(&fixture.native_name);
    #[cfg(windows)] match std::os::windows::fs::symlink_file(&source, &native) {
        Ok(()) => (),
        Err(error) if error.raw_os_error() == Some(1314) => eprintln!("symlink privilege unavailable; missing, ambiguous and directory rejection verified"),
        Err(error) => panic!("{error}"),
    }
    #[cfg(unix)] std::os::unix::fs::symlink(&source, &native).unwrap();
    assert!(!fixture.run(false).status.success());
    assert!(!fixture.root.join("gates.txt").exists());
}
