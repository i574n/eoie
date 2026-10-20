#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
#[derive(Clone)]
enum US2 {
    US2_StateWritten,
    US2_StateChangedSinceRead,
    US2_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Replaced,
    US3_TemporaryWriteFailed,
    US3_RenameFailed,
    US3_ReadbackMismatch,
}
fn field_end_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US1::US1_ScanPlain => {
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
                let mut v25: US1 = match &v3 {
                    US1::US1_ScanEscaped => {
                        US1::US1_ScanQuoted
                    }
                    US1::US1_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US1::US1_ScanQuoted
                        } else {
                            US1::US1_ScanPlain
                        }
                    }
                    US1::US1_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US1::US1_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US1::US1_ScanPlain
                            } else {
                                US1::US1_ScanQuoted
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
fn skip_spaces_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US1 = US1::US1_ScanPlain;
            let mut v6: i32 = field_end_4(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_5(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn unescaped_from_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
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
fn unquoted_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = v0.len() as i32;
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = v3.len() as i32;
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_7(v3.clone(), v4, v5, v6, v7.clone())
}
fn note_at_2(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = v2.len() as i32;
    let mut v4: i32 = v1 + v3;
    let mut v5: i32 = v0.len() as i32;
    let mut v6: i32 = 2i32;
    let mut v7: i32 = field_start_3(v0.clone(), v4, v5, v6);
    let mut v8: US1 = US1::US1_ScanPlain;
    let mut v9: i32 = field_end_4(v0.clone(), v7, v5, v8.clone());
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(v7 as usize..v9 as usize).unwrap_or(""));
    let mut v11: Rc<str> = unquoted_6(v10.clone());
    let mut v12: i32 = v2.len() as i32;
    let mut v13: i32 = v1 + v12;
    let mut v14: i32 = v0.len() as i32;
    let mut v15: i32 = 3i32;
    let mut v16: i32 = field_start_3(v0.clone(), v13, v14, v15);
    let mut v17: US1 = US1::US1_ScanPlain;
    let mut v18: i32 = field_end_4(v0.clone(), v16, v14, v17.clone());
    let mut v19: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v18 as usize).unwrap_or(""));
    let mut v20: Rc<str> = unquoted_6(v19.clone());
    let mut v21: i32 = v2.len() as i32;
    let mut v22: i32 = v1 + v21;
    let mut v23: i32 = v0.len() as i32;
    let mut v24: i32 = 4i32;
    let mut v25: i32 = field_start_3(v0.clone(), v22, v23, v24);
    let mut v26: US1 = US1::US1_ScanPlain;
    let mut v27: i32 = field_end_4(v0.clone(), v25, v23, v26.clone());
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let mut v29: Rc<str> = unquoted_6(v28.clone());
    (v11.clone(), v20.clone(), v29.clone())
}
fn note_row_8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v7, &v8));
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v10, &*v8, &*v11, &*v12].concat());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v16, &v17));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v18.replace(&*v15, &v19));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v20.replace(&*v21, &v22));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v23.replace(&*v24, &v25));
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v26.replace(&*v27, &v28));
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v29, &*v15, &*v12, &*v12].concat());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v13, &*v14, &*v30, &*v31].concat());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v16, &v17));
    let mut v36: Rc<str> = std::rc::Rc::<str>::from(v35.replace(&*v15, &v19));
    let mut v37: Rc<str> = std::rc::Rc::<str>::from(v36.replace(&*v21, &v22));
    let mut v38: Rc<str> = std::rc::Rc::<str>::from(v37.replace(&*v24, &v25));
    let mut v39: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v27, &v28));
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v39, &*v15, &*v12, &*v12].concat());
    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v16, &v17));
    let mut v42: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v15, &v19));
    let mut v43: Rc<str> = std::rc::Rc::<str>::from(v42.replace(&*v21, &v22));
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v43.replace(&*v24, &v25));
    let mut v45: Rc<str> = std::rc::Rc::<str>::from(v44.replace(&*v27, &v28));
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v45, &*v15, &*v12, &*v12].concat());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v40, &*v31, &*v46].concat());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v16, &v17));
    let mut v49: Rc<str> = std::rc::Rc::<str>::from(v48.replace(&*v15, &v19));
    let mut v50: Rc<str> = std::rc::Rc::<str>::from(v49.replace(&*v21, &v22));
    let mut v51: Rc<str> = std::rc::Rc::<str>::from(v50.replace(&*v24, &v25));
    let mut v52: Rc<str> = std::rc::Rc::<str>::from(v51.replace(&*v27, &v28));
    let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v52, &*v15, &*v12, &*v12].concat());
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v47, &*v31, &*v53, &*v54].concat());
    v55.clone()
}
fn state_written_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US2 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    std::env::var("EOIE_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v4, &*v4, &*v4].concat());
        let mut v9: bool = std::fs::write(&*v8, &*v2).is_ok();
        let mut v18: US3 = if v9 {
            let mut v10: bool = std::fs::rename(&*v8, &*v0).is_ok();
            if v10 {
                let mut v11: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v11 {
                    US3::US3_Replaced
                } else {
                    US3::US3_ReadbackMismatch
                }
            } else {
                US3::US3_RenameFailed
            }
        } else {
            US3::US3_TemporaryWriteFailed
        };
        match &v18 {
            US3::US3_Replaced => {
                US2::US2_StateWritten
            }
            _ => {
                let mut v25: Rc<str> = match &v18 {
                    US3::US3_ReadbackMismatch => {
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v22.clone()
                    }
                    US3::US3_RenameFailed => {
                        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v21.clone()
                    }
                    US3::US3_TemporaryWriteFailed => {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v20.clone()
                    }
                    _ => unreachable!(),
                };
                US2::US2_StateWriteFailed(v25.clone())
            }
        }
    } else {
        US2::US2_StateChangedSinceRead
    }
}
fn note_edited_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> i32 {
    let mut v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v5, &*v6, &*v6, &*v6].concat());
    let mut v8: bool = std::path::Path::new(&*v7).is_file();
    let mut v11: US0 = if v8 {
        US0::US0_FileReadable(v7.clone())
    } else {
        US0::US0_FileMissing(v7.clone())
    };
    match &v11 {
        US0::US0_FileMissing(v12) => {
            let mut v12: Rc<str> = v12.clone();
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v1, &*v15, &*v13, &*v6].concat());
            let mut v17: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v16 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v17
        }
        US0::US0_FileReadable(v18) => {
            let mut v18: Rc<str> = v18.clone();
            let mut v19: Rc<str> = std::fs::read_to_string(&*v18).unwrap_or_default().into();
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v22, &v23));
            let mut v25: Rc<str> = std::rc::Rc::<str>::from(v24.to_ascii_lowercase());
            let mut v26: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v25, &*v23, &*v26, &*v6].concat());
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v27, &*v28, &*v6, &*v6].concat());
            let mut v30: i32 = v19.find(&*v29).map(|offset| offset as i32).unwrap_or(-1);
            let mut v31: bool = v30 < 0i32;
            if v31 {
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no note with this number"); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v1, &*v34, &*v32, &*v6].concat());
                let mut v36: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v35 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v36
            } else {
                let mut v37: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v22, &v23));
                let mut v38: Rc<str> = std::rc::Rc::<str>::from(v37.to_ascii_lowercase());
                let mut v39: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v38, &*v23, &*v39, &*v6].concat());
                let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v40, &*v28, &*v6, &*v6].concat());
                let mut v42: i32 = v41.len() as i32;
                let mut v43: i32 = v30.wrapping_add(v42);
                let (mut v44, mut v45, mut v46): (Rc<str>, Rc<str>, Rc<str>) = note_at_2(v19.clone(), v43);
                let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v48: i32 = v19.get(v30 as usize..).and_then(|rest| rest.find(&*v47)).map(|found| (found + v30 as usize) as i32).unwrap_or(-1);
                let mut v49: bool = v48 < 0i32;
                let mut v51: i32 = if v49 {
                    let mut v50: i32 = v19.len() as i32;
                    v50
                } else {
                    v48
                };
                let mut v52: Rc<str> = note_row_8(v1.clone(), v2, v44.clone(), v45.clone(), v3.clone());
                let mut v53: Rc<str> = std::rc::Rc::<str>::from(v19.get(0i32 as usize..v30 as usize).unwrap_or(""));
                let mut v54: i32 = v19.len() as i32;
                let mut v55: Rc<str> = std::rc::Rc::<str>::from(v19.get(v51 as usize..v54 as usize).unwrap_or(""));
                let mut v56: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v52, &*v55, &*v6, &*v6].concat());
                let mut v57: US2 = state_written_9(v7.clone(), v19.clone(), v56.clone());
                match &v57 {
                    US2::US2_StateWritten => {
                        let mut v58: u64 = v19.len() as u64;
                        let mut v59: u64 = v56.len() as u64;
                        let mut v60: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
                        let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile note-edit ok id="); } LIT.with(|lit| lit.clone()) };
                        let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" note="); } LIT.with(|lit| lit.clone()) };
                        let mut v63: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                        let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) };
                        let mut v65: Rc<str> = std::rc::Rc::<str>::from([&*v61, &*v1, &*v62, &*v63, &*v64].concat());
                        let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" author="); } LIT.with(|lit| lit.clone()) };
                        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                        let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                        let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                        let mut v70: Rc<str> = std::rc::Rc::<str>::from(v45.replace(&*v68, &v69));
                        let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                        let mut v72: Rc<str> = std::rc::Rc::<str>::from(v70.replace(&*v67, &v71));
                        let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v74: Rc<str> = std::rc::Rc::<str>::from(v72.replace(&*v47, &v73));
                        let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                        let mut v77: Rc<str> = std::rc::Rc::<str>::from(v74.replace(&*v75, &v76));
                        let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                        let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                        let mut v80: Rc<str> = std::rc::Rc::<str>::from(v77.replace(&*v78, &v79));
                        let mut v81: Rc<str> = std::rc::Rc::<str>::from([&*v67, &*v80, &*v67, &*v6, &*v6].concat());
                        let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" text_bytes="); } LIT.with(|lit| lit.clone()) };
                        let mut v83: i32 = v46.len() as i32;
                        let mut v84: Rc<str> = std::rc::Rc::<str>::from((v83).to_string());
                        let mut v85: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v66, &*v81, &*v82, &*v84].concat());
                        let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->"); } LIT.with(|lit| lit.clone()) };
                        let mut v87: i32 = v3.len() as i32;
                        let mut v88: Rc<str> = std::rc::Rc::<str>::from((v87).to_string());
                        let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                        let mut v90: Rc<str> = std::rc::Rc::<str>::from((v58).to_string());
                        let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                        let mut v92: Rc<str> = std::rc::Rc::<str>::from((v59).to_string());
                        let mut v93: Rc<str> = std::rc::Rc::<str>::from([&*v89, &*v90, &*v91, &*v92, &*v6].concat());
                        let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                        let mut v95: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                        let mut v96: Rc<str> = std::rc::Rc::<str>::from([&*v93, &*v94, &*v95, &*v6, &*v6].concat());
                        let mut v97: Rc<str> = std::rc::Rc::<str>::from([&*v65, &*v85, &*v86, &*v88, &*v96].concat());
                        let mut v98: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v97.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v98
                    }
                    _ => {
                        let mut v102: Rc<str> = match &v57 {
                            US2::US2_StateChangedSinceRead => {
                                let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                v99.clone()
                            }
                            US2::US2_StateWriteFailed(v100) => {
                                let mut v100: Rc<str> = v100.clone();
                                v100.clone()
                            }
                            _ => unreachable!(),
                        };
                        let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                        let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v105: Rc<str> = std::rc::Rc::<str>::from([&*v103, &*v1, &*v104, &*v102, &*v6].concat());
                        let mut v106: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v105 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v106
                    }
                }
            }
        }
    }
}
fn note_edit_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 3i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: i32 = v3.trim().parse::<i32>().unwrap_or(-1);
    let mut v5: i32 = 4i32;
    let mut v6: Rc<str> = usize::try_from(v5).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v6.trim());
    let mut v8: bool = v4 < 1i32;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note number must be a positive integer"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v1, &*v11, &*v9, &*v12].concat());
        let mut v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v13 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v14
    } else {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v16: bool = v7 == v15 ;
        if v16 {
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v1, &*v19, &*v17, &*v15].concat());
            let mut v21: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v20 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v21
        } else {
            let mut v22: i32 = 1i32;
            let mut v23: Rc<str> = usize::try_from(v22).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            note_edited_1(v23.clone(), v1.clone(), v4, v7.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        note_edit_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_note_edit_run() -> i32 {
    closure0()()
}
