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
fn declared_from_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
fn binding_declared_3(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v1, &*v3, &*v4, &*v4].concat());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v1, &*v3, &*v4, &*v4].concat());
    let mut v8: bool = v0.starts_with(&*v7);
    if v8 {
        true
    } else {
        let mut v9: i32 = v0.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
        let mut v10: bool = v9 < 0i32;
        let mut v11: bool = v10 == false;
        v11
    }
}
fn state_written_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    std::env::var("EOIE_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v4, &*v4, &*v4].concat());
        let mut v9: bool = std::fs::write(&*v8, &*v2).is_ok();
        let mut v18: US2 = if v9 {
            let mut v10: bool = std::fs::rename(&*v8, &*v0).is_ok();
            if v10 {
                let mut v11: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v11 {
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
        match &v18 {
            US2::US2_Replaced => {
                US1::US1_StateWritten
            }
            _ => {
                let mut v25: Rc<str> = match &v18 {
                    US2::US2_ReadbackMismatch => {
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v22.clone()
                    }
                    US2::US2_RenameFailed => {
                        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v21.clone()
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v20.clone()
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v25.clone())
            }
        }
    } else {
        US1::US1_StateChangedSinceRead
    }
}
fn renamed_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let mut v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v5, &*v5, &*v5].concat());
    let mut v7: bool = std::path::Path::new(&*v6).is_file();
    let mut v10: US0 = if v7 {
        US0::US0_FileReadable(v6.clone())
    } else {
        US0::US0_FileMissing(v6.clone())
    };
    match &v10 {
        US0::US0_FileMissing(v11) => {
            let mut v11: Rc<str> = v11.clone();
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v1, &*v14, &*v12, &*v5].concat());
            let mut v16: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v15 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v16
        }
        US0::US0_FileReadable(v17) => {
            let mut v17: Rc<str> = v17.clone();
            let mut v18: Rc<str> = std::fs::read_to_string(&*v17).unwrap_or_default().into();
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v20, &*v1, &*v21, &*v5].concat());
            let mut v23: i32 = 0i32;
            let mut v24: i32 = declared_from_2(v18.clone(), v22.clone(), v23);
            let mut v25: bool = v24 < 0i32;
            let mut v26: bool = v25 == false;
            let mut v27: bool = v26 == false;
            if v27 {
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v1, &*v30, &*v28, &*v5].concat());
                let mut v32: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v31 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v32
            } else {
                let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v20, &*v2, &*v21, &*v5].concat());
                let mut v34: i32 = 0i32;
                let mut v35: i32 = declared_from_2(v18.clone(), v33.clone(), v34);
                let mut v36: bool = v35 < 0i32;
                let mut v37: bool = v36 == false;
                if v37 {
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id "); } LIT.with(|lit| lit.clone()) };
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already taken"); } LIT.with(|lit| lit.clone()) };
                    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v2, &*v39, &*v5, &*v5].concat());
                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v1, &*v42, &*v40, &*v5].concat());
                    let mut v44: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v43 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v44
                } else {
                    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v45, &v46));
                    let mut v48: Rc<str> = std::rc::Rc::<str>::from(v47.to_ascii_lowercase());
                    let mut v49: bool = binding_declared_3(v18.clone(), v48.clone());
                    if v49 {
                        let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the name inl "); } LIT.with(|lit| lit.clone()) };
                        let mut v51: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v45, &v46));
                        let mut v52: Rc<str> = std::rc::Rc::<str>::from(v51.to_ascii_lowercase());
                        let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already declared"); } LIT.with(|lit| lit.clone()) };
                        let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v50, &*v52, &*v53, &*v5, &*v5].concat());
                        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v57: Rc<str> = std::rc::Rc::<str>::from([&*v55, &*v1, &*v56, &*v54, &*v5].concat());
                        let mut v58: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v57 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v58
                    } else {
                        let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
                        let mut v60: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v45, &v46));
                        let mut v61: Rc<str> = std::rc::Rc::<str>::from(v60.to_ascii_lowercase());
                        let mut v62: Rc<str> = std::rc::Rc::<str>::from([&*v59, &*v61, &*v5, &*v5, &*v5].concat());
                        let mut v63: bool = binding_declared_3(v18.clone(), v62.clone());
                        if v63 {
                            let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the name inl "); } LIT.with(|lit| lit.clone()) };
                            let mut v65: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v45, &v46));
                            let mut v66: Rc<str> = std::rc::Rc::<str>::from(v65.to_ascii_lowercase());
                            let mut v67: Rc<str> = std::rc::Rc::<str>::from([&*v59, &*v66, &*v5, &*v5, &*v5].concat());
                            let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already declared"); } LIT.with(|lit| lit.clone()) };
                            let mut v69: Rc<str> = std::rc::Rc::<str>::from([&*v64, &*v67, &*v68, &*v5, &*v5].concat());
                            let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                            let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let mut v72: Rc<str> = std::rc::Rc::<str>::from([&*v70, &*v1, &*v71, &*v69, &*v5].concat());
                            let mut v73: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v72 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v73
                        } else {
                            let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79): (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) = agile_rename_rewrite::eoie_agile_renamed_text(&*v18, &*v1, &*v2);
                            let mut v80: US1 = state_written_4(v6.clone(), v18.clone(), v74.clone());
                            match &v80 {
                                US1::US1_StateWritten => {
                                    let mut v81: u64 = v18.len() as u64;
                                    let mut v82: u64 = v74.len() as u64;
                                    let mut v83: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
                                    let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile rename ok old="); } LIT.with(|lit| lit.clone()) };
                                    let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" new="); } LIT.with(|lit| lit.clone()) };
                                    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" items="); } LIT.with(|lit| lit.clone()) };
                                    let mut v87: Rc<str> = std::rc::Rc::<str>::from([&*v84, &*v1, &*v85, &*v2, &*v86].concat());
                                    let mut v88: Rc<str> = std::rc::Rc::<str>::from((v75).to_string());
                                    let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deps_rewritten="); } LIT.with(|lit| lit.clone()) };
                                    let mut v90: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deps_in="); } LIT.with(|lit| lit.clone()) };
                                    let mut v92: bool = v77 == v5 ;
                                    let mut v93: Rc<str> = if v92 {
                                        v45.clone()
                                    } else {
                                        v77.clone()
                                    };
                                    let mut v94: Rc<str> = std::rc::Rc::<str>::from([&*v88, &*v89, &*v90, &*v91, &*v93].concat());
                                    let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" titles_mentioning_old="); } LIT.with(|lit| lit.clone()) };
                                    let mut v96: Rc<str> = std::rc::Rc::<str>::from((v78).to_string());
                                    let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" titles_in="); } LIT.with(|lit| lit.clone()) };
                                    let mut v98: bool = v79 == v5 ;
                                    let mut v99: Rc<str> = if v98 {
                                        v45.clone()
                                    } else {
                                        v79.clone()
                                    };
                                    let mut v100: Rc<str> = std::rc::Rc::<str>::from([&*v95, &*v96, &*v97, &*v99, &*v5].concat());
                                    let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                    let mut v102: Rc<str> = std::rc::Rc::<str>::from((v81).to_string());
                                    let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                    let mut v104: Rc<str> = std::rc::Rc::<str>::from((v82).to_string());
                                    let mut v105: Rc<str> = std::rc::Rc::<str>::from([&*v101, &*v102, &*v103, &*v104, &*v5].concat());
                                    let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                    let mut v107: Rc<str> = std::rc::Rc::<str>::from((v83).to_string());
                                    let mut v108: Rc<str> = std::rc::Rc::<str>::from([&*v105, &*v106, &*v107, &*v5, &*v5].concat());
                                    let mut v109: Rc<str> = std::rc::Rc::<str>::from([&*v87, &*v94, &*v100, &*v108, &*v5].concat());
                                    let mut v110: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v109.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                    v110
                                }
                                _ => {
                                    let mut v114: Rc<str> = match &v80 {
                                        US1::US1_StateChangedSinceRead => {
                                            let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                            v111.clone()
                                        }
                                        US1::US1_StateWriteFailed(v112) => {
                                            let mut v112: Rc<str> = v112.clone();
                                            v112.clone()
                                        }
                                        _ => unreachable!(),
                                    };
                                    let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                                    let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                    let mut v117: Rc<str> = std::rc::Rc::<str>::from([&*v115, &*v1, &*v116, &*v114, &*v5].concat());
                                    let mut v118: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v117 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v118
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn id_bytes_valid_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v2 <= v1;
        if v3 {
            return true;
        } else {
            let mut v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v5: bool = 65i32 <= v4;
            let mut v11: bool = if v5 {
                let mut v6: bool = v4 <= 90i32;
                v6
            } else {
                let mut v7: bool = 48i32 <= v4;
                if v7 {
                    let mut v8: bool = v4 <= 57i32;
                    v8
                } else {
                    let mut v9: bool = v4 == 45i32;
                    v9
                }
            };
            if v11 {
                let mut v12: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v12, v2);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn rename_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 3i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: i32 = std::env::args().skip(2).count() as i32;
    let mut v5: bool = v4 == 4i32;
    let mut v6: bool = v5 == false;
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile rename <root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v1, &*v9, &*v7, &*v10].concat());
        let mut v12: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v11 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v12
    } else {
        let mut v13: i32 = v3.len() as i32;
        let mut v14: bool = v13 <= 0i32;
        let mut v18: bool = if v14 {
            false
        } else {
            let mut v15: i32 = 0i32;
            let mut v16: i32 = v3.len() as i32;
            id_bytes_valid_5(v3.clone(), v15, v16)
        };
        let mut v19: bool = v18 == false;
        if v19 {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task id is upper-case letters, digits and -"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v1, &*v22, &*v20, &*v23].concat());
            let mut v25: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v24 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v25
        } else {
            let mut v26: i32 = v3.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v27: bool = 48i32 <= v26;
            let mut v29: bool = if v27 {
                let mut v28: bool = v26 <= 57i32;
                v28
            } else {
                false
            };
            if v29 {
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the new id must start with a letter, so its inl name is a valid identifier"); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v1, &*v32, &*v30, &*v33].concat());
                let mut v35: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v34 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v35
            } else {
                let mut v36: bool = v1 == v3 ;
                if v36 {
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the new id equals the old one"); } LIT.with(|lit| lit.clone()) };
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v1, &*v39, &*v37, &*v40].concat());
                    let mut v42: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v41 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v42
                } else {
                    let mut v43: i32 = 1i32;
                    let mut v44: Rc<str> = usize::try_from(v43).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    renamed_1(v44.clone(), v1.clone(), v3.clone())
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        rename_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_rename_run() -> i32 {
    closure0()()
}
