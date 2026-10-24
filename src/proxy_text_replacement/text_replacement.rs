#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ReplacementReceipt(Rc<str>),
    US0_ReplacementRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_CheckReplacement,
    US2_ApplyReplacement,
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
#[derive(Clone)]
enum US4 {
    US4_ExactlyOneMatch,
    US4_ExactlyMatches(u64),
    US4_EveryMatch,
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US4),
    US3_Invalid,
}
#[derive(Clone)]
enum US5 {
    US5_Replaced,
    US5_TemporaryWriteFailed,
    US5_RenameFailed,
    US5_ReadbackMismatch,
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn proxy_argument_count_2() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn regular_file_4(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn text_replace_outcome_1() -> US0 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
    let mut v10: US1 = if v2 {
        let mut v3: US2 = US2::US2_CheckReplacement;
        US1::US1_Valid(v3.clone())
    } else {
        let mut v5: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
        if v5 {
            let mut v6: US2 = US2::US2_ApplyReplacement;
            US1::US1_Valid(v6.clone())
        } else {
            US1::US1_Invalid
        }
    };
    let mut v11: i32 = proxy_argument_count_2();
    let mut v12: bool = 6i32 == v11;
    let mut v37: US3 = if v12 {
        let mut v13: US4 = US4::US4_ExactlyOneMatch;
        US3::US3_Valid(v13.clone())
    } else {
        let mut v15: bool = 7i32 == v11;
        if v15 {
            let mut v16: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 6i32 as usize).unwrap_or_default());
            let mut v17: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--all"); } LIT.with(|lit| lit.clone()) };
            if v17 {
                let mut v18: US4 = US4::US4_EveryMatch;
                US3::US3_Valid(v18.clone())
            } else {
                US3::US3_Invalid
            }
        } else {
            let mut v22: bool = 8i32 == v11;
            if v22 {
                let mut v23: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 7i32 as usize).unwrap_or_default());
                let mut v24: u64 = v23.parse::<u64>().unwrap_or(0);
                let mut v25: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 6i32 as usize).unwrap_or_default());
                let mut v26: bool = v25.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--expect"); } LIT.with(|lit| lit.clone()) };
                if v26 {
                    let mut v27: bool = 0 < v24;
                    if v27 {
                        let mut v28: US4 = US4::US4_ExactlyMatches(v24);
                        US3::US3_Valid(v28.clone())
                    } else {
                        US3::US3_Invalid
                    }
                } else {
                    US3::US3_Invalid
                }
            } else {
                US3::US3_Invalid
            }
        }
    };
    match &v10 {
        US1::US1_Invalid => {
            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace mode must be check or apply"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ReplacementRejected(v38.clone())
        }
        _ => {
            match &v37 {
                US3::US3_Invalid => {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace expects <check|apply> <root> <relative> <old> <new> [--expect N | --all]"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_ReplacementRejected(v40.clone())
                }
                _ => {
                    match &v10 {
                        US1::US1_Valid(v42) => {
                            let mut v42: US2 = v42.clone();
                            match &v37 {
                                US3::US3_Valid(v43) => {
                                    let mut v43: US4 = v43.clone();
                                    let mut v44: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 4i32 as usize).unwrap_or_default());
                                    let mut v45: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 5i32 as usize).unwrap_or_default());
                                    let mut v46: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
                                    let mut v47: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 3i32 as usize).unwrap_or_default());
                                    let mut v48: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v46), &*v47).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                    let mut v49: bool = v48.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    if v49 {
                                        let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace target escapes the root"); } LIT.with(|lit| lit.clone()) };
                                        US0::US0_ReplacementRejected(v50.clone())
                                    } else {
                                        let mut v52: bool = v44.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        if v52 {
                                            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace old text is empty"); } LIT.with(|lit| lit.clone()) };
                                            US0::US0_ReplacementRejected(v53.clone())
                                        } else {
                                            let mut v55: bool = v44.clone() == v45.clone();
                                            if v55 {
                                                let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace old and new text are identical"); } LIT.with(|lit| lit.clone()) };
                                                US0::US0_ReplacementRejected(v56.clone())
                                            } else {
                                                let mut v58: bool = regular_file_4(v48.clone());
                                                if v58 {
                                                    let mut v59: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v48).unwrap_or_default());
                                                    let mut v60: u64 = v59.match_indices(&*v44).count() as u64;
                                                    let mut v67: bool = match &v43 {
                                                        US4::US4_EveryMatch => {
                                                            let mut v64: bool = 0 < v60;
                                                            v64
                                                        }
                                                        US4::US4_ExactlyMatches(v62) => {
                                                            let mut v62: u64 = *v62;
                                                            let mut v63: bool = v60 == v62;
                                                            v63
                                                        }
                                                        US4::US4_ExactlyOneMatch => {
                                                            let mut v61: bool = v60 == 1u64;
                                                            v61
                                                        }
                                                    };
                                                    if v67 {
                                                        let mut v70: Rc<str> = match &v43 {
                                                            US4::US4_EveryMatch => {
                                                                let mut v68: Rc<str> = std::rc::Rc::<str>::from(v59.replace(&*v44, &*v45));
                                                                v68.clone()
                                                            }
                                                            _ => {
                                                                let mut v69: Rc<str> = std::rc::Rc::<str>::from(v59.replacen(&*v44, &*v45, v60 as usize));
                                                                v69.clone()
                                                            }
                                                        };
                                                        match &v42 {
                                                            US2::US2_ApplyReplacement => {
                                                                let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                                                                let mut v106: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
                                                                let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                                                                let mut v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                let mut v109: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v105, &*v106, &*v107, &*v108].concat());
                                                                let mut v110: bool = std::fs::write(&*v109, &*v70).is_ok();
                                                                let mut v119: US5 = if v110 {
                                                                    let mut v111: bool = std::fs::rename(&*v109, &*v48).is_ok();
                                                                    if v111 {
                                                                        let mut v112: bool = std::fs::read_to_string(&*v48).is_ok_and(|value| value == *v70);
                                                                        if v112 {
                                                                            US5::US5_Replaced
                                                                        } else {
                                                                            US5::US5_ReadbackMismatch
                                                                        }
                                                                    } else {
                                                                        US5::US5_RenameFailed
                                                                    }
                                                                } else {
                                                                    US5::US5_TemporaryWriteFailed
                                                                };
                                                                match &v119 {
                                                                    US5::US5_Replaced => {
                                                                        let mut v120: u64 = v59.len() as u64;
                                                                        let mut v121: u64 = v70.len() as u64;
                                                                        let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy text-replace ok mode="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy text-replace ok mode=apply"); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy text-replace ok mode=apply path="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v127: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v48));
                                                                        let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matches="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v129: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v128));
                                                                        let mut v130: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                                        let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expected="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v132: Rc<str> = Rc::<str>::from(format!("{}{}", v130, v131));
                                                                        let mut v139: Rc<str> = match &v43 {
                                                                            US4::US4_EveryMatch => {
                                                                                let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                                v136.clone()
                                                                            }
                                                                            US4::US4_ExactlyMatches(v134) => {
                                                                                let mut v134: u64 = *v134;
                                                                                let mut v135: Rc<str> = std::rc::Rc::<str>::from((v134).to_string());
                                                                                v135.clone()
                                                                            }
                                                                            US4::US4_ExactlyOneMatch => {
                                                                                let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                                v133.clone()
                                                                            }
                                                                        };
                                                                        let mut v140: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v139));
                                                                        let mut v141: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v140));
                                                                        let mut v142: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                                                        let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v144: Rc<str> = std::rc::Rc::<str>::from((v120).to_string());
                                                                        let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v146: Rc<str> = std::rc::Rc::<str>::from((v121).to_string());
                                                                        let mut v147: Rc<str> = std::rc::Rc::<str>::from([&*v143, &*v144, &*v145, &*v146, &*v108].concat());
                                                                        let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v149: Rc<str> = std::rc::Rc::<str>::from((v142).to_string());
                                                                        let mut v150: Rc<str> = std::rc::Rc::<str>::from([&*v147, &*v148, &*v149, &*v108, &*v108].concat());
                                                                        let mut v151: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v150));
                                                                        US0::US0_ReplacementReceipt(v151.clone())
                                                                    }
                                                                    _ => {
                                                                        let mut v158: Rc<str> = match &v119 {
                                                                            US5::US5_ReadbackMismatch => {
                                                                                let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                                v155.clone()
                                                                            }
                                                                            US5::US5_RenameFailed => {
                                                                                let mut v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                                v154.clone()
                                                                            }
                                                                            US5::US5_TemporaryWriteFailed => {
                                                                                let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                                v153.clone()
                                                                            }
                                                                            _ => unreachable!(),
                                                                        };
                                                                        US0::US0_ReplacementRejected(v158.clone())
                                                                    }
                                                                }
                                                            }
                                                            US2::US2_CheckReplacement => {
                                                                let mut v71: u64 = v59.len() as u64;
                                                                let mut v72: u64 = v70.len() as u64;
                                                                let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy text-replace ok mode="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
                                                                let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy text-replace ok mode=check"); } LIT.with(|lit| lit.clone()) };
                                                                let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy text-replace ok mode=check path="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v48));
                                                                let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matches="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
                                                                let mut v81: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                                let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expected="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                                                                let mut v90: Rc<str> = match &v43 {
                                                                    US4::US4_EveryMatch => {
                                                                        let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                        v87.clone()
                                                                    }
                                                                    US4::US4_ExactlyMatches(v85) => {
                                                                        let mut v85: u64 = *v85;
                                                                        let mut v86: Rc<str> = std::rc::Rc::<str>::from((v85).to_string());
                                                                        v86.clone()
                                                                    }
                                                                    US4::US4_ExactlyOneMatch => {
                                                                        let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                        v84.clone()
                                                                    }
                                                                };
                                                                let mut v91: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v90));
                                                                let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v91));
                                                                let mut v93: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                                                let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v95: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
                                                                let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v97: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
                                                                let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                let mut v99: Rc<str> = std::rc::Rc::<str>::from([&*v94, &*v95, &*v96, &*v97, &*v98].concat());
                                                                let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v101: Rc<str> = std::rc::Rc::<str>::from((v93).to_string());
                                                                let mut v102: Rc<str> = std::rc::Rc::<str>::from([&*v99, &*v100, &*v101, &*v98, &*v98].concat());
                                                                let mut v103: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v102));
                                                                US0::US0_ReplacementReceipt(v103.clone())
                                                            }
                                                        }
                                                    } else {
                                                        let mut v169: Rc<str> = match &v43 {
                                                            US4::US4_EveryMatch => {
                                                                let mut v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                v166.clone()
                                                            }
                                                            US4::US4_ExactlyMatches(v164) => {
                                                                let mut v164: u64 = *v164;
                                                                let mut v165: Rc<str> = std::rc::Rc::<str>::from((v164).to_string());
                                                                v165.clone()
                                                            }
                                                            US4::US4_ExactlyOneMatch => {
                                                                let mut v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                v163.clone()
                                                            }
                                                        };
                                                        let mut v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace expected "); } LIT.with(|lit| lit.clone()) };
                                                        let mut v171: Rc<str> = Rc::<str>::from(format!("{}{}", v170, v169));
                                                        let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" match(es), observed matches="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v173: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v172));
                                                        let mut v174: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                        let mut v175: Rc<str> = Rc::<str>::from(format!("{}{}", v173, v174));
                                                        let mut v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (use --expect N or --all for repeated text)"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v177: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v176));
                                                        US0::US0_ReplacementRejected(v177.clone())
                                                    }
                                                } else {
                                                    let mut v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace target is not a regular file"); } LIT.with(|lit| lit.clone()) };
                                                    US0::US0_ReplacementRejected(v180.clone())
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
fn text_replace_report_0() -> (i32, Rc<str>) {
    let mut v0: US0 = text_replace_outcome_1();
    match &v0 {
        US0::US0_ReplacementReceipt(v1) => {
            let mut v1: Rc<str> = v1.clone();
            (0i32, v1.clone())
        }
        US0::US0_ReplacementRejected(v2) => {
            let mut v2: Rc<str> = v2.clone();
            (2i32, v2.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        text_replace_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_text_replace_outcome() -> (i32, Rc<str>) {
    closure0()()
}
