#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ChildSucceeded,
    US0_ChildFailed(i32),
}
fn command_output_receipt_text_0(mut v0: i32, mut v1: u64, mut v2: u64, mut v3: u64) -> Rc<str> {
    let mut v4: bool = v0 == 0i32;
    let mut v7: US0 = if v4 {
        US0::US0_ChildSucceeded
    } else {
        US0::US0_ChildFailed(v0)
    };
    let mut v12: Rc<str> = match &v7 {
        US0::US0_ChildFailed(v9) => {
            let mut v9: i32 = *v9;
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("child-failed"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_ChildSucceeded => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
    };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy command-output "); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v12));
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status="); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    let mut v17: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let mut v21: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stdout_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v21));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let mut v26: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
    let mut v28: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v28));
    let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v27));
    v30.clone()
}
fn closure0() -> Rc<dyn Fn(i32, u64, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, u64, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: u64, mut v2: u64, mut v3: u64| -> Rc<str> {
        command_output_receipt_text_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_command_output_receipt(v0: i32, v1: u64, v2: u64, v3: u64) -> Rc<str> {
    closure0()(v0, v1, v2, v3)
}
