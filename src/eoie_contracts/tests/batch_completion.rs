use std::{env, fs, path::PathBuf, process::{Command, Output}, time::{SystemTime, UNIX_EPOCH}};

struct Fixture { root: PathBuf, binary: PathBuf }
impl Fixture {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie batch completion ü {} {stamp}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
        });
        Self { root, binary }
    }
    fn run(&self, slice: &[&str], fail: bool) -> Output {
        let second = if fail { "not-a-command" } else { "help" };
        let rows = [("first", "help"), ("second", second)].map(|(id, arg)| {
            format!("{id}\t{}\tjson:[\"{arg}\"]\t.\t\tpass\tstop\t\t{id}.txt", self.binary.display())
        });
        fs::write(self.root.join("matrix.tsv"), format!("id\tprogram\targs\tcwd\tenv\trequired_status\ton_error\tdepends_on\treceipt\n{}\n", rows.join("\n"))).unwrap();
        Command::new(&self.binary).current_dir(&self.root).env_remove("EOIE_LEASE_ROOT").env_remove("EOIE_CONTROL_READ_ONLY")
            .args(["proxy", "batch-plan"]).arg(&self.root)
            .args(["matrix.tsv", "5000", "report.tsv", "1"]).args(slice).output().unwrap()
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.root); } }

fn summary(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).lines()
        .find(|line| line.starts_with("eoie proxy batch-plan "))
        .unwrap_or_else(|| panic!("missing summary: {output:?}")).to_owned()
}

#[test]
fn batch_completion_distinguishes_full_manual_run_from_slices() {
    let fixture = Fixture::new();
    let full = fixture.run(&[], false);
    assert!(full.status.success(), "{full:?}");
    assert!(summary(&full).contains(" auto=0 complete=1 "));
    for slice in [["0", "1"], ["1", "1"]] {
        let output = fixture.run(&slice, false);
        assert!(output.status.success(), "{output:?}");
        assert!(summary(&output).contains(" auto=0 complete=0 "));
    }
}

#[test]
fn batch_completion_rejects_a_failed_full_manual_run() {
    let fixture = Fixture::new();
    let output = fixture.run(&[], true);
    assert!(!output.status.success(), "{output:?}");
    assert!(summary(&output).contains(" auto=0 complete=0 "));
}
