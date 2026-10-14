use std::{env, fs, path::PathBuf, process::{Command, Output, Stdio}, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

struct Backlog { root: PathBuf, binary: PathBuf }
impl Backlog {
    fn new(label: &str) -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie agile notes ü {label} {} {stamp}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
        });
        let backlog = Self { root, binary };
        backlog.ok("init", &[]);
        for id in ["ID-A", "ID-B", "ID-C"] { backlog.ok("add", &[id, "TaskKind", "P1", "D3", &format!("Title of {id}"), "eoie"]); }
        backlog.ok("edit", &["ID-B", "deps", "ID-A"]);
        backlog.ok("edit", &["ID-C", "title", "follow-up to ID-A"]);
        backlog
    }
    fn command(&self, verb: &str) -> Command {
        let mut command = Command::new(&self.binary);
        command.current_dir(env::temp_dir()).env_remove("EOIE_LEASE_ROOT").env_remove("EOIE_CONTROL_READ_ONLY").env_remove("EOIE_AGILE_TEST_PAUSE_MS")
            .env("EOIE_LEASE_POLICY", "renewing").env("EOIE_CONFIG_HOME", &self.root).args(["agile", verb]);
        command
    }
    fn agile(&self, verb: &str, rest: &[&str]) -> Output { self.command(verb).arg(&self.root).args(rest).output().unwrap() }
    fn from_title(&self, id: &str) -> Output { self.command("notes").arg("from-title").arg(&self.root).arg(id).output().unwrap() }
    fn ok(&self, verb: &str, rest: &[&str]) -> String {
        let output = self.agile(verb, rest);
        assert!(output.status.success(), "{verb} {rest:?}: {output:?}");
        String::from_utf8(output.stdout).unwrap()
    }
    fn state_path(&self) -> PathBuf { self.root.join("state").join("agile.spi") }
    fn state(&self) -> String { fs::read_to_string(self.state_path()).unwrap() }
    fn rejected(&self, output: Output, message: &str, before: &str) {
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(String::from_utf8_lossy(&output.stderr).contains(message), "{output:?}");
        assert_eq!(self.state(), before);
    }
}
impl Drop for Backlog { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.root); } }

fn receipt_shape(stdout: &str, prefix: &str) -> String {
    let line = stdout.lines().find(|line| line.starts_with(prefix)).unwrap_or_else(|| panic!("no {prefix:?} line in {stdout:?}"));
    line.split(' ').map(|token| match token.split_once('=') {
        Some((key, _)) if ["bytes_before", "bytes_after", "elapsed_ms", "at"].contains(&key) => format!("{key}=N"),
        _ => token.to_owned(),
    }).collect::<Vec<_>>().join(" ")
}

fn utc_stamp(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 17 && bytes[4] == b'-' && bytes[7] == b'-' && bytes[10] == b'T' && bytes[13] == b':' && bytes[16] == b'Z'
}

