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
#[derive(Clone)]
enum US1 {
    US1_StateWritten,
    US1_StateChangedSinceRead,
    US1_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Replaced,
    US2_TemporaryWriteFailed,
    US2_RenameFailed,
    US2_ReadbackMismatch,
}
#[derive(Clone)]
enum US3 {
    US3_FileReadable(Rc<str>),
    US3_FileMissing(Rc<str>),
}
fn note_schema_ensured_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note ="); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
    let mut v3: bool = v2 < 0i32;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> task_planning"); } LIT.with(|lit| lit.clone()) };
        let mut v5: i32 = v0.find(&*v4).map(|offset| offset as i32).unwrap_or(-1);
        let mut v6: bool = v5 < 0i32;
        let mut v9: i32 = if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> agile_item"); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v0.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
            v8
        } else {
            v5
        };
        let mut v10: bool = v9 < 0i32;
        if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v0, &*v13, &*v13].concat());
            v14.clone()
        } else {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v16: i32 = v0.get(v9 as usize..).and_then(|rest| rest.find(&*v15)).map(|found| (found + v9 as usize) as i32).unwrap_or(-1);
            let mut v17: bool = v16 < 0i32;
            let mut v19: i32 = if v17 {
                let mut v18: i32 = (v0.clone().len() as i32);
                v18
            } else {
                v16
            };
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v21, &*v22, &*v22, &*v22].concat());
            let mut v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v19 as usize).unwrap_or(""));
            let mut v25: i32 = (v0.clone().len() as i32);
            let mut v26: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v25 as usize).unwrap_or(""));
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v23, &*v26, &*v22, &*v22].concat());
            v27.clone()
        }
    } else {
        v0.clone()
    }
}
fn declared_from_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let mut v9: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
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
fn method9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method10(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn declared_count_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = declared_from_3(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            let mut v7: i32 = v3.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v7);
            continue;
        }
    }
}
fn civil_of_days_13(mut v0: u64) -> (u64, u64, u64) {
    let mut v1: u64 = v0.wrapping_add(719468u64);
    let mut v2: u64 = v1.wrapping_div(146097u64);
    let mut v3: u64 = v2.wrapping_mul(146097u64);
    let mut v4: u64 = v1.wrapping_sub(v3);
    let mut v5: u64 = v4.wrapping_div(1460u64);
    let mut v6: u64 = v4.wrapping_sub(v5);
    let mut v7: u64 = v4.wrapping_div(36524u64);
    let mut v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let mut v10: u64 = v8.wrapping_sub(v9);
    let mut v11: u64 = v10.wrapping_div(365u64);
    let mut v12: u64 = 365u64.wrapping_mul(v11);
    let mut v13: u64 = v11.wrapping_div(4u64);
    let mut v14: u64 = v12.wrapping_add(v13);
    let mut v15: u64 = v11.wrapping_div(100u64);
    let mut v16: u64 = v14.wrapping_sub(v15);
    let mut v17: u64 = v4.wrapping_sub(v16);
    let mut v18: u64 = 5u64.wrapping_mul(v17);
    let mut v19: u64 = v18.wrapping_add(2u64);
    let mut v20: u64 = v19.wrapping_div(153u64);
    let mut v21: u64 = 153u64.wrapping_mul(v20);
    let mut v22: u64 = v21.wrapping_add(2u64);
    let mut v23: u64 = v22.wrapping_div(5u64);
    let mut v24: u64 = v17.wrapping_sub(v23);
    let mut v25: u64 = v24.wrapping_add(1u64);
    let mut v26: bool = v20 < 10u64;
    let mut v29: u64 = if v26 {
        let mut v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let mut v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let mut v30: u64 = v2.wrapping_mul(400u64);
    let mut v31: u64 = v11.wrapping_add(v30);
    let mut v32: bool = v29 < 3u64;
    let mut v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let mut v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_12(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_13(v2);
    let mut v6: u64 = v1.wrapping_rem(86400u64);
    let mut v7: u64 = v6.wrapping_div(60u64);
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v4 < 10u64;
    let mut v16: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v13, &*v13].concat());
        v14.clone()
    } else {
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15.clone()
    };
    let mut v17: bool = v5 < 10u64;
    let mut v23: Rc<str> = if v17 {
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v20, &*v20].concat());
        v21.clone()
    } else {
        let mut v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22.clone()
    };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v16, &*v9, &*v23].concat());
    let mut v25: u64 = v7.wrapping_div(60u64);
    let mut v26: bool = v25 < 10u64;
    let mut v32: Rc<str> = if v26 {
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v29, &*v29].concat());
        v30.clone()
    } else {
        let mut v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31.clone()
    };
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v34: u64 = v7.wrapping_rem(60u64);
    let mut v35: bool = v34 < 10u64;
    let mut v41: Rc<str> = if v35 {
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v37, &*v38, &*v38, &*v38].concat());
        v39.clone()
    } else {
        let mut v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40.clone()
    };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v33, &*v41, &*v42, &*v43].concat());
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v45, &*v44, &*v43, &*v43].concat());
    v46.clone()
}
fn last_declared_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = declared_from_3(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v4);
            continue;
        }
    }
}
fn notes_insertion_15(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v7: i32 = 0i32;
    let mut v8: i32 = declared_from_3(v0.clone(), v6.clone(), v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v11: i32 = 0i32;
    let mut v12: i32 = declared_from_3(v0.clone(), v10.clone(), v11);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v15: i32 = 0i32;
    let mut v16: i32 = -1i32;
    let mut v17: i32 = last_declared_16(v0.clone(), v14.clone(), v15, v16);
    let mut v18: bool = v12 <= v17;
    let mut v19: i32 = if v18 {
        v17
    } else {
        v12
    };
    let mut v20: bool = v8 <= v19;
    let mut v21: i32 = if v20 {
        v19
    } else {
        v8
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v23: i32 = v0.get(v21 as usize..).and_then(|rest| rest.find(&*v22)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
    let mut v24: bool = v23 < 0i32;
    if v24 {
        let mut v25: i32 = (v0.clone().len() as i32);
        v25
    } else {
        v23
    }
}
fn notes_appended_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: i32 = notes_insertion_15(v0.clone(), v1.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v2, &*v5, &*v5, &*v5].concat());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v3 as usize).unwrap_or(""));
    let mut v8: i32 = (v0.clone().len() as i32);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v8 as usize).unwrap_or(""));
    let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v6, &*v9, &*v5, &*v5].concat());
    v10.clone()
}
fn state_written_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    std::env::var("EOIE_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v8, &*v9, &*v4].concat());
        let mut v11: bool = std::fs::write(&*v10, &*v2).is_ok();
        let mut v20: US2 = if v11 {
            let mut v12: bool = std::fs::rename(&*v10, &*v0).is_ok();
            if v12 {
                let mut v13: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v13 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                US2::US2_RenameFailed
            }
        } else {
            US2::US2_TemporaryWriteFailed
        };
        match &v20 {
            US2::US2_Replaced => {
                US1::US1_StateWritten
            }
            _ => {
                let mut v27: Rc<str> = match &v20 {
                    US2::US2_ReadbackMismatch => {
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v24.clone()
                    }
                    US2::US2_RenameFailed => {
                        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v23.clone()
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v22.clone()
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v27.clone())
            }
        }
    } else {
        US1::US1_StateChangedSinceRead
    }
}
fn title_moved_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> i32 {
    let mut v4: Rc<str> = note_schema_ensured_2(v2.clone());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v6, &*v1, &*v7, &*v8].concat());
    let mut v10: i32 = 0i32;
    let mut v11: i32 = declared_from_3(v4.clone(), v9.clone(), v10);
    let mut v12: i32 = (v4.clone().len() as i32);
    let mut v13: i32 = v11 + 6i32;
    let mut v14: i32 = 1i32;
    let mut v15: i32 = field_start_4(v4.clone(), v13, v12, v14);
    let mut v16: US0 = US0::US0_ScanPlain;
    let mut v17: i32 = field_end_5(v4.clone(), v15, v12, v16.clone());
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v4.get(v15 as usize..v17 as usize).unwrap_or(""));
    let mut v19: Rc<str> = unquoted_7(v18.clone());
    let mut v20: i32 = agile_title_tails::eoie_agile_title_kept_end(&*v19);
    let mut v21: bool = v20 < 0i32;
    if v21 {
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the title has no \"; \" tail starting with a date or time to move"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile notes from-title "); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v1));
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v22));
        let mut v28: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v27 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v28
    } else {
        let mut v29: Rc<str> = std::rc::Rc::<str>::from(v19.get(0i32 as usize..v20 as usize).unwrap_or(""));
        let mut v30: i32 = (v29.clone().len() as i32);
        let mut v31: i32 = 0i32;
        let mut v32: i32 = method9(v29.clone(), v30, v31);
        let mut v33: i32 = v30.wrapping_sub(1i32);
        let mut v34: Rc<str> = string_slice(&v29.clone(), v32 as i64, v33 as i64);
        let mut v35: i32 = (v34.clone().len() as i32);
        let mut v36: i32 = method10(v34.clone(), v35);
        let mut v37: Rc<str> = string_slice(&v34.clone(), 0i32 as i64, v36 as i64);
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v6, &*v1, &*v7, &*v8].concat());
        let mut v40: i32 = 0i32;
        let mut v41: i32 = 0i32;
        let mut v42: i32 = declared_count_11(v4.clone(), v39.clone(), v40, v41);
        let mut v43: i32 = v42.wrapping_add(1i32);
        let mut v44: Rc<str> = utc_stamp_12(v3);
        let (mut v45, mut v46): (i32, Rc<str>) = agile_title_tails::eoie_agile_title_tail_rows(&*v19, &*v1, v43, &*v44);
        let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
        let mut v49: Rc<str> = std::rc::Rc::<str>::from(v37.replace(&*v47, &v48));
        let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
        let mut v51: Rc<str> = std::rc::Rc::<str>::from(v49.replace(&*v6, &v50));
        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
        let mut v54: Rc<str> = std::rc::Rc::<str>::from(v51.replace(&*v52, &v53));
        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
        let mut v57: Rc<str> = std::rc::Rc::<str>::from(v54.replace(&*v55, &v56));
        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
        let mut v60: Rc<str> = std::rc::Rc::<str>::from(v57.replace(&*v58, &v59));
        let mut v61: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v60, &*v6, &*v8, &*v8].concat());
        let mut v62: Rc<str> = std::rc::Rc::<str>::from(v4.get(0i32 as usize..v15 as usize).unwrap_or(""));
        let mut v63: Rc<str> = std::rc::Rc::<str>::from(v4.get(v17 as usize..v12 as usize).unwrap_or(""));
        let mut v64: Rc<str> = std::rc::Rc::<str>::from([&*v62, &*v61, &*v63, &*v8, &*v8].concat());
        let mut v65: Rc<str> = notes_appended_14(v64.clone(), v1.clone(), v46.clone());
        let mut v66: US1 = state_written_17(v0.clone(), v2.clone(), v65.clone());
        match &v66 {
            US1::US1_StateWritten => {
                let mut v67: i32 = (v19.clone().len() as i32);
                let mut v68: i32 = (v37.clone().len() as i32);
                let mut v69: u64 = v2.len() as u64;
                let mut v70: u64 = v65.len() as u64;
                let mut v71: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
                let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile notes ok id="); } LIT.with(|lit| lit.clone()) };
                let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v1));
                let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" moved="); } LIT.with(|lit| lit.clone()) };
                let mut v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
                let mut v76: Rc<str> = std::rc::Rc::<str>::from((v45).to_string());
                let mut v77: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v76));
                let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" title_bytes_before="); } LIT.with(|lit| lit.clone()) };
                let mut v79: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v78));
                let mut v80: Rc<str> = std::rc::Rc::<str>::from((v67).to_string());
                let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" title_bytes_after="); } LIT.with(|lit| lit.clone()) };
                let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
                let mut v83: Rc<str> = std::rc::Rc::<str>::from((v68).to_string());
                let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v83));
                let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v84));
                let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                let mut v87: Rc<str> = std::rc::Rc::<str>::from((v69).to_string());
                let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                let mut v89: Rc<str> = std::rc::Rc::<str>::from((v70).to_string());
                let mut v90: Rc<str> = std::rc::Rc::<str>::from([&*v86, &*v87, &*v88, &*v89, &*v8].concat());
                let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                let mut v92: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
                let mut v93: Rc<str> = std::rc::Rc::<str>::from([&*v90, &*v91, &*v92, &*v8, &*v8].concat());
                let mut v94: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v93));
                let mut v95: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v94.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                v95
            }
            _ => {
                let mut v99: Rc<str> = match &v66 {
                    US1::US1_StateChangedSinceRead => {
                        let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v96.clone()
                    }
                    US1::US1_StateWriteFailed(v97) => {
                        let mut v97: Rc<str> = v97.clone();
                        v97.clone()
                    }
                    _ => unreachable!(),
                };
                let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile notes from-title "); } LIT.with(|lit| lit.clone()) };
                let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v1));
                let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v103: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v102));
                let mut v104: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v99));
                let mut v105: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v104 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v105
            }
        }
    }
}
fn command_line_argument_count_19() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_18() -> i32 {
    let mut v0: i32 = command_line_argument_count_19();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn from_title_entry_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = argument_count_18();
    let mut v3: bool = v2 == 4i32;
    if v3 {
        let mut v4: i32 = 1i32;
        let mut v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from-title"); } LIT.with(|lit| lit.clone()) };
        if v6 {
            let mut v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
            let mut v8: i32 = 2i32;
            let mut v9: Rc<str> = usize::try_from(v8).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v11, &*v11].concat());
            let mut v13: bool = std::path::Path::new(&*v12).is_file();
            let mut v16: US3 = if v13 {
                US3::US3_FileReadable(v12.clone())
            } else {
                US3::US3_FileMissing(v12.clone())
            };
            match &v16 {
                US3::US3_FileMissing(v17) => {
                    let mut v17: Rc<str> = v17.clone();
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile notes from-title "); } LIT.with(|lit| lit.clone()) };
                    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v1));
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v18));
                    let mut v24: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v23 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v24
                }
                US3::US3_FileReadable(v25) => {
                    let mut v25: Rc<str> = v25.clone();
                    let mut v26: Rc<str> = std::fs::read_to_string(&*v25).unwrap_or_default().into();
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v1, &*v29, &*v11].concat());
                    let mut v31: i32 = 0i32;
                    let mut v32: i32 = declared_from_3(v26.clone(), v30.clone(), v31);
                    let mut v33: bool = v32 < 0i32;
                    let mut v34: bool = v33 == false;
                    if v34 {
                        title_moved_1(v12.clone(), v1.clone(), v26.clone(), v7)
                    } else {
                        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile notes from-title "); } LIT.with(|lit| lit.clone()) };
                        let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v1));
                        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
                        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v36));
                        let mut v42: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v41 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v42
                    }
                }
            }
        } else {
            let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile notes from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
            let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile notes from-title "); } LIT.with(|lit| lit.clone()) };
            let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v1));
            let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
            let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v46));
            let mut v52: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v51 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v52
        }
    } else {
        let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile notes from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile notes from-title "); } LIT.with(|lit| lit.clone()) };
        let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v1));
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
        let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v54));
        let mut v60: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v59 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v60
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        from_title_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_notes_from_title_run() -> i32 {
    closure0()()
}
