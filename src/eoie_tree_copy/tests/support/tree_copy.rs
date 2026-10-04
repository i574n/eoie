use super::*;
use std::process::Command;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("eoie-tree-copy-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn source(&self) -> PathBuf {
        let source = self.0.join("source");
        fs::create_dir_all(source.join("nested")).unwrap();
        fs::write(source.join("first.txt"), b"first").unwrap();
        fs::write(source.join("nested/second.txt"), b"second").unwrap();
        source
    }
    fn assert_no_stage(&self) {
        assert!(!fs::read_dir(&self.0).unwrap().any(|entry| entry.unwrap().file_name().to_string_lossy().starts_with(".eoie-copy-stage-")));
    }
}
impl Drop for Fixture {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}
#[cfg(windows)]
fn directory_link(target: &Path, link: &Path) {
    let status = Command::new("pwsh").args(["-NoProfile", "-NonInteractive", "-Command", "New-Item -ItemType Junction -Path $env:EOIE_TEST_LINK -Target $env:EOIE_TEST_TARGET | Out-Null"])
        .env("EOIE_TEST_LINK", link).env("EOIE_TEST_TARGET", target).status().unwrap();
    assert!(status.success());
}
#[cfg(unix)]
fn directory_link(target: &Path, link: &Path) { std::os::unix::fs::symlink(target, link).unwrap(); }

#[test]
fn copies_complete_tree_and_creates_missing_parent() {
    let fixture = Fixture::new();
    let source = fixture.source();
    let destination = fixture.0.join("new-parent/copied");
    assert_eq!(legacy_copy_tree_with_hook(&source, &destination, |path| {
        assert!(!path.exists());
        Ok(())
    }).unwrap(), 2);
    assert_eq!(fs::read(destination.join("first.txt")).unwrap(), b"first");
    assert_eq!(fs::read(destination.join("nested/second.txt")).unwrap(), b"second");
    assert_eq!(fs::read_dir(destination.parent().unwrap()).unwrap().count(), 1);
}
#[test]
fn rejects_overlap_before_creating_directories() {
    let fixture = Fixture::new();
    let source = fixture.source();
    let destination = source.join("new-parent/copied");
    assert!(legacy_copy_tree_with_hook(&source, &destination, |_| Ok(())).unwrap_err().contains("overlap"));
    assert!(!source.join("new-parent").exists());
    assert!(legacy_copy_tree_with_hook(&source, &source, |_| Ok(())).is_err());
    fixture.assert_no_stage();
}
#[test]
fn preserves_existing_and_competing_destinations() {
    let fixture = Fixture::new();
    let source = fixture.source();
    let destination = fixture.0.join("copied");
    let error = legacy_copy_tree_with_hook(&source, &destination, |path| {
        fs::create_dir(path).unwrap();
        fs::write(path.join("foreign.txt"), b"keep").unwrap();
        Ok(())
    }).unwrap_err();
    assert!(error.contains("rolled back"), "{error}");
    assert_eq!(fs::read(destination.join("foreign.txt")).unwrap(), b"keep");
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 1);
    assert!(legacy_copy_tree_with_hook(&source, &destination, |_| Ok(())).is_err());
    fixture.assert_no_stage();
}
#[test]
fn rolls_back_partial_copy_on_linked_child() {
    let fixture = Fixture::new();
    let source = fixture.source();
    let external = fixture.0.join("external");
    fs::create_dir(&external).unwrap();
    fs::write(external.join("sentinel"), b"outside").unwrap();
    let link = source.join("zz-linked");
    directory_link(&external, &link);
    let destination = fixture.0.join("copied");
    let error = legacy_copy_tree_with_hook(&source, &destination, |_| Ok(())).unwrap_err();
    assert!(error.contains("linked"), "{error}");
    assert!(!destination.exists());
    assert_eq!(fs::read(external.join("sentinel")).unwrap(), b"outside");
    fixture.assert_no_stage();
    #[cfg(windows)] fs::remove_dir(link).unwrap();
    #[cfg(unix)] fs::remove_file(link).unwrap();
}
#[test]
fn rejects_linked_source_and_destination_ancestors() {
    let fixture = Fixture::new();
    let source = fixture.source();
    let link = fixture.0.join("linked");
    directory_link(&source, &link);
    assert!(legacy_copy_tree_with_hook(&link.join("nested"), &fixture.0.join("copied"), |_| Ok(())).is_err());
    assert!(legacy_copy_tree_with_hook(&source, &link.join("new-parent/copied"), |_| Ok(())).is_err());
    assert!(!source.join("new-parent").exists());
    fixture.assert_no_stage();
    #[cfg(windows)] fs::remove_dir(link).unwrap();
    #[cfg(unix)] fs::remove_file(link).unwrap();
}
#[test]
fn traversal_limit_rejects_before_copying() {
    let fixture = Fixture::new();
    let source = fixture.source();
    let stage = fixture.0.join("stage");
    fs::create_dir(&stage).unwrap();
    assert!(legacy_copy_tree_inner(&source, &stage, &stage, 129, &mut 0, &mut Vec::new()).is_err());
    assert!(legacy_copy_tree_inner(&source, &stage, &stage, 0, &mut 100_000, &mut Vec::new()).is_err());
    assert_eq!(fs::read_dir(&stage).unwrap().count(), 0);
}
#[cfg(unix)]
#[test]
fn preserves_readonly_directory_modes_and_cleans_failed_stage() {
    use std::os::unix::fs::PermissionsExt as _;
    let fixture = Fixture::new();
    let source = fixture.source();
    fs::set_permissions(source.join("nested"), fs::Permissions::from_mode(0o555)).unwrap();
    fs::set_permissions(&source, fs::Permissions::from_mode(0o555)).unwrap();
    let destination = fixture.0.join("copied");
    assert_eq!(legacy_copy_tree_with_hook(&source, &destination, |_| Ok(())).unwrap(), 2);
    assert_eq!(fs::metadata(&destination).unwrap().permissions().mode() & 0o777, 0o555);
    assert_eq!(fs::metadata(destination.join("nested")).unwrap().permissions().mode() & 0o777, 0o555);
    let raced = fixture.0.join("raced");
    assert!(legacy_copy_tree_with_hook(&source, &raced, |path| { fs::create_dir(path).unwrap(); Ok(()) }).is_err());
    fixture.assert_no_stage();
    for root in [&source, &destination] {
        fs::set_permissions(root, fs::Permissions::from_mode(0o755)).unwrap();
        fs::set_permissions(root.join("nested"), fs::Permissions::from_mode(0o755)).unwrap();
    }
}
