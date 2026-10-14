#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn eoie_binary() -> PathBuf {
    if let Ok(path) = env::var("EOIE_BIN_UNDER_TEST") { return PathBuf::from(path); }
    let current = env::current_exe().expect("current test executable");
    let name = if cfg!(windows) { "eoie.exe" } else { "eoie" };
    let debug = current.parent().and_then(Path::parent).expect("target profile directory");
    current.ancestors().skip(1).take_while(|directory| directory.file_name().is_some_and(|dir| dir != "target")).map(|directory| directory.join(name)).find(|path| path.is_file()).unwrap_or_else(|| debug.join(name))
}

fn proxy(args: &[&str]) -> Output {
    let mut words = vec!["proxy"];
    words.extend_from_slice(args);
    Command::new(eoie_binary()).current_dir(env::temp_dir()).args(&words).output().expect("run EOIE")
}

fn fs_write(root: &Path, relative: &str, flags: &[&str], text: &str) -> Output {
    let mut args = vec!["fs-write", root.to_str().expect("root"), relative];
    args.extend_from_slice(flags);
    if !flags.is_empty() { args.push("--"); }
    args.push(text);
    proxy(&args)
}

fn temporary(label: &str) -> PathBuf {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("clock").as_nanos();
    let root = env::temp_dir().join(format!("eoie-proxy-edit-{label}-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&root).expect("fixture root");
    root
}

fn receipt(output: &Output) -> String {
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn corrected(output: &Output) -> String {
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.lines().find_map(|line| line.strip_prefix("corrected: ")).expect("corrected line").to_owned()
}

#[test]
fn create_or_replace_creates_a_missing_file_and_replaces_an_existing_one() {
    let root = temporary("either");
    assert!(receipt(&fs_write(&root, "either.txt", &["--create-or-replace"], "created")).contains("mode=create"));
    assert_eq!(fs::read(root.join("either.txt")).expect("created"), b"created");
    let replaced = receipt(&fs_write(&root, "either.txt", &["--create-or-replace"], "replaced"));
    assert!(replaced.contains("mode=replace") && replaced.contains("create_or_replace=1"), "{replaced}");
    assert_eq!(fs::read(root.join("either.txt")).expect("replaced"), b"replaced");
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn a_refused_overwrite_suggests_the_same_command_with_create_or_replace() {
    let root = temporary("plain");
    let path = root.to_str().expect("root");
    receipt(&fs_write(&root, "value.txt", &[], "first"));
    let refused = fs_write(&root, "value.txt", &[], "it's two words");
    assert_eq!(corrected(&refused), format!("eoie proxy fs-write {path} value.txt --create-or-replace -- 'it''s two words'"));
    assert_eq!(fs::read(root.join("value.txt")).expect("kept"), b"first");
    receipt(&proxy(&["fs-write", path, "value.txt", "--create-or-replace", "--", "it's two words"]));
    assert_eq!(fs::read(root.join("value.txt")).expect("replaced"), b"it's two words");
    let flagged = fs_write(&root, "value.txt", &["--expect-nonempty"], "next");
    assert_eq!(corrected(&flagged), format!("eoie proxy fs-write {path} value.txt --create-or-replace --expect-nonempty -- next"));
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn create_or_replace_keeps_the_existing_file_guards() {
    let root = temporary("guards");
    assert!(!fs_write(&root, "missing.txt", &["--create-or-replace", "--expect-nonempty"], "x").status.success());
    assert!(!root.join("missing.txt").exists());
    fs::write(root.join("empty.txt"), b"").expect("empty fixture");
    assert!(!fs_write(&root, "empty.txt", &["--create-or-replace", "--expect-nonempty"], "x").status.success());
    assert_eq!(fs::read(root.join("empty.txt")).expect("still empty"), b"");
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn a_repeated_match_suggests_expecting_the_observed_count() {
    let root = temporary("repeated");
    let path = root.to_str().expect("root");
    fs::write(root.join("value.txt"), "a a a").expect("fixture");
    let refused = proxy(&["text-replace", "apply", path, "value.txt", "a", "b"]);
    assert_eq!(corrected(&refused), format!("eoie proxy text-replace apply {path} value.txt a b --expect 3"));
    receipt(&proxy(&["text-replace", "apply", path, "value.txt", "a", "b", "--expect", "3"]));
    assert_eq!(fs::read_to_string(root.join("value.txt")).expect("replaced"), "b b b");
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn a_single_match_against_a_wrong_expectation_suggests_dropping_it() {
    let root = temporary("single");
    let path = root.to_str().expect("root");
    fs::write(root.join("value.txt"), "one").expect("fixture");
    let refused = proxy(&["text-replace", "check", path, "value.txt", "one", "two", "--expect", "2"]);
    assert_eq!(corrected(&refused), format!("eoie proxy text-replace check {path} value.txt one two"));
    fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn a_failure_without_a_known_repair_keeps_the_usage() {
    let root = temporary("absent");
    let path = root.to_str().expect("root");
    fs::write(root.join("value.txt"), "one").expect("fixture");
    let refused = proxy(&["text-replace", "check", path, "value.txt", "zero", "two"]);
    assert!(corrected(&refused).starts_with("eoie proxy text-replace <check|apply>"));
    fs::remove_dir_all(root).expect("cleanup");
}

fn spiral_main() -> i32 {
    0i32
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 101 });
}
