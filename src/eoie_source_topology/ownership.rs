use std::path::{Component, Path, PathBuf};

fn declared_path(owner: &Path, value: &toml::Value, key: &str, extension: &str) -> Result<PathBuf, String> {
    let text = value.get(key).and_then(toml::Value::as_str).ok_or_else(|| format!("Spiral metadata needs string {key}: {}", owner.display()))?;
    let relative = Path::new(text);
    if text.is_empty() || relative.components().any(|part| !matches!(part, Component::Normal(_))) || relative.extension().and_then(|part| part.to_str()) != Some(extension) {
        return Err(format!("invalid Spiral {key} path: {text}"));
    }
    let mut path = owner.to_path_buf();
    let mut components = relative.components().peekable();
    while let Some(part) = components.next() {
        path.push(part);
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && key == "output" && components.peek().is_none() => return Ok(path),
            Err(error) => return Err(format!("Spiral {key} {}: {error}", path.display())),
        };
        if metadata.file_type().is_symlink() {
            return Err(format!("Spiral {key} rejects symlink: {}", path.display()));
        }
    }
    if !path.is_file() {
        return Err(format!("Spiral {key} is not a regular file: {}", path.display()));
    }
    Ok(path)
}

pub type SourceOwner = (PathBuf, PathBuf);

pub fn source_declared_pairs(owner: &Path) -> Result<Option<Vec<SourceOwner>>, String> {
    let manifest = owner.join("Cargo.toml");
    let metadata = match std::fs::symlink_metadata(&manifest) {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("manifest {}: {error}", manifest.display())),
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!("manifest must be a regular file: {}", manifest.display()));
    }
    let text = std::fs::read_to_string(&manifest).map_err(|error| format!("read {}: {error}", manifest.display()))?;
    let document = text.parse::<toml::Value>().map_err(|error| format!("parse {}: {error}", manifest.display()))?;
    let Some(declaration) = document.get("package").and_then(|package| package.get("metadata")).and_then(|metadata| metadata.get("spiral")) else {
        return Ok(None);
    };
    let auxiliary = match declaration.get("auxiliary") {
        Some(value) => value.as_array().ok_or_else(|| format!("Spiral auxiliary owners must be an array: {}", manifest.display()))?.as_slice(),
        None => &[],
    };
    let mut outputs = std::collections::BTreeSet::new();
    let mut pairs = Vec::new();
    for item in std::iter::once(declaration).chain(auxiliary.iter()) {
        let entry = declared_path(owner, item, "entry", "spi")?;
        let output = declared_path(owner, item, "output", "rs")?;
        let output_relative = output.strip_prefix(owner).map_err(|error| error.to_string())?;
        let target_matches = |target: &toml::Value| target.get("path").and_then(toml::Value::as_str).is_some_and(|path| Path::new(path) == output_relative);
        let library = usize::from(document.get("lib").is_some_and(target_matches));
        let executables = ["bin", "example", "test", "bench"]
            .into_iter()
            .map(|kind| document.get(kind).and_then(toml::Value::as_array).map_or(0, |targets| targets.iter().filter(|target| target_matches(target)).count()))
            .sum::<usize>();
        if library + executables != 1 {
            return Err(format!("Spiral output must identify one explicit Cargo target: {}", output.display()));
        }
        if !outputs.insert(output.clone()) {
            return Err(format!("duplicate Spiral output: {}", output.display()));
        }
        pairs.push((entry, output));
    }
    Ok(Some(pairs))
}

pub fn source_declared_owner(owner: &Path) -> Result<Option<SourceOwner>, String> {
    Ok(source_declared_pairs(owner)?.and_then(|pairs| pairs.into_iter().next()))
}

