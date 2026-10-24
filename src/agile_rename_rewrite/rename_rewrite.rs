#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
fn unescaped_from_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v6, &*v7, &*v7, &*v7].concat());
            return v8.clone();
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v10: bool = v9 == 92i32;
            if v10 {
                let mut v11: i32 = v1.wrapping_add(1i32);
                let mut v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v15: bool = v14 == 110i32;
                let mut v24: Rc<str> = if v15 {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let mut v19: bool = v14 == 116i32;
                        if v19 {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21.clone()
                        }
                    }
                };
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v13, &*v24, &*v25, &*v25].concat());
                (v0, v1, v2, v3, v4) = (v0.clone(), v12, v12, v3, v26.clone());
                continue;
            } else {
                let mut v28: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v28, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn unquoted_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_8(v3.clone(), v4, v5, v6, v7.clone())
}
fn method10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method11(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn deps_renamed_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool) -> (Rc<str>, bool) {
    loop {
        let mut v7: bool = v2 <= v1;
        if v7 {
            return (v5.clone(), v6);
        } else {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v9: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v8)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
            let mut v10: bool = v9 < 0i32;
            let mut v11: i32 = if v10 {
                v2
            } else {
                v9
            };
            let mut v13: i32 = if v10 {
                v2
            } else {
                let mut v12: i32 = v9.wrapping_add(1i32);
                v12
            };
            let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v11 as usize).unwrap_or(""));
            let mut v15: i32 = (v14.clone().len() as i32);
            let mut v16: i32 = 0i32;
            let mut v17: i32 = method10(v14.clone(), v15, v16);
            let mut v18: i32 = v15.wrapping_sub(1i32);
            let mut v19: Rc<str> = string_slice(&v14.clone(), v17 as i64, v18 as i64);
            let mut v20: i32 = (v19.clone().len() as i32);
            let mut v21: i32 = method11(v19.clone(), v20);
            let mut v22: Rc<str> = string_slice(&v19.clone(), 0i32 as i64, v21 as i64);
            let mut v23: bool = v22.clone() == v3.clone();
            let mut v24: Rc<str> = if v23 {
                v4.clone()
            } else {
                v22.clone()
            };
            let mut v25: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = if v25 {
                v24.clone()
            } else {
                let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v8));
                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v24));
                v27.clone()
            };
            let mut v29: bool = v23 || v6;
            (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v13, v2, v3.clone(), v4.clone(), v28.clone(), v29);
            continue;
        }
    }
}
fn dependent_line_3(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let mut v9: i32 = (v0.clone().len() as i32);
    let mut v10: i32 = v1.wrapping_add(6i32);
    let mut v11: i32 = 0i32;
    let mut v12: i32 = field_start_4(v0.clone(), v10, v9, v11);
    let mut v13: US0 = US0::US0_ScanPlain;
    let mut v14: i32 = field_end_5(v0.clone(), v12, v9, v13.clone());
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: Rc<str> = unquoted_7(v15.clone());
    let mut v17: i32 = 1i32;
    let mut v18: i32 = field_start_4(v0.clone(), v10, v9, v17);
    let mut v19: US0 = US0::US0_ScanPlain;
    let mut v20: i32 = field_end_5(v0.clone(), v18, v9, v19.clone());
    let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v18 as usize..v20 as usize).unwrap_or(""));
    let mut v22: i32 = v21.find(&*v2).map(|offset| offset as i32).unwrap_or(-1);
    let mut v23: bool = v22 < 0i32;
    let mut v24: bool = v23 == false;
    let (mut v31, mut v32, mut v33, mut v34, mut v35): (i32, Rc<str>, i32, Rc<str>, i32) = if v24 {
        let mut v25: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = if v25 {
            v16.clone()
        } else {
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v26));
            let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v16));
            v28.clone()
        };
        let mut v30: i32 = v8.wrapping_add(1i32);
        (v4, v5.clone(), v6, v29.clone(), v30)
    } else {
        (v4, v5.clone(), v6, v7.clone(), v8)
    };
    let mut v36: i32 = 7i32;
    let mut v37: i32 = field_start_4(v0.clone(), v10, v9, v36);
    let mut v38: US0 = US0::US0_ScanPlain;
    let mut v39: i32 = field_end_5(v0.clone(), v37, v9, v38.clone());
    let mut v40: Rc<str> = std::rc::Rc::<str>::from(v0.get(v37 as usize..v39 as usize).unwrap_or(""));
    let mut v41: Rc<str> = unquoted_7(v40.clone());
    let mut v42: i32 = 0i32;
    let mut v43: i32 = (v41.clone().len() as i32);
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v45: bool = false;
    let (mut v46, mut v47): (Rc<str>, bool) = deps_renamed_9(v41.clone(), v42, v43, v2.clone(), v3.clone(), v44.clone(), v45);
    if v47 {
        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
        let mut v51: Rc<str> = std::rc::Rc::<str>::from(v46.replace(&*v49, &v50));
        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
        let mut v53: Rc<str> = std::rc::Rc::<str>::from(v51.replace(&*v48, &v52));
        let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
        let mut v56: Rc<str> = std::rc::Rc::<str>::from(v53.replace(&*v54, &v55));
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
        let mut v59: Rc<str> = std::rc::Rc::<str>::from(v56.replace(&*v57, &v58));
        let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
        let mut v62: Rc<str> = std::rc::Rc::<str>::from(v59.replace(&*v60, &v61));
        let mut v63: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v62, &*v48, &*v44, &*v44].concat());
        let mut v64: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v37 as usize).unwrap_or(""));
        let mut v65: Rc<str> = std::rc::Rc::<str>::from(v0.get(v39 as usize..v9 as usize).unwrap_or(""));
        let mut v66: Rc<str> = std::rc::Rc::<str>::from([&*v64, &*v63, &*v65, &*v44, &*v44].concat());
        let mut v67: bool = v32.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v71: Rc<str> = if v67 {
            v16.clone()
        } else {
            let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v68));
            let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v16));
            v70.clone()
        };
        let mut v72: i32 = v33.wrapping_add(1i32);
        (v66.clone(), v31, v71.clone(), v72, v34.clone(), v35)
    } else {
        (v0.clone(), v31, v32.clone(), v33, v34.clone(), v35)
    }
}
fn renamed_line_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v9: bool = v0.starts_with(&*v8);
    if v9 {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v1, &*v12, &*v13].concat());
        let mut v15: i32 = v0.find(&*v14).map(|offset| offset as i32).unwrap_or(-1);
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v11, &*v1, &*v12, &*v13].concat());
        let mut v18: i32 = v0.find(&*v17).map(|offset| offset as i32).unwrap_or(-1);
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v11, &*v1, &*v12, &*v13].concat());
        let mut v21: i32 = v0.find(&*v20).map(|offset| offset as i32).unwrap_or(-1);
        let mut v22: i32 = v3.wrapping_add(1i32);
        let mut v23: bool = v15 < 0i32;
        let mut v24: bool = v23 == false;
        if v24 {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v25, &v26));
            let mut v28: Rc<str> = std::rc::Rc::<str>::from(v27.to_ascii_lowercase());
            let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v1, &*v12, &*v13].concat());
            let mut v30: i32 = (v29.clone().len() as i32);
            let mut v31: i32 = v15.wrapping_add(v30);
            let mut v32: i32 = (v0.clone().len() as i32);
            let mut v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v32 as usize).unwrap_or(""));
            let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v28));
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
            let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v2, &*v12, &*v13].concat());
            let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
            let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v33));
            (v39.clone(), v22, v4.clone(), v5, v6.clone(), v7)
        } else {
            let mut v40: bool = v18 < 0i32;
            let mut v41: bool = v40 == false;
            if v41 {
                let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
                let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                let mut v45: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v43, &v44));
                let mut v46: Rc<str> = std::rc::Rc::<str>::from(v45.to_ascii_lowercase());
                let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v46, &*v13, &*v13, &*v13].concat());
                let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v11, &*v1, &*v12, &*v13].concat());
                let mut v49: i32 = (v48.clone().len() as i32);
                let mut v50: i32 = v18.wrapping_add(v49);
                let mut v51: i32 = (v0.clone().len() as i32);
                let mut v52: Rc<str> = std::rc::Rc::<str>::from(v0.get(v50 as usize..v51 as usize).unwrap_or(""));
                let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v47));
                let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
                let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
                let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v11, &*v2, &*v12, &*v13].concat());
                let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
                let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v52));
                (v58.clone(), v22, v4.clone(), v5, v6.clone(), v7)
            } else {
                let mut v59: bool = v21 < 0i32;
                let mut v60: bool = v59 == false;
                if v60 {
                    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v63: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v61, &v62));
                    let mut v64: Rc<str> = std::rc::Rc::<str>::from(v63.to_ascii_lowercase());
                    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
                    let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v64));
                    let mut v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v62));
                    let mut v68: i32 = v21 + 6i32;
                    let mut v69: i32 = (v0.clone().len() as i32);
                    let mut v70: i32 = 1i32;
                    let mut v71: i32 = field_start_4(v0.clone(), v68, v69, v70);
                    let mut v72: US0 = US0::US0_ScanPlain;
                    let mut v73: i32 = field_end_5(v0.clone(), v71, v69, v72.clone());
                    let mut v74: Rc<str> = std::rc::Rc::<str>::from(v0.get(v71 as usize..v73 as usize).unwrap_or(""));
                    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                    let mut v76: Rc<str> = Rc::<str>::from(v74.replace(&*v75, &*v13));
                    let mut v77: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v76));
                    let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v11, &*v1, &*v12, &*v13].concat());
                    let mut v79: i32 = (v78.clone().len() as i32);
                    let mut v80: i32 = v21.wrapping_add(v79);
                    let mut v81: Rc<str> = std::rc::Rc::<str>::from(v0.get(v80 as usize..v69 as usize).unwrap_or(""));
                    let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v77));
                    let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
                    let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v83));
                    let mut v85: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v11, &*v2, &*v12, &*v13].concat());
                    let mut v86: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v85));
                    let mut v87: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v81));
                    (v87.clone(), v22, v4.clone(), v5, v6.clone(), v7)
                } else {
                    let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                    let mut v89: i32 = v0.find(&*v88).map(|offset| offset as i32).unwrap_or(-1);
                    let mut v90: bool = v89 < 0i32;
                    if v90 {
                        (v0.clone(), v3, v4.clone(), v5, v6.clone(), v7)
                    } else {
                        dependent_line_3(v0.clone(), v89, v1.clone(), v2.clone(), v3, v4.clone(), v5, v6.clone(), v7)
                    }
                }
            }
        }
    } else {
        (v0.clone(), v3, v4.clone(), v5, v6.clone(), v7)
    }
}
fn renamed_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: i32, mut v9: Rc<str>, mut v10: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    loop {
        let mut v11: bool = v2 <= v1;
        if v11 {
            return (v5.clone(), v6, v7.clone(), v8, v9.clone(), v10);
        } else {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v13: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v12)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
            let mut v14: bool = v13 < 0i32;
            let mut v15: i32 = if v14 {
                v2
            } else {
                v13
            };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v15 as usize).unwrap_or(""));
            let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) = renamed_line_2(v16.clone(), v3.clone(), v4.clone(), v6, v7.clone(), v8, v9.clone(), v10);
            let mut v24: Rc<str> = if v14 {
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v23.clone()
            } else {
                v12.clone()
            };
            let mut v25: i32 = v15.wrapping_add(1i32);
            let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v17));
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v24));
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = (v0.clone(), v25, v2, v3.clone(), v4.clone(), v27.clone(), v18, v19.clone(), v20, v21.clone(), v22);
            continue;
        }
    }
}
fn renamed_text_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = (v0.clone().len() as i32);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let mut v8: i32 = 0i32;
    let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) = renamed_from_1(v0.clone(), v3, v4, v1.clone(), v2.clone(), v5.clone(), v6, v5.clone(), v7, v5.clone(), v8);
    (v9.clone(), v10, v12, v11.clone(), v14, v13.clone())
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
        renamed_text_0(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_renamed_text(v0: &str, v1: &str, v2: &str) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
