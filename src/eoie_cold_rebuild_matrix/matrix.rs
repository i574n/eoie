#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
fn cold_native_root(root: &Path) -> Result<std::path::PathBuf, String> {
    let root = fs::canonicalize(root).map_err(|error| format!("cold root {}: {error}", root.display()))?;
    #[cfg(windows)] {
        let text = root.to_string_lossy();
        if let Some(unc) = text.strip_prefix(r"\\?\UNC\") { return Ok(std::path::PathBuf::from(format!(r"\\{unc}"))); }
        if let Some(disk) = text.strip_prefix(r"\\?\") { return Ok(std::path::PathBuf::from(disk)); }
    }
    Ok(root)
}

fn cold_snapshot_copy(root: &Path, from: &Path, to: &Path, outputs: &std::collections::HashSet<std::path::PathBuf>, inputs: &mut Vec<(String, String)>, total: &mut u64) -> Result<(), String> {
    fs::create_dir(to).map_err(|error| format!("create snapshot {}: {error}", to.display()))?;
    let mut entries = fs::read_dir(from).map_err(|error| error.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let name = entry.file_name();
        let metadata = fs::symlink_metadata(&path).map_err(|error| error.to_string())?;
        if metadata.file_type().is_symlink() { return Err(format!("cold snapshot rejects link: {}", path.display())); }
        let relative = cold_relative(root, &path)?;
        let checked = rooted_path(root, &relative)?;
        if metadata.is_dir() {
            if matches!(name.to_str(), Some(".cache" | "target" | "vendor" | ".git")) { continue; }
            cold_snapshot_copy(root, &checked, &to.join(name), outputs, inputs, total)?;
        } else if metadata.is_file() {
            if from == root && matches!(name.to_str(), Some("eoie" | "eoie.exe")) { continue; }
            *total = total.checked_add(metadata.len()).ok_or("cold snapshot size overflow")?;
            if *total > 268_435_456 { return Err("cold snapshot exceeds 256 MiB source budget".to_owned()); }
            let destination = to.join(name);
            let hash = eoie_proxy_search::inspection_file_sha256(&checked)?;
            fs::copy(&checked, &destination).map_err(|error| format!("snapshot {}: {error}", checked.display()))?;
            if eoie_proxy_search::inspection_file_sha256(&destination)? != hash { return Err(format!("source changed during snapshot: {relative}")); }
            if !outputs.contains(&path) { inputs.push((relative, hash)); }
        } else { return Err(format!("cold snapshot rejects special entry: {}", path.display())); }
    }
    Ok(())
}

fn cold_snapshot_manifest(snapshot: &Path) -> Result<serde_json::Value, String> {
    let text = eoie_rust_std_fs::read_regular_text_limited(&snapshot.join("manifest.json"), 8 * 1024 * 1024)?;
    let value: serde_json::Value = serde_json::from_str(&text).map_err(|error| format!("cold snapshot manifest: {error}"))?;
    if value["schema"].as_u64() != Some(1) { return Err("unsupported cold snapshot schema".to_owned()); }
    Ok(value)
}

fn cold_snapshot_verify(root: &Path, manifest: &serde_json::Value) -> Result<(), String> {
    let inputs = manifest["inputs"].as_array().ok_or("cold snapshot inputs missing")?;
    if inputs.is_empty() { return Err("cold snapshot inputs are empty".to_owned()); }
    for row in inputs {
        let relative = row[0].as_str().ok_or("invalid cold source path")?;
        let expected = row[1].as_str().ok_or("invalid cold source hash")?;
        let path = rooted_path(root, relative)?;
        if eoie_proxy_search::inspection_file_sha256(&path)? != expected { return Err(format!("source changed since cold planning: {relative}")); }
    }
    Ok(())
}

fn cold_snapshot_create(root: &Path, pairs: &[(std::path::PathBuf, std::path::PathBuf)]) -> Result<String, String> {
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|error| error.to_string())?.as_nanos();
    let relative = format!(".cache/cold-rebuild/{}-{stamp}", std::process::id());
    let snapshot = rooted_path(root, &relative)?;
    fs::create_dir_all(snapshot.parent().ok_or("cold snapshot parent missing")?).map_err(|error| error.to_string())?;
    fs::create_dir(&snapshot).map_err(|error| format!("claim cold snapshot: {error}"))?;
    let mut inputs = Vec::new();
    let outputs = pairs.iter().map(|(_, output)| output.clone()).collect::<std::collections::HashSet<_>>();
    cold_snapshot_copy(root, root, &snapshot.join("workspace"), &outputs, &mut inputs, &mut 0)?;
    let mut owners = Vec::new();
    for (input, output) in pairs {
        let previous = if eoie_rust_std_fs::rooted_entry_exists(output)? { Some(eoie_proxy_search::inspection_file_sha256(output)?) } else { None };
        owners.push((cold_relative(root, input)?, cold_relative(root, output)?, previous));
    }
    let manifest = serde_json::json!({"schema": 1, "inputs": inputs, "owners": owners});
    cold_snapshot_verify(root, &manifest)?;
    atomic_write(&snapshot.join("manifest.json"), &serde_json::to_vec(&manifest).map_err(|error| error.to_string())?)?;
    Ok(relative)
}

