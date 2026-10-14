use std::{env, fs, path::PathBuf, process::{Command, Output}, time::{SystemTime, UNIX_EPOCH}};

struct Fixture { root: PathBuf, binary: PathBuf }
impl Fixture {
    fn new(name: &str) -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = env::temp_dir().join(format!("eoie receipts ü {name} {} {stamp}", std::process::id()));
        fs::create_dir_all(root.join("dir")).unwrap();
        fs::write(root.join("value.txt"), "alpha\nbeta\ngamma\n").unwrap();
        fs::write(root.join("dir").join("inner.txt"), "beta inner\n").unwrap();
        let binary = env::var_os("EOIE_BIN_UNDER_TEST").map(PathBuf::from).unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().parent().unwrap().join(format!("eoie{}", env::consts::EXE_SUFFIX))
        });
        Self { root, binary }
    }
    fn root_text(&self) -> String { self.root.display().to_string() }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(&self.binary).current_dir(&self.root).env_remove("EOIE_LEASE_ROOT").env_remove("EOIE_CONTROL_READ_ONLY")
            .env("EOIE_LEASE_POLICY", "renewing").env("EOIE_CONFIG_HOME", &self.root)
            .args(args).output().unwrap()
    }
    fn proxy(&self, verb: &str, rest: &[&str]) -> Output {
        let root = self.root_text();
        let mut args = vec!["proxy", verb, root.as_str()];
        args.extend_from_slice(rest);
        self.run(&args)
    }
    fn text_replace(&self, mode: &str, rest: &[&str]) -> Output {
        let root = self.root_text();
        let mut args = vec!["proxy", "text-replace", mode, root.as_str()];
        args.extend_from_slice(rest);
        self.run(&args)
    }
    fn ok_text_replace(&self, mode: &str, rest: &[&str]) -> String {
        let output = self.text_replace(mode, rest);
        assert!(output.status.success(), "text-replace {mode} {rest:?}: {output:?}");
        self.receipt(&output, "eoie proxy text-replace ")
    }
    fn receipt(&self, output: &Output, prefix: &str) -> String {
        let text = format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        let line = text.lines().find(|line| line.starts_with(prefix)).unwrap_or_else(|| panic!("no line starting with {prefix:?}: {output:?}"));
        normalized_receipt(&line.replace(&self.root_text(), "<root>"))
    }
    fn ok_receipt(&self, verb: &str, rest: &[&str]) -> String {
        let output = self.proxy(verb, rest);
        assert!(output.status.success(), "{verb} {rest:?}: {output:?}");
        self.receipt(&output, &format!("eoie proxy {verb} "))
    }
}
impl Drop for Fixture { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.root); } }

const MEASURED_FIELDS: [&str; 4] = ["elapsed_ms", "serial_ms", "parallel_ms", "workers"];

fn normalized_receipt(line: &str) -> String {
    line.split(' ').map(|token| match token.split_once('=') {
        Some((key, _)) if MEASURED_FIELDS.contains(&key) => format!("{key}=N"),
        _ => token.replace('\\', "/"),
    }).collect::<Vec<_>>().join(" ")
}

#[test]
fn file_receipts_name_their_effect() {
    let f = Fixture::new("files");
    assert_eq!(f.ok_receipt("fs-read", &["value.txt"]), "eoie proxy fs-read ok bytes=17 shown=17 elapsed_ms=N");
    assert_eq!(f.ok_receipt("fs-list", &["."]), "eoie proxy fs-list ok path=<root> entries=2 elapsed_ms=N");
    assert_eq!(f.ok_receipt("fs-write", &["new.txt", "hello"]),
        "eoie proxy fs-write ok mode=create path=<root>/new.txt bytes=5 replace_existing=0 create_or_replace=0 expect_hash=0 expect_nonempty=0 elapsed_ms=N");
    assert_eq!(f.ok_receipt("fs-write", &["new.txt", "--create-or-replace", "--", "hello2"]),
        "eoie proxy fs-write ok mode=replace path=<root>/new.txt bytes=6 replace_existing=0 create_or_replace=1 expect_hash=0 expect_nonempty=0 elapsed_ms=N");
    assert_eq!(fs::read_to_string(f.root.join("new.txt")).unwrap(), "hello2");
    assert_eq!(f.ok_receipt("fs-copy", &["value.txt", "copy.txt"]),
        "eoie proxy fs-copy ok source=<root>/value.txt target=<root>/copy.txt bytes=17 elapsed_ms=N");
    assert_eq!(f.ok_receipt("fs-remove", &["copy.txt"]), "eoie proxy fs-remove ok path=<root>/copy.txt kind=file bytes=17 elapsed_ms=N");
    assert!(!f.root.join("copy.txt").exists());
    assert_eq!(f.ok_receipt("fs-remove-tree", &["dir"]), "eoie proxy fs-remove-tree ok removed=2 missing_ok=0 elapsed_ms=N");
    assert!(!f.root.join("dir").exists());
}

