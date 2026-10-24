#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_CopyReceipt(Rc<str>),
    US0_CopyRejected(Rc<str>),
}
fn file_copy_outcome_1(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US0 {
    let mut v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v5: bool = v0 == 4i32;
    if v5 {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v2).err().unwrap_or_default().as_str());
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v3).err().unwrap_or_default().as_str());
        let mut v8: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v8 {
            let mut v9: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v9 {
                let mut v10: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v2).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                let mut v11: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v3).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                let mut v12: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::copy_regular_atomic_preserve(std::path::Path::new(&*v10), std::path::Path::new(&*v11)).err().unwrap_or_default().as_str());
                let mut v13: bool = v12.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v13 {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-copy ok source="); } LIT.with(|lit| lit.clone()) };
                    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v10));
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" target="); } LIT.with(|lit| lit.clone()) };
                    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v11));
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes="); } LIT.with(|lit| lit.clone()) };
                    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
                    let mut v21: u64 = std::fs::symlink_metadata(&*v11).map(|meta| meta.len()).unwrap_or(0);
                    let mut v22: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
                    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v22));
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
                    let mut v26: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
                    let mut v27: Rc<str> = std::rc::Rc::<str>::from((v26).to_string());
                    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v27));
                    US0::US0_CopyReceipt(v28.clone())
                } else {
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-copy failed: "); } LIT.with(|lit| lit.clone()) };
                    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v12));
                    US0::US0_CopyRejected(v31.clone())
                }
            } else {
                US0::US0_CopyRejected(v7.clone())
            }
        } else {
            US0::US0_CopyRejected(v6.clone())
        }
    } else {
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-copy expects root, source, target"); } LIT.with(|lit| lit.clone()) };
        US0::US0_CopyRejected(v38.clone())
    }
}
fn file_copy_report_0(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let mut v4: US0 = file_copy_outcome_1(v0, v1.clone(), v2.clone(), v3.clone());
    match &v4 {
        US0::US0_CopyReceipt(v5) => {
            let mut v5: Rc<str> = v5.clone();
            (0i32, v5.clone())
        }
        US0::US0_CopyRejected(v6) => {
            let mut v6: Rc<str> = v6.clone();
            (2i32, v6.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> (i32, Rc<str>) {
        file_copy_report_0(v0, v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_file_copy_outcome(v0: i32, v1: &str, v2: &str, v3: &str) -> (i32, Rc<str>) {
    closure0()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
