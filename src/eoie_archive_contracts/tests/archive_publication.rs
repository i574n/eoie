use std::{env, fs, path::{Path, PathBuf}, process::{Command, Output}, time::{SystemTime, UNIX_EPOCH}};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = env::temp_dir().join(format!("eoie archive publication ü {} {stamp}", std::process::id()));
        fs::create_dir_all(path.join("source")).unwrap();
        fs::write(path.join("source/value.txt"), "original payload").unwrap();
        Self(path)
    }
    fn create(&self, command: &str, output: &Path, limit: &str) -> Output {
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
        });
        Command::new(binary).args(["bundle", command]).arg(self.0.join("source")).arg(output).arg(limit).output().unwrap()
    }
    fn assert_no_stage(&self) {
        for entry in fs::read_dir(&self.0).unwrap() {
            let name = entry.unwrap().file_name();
            assert!(name == "source" || name == "archive.zip", "orphaned output: {name:?}");
        }
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

fn binary() -> PathBuf {
    env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
        env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
    })
}

#[test]
fn rejected_archive_preserves_previous_and_success_replaces_it() {
    let fixture = Fixture::new();
    let archive = fixture.0.join("archive.zip");
    let first = fixture.create("create-flat", &archive, "100000");
    assert!(first.status.success(), "{}", String::from_utf8_lossy(&first.stderr));
    let original = fs::read(&archive).unwrap();
    fs::write(fixture.0.join("source/value.txt"), "updated payload").unwrap();
    let rejected = fixture.create("create-flat", &archive, "1");
    assert!(!rejected.status.success());
    assert_eq!(fs::read(&archive).unwrap(), original);
    fixture.assert_no_stage();
    let updated = fixture.create("create-flat", &archive, "100000");
    assert!(updated.status.success(), "{}", String::from_utf8_lossy(&updated.stderr));
    assert_ne!(fs::read(&archive).unwrap(), original);
    fixture.assert_no_stage();
    fs::remove_file(&archive).unwrap();
    assert!(!fixture.create("create-flat", &archive, "1").status.success());
    assert!(!archive.exists());
    fixture.assert_no_stage();
}

#[test]
fn archive_output_inside_source_is_rejected_before_mutation() {
    for command in ["create-flat", "create"] {
        let fixture = Fixture::new();
        for relative in ["inside.zip", "new folder/inside.zip"] {
            let output = fixture.0.join("source").join(relative);
            let result = fixture.create(command, &output, if command == "create" { "generic" } else { "100000" });
            assert!(!result.status.success(), "{command} accepted {}", output.display());
            assert!(String::from_utf8_lossy(&result.stderr).contains("outside bundle root"));
            assert!(!output.exists());
        }
        assert!(!fixture.0.join("source/new folder").exists());
        let output = fixture.0.join("source/existing.zip");
        fs::write(&output, "preserve existing file").unwrap();
        assert!(!fixture.create(command, &output, if command == "create" { "generic" } else { "100000" }).status.success());
        assert_eq!(fs::read_to_string(output).unwrap(), "preserve existing file");
    }
}

#[test]
fn native_profiles_require_release_validation_and_cleanup() {
    for names in [&["eoie"][..], &["eoie.exe"][..], &["eoie", "eoie.exe"][..]] {
        let fixture = Fixture::new();
        let source = fixture.0.join("source");
        for name in names {
            let path = source.join(name);
            fs::write(&path, "placeholder binary").unwrap();
            #[cfg(unix)] {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
            }
        }
        for name in ["state/package.spiproj", "state/core.spi", "src/Cargo.toml", "evidence/coverage/current.lcov"] {
            let path = source.join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, "placeholder").unwrap();
        }
        let checked = Command::new(binary()).args(["bundle", "check"]).arg(&source).arg("auto").output().unwrap();
        assert!(!checked.status.success(), "native layout silently downgraded to generic: {names:?}");
        let generic = Command::new(binary()).args(["bundle", "check"]).arg(&source).arg("generic").output().unwrap();
        assert!(generic.status.success(), "{}", String::from_utf8_lossy(&generic.stderr));
        let archive = fixture.0.join("archive.zip");
        let created = fixture.create("create-flat", &archive, "100000");
        assert!(created.status.success(), "{}", String::from_utf8_lossy(&created.stderr));
        let scratch = fixture.0.join("scratch parent");
        fs::create_dir(&scratch).unwrap();
        for profile in ["eoie", "auto", "generic"] {
            let checked = Command::new(binary()).args(["bundle", "verify"]).arg(&archive).arg(profile)
                .env("TMP", &scratch).env("TEMP", &scratch).env("TMPDIR", &scratch).output().unwrap();
            assert_eq!(checked.status.success(), profile == "generic", "{names:?} profile={profile}: {}", String::from_utf8_lossy(&checked.stdout));
            assert_eq!(fs::read_dir(&scratch).unwrap().count(), 0, "verification left scratch output");
        }
    }
}