#[test]
fn agile_note_appends_dated_notes_shown_under_the_title_and_in_json() {
    let backlog = Backlog::new("note");
    let first = backlog.ok("note", &["ID-A", "first finding", "--as", "lane 2"]);
    assert_eq!(receipt_shape(&first, "eoie agile note "),
        "eoie agile note ok id=ID-A note=1 at=N author=\"lane 2\" text_bytes=13 bytes_before=N bytes_after=N elapsed_ms=N");
    let stamp = first.split(" at=").nth(1).unwrap().split(' ').next().unwrap().to_owned();
    assert!(utc_stamp(&stamp), "{stamp}");
    let second = backlog.ok("note", &["ID-A", "a \"quoted\" path C:\\x"]);
    assert_eq!(receipt_shape(&second, "eoie agile note "),
        "eoie agile note ok id=ID-A note=2 at=N author=\"user\" text_bytes=20 bytes_before=N bytes_after=N elapsed_ms=N");
    let show = backlog.ok("show", &["ID-A"]);
    let lines: Vec<&str> = show.lines().collect();
    let title = lines.iter().position(|line| *line == "Title of ID-A").unwrap();
    assert_eq!(lines[title + 1], "notes (2)");
    assert!(lines[title + 2].ends_with("  lane 2  first finding") && lines[title + 2].starts_with(&stamp), "{show}");
    assert!(lines[title + 3].ends_with("  user  a \"quoted\" path C:\\x"), "{show}");
    let json = backlog.ok("show", &["ID-A", "--json"]);
    assert!(json.contains(&format!("\"notes\":[{{\"at\":\"{stamp}\",\"author\":\"lane 2\",\"text\":\"first finding\"}},")), "{json}");
    assert!(backlog.ok("show", &["ID-B", "--json"]).contains("\"notes\":[]"));
    assert!(!backlog.ok("show", &["ID-B"]).contains("notes ("));
    let before = backlog.state();
    assert!(before.contains("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"));
    backlog.rejected(backlog.agile("note", &["ID-A", "   "]), "the note text is empty", &before);
    backlog.rejected(backlog.agile("note", &["NO-SUCH", "text"]), "no task with this id", &before);
    backlog.rejected(backlog.agile("note", &["ID-A", "text", "--by", "x"]), "usage: agile note <root> <id> <text> [--as <author>]", &before);
}

#[test]
fn notes_from_title_moves_each_tail_into_a_dated_note() {
    let backlog = Backlog::new("from-title");
    backlog.ok("note", &["ID-A", "kept note"]);
    let title = "base work; 2026-10-13 22:10 found the gap; 23:05 fixed it; plain tail";
    backlog.ok("edit", &["ID-A", "title", title]);
    let output = backlog.from_title("ID-A");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(receipt_shape(&String::from_utf8(output.stdout).unwrap(), "eoie agile notes "),
        format!("eoie agile notes ok id=ID-A moved=3 title_bytes_before={} title_bytes_after=9 bytes_before=N bytes_after=N elapsed_ms=N", title.len()));
    let show = backlog.ok("show", &["ID-A"]);
    let lines: Vec<&str> = show.lines().collect();
    let at = lines.iter().position(|line| *line == "base work").unwrap();
    assert_eq!(lines[at + 1], "notes (4)");
    assert!(lines[at + 2].ends_with("  user  kept note"));
    assert_eq!(lines[at + 3], "2026-10-13 22:10  migrated  found the gap");
    assert_eq!(lines[at + 4], "2026-10-13 23:05  migrated  fixed it");
    assert_eq!(lines[at + 5], "2026-10-13 23:05  migrated  plain tail");
    let before = backlog.state();
    backlog.rejected(backlog.from_title("ID-A"), "the title has no \"; \" tail to move", &before);
    backlog.rejected(backlog.from_title("NO-SUCH"), "no task with this id", &before);
}

#[test]
fn rename_rewrites_the_task_its_planning_notes_and_every_dependency() {
    let backlog = Backlog::new("rename");
    backlog.ok("note", &["ID-A", "moves with the task"]);
    backlog.ok("edit", &["ID-A", "estimate", "2h"]);
    let renamed = backlog.ok("rename", &["ID-A", "ID-Z"]);
    assert_eq!(receipt_shape(&renamed, "eoie agile rename "),
        "eoie agile rename ok old=ID-A new=ID-Z items=3 deps_rewritten=1 deps_in=ID-B titles_mentioning_old=1 titles_in=ID-C bytes_before=N bytes_after=N elapsed_ms=N");
    let state = backlog.state();
    assert!(state.contains("inl id_z () = Task (\"ID-Z\",") && state.contains("inl planning_of_id_z () = Planning (\"ID-Z\",") && state.contains("inl note_of_id_z_1 () = Note (\"ID-Z\", 1u32,"));
    assert!(!state.contains("(\"ID-A\","));
    assert!(backlog.ok("show", &["ID-B"]).lines().any(|line| line == "deps: ID-Z"));
    let shown = backlog.ok("show", &["ID-Z"]);
    assert!(shown.contains("planning: estimate=120m") && shown.contains("  user  moves with the task"), "{shown}");
    assert!(backlog.ok("show", &["ID-C"]).contains("follow-up to ID-A"));
    let before = backlog.state();
    backlog.rejected(backlog.agile("rename", &["ID-Z", "ID-B"]), "the id ID-B is already taken", &before);
    backlog.rejected(backlog.agile("rename", &["ID-Z", "id-lower"]), "a task id is upper-case letters, digits and -", &before);
    backlog.rejected(backlog.agile("rename", &["NO-SUCH", "ID-Y"]), "no task with this id", &before);
    backlog.rejected(backlog.agile("rename", &["ID-Z", "ID-Z"]), "the new id equals the old one", &before);
    backlog.rejected(backlog.agile("rename", &["ID-Z", "MAIN"]), "the name inl main is already declared", &before);
    backlog.rejected(backlog.agile("rename", &["ID-Z", "2FA-LOGIN"]), "the new id must start with a letter", &before);
}

