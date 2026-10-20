#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ScanPlain,
    US0_ScanQuoted,
    US0_ScanEscaped,
}
fn field_end_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US0::US0_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let mut v10: i32 = v1.wrapping_add(1i32);
                let mut v25: US0 = match &v3 {
                    US0::US0_ScanEscaped => {
                        US0::US0_ScanQuoted
                    }
                    US0::US0_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US0::US0_ScanQuoted
                        } else {
                            US0::US0_ScanPlain
                        }
                    }
                    US0::US0_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US0::US0_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US0::US0_ScanPlain
                            } else {
                                US0::US0_ScanQuoted
                            }
                        }
                    }
                };
                (v0, v1, v2, v3) = (v0.clone(), v10, v2, v25.clone());
                continue;
            }
        }
    }
}
fn skip_spaces_6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US0 = US0::US0_ScanPlain;
            let mut v6: i32 = field_end_5(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_6(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn count_status_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: i32) -> i32 {
    loop {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v7: bool = v6 < 0i32;
        if v7 {
            return v4;
        } else {
            let mut v8: i32 = v6 + 6i32;
            let mut v9: i32 = v0.get(..v6 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(v9 as usize..v6 as usize).unwrap_or(""));
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v12: bool = v10.starts_with(&*v11);
            let mut v19: bool = if v12 {
                let mut v13: i32 = 6i32;
                let mut v14: i32 = field_start_4(v0.clone(), v8, v2, v13);
                let mut v15: US0 = US0::US0_ScanPlain;
                let mut v16: i32 = field_end_5(v0.clone(), v14, v2, v15.clone());
                let mut v17: Rc<str> = std::rc::Rc::<str>::from(v0.get(v14 as usize..v16 as usize).unwrap_or(""));
                let mut v18: bool = v17 == v3 ;
                v18
            } else {
                false
            };
            let mut v21: i32 = if v19 {
                let mut v20: i32 = v4.wrapping_add(1i32);
                v20
            } else {
                v4
            };
            (v0, v1, v2, v3, v4) = (v0.clone(), v8, v2, v3.clone(), v21);
            continue;
        }
    }
}
fn status_count_text_2(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(v2.to_ascii_lowercase());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let mut v8: i32 = count_status_3(v0.clone(), v6, v1, v2.clone(), v7);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v5, &*v9, &*v10].concat());
    v11.clone()
}
fn status_counts_1(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = status_count_text_2(v0.clone(), v1, v2.clone());
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v3.clone()));
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v4.clone()));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = status_count_text_2(v0.clone(), v1, v6.clone());
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v7.clone()));
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), v8.clone()));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = status_count_text_2(v0.clone(), v1, v10.clone());
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v11.clone()));
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v9.clone(), v12.clone()));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = status_count_text_2(v0.clone(), v1, v14.clone());
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v15.clone()));
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), v16.clone()));
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = status_count_text_2(v0.clone(), v1, v18.clone());
    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v19.clone()));
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17.clone(), v20.clone()));
    v21.clone()
}
fn census_text_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = v0.len() as i32;
    status_counts_1(v0.clone(), v1)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        census_text_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn status_count_json_9(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(v2.to_ascii_lowercase());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let mut v8: i32 = count_status_3(v0.clone(), v6, v1, v2.clone(), v7);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v5, &*v9, &*v10].concat());
    v11.clone()
}
fn status_counts_json_8(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = status_count_json_9(v0.clone(), v1, v2.clone());
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v3.clone()));
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v4.clone()));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = status_count_json_9(v0.clone(), v1, v6.clone());
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }, v7.clone()));
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), v8.clone()));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = status_count_json_9(v0.clone(), v1, v10.clone());
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }, v11.clone()));
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v9.clone(), v12.clone()));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = status_count_json_9(v0.clone(), v1, v14.clone());
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }, v15.clone()));
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), v16.clone()));
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = status_count_json_9(v0.clone(), v1, v18.clone());
    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }, v19.clone()));
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17.clone(), v20.clone()));
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"counts\":{"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v21, &*v23, &*v24, &*v24].concat());
    v25.clone()
}
fn census_json_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = v0.len() as i32;
    status_counts_json_8(v0.clone(), v1)
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        census_json_7(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn status_of_id_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> Rc<str> {
    loop {
        let mut v4: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v6.clone();
        } else {
            let mut v7: i32 = v4 + 6i32;
            let mut v8: i32 = v0.get(..v4 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v4 as usize).unwrap_or(""));
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v11: bool = v9.starts_with(&*v10);
            if v11 {
                let mut v12: i32 = 6i32;
                let mut v13: i32 = field_start_4(v0.clone(), v7, v3, v12);
                let mut v14: US0 = US0::US0_ScanPlain;
                let mut v15: i32 = field_end_5(v0.clone(), v13, v3, v14.clone());
                let mut v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v15 as usize).unwrap_or(""));
                return v16.clone();
            } else {
                let mut v17: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v17, v3);
                continue;
            }
        }
    }
}
fn task_status_of_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v1, &*v3, &*v4, &*v4].concat());
    let mut v6: i32 = 0i32;
    let mut v7: i32 = v0.len() as i32;
    status_of_id_11(v0.clone(), v5.clone(), v6, v7)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        task_status_of_10(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_status_counts(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_agile_status_counts_json(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
pub fn eoie_agile_task_status(v0: &str, v1: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
