#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn spiral_main() -> i32 {
    let mut v0: Rc<str> = std::env::temp_dir().display().to_string().into();
    let mut v1: bool = std::path::Path::new(&*v0).is_dir();
    if v1 {
        let mut v2: bool = eoie_proxy_search::inspection_tree_sha256(std::path::Path::new(&*v0), "../escape").is_err_and(|error| error.contains("unsafe relative path"));
        if v2 {
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
