#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ArgumentFailure,
    US0_ChildProcessFailure(i32),
}
#[derive(Clone)]
enum US1 {
    US1_AddCreateOrReplace,
    US1_ExpectObservedMatches(u64),
    US1_NoRepair,
}
fn proxy_child_exit_code_0(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = v0.find("failed: exit code: ").and_then(|at| v0[at + 19..].split(|c: char| c.is_ascii_digit() == false).next().and_then(|digits| digits.parse::<i32>().ok())).unwrap_or(0);
    let mut v2: bool = v1 > 0;
    let mut v5: US0 = if v2 {
        US0::US0_ChildProcessFailure(v1)
    } else {
        US0::US0_ArgumentFailure
    };
    match &v5 {
        US0::US0_ArgumentFailure => {
            0i32
        }
        US0::US0_ChildProcessFailure(v6) => {
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
        proxy_child_exit_code_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn proxy_argument_count_2() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn proxy_corrected_command_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: u64 = v0.find("observed matches=").and_then(|at| v0[at + 17..].split(|c: char| c.is_ascii_digit() == false).next().and_then(|digits| digits.parse::<u64>().ok())).unwrap_or(0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-write refuses to overwrite existing file"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v0.starts_with(&*v2);
    let mut v12: US1 = if v3 {
        US1::US1_AddCreateOrReplace
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace expected "); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v0.starts_with(&*v5);
        let mut v8: bool = if v6 {
            let mut v7: bool = v1 > 0;
            v7
        } else {
            false
        };
        if v8 {
            US1::US1_ExpectObservedMatches(v1)
        } else {
            US1::US1_NoRepair
        }
    };
    match &v12 {
        US1::US1_AddCreateOrReplace => {
            let mut v13: bool = std::env::args().nth(2 + 3i32 as usize).is_some_and(|word| word.starts_with("--"));
            let mut v16: Rc<str> = if v13 {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --create-or-replace "); } LIT.with(|lit| lit.clone()) };
                v14.clone()
            } else {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --create-or-replace -- "); } LIT.with(|lit| lit.clone()) };
                v15.clone()
            };
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy "); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(2 + 0i32 as usize).take((3i32 - 0i32).max(0) as usize).map(|word| Some(word.clone()).filter(|plain| plain.is_empty() == false && plain.chars().enumerate().all(|(at, c)| c.is_ascii_alphanumeric() || "_-./:=+,".contains(c) || c == char::from(92u8) || (c == char::from(126u8) && at > 0))).unwrap_or_else(|| [String::from(char::from(39u8)), word.replace(char::from(39u8), &String::from_iter([char::from(39u8); 2])), String::from(char::from(39u8))].concat())).collect::<Vec<_>>().join(" ").as_str());
            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v16].concat().as_str());
            let mut v20: i32 = proxy_argument_count_2();
            let mut v21: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(2 + 3i32 as usize).take((v20 - 3i32).max(0) as usize).map(|word| Some(word.clone()).filter(|plain| plain.is_empty() == false && plain.chars().enumerate().all(|(at, c)| c.is_ascii_alphanumeric() || "_-./:=+,".contains(c) || c == char::from(92u8) || (c == char::from(126u8) && at > 0))).unwrap_or_else(|| [String::from(char::from(39u8)), word.replace(char::from(39u8), &String::from_iter([char::from(39u8); 2])), String::from(char::from(39u8))].concat())).collect::<Vec<_>>().join(" ").as_str());
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v21, &*v22].concat().as_str());
            v23.clone()
        }
        US1::US1_ExpectObservedMatches(v24) => {
            let mut v24: u64 = *v24;
            let mut v25: bool = v24 == 1;
            if v25 {
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy "); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(2 + 0i32 as usize).take((6i32 - 0i32).max(0) as usize).map(|word| Some(word.clone()).filter(|plain| plain.is_empty() == false && plain.chars().enumerate().all(|(at, c)| c.is_ascii_alphanumeric() || "_-./:=+,".contains(c) || c == char::from(92u8) || (c == char::from(126u8) && at > 0))).unwrap_or_else(|| [String::from(char::from(39u8)), word.replace(char::from(39u8), &String::from_iter([char::from(39u8); 2])), String::from(char::from(39u8))].concat())).collect::<Vec<_>>().join(" ").as_str());
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v27, &*v28].concat().as_str());
                v29.clone()
            } else {
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy "); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(2 + 0i32 as usize).take((6i32 - 0i32).max(0) as usize).map(|word| Some(word.clone()).filter(|plain| plain.is_empty() == false && plain.chars().enumerate().all(|(at, c)| c.is_ascii_alphanumeric() || "_-./:=+,".contains(c) || c == char::from(92u8) || (c == char::from(126u8) && at > 0))).unwrap_or_else(|| [String::from(char::from(39u8)), word.replace(char::from(39u8), &String::from_iter([char::from(39u8); 2])), String::from(char::from(39u8))].concat())).collect::<Vec<_>>().join(" ").as_str());
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --expect "); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v32].concat().as_str());
                let mut v34: Rc<str> = std::rc::Rc::<str>::from(v24.to_string().as_str());
                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35].concat().as_str());
                v36.clone()
            }
        }
        US1::US1_NoRepair => {
            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v38.clone()
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        proxy_corrected_command_1(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_proxy_child_exit_code(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_proxy_corrected_command(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
