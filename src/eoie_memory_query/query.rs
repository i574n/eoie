#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_KindUser,
    US0_KindFeedback,
    US0_KindProject,
    US0_KindReference,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoMemories,
    UH0_MemoryThen(Rc<str>, Rc<str>, Rc<str>, US0, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_EntryRead(Rc<str>, Rc<str>, Rc<str>, US0, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US1_EntryRefused(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_KindKnown(US0),
    US2_KindUnknown,
}
#[derive(Clone)]
enum UH1 {
    UH1_NoRanked,
    UH1_RankedThen(i32, Rc<str>, Rc<str>, Rc<str>, US0, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
fn file_present_1(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn memories_flipped_3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_MemoryThen(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: US0 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<str> = v7.clone();
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<UH0> = v10.clone();
                let mut v11: Rc<UH0> = Rc::new(UH0::UH0_MemoryThen(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v1.clone()));
                (v0, v1) = (v10.clone(), v11.clone());
                continue;
            }
            UH0::UH0_NoMemories => {
                return v1.clone();
            }
        }
    }
}
fn kind_of_case_5(mut v0: Rc<str>) -> US2 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US0 = US0::US0_KindUser;
        US2::US2_KindKnown(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US0 = US0::US0_KindFeedback;
            US2::US2_KindKnown(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US0 = US0::US0_KindProject;
                US2::US2_KindKnown(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US0 = US0::US0_KindReference;
                    US2::US2_KindKnown(v11.clone())
                } else {
                    US2::US2_KindUnknown
                }
            }
        }
    }
}
fn unescaped_from_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> (Rc<str>, i32) {
    loop {
        let mut v4: i32 = (v0).as_bytes().get(v1 as usize).map(|byte| *byte as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0i32;
        let mut v7: bool = if v5 {
            true
        } else {
            let mut v6: bool = v4 == 34i32;
            v6
        };
        if v7 {
            let mut v8: Rc<str> = std::rc::Rc::<str>::from((v0).get(v2 as usize..v1 as usize).unwrap_or(""));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v8, &*v9, &*v9, &*v9].concat());
            let mut v11: i32 = v1.wrapping_add(1i32);
            return (v10.clone(), v11);
        } else {
            let mut v12: bool = v4 == 92i32;
            if v12 {
                let mut v13: i32 = v1.wrapping_add(1i32);
                let mut v14: i32 = (v0).as_bytes().get(v13 as usize).map(|byte| *byte as i32).unwrap_or(-1);
                let mut v15: Rc<str> = std::rc::Rc::<str>::from((v0).get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v15, &*v16, &*v16, &*v16].concat());
                let mut v18: bool = v14 == 110i32;
                if v18 {
                    let mut v19: i32 = v1.wrapping_add(2i32);
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v20, &*v16, &*v16, &*v16].concat());
                    (v0, v1, v2, v3) = (v0.clone(), v19, v19, v21.clone());
                    continue;
                } else {
                    let mut v24: bool = v14 == 114i32;
                    if v24 {
                        let mut v25: i32 = v1.wrapping_add(2i32);
                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v26, &*v16, &*v16, &*v16].concat());
                        (v0, v1, v2, v3) = (v0.clone(), v25, v25, v27.clone());
                        continue;
                    } else {
                        let mut v30: bool = v14 == 116i32;
                        if v30 {
                            let mut v31: i32 = v1.wrapping_add(2i32);
                            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v32, &*v16, &*v16, &*v16].concat());
                            (v0, v1, v2, v3) = (v0.clone(), v31, v31, v33.clone());
                            continue;
                        } else {
                            let mut v36: i32 = v1.wrapping_add(2i32);
                            (v0, v1, v2, v3) = (v0.clone(), v36, v13, v17.clone());
                            continue;
                        }
                    }
                }
            } else {
                let mut v45: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v45, v2, v3.clone());
                continue;
            }
        }
    }
}
fn quoted_at_6(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, i32) {
    let mut v2: bool = v1 < 0i32;
    let mut v5: i32 = if v2 {
        -1i32
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = (v0).get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|at| at as i32 + v1).unwrap_or(-1);
        v4
    };
    let mut v6: bool = v5 < 0i32;
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v7.clone(), -1i32)
    } else {
        let mut v8: i32 = v5.wrapping_add(1i32);
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        unescaped_from_7(v0.clone(), v8, v8, v9.clone())
    }
}
fn entry_of_line_4(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= Memory ("); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = (v0).get(0i32 as usize..).and_then(|rest| rest.find(&*v1)).map(|at| at as i32 + 0i32).unwrap_or(-1);
    let mut v3: i32 = v2.wrapping_add(10i32);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = (v0).get(v3 as usize..).and_then(|rest| rest.find(&*v4)).map(|at| at as i32 + v3).unwrap_or(-1);
    let mut v6: Rc<str> = std::rc::Rc::<str>::from((v0).get(v3 as usize..v5 as usize).unwrap_or(""));
    let mut v7: US2 = kind_of_case_5(v6.clone());
    match &v7 {
        US2::US2_KindKnown(v13) => {
            let mut v13: US0 = v13.clone();
            let (mut v14, mut v15): (Rc<str>, i32) = quoted_at_6(v0.clone(), v5);
            let (mut v16, mut v17): (Rc<str>, i32) = quoted_at_6(v0.clone(), v15);
            let (mut v18, mut v19): (Rc<str>, i32) = quoted_at_6(v0.clone(), v17);
            let (mut v20, mut v21): (Rc<str>, i32) = quoted_at_6(v0.clone(), v19);
            let (mut v22, mut v23): (Rc<str>, i32) = quoted_at_6(v0.clone(), v21);
            let (mut v24, mut v25): (Rc<str>, i32) = quoted_at_6(v0.clone(), v23);
            let (mut v26, mut v27): (Rc<str>, i32) = quoted_at_6(v0.clone(), v25);
            let mut v28: bool = v27 < 0i32;
            if v28 {
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed memory line: "); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = { let text = v0; if text.chars().count() <= 80i32 as usize { text } else { std::rc::Rc::<str>::from(text.chars().take((80i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v30, &*v31, &*v31, &*v31].concat());
                US1::US1_EntryRefused(v32.clone())
            } else {
                US1::US1_EntryRead(v14.clone(), v16.clone(), v18.clone(), v13.clone(), v20.clone(), v22.clone(), v24.clone(), v26.clone())
            }
        }
        US2::US2_KindUnknown => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown memory kind in: "); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = { let text = v0; if text.chars().count() <= 80i32 as usize { text } else { std::rc::Rc::<str>::from(text.chars().take((80i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v10, &*v10, &*v10].concat());
            US1::US1_EntryRefused(v11.clone())
        }
    }
}
fn store_from_2(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>) -> (Rc<UH0>, i32, Rc<str>, Rc<str>) {
    let mut v6: i32 = (v0.clone().len() as i32);
    let mut v7: bool = v1 >= v6;
    let (mut v60, mut v61, mut v62, mut v63): (Rc<UH0>, i32, Rc<str>, Rc<str>) = if v7 {
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<UH0> = memories_flipped_3(v2.clone(), v8.clone());
        (v9.clone(), v3, v4.clone(), v5.clone())
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v11: i32 = (v0).get(v1 as usize..).and_then(|rest| rest.find(&*v10)).map(|at| at as i32 + v1).unwrap_or(-1);
        let mut v12: bool = v11 < 0i32;
        let mut v13: i32 = if v12 {
            v6
        } else {
            v11
        };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize..v13 as usize).unwrap_or(""));
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl memory_"); } LIT.with(|lit| lit.clone()) };
        let mut v16: bool = v14.starts_with(&*v15);
        let mut v19: bool = if v16 {
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= Memory ("); } LIT.with(|lit| lit.clone()) };
            let mut v18: bool = v14.contains(&*v17);
            v18
        } else {
            false
        };
        let (mut v51, mut v52, mut v53, mut v54): (Rc<UH0>, i32, Rc<str>, Rc<str>) = if v19 {
            let mut v20: US1 = entry_of_line_4(v14.clone());
            let (mut v38, mut v39, mut v40, mut v41): (Rc<UH0>, i32, Rc<str>, Rc<str>) = match &v20 {
                US1::US1_EntryRead(v21, v22, v23, v24, v25, v26, v27, v28) => {
                    let mut v21: Rc<str> = v21.clone();
                    let mut v22: Rc<str> = v22.clone();
                    let mut v23: Rc<str> = v23.clone();
                    let mut v24: US0 = v24.clone();
                    let mut v25: Rc<str> = v25.clone();
                    let mut v26: Rc<str> = v26.clone();
                    let mut v27: Rc<str> = v27.clone();
                    let mut v28: Rc<str> = v28.clone();
                    let mut v29: i32 = v3.wrapping_add(1i32);
                    let mut v30: Rc<UH0> = Rc::new(UH0::UH0_MemoryThen(v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone(), v2.clone()));
                    (v30.clone(), v29, v4.clone(), v5.clone())
                }
                _ => {
                    let (mut v34, mut v35, mut v36, mut v37): (Rc<UH0>, i32, Rc<str>, Rc<str>) = match &v20 {
                        US1::US1_EntryRefused(v31) => {
                            let mut v31: Rc<str> = v31.clone();
                            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v31, &*v10, &*v32, &*v32].concat());
                            (v2.clone(), v3, v4.clone(), v33.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v34.clone(), v35, v36.clone(), v37.clone())
                }
            };
            (v38.clone(), v39, v40.clone(), v41.clone())
        } else {
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl index_of_memory () = MemoryIndex ("); } LIT.with(|lit| lit.clone()) };
            let mut v43: bool = v14.starts_with(&*v42);
            let (mut v47, mut v48, mut v49, mut v50): (Rc<UH0>, i32, Rc<str>, Rc<str>) = if v43 {
                let mut v44: i32 = 0i32;
                let (mut v45, mut v46): (Rc<str>, i32) = quoted_at_6(v14.clone(), v44);
                (v2.clone(), v3, v45.clone(), v5.clone())
            } else {
                (v2.clone(), v3, v4.clone(), v5.clone())
            };
            (v47.clone(), v48, v49.clone(), v50.clone())
        };
        let mut v55: i32 = v13.wrapping_add(1i32);
        let (mut v56, mut v57, mut v58, mut v59): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_2(v0.clone(), v55, v51.clone(), v52, v53.clone(), v54.clone());
        (v56.clone(), v57, v58.clone(), v59.clone())
    };
    (v60.clone(), v61, v62.clone(), v63.clone())
}
fn slug_lines_8(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_MemoryThen(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: US0 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<str> = v7.clone();
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<UH0> = v10.clone();
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v11, &*v12, &*v12].concat());
                (v0, v1) = (v10.clone(), v13.clone());
                continue;
            }
            UH0::UH0_NoMemories => {
                return v1.clone();
            }
        }
    }
}
fn dangling_in_10(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[["); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = (v0).get(v1 as usize..).and_then(|rest| rest.find(&*v5)).map(|at| at as i32 + v1).unwrap_or(-1);
        let mut v7: bool = v6 < 0i32;
        let mut v10: i32 = if v7 {
            -1i32
        } else {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]]"); } LIT.with(|lit| lit.clone()) };
            let mut v9: i32 = (v0).get(v6 as usize..).and_then(|rest| rest.find(&*v8)).map(|at| at as i32 + v6).unwrap_or(-1);
            v9
        };
        let mut v11: bool = v10 < 0i32;
        if v11 {
            return v4.clone();
        } else {
            let mut v12: i32 = v6.wrapping_add(2i32);
            let mut v13: Rc<str> = std::rc::Rc::<str>::from((v0).get(v12 as usize..v10 as usize).unwrap_or(""));
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v13, &*v14, &*v15, &*v15].concat());
            let mut v17: bool = v2.contains(&*v16);
            let mut v18: i32 = v10.wrapping_add(2i32);
            let mut v22: Rc<str> = if v17 {
                v4.clone()
            } else {
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": dangling [["); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]]\n"); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v3, &*v19, &*v13, &*v20].concat());
                v21.clone()
            };
            (v0, v1, v2, v3, v4) = (v0.clone(), v18, v2.clone(), v3.clone(), v22.clone());
            continue;
        }
    }
}
fn checked_9(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: i32) -> (Rc<str>, i32, i32, i32) {
    let (mut v61, mut v62, mut v63, mut v64): (Rc<str>, i32, i32, i32) = match &*v0 {
        UH0::UH0_NoMemories => {
            (v3.clone(), v4, v5, v6)
        }
        _ => {
            let (mut v57, mut v58, mut v59, mut v60): (Rc<str>, i32, i32, i32) = match &*v0 {
                UH0::UH0_MemoryThen(v7, v8, v9, v10, v11, v12, v13, v14, v15) => {
                    let mut v7: Rc<str> = v7.clone();
                    let mut v8: Rc<str> = v8.clone();
                    let mut v9: Rc<str> = v9.clone();
                    let mut v10: US0 = v10.clone();
                    let mut v11: Rc<str> = v11.clone();
                    let mut v12: Rc<str> = v12.clone();
                    let mut v13: Rc<str> = v13.clone();
                    let mut v14: Rc<str> = v14.clone();
                    let mut v15: Rc<UH0> = v15.clone();
                    let mut v16: i32 = 0i32;
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v18: Rc<str> = dangling_in_10(v14.clone(), v16, v1.clone(), v7.clone(), v17.clone());
                    let mut v19: i32 = (v18).lines().count() as i32;
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**Why:**"); } LIT.with(|lit| lit.clone()) };
                    let mut v21: bool = v14.contains(&*v20);
                    let mut v24: bool = if v21 {
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**How to apply:**"); } LIT.with(|lit| lit.clone()) };
                        let mut v23: bool = v14.contains(&*v22);
                        v23
                    } else {
                        false
                    };
                    let mut v26: bool = match &v10 {
                        US0::US0_KindFeedback => {
                            true
                        }
                        US0::US0_KindProject => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let mut v28: bool = if v26 {
                        let mut v27: bool = v24 == false;
                        v27
                    } else {
                        false
                    };
                    let mut v38: Rc<str> = if v28 {
                        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
                        let mut v34: Rc<str> = match &v10 {
                            US0::US0_KindFeedback => {
                                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                                v31.clone()
                            }
                            US0::US0_KindProject => {
                                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                                v32.clone()
                            }
                            US0::US0_KindReference => {
                                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                                v33.clone()
                            }
                            US0::US0_KindUser => {
                                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                                v30.clone()
                            }
                        };
                        let mut v35: Rc<str> = std::rc::Rc::<str>::from((v34).to_ascii_lowercase().as_str());
                        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" without **Why:** and **How to apply:**\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v29, &*v35, &*v36, &*v17].concat());
                        v37.clone()
                    } else {
                        v17.clone()
                    };
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]("); } LIT.with(|lit| lit.clone()) };
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                    let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v7, &*v40, &*v17, &*v17].concat());
                    let mut v42: bool = v2.contains(&*v41);
                    let mut v45: Rc<str> = if v42 {
                        v17.clone()
                    } else {
                        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": missing from MEMORY.md\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v43, &*v17, &*v17, &*v17].concat());
                        v44.clone()
                    };
                    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v18, &*v38, &*v45, &*v17].concat());
                    let mut v47: i32 = v4.wrapping_add(v19);
                    let mut v48: bool = v38.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v49: i32 = if v48 {
                        0i32
                    } else {
                        1i32
                    };
                    let mut v50: i32 = v5.wrapping_add(v49);
                    let mut v51: i32 = if v42 {
                        0i32
                    } else {
                        1i32
                    };
                    let mut v52: i32 = v6.wrapping_add(v51);
                    checked_9(v15.clone(), v1.clone(), v2.clone(), v46.clone(), v47, v50, v52)
                }
                _ => unreachable!(),
            };
            (v57.clone(), v58, v59, v60)
        }
    };
    (v61.clone(), v62, v63, v64)
}
fn file_lines_11(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_MemoryThen(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: US0 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<str> = v7.clone();
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<UH0> = v10.clone();
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v11, &*v12, &*v12].concat());
                (v0, v1) = (v10.clone(), v13.clone());
                continue;
            }
            UH0::UH0_NoMemories => {
                return v1.clone();
            }
        }
    }
}
fn index_targets_missing_12(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]("); } LIT.with(|lit| lit.clone()) };
        let mut v5: i32 = (v0).get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|at| at as i32 + v1).unwrap_or(-1);
        let mut v6: bool = v5 < 0i32;
        let mut v9: i32 = if v6 {
            -1i32
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = (v0).get(v5 as usize..).and_then(|rest| rest.find(&*v7)).map(|at| at as i32 + v5).unwrap_or(-1);
            v8
        };
        let mut v10: bool = v9 < 0i32;
        if v10 {
            return v3.clone();
        } else {
            let mut v11: i32 = v5.wrapping_add(2i32);
            let mut v12: Rc<str> = std::rc::Rc::<str>::from((v0).get(v11 as usize..v9 as usize).unwrap_or(""));
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v12, &*v13, &*v14, &*v14].concat());
            let mut v16: bool = v2.contains(&*v15);
            let mut v17: i32 = v9.wrapping_add(1i32);
            let mut v21: bool = if v16 {
                true
            } else {
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".md"); } LIT.with(|lit| lit.clone()) };
                let mut v19: bool = v12.ends_with(&*v18);
                let mut v20: bool = v19 == false;
                v20
            };
            let mut v24: Rc<str> = if v21 {
                v3.clone()
            } else {
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MEMORY.md: entry for missing "); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v22, &*v12, &*v13, &*v14].concat());
                v23.clone()
            };
            (v0, v1, v2, v3) = (v0.clone(), v17, v2.clone(), v24.clone());
            continue;
        }
    }
}
fn memory_check_0(mut v0: Rc<str>) -> i32 {
    let mut v1: bool = file_present_1(v0.clone());
    let mut v2: bool = v1 == false;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check needs a readable data file: "); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v0, &*v4, &*v4, &*v4].concat());
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v5, &*v4, &*v4, &*v4].concat());
        eprintln!("{}", &*v7);
        2i32
    } else {
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
        let mut v9: i32 = 0i32;
        let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
        let mut v11: i32 = 0i32;
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let (mut v13, mut v14, mut v15, mut v16): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_2(v8.clone(), v9, v10.clone(), v11, v12.clone(), v12.clone());
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = slug_lines_8(v13.clone(), v17.clone());
        let mut v19: i32 = 0i32;
        let mut v20: i32 = 0i32;
        let mut v21: i32 = 0i32;
        let (mut v22, mut v23, mut v24, mut v25): (Rc<str>, i32, i32, i32) = checked_9(v13.clone(), v18.clone(), v15.clone(), v12.clone(), v19, v20, v21);
        let mut v26: i32 = 0i32;
        let mut v27: Rc<str> = file_lines_11(v13.clone(), v17.clone());
        let mut v28: Rc<str> = index_targets_missing_12(v15.clone(), v26, v27.clone(), v12.clone());
        let mut v29: i32 = (v28).lines().count() as i32;
        let mut v30: i32 = v24.wrapping_add(v25);
        let mut v31: i32 = v30.wrapping_add(v29);
        let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v28, &*v16, &*v12, &*v12].concat());
        print!("{}", &*v32);
        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" memories="); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = std::rc::Rc::<str>::from((v14).to_string().as_str());
        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" dangling_links="); } LIT.with(|lit| lit.clone()) };
        let mut v36: Rc<str> = std::rc::Rc::<str>::from((v23).to_string().as_str());
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" missing_reasons="); } LIT.with(|lit| lit.clone()) };
        let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v36, &*v37].concat());
        let mut v39: bool = v31 == 0i32;
        let mut v42: Rc<str> = if v39 {
            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie memory check ok"); } LIT.with(|lit| lit.clone()) };
            v40.clone()
        } else {
            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie memory check failed"); } LIT.with(|lit| lit.clone()) };
            v41.clone()
        };
        let mut v43: Rc<str> = std::rc::Rc::<str>::from((v24).to_string().as_str());
        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unindexed="); } LIT.with(|lit| lit.clone()) };
        let mut v45: Rc<str> = std::rc::Rc::<str>::from((v25).to_string().as_str());
        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" index_missing="); } LIT.with(|lit| lit.clone()) };
        let mut v47: Rc<str> = std::rc::Rc::<str>::from((v29).to_string().as_str());
        let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v45, &*v46, &*v47, &*v12].concat());
        let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v38, &*v43, &*v48, &*v12].concat());
        println!("{}", &*v49);
        if v39 {
            0i32
        } else {
            1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        memory_check_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn kind_of_word_14(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v1).to_ascii_lowercase().as_str());
    let mut v3: bool = v0.clone() == v2.clone();
    if v3 {
        let mut v4: US0 = US0::US0_KindUser;
        US2::US2_KindKnown(v4.clone())
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from((v6).to_ascii_lowercase().as_str());
        let mut v8: bool = v0.clone() == v7.clone();
        if v8 {
            let mut v9: US0 = US0::US0_KindFeedback;
            US2::US2_KindKnown(v9.clone())
        } else {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from((v11).to_ascii_lowercase().as_str());
            let mut v13: bool = v0.clone() == v12.clone();
            if v13 {
                let mut v14: US0 = US0::US0_KindProject;
                US2::US2_KindKnown(v14.clone())
            } else {
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = std::rc::Rc::<str>::from((v16).to_ascii_lowercase().as_str());
                let mut v18: bool = v0.clone() == v17.clone();
                if v18 {
                    let mut v19: US0 = US0::US0_KindReference;
                    US2::US2_KindKnown(v19.clone())
                } else {
                    US2::US2_KindUnknown
                }
            }
        }
    }
}
fn listed_15(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_MemoryThen(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let mut v6: US0 = v6.clone();
                let mut v7: Rc<str> = v7.clone();
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<str> = v10.clone();
                let mut v11: Rc<UH0> = v11.clone();
                let mut v12: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v20: bool = if v12 {
                    true
                } else {
                    let mut v17: Rc<str> = match &v6 {
                        US0::US0_KindFeedback => {
                            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                            v14.clone()
                        }
                        US0::US0_KindProject => {
                            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                            v15.clone()
                        }
                        US0::US0_KindReference => {
                            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                            v16.clone()
                        }
                        US0::US0_KindUser => {
                            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                            v13.clone()
                        }
                    };
                    let mut v18: Rc<str> = std::rc::Rc::<str>::from((v17).to_ascii_lowercase().as_str());
                    let mut v19: bool = v1.clone() == v18.clone();
                    v19
                };
                if v20 {
                    let mut v25: Rc<str> = match &v6 {
                        US0::US0_KindFeedback => {
                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        }
                        US0::US0_KindProject => {
                            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                            v23.clone()
                        }
                        US0::US0_KindReference => {
                            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                            v24.clone()
                        }
                        US0::US0_KindUser => {
                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                            v21.clone()
                        }
                    };
                    let mut v26: Rc<str> = std::rc::Rc::<str>::from((v25).to_ascii_lowercase().as_str());
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                    let mut v28: Rc<str> = { let text = v5; if text.chars().count() <= 100i32 as usize { text } else { std::rc::Rc::<str>::from(text.chars().take((100i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
                    let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v27, &*v4, &*v27, &*v28].concat());
                    println!("{}", &*v29);
                    let mut v30: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2) = (v11.clone(), v1.clone(), v30);
                    continue;
                } else {
                    (v0, v1, v2) = (v11.clone(), v1.clone(), v2);
                    continue;
                }
            }
            UH0::UH0_NoMemories => {
                return v2;
            }
        }
    }
}
fn memory_list_run_13(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: bool = file_present_1(v0.clone());
    let mut v3: bool = v2 == false;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list needs a readable data file: "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v0, &*v5, &*v5, &*v5].concat());
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v6, &*v5, &*v5, &*v5].concat());
        eprintln!("{}", &*v8);
        2i32
    } else {
        let mut v9: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v13: US2 = if v9 {
            let mut v10: US0 = US0::US0_KindUser;
            US2::US2_KindKnown(v10.clone())
        } else {
            kind_of_word_14(v1.clone())
        };
        match &v13 {
            US2::US2_KindKnown(v36) => {
                let mut v36: US0 = v36.clone();
                let mut v37: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
                let mut v38: i32 = 0i32;
                let mut v39: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
                let mut v40: i32 = 0i32;
                let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let (mut v42, mut v43, mut v44, mut v45): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_2(v37.clone(), v38, v39.clone(), v40, v41.clone(), v41.clone());
                let mut v46: i32 = 0i32;
                let mut v47: i32 = listed_15(v42.clone(), v1.clone(), v46);
                let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie memory list ok shown="); } LIT.with(|lit| lit.clone()) };
                let mut v49: Rc<str> = std::rc::Rc::<str>::from((v47).to_string().as_str());
                let mut v52: Rc<str> = if v9 {
                    v41.clone()
                } else {
                    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" kind="); } LIT.with(|lit| lit.clone()) };
                    let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v50, &*v1, &*v41, &*v41, &*v41].concat());
                    v51.clone()
                };
                let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v49, &*v52, &*v41, &*v41].concat());
                println!("{}", &*v53);
                0i32
            }
            US2::US2_KindUnknown => {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list --kind expects "); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = std::rc::Rc::<str>::from((v15).to_ascii_lowercase().as_str());
                let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v16.clone()));
                let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v17.clone()));
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = std::rc::Rc::<str>::from((v19).to_ascii_lowercase().as_str());
                let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v20.clone()));
                let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v18.clone(), v21.clone()));
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                let mut v24: Rc<str> = std::rc::Rc::<str>::from((v23).to_ascii_lowercase().as_str());
                let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v24.clone()));
                let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v22.clone(), v25.clone()));
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                let mut v28: Rc<str> = std::rc::Rc::<str>::from((v27).to_ascii_lowercase().as_str());
                let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v28.clone()));
                let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v26.clone(), v29.clone()));
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", got "); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v30, &*v31, &*v1, &*v32].concat());
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
                let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v33, &*v32, &*v32, &*v32].concat());
                eprintln!("{}", &*v35);
                2i32
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        memory_list_run_13(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn query_score_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32) -> i32 {
    loop {
        let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v10 {
            return v9;
        } else {
            let mut v11: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let mut v12: Rc<str> = std::rc::Rc::<str>::from((v2).to_lowercase().as_str());
            let mut v13: i32 = (v12).matches(&*v11).count() as i32;
            let mut v14: i32 = 5i32.wrapping_mul(v13);
            let mut v15: Rc<str> = std::rc::Rc::<str>::from((v3).to_lowercase().as_str());
            let mut v16: i32 = (v15).matches(&*v11).count() as i32;
            let mut v17: i32 = 3i32.wrapping_mul(v16);
            let mut v18: i32 = v14.wrapping_add(v17);
            let mut v19: Rc<str> = std::rc::Rc::<str>::from((v8).to_lowercase().as_str());
            let mut v20: i32 = (v19).matches(&*v11).count() as i32;
            let mut v21: i32 = v18.wrapping_add(v20);
            let mut v22: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let mut v23: i32 = v9.wrapping_add(v21);
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v22.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v23);
            continue;
        }
    }
}
fn ranked_inserted_19(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<UH1>) -> Rc<UH1> {
    match &*v9 {
        UH1::UH1_NoRanked => {
            let mut v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRanked); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_RankedThen(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v10.clone()))
        }
        UH1::UH1_RankedThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21) => {
            let mut v12: i32 = *v12;
            let mut v13: Rc<str> = v13.clone();
            let mut v14: Rc<str> = v14.clone();
            let mut v15: Rc<str> = v15.clone();
            let mut v16: US0 = v16.clone();
            let mut v17: Rc<str> = v17.clone();
            let mut v18: Rc<str> = v18.clone();
            let mut v19: Rc<str> = v19.clone();
            let mut v20: Rc<str> = v20.clone();
            let mut v21: Rc<UH1> = v21.clone();
            let mut v22: bool = v0 > v12;
            let mut v26: bool = if v22 {
                true
            } else {
                let mut v23: bool = v0 == v12;
                if v23 {
                    let mut v24: bool = v2 < v14;
                    v24
                } else {
                    false
                }
            };
            if v26 {
                Rc::new(UH1::UH1_RankedThen(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone()))
            } else {
                let mut v28: Rc<UH1> = ranked_inserted_19(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v21.clone());
                Rc::new(UH1::UH1_RankedThen(v12, v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone(), v28.clone()))
            }
        }
    }
}
fn ranked_from_17(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH0::UH0_MemoryThen(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let mut v6: US0 = v6.clone();
                let mut v7: Rc<str> = v7.clone();
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<str> = v10.clone();
                let mut v11: Rc<UH0> = v11.clone();
                let mut v12: i32 = 0i32;
                let mut v13: i32 = query_score_18(v1.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v12);
                let mut v14: bool = v13 > 0i32;
                let mut v16: Rc<UH1> = if v14 {
                    ranked_inserted_19(v13, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v2.clone())
                } else {
                    v2.clone()
                };
                (v0, v1, v2) = (v11.clone(), v1.clone(), v16.clone());
                continue;
            }
            UH0::UH0_NoMemories => {
                return v2.clone();
            }
        }
    }
}
fn ranked_printed_20(mut v0: Rc<UH1>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_NoRanked => {
                return v2;
            }
            UH1::UH1_RankedThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let mut v3: i32 = *v3;
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: US0 = v7.clone();
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<str> = v10.clone();
                let mut v11: Rc<str> = v11.clone();
                let mut v12: Rc<UH1> = v12.clone();
                let mut v13: bool = v2 < v1;
                if v13 {
                    let mut v14: Rc<str> = std::rc::Rc::<str>::from((v3).to_string().as_str());
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                    let mut v20: Rc<str> = match &v7 {
                        US0::US0_KindFeedback => {
                            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                            v17.clone()
                        }
                        US0::US0_KindProject => {
                            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                            v18.clone()
                        }
                        US0::US0_KindReference => {
                            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                            v19.clone()
                        }
                        US0::US0_KindUser => {
                            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                            v16.clone()
                        }
                    };
                    let mut v21: Rc<str> = std::rc::Rc::<str>::from((v20).to_ascii_lowercase().as_str());
                    let mut v22: Rc<str> = { let text = v6; if text.chars().count() <= 100i32 as usize { text } else { std::rc::Rc::<str>::from(text.chars().take((100i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
                    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v15, &*v22, &*v23, &*v23].concat());
                    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v21, &*v15, &*v24].concat());
                    println!("{}", &*v25);
                    let mut v26: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2) = (v12.clone(), v1, v26);
                    continue;
                } else {
                    return v2;
                }
            }
        }
    }
}
fn memory_recall_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    let mut v3: bool = file_present_1(v0.clone());
    let mut v4: bool = v3 == false;
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("recall needs a readable data file: "); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v0, &*v6, &*v6, &*v6].concat());
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v6, &*v6, &*v6].concat());
        eprintln!("{}", &*v9);
        2i32
    } else {
        let mut v10: Rc<str> = std::rc::Rc::<str>::from((v1).to_lowercase().split_whitespace().collect::<Vec<_>>().join("
").as_str());
        let mut v11: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("recall needs query words"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v12, &*v14, &*v14, &*v14].concat());
            eprintln!("{}", &*v15);
            2i32
        } else {
            let mut v16: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
            let mut v17: i32 = 0i32;
            let mut v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
            let mut v19: i32 = 0i32;
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let (mut v21, mut v22, mut v23, mut v24): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_2(v16.clone(), v17, v18.clone(), v19, v20.clone(), v20.clone());
            let mut v25: Rc<str> = std::rc::Rc::<str>::from((v1).to_lowercase().split_whitespace().collect::<Vec<_>>().join("
").as_str());
            let mut v26: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRanked); } CASE.with(|case| case.clone()) };
            let mut v27: Rc<UH1> = ranked_from_17(v21.clone(), v25.clone(), v26.clone());
            let mut v28: i32 = 0i32;
            let mut v29: i32 = ranked_printed_20(v27.clone(), v2, v28);
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie memory recall ok shown="); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = std::rc::Rc::<str>::from((v29).to_string().as_str());
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of="); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = std::rc::Rc::<str>::from((v22).to_string().as_str());
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" query="); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v1, &*v20, &*v20, &*v20].concat());
            let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v32, &*v33, &*v35].concat());
            println!("{}", &*v36);
            0i32
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32| -> i32 {
        memory_recall_16(v0.clone(), v1.clone(), v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_memory_check(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_memory_list(v0: &str, v1: &str) -> i32 {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_memory_recall(v0: &str, v1: &str, v2: i32) -> i32 {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2)
}
