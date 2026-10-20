#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_Search,
    US1_ReadAround,
    US1_ListFiles,
    US1_Head,
    US1_Tail,
    US1_Wait,
}
#[derive(Clone)]
enum US0 {
    US0_QueryParsed(US1, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64),
    US0_QueryRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Bare(Rc<str>),
    US2_Text(Rc<str>),
    US2_Finished,
}
#[derive(Clone)]
enum US3 {
    US3_Succeeded,
    US3_Failed,
}
fn now_ms_1() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_at_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn verb_of_6(mut v0: Rc<str>) -> US1 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        US1::US1_Search
    } else {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read"); } LIT.with(|lit| lit.clone()) };
        if v3 {
            US1::US1_ReadAround
        } else {
            let mut v5: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
            if v5 {
                US1::US1_ListFiles
            } else {
                let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("head"); } LIT.with(|lit| lit.clone()) };
                if v7 {
                    US1::US1_Head
                } else {
                    let mut v9: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                    if v9 {
                        US1::US1_Wait
                    } else {
                        US1::US1_Tail
                    }
                }
            }
        }
    }
}
fn opened_query_7(mut v0: US1) -> (US1, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v1.clone(), false, v1.clone(), 0u64, 0u64, 0u64, v1.clone(), v1.clone(), false, 0u64, 0u64)
}
fn item_at_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> US2 {
    let mut v3: Rc<str> = field_at_5(v0.clone(), v1, v2);
    let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v4 {
        US2::US2_Finished
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(&(&*v3)[..1]);
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(&(&*v3)[1..]);
        let mut v8: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("q"); } LIT.with(|lit| lit.clone()) };
        if v8 {
            US2::US2_Text(v7.clone())
        } else {
            US2::US2_Bare(v7.clone())
        }
    }
}
fn value_at_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: US2 = item_at_9(v0.clone(), v1, v2);
    match &v3 {
        US2::US2_Bare(v4) => {
            let mut v4: Rc<str> = v4.clone();
            v4.clone()
        }
        US2::US2_Finished => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US2::US2_Text(v5) => {
            let mut v5: Rc<str> = v5.clone();
            v5.clone()
        }
    }
}
fn decimal_of_11(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = v0.trim().parse::<u64>().unwrap_or(0);
    v1
}
fn query_options_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: US1, mut v4: Rc<str>, mut v5: bool, mut v6: Rc<str>, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: bool, mut v13: u64, mut v14: u64) -> US0 {
    loop {
        let mut v15: US2 = item_at_9(v0.clone(), v1, v2);
        match &v15 {
            US2::US2_Bare(v20) => {
                let mut v20: Rc<str> = v20.clone();
                let mut v21: i32 = v2.wrapping_add(1i32);
                let mut v22: Rc<str> = value_at_10(v0.clone(), v1, v21);
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in|context|first|include|exclude|timeout"); } LIT.with(|lit| lit.clone()) };
                let mut v24: bool = v23.split("|").any(|item| item == &*v20);
                let mut v25: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("literal"); } LIT.with(|lit| lit.clone()) };
                if v25 {
                    let mut v26: bool = true;
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v21, v3.clone(), v4.clone(), v26, v6.clone(), v7, v8, v9, v10.clone(), v11.clone(), v12, v13, v14);
                    continue;
                } else {
                    let mut v28: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths"); } LIT.with(|lit| lit.clone()) };
                    if v28 {
                        let mut v29: bool = true;
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v21, v3.clone(), v4.clone(), v5, v6.clone(), v7, v8, v9, v10.clone(), v11.clone(), v29, v13, v14);
                        continue;
                    } else {
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("around|per|file|~|with|git|status"); } LIT.with(|lit| lit.clone()) };
                        let mut v32: bool = v31.split("|").any(|item| item == &*v20);
                        if v32 {
                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v21, v3.clone(), v4.clone(), v5, v6.clone(), v7, v8, v9, v10.clone(), v11.clone(), v12, v13, v14);
                            continue;
                        } else {
                            let mut v35: bool = if v24 {
                                let mut v34: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v34
                            } else {
                                false
                            };
                            if v35 {
                                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": expected a value after it"); } LIT.with(|lit| lit.clone()) };
                                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v36, &*v37, &*v37, &*v37].concat());
                                return US0::US0_QueryRejected(v38.clone());
                            } else {
                                let mut v40: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in"); } LIT.with(|lit| lit.clone()) };
                                if v40 {
                                    let mut v41: i32 = v2.wrapping_add(2i32);
                                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v41, v3.clone(), v4.clone(), v5, v22.clone(), v7, v8, v9, v10.clone(), v11.clone(), v12, v13, v14);
                                    continue;
                                } else {
                                    let mut v43: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("context"); } LIT.with(|lit| lit.clone()) };
                                    if v43 {
                                        let mut v44: i32 = v2.wrapping_add(2i32);
                                        let mut v45: u64 = v22.split(char::from(44u8)).next().and_then(|value| value.trim().parse::<u64>().ok()).unwrap_or(0);
                                        let mut v46: u64 = v22.rsplit(char::from(44u8)).next().and_then(|value| value.trim().parse::<u64>().ok()).unwrap_or(0);
                                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v44, v3.clone(), v4.clone(), v5, v6.clone(), v45, v46, v9, v10.clone(), v11.clone(), v12, v13, v14);
                                        continue;
                                    } else {
                                        let mut v48: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("first"); } LIT.with(|lit| lit.clone()) };
                                        if v48 {
                                            let mut v49: i32 = v2.wrapping_add(2i32);
                                            let mut v50: u64 = decimal_of_11(v22.clone());
                                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v49, v3.clone(), v4.clone(), v5, v6.clone(), v7, v8, v50, v10.clone(), v11.clone(), v12, v13, v14);
                                            continue;
                                        } else {
                                            let mut v52: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("include"); } LIT.with(|lit| lit.clone()) };
                                            if v52 {
                                                let mut v53: i32 = v2.wrapping_add(2i32);
                                                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v53, v3.clone(), v4.clone(), v5, v6.clone(), v7, v8, v9, v22.clone(), v11.clone(), v12, v13, v14);
                                                continue;
                                            } else {
                                                let mut v55: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exclude"); } LIT.with(|lit| lit.clone()) };
                                                if v55 {
                                                    let mut v56: i32 = v2.wrapping_add(2i32);
                                                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v56, v3.clone(), v4.clone(), v5, v6.clone(), v7, v8, v9, v10.clone(), v22.clone(), v12, v13, v14);
                                                    continue;
                                                } else {
                                                    let mut v58: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout"); } LIT.with(|lit| lit.clone()) };
                                                    if v58 {
                                                        let mut v59: i32 = v2.wrapping_add(2i32);
                                                        let mut v60: u64 = { let text = (&*v22).trim(); let (digits, scale) = if let Some(value) = text.strip_suffix("ms") { (value, 1) } else if let Some(value) = text.strip_suffix(char::from(115u8)) { (value, 1000) } else if let Some(value) = text.strip_suffix(char::from(109u8)) { (value, 60000) } else { (text, 1000) }; digits.trim().parse::<u64>().map(|value| value * scale).unwrap_or(0) };
                                                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v59, v3.clone(), v4.clone(), v5, v6.clone(), v7, v8, v9, v10.clone(), v11.clone(), v12, v13, v60);
                                                        continue;
                                                    } else {
                                                        let mut v62: bool = !v20.is_empty() && v20.bytes().all(|byte| byte.is_ascii_digit());
                                                        if v62 {
                                                            let mut v63: u64 = decimal_of_11(v20.clone());
                                                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v21, v3.clone(), v4.clone(), v5, v6.clone(), v7, v8, v9, v10.clone(), v11.clone(), v12, v63, v14);
                                                            continue;
                                                        } else {
                                                            let mut v65: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                            if v65 {
                                                                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v21, v3.clone(), v4.clone(), v5, v20.clone(), v7, v8, v9, v10.clone(), v11.clone(), v12, v13, v14);
                                                                continue;
                                                            } else {
                                                                let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected word: "); } LIT.with(|lit| lit.clone()) };
                                                                let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                let mut v69: Rc<str> = std::rc::Rc::<str>::from([&*v67, &*v20, &*v68, &*v68, &*v68].concat());
                                                                return US0::US0_QueryRejected(v69.clone());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            US2::US2_Finished => {
                return US0::US0_QueryParsed(v3.clone(), v4.clone(), v5, v6.clone(), v7, v8, v9, v10.clone(), v11.clone(), v12, v13, v14);
            }
            US2::US2_Text(v17) => {
                let mut v17: Rc<str> = v17.clone();
                let mut v18: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v0.clone(), v1, v18, v3.clone(), v17.clone(), v5, v6.clone(), v7, v8, v9, v10.clone(), v11.clone(), v12, v13, v14);
                continue;
            }
        }
    }
}
fn query_problem_12(mut v0: US1, mut v1: Rc<str>, mut v2: bool, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool, mut v10: u64, mut v11: u64) -> Rc<str> {
    match &v0 {
        US1::US1_Head => {
            let mut v30: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v30 {
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("head: expected a path or glob"); } LIT.with(|lit| lit.clone()) };
                v31.clone()
            } else {
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v32.clone()
            }
        }
        US1::US1_ListFiles => {
            let mut v26: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v26 {
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files: expected a path or glob"); } LIT.with(|lit| lit.clone()) };
                v27.clone()
            } else {
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v28.clone()
            }
        }
        US1::US1_ReadAround => {
            let mut v19: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v19 {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read: expected a path"); } LIT.with(|lit| lit.clone()) };
                v20.clone()
            } else {
                let mut v21: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v21 {
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read: expected around \"<text>\""); } LIT.with(|lit| lit.clone()) };
                    v22.clone()
                } else {
                    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v23.clone()
                }
            }
        }
        US1::US1_Search => {
            let mut v12: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v12 {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search: expected a quoted pattern"); } LIT.with(|lit| lit.clone()) };
                v13.clone()
            } else {
                let mut v14: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v14 {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search: expected in <path|glob>"); } LIT.with(|lit| lit.clone()) };
                    v15.clone()
                } else {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                }
            }
        }
        US1::US1_Tail => {
            let mut v34: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v34 {
                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail: expected a path or glob"); } LIT.with(|lit| lit.clone()) };
                v35.clone()
            } else {
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v36.clone()
            }
        }
        US1::US1_Wait => {
            let mut v38: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v38 {
                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: expected a file"); } LIT.with(|lit| lit.clone()) };
                v39.clone()
            } else {
                let mut v40: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v40 {
                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: expected ~ \"<regex>\""); } LIT.with(|lit| lit.clone()) };
                    v41.clone()
                } else {
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v42.clone()
                }
            }
        }
    }
}
fn query_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 2i32;
    let mut v3: i32 = 1i32;
    let mut v4: Rc<str> = field_at_5(v0.clone(), v1, v3);
    let mut v5: US1 = verb_of_6(v4.clone());
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (US1, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = opened_query_7(v5.clone());
    let mut v18: US0 = query_options_8(v0.clone(), v1, v2, v6.clone(), v7.clone(), v8, v9.clone(), v10, v11, v12, v13.clone(), v14.clone(), v15, v16, v17);
    match &v18 {
        US0::US0_QueryParsed(v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32) => {
            let mut v21: US1 = v21.clone();
            let mut v22: Rc<str> = v22.clone();
            let mut v23: bool = *v23;
            let mut v24: Rc<str> = v24.clone();
            let mut v25: u64 = *v25;
            let mut v26: u64 = *v26;
            let mut v27: u64 = *v27;
            let mut v28: Rc<str> = v28.clone();
            let mut v29: Rc<str> = v29.clone();
            let mut v30: bool = *v30;
            let mut v31: u64 = *v31;
            let mut v32: u64 = *v32;
            let mut v33: Rc<str> = query_problem_12(v21.clone(), v22.clone(), v23, v24.clone(), v25, v26, v27, v28.clone(), v29.clone(), v30, v31, v32);
            let mut v34: bool = v31 == 0u64;
            let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (US1, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = if v34 {
                (v21.clone(), v22.clone(), v23, v24.clone(), v25, v26, v27, v28.clone(), v29.clone(), v30, 10u64, v32)
            } else {
                (v21.clone(), v22.clone(), v23, v24.clone(), v25, v26, v27, v28.clone(), v29.clone(), v30, v31, v32)
            };
            let mut v47: bool = v33.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v47 {
                let mut v48: bool = v46 == 0u64;
                let (mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (US1, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = if v48 {
                    (v35.clone(), v36.clone(), v37, v38.clone(), v39, v40, v41, v42.clone(), v43.clone(), v44, v45, 60000u64)
                } else {
                    (v35.clone(), v36.clone(), v37, v38.clone(), v39, v40, v41, v42.clone(), v43.clone(), v44, v45, v46)
                };
                US0::US0_QueryParsed(v49.clone(), v50.clone(), v51, v52.clone(), v53, v54, v55, v56.clone(), v57.clone(), v58, v59, v60)
            } else {
                US0::US0_QueryRejected(v33.clone())
            }
        }
        US0::US0_QueryRejected(v19) => {
            let mut v19: Rc<str> = v19.clone();
            US0::US0_QueryRejected(v19.clone())
        }
    }
}
fn first_rejection_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: US0 = query_parsed_4(v0.clone(), v1);
            match &v4 {
                US0::US0_QueryParsed(v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17) => {
                    let mut v6: US1 = v6.clone();
                    let mut v7: Rc<str> = v7.clone();
                    let mut v8: bool = *v8;
                    let mut v9: Rc<str> = v9.clone();
                    let mut v10: u64 = *v10;
                    let mut v11: u64 = *v11;
                    let mut v12: u64 = *v12;
                    let mut v13: Rc<str> = v13.clone();
                    let mut v14: Rc<str> = v14.clone();
                    let mut v15: bool = *v15;
                    let mut v16: u64 = *v16;
                    let mut v17: u64 = *v17;
                    let mut v18: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v18);
                    continue;
                }
                US0::US0_QueryRejected(v5) => {
                    let mut v5: Rc<str> = v5.clone();
                    return v5.clone();
                }
            }
        } else {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v22.clone();
        }
    }
}
fn empty_query_tally_13() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn git_marked_15(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> bool {
    let mut v2: bool = v0.get(v1 as usize).map(|fields| fields.iter().any(|field| &**field == "wgit") && fields.iter().any(|field| &**field == "wstatus")).unwrap_or(false);
    v2
}
fn edge_lines_17(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v2: u64, mut v3: bool) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v4: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = { let root = std::path::Path::new(&*v0); let count = v2 as usize; let from_end = v3; let files = v1.iter().map(|path| (path.to_string(), std::fs::read(root.join(&**path)).ok().map(|bytes| String::from_utf8_lossy(&bytes).lines().map(str::to_owned).collect::<Vec<_>>()))).collect::<Vec<_>>(); let mut failed = false; let mut shown = Vec::new(); let path_width = files.iter().map(|file| file.0.len()).max().unwrap_or(0); for (path, lines) in &files { let Some(lines) = lines else { failed = true; shown.push(format!("{path}  unreadable")); continue }; let first = if from_end { lines.len().saturating_sub(count) } else { 0 }; let last = if from_end { lines.len() } else { count.min(lines.len()) }; if count == 1 { shown.push(format!("{:<path_width$}  {}", path, lines.get(first).map(String::as_str).unwrap_or("(empty)"))); continue; } shown.push(format!("{path} ({} {} lines)", if from_end { "last" } else { "first" }, count)); let width = last.to_string().len(); for index in first..last { if from_end { shown.push(lines[index].clone()); } else { shown.push(format!("{:<width$}  {}", index + 1, lines[index])); } } } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [files.len() as u64, 0, 0, 0], failed)) };
    v4.clone()
}
fn listing_18(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<std::rc::Rc<str>>>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v2: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = { let root = std::path::Path::new(&*v0); let rows = v1.iter().map(|path| (path.to_string(), std::fs::read(root.join(&**path)).ok().map(|bytes| (bytes.len() as u64, String::from_utf8_lossy(&bytes).lines().count() as u64)))).collect::<Vec<_>>(); let path_width = rows.iter().map(|row| row.0.len()).max().unwrap_or(0).max(4); let bytes_width = rows.iter().map(|row| row.1.map(|counts| counts.0.to_string().len()).unwrap_or(1)).max().unwrap_or(0).max(5); let mut shown = vec![format!("{:<path_width$}  {:<bytes_width$}  {}", "path", "bytes", "lines")]; let mut tally = [rows.len() as u64, 0, 0, 0]; let mut failed = false; for (path, counts) in &rows { match counts { Some((bytes, lines)) => { tally[1] += bytes; tally[2] += lines; shown.push(format!("{:<path_width$}  {:<bytes_width$}  {}", path, bytes, lines)); } None => { failed = true; shown.push(format!("{:<path_width$}  {:<bytes_width$}  unreadable", path, "-")); } } } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), tally, failed)) };
    v2.clone()
}
fn around_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v5: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let path = &*v1; let text = match std::fs::read(std::path::Path::new(&*v0).join(path)) { Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(), Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("{path}  unreadable: {error}")), [1, 0, 0, 0], true)) }; let lines = text.lines().collect::<Vec<_>>(); let found = lines.iter().enumerate().filter(|(_, line)| line.contains(&*v2)).map(|(index, _)| index).collect::<Vec<_>>(); if found.is_empty() { return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("{path}  no match for {:?}", &*v2)), [1, 0, 0, 0], true)); } let (before, after) = (v3 as usize, v4 as usize); let numbers = found.iter().map(|index| (index + 1).to_string()).collect::<Vec<_>>().join(","); let mut shown = vec![if found.len() == 1 { format!("{path}  match at line {numbers}") } else { format!("{path}  matches at lines {numbers}") }]; let last_line = found.iter().map(|index| (index + after).min(lines.len() - 1)).max().unwrap_or(0); let width = (last_line + 1).to_string().len(); let mut printed: Option<usize> = None; let mut count = 0u64; for index in &found { let start = index.saturating_sub(before).max(printed.map_or(0, |line| line + 1)); let end = (index + after).min(lines.len() - 1); for line in start..=end { let marker = if found.contains(&line) { char::from(62u8) } else { char::from(32u8) }; shown.push(format!("{:>width$}{} {}", line + 1, marker, lines[line])); count += 1; } printed = Some(printed.map_or(end, |line| line.max(end))); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [1, found.len() as u64, count, 0], false)) })();
    v5.clone()
}
fn git_hits_20(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v2: Rc<str>, mut v3: bool) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v4: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let source = if v3 { regex::escape(&*v2) } else { (&*v2).to_owned() }; let pattern = match regex::Regex::new(&source) { Ok(pattern) => pattern, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("search: invalid pattern: {error}")), [0, 0, 0, 0], true)) }; let root = std::path::PathBuf::from(&*v0); let state_of = |path: &str| { let full = root.join(path); let folder = full.parent().map(std::path::Path::to_path_buf).unwrap_or_else(|| root.clone()); let name = full.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(); match std::process::Command::new("git").current_dir(&folder).args(["status", "--porcelain", "--untracked-files=all", "--"]).arg(&name).stdin(std::process::Stdio::null()).output() { Ok(output) if output.status.success() => { let text = String::from_utf8_lossy(&output.stdout).into_owned(); if text.trim().is_empty() { "clean" } else if text.starts_with("??") { "untracked" } else { "modified" } } _ => "no repo" } }; let mut hits: Vec<(String, String, &str)> = Vec::new(); let mut matched = 0u64; for path in v1.clone().iter() { let text = String::from_utf8_lossy(&std::fs::read(root.join(&**path)).unwrap_or_default()).into_owned(); let found = text.lines().enumerate().filter(|(_, line)| pattern.is_match(line)).map(|(index, line)| (format!("{}:{}", path, index + 1), line.to_owned())).collect::<Vec<_>>(); if found.is_empty() { continue; } matched += 1; let state = state_of(path); hits.extend(found.into_iter().map(|(place, line)| (place, line, state))); } let place_width = hits.iter().map(|hit| hit.0.len()).max().unwrap_or(0); let text_width = hits.iter().map(|hit| hit.1.chars().count()).max().unwrap_or(0); let shown = hits.iter().map(|(place, line, state)| format!("{:<place_width$}  {:<text_width$}  {}", place, line, state)).collect::<Vec<_>>(); std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [v1.len() as u64, matched, hits.len() as u64, 0], false)) })();
    v4.clone()
}
fn searched_21(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: bool) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v8: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { use rayon::prelude::*; let source = if v3 { regex::escape(&*v2) } else { (&*v2).to_owned() }; let pattern = match regex::Regex::new(&source) { Ok(pattern) => pattern, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("search: invalid pattern: {error}")), [0, 0, 0, 0], true)) }; let root = std::path::PathBuf::from(&*v0); let paths = v1.iter().map(|path| path.to_string()).collect::<Vec<_>>(); let hits = paths.par_iter().filter_map(|path| { let bytes = std::fs::read(root.join(path)).ok()?; let text = String::from_utf8_lossy(&bytes).into_owned(); let lines = text.lines().map(str::to_owned).collect::<Vec<_>>(); let found = lines.iter().enumerate().filter(|(_, line)| pattern.is_match(line)).map(|(index, _)| index).collect::<Vec<_>>(); if found.is_empty() { None } else { Some((path.clone(), found, lines)) } }).collect::<Vec<_>>(); let (before, after, first) = (v4 as usize, v5 as usize, v6 as usize); let clip = |line: &str| if line.chars().count() > 240 { format!("{}...", line.chars().take(240).collect::<String>()) } else { line.to_owned() }; let mut shown = Vec::new(); let mut total = 0u64; for (path, found, lines) in &hits { total += found.len() as u64; let taken = if first > 0 { found.iter().copied().take(first).collect::<Vec<_>>() } else { found.clone() }; if v7 { for index in &taken { shown.push(format!("{}:{}", path, index + 1)); } continue; } shown.push(path.clone()); let last_line = taken.iter().map(|index| (index + after).min(lines.len() - 1)).max().unwrap_or(0); let width = (last_line + 1).to_string().len(); let mut printed: Option<usize> = None; for index in &taken { let start = index.saturating_sub(before).max(printed.map_or(0, |line| line + 1)); let end = (index + after).min(lines.len() - 1); for line in start..=end { let marker = if taken.contains(&line) { char::from(62u8) } else { char::from(32u8) }; shown.push(format!("{:>width$}{} {}", line + 1, marker, clip(&lines[line]))); } printed = Some(printed.map_or(end, |line| line.max(end))); } } if shown.len() > 4000 { let hidden = shown.len() - 4000; shown.truncate(4000); shown.push(format!("... {hidden} more lines (narrow the search or use first n per file)")); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [paths.len() as u64, hits.len() as u64, total, 0], false)) })();
    v8.clone()
}
fn waited_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v4: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let pattern = match regex::Regex::new(&*v2) { Ok(pattern) => pattern, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("wait: invalid pattern: {error}")), [1, 0, 0, 0], true)) }; let path = &*v1; let file = std::path::Path::new(&*v0).join(path); let started = std::time::Instant::now(); let timeout = v3; let limit = std::time::Duration::from_millis(timeout); let elapsed_text = |millis: u128| if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }; let timeout_text = if timeout % 1000 == 0 { format!("{}s", timeout / 1000) } else { format!("{timeout}ms") }; loop { let text = std::fs::read(&file).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).unwrap_or_default(); if let Some((index, line)) = text.lines().enumerate().find(|(_, line)| pattern.is_match(line)) { return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("wait {path}  matched {:?} at line {} after {}", line, index + 1, elapsed_text(started.elapsed().as_millis()))), [1, 0, 0, 0], false)); } if started.elapsed() >= limit { let lines = text.lines().collect::<Vec<_>>(); let last = lines.last().copied().unwrap_or("(empty)"); let report = [format!("wait {path}  no match for {:?} in {timeout_text}  lines {}  bytes {}", &*v2, lines.len(), text.len()), format!("{path} (last 1 line)"), last.to_owned()].join(&char::from(10u8).to_string()); return std::rc::Rc::new((std::rc::Rc::<str>::from(report), [1, 1, 0, 0], true)); } std::thread::sleep(std::time::Duration::from_millis(100)); } })();
    v4.clone()
}
fn files_matching_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let mut v4: std::rc::Rc<Vec<std::rc::Rc<str>>> = { fn wildcard(mask: &str, name: &str) -> bool { let parts = mask.split(char::from(42u8)).collect::<Vec<_>>(); if parts.len() == 1 { return mask == name; } let mut rest = name; if !rest.starts_with(parts[0]) { return false; } rest = &rest[parts[0].len()..]; for middle in &parts[1..parts.len() - 1] { match rest.find(middle) { Some(at) => rest = &rest[at + middle.len()..], None => return false } } rest.len() >= parts[parts.len() - 1].len() && rest.ends_with(parts[parts.len() - 1]) } fn glob_match(pattern: &[&str], path: &[&str]) -> bool { match pattern.split_first() { None => path.is_empty(), Some((head, rest)) if *head == "**" => (0..=path.len()).any(|skip| glob_match(rest, &path[skip..])), Some((head, rest)) => match path.split_first() { Some((name, tail)) => wildcard(head, name) && glob_match(rest, tail), None => false } } } fn segments(text: &str) -> Vec<String> { text.replace(char::from(92u8), "/").split(char::from(47u8)).filter(|segment| !segment.is_empty() && *segment != ".").map(str::to_owned).collect() } let root = std::path::Path::new(&*v0); let mut pattern = segments(&*v1); let star = char::from(42u8); let globbed = pattern.iter().any(|segment| segment.contains(star)); if !globbed { let joined = pattern.join("/"); if !root.join(&joined).is_dir() { return std::rc::Rc::new(vec![std::rc::Rc::<str>::from(joined)]); } pattern.push("**".to_owned()); } let base = pattern.iter().take_while(|segment| !segment.contains(star)).cloned().collect::<Vec<_>>(); let pattern_refs = pattern.iter().map(String::as_str).collect::<Vec<_>>(); let excluded = segments(&*v3); let excluded_refs = excluded.iter().map(String::as_str).collect::<Vec<_>>(); let masks = (&*v2).split(char::from(44u8)).map(str::trim).filter(|mask| !mask.is_empty()).collect::<Vec<_>>(); let skip = [".git", "target", "node_modules", "_build", "deps"]; let mut found = Vec::new(); let mut stack = vec![base.clone()]; while let Some(folder) = stack.pop() { let Ok(entries) = std::fs::read_dir(root.join(folder.join("/"))) else { continue }; for entry in entries.filter_map(|entry| entry.ok()) { let Ok(kind) = entry.file_type() else { continue }; if kind.is_symlink() { continue; } let name = entry.file_name().to_string_lossy().into_owned(); let mut relative = folder.clone(); relative.push(name.clone()); if kind.is_dir() { if !skip.contains(&name.as_str()) { stack.push(relative); } continue; } let relative_refs = relative.iter().map(String::as_str).collect::<Vec<_>>(); if glob_match(&pattern_refs, &relative_refs) && (masks.is_empty() || masks.iter().any(|mask| wildcard(mask, &name))) && (excluded_refs.is_empty() || !glob_match(&excluded_refs, &relative_refs)) { found.push(relative.join("/")); } } } found.sort(); std::rc::Rc::new(found.into_iter().map(std::rc::Rc::<str>::from).collect::<Vec<_>>()) };
    v4.clone()
}
fn touched_with_24(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>>, mut v1: std::rc::Rc<Vec<std::rc::Rc<str>>>) -> () {
    v0.borrow_mut().extend(v1.iter().cloned());
    ()
}
fn query_step_16(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>>, mut v2: US1, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: bool, mut v12: u64, mut v13: u64, mut v14: bool) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v15: std::rc::Rc<Vec<std::rc::Rc<str>>> = files_matching_23(v0.clone(), v5.clone(), v9.clone(), v10.clone());
    touched_with_24(v1.clone(), v15.clone());
    match &v2 {
        US1::US1_Head => {
            let mut v21: bool = false;
            edge_lines_17(v0.clone(), v15.clone(), v12, v21)
        }
        US1::US1_ListFiles => {
            listing_18(v0.clone(), v15.clone())
        }
        US1::US1_ReadAround => {
            around_19(v0.clone(), v5.clone(), v3.clone(), v6, v7)
        }
        US1::US1_Search => {
            if v14 {
                git_hits_20(v0.clone(), v15.clone(), v3.clone(), v4)
            } else {
                searched_21(v0.clone(), v15.clone(), v3.clone(), v4, v6, v7, v8, v11)
            }
        }
        US1::US1_Tail => {
            let mut v23: bool = true;
            edge_lines_17(v0.clone(), v15.clone(), v12, v23)
        }
        US1::US1_Wait => {
            waited_22(v0.clone(), v5.clone(), v3.clone(), v13)
        }
    }
}
fn lines_joined_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v0.clone()
    } else {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            v1.clone()
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v1, &*v5, &*v5].concat());
            v6.clone()
        }
    }
}
fn outcome_merged_25(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: US1, mut v13: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)>) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v14: Rc<str> = v13.0.clone();
    let mut v15: Rc<str> = lines_joined_26(v0.clone(), v14.clone());
    let mut v17: bool = if v1 {
        true
    } else {
        let mut v16: bool = v13.clone().2;
        v16
    };
    let mut v18: u64 = 0u64;
    let mut v19: u64 = v13.1[v18 as usize];
    let mut v20: u64 = 1u64;
    let mut v21: u64 = v13.1[v20 as usize];
    let mut v22: u64 = 2u64;
    let mut v23: u64 = v13.1[v22 as usize];
    let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v12 {
        US1::US1_Search => {
            let mut v24: u64 = v2.wrapping_add(v19);
            let mut v25: u64 = v3.wrapping_add(v21);
            let mut v26: u64 = v4.wrapping_add(v23);
            let mut v27: u64 = v7.wrapping_add(1u64);
            (v15.clone(), v17, v24, v25, v26, v5, v6, v27, v8, v9, v10, v11)
        }
        _ => {
            let (mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v12 {
                US1::US1_ReadAround => {
                    let mut v28: u64 = v4.wrapping_add(v21);
                    let mut v29: u64 = v5.wrapping_add(v23);
                    let mut v30: u64 = v8.wrapping_add(1u64);
                    (v15.clone(), v17, v2, v3, v28, v29, v6, v7, v30, v9, v10, v11)
                }
                _ => {
                    let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v12 {
                        US1::US1_ListFiles => {
                            let mut v31: u64 = v6.wrapping_add(v21);
                            let mut v32: u64 = v9.wrapping_add(1u64);
                            (v15.clone(), v17, v2, v3, v4, v5, v31, v7, v8, v32, v10, v11)
                        }
                        _ => {
                            let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v12 {
                                US1::US1_Wait => {
                                    let mut v33: u64 = v10.wrapping_add(1u64);
                                    let mut v34: u64 = v11.wrapping_add(v21);
                                    (v15.clone(), v17, v2, v3, v4, v5, v6, v7, v8, v9, v33, v34)
                                }
                                _ => {
                                    (v15.clone(), v17, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
                                }
                            };
                            (v35.clone(), v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46)
                        }
                    };
                    (v47.clone(), v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58)
                }
            };
            (v59.clone(), v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70)
        }
    };
    (v71.clone(), v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82)
}
fn queries_run_14(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>>, mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v3: i32, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v16: i32 = v2.len() as i32;
    let mut v17: bool = v3 < v16;
    let (mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let mut v18: US0 = query_parsed_4(v2.clone(), v3);
        let (mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
            US0::US0_QueryParsed(v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) => {
                let mut v19: US1 = v19.clone();
                let mut v20: Rc<str> = v20.clone();
                let mut v21: bool = *v21;
                let mut v22: Rc<str> = v22.clone();
                let mut v23: u64 = *v23;
                let mut v24: u64 = *v24;
                let mut v25: u64 = *v25;
                let mut v26: Rc<str> = v26.clone();
                let mut v27: Rc<str> = v27.clone();
                let mut v28: bool = *v28;
                let mut v29: u64 = *v29;
                let mut v30: u64 = *v30;
                let mut v31: i32 = v3.wrapping_add(1i32);
                let mut v32: bool = git_marked_15(v2.clone(), v3);
                let mut v33: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = query_step_16(v0.clone(), v1.clone(), v19.clone(), v20.clone(), v21, v22.clone(), v23, v24, v25, v26.clone(), v27.clone(), v28, v29, v30, v32);
                let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = outcome_merged_25(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v19.clone(), v33.clone());
                queries_run_14(v0.clone(), v1.clone(), v2.clone(), v31, v34.clone(), v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45)
            }
            _ => {
                let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
                    US0::US0_QueryRejected(v58) => {
                        let mut v58: Rc<str> = v58.clone();
                        let mut v59: i32 = v3.wrapping_add(1i32);
                        queries_run_14(v0.clone(), v1.clone(), v2.clone(), v59, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
                    }
                    _ => unreachable!(),
                };
                (v72.clone(), v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83)
            }
        };
        (v84.clone(), v85, v86, v87, v88, v89, v90, v91, v92, v93, v94, v95)
    } else {
        (v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    };
    (v96.clone(), v97, v98, v99, v100, v101, v102, v103, v104, v105, v106, v107)
}
fn decimal_text_28(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn receipt_counts_27(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> Rc<str> {
    let mut v13: bool = v7 > 0u64;
    let mut v27: Rc<str> = if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = decimal_text_28(v2);
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" searched  "); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = decimal_text_28(v3);
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched  matches "); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = decimal_text_28(v4);
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v20, &*v20].concat());
        let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v16, &*v17, &*v21].concat());
        v22.clone()
    } else {
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = decimal_text_28(v12);
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v25, &*v25, &*v25].concat());
        v26.clone()
    };
    let mut v28: bool = v7 == 0u64;
    let mut v30: bool = if v28 {
        let mut v29: bool = v8 > 0u64;
        v29
    } else {
        false
    };
    let mut v35: Rc<str> = if v30 {
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  matches "); } LIT.with(|lit| lit.clone()) };
        let mut v32: Rc<str> = decimal_text_28(v4);
        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v31, &*v32, &*v33, &*v33].concat());
        v34.clone()
    } else {
        v27.clone()
    };
    let mut v36: bool = v8 > 0u64;
    let mut v41: Rc<str> = if v36 {
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) };
        let mut v38: Rc<str> = decimal_text_28(v5);
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v37, &*v38, &*v39, &*v39].concat());
        v40.clone()
    } else {
        v35.clone()
    };
    let mut v42: bool = v9 > 0u64;
    let mut v47: Rc<str> = if v42 {
        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  bytes "); } LIT.with(|lit| lit.clone()) };
        let mut v44: Rc<str> = decimal_text_28(v6);
        let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v43, &*v44, &*v45, &*v45].concat());
        v46.clone()
    } else {
        v41.clone()
    };
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waits "); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = decimal_text_28(v10);
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v49, &*v50, &*v50, &*v50].concat());
    let mut v52: bool = v10 == 0u64;
    if v52 {
        v47.clone()
    } else {
        let mut v53: bool = v11 == 0u64;
        if v53 {
            v51.clone()
        } else {
            let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  timed out "); } LIT.with(|lit| lit.clone()) };
            let mut v55: Rc<str> = decimal_text_28(v11);
            let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit 3"); } LIT.with(|lit| lit.clone()) };
            let mut v57: Rc<str> = std::rc::Rc::<str>::from([&*v51, &*v54, &*v55, &*v56, &*v50].concat());
            v57.clone()
        }
    }
}
fn elapsed_text_30(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_32() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_31() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_32();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_29(mut v0: US3, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US3::US3_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US3::US3_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = elapsed_text_30(v2);
    let mut v9: Rc<str> = clock_text_31();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn read_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_1();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1.clone());
    let mut v4: bool = v3.iter().skip(1).any(|fields| fields.first().map(|kind| !kind.is_empty() && &**kind != "query").unwrap_or(false));
    let mut v8: Rc<str> = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit and query statements cannot share one program yet; run them as two calls"); } LIT.with(|lit| lit.clone()) };
        v5.clone()
    } else {
        let mut v6: i32 = 1i32;
        first_rejection_3(v3.clone(), v6)
    };
    let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v9 {
        let mut v10: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeSet::new()));
        let mut v11: i32 = 1i32;
        let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_query_tally_13();
        let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = queries_run_14(v0.clone(), v10.clone(), v3.clone(), v11, v12.clone(), v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23);
        let mut v36: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v36 {
            ()
        } else {
            println!("{}", &*v24);
            ()
        };
        let mut v37: u64 = v10.borrow().len() as u64;
        let mut v38: Rc<str> = receipt_counts_27(v24.clone(), v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v37);
        if v25 {
            let mut v39: US3 = US3::US3_Failed;
            receipt_line_29(v39.clone(), v38.clone(), v2);
            let mut v40: bool = v35 > 0u64;
            if v40 {
                3i32
            } else {
                1i32
            }
        } else {
            let mut v42: US3 = US3::US3_Succeeded;
            receipt_line_29(v42.clone(), v38.clone(), v2);
            0i32
        }
    } else {
        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v8, &*v45, &*v45, &*v45].concat());
        println!("{}", &*v46);
        let mut v47: US3 = US3::US3_Failed;
        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_29(v47.clone(), v48.clone(), v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        read_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn matching_text_33(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: std::rc::Rc<Vec<std::rc::Rc<str>>> = files_matching_23(v0.clone(), v1.clone(), v2.clone(), v2.clone());
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(v3.iter().map(|path| path.to_string()).collect::<Vec<_>>().join("
"));
    v4.clone()
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        matching_text_33(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v0.clone());
        let mut v2: i32 = 1i32;
        first_rejection_3(v1.clone(), v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn report_text_34(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1.clone());
    let mut v3: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeSet::new()));
    let mut v4: i32 = 1i32;
    let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_query_tally_13();
    let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = queries_run_14(v0.clone(), v3.clone(), v2.clone(), v4, v5.clone(), v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16);
    let mut v29: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
    let mut v32: Rc<str> = if v18 {
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v30.clone()
    } else {
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v31.clone()
    };
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v29, &*v32, &*v33, &*v33].concat());
    v34.clone()
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        report_text_34(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_read_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_do_read_matching(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_do_read_rejection(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_do_read_report(v0: &str, v1: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
