#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_CountMatches,
    US0_SumCaptures,
    US0_PathField,
    US0_CountByCapture,
    US0_LastCapture,
    US0_ValuesPerMatch,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoMeasures,
    UH0_MeasureThen(US0, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_RequestTaken(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<UH0>),
    US1_RequestRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_KindKnown(US0),
    US3_KindUnknown,
}
#[derive(Clone)]
enum US4 {
    US4_SpecTaken(Rc<UH0>),
    US4_SpecRejected(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_PerFile,
    US5_PerMatch(Rc<str>, Rc<str>),
    US5_MixedShapes,
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn failed_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table logs "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v0));
    stderr_line_2(v2.clone());
    2i32
}
fn stdout_text_4(mut v0: Rc<str>) -> () {
    { use std::io::Write; let mut out = std::io::stdout().lock(); let _ = out.write_all((v0).as_bytes()); };
    ()
}
fn text_written_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::write(&*v0, (v1).as_bytes()).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default().as_str());
    v2.clone()
}
fn stdout_line_6(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn table_written_3(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5.clone()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v0));
        v7.clone()
    };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v8));
    if v4 {
        stdout_text_4(v2.clone());
        if v1 {
            ()
        } else {
            stderr_line_2(v9.clone())
        };
        0i32
    } else {
        let mut v10: Rc<str> = text_written_5(v0.clone(), v2.clone());
        let mut v11: bool = v10.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v11 {
            failed_1(v10.clone())
        } else {
            if v1 {
                ()
            } else {
                stdout_line_6(v9.clone())
            };
            0i32
        }
    }
}
fn files_listed_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ fn wildcard(pattern: &[u8], name: &[u8]) -> bool { match (pattern.first(), name.first()) { (None, None) => true, (Some(&42u8), _) => wildcard(&pattern[1..], name) || (!name.is_empty() && wildcard(pattern, &name[1..])), (Some(&63u8), Some(_)) => wildcard(&pattern[1..], &name[1..]), (Some(a), Some(b)) if a.eq_ignore_ascii_case(b) => wildcard(&pattern[1..], &name[1..]), _ => false } } fn walk(path: &std::path::Path, found: &mut Vec<std::path::PathBuf>) { if path.is_dir() { let mut entries = std::fs::read_dir(path).into_iter().flatten().flatten().map(|entry| entry.path()).collect::<Vec<_>>(); entries.sort(); for entry in entries { walk(&entry, found); } } else if path.is_file() { found.push(path.to_path_buf()); } } let glob = &*v1; let mut lines = Vec::new(); for raw in (v0).split(char::from(10u8)).filter(|item| !item.is_empty()) { let path = std::path::Path::new(raw); if !path.exists() { lines.push(format!("{}{raw}", char::from(1u8))); continue; } let mut found = Vec::new(); walk(path, &mut found); for file in found { let name = file.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(); if glob.is_empty() || wildcard(glob.as_bytes(), name.as_bytes()) { lines.push(file.to_string_lossy().replace(char::from(92u8), "/")); } } } lines.join("
") }.as_str());
    v2.clone()
}
fn first_of_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
    v1.clone()
}
fn missing_marker_11(mut v0: Rc<str>) -> bool {
    let mut v1: bool = (v0).starts_with(char::from(1u8));
    v1
}
fn rest_of_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
    v1.clone()
}
fn file_text_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(String::from_utf8_lossy(&std::fs::read(&*v0).unwrap_or_default()).as_ref());
    v1.clone()
}
fn text_since_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v0.clone()
    } else {
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v1).ok().and_then(|marker| marker.find(&*v0)).map(|found| (&(v0)[found.end()..]).split_once(char::from(10u8)).map(|(_, after)| after).unwrap_or("")).unwrap_or(""));
        v3.clone()
    }
}
fn match_end_from_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    let mut v3: i32 = regex::Regex::new(&*v1).ok().and_then(|p| p.find_at(&*v0, v2 as usize)).map(|m| m.end().max(v2 as usize + 1) as i32).unwrap_or(-1);
    v3
}
fn capture_from_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v1).ok().and_then(|p| p.captures_at(&*v0, v2 as usize)).and_then(|f| f.get(1).or(f.get(0))).map(|g| g.as_str()).unwrap_or(""));
    v3.clone()
}
fn matched_rows_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32) -> (Rc<str>, i32) {
    loop {
        let mut v6: i32 = match_end_from_16(v1.clone(), v2.clone(), v3);
        let mut v7: bool = v6 < 0i32;
        let mut v10: bool = if v7 {
            true
        } else {
            let mut v8: i32 = (v1.clone().len() as i32);
            let mut v9: bool = v6 > v8;
            v9
        };
        if v10 {
            return (v4.clone(), v5);
        } else {
            let mut v11: Rc<str> = capture_from_17(v1.clone(), v2.clone(), v3);
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v13));
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v0));
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v12));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v14));
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
            let mut v20: i32 = v5.wrapping_add(1i32);
            (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2.clone(), v6, v19.clone(), v20);
            continue;
        }
    }
}
fn values_rows_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: Rc<str>) -> (Rc<str>, i32, i32, Rc<str>) {
    loop {
        let mut v7: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v7 {
            return (v3.clone(), v4, v5, v6.clone());
        } else {
            let mut v8: Rc<str> = first_of_10(v2.clone());
            let mut v9: bool = missing_marker_11(v8.clone());
            if v9 {
                let mut v10: Rc<str> = rest_of_12(v2.clone());
                let mut v11: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = if v11 {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v12.clone()
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                };
                let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v14));
                let mut v16: Rc<str> = std::rc::Rc::<str>::from(&(v8)[1..]);
                let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v10.clone(), v3.clone(), v4, v5, v17.clone());
                continue;
            } else {
                let mut v22: Rc<str> = file_text_13(v8.clone());
                let mut v23: Rc<str> = text_since_14(v22.clone(), v1.clone());
                let mut v24: i32 = 0i32;
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v26: i32 = 0i32;
                let (mut v27, mut v28): (Rc<str>, i32) = matched_rows_15(v8.clone(), v23.clone(), v0.clone(), v24, v25.clone(), v26);
                let mut v29: Rc<str> = rest_of_12(v2.clone());
                let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v27));
                let mut v31: i32 = v4.wrapping_add(1i32);
                let mut v32: i32 = v5.wrapping_add(v28);
                (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v29.clone(), v30.clone(), v31, v32, v6.clone());
                continue;
            }
        }
    }
}
fn count_text_18(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string().as_str());
    v1.clone()
}
fn values_run_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: Rc<str>) -> i32 {
    let mut v7: Rc<str> = files_listed_8(v0.clone(), v1.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: i32 = 0i32;
    let mut v10: i32 = 0i32;
    let (mut v11, mut v12, mut v13, mut v14): (Rc<str>, i32, i32, Rc<str>) = values_rows_9(v6.clone(), v2.clone(), v7.clone(), v8.clone(), v9, v10, v8.clone());
    let mut v15: bool = v14.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v15 {
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such file or directory: "); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v14));
        failed_1(v17.clone())
    } else {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file\t"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v5));
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v11));
        let mut v24: Rc<str> = count_text_18(v12);
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie table logs ok files="); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rows="); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
        let mut v29: Rc<str> = count_text_18(v13);
        let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns=1"); } LIT.with(|lit| lit.clone()) };
        let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
        table_written_3(v3.clone(), v4, v23.clone(), v32.clone())
    }
}
fn command_line_argument_count_20() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_19() -> i32 {
    let mut v0: i32 = command_line_argument_count_20();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn command_line_argument_23(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_22(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_20();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US2 = if v5 {
        let mut v6: Rc<str> = command_line_argument_23(v1);
        US2::US2_Some(v6.clone())
    } else {
        US2::US2_None
    };
    match &v9 {
        US2::US2_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US2::US2_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn kind_of_24(mut v0: Rc<str>) -> US3 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--count"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US0 = US0::US0_CountMatches;
        US3::US3_KindKnown(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--sum"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US0 = US0::US0_SumCaptures;
            US3::US3_KindKnown(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--field"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US0 = US0::US0_PathField;
                US3::US3_KindKnown(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--count-by"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US0 = US0::US0_CountByCapture;
                    US3::US3_KindKnown(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--last"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US0 = US0::US0_LastCapture;
                        US3::US3_KindKnown(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--values"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US0 = US0::US0_ValuesPerMatch;
                            US3::US3_KindKnown(v17.clone())
                        } else {
                            US3::US3_KindUnknown
                        }
                    }
                }
            }
        }
    }
}
fn name_ok_26(mut v0: Rc<str>) -> bool {
    let mut v1: bool = !(v0).is_empty() && (v0).chars().all(|c| c.is_ascii_alphanumeric() || c == char::from(95u8) || c == char::from(45u8));
    v1
}
fn measures_appended_27(mut v0: Rc<UH0>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_MeasureThen(v7, v8, v9, v10, v11) => {
            let mut v7: US0 = v7.clone();
            let mut v8: Rc<str> = v8.clone();
            let mut v9: Rc<str> = v9.clone();
            let mut v10: Rc<str> = v10.clone();
            let mut v11: Rc<UH0> = v11.clone();
            let mut v12: Rc<UH0> = measures_appended_27(v11.clone(), v1.clone(), v2.clone(), v3.clone());
            Rc::new(UH0::UH0_MeasureThen(v7.clone(), v8.clone(), v9.clone(), v10.clone(), v12.clone()))
        }
        UH0::UH0_NoMeasures => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMeasures); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_MeasureThen(v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone()))
        }
    }
}
fn spec_applied_25(mut v0: Rc<UH0>, mut v1: US0, mut v2: Rc<str>) -> US4 {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(61u8)).map(|(name, _)| name).unwrap_or(""));
    let mut v4: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(61u8)).map(|(_, pattern)| pattern).unwrap_or(""));
    let mut v5: bool = name_ok_26(v3.clone());
    let mut v6: bool = v5 == false;
    let mut v8: bool = if v6 {
        true
    } else {
        let mut v7: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v7
    };
    if v8 {
        let mut v20: Rc<str> = match &v1 {
            US0::US0_CountByCapture => {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--count-by"); } LIT.with(|lit| lit.clone()) };
                v12.clone()
            }
            US0::US0_CountMatches => {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--count"); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            }
            US0::US0_LastCapture => {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--last"); } LIT.with(|lit| lit.clone()) };
                v13.clone()
            }
            US0::US0_PathField => {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--field"); } LIT.with(|lit| lit.clone()) };
                v11.clone()
            }
            US0::US0_SumCaptures => {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--sum"); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            }
            US0::US0_ValuesPerMatch => {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--values"); } LIT.with(|lit| lit.clone()) };
                v14.clone()
            }
        };
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects name=regex with a letters/digits/_/- name, received '"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v2));
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
        US4::US4_SpecRejected(v25.clone())
    } else {
        let mut v27: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file"); } LIT.with(|lit| lit.clone()) };
        if v27 {
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the column name 'file' is taken by the file path"); } LIT.with(|lit| lit.clone()) };
            US4::US4_SpecRejected(v28.clone())
        } else {
            let mut v30: Rc<UH0> = measures_appended_27(v0.clone(), v1.clone(), v3.clone(), v4.clone());
            US4::US4_SpecTaken(v30.clone())
        }
    }
}
fn options_from_21(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool, mut v7: Rc<UH0>) -> US1 {
    loop {
        let mut v8: bool = v0 >= v1;
        if v8 {
            return US1::US1_RequestTaken(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone());
        } else {
            let mut v10: Rc<str> = argument_22(v0);
            let mut v11: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet"); } LIT.with(|lit| lit.clone()) };
            if v11 {
                let mut v12: i32 = v0.wrapping_add(1i32);
                let mut v13: bool = true;
                (v0, v1, v2, v3, v4, v5, v6, v7) = (v12, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v13, v7.clone());
                continue;
            } else {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                let mut v16: bool = v10.starts_with(&*v15);
                if v16 {
                    let mut v17: i32 = v0.wrapping_add(1i32);
                    let mut v18: bool = v17 >= v1;
                    if v18 {
                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) };
                        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v10));
                        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) };
                        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                        return US1::US1_RequestRejected(v22.clone());
                    } else {
                        let mut v24: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--glob"); } LIT.with(|lit| lit.clone()) };
                        if v24 {
                            let mut v25: i32 = v0.wrapping_add(2i32);
                            let mut v26: Rc<str> = argument_22(v17);
                            (v0, v1, v2, v3, v4, v5, v6, v7) = (v25, v1, v2.clone(), v26.clone(), v4.clone(), v5.clone(), v6, v7.clone());
                            continue;
                        } else {
                            let mut v28: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--since"); } LIT.with(|lit| lit.clone()) };
                            if v28 {
                                let mut v29: i32 = v0.wrapping_add(2i32);
                                let mut v30: Rc<str> = argument_22(v17);
                                (v0, v1, v2, v3, v4, v5, v6, v7) = (v29, v1, v2.clone(), v3.clone(), v4.clone(), v30.clone(), v6, v7.clone());
                                continue;
                            } else {
                                let mut v32: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--out"); } LIT.with(|lit| lit.clone()) };
                                if v32 {
                                    let mut v33: i32 = v0.wrapping_add(2i32);
                                    let mut v34: Rc<str> = argument_22(v17);
                                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v33, v1, v2.clone(), v3.clone(), v34.clone(), v5.clone(), v6, v7.clone());
                                    continue;
                                } else {
                                    let mut v36: US3 = kind_of_24(v10.clone());
                                    match &v36 {
                                        US3::US3_KindKnown(v40) => {
                                            let mut v40: US0 = v40.clone();
                                            let mut v41: Rc<str> = argument_22(v17);
                                            let mut v42: US4 = spec_applied_25(v7.clone(), v40.clone(), v41.clone());
                                            match &v42 {
                                                US4::US4_SpecRejected(v43) => {
                                                    let mut v43: Rc<str> = v43.clone();
                                                    return US1::US1_RequestRejected(v43.clone());
                                                }
                                                US4::US4_SpecTaken(v45) => {
                                                    let mut v45: Rc<UH0> = v45.clone();
                                                    let mut v46: i32 = v0.wrapping_add(2i32);
                                                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v46, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v45.clone());
                                                    continue;
                                                }
                                            }
                                        }
                                        US3::US3_KindUnknown => {
                                            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                                            let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v10));
                                            return US1::US1_RequestRejected(v38.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    let mut v56: i32 = v0.wrapping_add(1i32);
                    let mut v57: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v61: Rc<str> = if v57 {
                        v10.clone()
                    } else {
                        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v58));
                        let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v10));
                        v60.clone()
                    };
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v56, v1, v61.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone());
                    continue;
                }
            }
        }
    }
}
fn measure_count_28(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_MeasureThen(v2, v3, v4, v5, v6) => {
                let mut v2: US0 = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6.clone(), v7);
                continue;
            }
            UH0::UH0_NoMeasures => {
                return v1;
            }
        }
    }
}
fn pattern_error_30(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v0).err().map(|error| error.to_string()).unwrap_or_default().as_str());
    v1.clone()
}
fn pattern_problem_29(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_MeasureThen(v2, v3, v4, v5, v6) => {
                let mut v2: US0 = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: Rc<str> = pattern_error_30(v4.clone());
                let mut v8: bool = v7.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v8 {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid pattern for "); } LIT.with(|lit| lit.clone()) };
                    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v3));
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
                    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
                    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v7));
                    return v13.clone();
                } else {
                    v0 = v6.clone();
                    continue;
                }
            }
            UH0::UH0_NoMeasures => {
                let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v1.clone();
            }
        }
    }
}
fn values_measure_32(mut v0: Rc<UH0>) -> (Rc<str>, Rc<str>) {
    loop {
        match &*v0 {
            UH0::UH0_MeasureThen(v2, v3, v4, v5, v6) => {
                let mut v2: US0 = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                match &v2 {
                    US0::US0_ValuesPerMatch => {
                        return (v3.clone(), v4.clone());
                    }
                    _ => {
                        v0 = v6.clone();
                        continue;
                    }
                }
            }
            UH0::UH0_NoMeasures => {
                let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return (v1.clone(), v1.clone());
            }
        }
    }
}
fn shape_of_31(mut v0: Rc<UH0>) -> US5 {
    let (mut v1, mut v2): (Rc<str>, Rc<str>) = values_measure_32(v0.clone());
    let mut v3: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v3 {
        US5::US5_PerFile
    } else {
        let mut v5: i32 = 0i32;
        let mut v6: i32 = measure_count_28(v0.clone(), v5);
        let mut v7: bool = v6 == 1i32;
        if v7 {
            US5::US5_PerMatch(v1.clone(), v2.clone())
        } else {
            US5::US5_MixedShapes
        }
    }
}
fn value_measures_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<UH0> {
    let mut v4: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v4 {
        v3.clone()
    } else {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from((v2).split(char::from(10u8)).next().unwrap_or(""));
        let mut v6: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v7));
        let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v5));
        let mut v10: Rc<UH0> = value_measures_34(v0.clone(), v1.clone(), v6.clone(), v3.clone());
        let mut v11: US0 = US0::US0_CountByCapture;
        Rc::new(UH0::UH0_MeasureThen(v11.clone(), v9.clone(), v1.clone(), v5.clone(), v10.clone()))
    }
}
fn capture_values_35(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v1).map(|pattern| { let mut values = std::collections::BTreeSet::new(); for file in (v0).split(char::from(10u8)).filter(|line| !line.is_empty() && !line.starts_with(char::from(1u8))) { let text = String::from_utf8_lossy(&std::fs::read(file).unwrap_or_default()).into_owned(); for found in pattern.captures_iter(&text) { if let Some(group) = found.get(1) { values.insert(group.as_str().replace(char::from(9u8), " ").replace(char::from(10u8), " ")); } } } values.into_iter().collect::<Vec<_>>().join("
") }).unwrap_or_default().as_str());
    v2.clone()
}
fn expanded_33(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_MeasureThen(v3, v4, v5, v6, v7) => {
            let mut v3: US0 = v3.clone();
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = v6.clone();
            let mut v7: Rc<UH0> = v7.clone();
            match &v3 {
                US0::US0_CountByCapture => {
                    let mut v8: Rc<str> = capture_values_35(v1.clone(), v5.clone());
                    let mut v9: Rc<UH0> = expanded_33(v7.clone(), v1.clone());
                    value_measures_34(v4.clone(), v5.clone(), v8.clone(), v9.clone())
                }
                _ => {
                    let mut v11: Rc<UH0> = expanded_33(v7.clone(), v1.clone());
                    Rc::new(UH0::UH0_MeasureThen(v3.clone(), v4.clone(), v5.clone(), v6.clone(), v11.clone()))
                }
            }
        }
        UH0::UH0_NoMeasures => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMeasures); } CASE.with(|case| case.clone()) }
        }
    }
}
fn value_count_38(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = regex::Regex::new(&*v1).map(|pattern| pattern.captures_iter(&*v0).filter(|found| found.get(1).map(|group| group.as_str() == &*v2).unwrap_or(false)).count() as i32).unwrap_or(0);
    v3
}
fn match_count_39(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = regex::Regex::new(&*v1).map(|pattern| pattern.find_iter(&*v0).count() as i32).unwrap_or(0);
    v2
}
fn capture_at_40(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> Rc<str> {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v1).ok().and_then(|pattern| { let mut found = pattern.captures_iter(&*v0); if v2 { found.last() } else { found.next() } }.and_then(|found| found.get(1).or_else(|| found.get(0)).map(|group| group.as_str().replace(char::from(9u8), " ")))).unwrap_or_default().as_str());
    v3.clone()
}
fn capture_sum_41(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v1).map(|pattern| { let sum: f64 = pattern.captures_iter(&*v0).filter_map(|found| found.get(1).and_then(|group| group.as_str().trim().parse::<f64>().ok())).sum(); let text = format!("{:.6}", sum + 0.0); text.trim_end_matches(char::from(48u8)).trim_end_matches(char::from(46u8)).to_owned() }).unwrap_or_default().as_str());
    v2.clone()
}
fn row_cells_37(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    match &*v0 {
        UH0::UH0_MeasureThen(v4, v5, v6, v7, v8) => {
            let mut v4: US0 = v4.clone();
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = v6.clone();
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<UH0> = v8.clone();
            let mut v24: Rc<str> = match &v4 {
                US0::US0_CountByCapture => {
                    let mut v11: i32 = value_count_38(v2.clone(), v6.clone(), v7.clone());
                    count_text_18(v11)
                }
                US0::US0_CountMatches => {
                    let mut v9: i32 = match_count_39(v2.clone(), v6.clone());
                    count_text_18(v9)
                }
                US0::US0_LastCapture => {
                    let mut v16: bool = true;
                    capture_at_40(v2.clone(), v6.clone(), v16)
                }
                US0::US0_PathField => {
                    let mut v14: bool = false;
                    capture_at_40(v1.clone(), v6.clone(), v14)
                }
                US0::US0_SumCaptures => {
                    capture_sum_41(v2.clone(), v6.clone())
                }
                US0::US0_ValuesPerMatch => {
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v18.clone()
                }
            };
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
            let mut v27: Rc<str> = row_cells_37(v8.clone(), v1.clone(), v2.clone());
            let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
            v28.clone()
        }
        UH0::UH0_NoMeasures => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn rows_of_36(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>) -> (Rc<str>, i32, Rc<str>) {
    loop {
        let mut v6: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v6 {
            return (v3.clone(), v4, v5.clone());
        } else {
            let mut v7: Rc<str> = first_of_10(v2.clone());
            let mut v8: bool = missing_marker_11(v7.clone());
            if v8 {
                let mut v9: Rc<str> = rest_of_12(v2.clone());
                let mut v10: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                } else {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    v12.clone()
                };
                let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v13));
                let mut v15: Rc<str> = std::rc::Rc::<str>::from(&(v7)[1..]);
                let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v9.clone(), v3.clone(), v4, v16.clone());
                continue;
            } else {
                let mut v20: Rc<str> = rest_of_12(v2.clone());
                let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v7));
                let mut v22: Rc<str> = file_text_13(v7.clone());
                let mut v23: Rc<str> = text_since_14(v22.clone(), v1.clone());
                let mut v24: Rc<str> = row_cells_37(v0.clone(), v7.clone(), v23.clone());
                let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v24));
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
                let mut v28: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v20.clone(), v27.clone(), v28, v5.clone());
                continue;
            }
        }
    }
}
fn measure_header_42(mut v0: Rc<UH0>) -> Rc<str> {
    match &*v0 {
        UH0::UH0_MeasureThen(v2, v3, v4, v5, v6) => {
            let mut v2: US0 = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<UH0> = v6.clone();
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v3));
            let mut v9: Rc<str> = measure_header_42(v6.clone());
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
            v10.clone()
        }
        UH0::UH0_NoMeasures => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
    }
}
fn logs_run_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: i32 = argument_count_19();
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = false;
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMeasures); } CASE.with(|case| case.clone()) };
    let mut v5: US1 = options_from_21(v0, v1, v2.clone(), v2.clone(), v2.clone(), v2.clone(), v3, v4.clone());
    match &v5 {
        US1::US1_RequestRejected(v6) => {
            let mut v6: Rc<str> = v6.clone();
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet]"); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
            failed_1(v10.clone())
        }
        US1::US1_RequestTaken(v12, v13, v14, v15, v16, v17) => {
            let mut v12: Rc<str> = v12.clone();
            let mut v13: Rc<str> = v13.clone();
            let mut v14: Rc<str> = v14.clone();
            let mut v15: Rc<str> = v15.clone();
            let mut v16: bool = *v16;
            let mut v17: Rc<UH0> = v17.clone();
            let mut v18: bool = v12.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v18 {
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one file or directory; "); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet]"); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one file or directory; usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet]"); } LIT.with(|lit| lit.clone()) };
                failed_1(v21.clone())
            } else {
                let mut v23: i32 = 0i32;
                let mut v24: i32 = measure_count_28(v17.clone(), v23);
                let mut v25: bool = v24 == 0i32;
                if v25 {
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one --count, --count-by, --sum, --last or --field; "); } LIT.with(|lit| lit.clone()) };
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet]"); } LIT.with(|lit| lit.clone()) };
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one --count, --count-by, --sum, --last or --field; usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet]"); } LIT.with(|lit| lit.clone()) };
                    failed_1(v28.clone())
                } else {
                    let mut v30: Rc<str> = pattern_problem_29(v17.clone());
                    let mut v31: bool = v30.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v31 {
                        let mut v32: Rc<str> = pattern_problem_29(v17.clone());
                        failed_1(v32.clone())
                    } else {
                        let mut v34: bool = v15.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v37: bool = if v34 {
                            let mut v35: Rc<str> = pattern_error_30(v15.clone());
                            let mut v36: bool = v35.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v36
                        } else {
                            false
                        };
                        if v37 {
                            let mut v38: Rc<str> = pattern_error_30(v15.clone());
                            let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid --since pattern: "); } LIT.with(|lit| lit.clone()) };
                            let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v38));
                            failed_1(v40.clone())
                        } else {
                            let mut v42: US5 = shape_of_31(v17.clone());
                            match &v42 {
                                US5::US5_MixedShapes => {
                                    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--values makes one row per match and cannot share the table with other measures; "); } LIT.with(|lit| lit.clone()) };
                                    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet]"); } LIT.with(|lit| lit.clone()) };
                                    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--values makes one row per match and cannot share the table with other measures; usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet]"); } LIT.with(|lit| lit.clone()) };
                                    failed_1(v45.clone())
                                }
                                US5::US5_PerFile => {
                                    let mut v50: Rc<str> = files_listed_8(v12.clone(), v13.clone());
                                    let mut v51: Rc<UH0> = expanded_33(v17.clone(), v50.clone());
                                    let mut v52: i32 = 0i32;
                                    let (mut v53, mut v54, mut v55): (Rc<str>, i32, Rc<str>) = rows_of_36(v51.clone(), v15.clone(), v50.clone(), v2.clone(), v52, v2.clone());
                                    let mut v56: bool = v55.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    if v56 {
                                        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such file or directory: "); } LIT.with(|lit| lit.clone()) };
                                        let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v55));
                                        failed_1(v58.clone())
                                    } else {
                                        let mut v60: Rc<str> = measure_header_42(v51.clone());
                                        let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file"); } LIT.with(|lit| lit.clone()) };
                                        let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v60));
                                        let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                        let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v63));
                                        let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v53));
                                        let mut v66: Rc<str> = count_text_18(v54);
                                        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie table logs ok files="); } LIT.with(|lit| lit.clone()) };
                                        let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v66));
                                        let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns="); } LIT.with(|lit| lit.clone()) };
                                        let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v69));
                                        let mut v71: i32 = 0i32;
                                        let mut v72: i32 = measure_count_28(v51.clone(), v71);
                                        let mut v73: Rc<str> = count_text_18(v72);
                                        let mut v74: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v73));
                                        table_written_3(v14.clone(), v16, v65.clone(), v74.clone())
                                    }
                                }
                                US5::US5_PerMatch(v47, v48) => {
                                    let mut v47: Rc<str> = v47.clone();
                                    let mut v48: Rc<str> = v48.clone();
                                    values_run_7(v12.clone(), v13.clone(), v15.clone(), v14.clone(), v16, v47.clone(), v48.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        logs_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_table_logs_run() -> i32 {
    closure0()()
}
