#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Succeeded,
    US0_Failed,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(u64),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_LiteralNeedle,
    US3_CaselessNeedle,
    US3_PatternNeedle(bool),
}
#[derive(Clone)]
enum US4 {
    US4_FolderRoot,
    US4_FileRoot,
    US4_MissingRoot,
}
fn counts_start_2() -> (i32, i32, Rc<str>) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0i32, 0i32, v0.clone())
}
fn json_matches_4(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 < v2;
        if v5 {
            let mut v6: i32 = (v0).1[v1 as usize].1[v3 as usize] as i32;
            let mut v7: Rc<str> = std::rc::Rc::<str>::from((v0).1[v1 as usize].0.as_str());
            let mut v8: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let mut out = String::from(q); for c in (v7).chars() { match c { c if c == q || c == b => { out.push(b); out.push(c); } c if (c as u32) < 32 => out.push_str(&format!("{b}u{:04x}", c as u32)), c => out.push(c) } } out.push(q); out }.as_str());
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":"); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v8));
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"line\":"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
            let mut v13: i32 = v6.wrapping_add(1i32);
            let mut v14: Rc<str> = std::rc::Rc::<str>::from((v13).to_string().as_str());
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v14));
            let mut v16: Rc<str> = std::rc::Rc::<str>::from({ let line = &(v0).1[v1 as usize].2[v6 as usize]; if line.chars().count() > 200 { format!("{}...", line.chars().take(200).collect::<String>()) } else { line.to_owned() } }.as_str());
            let mut v17: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let mut out = String::from(q); for c in (v16).chars() { match c { c if c == q || c == b => { out.push(b); out.push(c); } c if (c as u32) < 32 => out.push_str(&format!("{b}u{:04x}", c as u32)), c => out.push(c) } } out.push(q); out }.as_str());
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"text\":"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v17));
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
            let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v21));
            let mut v23: i32 = v3.wrapping_add(1i32);
            let mut v24: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = if v24 {
                v22.clone()
            } else {
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v25));
                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v22));
                v27.clone()
            };
            (v0, v1, v2, v3, v4) = (v0.clone(), v1, v2, v23, v28.clone());
            continue;
        } else {
            return v4.clone();
        }
    }
}
fn last_window_line_5(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32) -> i32 {
    loop {
        let mut v6: bool = v4 < v2;
        if v6 {
            let mut v7: i32 = v4.wrapping_add(1i32);
            let mut v8: i32 = (v0).1[v1 as usize].1[v4 as usize] as i32;
            let mut v9: i32 = v8.wrapping_add(v3);
            let mut v10: i32 = (v0).1[v1 as usize].2.len() as i32;
            let mut v11: i32 = v10.wrapping_sub(1i32);
            let mut v12: bool = v9 < v11;
            let mut v13: i32 = if v12 {
                v9
            } else {
                v11
            };
            let mut v14: bool = v5 < v13;
            let mut v15: i32 = if v14 {
                v13
            } else {
                v5
            };
            (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1, v2, v3, v7, v15);
            continue;
        } else {
            return v5;
        }
    }
}
fn window_lines_7(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32) -> () {
    loop {
        let mut v6: bool = v4 <= v5;
        if v6 {
            let mut v7: i32 = v4.wrapping_add(1i32);
            let mut v8: bool = (v0).1[v1 as usize].1.iter().take(v2 as usize).any(|index| *index == v4 as usize);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from({ let line = &(v0).1[v1 as usize].2[v4 as usize]; if line.chars().count() > 200 { format!("{}...", line.chars().take(200).collect::<String>()) } else { line.to_owned() } }.as_str());
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(format!("{:>width$}{} {}", v7, if v8 { char::from(62u8) } else { char::from(32u8) }, &*v9, width = v3 as usize).as_str());
            println!("{}", &*v10);
            (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1, v2, v3, v7, v5);
            continue;
        }
        return ();
    }
}
fn windows_printed_6(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32) -> () {
    loop {
        let mut v7: bool = v5 < v2;
        if v7 {
            let mut v8: i32 = (v0).1[v1 as usize].1[v5 as usize] as i32;
            let mut v9: i32 = v8.wrapping_sub(v3);
            let mut v10: bool = v9 < 0i32;
            let mut v11: i32 = if v10 {
                0i32
            } else {
                v9
            };
            let mut v12: bool = 0i32 <= v6;
            let mut v15: bool = if v12 {
                let mut v13: i32 = v6.wrapping_add(1i32);
                let mut v14: bool = v11 < v13;
                v14
            } else {
                false
            };
            let mut v17: i32 = if v15 {
                let mut v16: i32 = v6.wrapping_add(1i32);
                v16
            } else {
                v11
            };
            let mut v18: i32 = v8.wrapping_add(v3);
            let mut v19: i32 = (v0).1[v1 as usize].2.len() as i32;
            let mut v20: i32 = v19.wrapping_sub(1i32);
            let mut v21: bool = v18 < v20;
            let mut v22: i32 = if v21 {
                v18
            } else {
                v20
            };
            window_lines_7(v0.clone(), v1, v2, v4, v17, v22);
            let mut v23: i32 = v5.wrapping_add(1i32);
            let mut v24: bool = v6 < v22;
            let mut v25: i32 = if v24 {
                v22
            } else {
                v6
            };
            (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1, v2, v3, v4, v23, v25);
            continue;
        }
        return ();
    }
}
fn files_rendered_3(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: bool, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: Rc<str>) -> (i32, i32, Rc<str>) {
    let mut v8: i32 = (v0).1.len() as i32;
    let mut v9: bool = v8 <= v4;
    let (mut v42, mut v43, mut v44): (i32, i32, Rc<str>) = if v9 {
        (v5, v6, v7.clone())
    } else {
        let mut v10: i32 = (v0).1[v4 as usize].1.len() as i32;
        let mut v11: i32 = v6.wrapping_add(v10);
        let mut v12: bool = v3 <= v5;
        let (mut v39, mut v40, mut v41): (i32, i32, Rc<str>) = if v12 {
            let mut v13: i32 = v4.wrapping_add(1i32);
            let (mut v14, mut v15, mut v16): (i32, i32, Rc<str>) = files_rendered_3(v0.clone(), v1, v2, v3, v13, v5, v11, v7.clone());
            (v14, v15, v16.clone())
        } else {
            let mut v17: i32 = v3.wrapping_sub(v5);
            let mut v18: bool = v10 < v17;
            let mut v19: i32 = if v18 {
                v10
            } else {
                v17
            };
            let (mut v32, mut v33, mut v34): (i32, i32, Rc<str>) = if v1 {
                let mut v20: i32 = v5.wrapping_add(v19);
                let mut v21: i32 = 0i32;
                let mut v22: Rc<str> = json_matches_4(v0.clone(), v4, v19, v21, v7.clone());
                (v20, v11, v22.clone())
            } else {
                let mut v23: Rc<str> = std::rc::Rc::<str>::from((v0).1[v4 as usize].0.as_str());
                println!("{}", &*v23);
                let mut v24: i32 = 0i32;
                let mut v25: i32 = 0i32;
                let mut v26: i32 = last_window_line_5(v0.clone(), v4, v19, v2, v24, v25);
                let mut v27: i32 = v26.wrapping_add(1i32);
                let mut v28: i32 = (v27).to_string().len() as i32;
                let mut v29: i32 = 0i32;
                let mut v30: i32 = -1i32;
                windows_printed_6(v0.clone(), v4, v19, v2, v28, v29, v30);
                let mut v31: i32 = v5.wrapping_add(v19);
                (v31, v11, v7.clone())
            };
            let mut v35: i32 = v4.wrapping_add(1i32);
            let (mut v36, mut v37, mut v38): (i32, i32, Rc<str>) = files_rendered_3(v0.clone(), v1, v2, v3, v35, v32, v33, v34.clone());
            (v36, v37, v38.clone())
        };
        (v39, v40, v41.clone())
    };
    (v42, v43, v44.clone())
}
fn decimal_text_8(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn elapsed_text_10(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_12() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_11() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_12();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_9(mut v0: US0, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US0::US0_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = elapsed_text_10(v2);
    let mut v9: Rc<str> = clock_text_11();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn search_receipt_1(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: bool, mut v2: i32, mut v3: i32, mut v4: u64) -> i32 {
    let mut v5: i32 = 0i32;
    let (mut v6, mut v7, mut v8): (i32, i32, Rc<str>) = counts_start_2();
    let (mut v9, mut v10, mut v11): (i32, i32, Rc<str>) = files_rendered_3(v0.clone(), v1, v2, v3, v5, v6, v7, v8.clone());
    let mut v12: u64 = (v0.clone()).0;
    let mut v13: Rc<str> = decimal_text_8(v12);
    let mut v14: i32 = (v0).1.len() as i32;
    let mut v15: Rc<str> = std::rc::Rc::<str>::from((v14).to_string().as_str());
    if v1 {
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"files\":"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v13));
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"matched\":"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v15));
        let mut v21: Rc<str> = std::rc::Rc::<str>::from((v10).to_string().as_str());
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"total\":"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v21));
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"shown\":"); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
        let mut v26: Rc<str> = std::rc::Rc::<str>::from((v9).to_string().as_str());
        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
        let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v27));
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"matches\":["); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
        let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v11));
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]}"); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
        println!("{}", &*v33);
        0i32
    } else {
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
        let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v13));
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" searched  "); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
        let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v15));
        let mut v39: Rc<str> = std::rc::Rc::<str>::from((v10).to_string().as_str());
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched  matches "); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v39));
        let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v41));
        let mut v43: US0 = US0::US0_Succeeded;
        let mut v44: bool = v9 < v10;
        let mut v49: Rc<str> = if v44 {
            let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  shown "); } LIT.with(|lit| lit.clone()) };
            let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v45));
            let mut v47: Rc<str> = std::rc::Rc::<str>::from((v9).to_string().as_str());
            let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
            v48.clone()
        } else {
            v42.clone()
        };
        receipt_line_9(v43.clone(), v49.clone(), v4);
        0i32
    }
}
fn now_ms_13() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_15() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_14() -> i32 {
    let mut v0: i32 = command_line_argument_count_15();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn search_start_16() -> (Rc<str>, u64, u64, bool, bool, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), 0u64, 200u64, false, false, false)
}
fn command_line_argument_19(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_18(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_15();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US1 = if v5 {
        let mut v6: Rc<str> = command_line_argument_19(v1);
        US1::US1_Some(v6.clone())
    } else {
        US1::US1_None
    };
    match &v9 {
        US1::US1_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US1::US1_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn method21(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let mut v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_20(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method21(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US2> = closure1();
    let mut v4: Option<US2> = v2.map(|x| v3(x));
    let mut v5: US2 = US2::US2_None;
    let mut v6: US2 = v4.unwrap_or(v5);
    match &v6 {
        US2::US2_None => {
            0u64
        }
        US2::US2_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn search_options_from_17(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool, mut v7: bool) -> (Rc<str>, u64, u64, bool, bool, bool) {
    let mut v8: bool = v0 < v1;
    let (mut v114, mut v115, mut v116, mut v117, mut v118, mut v119): (Rc<str>, u64, u64, bool, bool, bool) = if v8 {
        let mut v9: Rc<str> = argument_18(v0);
        let mut v10: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ignore-case"); } LIT.with(|lit| lit.clone()) };
        let (mut v108, mut v109, mut v110, mut v111, mut v112, mut v113): (Rc<str>, u64, u64, bool, bool, bool) = if v10 {
            let mut v11: i32 = v0.wrapping_add(1i32);
            let mut v12: bool = true;
            let (mut v13, mut v14, mut v15, mut v16, mut v17, mut v18): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_17(v11, v1, v2.clone(), v3, v4, v12, v6, v7);
            (v13.clone(), v14, v15, v16, v17, v18)
        } else {
            let mut v19: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
            let (mut v102, mut v103, mut v104, mut v105, mut v106, mut v107): (Rc<str>, u64, u64, bool, bool, bool) = if v19 {
                let mut v20: i32 = v0.wrapping_add(1i32);
                let mut v21: bool = true;
                let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_17(v20, v1, v2.clone(), v3, v4, v5, v6, v21);
                (v22.clone(), v23, v24, v25, v26, v27)
            } else {
                let mut v28: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--regex"); } LIT.with(|lit| lit.clone()) };
                let (mut v96, mut v97, mut v98, mut v99, mut v100, mut v101): (Rc<str>, u64, u64, bool, bool, bool) = if v28 {
                    let mut v29: i32 = v0.wrapping_add(1i32);
                    let mut v30: bool = true;
                    let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_17(v29, v1, v2.clone(), v3, v4, v5, v30, v7);
                    (v31.clone(), v32, v33, v34, v35, v36)
                } else {
                    let mut v37: i32 = v0.wrapping_add(1i32);
                    let mut v38: bool = v37 < v1;
                    let (mut v90, mut v91, mut v92, mut v93, mut v94, mut v95): (Rc<str>, u64, u64, bool, bool, bool) = if v38 {
                        let mut v39: Rc<str> = argument_18(v37);
                        let mut v40: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--glob"); } LIT.with(|lit| lit.clone()) };
                        let (mut v84, mut v85, mut v86, mut v87, mut v88, mut v89): (Rc<str>, u64, u64, bool, bool, bool) = if v40 {
                            let mut v41: i32 = v0.wrapping_add(2i32);
                            let (mut v42, mut v43, mut v44, mut v45, mut v46, mut v47): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_17(v41, v1, v39.clone(), v3, v4, v5, v6, v7);
                            (v42.clone(), v43, v44, v45, v46, v47)
                        } else {
                            let mut v48: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--context"); } LIT.with(|lit| lit.clone()) };
                            let (mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (Rc<str>, u64, u64, bool, bool, bool) = if v48 {
                                let mut v49: i32 = v0.wrapping_add(2i32);
                                let mut v50: u64 = decimal_u64_20(v39.clone());
                                let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_17(v49, v1, v2.clone(), v50, v4, v5, v6, v7);
                                (v51.clone(), v52, v53, v54, v55, v56)
                            } else {
                                let mut v57: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max"); } LIT.with(|lit| lit.clone()) };
                                let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77): (Rc<str>, u64, u64, bool, bool, bool) = if v57 {
                                    let mut v58: i32 = v0.wrapping_add(2i32);
                                    let mut v59: u64 = decimal_u64_20(v39.clone());
                                    let (mut v60, mut v61, mut v62, mut v63, mut v64, mut v65): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_17(v58, v1, v2.clone(), v3, v59, v5, v6, v7);
                                    (v60.clone(), v61, v62, v63, v64, v65)
                                } else {
                                    let (mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_17(v37, v1, v2.clone(), v3, v4, v5, v6, v7);
                                    (v66.clone(), v67, v68, v69, v70, v71)
                                };
                                (v72.clone(), v73, v74, v75, v76, v77)
                            };
                            (v78.clone(), v79, v80, v81, v82, v83)
                        };
                        (v84.clone(), v85, v86, v87, v88, v89)
                    } else {
                        (v2.clone(), v3, v4, v5, v6, v7)
                    };
                    (v90.clone(), v91, v92, v93, v94, v95)
                };
                (v96.clone(), v97, v98, v99, v100, v101)
            };
            (v102.clone(), v103, v104, v105, v106, v107)
        };
        (v108.clone(), v109, v110, v111, v112, v113)
    } else {
        (v2.clone(), v3, v4, v5, v6, v7)
    };
    (v114.clone(), v115, v116, v117, v118, v119)
}
fn pattern_problem_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v0).err().map(|error| error.to_string()).unwrap_or_default().as_str());
    v1.clone()
}
fn grep_run_0() -> i32 {
    let mut v0: u64 = now_ms_13();
    let mut v1: i32 = argument_count_14();
    let mut v2: bool = v1 < 4i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", &*v5);
        2i32
    } else {
        let mut v6: i32 = 4i32;
        let mut v7: i32 = argument_count_14();
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, u64, u64, bool, bool, bool) = search_start_16();
        let (mut v14, mut v15, mut v16, mut v17, mut v18, mut v19): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_17(v6, v7, v8.clone(), v9, v10, v11, v12, v13);
        let mut v20: i32 = 2i32;
        let mut v21: Rc<str> = argument_18(v20);
        let mut v22: i32 = 3i32;
        let mut v23: Rc<str> = argument_18(v22);
        let mut v28: US3 = if v18 {
            US3::US3_PatternNeedle(v17)
        } else {
            if v17 {
                US3::US3_CaselessNeedle
            } else {
                US3::US3_LiteralNeedle
            }
        };
        let mut v31: Rc<str> = if v18 {
            pattern_problem_22(v23.clone())
        } else {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v30.clone()
        };
        let mut v32: bool = v31.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v32 {
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("grep: invalid --regex pattern: "); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v31));
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
            eprintln!("{}", &*v36);
            2i32
        } else {
            let mut v37: bool = std::path::Path::new(&*v21).is_dir();
            let mut v43: US4 = if v37 {
                US4::US4_FolderRoot
            } else {
                let mut v39: bool = std::path::Path::new(&*v21).is_file();
                if v39 {
                    US4::US4_FileRoot
                } else {
                    US4::US4_MissingRoot
                }
            };
            match &v43 {
                US4::US4_MissingRoot => {
                    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("grep root not found: "); } LIT.with(|lit| lit.clone()) };
                    let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v21));
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v45));
                    eprintln!("{}", &*v47);
                    2i32
                }
                _ => {
                    let mut v50: std::rc::Rc<Vec<std::path::PathBuf>> = match &v43 {
                        US4::US4_FileRoot => {
                            let mut v48: std::rc::Rc<Vec<std::path::PathBuf>> = std::rc::Rc::new(vec![std::path::PathBuf::from(&*v21)]);
                            v48.clone()
                        }
                        _ => {
                            let mut v49: std::rc::Rc<Vec<std::path::PathBuf>> = { let skip = ["target", "node_modules", ".git", "_build", "deps", "dist", ".cache", "obj"]; let root = std::path::PathBuf::from(&*v21); let mut stack = vec![root.clone()]; let mut files = Vec::new(); while let Some(folder) = stack.pop() { let Ok(entries) = std::fs::read_dir(&folder) else { continue }; for entry in entries.filter_map(|entry| entry.ok()) { let path = entry.path(); let Ok(kind) = entry.file_type() else { continue }; if kind.is_symlink() { continue; } if kind.is_dir() { if !skip.iter().any(|name| path.file_name().map(|value| value == *name).unwrap_or(false)) { stack.push(path); } } else if kind.is_file() { files.push(path); } } } std::rc::Rc::new(files) };
                            v49.clone()
                        }
                    };
                    let mut v57: std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> = match &v28 {
                        US3::US3_CaselessNeedle => {
                            let mut v52: std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> = { let needle = (&*v23).to_lowercase(); std::sync::Arc::new(move |line: &str| line.to_lowercase().contains(needle.as_str())) as std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> };
                            v52.clone()
                        }
                        US3::US3_LiteralNeedle => {
                            let mut v51: std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> = { let needle = (&*v23).to_owned(); std::sync::Arc::new(move |line: &str| line.contains(needle.as_str())) as std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> };
                            v51.clone()
                        }
                        US3::US3_PatternNeedle(v53) => {
                            let mut v53: bool = *v53;
                            let mut v54: std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> = { let found = regex::RegexBuilder::new(&*v23).case_insensitive(v53).build().unwrap(); std::sync::Arc::new(move |line: &str| found.is_match(line)) as std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> };
                            v54.clone()
                        }
                    };
                    let mut v58: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)> = (|| { use rayon::prelude::*; let root = std::path::PathBuf::from(&*v21); let mut files = (*v50).clone(); let glob = &*v14; let (head, tail) = glob.split_once(char::from(42u8)).unwrap_or((glob, "")); let wanted = |path: &std::path::Path| glob.is_empty() || path.file_name().map(|name| { let name = name.to_string_lossy(); if glob.contains(char::from(42u8)) { name.starts_with(head) && name.ends_with(tail) } else { name == glob } }).unwrap_or(false); files.retain(|path| wanted(path)); let matcher = (v57).clone(); let mut hits = files.par_iter().filter_map(|path| { let bytes = std::fs::read(path).ok()?; let text = String::from_utf8_lossy(&bytes); let lines = text.lines().map(str::to_owned).collect::<Vec<_>>(); let found = lines.iter().enumerate().filter(|(_, line)| matcher(line.as_str())).map(|(index, _)| index).collect::<Vec<_>>(); if found.is_empty() { return None; } Some((path.clone(), found, lines)) }).collect::<Vec<_>>(); hits.sort_by(|left, right| left.0.cmp(&right.0)); let shown = |path: &std::path::Path| path.strip_prefix(&root).ok().filter(|relative| !relative.as_os_str().is_empty()).unwrap_or(path).to_string_lossy().replace(char::from(92u8), "/"); std::rc::Rc::new((files.len() as u64, hits.into_iter().map(|(path, found, lines)| (shown(&path), found, lines)).collect::<Vec<_>>())) })();
                    let mut v59: i32 = v15 as i32;
                    let mut v60: i32 = v16 as i32;
                    search_receipt_1(v58.clone(), v19, v59, v60, v0)
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        grep_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_grep_run() -> i32 {
    closure0()()
}
