use eoie_agile_lease::agile_package_input_fingerprint;
use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let fixture = Self(std::env::temp_dir().join(format!("eoie graph ü {} {nonce}", std::process::id())));
        fixture.write("state/package.spiproj", "packageDir: ../src\npackages:\n    dep-\nmodules:\n    state\n");
        fixture.write("state/state.spi", "inl main () = 0i32\n");
        fixture.write("src/dep/package.spiproj", "modules:\n    model-\n");
        fixture.write("src/dep/model.spi", "inl value = 1i32\n");
        fixture
    }
    fn write(&self, name: &str, value: &str) {
        let path = self.0.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, value).unwrap();
    }
    fn hash(&self, include_state: bool) -> Option<u64> { agile_package_input_fingerprint(&self.0.join("state"), include_state) }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }
#[test]
fn manifest_and_imported_source_changes_invalidate_but_state_has_separate_receipts() {
    let fixture = Fixture::new();
    let initial = fixture.hash(false).unwrap();
    let full = fixture.hash(true).unwrap();
    fixture.write("state/state.spi", "inl main () = 2i32\n");
    assert_eq!(Some(initial), fixture.hash(false));
    assert_ne!(Some(full), fixture.hash(true));
    fixture.write("src/dep/model.spi", "inl value = 3i32\n");
    assert_ne!(Some(initial), fixture.hash(false));
    let changed = fixture.hash(false).unwrap();
    fixture.write("state/package.spiproj", "packageDir: ../src\npackages:\n    dep\nmodules:\n    state\n");
    assert_ne!(Some(changed), fixture.hash(false));
}
#[test]
fn relative_directories_spir_files_transitive_dependencies_and_relocation_are_bound() {
    let first = Fixture::new();
    let second = Fixture::new();
    assert_eq!(first.hash(false), second.hash(false));
    first.write("src/dep/package.spiproj", "moduleDir: body\npackages:\n    other\nmodules:\n    model*-\n");
    first.write("src/dep/body/model.spir", "let value = 0i32");
    first.write("src/other/package.spiproj", "modules:\n    model\n");
    first.write("src/other/model.spi", "inl main () = 0i32");
    let initial = first.hash(false).unwrap();
    first.write("src/other/model.spi", "inl main () = 1i32");
    assert_ne!(Some(initial), first.hash(false));
    let changed = first.hash(false).unwrap();
    first.write("src/dep/body/model.spir", "let value = 1i32");
    assert_ne!(Some(changed), first.hash(false));
}
#[test]
fn missing_cyclic_ambiguous_or_unsupported_graphs_are_not_cacheable() {
    let fixture = Fixture::new();
    let valid = fs::read_to_string(fixture.0.join("state/package.spiproj")).unwrap();
    for invalid in [
        "packages:\n    missing\nmodules:\n    state\n",
        "packages:\n    |core-\nmodules:\n    state\n",
        "modules:\n    nested/\n        state\n",
        "unknown: ignored\nmodules:\n    state\n",
        "modules:\n    state\nmodules:\n    state\n",
        "modules:\n\tstate\n",
    ] {
        fixture.write("state/package.spiproj", invalid);
        assert_eq!(None, fixture.hash(false), "{invalid}");
    }
    fixture.write("state/package.spiproj", &valid);
    fixture.write("src/dep/package.spiproj", "packages:\n    dep\nmodules:\n    model\n");
    assert_eq!(None, fixture.hash(false));
    fixture.write("src/dep/package.spiproj", "modules:\n    absent\n");
    assert_eq!(None, fixture.hash(false));
}
#[test]
fn bounded_inputs_reject_oversized_sources() {
    let fixture = Fixture::new();
    let source = fs::File::create(fixture.0.join("src/dep/model.spi")).unwrap();
    source.set_len(8 * 1024 * 1024 + 1).unwrap();
    assert_eq!(None, fixture.hash(false));
}
#[cfg(windows)]
#[test]
fn junction_dependencies_and_cancelled_link_components_are_not_cacheable() {
    let fixture = Fixture::new();
    let link = fixture.0.join("src/link");
    let target = fixture.0.join("src/dep");
    let output = std::process::Command::new("cmd.exe").args(["/d", "/c", "mklink", "/J"]).arg(&link).arg(&target).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    fixture.write("state/package.spiproj", "packageDir: ../src\npackages:\n    link\nmodules:\n    state\n");
    assert_eq!(None, fixture.hash(false));
    fixture.write("state/package.spiproj", "packageDir: ../src/link/..\npackages:\n    dep\nmodules:\n    state\n");
    assert_eq!(None, fixture.hash(false));
    fs::remove_dir(link).unwrap();
}

#[cfg(windows)]
#[test]
fn root_path_cannot_cancel_a_junction_before_validation() {
    let fixture = Fixture::new();
    let link = fixture.0.join("link");
    let output = std::process::Command::new("cmd.exe").args(["/d", "/c", "mklink", "/J"]).arg(&link).arg(fixture.0.join("src")).output().unwrap();
    assert!(output.status.success());
    assert_eq!(None, agile_package_input_fingerprint(&link.join("../state"), false));
    fs::remove_dir(link).unwrap();
}
