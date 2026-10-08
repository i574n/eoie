#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn method0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v4, &*v4, &*v4].concat());
    let mut v6: bool = std::path::Path::new(&*v5).exists();
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi already exists"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v1, &*v9, &*v7, &*v4].concat());
        let mut v11: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v10 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v11
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v12, &*v4, &*v4, &*v4].concat());
        let mut v14: bool = std::path::Path::new(&*v13).exists();
        if v14 {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj already exists"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v1, &*v17, &*v15, &*v4].concat());
            let mut v19: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v18 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v19
        } else {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v20, &*v4, &*v4, &*v4].concat());
            let mut v22: bool = std::fs::create_dir_all(&*v21).is_ok();
            if v22 {
                let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v12, &*v4, &*v4, &*v4].concat());
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    agile\n    prompt\n"); } LIT.with(|lit| lit.clone()) };
                let mut v25: bool = std::fs::write(&*v23, &*v24).is_ok();
                if v25 {
                    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v4, &*v4, &*v4].concat());
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v30, &*v31].concat());
                    let mut v33: bool = std::fs::write(&*v26, &*v32).is_ok();
                    if v33 {
                        let mut v34: u64 = v24.len() as u64;
                        let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v30, &*v31].concat());
                        let mut v36: u64 = v35.len() as u64;
                        let mut v37: u64 = v34 + v36;
                        let mut v38: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
                        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile init ok root="); } LIT.with(|lit| lit.clone()) };
                        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files=2 bytes="); } LIT.with(|lit| lit.clone()) };
                        let mut v41: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
                        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                        let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v1, &*v40, &*v41, &*v42].concat());
                        let mut v44: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
                        let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" next=eoie agile begin <root> <title>"); } LIT.with(|lit| lit.clone()) };
                        let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v44, &*v45, &*v4, &*v4].concat());
                        let mut v47: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v46.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v47
                    } else {
                        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                        let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v49, &*v1, &*v50, &*v48, &*v4].concat());
                        let mut v52: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v51 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v52
                    }
                } else {
                    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
                    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v57: Rc<str> = std::rc::Rc::<str>::from([&*v55, &*v1, &*v56, &*v54, &*v4].concat());
                    let mut v58: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v57 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v58
                }
            } else {
                let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not create the state directory"); } LIT.with(|lit| lit.clone()) };
                let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v63: Rc<str> = std::rc::Rc::<str>::from([&*v61, &*v1, &*v62, &*v60, &*v4].concat());
                let mut v64: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v63 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v64
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        method0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_init_run() -> i32 {
    closure0()()
}
