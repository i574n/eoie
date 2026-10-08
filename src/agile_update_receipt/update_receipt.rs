#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn method0() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    v0
}
fn closure0() -> Rc<dyn Fn() -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> u64> = Rc::new(move || -> u64 {
        method0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method1(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile set ok id="); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" progress="); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status="); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v0, &*v7, &*v8, &*v9].concat());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" validation=runtime-attested compiler=not-required"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v2, &*v11, &*v12, &*v12].concat());
    let mut v14: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v5);
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v17, &*v18, &*v12].concat());
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v20, &*v21, &*v12, &*v12].concat());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v22, &*v12, &*v12, &*v12].concat());
    v23.clone()
}
fn closure1() -> Rc<dyn Fn(Rc<str>, i32, Rc<str>, u64, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, Rc<str>, u64, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64| -> Rc<str> {
        method1(v0.clone(), v1, v2.clone(), v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_clock_ms() -> u64 {
    closure0()()
}
pub fn eoie_agile_set_receipt_text(v0: &str, v1: i32, v2: &str, v3: u64, v4: u64, v5: u64) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), v3, v4, v5)
}