#[test]
fn text_replace_receipts_count_matches_and_bytes() {
    let f = Fixture::new("replace");
    fs::write(f.root.join("copy.txt"), "alpha\nbeta\nbeta\n").unwrap();
    assert_eq!(f.ok_text_replace("check", &["copy.txt", "alpha", "ALPHA"]),
        "eoie proxy text-replace ok mode=check path=<root>/copy.txt matches=1 expected=1 bytes_before=16 bytes_after=16 elapsed_ms=N");
    assert_eq!(fs::read_to_string(f.root.join("copy.txt")).unwrap(), "alpha\nbeta\nbeta\n");
    assert_eq!(f.ok_text_replace("apply", &["copy.txt", "beta", "B", "--all"]),
        "eoie proxy text-replace ok mode=apply path=<root>/copy.txt matches=2 expected=all bytes_before=16 bytes_after=10 elapsed_ms=N");
    assert_eq!(fs::read_to_string(f.root.join("copy.txt")).unwrap(), "alpha\nB\nB\n");
    let mismatch = f.text_replace("apply", &["copy.txt", "B", "C", "--expect", "3"]);
    assert_eq!(mismatch.status.code(), Some(2), "{mismatch:?}");
    assert_eq!(f.receipt(&mismatch, "eoie error: "),
        "eoie error: text-replace expected 3 match(es), observed matches=2 (use --expect N or --all for repeated text)");
    assert_eq!(f.receipt(&mismatch, "corrected: "), "corrected: eoie proxy text-replace apply '<root>' copy.txt B C --expect 2");
    assert_eq!(fs::read_to_string(f.root.join("copy.txt")).unwrap(), "alpha\nB\nB\n");
}

#[test]
fn search_receipts_report_hits_exclusions_and_truncation() {
    let f = Fixture::new("search");
    assert_eq!(f.ok_receipt("fs-search", &[".", "beta"]),
        "eoie proxy fs-search ok hits=2 excludes=.git,node_modules,build-output,symlinks elapsed_ms=N");
    assert_eq!(f.ok_receipt("parallel-search", &[".", "beta", "10"]),
        "eoie proxy parallel-search ok workers=N hits=2 parity=exact ordering=deterministic excludes=.git,node_modules,build-output,symlinks serial_ms=N parallel_ms=N elapsed_ms=N");
    assert_eq!(f.ok_receipt("parallel-search", &[".", "beta", "1"]),
        "eoie proxy parallel-search ok workers=N hits=1 truncated=true found_at_least=2 parity=exact ordering=deterministic excludes=.git,node_modules,build-output,symlinks serial_ms=N parallel_ms=N elapsed_ms=N");
}

#[test]
fn inspection_receipts_keep_digests_on_stdout() {
    let f = Fixture::new("inspect");
    assert_eq!(f.ok_receipt("fs-slice", &["value.txt", "2", "3"]), "eoie proxy fs-slice ok start=2 lines=2 requested=3 end_of_file=true elapsed_ms=N");
    assert_eq!(f.ok_receipt("fs-context", &["value.txt", "beta", "1"]), "eoie proxy fs-context ok offset=6 window_bytes=6 file_bytes=17 elapsed_ms=N");
    let hash = f.proxy("hash", &["value.txt"]);
    assert!(hash.status.success(), "{hash:?}");
    assert_eq!(String::from_utf8_lossy(&hash.stdout).trim(), "4fdbc441ea7b546100e086ac1e4fc5ae6749b7314311c99db05be450eca12996");
    assert_eq!(f.receipt(&hash, "eoie proxy hash "), "eoie proxy hash ok bytes=17 elapsed_ms=N");
    let tree = f.proxy("hash-tree", &["dir"]);
    assert!(tree.status.success(), "{tree:?}");
    assert_eq!(String::from_utf8_lossy(&tree.stdout).trim().len(), 64, "{tree:?}");
    assert_eq!(f.receipt(&tree, "eoie proxy hash-tree "), "eoie proxy hash-tree ok relative=dir elapsed_ms=N");
    let manifest = f.ok_receipt("parallel-manifest", &["dir", "100"]);
    let (head, tail) = manifest.split_once(" sha256=").unwrap();
    assert_eq!(head, "eoie proxy parallel-manifest ok workers=N entries=2");
    assert_eq!(tail.split_once(' ').unwrap().1, "parity=exact ordering=deterministic serial_ms=N parallel_ms=N elapsed_ms=N");
}

#[test]
fn command_output_receipt_propagates_the_child_exit() {
    let f = Fixture::new("command");
    let binary = f.binary.display().to_string();
    let ok = f.proxy("command-output", &["30000", binary.as_str(), "help"]);
    assert!(ok.status.success(), "{ok:?}");
    let ok_receipt = f.receipt(&ok, "eoie proxy command-output ");
    assert!(ok_receipt.starts_with("eoie proxy command-output ok status=0 elapsed_ms=N stdout_bytes="), "{ok_receipt}");
    assert!(ok_receipt.ends_with(" stderr_bytes=0"), "{ok_receipt}");
    let failed = f.proxy("command-output", &["30000", binary.as_str(), "not-a-command"]);
    assert_eq!(failed.status.code(), Some(2), "{failed:?}");
    let failed_receipt = f.receipt(&failed, "eoie proxy command-output ");
    assert!(failed_receipt.starts_with("eoie proxy command-output child-failed status=2 elapsed_ms=N stdout_bytes="), "{failed_receipt}");
}
