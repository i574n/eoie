#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn spiral_trace_hold<T: Clone>(fresh: &std::rc::Rc<dyn Fn() -> T>) -> T {
    use std::sync::OnceLock;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let raw = *SLOT.get_or_init(|| Box::into_raw(Box::new((*fresh)())) as usize);
    unsafe { (*(raw as *const T)).clone() }
}
#[cfg(target_arch = "wasm32")]
fn spiral_trace_near_log(text: &str) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static LOGGED: AtomicUsize = AtomicUsize::new(0);
    let cs: Vec<char> = text.chars().collect();
    for c in cs.chunks(15000) {
        let s: String = c.iter().collect();
        let used = LOGGED.load(Ordering::Relaxed);
        let budget = 16000usize.saturating_sub(used);
        if budget < 13 {
            break;
        }
        let s = if s.len() <= budget {
            s
        } else {
            let mut end = budget - 12;
            while !s.is_char_boundary(end) {
                end -= 1;
            }
            format!("{} [truncated]", &s[..end])
        };
        LOGGED.store(used + s.len(), Ordering::Relaxed);
        near_sdk::env::log_str(&s);
    }
}
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_Ok,
    US0_Error(Rc<str>),
}
struct Mut0 { l0: i64 }
struct Mut1 { l0: Rc<dyn Fn(Rc<str>) -> ()> }
struct Mut2 { l0: bool }
struct Mut3 { l0: Rc<str> }
#[derive(Clone)]
enum US1 {
    US1_Verbose,
    US1_Debug,
    US1_Info,
    US1_Warning,
    US1_Critical,
}
struct Mut4 { l0: US1 }
#[derive(Clone)]
enum US2 {
    US2_Some(US1),
    US2_None,
}
struct Mut5 { l0: i32, l1: US2 }
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>),
    US3_None,
}
fn joined_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_4() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_4();
    joined_2(v1.clone(), v0.clone())
}
fn job_file_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_3(v0.clone());
    joined_2(v2.clone(), v1.clone())
}
fn file_exists_5(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn text_or_empty_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn environment_restored_7(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    { let hidden = |key: &str| key.starts_with(char::from(61u8)); let wanted = (v0).split(char::from(0u8)).filter_map(|pair| pair.char_indices().skip(1).find(|(_, c)| *c == char::from(61u8)).map(|(at, _)| (pair[..at].to_owned(), pair[at + 1..].to_owned()))).filter(|(key, _)| !hidden(key)).collect::<std::collections::HashMap<String, String>>(); for (key, _) in std::env::vars_os() { let key = key.to_string_lossy().into_owned(); if !hidden(&key) && !key.starts_with(&*v1) && !wanted.contains_key(&key) { unsafe { std::env::remove_var(&key); } } } for (key, value) in &wanted { if std::env::var_os(key).map_or(true, |now| now.to_string_lossy() != value.as_str()) { unsafe { std::env::set_var(key, value); } } } };
    ()
}
fn file_exists_8(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn closure1() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let mut v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure1()
}
fn ok_c2() -> Rc<dyn Fn() -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US0> = Rc::new(move || -> US0 {
        US0::US0_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10() -> Rc<dyn Fn() -> US0> {
    ok_c2()
}
fn error_c3() -> Rc<dyn Fn(Rc<str>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US0> = Rc::new(move |mut v0: Rc<str>| -> US0 {
        US0::US0_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11() -> Rc<dyn Fn(Rc<str>) -> US0> {
    error_c3()
}
fn get_environment_variable_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1.clone()
}
fn method14(mut v0: Rc<RefCell<Mut5>>) -> bool {
    let mut v1: i32 = v0.borrow().l0.clone();
    let mut v2: bool = v1 > 0i32;
    v2
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> ()> = Rc::new(move |mut v0: Rc<str>| -> () {
        ()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn new_trace_state_12(mut v0: US1) -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TRACE_LEVEL"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = get_environment_variable_13(v1.clone());
    ;
    ;
    ;
    ;
    ;
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Critical"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v3.to_lowercase());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Warning"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v5.to_lowercase());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Info"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Debug"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from(v9.to_lowercase());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Verbose"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(v11.to_lowercase());
    let mut v13: Rc<RefCell<Vec<(Rc<str>, US1)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v14: US1 = US1::US1_Verbose;
    v13.borrow_mut().push((v11.clone(), v14.clone()));
    let mut v15: US1 = US1::US1_Debug;
    v13.borrow_mut().push((v9.clone(), v15.clone()));
    let mut v16: US1 = US1::US1_Info;
    v13.borrow_mut().push((v7.clone(), v16.clone()));
    let mut v17: US1 = US1::US1_Warning;
    v13.borrow_mut().push((v5.clone(), v17.clone()));
    let mut v18: US1 = US1::US1_Critical;
    v13.borrow_mut().push((v3.clone(), v18.clone()));
    let mut v19: US1 = US1::US1_Verbose;
    v13.borrow_mut().push((v12.clone(), v19.clone()));
    let mut v20: US1 = US1::US1_Debug;
    v13.borrow_mut().push((v10.clone(), v20.clone()));
    let mut v21: US1 = US1::US1_Info;
    v13.borrow_mut().push((v8.clone(), v21.clone()));
    let mut v22: US1 = US1::US1_Warning;
    v13.borrow_mut().push((v6.clone(), v22.clone()));
    let mut v23: US1 = US1::US1_Critical;
    v13.borrow_mut().push((v4.clone(), v23.clone()));
    let mut v24: Rc<Vec<(Rc<str>, US1)>> = Rc::new(v13.borrow().clone());
    let mut v25: Rc<RefCell<Vec<(Rc<str>, US1)>>> = Rc::new(RefCell::new((v24).as_ref().clone()));
    let mut v26: i32 = (v25.clone().borrow().len() as i32);
    let mut v27: US2 = US2::US2_None;
    let mut v28: Rc<RefCell<Mut5>> = Rc::new(RefCell::new(Mut5 { l0: v26, l1: v27.clone() }));
    while method14(v28.clone()) {
        let mut v30: i32 = v28.borrow().l0.clone();
        let mut v31: i32 = v30.wrapping_sub(1i32);
        let mut v32: US2 = v28.borrow().l1.clone();
        let (mut v33, mut v34): (Rc<str>, US1) = v25.clone().borrow()[v31 as usize].clone();
        let mut v41: US2 = match &v32 {
            US2::US2_None => {
                let mut v36: bool = v33 == v2 ;
                if v36 {
                    US2::US2_Some(v34.clone())
                } else {
                    US2::US2_None
                }
            }
            US2::US2_Some(v35) => {
                let mut v35: US1 = v35.clone();
                v32.clone()
            }
        };
        v28.borrow_mut().l0 = v31;
        v28.borrow_mut().l1 = v41.clone();
        ()
    };
    let mut v42: US2 = v28.borrow().l1.clone();
    let mut v43: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 1i64 }));
    let mut v44: Rc<dyn Fn(Rc<str>) -> ()> = closure5();
    let mut v45: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v44.clone() }));
    let mut v46: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: true }));
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v48: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v47.clone() }));
    let mut v51: US1 = match &v42 {
        US2::US2_None => {
            v0.clone()
        }
        US2::US2_Some(v49) => {
            let mut v49: US1 = v49.clone();
            v49.clone()
        }
    };
    let mut v52: Rc<RefCell<Mut4>> = Rc::new(RefCell::new(Mut4 { l0: v51.clone() }));
    let mut v53: Option<i64> = None;
    (v43.clone(), v45.clone(), v46.clone(), v48.clone(), v52.clone(), v53.clone())
}
fn fresh_c4() -> Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = Rc::new(move || -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) {
        let mut v0: US1 = US1::US1_Verbose;
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = new_trace_state_12(v0.clone());
        (v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fresh_c6() -> Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = Rc::new(move || -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) {
        let mut v0: US1 = US1::US1_Verbose;
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = new_trace_state_12(v0.clone());
        (v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<RefCell<Mut1>>, mut v2: Rc<RefCell<Mut2>>, mut v3: Rc<RefCell<Mut3>>, mut v4: Rc<RefCell<Mut4>>, mut v5: Option<i64>) -> Rc<str> {
    let mut v6: u64 = { #[cfg(target_arch = "wasm32")] let (h, m, s) = { let secs = near_sdk::env::block_timestamp() / 1_000_000_000; ((secs / 3600) % 24, (secs / 60) % 60, secs % 60) }; #[cfg(all(windows, not(target_arch = "wasm32")))] let (h, m, s) = { #[repr(C)] struct St([u16; 8]); unsafe extern "system" { fn GetLocalTime(t: *mut St); } let mut t = St([0; 8]); unsafe { GetLocalTime(&mut t) }; (t.0[4] as u64, t.0[5] as u64, t.0[6] as u64) }; #[cfg(all(unix, not(target_arch = "wasm32")))] let (h, m, s) = { #[repr(C, align(8))] struct Tm([i32; 16]); unsafe extern "C" { fn localtime_r(t: *const std::os::raw::c_long, tm: *mut Tm) -> *mut Tm; } let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) as std::os::raw::c_long; let mut tm = Tm([0; 16]); unsafe { localtime_r(&secs, &mut tm) }; (tm.0[2] as u64, tm.0[1] as u64, tm.0[0] as u64) }; #[cfg(not(any(windows, unix, target_arch = "wasm32")))] let (h, m, s) = { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0); ((secs / 3600) % 24, (secs / 60) % 60, secs % 60) }; h * 3600 + m * 60 + s };
    let mut v7: u64 = v6.wrapping_div(3600u64);
    let mut v8: bool = v7 < 10u64;
    let mut v11: Rc<str> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v10.clone()
    };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{:?}", v7));
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let mut v16: u64 = v6.wrapping_div(60u64);
    let mut v17: u64 = v16.wrapping_rem(60u64);
    let mut v18: bool = v17 < 10u64;
    let mut v21: Rc<str> = if v18 {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v19.clone()
    } else {
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v20.clone()
    };
    let mut v22: Rc<str> = Rc::<str>::from(format!("{:?}", v17));
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v23));
    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v14));
    let mut v26: u64 = v6.wrapping_rem(60u64);
    let mut v27: bool = v26 < 10u64;
    let mut v30: Rc<str> = if v27 {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v28.clone()
    } else {
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v29.clone()
    };
    let mut v31: Rc<str> = Rc::<str>::from(format!("{:?}", v26));
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v32));
    v33.clone()
}
fn method18(mut v0: Rc<RefCell<Mut3>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_17(mut v0: u8) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0 as char));
    method18(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method16() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[91m"); } LIT.with(|lit| lit.clone()) };
    ;
    ;
    ;
    ;
    ;
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Critical"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
    let mut v3: u8 = v2.clone().as_bytes()[0i32 as usize];
    let mut v4: Rc<str> = format_real_17(v3);
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[0m"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    v7.clone()
}
fn method21(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method22(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v7: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'/';
                v6
            };
            if v7 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method21(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method22(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    v8.clone()
}
fn format_real_23(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method18(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method19(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<RefCell<Mut1>>, mut v2: Rc<RefCell<Mut2>>, mut v3: Rc<RefCell<Mut3>>, mut v4: Rc<RefCell<Mut4>>, mut v5: Option<i64>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>) -> Rc<str> {
    let mut v9: i64 = v0.borrow().l0.clone();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v10));
    let mut v12: Rc<str> = format_real_23(v9);
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v7));
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v10));
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.file_delete"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" / "); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error'"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error'"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error' = "); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v8));
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v26));
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v30));
    method20(v31.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> ()> = Rc::new(move |mut v0: Rc<str>| -> () {
        println!("{}", v0);
        ()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn job_environment_adopted_0(mut v0: Rc<str>) -> () {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_1(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_5(v2.clone());
    if v3 {
        let mut v4: Rc<str> = text_or_empty_6(v2.clone());
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE_JOB_"); } LIT.with(|lit| lit.clone()) };
        environment_restored_7(v4.clone(), v5.clone());
        let mut v6: bool = file_exists_8(v2.clone());
        let mut v10: Result<(), std::io::Error> = if v6 {
            let mut v8: Result<(), std::io::Error> = std::fs::remove_file(&*v2);
            v8
        } else {
            let mut v9: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
            v9
        };
        let mut v11: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method9();
        let mut v13: Result<(), Rc<str>> = v10.map_err(|x| v11(x));
        let mut v14: Rc<dyn Fn() -> US0> = method10();
        let mut v15: Rc<dyn Fn(Rc<str>) -> US0> = method11();
        let mut v16: US0 = match v13 { Ok(()) => v14(), Err(e) => v15(e) };
        match &v16 {
            US0::US0_Error(v17) => {
                let mut v17: Rc<str> = v17.clone();
                let mut v19: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c4();
                { let _ = spiral_trace_hold(&v19); };
                let mut v21: Rc<dyn Fn() -> (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c6();
                let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = { spiral_trace_hold(&v21) };
                let mut v28: US1 = v26.borrow().l0.clone();
                let mut v33: i32 = match &v28 {
                    US1::US1_Critical => {
                        50i32
                    }
                    US1::US1_Debug => {
                        20i32
                    }
                    US1::US1_Info => {
                        30i32
                    }
                    US1::US1_Verbose => {
                        10i32
                    }
                    US1::US1_Warning => {
                        40i32
                    }
                };
                let mut v34: bool = v24.borrow().l0.clone();
                let mut v35: bool = v34 == false;
                let mut v37: bool = if v35 {
                    false
                } else {
                    let mut v36: bool = 50i32 >= v33;
                    v36
                };
                let mut v38: bool = v37 == false;
                let mut v83: US3 = if v38 {
                    US3::US3_None
                } else {
                    { let _ = spiral_trace_hold(&v19); };
                    let (mut v42, mut v43, mut v44, mut v45, mut v46, mut v47): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = { spiral_trace_hold(&v21) };
                    let mut v48: Rc<str> = method15(v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46.clone(), v47.clone());
                    let mut v49: Rc<str> = method16();
                    let mut v50: Rc<str> = method19(v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46.clone(), v47.clone(), v48.clone(), v49.clone(), v17.clone());
                    { let _ = spiral_trace_hold(&v19); };
                    let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<RefCell<Mut0>>, Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut4>>, Option<i64>) = { spiral_trace_hold(&v21) };
                    let mut v59: i64 = v53.borrow().l0.clone();
                    let mut v60: i64 = v59.wrapping_add(1i64);
                    v53.borrow_mut().l0 = v60;
                    let mut v61: Rc<dyn Fn(Rc<str>) -> ()> = closure7();
                    let mut v62: bool = cfg!(target_arch = "wasm32");
                    if v62 {
                        let mut v63: Rc<str> = v56.borrow().l0.clone();
                        let mut v64: bool = v63.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v72: Rc<str> = if v64 {
                            v50.clone()
                        } else {
                            let mut v65: bool = v50.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            if v65 {
                                let mut v66: Rc<str> = v56.borrow().l0.clone();
                                v66.clone()
                            } else {
                                let mut v67: Rc<str> = v56.borrow().l0.clone();
                                let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
                                let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v50));
                                v70.clone()
                            }
                        };
                        let mut v74: i32 = ((v72.chars().count() + 14999) / 15000) as i32;
                        let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v76: bool = v50 != v75 ;
                        let mut v78: bool = if v76 {
                            let mut v77: bool = v74 <= 1i32;
                            v77
                        } else {
                            false
                        };
                        if v78 {
                            v56.borrow_mut().l0 = v72.clone();
                            ()
                        } else {
                            v56.borrow_mut().l0 = v75.clone();
                            { #[cfg(target_arch = "wasm32")] spiral_trace_near_log(&v72); };
                            ()
                        }
                    } else {
                        println!("{}", v50);
                        ()
                    };
                    let mut v81: Rc<dyn Fn(Rc<str>) -> ()> = v54.borrow().l0.clone();
                    v81(v50.clone());
                    US3::US3_Some(v53.clone(), v54.clone(), v55.clone(), v56.clone(), v57.clone(), v58.clone())
                };
                ()
            }
            US0::US0_Ok => {
                ()
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> ()> = Rc::new(move |mut v0: Rc<str>| -> () {
        job_environment_adopted_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_environment_adopted(v0: &str) -> () {
    closure0()(Rc::<str>::from(v0))
}
