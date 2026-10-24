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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_UserMemory,
    US1_FeedbackMemory,
    US1_ProjectMemory,
    US1_ReferenceMemory,
    US1_UntypedMemory,
}
#[derive(Clone)]
enum US2 {
    US2_CitesOpen,
    US2_CitesDone,
    US2_CitesMissing,
}
#[derive(Clone)]
enum US3 {
    US3_NoFinding,
    US3_DanglingId(Rc<str>, Rc<str>),
    US3_DoneCited(Rc<str>, Rc<str>),
    US3_UnwrittenLink(Rc<str>, Rc<str>),
    US3_UnknownMemory(Rc<str>, Rc<str>),
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_1(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_2();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_3(v1);
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn directory_exists_4(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_dir();
    v8
}
fn names_from_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            return v2.clone();
        } else {
            let mut v4: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(10u8)).next().unwrap_or(""));
            let mut v5: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v4).to_string_lossy().as_ref());
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v5).unwrap_or_default().as_str());
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("name"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v6; let front = text.strip_prefix("---").and_then(|rest| rest.split_once("
---").map(|(front, _)| front)).unwrap_or(""); front.lines().find_map(|line| line.trim().strip_prefix(&*v7).and_then(|rest| rest.strip_prefix(char::from(58u8))).map(|value| value.trim().trim_matches(char::from(34u8)).to_owned())).unwrap_or_default() }.as_str());
            let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = if v9 {
                let mut v10: Rc<str> = std::rc::Rc::<str>::from((v4).trim_end_matches(".md"));
                v10.clone()
            } else {
                v8.clone()
            };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let mut v13: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = if v13 {
                v11.clone()
            } else {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v14));
                let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v11));
                v16.clone()
            };
            (v0, v1, v2) = (v0.clone(), v12.clone(), v17.clone());
            continue;
        }
    }
}
fn links_start_6() -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0i32, 0i32, 0i32, 0i32, 0i32, 0i32, v0.clone())
}
fn ids_checked_8(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let mut v11: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125): (i32, i32, i32, i32, i32, i32, Rc<str>) = if v11 {
        (v4, v5, v6, v7, v8, v9, v10.clone())
    } else {
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v3).split(char::from(10u8)).next().unwrap_or(""));
        let mut v13: i32 = v5.wrapping_add(1i32);
        let mut v14: Rc<str> = std::rc::Rc::<str>::from((v3).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let mut v15: Rc<str> = agile_task_census::eoie_agile_task_status(&*v0, &*v12);
        let mut v16: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v22: US2 = if v16 {
            US2::US2_CitesMissing
        } else {
            let mut v18: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
            if v18 {
                US2::US2_CitesDone
            } else {
                US2::US2_CitesOpen
            }
        };
        let mut v32: US3 = match &v22 {
            US2::US2_CitesDone => {
                let mut v25: bool = match &v1 {
                    US1::US1_FeedbackMemory => {
                        true
                    }
                    US1::US1_ProjectMemory => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v25 {
                    US3::US3_DoneCited(v2.clone(), v12.clone())
                } else {
                    US3::US3_NoFinding
                }
            }
            US2::US2_CitesMissing => {
                US3::US3_DanglingId(v2.clone(), v12.clone())
            }
            US2::US2_CitesOpen => {
                US3::US3_NoFinding
            }
        };
        let (mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
            US3::US3_NoFinding => {
                (v4, v13, v6, v7, v8, v9, v10.clone())
            }
            _ => {
                let (mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                    US3::US3_DanglingId(v33, v34) => {
                        let mut v33: Rc<str> = v33.clone();
                        let mut v34: Rc<str> = v34.clone();
                        let mut v35: i32 = v6.wrapping_add(1i32);
                        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dangling  "); } LIT.with(|lit| lit.clone()) };
                        let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v33));
                        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
                        let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v34));
                        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  no backlog item with this id\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
                        let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v42));
                        (v4, v13, v35, v7, v8, v9, v43.clone())
                    }
                    _ => {
                        let (mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                            US3::US3_DoneCited(v44, v45) => {
                                let mut v44: Rc<str> = v44.clone();
                                let mut v45: Rc<str> = v45.clone();
                                let mut v46: i32 = v7.wrapping_add(1i32);
                                let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done  "); } LIT.with(|lit| lit.clone()) };
                                let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v44));
                                let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                                let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
                                let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v45));
                                let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  the item is Done; check that the rule still holds\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
                                let mut v54: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v53));
                                (v4, v13, v6, v46, v8, v9, v54.clone())
                            }
                            _ => {
                                let (mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                                    US3::US3_UnwrittenLink(v55, v56) => {
                                        let mut v55: Rc<str> = v55.clone();
                                        let mut v56: Rc<str> = v56.clone();
                                        let mut v57: i32 = v8.wrapping_add(1i32);
                                        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unwritten-link  "); } LIT.with(|lit| lit.clone()) };
                                        let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v55));
                                        let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  [["); } LIT.with(|lit| lit.clone()) };
                                        let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v60));
                                        let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v56));
                                        let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]]  no memory with this name yet\n"); } LIT.with(|lit| lit.clone()) };
                                        let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v63));
                                        let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v64));
                                        (v4, v13, v6, v7, v57, v9, v65.clone())
                                    }
                                    _ => {
                                        let (mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v32 {
                                            US3::US3_UnknownMemory(v66, v67) => {
                                                let mut v66: Rc<str> = v66.clone();
                                                let mut v67: Rc<str> = v67.clone();
                                                let mut v68: i32 = v9.wrapping_add(1i32);
                                                let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown-memory  "); } LIT.with(|lit| lit.clone()) };
                                                let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v66));
                                                let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                                                let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v71));
                                                let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v67));
                                                let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  a backlog note cites this memory, which does not exist\n"); } LIT.with(|lit| lit.clone()) };
                                                let mut v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
                                                let mut v76: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v75));
                                                (v4, v13, v6, v7, v8, v68, v76.clone())
                                            }
                                            _ => unreachable!(),
                                        };
                                        (v77, v78, v79, v80, v81, v82, v83.clone())
                                    }
                                };
                                (v84, v85, v86, v87, v88, v89, v90.clone())
                            }
                        };
                        (v91, v92, v93, v94, v95, v96, v97.clone())
                    }
                };
                (v98, v99, v100, v101, v102, v103, v104.clone())
            }
        };
        let (mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118): (i32, i32, i32, i32, i32, i32, Rc<str>) = ids_checked_8(v0.clone(), v1.clone(), v2.clone(), v14.clone(), v105, v106, v107, v108, v109, v110, v111.clone());
        (v112, v113, v114, v115, v116, v117, v118.clone())
    };
    (v119, v120, v121, v122, v123, v124, v125.clone())
}
fn method11(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method12(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn loop_10(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v3 >= v2;
        if v4 {
            return false;
        } else {
            let mut v5: Rc<str> = v1.clone().borrow()[v3 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: i32 = 0i32;
            let mut v8: i32 = method11(v5.clone(), v6, v7);
            let mut v9: i32 = v6.wrapping_sub(1i32);
            let mut v10: Rc<str> = string_slice(&v5.clone(), v8 as i64, v9 as i64);
            let mut v11: i32 = (v10.clone().len() as i32);
            let mut v12: i32 = method12(v10.clone(), v11);
            let mut v13: Rc<str> = string_slice(&v10.clone(), 0i32 as i64, v12 as i64);
            let mut v14: bool = v13.clone() == v0.clone();
            if v14 {
                return true;
            } else {
                let mut v15: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2, v15);
                continue;
            }
        }
    }
}
fn links_checked_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let mut v10: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113): (i32, i32, i32, i32, i32, i32, Rc<str>) = if v10 {
        (v3, v4, v5, v6, v7, v8, v9.clone())
    } else {
        let mut v11: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let mut v12: Rc<str> = std::rc::Rc::<str>::from((v2).split(char::from(10u8)).next().unwrap_or(""));
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v13).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v15: i32 = (v14.clone().borrow().len() as i32);
        let mut v16: i32 = 0i32;
        let mut v17: bool = loop_10(v12.clone(), v14.clone(), v15, v16);
        let mut v20: US3 = if v17 {
            US3::US3_NoFinding
        } else {
            US3::US3_UnwrittenLink(v1.clone(), v12.clone())
        };
        let (mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
            US3::US3_NoFinding => {
                (v3, v4, v5, v6, v7, v8, v9.clone())
            }
            _ => {
                let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                    US3::US3_DanglingId(v21, v22) => {
                        let mut v21: Rc<str> = v21.clone();
                        let mut v22: Rc<str> = v22.clone();
                        let mut v23: i32 = v5.wrapping_add(1i32);
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dangling  "); } LIT.with(|lit| lit.clone()) };
                        let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v21));
                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
                        let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v22));
                        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  no backlog item with this id\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
                        let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v30));
                        (v3, v4, v23, v6, v7, v8, v31.clone())
                    }
                    _ => {
                        let (mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                            US3::US3_DoneCited(v32, v33) => {
                                let mut v32: Rc<str> = v32.clone();
                                let mut v33: Rc<str> = v33.clone();
                                let mut v34: i32 = v6.wrapping_add(1i32);
                                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done  "); } LIT.with(|lit| lit.clone()) };
                                let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v32));
                                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                                let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
                                let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v33));
                                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  the item is Done; check that the rule still holds\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                                let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v41));
                                (v3, v4, v5, v34, v7, v8, v42.clone())
                            }
                            _ => {
                                let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                                    US3::US3_UnwrittenLink(v43, v44) => {
                                        let mut v43: Rc<str> = v43.clone();
                                        let mut v44: Rc<str> = v44.clone();
                                        let mut v45: i32 = v7.wrapping_add(1i32);
                                        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unwritten-link  "); } LIT.with(|lit| lit.clone()) };
                                        let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v43));
                                        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  [["); } LIT.with(|lit| lit.clone()) };
                                        let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
                                        let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v44));
                                        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]]  no memory with this name yet\n"); } LIT.with(|lit| lit.clone()) };
                                        let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v51));
                                        let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v52));
                                        (v3, v4, v5, v6, v45, v8, v53.clone())
                                    }
                                    _ => {
                                        let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v20 {
                                            US3::US3_UnknownMemory(v54, v55) => {
                                                let mut v54: Rc<str> = v54.clone();
                                                let mut v55: Rc<str> = v55.clone();
                                                let mut v56: i32 = v8.wrapping_add(1i32);
                                                let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown-memory  "); } LIT.with(|lit| lit.clone()) };
                                                let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v54));
                                                let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                                                let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
                                                let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v55));
                                                let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  a backlog note cites this memory, which does not exist\n"); } LIT.with(|lit| lit.clone()) };
                                                let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v62));
                                                let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v63));
                                                (v3, v4, v5, v6, v7, v56, v64.clone())
                                            }
                                            _ => unreachable!(),
                                        };
                                        (v65, v66, v67, v68, v69, v70, v71.clone())
                                    }
                                };
                                (v72, v73, v74, v75, v76, v77, v78.clone())
                            }
                        };
                        (v79, v80, v81, v82, v83, v84, v85.clone())
                    }
                };
                (v86, v87, v88, v89, v90, v91, v92.clone())
            }
        };
        let (mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106): (i32, i32, i32, i32, i32, i32, Rc<str>) = links_checked_9(v0.clone(), v1.clone(), v11.clone(), v93, v94, v95, v96, v97, v98, v99.clone());
        (v100, v101, v102, v103, v104, v105, v106.clone())
    };
    (v107, v108, v109, v110, v111, v112, v113.clone())
}
fn memories_checked_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: i32, mut v11: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let mut v12: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67): (i32, i32, i32, i32, i32, i32, Rc<str>) = if v12 {
        (v5, v6, v7, v8, v9, v10, v11.clone())
    } else {
        let mut v13: Rc<str> = std::rc::Rc::<str>::from((v4).split(char::from(10u8)).next().unwrap_or(""));
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v3).join(&*v13).to_string_lossy().as_ref());
        let mut v15: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v14).unwrap_or_default().as_str());
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("name"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v15; let front = text.strip_prefix("---").and_then(|rest| rest.split_once("
---").map(|(front, _)| front)).unwrap_or(""); front.lines().find_map(|line| line.trim().strip_prefix(&*v16).and_then(|rest| rest.strip_prefix(char::from(58u8))).map(|value| value.trim().trim_matches(char::from(34u8)).to_owned())).unwrap_or_default() }.as_str());
        let mut v18: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = if v18 {
            let mut v19: Rc<str> = std::rc::Rc::<str>::from((v13).trim_end_matches(".md"));
            v19.clone()
        } else {
            v17.clone()
        };
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v15; let front = text.strip_prefix("---").and_then(|rest| rest.split_once("
---").map(|(front, _)| front)).unwrap_or(""); front.lines().find_map(|line| line.trim().strip_prefix(&*v21).and_then(|rest| rest.strip_prefix(char::from(58u8))).map(|value| value.trim().trim_matches(char::from(34u8)).to_owned())).unwrap_or_default() }.as_str());
        let mut v23: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user"); } LIT.with(|lit| lit.clone()) };
        let mut v35: US1 = if v23 {
            US1::US1_UserMemory
        } else {
            let mut v25: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("feedback"); } LIT.with(|lit| lit.clone()) };
            if v25 {
                US1::US1_FeedbackMemory
            } else {
                let mut v27: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("project"); } LIT.with(|lit| lit.clone()) };
                if v27 {
                    US1::US1_ProjectMemory
                } else {
                    let mut v29: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reference"); } LIT.with(|lit| lit.clone()) };
                    if v29 {
                        US1::US1_ReferenceMemory
                    } else {
                        US1::US1_UntypedMemory
                    }
                }
            }
        };
        let mut v36: Rc<str> = std::rc::Rc::<str>::from({ let known = (v1).split(char::from(124u8)).collect::<std::collections::HashSet<_>>(); let mut ids = (v15).split(|c: char| !(c.is_ascii_alphanumeric() || c == char::from(45u8) || c == char::from(95u8))).map(|token| token.trim_matches(char::from(45u8))).filter(|token| token.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == char::from(45u8))).filter(|token| token.contains(char::from(45u8)) && token.split(char::from(45u8)).all(|part| !part.is_empty()) && token.starts_with(|c: char| c.is_ascii_uppercase()) && known.contains(token.split(char::from(45u8)).next().unwrap_or(""))).map(str::to_owned).collect::<Vec<_>>(); ids.sort(); ids.dedup(); ids.join("
") }.as_str());
        let mut v37: i32 = v5.wrapping_add(1i32);
        let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (i32, i32, i32, i32, i32, i32, Rc<str>) = ids_checked_8(v0.clone(), v35.clone(), v20.clone(), v36.clone(), v37, v6, v7, v8, v9, v10, v11.clone());
        let mut v45: Rc<str> = std::rc::Rc::<str>::from((v4).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let mut v46: Rc<str> = std::rc::Rc::<str>::from({ let mut links = (v15).split("[[").skip(1).filter_map(|rest| rest.split_once("]]").map(|(target, _)| target.trim().to_owned())).filter(|target| !target.is_empty()).collect::<Vec<_>>(); links.sort(); links.dedup(); links.join("
") }.as_str());
        let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (i32, i32, i32, i32, i32, i32, Rc<str>) = links_checked_9(v2.clone(), v20.clone(), v46.clone(), v38, v39, v40, v41, v42, v43, v44.clone());
        let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (i32, i32, i32, i32, i32, i32, Rc<str>) = memories_checked_7(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v45.clone(), v47, v48, v49, v50, v51, v52, v53.clone());
        (v54, v55, v56, v57, v58, v59, v60.clone())
    };
    (v61, v62, v63, v64, v65, v66, v67.clone())
}
fn cites_in_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let mut v11: i32 = (v2).get(v3 as usize..).and_then(|rest| rest.find("memory ")).map(|at| at as i32 + v3).unwrap_or(-1);
    let mut v12: bool = v11 < 0i32;
    let (mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119): (i32, i32, i32, i32, i32, i32, Rc<str>) = if v12 {
        (v4, v5, v6, v7, v8, v9, v10.clone())
    } else {
        let mut v13: i32 = v11.wrapping_add(7i32);
        let mut v14: Rc<str> = std::rc::Rc::<str>::from((v2).get(v13 as usize..).unwrap_or("").chars().take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == char::from(45u8)).collect::<String>().trim_end_matches(char::from(45u8)));
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v15).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v17: i32 = (v16.clone().borrow().len() as i32);
        let mut v18: i32 = 0i32;
        let mut v19: bool = loop_10(v14.clone(), v16.clone(), v17, v18);
        let mut v26: US3 = if v19 {
            US3::US3_NoFinding
        } else {
            let mut v21: i32 = (v14).matches(char::from(45u8)).count() as i32;
            let mut v22: bool = v21 >= 2i32;
            if v22 {
                US3::US3_UnknownMemory(v1.clone(), v14.clone())
            } else {
                US3::US3_NoFinding
            }
        };
        let (mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
            US3::US3_NoFinding => {
                (v4, v5, v6, v7, v8, v9, v10.clone())
            }
            _ => {
                let (mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                    US3::US3_DanglingId(v27, v28) => {
                        let mut v27: Rc<str> = v27.clone();
                        let mut v28: Rc<str> = v28.clone();
                        let mut v29: i32 = v6.wrapping_add(1i32);
                        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dangling  "); } LIT.with(|lit| lit.clone()) };
                        let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v27));
                        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                        let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v28));
                        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  no backlog item with this id\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
                        let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v36));
                        (v4, v5, v29, v7, v8, v9, v37.clone())
                    }
                    _ => {
                        let (mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                            US3::US3_DoneCited(v38, v39) => {
                                let mut v38: Rc<str> = v38.clone();
                                let mut v39: Rc<str> = v39.clone();
                                let mut v40: i32 = v7.wrapping_add(1i32);
                                let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done  "); } LIT.with(|lit| lit.clone()) };
                                let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v38));
                                let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                                let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v43));
                                let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v39));
                                let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  the item is Done; check that the rule still holds\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
                                let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v47));
                                (v4, v5, v6, v40, v8, v9, v48.clone())
                            }
                            _ => {
                                let (mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                                    US3::US3_UnwrittenLink(v49, v50) => {
                                        let mut v49: Rc<str> = v49.clone();
                                        let mut v50: Rc<str> = v50.clone();
                                        let mut v51: i32 = v8.wrapping_add(1i32);
                                        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unwritten-link  "); } LIT.with(|lit| lit.clone()) };
                                        let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v49));
                                        let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  [["); } LIT.with(|lit| lit.clone()) };
                                        let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
                                        let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v50));
                                        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]]  no memory with this name yet\n"); } LIT.with(|lit| lit.clone()) };
                                        let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
                                        let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v58));
                                        (v4, v5, v6, v7, v51, v9, v59.clone())
                                    }
                                    _ => {
                                        let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77): (i32, i32, i32, i32, i32, i32, Rc<str>) = match &v26 {
                                            US3::US3_UnknownMemory(v60, v61) => {
                                                let mut v60: Rc<str> = v60.clone();
                                                let mut v61: Rc<str> = v61.clone();
                                                let mut v62: i32 = v9.wrapping_add(1i32);
                                                let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown-memory  "); } LIT.with(|lit| lit.clone()) };
                                                let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v60));
                                                let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                                                let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v65));
                                                let mut v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v61));
                                                let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  a backlog note cites this memory, which does not exist\n"); } LIT.with(|lit| lit.clone()) };
                                                let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
                                                let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v69));
                                                (v4, v5, v6, v7, v8, v62, v70.clone())
                                            }
                                            _ => unreachable!(),
                                        };
                                        (v71, v72, v73, v74, v75, v76, v77.clone())
                                    }
                                };
                                (v78, v79, v80, v81, v82, v83, v84.clone())
                            }
                        };
                        (v85, v86, v87, v88, v89, v90, v91.clone())
                    }
                };
                (v92, v93, v94, v95, v96, v97, v98.clone())
            }
        };
        let (mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112): (i32, i32, i32, i32, i32, i32, Rc<str>) = cites_in_14(v0.clone(), v1.clone(), v2.clone(), v13, v99, v100, v101, v102, v103, v104, v105.clone());
        (v106, v107, v108, v109, v110, v111, v112.clone())
    };
    (v113, v114, v115, v116, v117, v118, v119.clone())
}
fn notes_checked_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> (i32, i32, i32, i32, i32, i32, Rc<str>) {
    let mut v10: i32 = (v1).get(v2 as usize..).and_then(|rest| rest.find("= Note (")).map(|at| at as i32 + v2).unwrap_or(-1);
    let mut v11: bool = v10 < 0i32;
    let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (i32, i32, i32, i32, i32, i32, Rc<str>) = if v11 {
        (v3, v4, v5, v6, v7, v8, v9.clone())
    } else {
        let mut v12: i32 = (v1).get(v10 as usize..).and_then(|rest| rest.find(char::from(10u8))).map(|at| at as i32 + v10).unwrap_or((v1).len() as i32);
        let mut v13: Rc<str> = std::rc::Rc::<str>::from((v1).get(v10 as usize..v12 as usize).unwrap_or(""));
        let mut v14: Rc<str> = std::rc::Rc::<str>::from((v13).split(char::from(34u8)).nth(1).unwrap_or(""));
        let mut v15: Rc<str> = std::rc::Rc::<str>::from((v1).get(v10 as usize..v12 as usize).unwrap_or(""));
        let mut v16: i32 = 0i32;
        let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (i32, i32, i32, i32, i32, i32, Rc<str>) = cites_in_14(v0.clone(), v14.clone(), v15.clone(), v16, v3, v4, v5, v6, v7, v8, v9.clone());
        let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (i32, i32, i32, i32, i32, i32, Rc<str>) = notes_checked_13(v0.clone(), v1.clone(), v12, v17, v18, v19, v20, v21, v22, v23.clone());
        (v24, v25, v26, v27, v28, v29, v30.clone())
    };
    (v31, v32, v33, v34, v35, v36, v37.clone())
}
fn links_run_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = argument_1(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--memory"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v2).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
    let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: bool = if v4 {
        true
    } else {
        let mut v5: bool = directory_exists_4(v3.clone());
        let mut v6: bool = v5 == false;
        v6
    };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: agile links rejected: --memory <dir> must name a memory directory"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", &*v8);
        2i32
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v9).to_string_lossy().as_ref());
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v10).join(&*v11).to_string_lossy().as_ref());
        let mut v13: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v12).unwrap_or_default().as_str());
        let mut v14: Rc<str> = std::rc::Rc::<str>::from({ let mut names = std::fs::read_dir(&*v3).into_iter().flatten().flatten().map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.ends_with(".md") && name != "MEMORY.md").collect::<Vec<_>>(); names.sort(); names.join("
") }.as_str());
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = names_from_5(v3.clone(), v14.clone(), v15.clone());
        let mut v17: i32 = 0i32;
        let mut v18: Rc<str> = std::rc::Rc::<str>::from({ let mut prefixes = (v13).match_indices("= Task (").filter_map(|(at, marker)| (v13)[at + marker.len()..].strip_prefix(char::from(34u8)).and_then(|rest| rest.split(char::from(45u8)).next()).map(str::to_owned)).collect::<Vec<_>>(); prefixes.sort(); prefixes.dedup(); prefixes.join("|") }.as_str());
        let (mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (i32, i32, i32, i32, i32, i32, Rc<str>) = links_start_6();
        let (mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32): (i32, i32, i32, i32, i32, i32, Rc<str>) = memories_checked_7(v13.clone(), v18.clone(), v16.clone(), v3.clone(), v14.clone(), v19, v20, v21, v22, v23, v24, v25.clone());
        let (mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39): (i32, i32, i32, i32, i32, i32, Rc<str>) = notes_checked_13(v16.clone(), v13.clone(), v17, v26, v27, v28, v29, v30, v31, v32.clone());
        let mut v40: Rc<str> = std::rc::Rc::<str>::from((v33).to_string().as_str());
        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memories="); } LIT.with(|lit| lit.clone()) };
        let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v40));
        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cited="); } LIT.with(|lit| lit.clone()) };
        let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v43));
        let mut v45: Rc<str> = std::rc::Rc::<str>::from((v34).to_string().as_str());
        let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
        let mut v47: Rc<str> = std::rc::Rc::<str>::from((v35).to_string().as_str());
        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" dangling="); } LIT.with(|lit| lit.clone()) };
        let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v47));
        let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" done="); } LIT.with(|lit| lit.clone()) };
        let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
        let mut v52: Rc<str> = std::rc::Rc::<str>::from((v36).to_string().as_str());
        let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
        let mut v54: Rc<str> = std::rc::Rc::<str>::from((v37).to_string().as_str());
        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unwritten_links="); } LIT.with(|lit| lit.clone()) };
        let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v54));
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unknown_memories="); } LIT.with(|lit| lit.clone()) };
        let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
        let mut v59: Rc<str> = std::rc::Rc::<str>::from((v38).to_string().as_str());
        let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
        let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v60));
        let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v61));
        let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
        let mut v64: bool = std::env::args().any(|argument| argument == &*v63);
        if v64 {
            let mut v65: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); (v39).lines().filter_map(|line| { let mut parts = line.splitn(4, "  "); Some(format!("{{{q}kind{q}:{q}{}{q},{q}memory{q}:{q}{}{q},{q}target{q}:{q}{}{q}}}", parts.next()?, parts.next()?, parts.next()?.trim_start_matches("[[").trim_end_matches("]]"))) }).collect::<Vec<_>>().join(",") }.as_str());
            let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"findings\":["); } LIT.with(|lit| lit.clone()) };
            let mut v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v65));
            let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"dangling\":"); } LIT.with(|lit| lit.clone()) };
            let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
            let mut v70: Rc<str> = std::rc::Rc::<str>::from((v35).to_string().as_str());
            let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
            let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v72));
            println!("{}", &*v73);
            ()
        } else {
            let mut v74: bool = v39.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v74 {
                let mut v75: Rc<str> = std::rc::Rc::<str>::from((v39).trim_end());
                println!("{}", &*v75);
                ()
            };
            let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile links ok "); } LIT.with(|lit| lit.clone()) };
            let mut v77: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v62));
            println!("{}", &*v77);
            ()
        };
        let mut v78: bool = v35 == 0i32;
        if v78 {
            0i32
        } else {
            1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        links_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_links_run() -> i32 {
    closure0()()
}
