use eoie_source_map::{source_generated_residual, source_rebuild_pairs};

#[test]
fn cold_rebuild_uses_declared_owner_and_keeps_adapters_and_tests_as_consumers() {
    let root = std::env::temp_dir().join(format!("eoie-cold-owners-{}", std::process::id()));
    let owner = root.join("src/owner");
    std::fs::create_dir_all(&owner).unwrap();
    std::fs::write(
        owner.join("Cargo.toml"),
        r#"
[package]
name = "owner"
version = "0.0.0"
[lib]
path = "generated.rs"
[[test]]
name = "regression"
path = "regression.rs"
[package.metadata.spiral]
entry = "chosen.spi"
output = "generated.rs"
"#,
    )
    .unwrap();
    std::fs::write(owner.join("package.spiproj"), "modules:\n    chosen\n    main\n").unwrap();
    for name in ["chosen.spi", "main.spi"] {
        std::fs::write(owner.join(name), "inl main () : i32 = 0i32\n").unwrap();
    }
    for name in ["generated.rs", "adapter.rs", "regression.rs"] {
        std::fs::write(owner.join(name), "pub fn value() {}\n").unwrap();
    }
    assert_eq!(source_rebuild_pairs(&root).unwrap(), vec![(owner.join("chosen.spi"), owner.join("generated.rs"))]);
    assert!(source_generated_residual(&root, &owner.join("generated.rs")).unwrap());
    assert!(!source_generated_residual(&root, &owner.join("adapter.rs")).unwrap());
    assert!(!source_generated_residual(&root, &owner.join("regression.rs")).unwrap());
    std::fs::write(owner.join("Cargo.toml"), "[package.metadata.spiral]\nentry = 'missing.spi'\noutput = 'generated.rs'\n[package]\nname = 'owner'\n").unwrap();
    assert!(source_rebuild_pairs(&root).is_err());
    std::fs::remove_dir_all(&root).unwrap();
}
