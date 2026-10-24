use super::*;
use std::path::PathBuf;

const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const LCOV: &str = "SF:src/eoie_cli/eoie.rs\nDA:1,1\nDA:2,0\nend_of_record\n";

struct Fixture(PathBuf);
impl Fixture {
    fn new(label: &str) -> Self {
        let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie evidence {label} ü {} {stamp}", std::process::id()));
        fs::create_dir_all(root.join("state")).unwrap();
        fs::create_dir_all(root.join("input")).unwrap();
        Self(root)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}

fn coverage_fixture(root: &Path) {
    let rows = ["evidence/coverage/current.lcov", "evidence/coverage/current.lcov.receipt", "evidence/coverage/replay-seed.lcov", "evidence/coverage/replay-seed.receipt"]
        .iter()
        .map(|relative| format!("inl row () : core.evidence_ref =\n    core.EvidenceRef (\"{relative}\", \"{ZERO}\", 1u64, core.CoverageLcovMedia, core.BundlePath)\n"))
        .collect::<String>();
    fs::write(root.join("state/evidence.spi"), format!("{rows}inl main () : i32 = 0i32\n")).unwrap();
    fs::write(root.join("state/coverage.spi"), format!("inl current_checkpoint_durability () : coverage_checkpoint_durability = CoverageCheckpointDurable (804u32, \"{ZERO}\")\ninl current_replay_seed () : coverage_replay_seed = CoverageReplaySeed (725u32, 17303u32, \"{ZERO}\", \"evidence/coverage/replay-seed.lcov\")\ninl main () : i32 = 0i32\n")).unwrap();
}

fn lcov_input(root: &Path, name: &str, body: &str, receipt_sha: Option<&str>) -> PathBuf {
    let path = root.join("input").join(name);
    fs::write(&path, body).unwrap();
    let sha = receipt_sha.map(str::to_owned).unwrap_or_else(|| inspection_sha256(body.as_bytes().to_vec()));
    fs::write(PathBuf::from(format!("{}.receipt", path.display())), format!("route=lcov-union\ninputs=2\nlines=2\ncovered_permille=500\nsha256={sha}\n")).unwrap();
    path
}

#[test]
fn coverage_registration_imports_receipted_lcov_and_renews_durability() {
    let f = Fixture::new("register");
    coverage_fixture(&f.0);
    let current = lcov_input(&f.0, "current.lcov", LCOV, None);
    let seed = lcov_input(&f.0, "seed.lcov", "SF:src/a.rs\nDA:1,1\nDA:2,0\nend_of_record\n", None);
    let report = register_coverage_evidence(&f.0, &current, &seed).unwrap();
    let current_sha = inspection_sha256(LCOV.as_bytes().to_vec());
    assert!(report.contains(&current_sha), "{report}");
    assert_eq!(fs::read(f.0.join("evidence/coverage/current.lcov")).unwrap(), LCOV.as_bytes());
    assert!(f.0.join("evidence/coverage/replay-seed.receipt").is_file());
    let coverage = fs::read_to_string(f.0.join("state/coverage.spi")).unwrap();
    assert!(coverage.contains(&format!("CoverageCheckpointDurable (500u32, \"{current_sha}\")")), "{coverage}");
    assert!(coverage.contains("CoverageReplaySeed (500u32, 2u32, "), "{coverage}");
    let summary = check_evidence_tree(&f.0).unwrap();
    assert_eq!((summary.declared, summary.verified), (4, 4));
}

#[test]
fn coverage_registration_rejects_unbound_or_absolute_lcov_and_preserves_state() {
    let f = Fixture::new("reject");
    coverage_fixture(&f.0);
    let evidence = fs::read(f.0.join("state/evidence.spi")).unwrap();
    let coverage = fs::read(f.0.join("state/coverage.spi")).unwrap();
    let seed = lcov_input(&f.0, "seed.lcov", LCOV, None);
    let unbound = lcov_input(&f.0, "unbound.lcov", LCOV, Some(ZERO));
    assert!(register_coverage_evidence(&f.0, &unbound, &seed).unwrap_err().contains("sha256 does not match"));
    let absolute = lcov_input(&f.0, "absolute.lcov", "SF:C:\\work\\src\\a.rs\nDA:1,1\nend_of_record\n", None);
    assert!(register_coverage_evidence(&f.0, &absolute, &seed).unwrap_err().contains("must be relative"));
    fs::write(f.0.join("state/coverage.spi"), "inl main () : i32 = 0i32\n").unwrap();
    let current = lcov_input(&f.0, "current.lcov", LCOV, None);
    assert!(register_coverage_evidence(&f.0, &current, &seed).unwrap_err().contains("coverage checkpoint rows rejected"));
    fs::write(f.0.join("state/coverage.spi"), &coverage).unwrap();
    assert_eq!(fs::read(f.0.join("state/evidence.spi")).unwrap(), evidence);
    assert!(!f.0.join("evidence/coverage/current.lcov").exists());
}

#[test]
fn closeout_evidence_refresh_binds_the_platform_binary_name() {
    let f = Fixture::new("closeout");
    let root = &f.0;
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("evidence")).unwrap();
    for (relative, text) in [("state/release_closeout.spi", "closeout\n"), ("state/cold_proof.spi", "cold\n"), ("src/Cargo.lock", "lock\n"), ("eoie.exe", "windows binary")] {
        fs::write(root.join(relative), text).unwrap();
    }
    let rows = ["state/release_closeout.spi", "state/cold_proof.spi", "src/Cargo.lock", "eoie"]
        .iter()
        .map(|relative| format!("    core.EvidenceRef (\"{relative}\", \"{ZERO}\", 1u64, core.Utf8TextMedia, core.BundlePath)\n"))
        .collect::<String>();
    fs::write(root.join("state/evidence.spi"), format!("{rows}    core.EvidenceTreeRef (\"src\", \"{ZERO}\", core.ExactTree)\n    core.EvidenceTreeRef (\"state\", \"{ZERO}\", core.StateTreeWithoutMetaManifests)\n")).unwrap();
    let closeout = inspection_file_sha256(&root.join("state/release_closeout.spi")).unwrap();
    assert_eq!(refresh_release_closeout_evidence(root, &closeout, 9).unwrap(), 127);
    let manifest = fs::read_to_string(root.join("state/evidence.spi")).unwrap();
    let binary = inspection_file_sha256(&root.join("eoie.exe")).unwrap();
    assert!(manifest.contains(&format!("(\"eoie.exe\", \"{binary}\", 14u64")), "{manifest}");
    fs::write(root.join("eoie"), "unix binary").unwrap();
    assert!(refresh_release_closeout_evidence(root, &closeout, 9).unwrap_err().contains("exactly one root binary"));
}

