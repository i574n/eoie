use std::{env, fs, path::PathBuf, process::{Command, Output}, time::{SystemTime, UNIX_EPOCH}};

const HEADER: &str = "union task_kind =\n    | Epic :: task_kind\n    | TaskKind :: task_kind\n\nunion agile_item =\n    | Task :: string * string * task_kind * u32 -> agile_item\n";

struct Fixture(PathBuf);
impl Fixture {
    fn new(label: &str, rows: &[&str]) -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie planning {label} {} {stamp}", std::process::id()));
        fs::create_dir_all(root.join("state")).unwrap();
        let mut source = String::from(HEADER);
        for row in rows { source.push_str(row); source.push('\n'); }
        source.push_str("inl main () : i32 = 0i32\n");
        fs::write(root.join("state/agile.spi"), source).unwrap();
        Self(root)
    }
    fn state(&self) -> String { fs::read_to_string(self.0.join("state/agile.spi")).unwrap() }
    fn agile(&self, verb: &str, rest: &[&str]) -> Output {
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
        });
        Command::new(binary).current_dir(env::temp_dir()).args(["agile", verb]).arg(&self.0).args(rest).output().unwrap()
    }
    fn ok(&self, verb: &str, rest: &[&str]) -> String {
        let output = self.agile(verb, rest);
        assert!(output.status.success(), "{verb} {rest:?}: {}", String::from_utf8_lossy(&output.stderr));
        String::from_utf8(output.stdout).unwrap()
    }
    fn ok_mode(&self, verb: &str, mode: &str) -> String {
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
        });
        let output = Command::new(binary).current_dir(env::temp_dir()).args(["agile", verb, mode]).arg(&self.0).output().unwrap();
        assert!(output.status.success(), "{verb} {mode}: {}", String::from_utf8_lossy(&output.stderr));
        String::from_utf8(output.stdout).unwrap()
    }
    fn rejected(&self, verb: &str, rest: &[&str]) -> String {
        let output = self.agile(verb, rest);
        assert!(!output.status.success(), "{verb} {rest:?} was accepted");
        String::from_utf8(output.stderr).unwrap()
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

fn task(id: &str, kind: &str, status: &str, deps: &str, tags: &str) -> String {
    format!("inl task_{} () = Task (\"{id}\", \"Title of {id}\", {kind}, P1, D3, 0u32, {status}, \"{deps}\", \"{tags}\", \"current\", \"current\")", id.to_lowercase().replace('-', "_"))
}

#[test]
fn planning_edit_validates_and_renders_typed_fields() {
    let a = task("ID-A", "TaskKind", "Planned", "", "eoie,agile");
    let fixture = Fixture::new("edit", &[&a]);
    let before = fixture.state();
    assert!(fixture.rejected("edit", &["ID-A", "estimate", "abc"]).contains("estimate must be"));
    assert!(fixture.rejected("edit", &["ID-A", "estimate", "20000h"]).contains("at most 999999 minutes"));
    assert!(fixture.rejected("edit", &["ID-A", "estimate", "-5"]).contains("estimate must be"));
    assert!(fixture.rejected("edit", &["ID-A", "load", "huge"]).contains("load must be"));
    assert!(fixture.rejected("edit", &["ID-A", "assignee", "bot"]).contains("assignee must be"));
    assert!(fixture.rejected("edit", &["NO-SUCH", "load", "heavy"]).contains("no task with this id"));
    assert_eq!(fixture.state(), before);
    assert!(fixture.ok("edit", &["ID-A", "estimate", "6h"]).contains("estimate=360m load=- assignee=-"));
    assert!(fixture.ok("edit", &["ID-A", "load", "Heavy"]).contains("load=heavy"));
    assert!(fixture.ok("edit", &["ID-A", "assignee", "pair"]).contains("estimate=360m load=heavy assignee=pair"));
    let state = fixture.state();
    assert_eq!(state.matches("union task_planning =").count(), 1, "{state}");
    assert!(state.contains("inl planning_of_id_a () = Planning (\"ID-A\", Minutes 360u32, Load Heavy, Assigned Pair)"), "{state}");
    assert!(state.find("-> agile_item").unwrap() < state.find("union task_load").unwrap());
    assert!(fixture.ok("show", &["ID-A"]).contains("planning: estimate=360m load=heavy assignee=pair"));
    assert!(fixture.ok("show", &["ID-A", "--json"]).contains("\"estimate_minutes\":360,\"load\":\"heavy\",\"assignee\":\"pair\",\"notes\":[],\"elapsed_ms\""));
    assert!(fixture.ok("edit", &["ID-A", "load", "none"]).contains("load=-"));
    assert!(fixture.state().contains("Planning (\"ID-A\", Minutes 360u32, LoadUnset, Assigned Pair)"));
}

#[test]
fn tag_migration_is_previewed_applied_once_and_keeps_rejected_tags() {
    let a = task("ID-A", "Epic", "Active", "", "eoie,est:300m,load:heavy,agent:yes");
    let b = task("ID-B", "TaskKind", "Planned", "ID-A", "est:abc,load:light,agent:no,kino");
    let c = task("ID-C", "TaskKind", "Done", "", "plain");
    let fixture = Fixture::new("migrate", &[&a, &b, &c]);
    let before = fixture.state();
    let preview = fixture.ok_mode("migrate-tags", "preview");
    assert!(preview.contains("ID-A moved=3 estimate=300m load=heavy assignee=agent tags=eoie"), "{preview}");
    assert!(preview.contains("ID-B moved=2 estimate=- load=light assignee=user tags=est:abc,kino rejected=est:abc"), "{preview}");
    assert!(preview.contains("eoie agile migrate-tags preview tasks=2 tags_moved=5 rejected=1"), "{preview}");
    assert_eq!(fixture.state(), before);
    assert!(fixture.ok_mode("migrate-tags", "apply").contains("eoie agile migrate-tags ok tasks=2"));
    let migrated = fixture.state();
    assert!(migrated.contains("\"eoie\", \"current\""), "{migrated}");
    assert!(migrated.contains("\"est:abc,kino\""), "{migrated}");
    assert!(migrated.contains("Planning (\"ID-B\", Unestimated, Load Light, Assigned User)"), "{migrated}");
    let again = fixture.ok_mode("migrate-tags", "apply");
    assert!(again.contains("ok unchanged tasks=0 tags_moved=0 rejected=1"), "{again}");
    assert_eq!(fixture.state(), migrated);
    let heavy = fixture.ok("list", &["--load", "heavy"]);
    assert!(heavy.contains("ID-A") && !heavy.contains("ID-B") && heavy.contains("shown=1 filter=all load=heavy assignee=all"), "{heavy}");
    let users = fixture.ok("list", &["Planned", "--assignee", "user", "--json"]);
    assert!(users.contains("\"id\":\"ID-B\"") && users.contains("\"estimate_minutes\":null,\"load\":\"light\",\"assignee\":\"user\"}"), "{users}");
    let table = fixture.ok("list", &[]);
    assert!(table.contains("EST    LOAD   WHO") && table.contains("300m   heavy  agent"), "{table}");
    assert!(fixture.rejected("list", &["--load", "huge"]).contains("invalid planning filter"));
    let plan = fixture.ok("plan", &[]);
    assert!(plan.contains("eoie agile plan ok tasks=3 open=2 estimated=1 minutes=300 open_minutes=300"), "{plan}");
    assert!(plan.lines().any(|line| line.starts_with("epic+deps") && line.contains("ID-A") && line.split_whitespace().nth(2) == Some("2")), "{plan}");
    assert!(fixture.ok("plan", &["--json"]).contains("{\"group\":\"load\",\"value\":\"heavy\",\"tasks\":1,\"open_tasks\":1,\"estimated\":1,\"minutes\":300,\"open_minutes\":300}"));
}
