use eoie_coverage_prune::prune_uncovered;
use std::{env, fs, path::{Path, PathBuf}, process::Command, time::{SystemTime, UNIX_EPOCH}};

const LEDGER: &str = "union prune_receipt = | PruneReceipt :: string * string * string * string -> prune_receipt\ninl main () : i32 = 0i32\n";
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie prune ü {} {stamp}", std::process::id()));
        fs::create_dir_all(root.join("state")).unwrap();
        fs::write(root.join("candidate.rs"), "original source").unwrap();
        fs::write(root.join("zero.lcov"), "SF:candidate.rs\nDA:1,0\nend_of_record\n").unwrap();
        Self(root)
    }
    fn ledger(&self) -> PathBuf { self.0.join("state/prune_receipts.spi") }
    fn run(&self, program: &str, mode: &str) -> Result<(), String> {
        let profile = self.0.join("zero.lcov");
        prune_uncovered(&["prune-uncovered", self.0.to_str().unwrap(), "candidate.rs", profile.to_str().unwrap(), profile.to_str().unwrap(), "--", program, mode].map(str::to_owned))
    }
    fn stages(&self) -> Vec<PathBuf> {
        fs::read_dir(&self.0).unwrap().map(|entry| entry.unwrap().path()).filter(|path| path.file_name().unwrap().to_string_lossy().starts_with(".eoie-prune-")).collect()
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

#[test]
fn unreadable_or_malformed_prune_ledgers_leave_the_source_in_place() {
    let fixture = Fixture::new();
    for ledger in [None, Some("no main marker"), Some("inl main () : i32 = 0i32\ninl main () : i32 = 0i32\n")] {
        if let Some(text) = ledger { fs::write(fixture.ledger(), text).unwrap(); }
        let error = fixture.run("eoie-validator-must-not-run", "unused").unwrap_err();
        assert!(!error.contains("validation failed"), "{error}");
        assert_eq!(fs::read_to_string(fixture.0.join("candidate.rs")).unwrap(), "original source");
        assert!(fixture.stages().is_empty());
    }
    fs::remove_file(fixture.ledger()).unwrap();
    fs::create_dir(fixture.ledger()).unwrap();
    assert!(fixture.run("eoie-validator-must-not-run", "unused").is_err());
    assert!(fixture.0.join("candidate.rs").is_file());
    assert!(fixture.stages().is_empty());
}

fn compile_validator(root: &Path) -> PathBuf {
    let source = root.join("validator.rs");
    fs::write(&source, r#"fn main() {
        match std::env::args().nth(1).as_deref().unwrap() {
            "fail" => std::process::exit(7),
            "recreate" => { std::fs::write("candidate.rs", "concurrent source").unwrap(); std::process::exit(7); }
            "ledger-change" => { std::fs::write("state/prune_receipts.spi", "concurrent ledger").unwrap(); }
            _ => panic!("unexpected mode"),
        }
    }"#).unwrap();
    let binary = root.join(format!("validator{}", env::consts::EXE_SUFFIX));
    let output = Command::new("rustc").arg(&source).arg("-o").arg(&binary).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    binary
}

#[test]
fn failed_prunes_restore_sources_and_preserve_concurrent_edits() {
    let tools = Fixture::new();
    let validator = compile_validator(&tools.0);
    for mode in ["fail", "recreate", "ledger-change"] {
        let fixture = Fixture::new();
        fs::write(fixture.ledger(), LEDGER).unwrap();
        let old_backup = fixture.0.join(format!("candidate.eoie-prune-{}", std::process::id()));
        fs::write(&old_backup, "older recovery copy").unwrap();
        let error = fixture.run(validator.to_str().unwrap(), mode).unwrap_err();
        assert_eq!(fs::read_to_string(old_backup).unwrap(), "older recovery copy");
        if mode == "recreate" {
            assert!(error.contains("rollback incomplete"), "{error}");
            assert_eq!(fs::read_to_string(fixture.0.join("candidate.rs")).unwrap(), "concurrent source");
            let stages = fixture.stages();
            assert_eq!(stages.len(), 1);
            assert_eq!(fs::read_to_string(stages[0].join("original")).unwrap(), "original source");
        } else {
            assert!(error.contains("rolled back"), "{error}");
            assert_eq!(fs::read_to_string(fixture.0.join("candidate.rs")).unwrap(), "original source");
            assert!(fixture.stages().is_empty());
        }
        assert_eq!(fs::read_to_string(fixture.ledger()).unwrap(), if mode == "ledger-change" { "concurrent ledger" } else { LEDGER });
    }
}