#[test]
fn closeout_evidence_refresh_attests_the_release_state() {
    let f = Fixture::new("closeout manifest");
    let root = &f.0;
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("evidence")).unwrap();
    for (relative, text) in [("state/release_closeout.spi", "closeout\n"), ("state/cold_proof.spi", "cold\n"), ("src/Cargo.lock", "lock\n"), ("src/Cargo.toml", "[workspace]\n"), ("state/bundle.spi", "bundle\n"), ("eoie.exe", "windows binary")] {
        fs::write(root.join(relative), text).unwrap();
    }
    let rows = ["state/release_closeout.spi", "state/cold_proof.spi", "src/Cargo.lock", "src/Cargo.toml", "state/bundle.spi", "eoie"]
        .iter()
        .map(|relative| format!("    core.EvidenceRef (\"{relative}\", \"{ZERO}\", 1u64, core.Utf8TextMedia, core.BundlePath)\n"))
        .collect::<String>();
    fs::write(root.join("state/evidence.spi"), format!("{rows}    core.EvidenceTreeRef (\"src\", \"{ZERO}\", core.ExactTree)\n    core.EvidenceTreeRef (\"state\", \"{ZERO}\", core.StateTreeWithoutMetaManifests)\n")).unwrap();
    let closeout = inspection_file_sha256(&root.join("state/release_closeout.spi")).unwrap();
    assert_eq!(refresh_release_closeout_evidence(root, &closeout, 9).unwrap(), 127);
    let manifest = fs::read_to_string(root.join("state/evidence.spi")).unwrap();
    let workspace = inspection_file_sha256(&root.join("src/Cargo.toml")).unwrap();
    assert!(manifest.contains(&format!("(\"src/Cargo.toml\", \"{workspace}\", 12u64")), "{manifest}");
    let bundle = inspection_file_sha256(&root.join("state/bundle.spi")).unwrap();
    assert!(manifest.contains(&format!("(\"state/bundle.spi\", \"{bundle}\", 7u64")), "{manifest}");
}

#[test]
fn evidence_rows_follow_the_spiral_currentness_plan() {
    for (regular, identical, stage) in [(0, 0, 0), (0, 1, 0), (1, 0, 1), (1, 1, 2)] { assert_eq!(eoie_evidence_row_stage(regular, identical), stage); }
    for (tool, name) in [("rustc", "RUSTC"), ("cargo", "CARGO"), ("rustdoc", "RUSTDOC"), ("rustfmt", "RUSTFMT"), ("rustup", ""), ("", "")] { assert_eq!(toolchain_env_name(tool).unwrap_or_default(), name); }
    for (unix, windows, name) in [(1, 0, "eoie"), (0, 1, "eoie.exe"), (1, 1, ""), (0, 0, "")] { assert_eq!(&*eoie_evidence_public_binary(unix, windows), name); }
    assert!(evidence_sha256_text(ZERO) && evidence_sha256_text(&"Ab".repeat(32)));
    assert!(!evidence_sha256_text(&ZERO[1..]) && !evidence_sha256_text(&"g".repeat(64)) && !evidence_sha256_text(&format!("{ZERO}0")));
    for (code, tail) in [(0, "contains no PatchExact values"), (-1, "is malformed"), (-2, "exceeds 256 KiB"), (-3, "source is unavailable"), (9, "returned an invalid witness")] { assert_eq!(&*eoie_patch_control_domain::eoie_patch_plan_failure_message(code), format!("typed patch plan {tail}")); }
}