pub fn cold_rebuild_owner_run(args: &[String]) -> Result<(), String> {
    if args.len() != 10 { return Err("cold-rebuild-owner expects root snapshot-relative input-relative output-relative compiler dotnet rustfmt-or-empty timeout-ms workspace-root-or-empty".to_owned()); }
    let timeout_ms = args[8].parse::<u64>().map_err(|_| "compiler-timeout-ms must be an integer")?;
    if !(1..=600_000).contains(&timeout_ms) { return Err("compiler-timeout-ms must be between 1 and 600000".to_owned()); }
    let root = cold_native_root(Path::new(&args[1]))?;
    let snapshot = rooted_path(&root, &args[2])?;
    if !args[2].replace('\\', "/").starts_with(".cache/cold-rebuild/") { return Err("cold snapshot must be under .cache/cold-rebuild".to_owned()); }
    let manifest = cold_snapshot_manifest(&snapshot)?;
    let owners = manifest["owners"].as_array().ok_or("cold snapshot owners missing")?;
    let owner = owners.iter().find(|row| row[0].as_str() == Some(args[3].as_str()) && row[1].as_str() == Some(args[4].as_str())).ok_or("owner is not declared in cold snapshot")?;
    let output = rooted_path(&root, &args[4])?;
    let declared_input = rooted_path(&root, &args[3])?;
    if !source_rebuild_pairs(&root)?.iter().any(|(input, target)| input == &declared_input && target == &output) {
        return Err("cold owner does not match current Cargo metadata".to_owned());
    }
    cold_snapshot_verify(&root, &manifest)?;
    let workspace = snapshot.join("workspace");
    let input = rooted_path(&workspace, &args[3])?;
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|error| error.to_string())?.as_nanos();
    let products = snapshot.join("products");
    fs::create_dir_all(&products).map_err(|error| error.to_string())?;
    let candidate = products.join(format!("{}-{stamp}.rs", std::process::id()));
    let compiler = &args[5];
    let managed = Path::new(compiler).extension().and_then(|value| value.to_str()).is_some_and(|value| value.eq_ignore_ascii_case("dll"));
    let native = managed || Path::new(compiler).file_stem().and_then(|value| value.to_str()).is_some_and(|value| value.eq_ignore_ascii_case("SpiralCompiler"));
    let mut command = std::process::Command::new(if managed { &args[6] } else { compiler });
    command.current_dir(&workspace).env("SPIRAL_BUILD_BUDGET_MS", timeout_ms.saturating_sub(3000).max(1).to_string());
    if !args[9].is_empty() { command.env("SPIRAL_WORKSPACE_ROOT", &args[9]); }
    if managed { command.arg(compiler); }
    command.args(["--backend", "Rust"]);
    if !native { command.args(["--dotnet", &args[6], "--timeout-ms", &timeout_ms.to_string()]); }
    command.arg(&input).arg(&candidate);
    eoie_process_observation::run_bounded_streaming_checked(&mut command, timeout_ms, "cold-owner-compile", None)?;
    if !args[7].is_empty() {
        // through stdin: given a path, rustfmt also opens the file's out-of-line `mod x;` children next to it, and the
        // candidate sits in products/ without the owner's sibling modules (cold_proof_domain: rebuild_receipt_tests)
        let unformatted = eoie_rust_std_fs::read_regular_limited(&candidate, 16 * 1024 * 1024)?;
        let mut formatter = std::process::Command::new(&args[7]);
        formatter.current_dir(&workspace).args(["--edition", "2024", "--emit", "stdout", "--config-path"]).arg(workspace.join("src/rustfmt.toml"));
        let formatted = eoie_process_observation::run_bounded_observed_with_input(&mut formatter, 15000, Some(&unformatted))?;
        if !formatted.status.success() || formatted.stdout.is_empty() { return Err(format!("cold-owner-format failed: {}", String::from_utf8_lossy(&formatted.stderr))); }
        eoie_rust_std_fs::atomic_write(&candidate, &formatted.stdout)?;
    }
    let generated = eoie_rust_std_fs::read_regular_text_limited(&candidate, 16 * 1024 * 1024)?;
    if generated.trim().is_empty() { return Err("compiler produced an empty Rust output".to_owned()); }
    let generated = format!("{}\n", generated.replace("\r\n", "\n").trim_end());
    cold_snapshot_verify(&root, &manifest)?;
    let current = if eoie_rust_std_fs::rooted_entry_exists(&output)? { Some(eoie_rust_std_fs::read_regular_limited(&output, 16 * 1024 * 1024)?) } else { None };
    let unchanged = current.as_deref() == Some(generated.as_bytes());
    if !unchanged {
        let current_hash = current.as_ref().map(|_| eoie_proxy_search::inspection_file_sha256(&output)).transpose()?;
        if current_hash.as_deref() != owner[2].as_str() { return Err(format!("generated output changed since cold planning: {}", args[4])); }
        atomic_write(&output, generated.as_bytes())?;
    }
    println!("eoie proxy cold-rebuild-owner ok input={} output={} changed={} snapshot={}", args[3], args[4], i32::from(!unchanged), args[2]);
    Ok(())
}

