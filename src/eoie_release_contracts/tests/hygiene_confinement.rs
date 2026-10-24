#![cfg(any(windows, target_os = "linux"))]
use std::{fs, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};
use eoie_release_hygiene::prune_release_transients;

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("eoie hygiene ü {} {stamp}", std::process::id()));
        fs::create_dir(&root).unwrap();
        Self(root)
    }
    fn write(&self, path: &str, bytes: &[u8]) {
        let path = self.0.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }
    fn args(&self, command: &str, dry_run: bool) -> Vec<String> {
        let mut args = vec![command.to_owned(), self.0.to_string_lossy().into_owned()];
        if dry_run { args.push("--dry-run".to_owned()); }
        args
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

#[cfg(windows)]
fn link_directory(source: &Path, link: &Path) {
    let created = std::process::Command::new("pwsh").args(["-NoProfile", "-NonInteractive", "-Command",
        "$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:EOIE_TEST_LINK -Target $env:EOIE_TEST_SOURCE | Out-Null"])
        .env("EOIE_TEST_LINK", link).env("EOIE_TEST_SOURCE", source).output().unwrap();
    assert!(created.status.success(), "{created:?}");
}
#[cfg(target_os = "linux")]
fn link_directory(source: &Path, link: &Path) { std::os::unix::fs::symlink(source, link).unwrap(); }

#[test]
fn transient_cleanup_rejects_linked_candidates_before_any_removal() {
    let outside = Fixture::new();
    outside.write("keep", b"outside");
    let fixture = Fixture::new();
    fixture.write("first.receipt", b"receipt");
    link_directory(&outside.0, &fixture.0.join(".cold-rebuild-linked"));
    for dry in [true, false] { assert!(prune_release_transients(&fixture.args("prune-release-transients", dry)).is_err()); }
    assert_eq!(fs::read(fixture.0.join("first.receipt")).unwrap(), b"receipt");
    drop(fixture);
    assert_eq!(fs::read(outside.0.join("keep")).unwrap(), b"outside");
}

#[test]
fn transient_cleanup_removes_only_selected_entries_and_never_follows_child_links() {
    let outside = Fixture::new();
    outside.write("keep", b"outside");
    let fixture = Fixture::new();
    fixture.write("build.receipt", b"receipt");
    fixture.write("source.spi", b"source");
    fixture.write(".cold-rebuild-own/generated", b"scratch");
    link_directory(&outside.0, &fixture.0.join(".cold-rebuild-own/linked"));
    prune_release_transients(&fixture.args("prune-release-transients", true)).unwrap();
    assert!(fixture.0.join("build.receipt").exists());
    prune_release_transients(&fixture.args("prune-release-transients", false)).unwrap();
    assert!(!fixture.0.join("build.receipt").exists());
    assert!(!fixture.0.join(".cold-rebuild-own").exists());
    assert_eq!(fs::read(fixture.0.join("source.spi")).unwrap(), b"source");
    assert_eq!(fs::read(outside.0.join("keep")).unwrap(), b"outside");
}