#[test]
fn a_write_from_outside_between_read_and_commit_wins_and_the_rename_writes_nothing() {
    let backlog = Backlog::new("concurrent");
    let child = backlog.command("rename").env("EOIE_AGILE_TEST_PAUSE_MS", "1500").arg(&backlog.root).args(["ID-A", "ID-Z"])
        .stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let marker = backlog.root.join("state").join("agile.spi.paused");
    let waited = SystemTime::now();
    while !marker.exists() {
        assert!(waited.elapsed().unwrap() < Duration::from_secs(30), "the rename never reached its pause after the read");
        thread::sleep(Duration::from_millis(20));
    }
    let outside = format!("{}inl outside_marker () = 0i32\n", backlog.state());
    fs::write(backlog.state_path(), &outside).unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("state/agile.spi changed since it was read; nothing written, run the call again"), "{output:?}");
    assert_eq!(backlog.state(), outside);
}

#[test]
fn an_expired_lease_blocks_note_from_title_and_rename_alike() {
    let backlog = Backlog::new("lease");
    backlog.ok("edit", &["ID-A", "title", "base; 22:10 tail"]);
    fs::write(backlog.root.join("state").join("prompt.spi"), "union lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\ninl current () : prompt_lease = PromptLease (\"fixture\", \"expired effect lease\", 3000000u64, 600000u64, 1000u64, 3601000u64, LeaseActive)\ninl main () : i32 = 0i32\n").unwrap();
    let before = backlog.state();
    let strict = |verb: &str| { let mut command = backlog.command(verb); command.env_remove("EOIE_LEASE_POLICY"); command };
    let note = strict("note").arg(&backlog.root).args(["ID-A", "text"]).output().unwrap();
    let from_title = strict("notes").arg("from-title").arg(&backlog.root).arg("ID-A").output().unwrap();
    let rename = strict("rename").arg(&backlog.root).args(["ID-A", "ID-Z"]).output().unwrap();
    for output in [note, from_title, rename] {
        assert!(!output.status.success(), "{output:?}");
        assert!(String::from_utf8_lossy(&output.stderr).contains("lease authority blocked mutating effect"), "{output:?}");
        assert_eq!(backlog.state(), before);
    }
}
#[test]
fn agile_add_rejects_an_id_that_starts_with_a_digit() {
    let backlog = Backlog::new("add-digit");
    let before = backlog.state();
    backlog.rejected(backlog.agile("add", &["2FA-LOGIN", "TaskKind", "P1", "D3", "Two factor", "eoie"]), "agile add 2FA-LOGIN rejected: the id must start with a letter, so its inl name is a valid identifier", &before);
    let added = backlog.ok("add", &["FA2-LOGIN", "TaskKind", "P1", "D3", "Two factor", "eoie"]);
    assert!(added.starts_with("eoie agile add ok id=FA2-LOGIN binding=fa2_login "), "{added}");
}