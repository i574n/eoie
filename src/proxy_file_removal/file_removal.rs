#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_0(Rc<str>),
    US0_1(Rc<str>),
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_0(..) => 0,
            US0::US0_1(..) => 1,
        }
    }
}
#[derive(Clone)]
enum US1 {
    US1_0,
    US1_1,
    US1_2,
    US1_3,
}
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_0 => 0,
            US1::US1_1 => 1,
            US1::US1_2 => 2,
            US1::US1_3 => 3,
        }
    }
}
fn method1() -> US0 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = std::env::args().skip(2).count() as i32;
    let mut v2: bool = v1 == 3;
    if v2 {
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
        let mut v5: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v5 == v6 ;
        if v7 {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove target escapes the root"); } LIT.with(|lit| lit.clone()) };
            US0::US0_1(v8.clone())
        } else {
            let mut v10: i32 = std::fs::symlink_metadata(&*v5).map(|meta| if meta.file_type().is_symlink() { 1 } else if meta.is_dir() { 3 } else { 0 }).unwrap_or(2);
            let mut v11: bool = 0i32 == v10;
            let mut v20: US1 = if v11 {
                US1::US1_0
            } else {
                let mut v13: bool = 1i32 == v10;
                if v13 {
                    US1::US1_1
                } else {
                    let mut v15: bool = 3i32 == v10;
                    if v15 {
                        US1::US1_3
                    } else {
                        US1::US1_2
                    }
                }
            };
            match &v20 {
                US1::US1_3 => {
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove target is a directory (use fs-remove-tree): "); } LIT.with(|lit| lit.clone()) };
                    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v5, &*v6, &*v6, &*v6].concat());
                    US0::US0_1(v25.clone())
                }
                US1::US1_2 => {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove target is missing: "); } LIT.with(|lit| lit.clone()) };
                    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v5, &*v6, &*v6, &*v6].concat());
                    US0::US0_1(v22.clone())
                }
                _ => {
                    let mut v27: u64 = std::fs::symlink_metadata(&*v5).map(|meta| meta.len()).unwrap_or(0);
                    let mut v28: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_unlink_nondirectory(std::path::Path::new(&*v5)).err().unwrap_or_default().as_str());
                    let mut v29: bool = v28 == v6 ;
                    if v29 {
                        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-remove ok path="); } LIT.with(|lit| lit.clone()) };
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" kind="); } LIT.with(|lit| lit.clone()) };
                        let mut v35: Rc<str> = match &v20 {
                            US1::US1_0 => {
                                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file"); } LIT.with(|lit| lit.clone()) };
                                v32.clone()
                            }
                            US1::US1_1 => {
                                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("symlink"); } LIT.with(|lit| lit.clone()) };
                                v33.clone()
                            }
                            _ => unreachable!(),
                        };
                        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes="); } LIT.with(|lit| lit.clone()) };
                        let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v5, &*v31, &*v35, &*v36].concat());
                        let mut v38: Rc<str> = std::rc::Rc::<str>::from((v27).to_string());
                        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                        let mut v40: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                        let mut v41: Rc<str> = std::rc::Rc::<str>::from((v40).to_string());
                        let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v38, &*v39, &*v41, &*v6].concat());
                        US0::US0_0(v42.clone())
                    } else {
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove failed: "); } LIT.with(|lit| lit.clone()) };
                        let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v28, &*v6, &*v6, &*v6].concat());
                        US0::US0_1(v45.clone())
                    }
                }
            }
        }
    } else {
        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-remove expects <root> <relative>"); } LIT.with(|lit| lit.clone()) };
        US0::US0_1(v51.clone())
    }
}
fn method0() -> (i32, Rc<str>) {
    let mut v0: US0 = method1();
    match &v0 {
        US0::US0_0(v1) => {
            let mut v1: Rc<str> = v1.clone();
            (0i32, v1.clone())
        }
        US0::US0_1(v2) => {
            let mut v2: Rc<str> = v2.clone();
            (2i32, v2.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        method0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_file_removal_outcome() -> (i32, Rc<str>) {
    closure0()()
}
