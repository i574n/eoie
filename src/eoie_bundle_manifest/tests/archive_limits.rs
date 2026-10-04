use eoie_bundle_manifest::{collect_bundle_manifest, write_deterministic_store_zip, write_deterministic_deflate_zip, zip_entries_with_limit};
use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("eoie zip limit {} {stamp}", std::process::id()));
        fs::create_dir_all(path.join("source")).unwrap();
        fs::write(path.join("source/a.txt"), vec![b'a'; 512]).unwrap();
        fs::write(path.join("source/b.txt"), vec![b'b'; 512]).unwrap();
        Self(path)
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

#[test]
fn zip_verification_bounds_total_expansion_for_both_methods() {
    for write in [write_deterministic_store_zip, write_deterministic_deflate_zip] {
        let fixture = Fixture::new();
        let source = fixture.0.join("source");
        let manifest = collect_bundle_manifest(&source).unwrap();
        let archive = fixture.0.join("archive.zip");
        write(&source, &archive, &manifest).unwrap();
        assert_eq!(zip_entries_with_limit(&archive, 1024).unwrap().len(), 2);
        assert!(zip_entries_with_limit(&archive, 1023).unwrap_err().contains("budget exceeded"));
        assert!(zip_entries_with_limit(&archive, 511).unwrap_err().contains("budget exceeded"));
        assert!(zip_entries_with_limit(&archive, 0).is_err());
    }
}

#[test]
fn stored_zip_preserves_previous_output_on_failure_and_replaces_on_success() {
    let fixture = Fixture::new();
    let source = fixture.0.join("source");
    let manifest = collect_bundle_manifest(&source).unwrap();
    let archive = fixture.0.join("archive.zip");
    write_deterministic_store_zip(&source, &archive, &manifest).unwrap();
    let previous = fs::read(&archive).unwrap();
    fs::remove_file(source.join("b.txt")).unwrap();
    assert!(write_deterministic_store_zip(&source, &archive, &manifest).is_err());
    assert_eq!(fs::read(&archive).unwrap(), previous);
    fs::write(source.join("b.txt"), "updated").unwrap();
    write_deterministic_store_zip(&source, &archive, &manifest).unwrap();
    assert_ne!(fs::read(&archive).unwrap(), previous);
    let inside = source.join("inside.zip");
    assert!(write_deterministic_store_zip(&source, &inside, &manifest).is_err());
    assert!(!inside.exists());
    assert_eq!(fs::read_dir(&fixture.0).unwrap().count(), 2);
}
