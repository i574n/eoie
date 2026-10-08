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
fn method0(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = v0.find("failed: exit code: ").and_then(|at| v0[at + 19..].split(|c: char| c.is_ascii_digit() == false).next().and_then(|digits| digits.parse::<i32>().ok())).unwrap_or(0);
    let mut v2: bool = v1 > 0;
    let mut v5: US0 = if v2 {
        US0::US0_1(v1)
    } else {
        US0::US0_0
    };
    match &v5 {
        US0::US0_0 => {
            0i32
        }
        US0::US0_1(v6) => {
            let mut v6: i32 = *v6;
            let mut v7: bool = v6 > 255;
            if v7 {
                255i32
            } else {
                v6
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        method0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_proxy_child_exit_code(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
