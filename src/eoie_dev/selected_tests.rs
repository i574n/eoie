#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
const DEV_BIN: &str = env!("CARGO_BIN_EXE_eoie-dev");
#[cfg(test)] mod native_selected_tests {
struct Fixture(std::path::PathBuf);
impl Fixture { fn new() -> Self { let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos(); let root=std::env::temp_dir().join(format!("eoie native workflow ü {} {stamp}",std::process::id())); std::fs::create_dir(&root).unwrap(); Self(root) } }
impl Drop for Fixture { fn drop(&mut self) { let _=std::fs::remove_dir_all(&self.0); } }
#[test] fn selected_packages_preserve_native_build_contracts() { let fixture=Fixture::new(); assert_eq!(super::eoie_dev_selected_scenarios(fixture.0.to_str().unwrap()),0); }
}

fn method1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> () {
    { let path=std::path::Path::new(&*v0).join(&*v1); std::fs::create_dir_all(path.parent().unwrap()).unwrap(); std::fs::write(path,v2.as_bytes()).unwrap(); };
    ()
}
fn method2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from("[package]\nname='eoie-coverage-prune'\nversion='0.0.0'\nedition='2024'\n[lib]\npath='lib.rs'\n[package.metadata.eoie]\ntest-requires-cli=");
    let mut v2: Rc<str> = Rc::<str>::from("\n");
    let mut v3: Rc<str> = Rc::<str>::from("");
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v3, &*v3].concat());
    v4.clone()
}
fn method3(mut v0: bool, mut v1: Rc<str>) -> () {
    if v0 { () } else { std::panic::panic_any(v1.to_string()) };
    ()
}
fn method5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from("--test");
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v3: Rc<str> = Rc::<str>::from("--skip-release");
    let mut v4: Rc<str> = Rc::<str>::from("");
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v4, &*v4].concat());
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v7: Rc<str> = Rc::<str>::from("--offline");
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v6, &*v7, &*v4, &*v4].concat());
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v10: Rc<str> = Rc::<str>::from("--package");
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v10, &*v4, &*v4].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v0, &*v4, &*v4].concat());
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v15: Rc<str> = Rc::<str>::from("--target-dir");
    let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v4, &*v4].concat());
    let mut v17: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v18: Rc<str> = Rc::<str>::from("target with spaces ü");
    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v17, &*v18, &*v4, &*v4].concat());
    v19.clone()
}
fn method6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let mut command=std::process::Command::new(DEV_BIN); command.current_dir(&*v0).args(v1.split(char::from(0u8))); let capture=eoie_process::run_bounded_capture_with_input(&mut command,120000,None).unwrap(); let status=if capture.status.success() { "success
" } else { "failure
" }; std::rc::Rc::<str>::from([status,&String::from_utf8_lossy(&capture.stdout),&String::from_utf8_lossy(&capture.stderr)].concat()) };
    v2.clone()
}
fn method4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: Rc<str>) -> () {
    let mut v4: Rc<str> = method5(v1.clone());
    let mut v5: Rc<str> = method6(v0.clone(), v4.clone());
    let mut v8: Rc<str> = if v2 {
        let mut v6: Rc<str> = Rc::<str>::from("success\n");
        v6.clone()
    } else {
        let mut v7: Rc<str> = Rc::<str>::from("failure\n");
        v7.clone()
    };
    let mut v9: bool = v5.starts_with(&*v8);
    method3(v9, v5.clone());
    let mut v10: bool = v5.contains(&*v3);
    method3(v10, v5.clone())
}
fn method0(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = Rc::<str>::from("src/Cargo.toml");
    let mut v2: Rc<str> = Rc::<str>::from("[workspace]\nmembers=['cli','client','isolated']\nresolver='3'\n");
    method1(v0.clone(), v1.clone(), v2.clone());
    let mut v3: Rc<str> = Rc::<str>::from("src/cli/Cargo.toml");
    let mut v4: Rc<str> = Rc::<str>::from("[package]\nname='eoie-cli'\nversion='0.0.0'\nedition='2024'\n[[bin]]\nname='eoie'\npath='main.rs'\n");
    method1(v0.clone(), v3.clone(), v4.clone());
    let mut v5: Rc<str> = Rc::<str>::from("src/cli/main.rs");
    let mut v6: Rc<str> = Rc::<str>::from("fn main() { println!(\"first\"); }\n");
    method1(v0.clone(), v5.clone(), v6.clone());
    let mut v7: Rc<str> = Rc::<str>::from("src/client/Cargo.toml");
    let mut v8: Rc<str> = Rc::<str>::from("true");
    let mut v9: Rc<str> = method2(v8.clone());
    method1(v0.clone(), v7.clone(), v9.clone());
    let mut v10: Rc<str> = Rc::<str>::from("src/client/lib.rs");
    let mut v11: Rc<str> = Rc::<str>::from("#[test] fn fresh_cli_is_available() { let current=std::env::current_exe().unwrap(); let binary=current.parent().unwrap().parent().unwrap().join(format!(\"eoie{}\",std::env::consts::EXE_SUFFIX)); let output=std::process::Command::new(binary).output().expect(\"selected test requires the CLI\"); assert!(output.status.success()); assert_eq!(String::from_utf8(output.stdout).unwrap().trim(),include_str!(\"expected.txt\").trim()); }\n");
    method1(v0.clone(), v10.clone(), v11.clone());
    let mut v12: Rc<str> = Rc::<str>::from("src/client/expected.txt");
    let mut v13: Rc<str> = Rc::<str>::from("first");
    method1(v0.clone(), v12.clone(), v13.clone());
    let mut v14: Rc<str> = Rc::<str>::from("src/isolated/Cargo.toml");
    let mut v15: Rc<str> = Rc::<str>::from("[package]\nname='isolated'\nversion='0.0.0'\nedition='2024'\n[lib]\npath='lib.rs'\n");
    method1(v0.clone(), v14.clone(), v15.clone());
    let mut v16: Rc<str> = Rc::<str>::from("src/isolated/lib.rs");
    let mut v17: Rc<str> = Rc::<str>::from("#[test] fn independent() { assert_eq!(2+2,4); }\n");
    method1(v0.clone(), v16.clone(), v17.clone());
    let mut v18: Rc<str> = Rc::<str>::from("generate-lockfile");
    let mut v19: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v20: Rc<str> = Rc::<str>::from("--offline");
    let mut v21: Rc<str> = Rc::<str>::from("");
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v21, &*v21].concat());
    let mut v23: i32 = { let mut command=std::process::Command::new("cargo"); command.current_dir(std::path::Path::new(&*v0).join("src")); command.args(v22.split(char::from(0u8))); if v21.len() > 0 { command.env("EOIE_SPIRAL_COMPILE", &*v21); } if std::env::var_os("CARGO_BUILD_JOBS").is_none() { command.env("CARGO_BUILD_JOBS", "2"); } if std::env::var_os("RUST_TEST_THREADS").is_none() { command.env("RUST_TEST_THREADS", "1"); } match eoie_process::run_bounded(&mut command,60000u64) { Ok(status) => status.code().unwrap_or(1), Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); 2 } } };
    let mut v24: bool = v23 == 0i32;
    let mut v25: Rc<str> = Rc::<str>::from("fixture lock generation failed");
    method3(v24, v25.clone());
    let mut v26: Rc<str> = Rc::<str>::from("eoie-coverage-prune");
    let mut v27: bool = true;
    let mut v28: Rc<str> = Rc::<str>::from("test result: ok");
    method4(v0.clone(), v26.clone(), v27, v28.clone());
    let mut v29: Rc<str> = Rc::<str>::from("fn main() { println!(\"second\"); }\n");
    method1(v0.clone(), v5.clone(), v29.clone());
    let mut v30: Rc<str> = Rc::<str>::from("second");
    method1(v0.clone(), v12.clone(), v30.clone());
    let mut v31: bool = true;
    method4(v0.clone(), v26.clone(), v31, v28.clone());
    let mut v32: Rc<str> = Rc::<str>::from("compile_error!(\"unselected CLI must not compile\"); fn main() {}\n");
    method1(v0.clone(), v5.clone(), v32.clone());
    let mut v33: Rc<str> = Rc::<str>::from("isolated");
    let mut v34: bool = true;
    method4(v0.clone(), v33.clone(), v34, v28.clone());
    let mut v35: Rc<str> = Rc::<str>::from("missing-package");
    let mut v36: bool = false;
    let mut v37: Rc<str> = Rc::<str>::from("Unknown selected Cargo package");
    method4(v0.clone(), v35.clone(), v36, v37.clone());
    let mut v38: Rc<str> = Rc::<str>::from("'yes'");
    let mut v39: Rc<str> = method2(v38.clone());
    method1(v0.clone(), v7.clone(), v39.clone());
    let mut v40: bool = false;
    let mut v41: Rc<str> = Rc::<str>::from("test-requires-cli must be a boolean");
    method4(v0.clone(), v26.clone(), v40, v41.clone());
    let mut v42: Rc<str> = Rc::<str>::from("--skip-release");
    let mut v43: Rc<str> = method6(v0.clone(), v42.clone());
    let mut v44: Rc<str> = Rc::<str>::from("require --test");
    let mut v45: bool = v43.contains(&*v44);
    method3(v45, v43.clone());
    let mut v46: Rc<str> = Rc::<str>::from("--timeout-ms");
    let mut v47: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v48: Rc<str> = Rc::<str>::from("0");
    let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v46, &*v47, &*v48, &*v21, &*v21].concat());
    let mut v50: Rc<str> = method6(v0.clone(), v49.clone());
    let mut v51: Rc<str> = Rc::<str>::from("--timeout-ms must be between");
    let mut v52: bool = v50.contains(&*v51);
    method3(v52, v50.clone());
    let mut v53: Rc<str> = Rc::<str>::from("--package");
    let mut v54: Rc<str> = method6(v0.clone(), v53.clone());
    let mut v55: Rc<str> = Rc::<str>::from("missing value");
    let mut v56: bool = v54.contains(&*v55);
    method3(v56, v54.clone());
    let mut v57: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(DEV_BIN).parent().unwrap().parent().unwrap().to_str().unwrap());
    let mut v58: Rc<str> = Rc::<str>::from("--target-dir");
    let mut v59: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v60: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v59, &*v57, &*v21, &*v21].concat());
    let mut v61: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v62: Rc<str> = Rc::<str>::from("--test");
    let mut v63: Rc<str> = std::rc::Rc::<str>::from([&*v60, &*v61, &*v62, &*v21, &*v21].concat());
    let mut v64: Rc<str> = method6(v0.clone(), v63.clone());
    let mut v65: Rc<str> = Rc::<str>::from("bootstrap eoie-dev with a separate --target-dir");
    let mut v66: bool = v64.contains(&*v65);
    method3(v66, v64.clone());
    let mut v67: Rc<str> = method5(v33.clone());
    let mut v68: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v69: Rc<str> = Rc::<str>::from("--dry-run");
    let mut v70: Rc<str> = std::rc::Rc::<str>::from([&*v67, &*v68, &*v69, &*v21, &*v21].concat());
    let mut v71: Rc<str> = method6(v0.clone(), v70.clone());
    let mut v72: Rc<str> = Rc::<str>::from("target with spaces ü");
    let mut v73: bool = v71.contains(&*v72);
    method3(v73, v71.clone());
    let mut v74: Rc<str> = Rc::<str>::from("cargo \"test\"");
    let mut v75: bool = v71.contains(&*v74);
    method3(v75, v71.clone());
    let mut v76: Rc<str> = Rc::<str>::from("#[test] fn deliberate_failure() { panic!(\"deliberate native workflow test failure\"); }\n");
    method1(v0.clone(), v16.clone(), v76.clone());
    let mut v77: bool = false;
    let mut v78: Rc<str> = Rc::<str>::from("deliberate native workflow test failure");
    method4(v0.clone(), v33.clone(), v77, v78.clone());
    0i32
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    Rc::new(move |mut v0: Rc<str>| -> i32 {
        method0(v0.clone())
    })
}
pub fn eoie_dev_selected_scenarios(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