pub fn source_declared_residual(root: &Path, path: &Path) -> Result<Option<bool>, String> {
    let Some(mut owner) = path.parent() else {
        return Ok(None);
    };
    while owner.starts_with(root) {
        if let Some(pairs) = source_declared_pairs(owner)? {
            return Ok(Some(pairs.iter().any(|(_, output)| path == output)));
        }
        if owner == root {
            break;
        }
        let Some(parent) = owner.parent() else {
            break;
        };
        owner = parent;
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture(PathBuf);
    impl Fixture {
        fn new(metadata: &str) -> Self {
            let path = std::env::temp_dir().join(format!("eoie-ownership-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            std::fs::create_dir_all(&path).unwrap();
            std::fs::write(path.join("logic.spi"), "inl main () : i32 = 0i32\n").unwrap();
            std::fs::write(path.join("generated.rs"), "pub fn value() {}\n").unwrap();
            std::fs::write(path.join("adapter.rs"), "pub fn adapter() {}\n").unwrap();
            std::fs::write(path.join("Cargo.toml"), format!("[package]\nname = 'fixture'\n[lib]\npath = 'generated.rs'\n{metadata}")).unwrap();
            Self(path)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn explicit_owner_distinguishes_generated_output_from_adapter() {
        let fixture = Fixture::new("[package.metadata.spiral]\nentry = 'logic.spi'\noutput = 'generated.rs'\n");
        assert_eq!(source_declared_owner(&fixture.0).unwrap(), Some((fixture.0.join("logic.spi"), fixture.0.join("generated.rs"))));
        assert_eq!(source_declared_residual(&fixture.0, &fixture.0.join("generated.rs")).unwrap(), Some(true));
        assert_eq!(source_declared_residual(&fixture.0, &fixture.0.join("adapter.rs")).unwrap(), Some(false));
        let (_, surfaces) = crate::topology_collect_generic(&fixture.0).unwrap();
        assert_eq!(surfaces.generated_rust_files, 1);
        assert_eq!(surfaces.own_rust_files, 1);
    }

    #[test]
    fn spiral_directory_policy_classifies_only_reserved_names() {
        for (name, expected) in [(".git", 0), ("vendor", 1), ("target", 2), (".cache", 4), ("src", 3), ("retarget", 3)] {
            assert_eq!(crate::eoie_source_directory_kind(name), expected);
        }
    }

    #[test]
    fn build_cache_fixtures_do_not_change_source_counts() {
        let fixture = Fixture::new("");
        let before = crate::topology_collect(&fixture.0).unwrap();
        let (_, generic_before) = crate::topology_collect_generic(&fixture.0).unwrap();
        std::fs::create_dir(fixture.0.join(".cache")).unwrap();
        std::fs::write(fixture.0.join(".cache/generated.rs"), "// cached fixture\n".repeat(2000)).unwrap();
        let after = crate::topology_collect(&fixture.0).unwrap();
        let (_, generic_after) = crate::topology_collect_generic(&fixture.0).unwrap();
        assert_eq!(before.rust_lines, after.rust_lines);
        assert_eq!(generic_before.own_rust_files, generic_after.own_rust_files);
    }

    #[test]
    fn toml_comments_and_quoted_keys_are_supported() {
        let fixture = Fixture::new("[package.metadata.\"spiral\"] # declaration\nentry = \"logic.spi\" # input\noutput = \"generated.rs\"\n");
        assert!(source_declared_owner(&fixture.0).unwrap().is_some());
    }

    #[test]
    fn auxiliary_probes_have_explicit_ownership() {
        let fixture =
            Fixture::new("[package.metadata.spiral]\nentry = 'logic.spi'\noutput = 'generated.rs'\n[[package.metadata.spiral.auxiliary]]\nentry = 'logic.spi'\noutput = 'adapter.rs'\n[[example]]\nname = 'probe'\npath = 'adapter.rs'\n");
        assert_eq!(source_declared_pairs(&fixture.0).unwrap().unwrap().len(), 2);
        assert_eq!(source_declared_residual(&fixture.0, &fixture.0.join("adapter.rs")).unwrap(), Some(true));
    }

    #[test]
    fn cold_owner_can_have_a_missing_output_but_requires_its_source() {
        let fixture = Fixture::new("[package.metadata.spiral]\nentry = 'logic.spi'\noutput = 'generated.rs'\n");
        std::fs::remove_file(fixture.0.join("generated.rs")).unwrap();
        assert_eq!(source_declared_pairs(&fixture.0).unwrap().unwrap(), [(fixture.0.join("logic.spi"), fixture.0.join("generated.rs"))]);
        std::fs::remove_file(fixture.0.join("logic.spi")).unwrap();
        assert!(source_declared_pairs(&fixture.0).is_err());
    }

    #[cfg(windows)]
    #[test]
    fn source_stats_does_not_open_unrelated_binary_files() {
        use std::os::windows::fs::OpenOptionsExt;
        let fixture = Fixture::new("");
        let locked = std::fs::OpenOptions::new().write(true).create_new(true).share_mode(0).open(fixture.0.join("opaque.bin")).unwrap();
        assert!(crate::topology_collect_generic(&fixture.0).is_ok());
        drop(locked);
    }

    #[test]
    fn unrelated_metadata_is_not_a_generated_owner() {
        let fixture = Fixture::new("[package.metadata.other]\nnote = '''\n[package.metadata.spiral]\nentry = 'logic.spi'\noutput = 'generated.rs'\n'''\n");
        assert!(source_declared_owner(&fixture.0).unwrap().is_none());
    }

    #[test]
    fn malformed_or_stale_declarations_are_rejected() {
        for metadata in [
            "[package.metadata.spiral]\nentry = '../logic.spi'\noutput = 'generated.rs'\n",
            "[package.metadata.spiral]\nentry = 'logic.spi'\noutput = 'adapter.rs'\n",
            "[package.metadata.spiral]\nentry = 'missing.spi'\noutput = 'generated.rs'\n",
            "[package.metadata.spiral]\nentry = 42\noutput = 'generated.rs'\n",
            "[package.metadata.spiral]\nentry = 'logic.spi'\n",
            "[package.metadata.spiral]\nentry = 'logic.spi'\noutput = 'generated.rs'\nauxiliary = 42\n",
            "[package.metadata.spiral]\nentry = 'logic.spi'\noutput = 'generated.rs'\n[[package.metadata.spiral.auxiliary]]\nentry = 'logic.spi'\noutput = 'generated.rs'\n",
        ] {
            let fixture = Fixture::new(metadata);
            assert!(source_declared_owner(&fixture.0).is_err(), "accepted {metadata}");
        }
    }
}
