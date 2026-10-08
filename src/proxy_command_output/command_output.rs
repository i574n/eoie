#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_0,
    US0_1(i32),
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_0 => 0,
            US0::US0_1(..) => 1,
        }
    }
}
fn command_output_receipt_text_0(mut v0: i32, mut v1: u64, mut v2: u64, mut v3: u64) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy command-output "); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v0 == 0i32;
    let mut v8: US0 = if v5 {
        US0::US0_0
    } else {
        US0::US0_1(v0)
    };
    let mut v13: Rc<str> = match &v8 {
        US0::US0_1(v10) => {
            let mut v10: i32 = *v10;
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("child-failed"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_0 => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
    };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status="); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v13, &*v14, &*v15, &*v16].concat());
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stdout_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v21, &*v22].concat());
    let mut v24: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v24, &*v23, &*v22, &*v22].concat());
    v25.clone()
}
fn closure0() -> Rc<dyn Fn(i32, u64, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, u64, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: u64, mut v2: u64, mut v3: u64| -> Rc<str> {
        command_output_receipt_text_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_command_output_receipt(v0: i32, v1: u64, v2: u64, v3: u64) -> Rc<str> {
    closure0()(v0, v1, v2, v3)
}
