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
fn method0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let (mut v3, mut v4): (bool, Rc<str>) = predicate_rewrite_domain::eoie_predicate_rewrite(&*v0,&*v1,&*v2);
    if v3 {
        US0::US0_1(v4.clone())
    } else {
        US0::US0_0(v4.clone())
    }
}
fn spiral_main() -> i32 {
    let mut v0: i32 = std::env::args().count() as i32;
    let mut v1: bool = v0 == 5i32 ;
    if v1 {
        let mut v2: Rc<str> = std::env::args().nth(1i32 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
        let mut v3: Rc<str> = std::env::args().nth(2i32 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
        let mut v4: Rc<str> = std::env::args().nth(3i32 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
        let mut v5: Rc<str> = std::env::args().nth(4i32 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v4.ends_with(&*v6);
        let mut v9: bool = if v7 {
            let mut v8: bool = v5.ends_with(&*v6);
            v8
        } else {
            false
        };
        if v9 {
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(String::from_utf8(eoie_rust_std_fs::read_regular_limited(std::path::Path::new(&*v4),1024*1024).expect("read input")).expect("input must be UTF-8"));
            let mut v11: US0 = method0(v10.clone(), v3.clone(), v2.clone());
            match &v11 {
                US0::US0_1(v16) => { // Candidate
                    let mut v16: Rc<str> = v16.clone();
                    let mut v17: i32 = std::fs::OpenOptions::new().write(true).create_new(true).open(&*v5).and_then(|mut file| std::io::Write::write_all(&mut file,v16.as_bytes())).map_or(2,|()|0);
                    let mut v18: bool = v17 == 0i32 ;
                    if v18 {
                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Spiral candidate written; input unchanged. Regenerate and test before publication.\n"); } LIT.with(|lit| lit.clone()) };
                        std::io::Write::write_all(&mut std::io::stdout(),v19.as_bytes()).expect("write output");
                        ()
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cannot create candidate; output must not exist.\n"); } LIT.with(|lit| lit.clone()) };
                        std::io::Write::write_all(&mut std::io::stdout(),v20.as_bytes()).expect("write output");
                        ()
                    };
                    v17
                }
                US0::US0_0(v12) => { // Rejected
                    let mut v12: Rc<str> = v12.clone();
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v14, &*v14, &*v14].concat());
                    std::io::Write::write_all(&mut std::io::stdout(),v15.as_bytes()).expect("write output");
                    2i32
                }
            }
        } else {
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Input and output must be .spi files.\n"); } LIT.with(|lit| lit.clone()) };
            std::io::Write::write_all(&mut std::io::stdout(),v23.as_bytes()).expect("write output");
            2i32
        }
    } else {
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-lift-predicate <sha256|receipt-text> <function> <input.spi> <new-output.spi>\nAdd typed_predicate to the owning package before regenerating.\n"); } LIT.with(|lit| lit.clone()) };
        std::io::Write::write_all(&mut std::io::stdout(),v25.as_bytes()).expect("write output");
        let mut v26: i32 = std::env::args().count() as i32;
        let mut v27: bool = v26 == 2i32 ;
        let mut v31: bool = if v27 {
            let mut v28: Rc<str> = std::env::args().nth(1i32 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--help"); } LIT.with(|lit| lit.clone()) };
            let mut v30: bool = v28 == v29 ;
            v30
        } else {
            false
        };
        if v31 {
            0i32
        } else {
            2i32
        }
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 101 });
}
