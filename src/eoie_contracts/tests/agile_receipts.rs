use std::{env, fs, path::PathBuf, process::{Command, Output}, time::{SystemTime, UNIX_EPOCH}};

struct Backlog { root: PathBuf, binary: PathBuf }
impl Backlog {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie agile receipts ü {} {stamp}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
        });
        let backlog = Self { root, binary };
        backlog.ok("init", &[]);
        backlog.ok("add", &["ID-A", "TaskKind", "P1", "D3", "Title A", "eoie"]);
        backlog
    }
    fn agile(&self, verb: &str, rest: &[&str]) -> Output {
        Command::new(&self.binary).current_dir(env::temp_dir()).env_remove("EOIE_LEASE_ROOT").env_remove("EOIE_CONTROL_READ_ONLY")
            .env("EOIE_LEASE_POLICY", "renewing").env("EOIE_CONFIG_HOME", &self.root)
            .args(["agile", verb]).arg(&self.root).args(rest).output().unwrap()
    }
    fn ok(&self, verb: &str, rest: &[&str]) -> String {
        let output = self.agile(verb, rest);
        assert!(output.status.success(), "{verb} {rest:?}: {output:?}");
        String::from_utf8(output.stdout).unwrap()
    }
    fn state(&self) -> String { fs::read_to_string(self.root.join("state").join("agile.spi")).unwrap() }
}
impl Drop for Backlog { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.root); } }

fn receipt_shape(stdout: &str, prefix: &str) -> String {
    let line = stdout.lines().find(|line| line.starts_with(prefix)).unwrap_or_else(|| panic!("no {prefix:?} line in {stdout:?}"));
    line.split(' ').map(|token| match token.split_once('=') {
        Some((key, _)) if ["bytes_before", "bytes_after", "elapsed_ms"].contains(&key) => format!("{key}=N"),
        _ => token.to_owned(),
    }).collect::<Vec<_>>().join(" ")
}

#[test]
fn agile_set_receipt_names_the_update_and_rejections_leave_state_untouched() {
    let backlog = Backlog::new();
    let set = backlog.ok("set", &["ID-A", "40", "Active"]);
    assert_eq!(receipt_shape(&set, "eoie agile set "),
        "eoie agile set ok id=ID-A progress=40 status=Active validation=runtime-attested compiler=not-required bytes_before=N bytes_after=N elapsed_ms=N");
    assert!(backlog.ok("show", &["ID-A"]).lines().next().unwrap().contains("40%  Active"));
    let before = backlog.state();
    for (rest, message) in [(["ID-A", "140", "Active"], "progress must be 0..100"), (["ID-A", "40", "Sleeping"], "invalid agile status"), (["NO-SUCH", "10", "Active"], "agile task not found: NO-SUCH")] {
        let output = backlog.agile("set", &rest);
        assert!(!output.status.success(), "set {rest:?} was accepted");
        assert!(String::from_utf8_lossy(&output.stderr).contains(message), "{output:?}");
        assert_eq!(backlog.state(), before);
    }
}
