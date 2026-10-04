use eoie_coverage_prune::{coverage_assess, coverage_export, prune_uncovered};
use std::{env, fs, path::PathBuf, process::Command, time::{SystemTime, UNIX_EPOCH}};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie coverage ü {} {stamp}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }
    fn profile(&self, text: &str) -> String {
        let file = self.0.join("test.lcov");
        fs::write(&file, text).unwrap();
        file.to_str().unwrap().to_owned()
    }
    fn assess(&self, text: &str, floor: &str) -> Result<(), String> {
        let file = self.profile(text);
        coverage_assess(&["coverage-assess", &file, &file, floor, floor, floor].map(str::to_owned))
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

#[test]
fn coverage_assessment_rejects_malformed_and_ambiguous_evidence() {
    let fixture = Fixture::new();
    for text in [
        "", "SF:x.rs\nend_of_record\n", "DA:1,0\nend_of_record\n",
        "SF:\nDA:1,0\nend_of_record\n", "SF:x.rs\nDA:1,0\n",
        "SF:x.rs\nDA:1,0\nSF:y.rs\nDA:1,0\nend_of_record\n",
        "SF:x.rs\nDA:1,bad\nend_of_record\n", "SF:x.rs\nDA:0,0\nend_of_record\n",
        "SF:x.rs\nDA:1,-1\nend_of_record\n", "SF:x.rs\nDA:1,18446744073709551616\nend_of_record\n",
        "SF:x.rs\nDA:4294967296,0\nend_of_record\n", "SF:x.rs\nDA:1,0,a,b\nend_of_record\n",
        "SF:/one/src/a.rs\nDA:1,0\nend_of_record\nSF:/two/src/a.rs\nDA:1,1\nend_of_record\n",
    ] {
        assert!(fixture.assess(text, "0").is_err(), "accepted malformed coverage: {text:?}");
    }
}

#[test]
fn coverage_assessment_merges_duplicate_hits_and_checks_floor_bounds() {
    let fixture = Fixture::new();
    let text = "TN:one\nSF:src/a.rs\nDA:1,0\nDA:2,0\nend_of_record\nTN:two\nSF:src/a.rs\nDA:1,2\nend_of_record\n";
    fixture.assess(text, "500").unwrap();
    assert!(fixture.assess(text, "501").is_err());
    assert!(fixture.assess(text, "1001").unwrap_err().contains("0..=1000"));
}

#[test]
fn pruning_requires_line_evidence_for_the_exact_rooted_file() {
    let fixture = Fixture::new();
    fs::create_dir(fixture.0.join("nested")).unwrap();
    fs::write(fixture.0.join("candidate.rs"), "other file").unwrap();
    let candidate = fixture.0.join("nested/candidate.rs");
    fs::write(&candidate, "keep this source").unwrap();
    for text in [
        "SF:candidate.rs\nDA:1,0\nend_of_record\n",
        "SF:nested/candidate.rs\nend_of_record\nSF:candidate.rs\nDA:1,0\nend_of_record\n",
        "SF:nested/candidate.rs\nDA:1,bad\nend_of_record\n",
    ] {
        let profile = fixture.profile(text);
        let result = prune_uncovered(&["prune-uncovered", fixture.0.to_str().unwrap(), "nested/candidate.rs", &profile, &profile, "--", "eoie-nonexistent-validation-command"].map(str::to_owned));
        let error = result.unwrap_err();
        assert!(!error.contains("validation failed"), "pruning reached validation: {error}");
        assert_eq!(fs::read_to_string(&candidate).unwrap(), "keep this source");
    }
}

#[test]
fn export_preserves_previous_evidence_until_a_valid_candidate_exists() {
    let fixture = Fixture::new();
    let helper_source = fixture.0.join("fake_grcov.rs");
    fs::write(&helper_source, r#"fn main() {
        let args = std::env::args().collect::<Vec<_>>();
        let mode = std::fs::read_to_string(std::path::Path::new(&args[1]).join("mode")).unwrap();
        let out = &args[args.iter().position(|arg| arg == "-o").unwrap() + 1];
        std::fs::write(out, if mode == "valid" { "SF:src/a.rs\nDA:1,2\nend_of_record\n" } else { "SF:src/a.rs\nDA:1,bad\nend_of_record\n" }).unwrap();
        if mode == "failed" { std::process::exit(7); }
        if mode == "timeout" { std::thread::sleep(std::time::Duration::from_secs(5)); }
    }"#).unwrap();
    let helper = fixture.0.join(format!("fake-grcov{}", env::consts::EXE_SUFFIX));
    let compiled = Command::new("rustc").arg(&helper_source).arg("-o").arg(&helper).output().unwrap();
    assert!(compiled.status.success(), "{}", String::from_utf8_lossy(&compiled.stderr));
    let profraw = fixture.0.join("profiles");
    fs::create_dir(&profraw).unwrap();
    fs::write(profraw.join("one.profraw"), "profile").unwrap();
    let output = fixture.0.join("output.lcov");
    let receipt = fixture.0.join("output.lcov.receipt");
    fs::write(&output, "previous evidence").unwrap();
    fs::write(&receipt, "previous receipt").unwrap();
    for mode in ["failed", "malformed", "timeout", "valid"] {
        fs::write(profraw.join("mode"), mode).unwrap();
        let timeout = if mode == "timeout" { "100" } else { "5000" };
        let args = ["coverage-export", profraw.to_str().unwrap(), helper.to_str().unwrap(), helper.to_str().unwrap(), fixture.0.to_str().unwrap(), fixture.0.to_str().unwrap(), output.to_str().unwrap(), timeout].map(str::to_owned);
        let result = coverage_export(&args);
        if mode == "valid" {
            result.unwrap();
            assert!(fs::read_to_string(&output).unwrap().contains("DA:1,2"));
            assert!(fs::read_to_string(&receipt).unwrap().contains("covered_permille=1000"));
        } else {
            assert!(result.is_err());
            assert_eq!(fs::read_to_string(&output).unwrap(), "previous evidence");
            assert_eq!(fs::read_to_string(&receipt).unwrap(), "previous receipt");
        }
        assert!(!fs::read_dir(&fixture.0).unwrap().any(|entry| entry.unwrap().file_name().to_string_lossy().starts_with(".eoie-coverage-")));
    }
}
