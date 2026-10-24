use super::*;
use std::path::PathBuf;
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("eoie-legacy-paths-{}-{nonce}", std::process::id()));
        fs::create_dir_all(path.join("root/state/nested")).unwrap();
        fs::create_dir(path.join("outside")).unwrap();
        fs::write(path.join("root/state/input.txt"), b"input").unwrap();
        fs::write(path.join("outside/keep.txt"), b"outside").unwrap();
        Self(path)
    }
    fn root(&self) -> PathBuf { self.0.join("root") }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }
fn chmod(root: &Path, relative: &str, mode: &str) -> Result<(), String> {
    legacy_chmod_run(&["fs-chmod".into(), root.to_string_lossy().into_owned(), relative.into(), mode.into()])
}
fn link(root: &Path, target: &str, relative: &str) -> Result<(), String> {
    legacy_symlink_run(&["fs-symlink".into(), root.to_string_lossy().into_owned(), target.into(), relative.into()])
}
fn directory_link(target: &Path, path: &Path) {
    #[cfg(windows)] {
        use std::os::windows::process::CommandExt as _;
        let output = Command::new("pwsh").creation_flags(0x08000000)
            .args(["-NoProfile", "-NonInteractive", "-Command", "$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:EOIE_TEST_LINK -Target $env:EOIE_TEST_TARGET | Out-Null"])
            .env("EOIE_TEST_LINK", path).env("EOIE_TEST_TARGET", target).output().unwrap();
        assert!(output.status.success(), "{output:?}");
    }
    #[cfg(unix)] std::os::unix::fs::symlink(target, path).unwrap();
}
fn remove_directory_link(path: &Path) {
    #[cfg(windows)] fs::remove_dir(path).unwrap();
    #[cfg(unix)] fs::remove_file(path).unwrap();
}
#[test]
fn chmod_changes_files_and_directories_and_restores_zero_mode() {
    let fixture = Fixture::new();
    for relative in ["state/input.txt", "state/nested"] {
        chmod(&fixture.root(), relative, "0000").unwrap();
        assert!(eoie_rust_std_fs::portable_mode_matches(eoie_rust_std_fs::portable_mode(&fs::metadata(fixture.root().join(relative)).unwrap().permissions()), 0));
        chmod(&fixture.root(), relative, "0755").unwrap();
        assert!(chmod(&fixture.root(), relative, "10000").is_err());
        assert!(eoie_rust_std_fs::portable_mode_matches(eoie_rust_std_fs::portable_mode(&fs::metadata(fixture.root().join(relative)).unwrap().permissions()), 0o755));
    }
}
#[test]
fn chmod_rejects_linked_roots_ancestors_and_leaves() {
    let fixture = Fixture::new();
    let outside = fixture.0.join("outside");
    let before = eoie_rust_std_fs::portable_mode(&fs::metadata(outside.join("keep.txt")).unwrap().permissions());
    let linked = fixture.root().join("linked");
    directory_link(&outside, &linked);
    assert!(chmod(&fixture.root(), "linked/keep.txt", "0000").is_err());
    assert!(chmod(&fixture.root(), "linked", "0000").is_err());
    assert!(chmod(&linked, "keep.txt", "0000").is_err());
    assert_eq!(eoie_rust_std_fs::portable_mode(&fs::metadata(outside.join("keep.txt")).unwrap().permissions()), before);
    remove_directory_link(&linked);
}
#[test]
fn symlinks_resolve_root_relative_targets_and_preserve_existing_paths() {
    let fixture = Fixture::new();
    for relative in ["link", "state/link", "state/nested/link"] {
        match link(&fixture.root(), "state/input.txt", relative) {
            Err(error) if cfg!(windows) && error.contains("1314") => { eprintln!("symlink privilege unavailable"); return; }
            result => result.unwrap(),
        }
        assert_eq!(fs::read(fixture.root().join(relative)).unwrap(), b"input");
        assert!(link(&fixture.root(), "outside", relative).is_err());
        assert_eq!(fs::read(fixture.root().join(relative)).unwrap(), b"input");
    }
    assert!(link(&fixture.root(), "missing", "state/input.txt").is_err());
    assert_eq!(fs::read(fixture.root().join("state/input.txt")).unwrap(), b"input");
    let moved = fixture.0.join("moved");
    fs::rename(fixture.root(), &moved).unwrap();
    assert_eq!(fs::read(moved.join("state/nested/link")).unwrap(), b"input");
}
#[test]
fn symlink_creation_rejects_linked_parents_and_keeps_unowned_stage_names() {
    let fixture = Fixture::new();
    let linked = fixture.root().join("linked");
    directory_link(&fixture.0.join("outside"), &linked);
    assert!(link(&fixture.root(), "state/input.txt", "linked/output").is_err());
    assert!(!fixture.0.join("outside/output").exists());
    let old_stage = fixture.root().join(format!(".eoie-link-stage-{}", std::process::id()));
    fs::write(&old_stage, b"foreign").unwrap();
    let result = link(&fixture.root(), "state/input.txt", "ordinary-link");
    if let Err(error) = result { assert!(cfg!(windows) && error.contains("1314"), "{error}"); }
    assert_eq!(fs::read(&old_stage).unwrap(), b"foreign");
    remove_directory_link(&linked);
}

