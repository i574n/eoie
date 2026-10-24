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
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v1));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    let mut v9: bool = v0.starts_with(&*v8);
    if v9 {
        true
    } else {
        let mut v10: i32 = v0.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
        let mut v11: bool = v10 < 0i32;
        let mut v12: bool = v11 == false;
        v12
    }
}
fn state_written_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
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
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v1));
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v12));
            let mut v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v18
        }
        US0::US0_FileReadable(v19) => {
            let mut v19: Rc<str> = v19.clone();
            let mut v20: Rc<str> = std::fs::read_to_string(&*v19).unwrap_or_default().into();
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v22, &*v1, &*v23, &*v5].concat());
            let mut v25: i32 = 0i32;
            let mut v26: i32 = declared_from_2(v20.clone(), v24.clone(), v25);
            let mut v27: bool = v26 < 0i32;
            let mut v28: bool = v27 == false;
            let mut v29: bool = v28 == false;
            if v29 {
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v1));
                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
                let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v30));
                let mut v36: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v35 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v36
            } else {
                let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v22, &*v2, &*v23, &*v5].concat());
                let mut v38: i32 = 0i32;
                let mut v39: i32 = declared_from_2(v20.clone(), v37.clone(), v38);
                let mut v40: bool = v39 < 0i32;
                let mut v41: bool = v40 == false;
                if v41 {
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id "); } LIT.with(|lit| lit.clone()) };
                    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v2));
                    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already taken"); } LIT.with(|lit| lit.clone()) };
                    let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v1));
                    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
                    let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v45));
                    let mut v51: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v50 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v51
                } else {
                    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v54: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v52, &v53));
                    let mut v55: Rc<str> = std::rc::Rc::<str>::from(v54.to_ascii_lowercase());
                    let mut v56: bool = binding_declared_3(v20.clone(), v55.clone());
                    if v56 {
                        let mut v57: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v52, &v53));
                        let mut v58: Rc<str> = std::rc::Rc::<str>::from(v57.to_ascii_lowercase());
                        let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the name inl "); } LIT.with(|lit| lit.clone()) };
                        let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v58));
                        let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already declared"); } LIT.with(|lit| lit.clone()) };
                        let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
                        let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                        let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v1));
                        let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v65));
                        let mut v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v62));
                        let mut v68: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v67 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v68
                    } else {
                        let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
                        let mut v70: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v52, &v53));
                        let mut v71: Rc<str> = std::rc::Rc::<str>::from(v70.to_ascii_lowercase());
                        let mut v72: Rc<str> = std::rc::Rc::<str>::from([&*v69, &*v71, &*v5, &*v5, &*v5].concat());
                        let mut v73: bool = binding_declared_3(v20.clone(), v72.clone());
                        if v73 {
                            let mut v74: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v52, &v53));
                            let mut v75: Rc<str> = std::rc::Rc::<str>::from(v74.to_ascii_lowercase());
                            let mut v76: Rc<str> = std::rc::Rc::<str>::from([&*v69, &*v75, &*v5, &*v5, &*v5].concat());
                            let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the name inl "); } LIT.with(|lit| lit.clone()) };
                            let mut v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v76));
                            let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already declared"); } LIT.with(|lit| lit.clone()) };
                            let mut v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
                            let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                            let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v1));
                            let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v83));
                            let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v80));
                            let mut v86: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v85 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v86
                        } else {
                            let (mut v87, mut v88, mut v89, mut v90, mut v91, mut v92): (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) = agile_rename_rewrite::eoie_agile_renamed_text(&*v20, &*v1, &*v2);
                            let mut v93: US1 = state_written_4(v6.clone(), v20.clone(), v87.clone());
                            match &v93 {
                                US1::US1_StateWritten => {
                                    let mut v94: u64 = v20.len() as u64;
                                    let mut v95: u64 = v87.len() as u64;
                                    let mut v96: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
                                    let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile rename ok old="); } LIT.with(|lit| lit.clone()) };
                                    let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v1));
                                    let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" new="); } LIT.with(|lit| lit.clone()) };
                                    let mut v100: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v99));
                                    let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v2));
                                    let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" items="); } LIT.with(|lit| lit.clone()) };
                                    let mut v103: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v102));
                                    let mut v104: Rc<str> = std::rc::Rc::<str>::from((v88).to_string());
                                    let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deps_rewritten="); } LIT.with(|lit| lit.clone()) };
                                    let mut v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
                                    let mut v107: Rc<str> = std::rc::Rc::<str>::from((v89).to_string());
                                    let mut v108: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v107));
                                    let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deps_in="); } LIT.with(|lit| lit.clone()) };
                                    let mut v110: Rc<str> = Rc::<str>::from(format!("{}{}", v108, v109));
                                    let mut v111: bool = v90.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v112: Rc<str> = if v111 {
                                        v52.clone()
                                    } else {
                                        v90.clone()
                                    };
                                    let mut v113: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v112));
                                    let mut v114: Rc<str> = std::rc::Rc::<str>::from((v91).to_string());
                                    let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" titles_mentioning_old="); } LIT.with(|lit| lit.clone()) };
                                    let mut v116: Rc<str> = Rc::<str>::from(format!("{}{}", v115, v114));
                                    let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" titles_in="); } LIT.with(|lit| lit.clone()) };
                                    let mut v118: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v117));
                                    let mut v119: bool = v92.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v120: Rc<str> = if v119 {
                                        v52.clone()
                                    } else {
                                        v92.clone()
                                    };
                                    let mut v121: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v120));
                                    let mut v122: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v113));
                                    let mut v123: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v121));
                                    let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                    let mut v125: Rc<str> = std::rc::Rc::<str>::from((v94).to_string());
                                    let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                    let mut v127: Rc<str> = std::rc::Rc::<str>::from((v95).to_string());
                                    let mut v128: Rc<str> = std::rc::Rc::<str>::from([&*v124, &*v125, &*v126, &*v127, &*v5].concat());
                                    let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                    let mut v130: Rc<str> = std::rc::Rc::<str>::from((v96).to_string());
                                    let mut v131: Rc<str> = std::rc::Rc::<str>::from([&*v128, &*v129, &*v130, &*v5, &*v5].concat());
                                    let mut v132: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v131));
                                    let mut v133: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v132.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                    v133
                                }
                                _ => {
                                    let mut v137: Rc<str> = match &v93 {
                                        US1::US1_StateChangedSinceRead => {
                                            let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                            v134.clone()
                                        }
                                        US1::US1_StateWriteFailed(v135) => {
                                            let mut v135: Rc<str> = v135.clone();
                                            v135.clone()
                                        }
                                        _ => unreachable!(),
                                    };
                                    let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                                    let mut v139: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v1));
                                    let mut v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                    let mut v141: Rc<str> = Rc::<str>::from(format!("{}{}", v139, v140));
                                    let mut v142: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v137));
                                    let mut v143: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v142 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v143
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn command_line_argument_count_6() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn argument_count_5() -> i32 {
    let mut v0: i32 = command_line_argument_count_6();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn id_bytes_valid_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
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
    let mut v4: i32 = argument_count_5();
    let mut v5: bool = v4 == 4i32;
    let mut v6: bool = v5 == false;
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile rename <root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v1));
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
        let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v7));
        let mut v13: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v12 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v13
    } else {
        let mut v14: i32 = (v3.clone().len() as i32);
        let mut v15: bool = v14 <= 0i32;
        let mut v18: bool = if v15 {
            false
        } else {
            let mut v16: i32 = 0i32;
            id_bytes_valid_7(v3.clone(), v16, v14)
        };
        let mut v19: bool = v18 == false;
        if v19 {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task id is upper-case letters, digits and -"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v1));
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
            let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v20));
            let mut v26: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v25 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v26
        } else {
            let mut v27: i32 = v3.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v28: bool = 48i32 <= v27;
            let mut v30: bool = if v28 {
                let mut v29: bool = v27 <= 57i32;
                v29
            } else {
                false
            };
            if v30 {
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the new id must start with a letter, so its inl name is a valid identifier"); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v1));
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v31));
                let mut v37: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v36 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v37
            } else {
                let mut v38: bool = v1.clone() == v3.clone();
                if v38 {
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the new id equals the old one"); } LIT.with(|lit| lit.clone()) };
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v1));
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
                    let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v39));
                    let mut v45: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v44 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v45
                } else {
                    let mut v46: i32 = 1i32;
                    let mut v47: Rc<str> = usize::try_from(v46).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    renamed_1(v47.clone(), v1.clone(), v3.clone())
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
