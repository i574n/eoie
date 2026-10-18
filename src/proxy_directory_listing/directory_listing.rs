#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ListingShown(Rc<str>),
    US0_ListingRejected(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_WholeRoot,
    US1_RootedDirectory(Rc<str>),
}
fn directory_listing_outcome_1() -> US0 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: i32 = std::env::args().skip(2).count() as i32;
    let mut v2: bool = v1 == 2;
    if v2 {
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v4 == v4 ;
        let mut v8: US1 = if v5 {
            US1::US1_WholeRoot
        } else {
            US1::US1_RootedDirectory(v4.clone())
        };
        let mut v12: Rc<str> = match &v8 {
            US1::US1_RootedDirectory(v9) => {
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v9).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                v10.clone()
            }
            US1::US1_WholeRoot => {
                v3.clone()
            }
        };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v14: bool = v12 == v13 ;
        if v14 {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list path rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).err().unwrap_or_default().as_str());
            let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v13, &*v13, &*v13].concat());
            US0::US0_ListingRejected(v17.clone())
        } else {
            let mut v19: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_list_names(std::path::Path::new(&*v12)).err().unwrap_or_default().as_str());
            let mut v20: bool = v19 == v13 ;
            if v20 {
                let mut v21: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_list_names(std::path::Path::new(&*v12)).unwrap_or_default().join(&char::from(10u8).to_string()).as_str());
                let mut v22: bool = v21 == v13 ;
                let mut v24: i32 = if v22 {
                    0i32
                } else {
                    let mut v23: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v21.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v23
                };
                let mut v25: bool = v21 == v13 ;
                let mut v28: u64 = if v25 {
                    0u64
                } else {
                    let mut v26: u64 = v21.matches(char::from(10u8)).count() as u64;
                    let mut v27: u64 = v26 + 1;
                    v27
                };
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-list ok path="); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" entries="); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = std::rc::Rc::<str>::from((v28).to_string());
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v12, &*v30, &*v31, &*v32].concat());
                let mut v34: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                let mut v35: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
                let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v35, &*v13, &*v13, &*v13].concat());
                US0::US0_ListingShown(v36.clone())
            } else {
                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list failed: "); } LIT.with(|lit| lit.clone()) };
                let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v19, &*v13, &*v13, &*v13].concat());
                US0::US0_ListingRejected(v39.clone())
            }
        }
    } else {
        let mut v43: bool = v1 == 3;
        if v43 {
            let mut v44: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
            let mut v45: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
            let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
            let mut v47: bool = v45 == v46 ;
            let mut v50: US1 = if v47 {
                US1::US1_WholeRoot
            } else {
                US1::US1_RootedDirectory(v45.clone())
            };
            let mut v54: Rc<str> = match &v50 {
                US1::US1_RootedDirectory(v51) => {
                    let mut v51: Rc<str> = v51.clone();
                    let mut v52: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v44), &*v51).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    v52.clone()
                }
                US1::US1_WholeRoot => {
                    v44.clone()
                }
            };
            let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v56: bool = v54 == v55 ;
            if v56 {
                let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list path rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v58: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v44), &*v45).err().unwrap_or_default().as_str());
                let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v57, &*v58, &*v55, &*v55, &*v55].concat());
                US0::US0_ListingRejected(v59.clone())
            } else {
                let mut v61: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_list_names(std::path::Path::new(&*v54)).err().unwrap_or_default().as_str());
                let mut v62: bool = v61 == v55 ;
                if v62 {
                    let mut v63: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_list_names(std::path::Path::new(&*v54)).unwrap_or_default().join(&char::from(10u8).to_string()).as_str());
                    let mut v64: bool = v63 == v55 ;
                    let mut v66: i32 = if v64 {
                        0i32
                    } else {
                        let mut v65: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v63.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v65
                    };
                    let mut v67: bool = v63 == v55 ;
                    let mut v70: u64 = if v67 {
                        0u64
                    } else {
                        let mut v68: u64 = v63.matches(char::from(10u8)).count() as u64;
                        let mut v69: u64 = v68 + 1;
                        v69
                    };
                    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-list ok path="); } LIT.with(|lit| lit.clone()) };
                    let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" entries="); } LIT.with(|lit| lit.clone()) };
                    let mut v73: Rc<str> = std::rc::Rc::<str>::from((v70).to_string());
                    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v75: Rc<str> = std::rc::Rc::<str>::from([&*v71, &*v54, &*v72, &*v73, &*v74].concat());
                    let mut v76: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let mut v77: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                    let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v75, &*v77, &*v55, &*v55, &*v55].concat());
                    US0::US0_ListingShown(v78.clone())
                } else {
                    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list failed: "); } LIT.with(|lit| lit.clone()) };
                    let mut v81: Rc<str> = std::rc::Rc::<str>::from([&*v80, &*v61, &*v55, &*v55, &*v55].concat());
                    US0::US0_ListingRejected(v81.clone())
                }
            }
        } else {
            let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list expects <root> [relative]"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ListingRejected(v85.clone())
        }
    }
}
fn directory_listing_report_0() -> (i32, Rc<str>) {
    let mut v0: US0 = directory_listing_outcome_1();
    match &v0 {
        US0::US0_ListingRejected(v2) => {
            let mut v2: Rc<str> = v2.clone();
            (2i32, v2.clone())
        }
        US0::US0_ListingShown(v1) => {
            let mut v1: Rc<str> = v1.clone();
            (0i32, v1.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        directory_listing_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_directory_listing_outcome() -> (i32, Rc<str>) {
    closure0()()
}
