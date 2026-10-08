use super::{eoie_cold_rebuild_expected_shards, verify_public_cold_rebuild};
use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("eoie cold counts ü {} {stamp}", std::process::id()));
        let owner = root.join("src/owner");
        fs::create_dir_all(&owner).unwrap();
        fs::create_dir(root.join("state")).unwrap();
        fs::write(owner.join("package.spiproj"), "modules:\n    main\n    probe\n").unwrap();
        for name in ["main", "probe"] {
            fs::write(owner.join(format!("{name}.spi")), "inl main () : i32 = 0i32\n").unwrap();
            fs::write(owner.join(format!("{name}.rs")), "fn main() {}\n").unwrap();
        }
        fs::write(owner.join("adapter.rs"), "// handwritten consumer\n").unwrap();
        fs::write(owner.join("Cargo.toml"), "[package]\nname='owner'\nversion='0.0.0'\n[lib]\npath='main.rs'\n[[example]]\nname='probe'\npath='probe.rs'\n[package.metadata.spiral]\nentry='main.spi'\noutput='main.rs'\n[[package.metadata.spiral.auxiliary]]\nentry='probe.spi'\noutput='probe.rs'\n").unwrap();
        Self(root)
    }
    fn receipt(&self, owners: u32, width: u32, shards: u32) {
        fs::write(self.0.join("state/cold_rebuild.spi"), format!("inl current_cold_rebuild_schema () : cold_rebuild_schema = ColdRebuildV1\ninl current_cold_rebuild_status () : cold_rebuild_status = ColdRebuildClosed\ninl current_owner_count () : u32 = {owners}u32\ninl current_shard_width () : u32 = {width}u32\ninl current_shard_count () : u32 = {shards}u32\ninl current_gate_flags () : u32 = 63u32\ninl current_max_owner_compile_ms () : u32 = 1u32\ninl current_max_global_gate_ms () : u32 = 1u32\n")).unwrap();
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }

#[test]
fn shard_arithmetic_rounds_up_without_overflow() {
    for owners in [1, 2, 3, 4, 5, i32::MAX - 1, i32::MAX] {
        for width in [1, 2, 4] {
            let expected = (i64::from(owners) + i64::from(width) - 1) / i64::from(width);
            assert_eq!(i64::from(eoie_cold_rebuild_expected_shards(owners, width)), expected);
        }
    }
    for (owners, width) in [(0, 1), (-1, 2), (1, 0), (1, 3), (1, i32::MAX)] {
        assert_eq!(eoie_cold_rebuild_expected_shards(owners, width), -1);
    }
}

#[test]
fn cold_receipt_rejects_counts_that_wrap_signed_transport() {
    let fixture = Fixture::new();
    for (owners, width, shards) in [(u32::MAX, 1, u32::MAX), (1, u32::MAX, u32::MAX), (2, 1, u32::MAX), (2_147_483_648, 2, 1_073_741_824), (0, 1, 0)] {
        fixture.receipt(owners, width, shards);
        assert!(verify_public_cold_rebuild(&fixture.0).unwrap_err().contains("shape rejected"));
    }
}

#[test]
fn cold_receipt_counts_declared_auxiliary_outputs_and_excludes_adapters() {
    let fixture = Fixture::new();
    fixture.receipt(1, 1, 1);
    assert!(verify_public_cold_rebuild(&fixture.0).unwrap_err().contains("inventory drift receipt=1 declared=2"));
    fixture.receipt(2, 2, 1);
    assert!(verify_public_cold_rebuild(&fixture.0).unwrap_err().contains("gate missing: contract"));
    fs::remove_file(fixture.0.join("src/owner/probe.spi")).unwrap();
    assert!(verify_public_cold_rebuild(&fixture.0).unwrap_err().contains("Spiral entry"));
}
