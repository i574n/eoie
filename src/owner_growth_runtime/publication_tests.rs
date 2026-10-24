use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("eoie growth ü {} {stamp}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}

#[test]
fn receipt_publication_preserves_an_unrelated_fixed_temporary_file() {
    let fixture = Fixture::new();
    let state = fixture.0.join("state");
    fs::create_dir(&state).unwrap();
    fs::write(state.join("owner_growth.spi"), b"original receipt").unwrap();
    fs::write(state.join("owner_growth.spi.tmp"), b"another writer's file").unwrap();
    super::owner_growth_write(&fixture.0, "new receipt").unwrap();
    assert_eq!(fs::read(state.join("owner_growth.spi")).unwrap(), b"new receipt");
    assert_eq!(fs::read(state.join("owner_growth.spi.tmp")).unwrap(), b"another writer's file");
    assert_eq!(fs::read_dir(state).unwrap().count(), 2);
}

#[test]
fn receipt_publication_rejects_a_linked_state_directory() {
    let fixture = Fixture::new();
    let outside = fixture.0.join("outside");
    let root = fixture.0.join("root");
    fs::create_dir(&outside).unwrap();
    fs::create_dir(&root).unwrap();
    fs::write(outside.join("owner_growth.spi"), b"outside receipt").unwrap();
    let link = root.join("state");
    #[cfg(windows)] {
        use std::os::windows::process::CommandExt as _;
        let output = std::process::Command::new("pwsh").creation_flags(0x08000000)
            .args(["-NoProfile", "-NonInteractive", "-Command", "$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:EOIE_TEST_LINK -Target $env:EOIE_TEST_TARGET | Out-Null"])
            .env("EOIE_TEST_LINK", &link).env("EOIE_TEST_TARGET", &outside).output().unwrap();
        assert!(output.status.success(), "{output:?}");
    }
    #[cfg(unix)] std::os::unix::fs::symlink(&outside, &link).unwrap();
    let result = super::owner_growth_write(&root, "must not escape");
    #[cfg(windows)] fs::remove_dir(&link).unwrap();
    #[cfg(unix)] fs::remove_file(&link).unwrap();
    assert!(result.is_err(), "receipt publication followed a linked state directory");
    assert_eq!(fs::read(outside.join("owner_growth.spi")).unwrap(), b"outside receipt");
    assert_eq!(fs::read_dir(outside).unwrap().count(), 1);
}