use eoie_source_map::source_rebuild_pairs;
use eoie_rust_std_fs::{atomic_write, rooted_path};
use process_batch_plan_domain::encode_matrix_arguments;
use std::fs;
use std::path::Path;

fn cold_relative(root: &Path, path: &Path) -> Result<String,String> {
    let relative = path.strip_prefix(root).map_err(|_| format!("cold rebuild path escapes root: {}", path.display()))?;
    Ok(relative.to_str().ok_or("cold rebuild path is not UTF-8")?.replace('\\', "/"))
}

fn cold_tsv_cell(value: &str) -> Result<(),String> {
    if value.is_empty() || value.contains(['\t','\r','\n','\0']) {
        Err("cold rebuild path cannot be empty or contain TSV control characters".to_owned())
    } else { Ok(()) }
}

fn cold_matrix_write(root: &Path, relative: &str, eoie: &str, compiler: &str, dotnet: &str, rustfmt: &str, timeout_ms: u64) -> Result<(), String> {
    cold_tsv_cell(eoie)?;
    cold_tsv_cell(relative)?;
    for value in [compiler, dotnet, rustfmt] {
        if value.is_empty() { return Err("cold rebuild tool cannot be empty".to_owned()); }
    }
    let root = cold_native_root(root)?;
    let target = rooted_path(&root, relative)?;
    let matrix_dir = target.parent().unwrap_or(&root);
    let receipt_dir = matrix_dir.join(format!("{}.receipts", target.file_stem().unwrap().to_string_lossy()));
    let pairs = source_rebuild_pairs(&root)?;
    let snapshot = cold_snapshot_create(&root, &pairs)?;
    let mut out = String::from("id\tprogram\targs\tcwd\tenv\trequired_status\ton_error\tdepends_on\treceipt\n");
    let mut format_count = 0usize;
    for (index, (input, output)) in pairs.iter().enumerate() {
        let input_rel = cold_relative(&root, input)?;
        let owner_code = if input_rel == "src/backup_domain/main.spi" { 1 } else if input_rel == "src/eoie_agile_mutation/agile_mutation.spi" { 2 } else if input_rel == "src/eoie_agile_policy/policy.spi" { 3 } else if input_rel == "src/eoie_agile_state/state.spi" { 4 } else if input_rel == "src/eoie_process/process.spi" { 5 } else if input_rel == "src/eoie_agile_lease/lease.spi" { 6 } else if input_rel == "src/patch_compat_driver/main.spi" { 7 } else if input_rel == "src/eoie_proxy_search/search.spi" { 8 } else if input_rel == "src/bundle_retention_domain/main.spi" { 9 } else if input_rel == "src/eoie_release_hygiene/main.spi" { 10 } else if input_rel == "src/cold_proof_domain/main.spi" { 11 } else if input_rel == "src/eoie_coverage_prune/coverage.spi" { 12 } else if input_rel == "src/eoie_bundle_retention/main.spi" { 13 } else if input_rel == "src/eoie_bundle_zip/bundle.spi" { 14 } else if input_rel == "src/archive_deflate_domain/main.spi" { 15 } else if input_rel == "src/eoie_cold_rebuild_matrix/main.spi" { 16 } else { 0 };
        let format_lane = eoie_cold_format_lane(owner_code,0);
        if format_lane < 0 { return Err(format!("typed cold format lane rejected owner: {input_rel}")); }
        let compile_id = format!("compile-{index:03}");
        let arguments = vec!["proxy".to_owned(), "cold-rebuild-owner".to_owned(), root.to_string_lossy().into_owned(), snapshot.clone(),
            input_rel.clone(), cold_relative(&root, output)?, compiler.to_owned(), dotnet.to_owned(),
            if format_lane == 1 { rustfmt.to_owned() } else { String::new() }, timeout_ms.to_string(),
            std::env::var("EOIE_SPIRAL_BUNDLE").unwrap_or_default()];
        let receipt = cold_relative(&root, &receipt_dir.join(format!("{compile_id}.txt")))?;
        out.push_str(&format!("{compile_id}\t{eoie}\t{}\t.\t\tpass\tstop\t\t{receipt}\n", encode_matrix_arguments(&arguments)?));
        if format_lane == 1 { format_count += 1; }
    }
    fs::create_dir_all(matrix_dir).map_err(|e| format!("mkdir {}: {e}", matrix_dir.display()))?;
    atomic_write(&target, out.as_bytes())?;
    println!("eoie proxy cold-rebuild-matrix ok pairs={} rows={} single_flight_chains={} density_formats={} compiler_timeout_ms={} path={}",
        pairs.len(), pairs.len(), pairs.len(), format_count, timeout_ms, target.display());
    Ok(())
}

