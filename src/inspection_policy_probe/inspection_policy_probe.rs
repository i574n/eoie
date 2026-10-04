#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn spiral_main() -> i32 {
    let mut v0: Rc<str> = std::env::temp_dir().join("eoie inspection ü ".to_owned() + &std::process::id().to_string() + "-" + &std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos().to_string()).display().to_string().into();
    let mut v1: bool = std::fs::create_dir(&*v0).is_ok();
    if v1 {
        let mut v2: Rc<str> = std::path::Path::new(&*v0).join("input.txt").display().to_string().into();
        let mut v3: bool = std::fs::write(&*v2, b"abc").is_ok();
        let mut v4: bool = eoie_proxy_search::inspection_file_sha256(std::path::Path::new(&*v2)).as_deref() == Ok("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        let mut v5: bool = std::fs::remove_file(&*v2).is_ok();
        let mut v6: bool = std::fs::remove_dir(&*v0).is_ok();
        let mut v7: bool = v3 && v4;
        let mut v8: bool = v7 && v5;
        let mut v9: bool = v8 && v6;
        if v9 {
            0i32
        } else {
            1i32
        }
    } else {
        10i32
    }
}
fn main() {
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(main.join().unwrap());
}
