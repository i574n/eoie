use std::{env, fs, path::PathBuf, process::{Command, Output}, time::{SystemTime, UNIX_EPOCH}};

struct Fixture(PathBuf);
impl Fixture {
    fn new(source: &str) -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie agile list ü {} {stamp}", std::process::id()));
        fs::create_dir_all(root.join("state")).unwrap();
        fs::write(root.join("state/agile.spi"), source).unwrap();
        Self(root)
    }
    fn list(&self, status: Option<&str>) -> Output {
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
        });
        let mut command = Command::new(binary);
        command.args(["agile", "list"]).arg(&self.0);
        if let Some(status) = status { command.arg(status); }
        command.output().unwrap()
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

#[test]
fn agile_list_filters_exact_status_and_preserves_source() {
    let statuses = ["Planned", "Active", "Blocked", "Paused", "Done"];
    let rows = statuses.map(|status| format!("inl task_{status} () = Task (\"ID-{status}\", \"Active, Done and \\\"quoted\\\" text\", TaskKind, P0, D4, 0u32, {status}, \"\", \"tag\", \"current\", \"current\")"));
    let source = format!("// {}\n{}\ninl main () : i32 = 0i32\n", rows[1], rows.join("\n"));
    let fixture = Fixture::new(&source);
    let all = fixture.list(None);
    assert!(all.status.success(), "{}", String::from_utf8_lossy(&all.stderr));
    let all_stdout = String::from_utf8(all.stdout).unwrap();
    assert_eq!(all_stdout.lines().filter(|line| line.starts_with("ID-")).count(), 5);
    assert!(all_stdout.contains("shown=5 filter=all"), "{all_stdout}");
    for status in statuses {
        let output = fixture.list(Some(status));
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let stdout = String::from_utf8(output.stdout).unwrap();
        let listed = stdout.lines().filter(|line| line.starts_with("ID-")).collect::<Vec<_>>();
        assert_eq!(listed.len(), 1, "{stdout}");
        assert!(listed[0].starts_with(&format!("ID-{status} ")) && listed[0].contains(&format!(" {status:<8}")) && stdout.contains(&format!("shown=1 filter={status}")), "{stdout}");
    }
    assert_eq!(fs::read_to_string(fixture.0.join("state/agile.spi")).unwrap(), source);
}

#[test]
fn agile_list_rejects_invalid_filters_and_malformed_tasks() {
    let fixture = Fixture::new("inl broken () = Task (\"BROKEN\")\n");
    let invalid = fixture.list(Some("active"));
    assert!(!invalid.status.success());
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("invalid agile status filter"));
    let malformed = fixture.list(Some("Active"));
    assert!(!malformed.status.success());
    assert!(String::from_utf8_lossy(&malformed.stderr).contains("malformed Task fields"));
}
