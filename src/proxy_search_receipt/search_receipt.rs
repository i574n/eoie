#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_0,
    US0_1,
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_0 => 0,
            US0::US0_1 => 1,
        }
    }
}
fn search_clock_ms_0() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    v0
}
fn closure0() -> Rc<dyn Fn() -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> u64> = Rc::new(move || -> u64 {
        search_clock_ms_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn parallel_search_receipt_text_1(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy parallel-search ok workers="); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let mut v8: bool = v1 > v2;
    let mut v11: US0 = if v8 {
        US0::US0_1
    } else {
        US0::US0_0
    };
    let mut v23: Rc<str> = match &v11 {
        US0::US0_0 => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v14, &*v14, &*v14].concat());
            v15.clone()
        }
        US0::US0_1 => {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" truncated=true found_at_least="); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v17, &*v18, &*v19, &*v20].concat());
            v21.clone()
        }
    };
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parity=exact ordering=deterministic excludes=.git,node_modules,build-output,symlinks"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v23, &*v24, &*v25].concat());
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" serial_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v28: u64 = (v4).saturating_sub(v3);
    let mut v29: Rc<str> = std::rc::Rc::<str>::from((v28).to_string());
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parallel_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v31: u64 = (v5).saturating_sub(v4);
    let mut v32: Rc<str> = std::rc::Rc::<str>::from((v31).to_string());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v29, &*v30, &*v32, &*v25].concat());
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v35: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let mut v36: Rc<str> = std::rc::Rc::<str>::from((v35).to_string());
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v33, &*v34, &*v36, &*v25].concat());
    v37.clone()
}
fn closure1() -> Rc<dyn Fn(i32, i32, i32, u64, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, i32, u64, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64, mut v4: u64, mut v5: u64| -> Rc<str> {
        parallel_search_receipt_text_1(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn parallel_search_receipt_text_2(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: u64) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy parallel-search ok workers="); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let mut v6: bool = v1 > v2;
    let mut v9: US0 = if v6 {
        US0::US0_1
    } else {
        US0::US0_0
    };
    let mut v21: Rc<str> = match &v9 {
        US0::US0_0 => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v12, &*v12].concat());
            v13.clone()
        }
        US0::US0_1 => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hits="); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" truncated=true found_at_least="); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v16, &*v17, &*v18].concat());
            v19.clone()
        }
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parity=exact ordering=deterministic excludes=.git,node_modules,build-output,symlinks"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v5, &*v21, &*v22, &*v23].concat());
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" serial_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v26: u64 = (v3).saturating_sub(v3);
    let mut v27: Rc<str> = std::rc::Rc::<str>::from((v26).to_string());
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parallel_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v29: u64 = (v3).saturating_sub(v3);
    let mut v30: Rc<str> = std::rc::Rc::<str>::from((v29).to_string());
    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v27, &*v28, &*v30, &*v23].concat());
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v33: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let mut v34: Rc<str> = std::rc::Rc::<str>::from((v33).to_string());
    let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v31, &*v32, &*v34, &*v23].concat());
    v35.clone()
}
pub fn eoie_search_clock_ms() -> u64 {
    closure0()()
}
pub fn eoie_parallel_search_receipt_text(v0: i32, v1: i32, v2: i32, v3: u64, v4: u64, v5: u64) -> Rc<str> {
    closure1()(v0, v1, v2, v3, v4, v5)
}
