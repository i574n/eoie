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
    US2_TokenRendered(Rc<str>),
    US2_TokenRejected,
}
#[derive(Clone)]
enum US3 {
    US3_Replaced,
    US3_TemporaryWriteFailed,
    US3_RenameFailed,
    US3_ReadbackMismatch,
}
fn agile_edit_failure_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile edit "); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v1, &*v3, &*v0, &*v4].concat());
    let mut v6: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v5 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v6
}
fn argument_owned_2(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    v1.clone()
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
fn agile_edit_entry_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = argument_owned_2(v0);
    let mut v2: i32 = agile_field_rendering::eoie_agile_edit_field_position(&*v1);
    let mut v3: bool = v2 < 0i32;
    if v3 {
        let mut v4: Rc<str> = agile_field_rendering::eoie_agile_edit_field_rule(&*v1);
        let mut v5: i32 = 2i32;
        let mut v6: Rc<str> = argument_owned_2(v5);
        agile_edit_failure_1(v4.clone(), v6.clone())
    } else {
        let mut v8: i32 = 1i32;
        let mut v9: Rc<str> = argument_owned_2(v8);
        let mut v10: i32 = 2i32;
        let mut v11: Rc<str> = argument_owned_2(v10);
        let mut v12: i32 = 4i32;
        let mut v13: Rc<str> = argument_owned_2(v12);
        let mut v14: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v15, &*v16, &*v16, &*v16].concat());
        let mut v18: bool = std::path::Path::new(&*v17).is_file();
        let mut v21: US0 = if v18 {
            US0::US0_FileReadable(v17.clone())
        } else {
            US0::US0_FileMissing(v17.clone())
        };
        match &v21 {
            US0::US0_FileMissing(v22) => {
                let mut v22: Rc<str> = v22.clone();
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                agile_edit_failure_1(v23.clone(), v11.clone())
            }
            US0::US0_FileReadable(v25) => {
                let mut v25: Rc<str> = v25.clone();
                let mut v26: Rc<str> = std::fs::read_to_string(&*v25).unwrap_or_default().into();
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v11, &*v28, &*v16, &*v16].concat());
                let mut v30: i32 = v26.find(&*v29).map(|offset| offset as i32).unwrap_or(-1);
                let mut v31: bool = v30 < 0i32;
                if v31 {
                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                    agile_edit_failure_1(v32.clone(), v11.clone())
                } else {
                    let mut v34: i32 = v26.len() as i32;
                    let mut v35: i32 = v30 + 6i32;
                    let mut v36: i32 = field_start_3(v26.clone(), v35, v34, v2);
                    let mut v37: US1 = US1::US1_ScanPlain;
                    let mut v38: i32 = field_end_4(v26.clone(), v36, v34, v37.clone());
                    let mut v39: Rc<str> = std::rc::Rc::<str>::from(v26.get(v36 as usize..v38 as usize).unwrap_or(""));
                    let mut v40: Rc<str> = agile_field_rendering::eoie_agile_edit_field_token(&*v1, &*v39, &*v13);
                    let mut v41: bool = v40 == v16 ;
                    let mut v44: US2 = if v41 {
                        US2::US2_TokenRejected
                    } else {
                        US2::US2_TokenRendered(v40.clone())
                    };
                    match &v44 {
                        US2::US2_TokenRejected => {
                            let mut v45: Rc<str> = agile_field_rendering::eoie_agile_edit_field_rule(&*v1);
                            agile_edit_failure_1(v45.clone(), v11.clone())
                        }
                        US2::US2_TokenRendered(v47) => {
                            let mut v47: Rc<str> = v47.clone();
                            let mut v48: Rc<str> = std::rc::Rc::<str>::from(v26.get(0i32 as usize..v36 as usize).unwrap_or(""));
                            let mut v49: Rc<str> = std::rc::Rc::<str>::from(v26.get(v38 as usize..v34 as usize).unwrap_or(""));
                            let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v47, &*v49, &*v16, &*v16].concat());
                            let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                            let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v51, &*v16, &*v16, &*v16].concat());
                            let mut v53: bool = std::fs::write(&*v52, &*v50).is_ok();
                            let mut v62: US3 = if v53 {
                                let mut v54: bool = std::fs::rename(&*v52, &*v17).is_ok();
                                if v54 {
                                    let mut v55: bool = std::fs::read_to_string(&*v17).is_ok_and(|value| value == *v50);
                                    if v55 {
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
                            match &v62 {
                                US3::US3_Replaced => {
                                    let mut v63: u64 = v26.len() as u64;
                                    let mut v64: u64 = v50.len() as u64;
                                    let mut v65: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v14);
                                    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile edit ok id="); } LIT.with(|lit| lit.clone()) };
                                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" field="); } LIT.with(|lit| lit.clone()) };
                                    let mut v68: Rc<str> = std::rc::Rc::<str>::from([&*v66, &*v11, &*v67, &*v1, &*v16].concat());
                                    let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                    let mut v70: Rc<str> = std::rc::Rc::<str>::from((v63).to_string());
                                    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                    let mut v72: Rc<str> = std::rc::Rc::<str>::from((v64).to_string());
                                    let mut v73: Rc<str> = std::rc::Rc::<str>::from([&*v69, &*v70, &*v71, &*v72, &*v16].concat());
                                    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                    let mut v75: Rc<str> = std::rc::Rc::<str>::from((v65).to_string());
                                    let mut v76: Rc<str> = std::rc::Rc::<str>::from([&*v73, &*v74, &*v75, &*v16, &*v16].concat());
                                    let mut v77: Rc<str> = std::rc::Rc::<str>::from([&*v68, &*v76, &*v16, &*v16, &*v16].concat());
                                    let mut v78: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v77.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                    v78
                                }
                                _ => {
                                    let mut v84: Rc<str> = match &v62 {
                                        US3::US3_ReadbackMismatch => {
                                            let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                            v81.clone()
                                        }
                                        US3::US3_RenameFailed => {
                                            let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                            v80.clone()
                                        }
                                        US3::US3_TemporaryWriteFailed => {
                                            let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                            v79.clone()
                                        }
                                        _ => unreachable!(),
                                    };
                                    agile_edit_failure_1(v84.clone(), v11.clone())
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
        agile_edit_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_edit_run() -> i32 {
    closure0()()
}
