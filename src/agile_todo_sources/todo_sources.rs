#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn walked_directory_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_list_names(std::path::Path::new(&*v0)).unwrap_or_default().join(&char::from(10u8).to_string()).as_str());
    let mut v4: i32 = 0i32;
    walked_names_3(v0.clone(), v1.clone(), v3.clone(), v4, v2.clone())
}
fn walked_names_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: i32 = v2.len() as i32;
        let mut v6: bool = v5 <= v3;
        if v6 {
            return v4.clone();
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v2.get(v3 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v3 as usize) as i32).unwrap_or(-1);
            let mut v9: bool = v8 < 0i32;
            let mut v11: i32 = if v9 {
                let mut v10: i32 = v2.len() as i32;
                v10
            } else {
                v8
            };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from(v2.get(v3 as usize..v11 as usize).unwrap_or(""));
            let mut v13: i32 = v11.wrapping_add(1i32);
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: bool = v0 == v14 ;
            let mut v18: Rc<str> = if v15 {
                v12.clone()
            } else {
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v16, &*v12, &*v14, &*v14].concat());
                v17.clone()
            };
            let mut v19: bool = v1 == v14 ;
            let mut v22: Rc<str> = if v19 {
                v12.clone()
            } else {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v20, &*v12, &*v14, &*v14].concat());
                v21.clone()
            };
            let mut v23: bool = v12 == v14 ;
            let mut v43: Rc<str> = if v23 {
                v4.clone()
            } else {
                let mut v24: i32 = std::fs::symlink_metadata(&*v18).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
                let mut v25: bool = v24 == 2i32;
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules|vendor|third_party|target|dist|_build|.zig-cache|zig-out|.lake|__pycache__|.venv|venv|.gradle|.next|bin|obj|deps|.git|.cache|elm-stuff|bower_components|.dart_tool|.mypy_cache|.pytest_cache|.tox"); } LIT.with(|lit| lit.clone()) };
                let mut v27: bool = v26.split("|").any(|item| item == &*v12);
                let mut v28: bool = v25 && v27;
                if v28 {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let mut v30: bool = v22 == v14 ;
                    let mut v33: Rc<str> = if v30 {
                        v29.clone()
                    } else {
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                        let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v31, &*v29, &*v14, &*v14].concat());
                        v32.clone()
                    };
                    let mut v34: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                    let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v33, &*v34, &*v14, &*v14].concat());
                    v35.clone()
                } else {
                    let mut v36: i32 = std::fs::symlink_metadata(&*v18).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
                    let mut v37: bool = v36 == 2i32;
                    if v37 {
                        walked_directory_4(v18.clone(), v22.clone(), v4.clone())
                    } else {
                        let mut v39: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                        let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v22, &*v39, &*v14, &*v14].concat());
                        v40.clone()
                    }
                }
            };
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v13, v43.clone());
            continue;
        }
    }
}
fn walked_names_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> Rc<str> {
    let mut v4: i32 = v2.len() as i32;
    let mut v5: bool = v4 <= v3;
    if v5 {
        v1.clone()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v7: i32 = v2.get(v3 as usize..).and_then(|rest| rest.find(&*v6)).map(|found| (found + v3 as usize) as i32).unwrap_or(-1);
        let mut v8: bool = v7 < 0i32;
        let mut v10: i32 = if v8 {
            let mut v9: i32 = v2.len() as i32;
            v9
        } else {
            v7
        };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from(v2.get(v3 as usize..v10 as usize).unwrap_or(""));
        let mut v12: i32 = v10.wrapping_add(1i32);
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v14: bool = v0 == v13 ;
        let mut v17: Rc<str> = if v14 {
            v11.clone()
        } else {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v15, &*v11, &*v13, &*v13].concat());
            v16.clone()
        };
        let mut v18: bool = v1 == v13 ;
        let mut v21: Rc<str> = if v18 {
            v11.clone()
        } else {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v19, &*v11, &*v13, &*v13].concat());
            v20.clone()
        };
        let mut v22: bool = v11 == v13 ;
        let mut v42: Rc<str> = if v22 {
            v1.clone()
        } else {
            let mut v23: i32 = std::fs::symlink_metadata(&*v17).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
            let mut v24: bool = v23 == 2i32;
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules|vendor|third_party|target|dist|_build|.zig-cache|zig-out|.lake|__pycache__|.venv|venv|.gradle|.next|bin|obj|deps|.git|.cache|elm-stuff|bower_components|.dart_tool|.mypy_cache|.pytest_cache|.tox"); } LIT.with(|lit| lit.clone()) };
            let mut v26: bool = v25.split("|").any(|item| item == &*v11);
            let mut v27: bool = v24 && v26;
            if v27 {
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let mut v29: bool = v21 == v13 ;
                let mut v32: Rc<str> = if v29 {
                    v28.clone()
                } else {
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v30, &*v28, &*v13, &*v13].concat());
                    v31.clone()
                };
                let mut v33: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v32, &*v33, &*v13, &*v13].concat());
                v34.clone()
            } else {
                let mut v35: i32 = std::fs::symlink_metadata(&*v17).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
                let mut v36: bool = v35 == 2i32;
                if v36 {
                    walked_directory_4(v17.clone(), v21.clone(), v1.clone())
                } else {
                    let mut v38: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                    let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v21, &*v38, &*v13, &*v13].concat());
                    v39.clone()
                }
            }
        };
        walked_names_3(v0.clone(), v1.clone(), v2.clone(), v12, v42.clone())
    }
}
fn walked_directory_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_list_names(std::path::Path::new(&*v0)).unwrap_or_default().join(&char::from(10u8).to_string()).as_str());
    let mut v3: i32 = 0i32;
    walked_names_2(v0.clone(), v1.clone(), v2.clone(), v3)
}
fn todo_sources_0(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let mut v1: i32 = std::fs::symlink_metadata(&*v0).map(|metadata| if metadata.file_type().is_symlink() { 3 } else if metadata.is_dir() { 2 } else if metadata.is_file() { 1 } else { 0 }).unwrap_or(0);
    let mut v2: bool = v1 == 2i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rev-parse\t--is-inside-work-tree"); } LIT.with(|lit| lit.clone()) };
        let (mut v4, mut v5): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v3.split(char::from(9u8))); match eoie_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == eoie_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
        if v4 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ls-files\t-co\t--exclude-standard\t-z"); } LIT.with(|lit| lit.clone()) };
            let (mut v7, mut v8): (bool, Rc<str>) = { let mut command = std::process::Command::new("git"); command.arg("-C").arg(&*v0).args(v6.split(char::from(9u8))); match eoie_process::run_bounded_capture_with_input(&mut command, 60000u64, None) { Ok(capture) => (capture.termination == eoie_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(String::from_utf8_lossy(&capture.stdout).as_ref())), Err(error) => (false, std::rc::Rc::<str>::from(error)) } };
            (v7, v8.clone())
        } else {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = walked_directory_1(v0.clone(), v9.clone());
            (true, v10.clone())
        }
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, v13.clone())
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (bool, Rc<str>) {
        todo_sources_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_todo_sources(v0: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0))
}
