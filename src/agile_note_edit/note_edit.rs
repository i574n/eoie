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
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_7(v3.clone(), v4, v5, v6, v7.clone())
}
fn note_at_2(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1 + 6i32;
    let mut v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = 2i32;
    let mut v5: i32 = field_start_3(v0.clone(), v2, v3, v4);
    let mut v6: US1 = US1::US1_ScanPlain;
    let mut v7: i32 = field_end_4(v0.clone(), v5, v3, v6.clone());
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_6(v8.clone());
    let mut v10: i32 = v1 + 6i32;
    let mut v11: i32 = 3i32;
    let mut v12: i32 = field_start_3(v0.clone(), v10, v3, v11);
    let mut v13: US1 = US1::US1_ScanPlain;
    let mut v14: i32 = field_end_4(v0.clone(), v12, v3, v13.clone());
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: Rc<str> = unquoted_6(v15.clone());
    let mut v17: i32 = v1 + 6i32;
    let mut v18: i32 = 4i32;
    let mut v19: i32 = field_start_3(v0.clone(), v17, v3, v18);
    let mut v20: US1 = US1::US1_ScanPlain;
    let mut v21: i32 = field_end_4(v0.clone(), v19, v3, v20.clone());
    let mut v22: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v21 as usize).unwrap_or(""));
    let mut v23: Rc<str> = unquoted_6(v22.clone());
    (v9.clone(), v16.clone(), v23.clone())
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
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v8, &*v9, &*v4].concat());
        let mut v11: bool = std::fs::write(&*v10, &*v2).is_ok();
        let mut v20: US3 = if v11 {
            let mut v12: bool = std::fs::rename(&*v10, &*v0).is_ok();
            if v12 {
                let mut v13: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v13 {
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
        match &v20 {
            US3::US3_Replaced => {
                US2::US2_StateWritten
            }
            _ => {
                let mut v27: Rc<str> = match &v20 {
                    US3::US3_ReadbackMismatch => {
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v24.clone()
                    }
                    US3::US3_RenameFailed => {
                        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v23.clone()
                    }
                    US3::US3_TemporaryWriteFailed => {
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v22.clone()
                    }
                    _ => unreachable!(),
                };
                US2::US2_StateWriteFailed(v27.clone())
            }
        }
    } else {
        US2::US2_StateChangedSinceRead
    }
}
fn note_edited_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>) -> i32 {
    let mut v5: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v6, &*v7, &*v7, &*v7].concat());
    let mut v9: bool = std::path::Path::new(&*v8).is_file();
    let mut v12: US0 = if v9 {
        US0::US0_FileReadable(v8.clone())
    } else {
        US0::US0_FileMissing(v8.clone())
    };
    match &v12 {
        US0::US0_FileMissing(v13) => {
            let mut v13: Rc<str> = v13.clone();
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v1));
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v14));
            let mut v20: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v19 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v20
        }
        US0::US0_FileReadable(v21) => {
            let mut v21: Rc<str> = v21.clone();
            let mut v22: Rc<str> = std::fs::read_to_string(&*v21).unwrap_or_default().into();
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v24, &v25));
            let mut v27: Rc<str> = std::rc::Rc::<str>::from(v26.to_ascii_lowercase());
            let mut v28: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v27, &*v25, &*v28, &*v7].concat());
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v29));
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
            let mut v34: i32 = v22.find(&*v33).map(|offset| offset as i32).unwrap_or(-1);
            let mut v35: bool = v34 < 0i32;
            if v35 {
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no note with this number"); } LIT.with(|lit| lit.clone()) };
                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v1));
                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
                let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v36));
                let mut v42: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v41 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v42
            } else {
                let mut v43: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v24, &v25));
                let mut v44: Rc<str> = std::rc::Rc::<str>::from(v43.to_ascii_lowercase());
                let mut v45: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v44, &*v25, &*v45, &*v7].concat());
                let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v46));
                let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v32));
                let mut v49: i32 = (v48.clone().len() as i32);
                let mut v50: i32 = v34.wrapping_add(v49);
                let (mut v51, mut v52, mut v53): (Rc<str>, Rc<str>, Rc<str>) = note_at_2(v22.clone(), v50);
                let mut v54: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v56: bool = if v54 {
                    true
                } else {
                    let mut v55: bool = v52.clone() == v4.clone();
                    v55
                };
                if v56 {
                    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v58: i32 = v22.get(v34 as usize..).and_then(|rest| rest.find(&*v57)).map(|found| (found + v34 as usize) as i32).unwrap_or(-1);
                    let mut v59: bool = v58 < 0i32;
                    let mut v61: i32 = if v59 {
                        let mut v60: i32 = (v22.clone().len() as i32);
                        v60
                    } else {
                        v58
                    };
                    let mut v62: Rc<str> = note_row_8(v1.clone(), v2, v51.clone(), v52.clone(), v3.clone());
                    let mut v63: Rc<str> = std::rc::Rc::<str>::from(v22.get(0i32 as usize..v34 as usize).unwrap_or(""));
                    let mut v64: i32 = (v22.clone().len() as i32);
                    let mut v65: Rc<str> = std::rc::Rc::<str>::from(v22.get(v61 as usize..v64 as usize).unwrap_or(""));
                    let mut v66: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v62, &*v65, &*v7, &*v7].concat());
                    let mut v67: US2 = state_written_9(v8.clone(), v22.clone(), v66.clone());
                    match &v67 {
                        US2::US2_StateWritten => {
                            let mut v68: u64 = v22.len() as u64;
                            let mut v69: u64 = v66.len() as u64;
                            let mut v70: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v5);
                            let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile note-edit ok id="); } LIT.with(|lit| lit.clone()) };
                            let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v1));
                            let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" note="); } LIT.with(|lit| lit.clone()) };
                            let mut v74: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v73));
                            let mut v75: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let mut v76: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v75));
                            let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) };
                            let mut v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
                            let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" author="); } LIT.with(|lit| lit.clone()) };
                            let mut v80: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v79));
                            let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                            let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                            let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                            let mut v84: Rc<str> = std::rc::Rc::<str>::from(v52.replace(&*v82, &v83));
                            let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                            let mut v86: Rc<str> = std::rc::Rc::<str>::from(v84.replace(&*v81, &v85));
                            let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v88: Rc<str> = std::rc::Rc::<str>::from(v86.replace(&*v57, &v87));
                            let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                            let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                            let mut v91: Rc<str> = std::rc::Rc::<str>::from(v88.replace(&*v89, &v90));
                            let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v94: Rc<str> = std::rc::Rc::<str>::from(v91.replace(&*v92, &v93));
                            let mut v95: Rc<str> = std::rc::Rc::<str>::from([&*v81, &*v94, &*v81, &*v7, &*v7].concat());
                            let mut v96: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v95));
                            let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" text_bytes="); } LIT.with(|lit| lit.clone()) };
                            let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v97));
                            let mut v99: i32 = (v53.clone().len() as i32);
                            let mut v100: Rc<str> = std::rc::Rc::<str>::from((v99).to_string());
                            let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v100));
                            let mut v102: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v101));
                            let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->"); } LIT.with(|lit| lit.clone()) };
                            let mut v104: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v103));
                            let mut v105: i32 = (v3.clone().len() as i32);
                            let mut v106: Rc<str> = std::rc::Rc::<str>::from((v105).to_string());
                            let mut v107: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v106));
                            let mut v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                            let mut v109: Rc<str> = std::rc::Rc::<str>::from((v68).to_string());
                            let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                            let mut v111: Rc<str> = std::rc::Rc::<str>::from((v69).to_string());
                            let mut v112: Rc<str> = std::rc::Rc::<str>::from([&*v108, &*v109, &*v110, &*v111, &*v7].concat());
                            let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                            let mut v114: Rc<str> = std::rc::Rc::<str>::from((v70).to_string());
                            let mut v115: Rc<str> = std::rc::Rc::<str>::from([&*v112, &*v113, &*v114, &*v7, &*v7].concat());
                            let mut v116: Rc<str> = Rc::<str>::from(format!("{}{}", v107, v115));
                            let mut v117: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v116.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                            v117
                        }
                        _ => {
                            let mut v121: Rc<str> = match &v67 {
                                US2::US2_StateChangedSinceRead => {
                                    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                    v118.clone()
                                }
                                US2::US2_StateWriteFailed(v119) => {
                                    let mut v119: Rc<str> = v119.clone();
                                    v119.clone()
                                }
                                _ => unreachable!(),
                            };
                            let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                            let mut v123: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v1));
                            let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let mut v125: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v124));
                            let mut v126: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v121));
                            let mut v127: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v126 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v127
                        }
                    }
                } else {
                    let mut v129: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                    let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note "); } LIT.with(|lit| lit.clone()) };
                    let mut v131: Rc<str> = Rc::<str>::from(format!("{}{}", v130, v129));
                    let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" was written by "); } LIT.with(|lit| lit.clone()) };
                    let mut v133: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v132));
                    let mut v134: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v52));
                    let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; only its author edits it"); } LIT.with(|lit| lit.clone()) };
                    let mut v136: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v135));
                    let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                    let mut v138: Rc<str> = Rc::<str>::from(format!("{}{}", v137, v1));
                    let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v140: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v139));
                    let mut v141: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v136));
                    let mut v142: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v141 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v142
                }
            }
        }
    }
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
fn note_edit_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 3i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: i32 = v3.trim().parse::<i32>().unwrap_or(-1);
    let mut v5: i32 = 4i32;
    let mut v6: Rc<str> = usize::try_from(v5).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v7: i32 = (v6.clone().len() as i32);
    let mut v8: i32 = 0i32;
    let mut v9: i32 = method10(v6.clone(), v7, v8);
    let mut v10: i32 = v7.wrapping_sub(1i32);
    let mut v11: Rc<str> = string_slice(&v6.clone(), v9 as i64, v10 as i64);
    let mut v12: i32 = (v11.clone().len() as i32);
    let mut v13: i32 = method11(v11.clone(), v12);
    let mut v14: Rc<str> = string_slice(&v11.clone(), 0i32 as i64, v13 as i64);
    let mut v15: bool = v4 < 1i32;
    if v15 {
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note number must be a positive integer"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v1));
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
        let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v16));
        let mut v22: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v21 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v22
    } else {
        let mut v23: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v23 {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v1));
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v24));
            let mut v30: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v29 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v30
        } else {
            let mut v31: i32 = 5i32;
            let mut v32: Rc<str> = usize::try_from(v31).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v33: bool = v32.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v33 {
                let mut v34: i32 = 1i32;
                let mut v35: Rc<str> = usize::try_from(v34).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                note_edited_1(v35.clone(), v1.clone(), v4, v14.clone(), v36.clone())
            } else {
                let mut v38: i32 = 5i32;
                let mut v39: Rc<str> = usize::try_from(v38).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v40: bool = v39.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
                let mut v53: bool = if v40 {
                    let mut v41: i32 = 6i32;
                    let mut v42: Rc<str> = usize::try_from(v41).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v43: i32 = (v42.clone().len() as i32);
                    let mut v44: i32 = 0i32;
                    let mut v45: i32 = method10(v42.clone(), v43, v44);
                    let mut v46: i32 = v43.wrapping_sub(1i32);
                    let mut v47: Rc<str> = string_slice(&v42.clone(), v45 as i64, v46 as i64);
                    let mut v48: i32 = (v47.clone().len() as i32);
                    let mut v49: i32 = method11(v47.clone(), v48);
                    let mut v50: Rc<str> = string_slice(&v47.clone(), 0i32 as i64, v49 as i64);
                    let mut v51: bool = v50.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v52: bool = v51 == false;
                    v52
                } else {
                    false
                };
                if v53 {
                    let mut v54: i32 = 1i32;
                    let mut v55: Rc<str> = usize::try_from(v54).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v56: i32 = 6i32;
                    let mut v57: Rc<str> = usize::try_from(v56).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v58: i32 = (v57.clone().len() as i32);
                    let mut v59: i32 = 0i32;
                    let mut v60: i32 = method10(v57.clone(), v58, v59);
                    let mut v61: i32 = v58.wrapping_sub(1i32);
                    let mut v62: Rc<str> = string_slice(&v57.clone(), v60 as i64, v61 as i64);
                    let mut v63: i32 = (v62.clone().len() as i32);
                    let mut v64: i32 = method11(v62.clone(), v63);
                    let mut v65: Rc<str> = string_slice(&v62.clone(), 0i32 as i64, v64 as i64);
                    note_edited_1(v55.clone(), v1.clone(), v4, v14.clone(), v65.clone())
                } else {
                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile note-edit <root> <id> <note-number> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
                    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                    let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v1));
                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
                    let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v67));
                    let mut v73: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v72 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v73
                }
            }
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
