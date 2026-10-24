use eoie_compiler_discovery::{resolve_spiral_compiler, SpiralCompilerDiscovery};
use std::{fs, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("eoie compiler identity ü {} {nonce}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let fixture = Self(root);
        fixture.write("SpiralCompiler.dll", b"entry");
        fixture.write("SpiralCompilerCore.dll", b"core");
        fixture
    }
    fn write(&self, relative: &str, bytes: &[u8]) {
        let path = self.0.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }
    fn identity(&self, entry: &str) -> Result<SpiralCompilerDiscovery, String> {
        resolve_spiral_compiler(self.0.join(entry).to_str()).map(|value| value.unwrap())
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

#[test]
fn core_changes_invalidate_managed_dll_and_apphost_identities() {
    let fixture = Fixture::new();
    fixture.write("SpiralCompiler.exe", b"fixed apphost");
    for entry in ["SpiralCompiler.dll", "SpiralCompiler.exe"] {
        fixture.write("SpiralCompilerCore.dll", b"revision A");
        let before = fixture.identity(entry).unwrap();
        fixture.write("SpiralCompilerCore.dll", b"revision B");
        let after = fixture.identity(entry).unwrap();
        assert_eq!(before.entry_sha256, after.entry_sha256);
        assert_ne!(before.fingerprint, after.fingerprint);
        assert_ne!(before.sha256, after.sha256);
    }
}

#[test]
fn runtime_configuration_native_dependencies_and_inventory_are_bound() {
    let fixture = Fixture::new();
    for path in ["SpiralCompiler.deps.json", "SpiralCompiler.runtimeconfig.json", "runtimes/win-x64/native/support.dll", "runtimes/linux-x64/native/libsupport.so.1", "fr/SpiralCompiler.resources.dll"] {
        let before = fixture.identity("SpiralCompiler.dll").unwrap();
        fixture.write(path, b"first");
        let added = fixture.identity("SpiralCompiler.dll").unwrap();
        assert_ne!(before.sha256, added.sha256, "{path}");
        fixture.write(path, b"changed");
        let changed = fixture.identity("SpiralCompiler.dll").unwrap();
        assert_ne!(added.sha256, changed.sha256, "{path}");
        fs::remove_file(fixture.0.join(path)).unwrap();
        assert_eq!(before.sha256, fixture.identity("SpiralCompiler.dll").unwrap().sha256, "{path}");
    }
}

#[test]
fn relocation_and_debug_artifacts_do_not_invalidate_semantic_identity() {
    let first = Fixture::new();
    let second = Fixture::new();
    let expected = first.identity("SpiralCompiler.dll").unwrap();
    assert_eq!(expected.sha256, second.identity("SpiralCompiler.dll").unwrap().sha256);
    first.write("SpiralCompiler.pdb", b"debug symbols");
    first.write("eoie-compiler-snapshot.json", b"machine-local manifest");
    let observed = first.identity("SpiralCompiler.dll").unwrap();
    assert_eq!(expected.sha256, observed.sha256);
    assert_eq!(expected.fingerprint, observed.fingerprint);
}

#[test]
fn entry_filename_is_bound_when_it_selects_a_different_runtime_configuration() {
    let fixture = Fixture::new();
    fixture.write("SpiralCompiler.runtimeconfig.json", b"runtime A");
    fixture.write("Renamed.runtimeconfig.json", b"runtime B");
    let before = fixture.identity("SpiralCompiler.dll").unwrap();
    fs::rename(fixture.0.join("SpiralCompiler.dll"), fixture.0.join("Renamed.dll")).unwrap();
    let after = fixture.identity("Renamed.dll").unwrap();
    assert_eq!(before.entry_sha256, after.entry_sha256);
    assert_ne!(before.sha256, after.sha256);
    assert_ne!(before.fingerprint, after.fingerprint);
}

#[test]
fn dependency_identity_rejects_oversized_files_and_deep_trees() {
    let fixture = Fixture::new();
    fs::File::create(fixture.0.join("oversized.dll")).unwrap().set_len(128 * 1024 * 1024 + 1).unwrap();
    assert!(fixture.identity("SpiralCompiler.dll").unwrap_err().contains("artifact budget"));
    fs::remove_file(fixture.0.join("oversized.dll")).unwrap();
    fixture.write("a/b/c/d/e/f/g/h/i/nested.dll", b"deep");
    assert!(fixture.identity("SpiralCompiler.dll").unwrap_err().contains("depth limit"));
}

#[cfg(windows)]
fn link_directory(source: &Path, link: &Path) {
    let output = std::process::Command::new("pwsh").args(["-NoProfile", "-NonInteractive", "-Command",
        "$ErrorActionPreference='Stop'; New-Item -ItemType Junction -Path $env:EOIE_TEST_LINK -Target $env:EOIE_TEST_SOURCE | Out-Null"])
        .env("EOIE_TEST_LINK", link).env("EOIE_TEST_SOURCE", source).output().unwrap();
    assert!(output.status.success(), "{output:?}");
}
#[cfg(unix)]
fn link_directory(source: &Path, link: &Path) { std::os::unix::fs::symlink(source, link).unwrap(); }

#[cfg(any(windows, unix))]
#[test]
fn dependency_identity_rejects_links_without_following_outside() {
    let fixture = Fixture::new();
    let outside = Fixture::new();
    link_directory(&outside.0, &fixture.0.join("runtimes"));
    assert!(fixture.identity("SpiralCompiler.dll").unwrap_err().contains("must not be a link"));
    drop(fixture);
    assert_eq!(fs::read(outside.0.join("SpiralCompilerCore.dll")).unwrap(), b"core");
}
