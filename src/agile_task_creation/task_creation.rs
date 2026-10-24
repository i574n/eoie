#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Valid(Rc<str>),
    US0_Invalid,
}
#[derive(Clone)]
enum US2 {
    US2_AgileEpic,
    US2_AgileFeature,
    US2_AgileStory,
    US2_AgileTaskKind,
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
#[derive(Clone)]
enum US4 {
    US4_AgileP0,
    US4_AgileP1,
    US4_AgileP2,
    US4_AgileP3,
    US4_AgileP4,
    US4_AgileP5,
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US4),
    US3_Invalid,
}
#[derive(Clone)]
enum US6 {
    US6_AgileD0,
    US6_AgileD1,
    US6_AgileD2,
    US6_AgileD3,
    US6_AgileD4,
    US6_AgileD5,
    US6_AgileD6,
    US6_AgileD7,
    US6_AgileD8,
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US6),
    US5_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_ExtrasRead(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US7_ExtrasRefused(Rc<str>),
}
#[derive(Clone)]
enum US9 {
    US9_FlagEstimate,
    US9_FlagLoad,
    US9_FlagAssignee,
    US9_FlagDeps,
    US9_FlagNote,
    US9_FlagAs,
}
#[derive(Clone)]
enum US8 {
    US8_FlagKnown(US9),
    US8_FlagUnknown,
}
#[derive(Clone)]
enum US10 {
    US10_FileReadable(Rc<str>),
    US10_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US11 {
    US11_AddedText(Rc<str>),
    US11_AddedRefused(Rc<str>),
}
#[derive(Clone)]
enum US12 {
    US12_Replaced,
    US12_TemporaryWriteFailed,
    US12_RenameFailed,
    US12_ReadbackMismatch,
}
fn task_id_bytes_allowed_1(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let mut v2: bool = v0 == v1;
        if v2 {
            return true;
        } else {
            let mut v3: i32 = 2i32;
            let mut v4: i32 = usize::try_from(v3).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v0).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
            let mut v5: bool = 65i32 <= v4;
            let mut v7: bool = if v5 {
                let mut v6: bool = v4 <= 90i32;
                v6
            } else {
                false
            };
            let mut v13: bool = if v7 {
                true
            } else {
                let mut v8: bool = 48i32 <= v4;
                let mut v10: bool = if v8 {
                    let mut v9: bool = v4 <= 57i32;
                    v9
                } else {
                    false
                };
                if v10 {
                    true
                } else {
                    let mut v11: bool = v4 == 45i32;
                    v11
                }
            };
            if v13 {
                let mut v14: i32 = v0.wrapping_add(1i32);
                (v0, v1) = (v14, v1);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn flag_of_word_3(mut v0: Rc<str>) -> US8 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v2.to_ascii_lowercase());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v4, &*v4, &*v4].concat());
    let mut v6: bool = v0.clone() == v5.clone();
    if v6 {
        let mut v7: US9 = US9::US9_FlagEstimate;
        US8::US8_FlagKnown(v7.clone())
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
        let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v10, &*v4, &*v4, &*v4].concat());
        let mut v12: bool = v0.clone() == v11.clone();
        if v12 {
            let mut v13: US9 = US9::US9_FlagLoad;
            US8::US8_FlagKnown(v13.clone())
        } else {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from(v15.to_ascii_lowercase());
            let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v16, &*v4, &*v4, &*v4].concat());
            let mut v18: bool = v0.clone() == v17.clone();
            if v18 {
                let mut v19: US9 = US9::US9_FlagAssignee;
                US8::US8_FlagKnown(v19.clone())
            } else {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Deps"); } LIT.with(|lit| lit.clone()) };
                let mut v22: Rc<str> = std::rc::Rc::<str>::from(v21.to_ascii_lowercase());
                let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v22, &*v4, &*v4, &*v4].concat());
                let mut v24: bool = v0.clone() == v23.clone();
                if v24 {
                    let mut v25: US9 = US9::US9_FlagDeps;
                    US8::US8_FlagKnown(v25.clone())
                } else {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note"); } LIT.with(|lit| lit.clone()) };
                    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v27.to_ascii_lowercase());
                    let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v28, &*v4, &*v4, &*v4].concat());
                    let mut v30: bool = v0.clone() == v29.clone();
                    if v30 {
                        let mut v31: US9 = US9::US9_FlagNote;
                        US8::US8_FlagKnown(v31.clone())
                    } else {
                        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                        let mut v34: Rc<str> = std::rc::Rc::<str>::from(v33.to_ascii_lowercase());
                        let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v34, &*v4, &*v4, &*v4].concat());
                        let mut v36: bool = v0.clone() == v35.clone();
                        if v36 {
                            let mut v37: US9 = US9::US9_FlagAs;
                            US8::US8_FlagKnown(v37.clone())
                        } else {
                            US8::US8_FlagUnknown
                        }
                    }
                }
            }
        }
    }
}
fn extras_from_2(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US7 {
    loop {
        let mut v8: bool = v1 <= v0;
        if v8 {
            return US7::US7_ExtrasRead(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone());
        } else {
            let mut v10: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v11: US8 = flag_of_word_3(v10.clone());
            match &v11 {
                US8::US8_FlagKnown(v12) => {
                    let mut v12: US9 = v12.clone();
                    let mut v13: i32 = v0.wrapping_add(1i32);
                    let mut v14: bool = v1 <= v13;
                    if v14 {
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs a value"); } LIT.with(|lit| lit.clone()) };
                        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v15, &*v16, &*v16, &*v16].concat());
                        return US7::US7_ExtrasRefused(v17.clone());
                    } else {
                        let mut v19: i32 = v0.wrapping_add(2i32);
                        let mut v20: Rc<str> = usize::try_from(v13).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                            US9::US9_FlagEstimate => {
                                (v20.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone())
                            }
                            _ => {
                                let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                    US9::US9_FlagLoad => {
                                        (v2.clone(), v20.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone())
                                    }
                                    _ => {
                                        let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                            US9::US9_FlagAssignee => {
                                                (v2.clone(), v3.clone(), v20.clone(), v5.clone(), v6.clone(), v7.clone())
                                            }
                                            _ => {
                                                let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                                    US9::US9_FlagDeps => {
                                                        (v2.clone(), v3.clone(), v4.clone(), v20.clone(), v6.clone(), v7.clone())
                                                    }
                                                    _ => {
                                                        let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                                            US9::US9_FlagNote => {
                                                                let mut v21: Rc<str> = std::rc::Rc::<str>::from(v20.trim());
                                                                (v2.clone(), v3.clone(), v4.clone(), v5.clone(), v21.clone(), v7.clone())
                                                            }
                                                            _ => {
                                                                let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                                                    US9::US9_FlagAs => {
                                                                        let mut v22: Rc<str> = std::rc::Rc::<str>::from(v20.trim());
                                                                        (v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v22.clone())
                                                                    }
                                                                    _ => unreachable!(),
                                                                };
                                                                (v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone())
                                                            }
                                                        };
                                                        (v29.clone(), v30.clone(), v31.clone(), v32.clone(), v33.clone(), v34.clone())
                                                    }
                                                };
                                                (v35.clone(), v36.clone(), v37.clone(), v38.clone(), v39.clone(), v40.clone())
                                            }
                                        };
                                        (v41.clone(), v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46.clone())
                                    }
                                };
                                (v47.clone(), v48.clone(), v49.clone(), v50.clone(), v51.clone(), v52.clone())
                            }
                        };
                        (v0, v1, v2, v3, v4, v5, v6, v7) = (v19, v1, v53.clone(), v54.clone(), v55.clone(), v56.clone(), v57.clone(), v58.clone());
                        continue;
                    }
                }
                US8::US8_FlagUnknown => {
                    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
                    let mut v65: Rc<str> = std::rc::Rc::<str>::from(v64.to_ascii_lowercase());
                    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v67: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v65, &*v66, &*v66, &*v66].concat());
                    let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v67.clone()));
                    let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v68.clone()));
                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
                    let mut v71: Rc<str> = std::rc::Rc::<str>::from(v70.to_ascii_lowercase());
                    let mut v72: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v71, &*v66, &*v66, &*v66].concat());
                    let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v72.clone()));
                    let mut v74: Rc<str> = Rc::<str>::from(format!("{}{}", v69.clone(), v73.clone()));
                    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
                    let mut v76: Rc<str> = std::rc::Rc::<str>::from(v75.to_ascii_lowercase());
                    let mut v77: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v76, &*v66, &*v66, &*v66].concat());
                    let mut v78: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v77.clone()));
                    let mut v79: Rc<str> = Rc::<str>::from(format!("{}{}", v74.clone(), v78.clone()));
                    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Deps"); } LIT.with(|lit| lit.clone()) };
                    let mut v81: Rc<str> = std::rc::Rc::<str>::from(v80.to_ascii_lowercase());
                    let mut v82: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v81, &*v66, &*v66, &*v66].concat());
                    let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v82.clone()));
                    let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", v79.clone(), v83.clone()));
                    let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note"); } LIT.with(|lit| lit.clone()) };
                    let mut v86: Rc<str> = std::rc::Rc::<str>::from(v85.to_ascii_lowercase());
                    let mut v87: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v86, &*v66, &*v66, &*v66].concat());
                    let mut v88: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v87.clone()));
                    let mut v89: Rc<str> = Rc::<str>::from(format!("{}{}", v84.clone(), v88.clone()));
                    let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                    let mut v91: Rc<str> = std::rc::Rc::<str>::from(v90.to_ascii_lowercase());
                    let mut v92: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v91, &*v66, &*v66, &*v66].concat());
                    let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v92.clone()));
                    let mut v94: Rc<str> = Rc::<str>::from(format!("{}{}", v89.clone(), v93.clone()));
                    let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                    let mut v96: Rc<str> = std::rc::Rc::<str>::from([&*v61, &*v10, &*v62, &*v94, &*v95].concat());
                    return US7::US7_ExtrasRefused(v96.clone());
                }
            }
        }
    }
}
fn extras_checked_4(mut v0: US7) -> US7 {
    match &v0 {
        US7::US7_ExtrasRead(v2, v3, v4, v5, v6, v7) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = v6.clone();
            let mut v7: Rc<str> = v7.clone();
            let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v8 {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE_AUTHOR"); } LIT.with(|lit| lit.clone()) };
                let mut v10: Rc<str> = std::env::var(&*v9).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                let mut v11: Rc<str> = std::rc::Rc::<str>::from(v10.trim());
                (v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v11.clone())
            } else {
                (v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone())
            };
            let mut v18: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v18 {
                if v8 {
                    US7::US7_ExtrasRead(v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone())
                } else {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as names the author of --note; give --note too"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_ExtrasRefused(v20.clone())
                }
            } else {
                let mut v23: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v23 {
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a note needs its author: --as <author> (or set EOIE_AUTHOR)"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_ExtrasRefused(v24.clone())
                } else {
                    US7::US7_ExtrasRead(v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone())
                }
            }
        }
        US7::US7_ExtrasRefused(v1) => {
            let mut v1: Rc<str> = v1.clone();
            v0.clone()
        }
    }
}
fn extras_applied_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US11 {
    let (mut v8, mut v9): (i32, Rc<str>) = agile_task_notes::eoie_agile_groomed(&*v0, &*v1, &*v2, &*v3, &*v4, &*v6, &*v7);
    let mut v10: bool = v8 == 0i32;
    if v10 {
        US11::US11_AddedText(v9.clone())
    } else {
        US11::US11_AddedRefused(v9.clone())
    }
}
fn agile_add_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = 0i32;
    let mut v2: i32 = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v1).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
    let mut v3: bool = 48i32 <= v2;
    let mut v5: bool = if v3 {
        let mut v4: bool = v2 <= 57i32;
        v4
    } else {
        false
    };
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must start with a letter, so its inl name is a valid identifier"); } LIT.with(|lit| lit.clone()) };
        let mut v7: i32 = 2i32;
        let mut v8: Rc<str> = usize::try_from(v7).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v8, &*v10, &*v6, &*v11].concat());
        let mut v13: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v12 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v13
    } else {
        let mut v14: i32 = 2i32;
        let mut v15: i32 = usize::try_from(v14).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
        let mut v16: bool = 0 < v15;
        let mut v25: US0 = if v16 {
            let mut v17: i32 = 0i32;
            let mut v18: bool = task_id_bytes_allowed_1(v17, v15);
            if v18 {
                let mut v19: i32 = 2i32;
                let mut v20: Rc<str> = usize::try_from(v19).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                US0::US0_Valid(v20.clone())
            } else {
                US0::US0_Invalid
            }
        } else {
            US0::US0_Invalid
        };
        let mut v26: i32 = 3i32;
        let mut v27: Rc<str> = usize::try_from(v26).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
        let mut v29: bool = v27 == v28 ;
        let mut v33: US1 = if v29 {
            let mut v30: US2 = US2::US2_AgileEpic;
            US1::US1_Valid(v30.clone())
        } else {
            US1::US1_Invalid
        };
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
        let mut v35: bool = v27 == v34 ;
        let mut v38: US1 = if v35 {
            let mut v36: US2 = US2::US2_AgileFeature;
            US1::US1_Valid(v36.clone())
        } else {
            v33.clone()
        };
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
        let mut v40: bool = v27 == v39 ;
        let mut v43: US1 = if v40 {
            let mut v41: US2 = US2::US2_AgileStory;
            US1::US1_Valid(v41.clone())
        } else {
            v38.clone()
        };
        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
        let mut v45: bool = v27 == v44 ;
        let mut v48: US1 = if v45 {
            let mut v46: US2 = US2::US2_AgileTaskKind;
            US1::US1_Valid(v46.clone())
        } else {
            v43.clone()
        };
        let mut v49: i32 = 4i32;
        let mut v50: Rc<str> = usize::try_from(v49).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
        let mut v52: bool = v50 == v51 ;
        let mut v56: US3 = if v52 {
            let mut v53: US4 = US4::US4_AgileP0;
            US3::US3_Valid(v53.clone())
        } else {
            US3::US3_Invalid
        };
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
        let mut v58: bool = v50 == v57 ;
        let mut v61: US3 = if v58 {
            let mut v59: US4 = US4::US4_AgileP1;
            US3::US3_Valid(v59.clone())
        } else {
            v56.clone()
        };
        let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
        let mut v63: bool = v50 == v62 ;
        let mut v66: US3 = if v63 {
            let mut v64: US4 = US4::US4_AgileP2;
            US3::US3_Valid(v64.clone())
        } else {
            v61.clone()
        };
        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
        let mut v68: bool = v50 == v67 ;
        let mut v71: US3 = if v68 {
            let mut v69: US4 = US4::US4_AgileP3;
            US3::US3_Valid(v69.clone())
        } else {
            v66.clone()
        };
        let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
        let mut v73: bool = v50 == v72 ;
        let mut v76: US3 = if v73 {
            let mut v74: US4 = US4::US4_AgileP4;
            US3::US3_Valid(v74.clone())
        } else {
            v71.clone()
        };
        let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
        let mut v78: bool = v50 == v77 ;
        let mut v81: US3 = if v78 {
            let mut v79: US4 = US4::US4_AgileP5;
            US3::US3_Valid(v79.clone())
        } else {
            v76.clone()
        };
        let mut v82: i32 = 5i32;
        let mut v83: Rc<str> = usize::try_from(v82).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
        let mut v85: bool = v83 == v84 ;
        let mut v89: US5 = if v85 {
            let mut v86: US6 = US6::US6_AgileD0;
            US5::US5_Valid(v86.clone())
        } else {
            US5::US5_Invalid
        };
        let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
        let mut v91: bool = v83 == v90 ;
        let mut v94: US5 = if v91 {
            let mut v92: US6 = US6::US6_AgileD1;
            US5::US5_Valid(v92.clone())
        } else {
            v89.clone()
        };
        let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
        let mut v96: bool = v83 == v95 ;
        let mut v99: US5 = if v96 {
            let mut v97: US6 = US6::US6_AgileD2;
            US5::US5_Valid(v97.clone())
        } else {
            v94.clone()
        };
        let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
        let mut v101: bool = v83 == v100 ;
        let mut v104: US5 = if v101 {
            let mut v102: US6 = US6::US6_AgileD3;
            US5::US5_Valid(v102.clone())
        } else {
            v99.clone()
        };
        let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
        let mut v106: bool = v83 == v105 ;
        let mut v109: US5 = if v106 {
            let mut v107: US6 = US6::US6_AgileD4;
            US5::US5_Valid(v107.clone())
        } else {
            v104.clone()
        };
        let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
        let mut v111: bool = v83 == v110 ;
        let mut v114: US5 = if v111 {
            let mut v112: US6 = US6::US6_AgileD5;
            US5::US5_Valid(v112.clone())
        } else {
            v109.clone()
        };
        let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
        let mut v116: bool = v83 == v115 ;
        let mut v119: US5 = if v116 {
            let mut v117: US6 = US6::US6_AgileD6;
            US5::US5_Valid(v117.clone())
        } else {
            v114.clone()
        };
        let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
        let mut v121: bool = v83 == v120 ;
        let mut v124: US5 = if v121 {
            let mut v122: US6 = US6::US6_AgileD7;
            US5::US5_Valid(v122.clone())
        } else {
            v119.clone()
        };
        let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
        let mut v126: bool = v83 == v125 ;
        let mut v129: US5 = if v126 {
            let mut v127: US6 = US6::US6_AgileD8;
            US5::US5_Valid(v127.clone())
        } else {
            v124.clone()
        };
        match &v25 {
            US0::US0_Invalid => {
                let mut v348: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must be uppercase letters, digits and dashes"); } LIT.with(|lit| lit.clone()) };
                let mut v349: i32 = 2i32;
                let mut v350: Rc<str> = usize::try_from(v349).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v351: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                let mut v352: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v353: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v354: Rc<str> = std::rc::Rc::<str>::from([&*v351, &*v350, &*v352, &*v348, &*v353].concat());
                let mut v355: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v354 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v355
            }
            US0::US0_Valid(v130) => {
                let mut v130: Rc<str> = v130.clone();
                match &v48 {
                    US1::US1_Invalid => {
                        let mut v338: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                        let mut v339: i32 = 2i32;
                        let mut v340: Rc<str> = usize::try_from(v339).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v341: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                        let mut v342: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v343: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v344: Rc<str> = std::rc::Rc::<str>::from([&*v341, &*v340, &*v342, &*v338, &*v343].concat());
                        let mut v345: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v344 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v345
                    }
                    US1::US1_Valid(v131) => {
                        let mut v131: US2 = v131.clone();
                        match &v81 {
                            US3::US3_Invalid => {
                                let mut v328: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                                let mut v329: i32 = 2i32;
                                let mut v330: Rc<str> = usize::try_from(v329).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v331: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                let mut v332: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let mut v333: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v334: Rc<str> = std::rc::Rc::<str>::from([&*v331, &*v330, &*v332, &*v328, &*v333].concat());
                                let mut v335: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v334 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v335
                            }
                            US3::US3_Valid(v132) => {
                                let mut v132: US4 = v132.clone();
                                match &v129 {
                                    US5::US5_Invalid => {
                                        let mut v318: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                                        let mut v319: i32 = 2i32;
                                        let mut v320: Rc<str> = usize::try_from(v319).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v321: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                        let mut v322: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                        let mut v323: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v324: Rc<str> = std::rc::Rc::<str>::from([&*v321, &*v320, &*v322, &*v318, &*v323].concat());
                                        let mut v325: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v324 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v325
                                    }
                                    US5::US5_Valid(v133) => {
                                        let mut v133: US6 = v133.clone();
                                        let mut v134: i32 = 8i32;
                                        let mut v135: i32 = std::env::args().skip(2).count() as i32;
                                        let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v137: US7 = extras_from_2(v134, v135, v136.clone(), v136.clone(), v136.clone(), v136.clone(), v136.clone(), v136.clone());
                                        let mut v138: US7 = extras_checked_4(v137.clone());
                                        match &v138 {
                                            US7::US7_ExtrasRead(v139, v140, v141, v142, v143, v144) => {
                                                let mut v139: Rc<str> = v139.clone();
                                                let mut v140: Rc<str> = v140.clone();
                                                let mut v141: Rc<str> = v141.clone();
                                                let mut v142: Rc<str> = v142.clone();
                                                let mut v143: Rc<str> = v143.clone();
                                                let mut v144: Rc<str> = v144.clone();
                                                let mut v145: i32 = 1i32;
                                                let mut v146: Rc<str> = usize::try_from(v145).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let mut v147: i32 = 6i32;
                                                let mut v148: Rc<str> = usize::try_from(v147).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let mut v149: i32 = 7i32;
                                                let mut v150: Rc<str> = usize::try_from(v149).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let mut v151: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                                let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                                let mut v153: Rc<str> = std::rc::Rc::<str>::from([&*v146, &*v152, &*v136, &*v136, &*v136].concat());
                                                let mut v154: bool = std::path::Path::new(&*v153).is_file();
                                                let mut v157: US10 = if v154 {
                                                    US10::US10_FileReadable(v153.clone())
                                                } else {
                                                    US10::US10_FileMissing(v153.clone())
                                                };
                                                match &v157 {
                                                    US10::US10_FileMissing(v158) => {
                                                        let mut v158: Rc<str> = v158.clone();
                                                        let mut v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                        let mut v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                        let mut v162: Rc<str> = std::rc::Rc::<str>::from([&*v160, &*v130, &*v161, &*v159, &*v136].concat());
                                                        let mut v163: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v162 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                        v163
                                                    }
                                                    US10::US10_FileReadable(v164) => {
                                                        let mut v164: Rc<str> = v164.clone();
                                                        let mut v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                                                        let mut v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                                                        let mut v167: Rc<str> = std::rc::Rc::<str>::from([&*v165, &*v130, &*v166, &*v136, &*v136].concat());
                                                        let mut v168: bool = std::fs::read_to_string(&*v153).is_ok_and(|value| value.contains(v167.as_ref()));
                                                        if v168 {
                                                            let mut v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                            let mut v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                            let mut v172: Rc<str> = std::rc::Rc::<str>::from([&*v170, &*v130, &*v171, &*v169, &*v136].concat());
                                                            let mut v173: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v172 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                            v173
                                                        } else {
                                                            let mut v174: Rc<str> = std::fs::read_to_string(&*v164).unwrap_or_default().into();
                                                            let mut v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v176: Rc<str> = std::rc::Rc::<str>::from(v174.trim_end_matches(&*v175));
                                                            let mut v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                                                            let mut v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v180: Rc<str> = std::rc::Rc::<str>::from(v130.replace(&*v178, &v179));
                                                            let mut v181: Rc<str> = std::rc::Rc::<str>::from(v180.to_ascii_lowercase());
                                                            let mut v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Task ("); } LIT.with(|lit| lit.clone()) };
                                                            let mut v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                            let mut v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v186: Rc<str> = std::rc::Rc::<str>::from(v130.replace(&*v184, &v185));
                                                            let mut v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                                            let mut v188: Rc<str> = std::rc::Rc::<str>::from(v186.replace(&*v183, &v187));
                                                            let mut v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v190: Rc<str> = std::rc::Rc::<str>::from(v188.replace(&*v175, &v189));
                                                            let mut v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v193: Rc<str> = std::rc::Rc::<str>::from(v190.replace(&*v191, &v192));
                                                            let mut v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v196: Rc<str> = std::rc::Rc::<str>::from(v193.replace(&*v194, &v195));
                                                            let mut v197: Rc<str> = std::rc::Rc::<str>::from([&*v183, &*v196, &*v183, &*v136, &*v136].concat());
                                                            let mut v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                                            let mut v199: Rc<str> = std::rc::Rc::<str>::from([&*v177, &*v181, &*v182, &*v197, &*v198].concat());
                                                            let mut v200: Rc<str> = std::rc::Rc::<str>::from(v148.replace(&*v184, &v185));
                                                            let mut v201: Rc<str> = std::rc::Rc::<str>::from(v200.replace(&*v183, &v187));
                                                            let mut v202: Rc<str> = std::rc::Rc::<str>::from(v201.replace(&*v175, &v189));
                                                            let mut v203: Rc<str> = std::rc::Rc::<str>::from(v202.replace(&*v191, &v192));
                                                            let mut v204: Rc<str> = std::rc::Rc::<str>::from(v203.replace(&*v194, &v195));
                                                            let mut v205: Rc<str> = std::rc::Rc::<str>::from([&*v183, &*v204, &*v183, &*v136, &*v136].concat());
                                                            let mut v206: Rc<str> = match &v131 {
                                                                US2::US2_AgileEpic => {
                                                                    v28.clone()
                                                                }
                                                                US2::US2_AgileFeature => {
                                                                    v34.clone()
                                                                }
                                                                US2::US2_AgileStory => {
                                                                    v39.clone()
                                                                }
                                                                US2::US2_AgileTaskKind => {
                                                                    v44.clone()
                                                                }
                                                            };
                                                            let mut v207: Rc<str> = match &v132 {
                                                                US4::US4_AgileP0 => {
                                                                    v51.clone()
                                                                }
                                                                US4::US4_AgileP1 => {
                                                                    v57.clone()
                                                                }
                                                                US4::US4_AgileP2 => {
                                                                    v62.clone()
                                                                }
                                                                US4::US4_AgileP3 => {
                                                                    v67.clone()
                                                                }
                                                                US4::US4_AgileP4 => {
                                                                    v72.clone()
                                                                }
                                                                US4::US4_AgileP5 => {
                                                                    v77.clone()
                                                                }
                                                            };
                                                            let mut v208: Rc<str> = std::rc::Rc::<str>::from([&*v205, &*v198, &*v206, &*v198, &*v207].concat());
                                                            let mut v209: Rc<str> = match &v133 {
                                                                US6::US6_AgileD0 => {
                                                                    v84.clone()
                                                                }
                                                                US6::US6_AgileD1 => {
                                                                    v90.clone()
                                                                }
                                                                US6::US6_AgileD2 => {
                                                                    v95.clone()
                                                                }
                                                                US6::US6_AgileD3 => {
                                                                    v100.clone()
                                                                }
                                                                US6::US6_AgileD4 => {
                                                                    v105.clone()
                                                                }
                                                                US6::US6_AgileD5 => {
                                                                    v110.clone()
                                                                }
                                                                US6::US6_AgileD6 => {
                                                                    v115.clone()
                                                                }
                                                                US6::US6_AgileD7 => {
                                                                    v120.clone()
                                                                }
                                                                US6::US6_AgileD8 => {
                                                                    v125.clone()
                                                                }
                                                            };
                                                            let mut v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", 0u32, Planned, "); } LIT.with(|lit| lit.clone()) };
                                                            let mut v211: Rc<str> = std::rc::Rc::<str>::from(v142.replace(&*v184, &v185));
                                                            let mut v212: Rc<str> = std::rc::Rc::<str>::from(v211.replace(&*v183, &v187));
                                                            let mut v213: Rc<str> = std::rc::Rc::<str>::from(v212.replace(&*v175, &v189));
                                                            let mut v214: Rc<str> = std::rc::Rc::<str>::from(v213.replace(&*v191, &v192));
                                                            let mut v215: Rc<str> = std::rc::Rc::<str>::from(v214.replace(&*v194, &v195));
                                                            let mut v216: Rc<str> = std::rc::Rc::<str>::from([&*v183, &*v215, &*v183, &*v136, &*v136].concat());
                                                            let mut v217: Rc<str> = std::rc::Rc::<str>::from(v150.replace(&*v184, &v185));
                                                            let mut v218: Rc<str> = std::rc::Rc::<str>::from(v217.replace(&*v183, &v187));
                                                            let mut v219: Rc<str> = std::rc::Rc::<str>::from(v218.replace(&*v175, &v189));
                                                            let mut v220: Rc<str> = std::rc::Rc::<str>::from(v219.replace(&*v191, &v192));
                                                            let mut v221: Rc<str> = std::rc::Rc::<str>::from(v220.replace(&*v194, &v195));
                                                            let mut v222: Rc<str> = std::rc::Rc::<str>::from([&*v183, &*v221, &*v183, &*v136, &*v136].concat());
                                                            let mut v223: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", \"current\", \"current\")"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v224: Rc<str> = std::rc::Rc::<str>::from([&*v198, &*v222, &*v223, &*v136, &*v136].concat());
                                                            let mut v225: Rc<str> = std::rc::Rc::<str>::from([&*v198, &*v209, &*v210, &*v216, &*v224].concat());
                                                            let mut v226: Rc<str> = std::rc::Rc::<str>::from([&*v199, &*v208, &*v225, &*v136, &*v136].concat());
                                                            let mut v227: Rc<str> = std::rc::Rc::<str>::from([&*v176, &*v175, &*v226, &*v175, &*v136].concat());
                                                            let mut v228: US11 = extras_applied_5(v227.clone(), v130.clone(), v139.clone(), v140.clone(), v141.clone(), v142.clone(), v143.clone(), v144.clone());
                                                            match &v228 {
                                                                US11::US11_AddedRefused(v229) => {
                                                                    let mut v229: Rc<str> = v229.clone();
                                                                    let mut v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                    let mut v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                    let mut v232: Rc<str> = std::rc::Rc::<str>::from([&*v230, &*v130, &*v231, &*v229, &*v136].concat());
                                                                    let mut v233: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v232 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                    v233
                                                                }
                                                                US11::US11_AddedText(v234) => {
                                                                    let mut v234: Rc<str> = v234.clone();
                                                                    let mut v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                                                                    let mut v236: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
                                                                    let mut v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                                                                    let mut v238: Rc<str> = std::rc::Rc::<str>::from([&*v153, &*v235, &*v236, &*v237, &*v136].concat());
                                                                    let mut v239: bool = std::fs::write(&*v238, &*v234).is_ok();
                                                                    let mut v248: US12 = if v239 {
                                                                        let mut v240: bool = std::fs::rename(&*v238, &*v153).is_ok();
                                                                        if v240 {
                                                                            let mut v241: bool = std::fs::read_to_string(&*v153).is_ok_and(|value| value == *v234);
                                                                            if v241 {
                                                                                US12::US12_Replaced
                                                                            } else {
                                                                                US12::US12_ReadbackMismatch
                                                                            }
                                                                        } else {
                                                                            US12::US12_RenameFailed
                                                                        }
                                                                    } else {
                                                                        US12::US12_TemporaryWriteFailed
                                                                    };
                                                                    match &v248 {
                                                                        US12::US12_Replaced => {
                                                                            let mut v249: Rc<str> = std::rc::Rc::<str>::from(v130.replace(&*v178, &v179));
                                                                            let mut v250: Rc<str> = std::rc::Rc::<str>::from(v249.to_ascii_lowercase());
                                                                            let mut v251: u64 = v174.len() as u64;
                                                                            let mut v252: u64 = v234.len() as u64;
                                                                            let mut v253: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v151);
                                                                            let mut v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile add ok id="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v255: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" binding="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status=Planned progress=0"); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v257: bool = v142.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v260: Rc<str> = if v257 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let mut v258: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deps="); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v259: Rc<str> = std::rc::Rc::<str>::from([&*v258, &*v142, &*v136, &*v136, &*v136].concat());
                                                                                v259.clone()
                                                                            };
                                                                            let mut v261: bool = v139.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v263: Rc<str> = if v261 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let mut v262: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" estimate"); } LIT.with(|lit| lit.clone()) };
                                                                                v262.clone()
                                                                            };
                                                                            let mut v264: bool = v140.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v266: Rc<str> = if v264 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let mut v265: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load"); } LIT.with(|lit| lit.clone()) };
                                                                                v265.clone()
                                                                            };
                                                                            let mut v267: bool = v141.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v269: Rc<str> = if v267 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let mut v268: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee"); } LIT.with(|lit| lit.clone()) };
                                                                                v268.clone()
                                                                            };
                                                                            let mut v270: Rc<str> = std::rc::Rc::<str>::from([&*v263, &*v266, &*v269, &*v136, &*v136].concat());
                                                                            let mut v271: bool = v270.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v278: Rc<str> = if v271 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let mut v272: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" planned="); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v273: Rc<str> = std::rc::Rc::<str>::from(v270.trim());
                                                                                let mut v274: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v275: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v276: Rc<str> = std::rc::Rc::<str>::from(v273.replace(&*v274, &v275));
                                                                                let mut v277: Rc<str> = std::rc::Rc::<str>::from([&*v272, &*v276, &*v136, &*v136, &*v136].concat());
                                                                                v277.clone()
                                                                            };
                                                                            let mut v279: bool = v143.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v281: Rc<str> = if v279 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let mut v280: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" notes=1"); } LIT.with(|lit| lit.clone()) };
                                                                                v280.clone()
                                                                            };
                                                                            let mut v282: Rc<str> = std::rc::Rc::<str>::from([&*v260, &*v278, &*v281, &*v136, &*v136].concat());
                                                                            let mut v283: Rc<str> = std::rc::Rc::<str>::from([&*v256, &*v282, &*v136, &*v136, &*v136].concat());
                                                                            let mut v284: Rc<str> = std::rc::Rc::<str>::from([&*v254, &*v130, &*v255, &*v250, &*v283].concat());
                                                                            let mut v285: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v286: Rc<str> = std::rc::Rc::<str>::from((v251).to_string());
                                                                            let mut v287: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v288: Rc<str> = std::rc::Rc::<str>::from((v252).to_string());
                                                                            let mut v289: Rc<str> = std::rc::Rc::<str>::from([&*v285, &*v286, &*v287, &*v288, &*v136].concat());
                                                                            let mut v290: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v291: Rc<str> = std::rc::Rc::<str>::from((v253).to_string());
                                                                            let mut v292: Rc<str> = std::rc::Rc::<str>::from([&*v289, &*v290, &*v291, &*v136, &*v136].concat());
                                                                            let mut v293: Rc<str> = std::rc::Rc::<str>::from([&*v284, &*v292, &*v136, &*v136, &*v136].concat());
                                                                            let mut v294: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v293.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                                                            v294
                                                                        }
                                                                        _ => {
                                                                            let mut v300: Rc<str> = match &v248 {
                                                                                US12::US12_ReadbackMismatch => {
                                                                                    let mut v297: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v297.clone()
                                                                                }
                                                                                US12::US12_RenameFailed => {
                                                                                    let mut v296: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                                    v296.clone()
                                                                                }
                                                                                US12::US12_TemporaryWriteFailed => {
                                                                                    let mut v295: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v295.clone()
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            let mut v301: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v302: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v303: Rc<str> = std::rc::Rc::<str>::from([&*v301, &*v130, &*v302, &*v300, &*v136].concat());
                                                                            let mut v304: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v303 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                            v304
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            US7::US7_ExtrasRefused(v311) => {
                                                let mut v311: Rc<str> = v311.clone();
                                                let mut v312: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                let mut v313: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                let mut v314: Rc<str> = std::rc::Rc::<str>::from([&*v312, &*v130, &*v313, &*v311, &*v136].concat());
                                                let mut v315: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v314 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                v315
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
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_add_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_add_run() -> i32 {
    closure0()()
}
