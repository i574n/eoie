#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_RemovalReceipt(Rc<str>),
    US0_RemovalRejected(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_RemovedFile,
    US1_RemovedSymlink,
    US1_MissingEntry,
    US1_DirectoryEntry,
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn proxy_argument_count_2() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn file_removal_outcome_1() -> US0 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = proxy_argument_count_2();
    let mut v2: bool = v1 == 3;
    if v2 {
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
        let mut v5: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove target escapes the root"); } LIT.with(|lit| lit.clone()) };
            US0::US0_RemovalRejected(v7.clone())
        } else {
            let mut v9: i32 = std::fs::symlink_metadata(&*v5).map(|meta| if meta.file_type().is_symlink() { 1 } else if meta.is_dir() { 3 } else { 0 }).unwrap_or(2);
            let mut v10: bool = 0i32 == v9;
            let mut v19: US1 = if v10 {
                US1::US1_RemovedFile
            } else {
                let mut v12: bool = 1i32 == v9;
                if v12 {
                    US1::US1_RemovedSymlink
                } else {
                    let mut v14: bool = 3i32 == v9;
                    if v14 {
                        US1::US1_DirectoryEntry
                    } else {
                        US1::US1_MissingEntry
                    }
                }
            };
            match &v19 {
                US1::US1_DirectoryEntry => {
                    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove target is a directory (use fs-remove-tree): "); } LIT.with(|lit| lit.clone()) };
                    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v5));
                    US0::US0_RemovalRejected(v24.clone())
                }
                US1::US1_MissingEntry => {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove target is missing: "); } LIT.with(|lit| lit.clone()) };
                    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v5));
                    US0::US0_RemovalRejected(v21.clone())
                }
                _ => {
                    let mut v26: u64 = std::fs::symlink_metadata(&*v5).map(|meta| meta.len()).unwrap_or(0);
                    let mut v27: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_unlink_nondirectory(std::path::Path::new(&*v5)).err().unwrap_or_default().as_str());
                    let mut v28: bool = v27.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v28 {
                        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-remove ok path="); } LIT.with(|lit| lit.clone()) };
                        let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v5));
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" kind="); } LIT.with(|lit| lit.clone()) };
                        let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
                        let mut v36: Rc<str> = match &v19 {
                            US1::US1_RemovedFile => {
                                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file"); } LIT.with(|lit| lit.clone()) };
                                v33.clone()
                            }
                            US1::US1_RemovedSymlink => {
                                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("symlink"); } LIT.with(|lit| lit.clone()) };
                                v34.clone()
                            }
                            _ => unreachable!(),
                        };
                        let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v36));
                        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes="); } LIT.with(|lit| lit.clone()) };
                        let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
                        let mut v40: Rc<str> = std::rc::Rc::<str>::from((v26).to_string());
                        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                        let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
                        let mut v44: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                        let mut v45: Rc<str> = std::rc::Rc::<str>::from((v44).to_string());
                        let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v45));
                        US0::US0_RemovalReceipt(v46.clone())
                    } else {
                        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove failed: "); } LIT.with(|lit| lit.clone()) };
                        let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v27));
                        US0::US0_RemovalRejected(v49.clone())
                    }
                }
            }
        }
    } else {
        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove expects <root> <relative>"); } LIT.with(|lit| lit.clone()) };
        US0::US0_RemovalRejected(v55.clone())
    }
}
fn file_removal_report_0() -> (i32, Rc<str>) {
    let mut v0: US0 = file_removal_outcome_1();
    match &v0 {
        US0::US0_RemovalReceipt(v1) => {
            let mut v1: Rc<str> = v1.clone();
            (0i32, v1.clone())
        }
        US0::US0_RemovalRejected(v2) => {
            let mut v2: Rc<str> = v2.clone();
            (2i32, v2.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        file_removal_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_file_removal_outcome() -> (i32, Rc<str>) {
    closure0()()
}
