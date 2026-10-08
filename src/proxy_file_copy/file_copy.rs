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
fn method1(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US0 {
    let mut v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v5: bool = v0 == 4i32;
    if v5 {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v2).err().unwrap_or_default().as_str());
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v3).err().unwrap_or_default().as_str());
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v9: bool = v6 == v8 ;
        if v9 {
            let mut v10: bool = v7 == v8 ;
            if v10 {
                let mut v11: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v2).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                let mut v12: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v3).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(eoie_rust_std_fs::copy_regular_atomic_preserve(std::path::Path::new(&*v11), std::path::Path::new(&*v12)).err().unwrap_or_default().as_str());
                let mut v14: bool = v13 == v8 ;
                if v14 {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy fs-copy ok source="); } LIT.with(|lit| lit.clone()) };
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" target="); } LIT.with(|lit| lit.clone()) };
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes="); } LIT.with(|lit| lit.clone()) };
                    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v11, &*v16, &*v12, &*v17].concat());
                    let mut v19: u64 = std::fs::symlink_metadata(&*v12).map(|meta| meta.len()).unwrap_or(0);
                    let mut v20: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v22: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
                    let mut v23: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
                    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v20, &*v21, &*v23, &*v8].concat());
                    US0::US0_0(v24.clone())
                } else {
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-copy failed: "); } LIT.with(|lit| lit.clone()) };
                    let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v13, &*v8, &*v8, &*v8].concat());
                    US0::US0_1(v27.clone())
                }
            } else {
                US0::US0_1(v7.clone())
            }
        } else {
            US0::US0_1(v6.clone())
        }
    } else {
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-copy expects root, source, target"); } LIT.with(|lit| lit.clone()) };
        US0::US0_1(v34.clone())
    }
}
fn method0(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let mut v4: US0 = method1(v0, v1.clone(), v2.clone(), v3.clone());
    match &v4 {
        US0::US0_0(v5) => {
            let mut v5: Rc<str> = v5.clone();
            (0i32, v5.clone())
        }
        US0::US0_1(v6) => {
            let mut v6: Rc<str> = v6.clone();
            (2i32, v6.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> (i32, Rc<str>) {
        method0(v0, v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_file_copy_outcome(v0: i32, v1: &str, v2: &str, v3: &str) -> (i32, Rc<str>) {
    closure0()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