pub fn cold_rebuild_matrix_run(args: &[String]) -> Result<(), String> {
    if !(7..=8).contains(&args.len()) { return Err("cold-rebuild-matrix expects root matrix-relative eoie compiler dotnet rustfmt [compiler-timeout-ms]".to_owned()); }
    let timeout_ms = args.get(7).map(|value| value.parse::<u64>().map_err(|_| "compiler-timeout-ms must be an integer".to_owned())).transpose()?.unwrap_or(180_000);
    if !(1..=600_000).contains(&timeout_ms) { return Err("compiler-timeout-ms must be between 1 and 600000".to_owned()); }
    cold_matrix_write(Path::new(&args[1]), &args[2], &args[3], &args[4], &args[5], &args[6], timeout_ms)
}

fn method0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 == 0i32;
    if v2 {
        let mut v3: bool = v0 < 0i32;
        if v3 {
            -1i32
        } else {
            let mut v4: bool = 16i32 < v0;
            if v4 {
                -1i32
            } else {
                let mut v5: bool = v0 == 10i32;
                if v5 {
                    0i32
                } else {
                    let mut v6: bool = v0 == 14i32;
                    if v6 {
                        0i32
                    } else {
                        let mut v7: bool = 6i32 < v0;
                        if v7 {
                            1i32
                        } else {
                            0i32
                        }
                    }
                }
            }
        }
    } else {
        -1i32
    }
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method0(v0, v1)
    }); }
    CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_cold_format_lane(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
