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
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_VerbImport,
    US5_VerbRender,
    US5_VerbCheck,
    US5_VerbList,
    US5_VerbRecall,
}
#[derive(Clone)]
enum US4 {
    US4_VerbKnown(US5),
    US4_VerbUnknown,
}
fn file_present_2(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn memories_flipped_4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn kind_of_case_6(mut v0: Rc<str>) -> US2 {
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
fn unescaped_from_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> (Rc<str>, i32) {
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
fn quoted_at_7(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, i32) {
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
        unescaped_from_8(v0.clone(), v8, v8, v9.clone())
    }
}
fn entry_of_line_5(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= Memory ("); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = (v0).get(0i32 as usize..).and_then(|rest| rest.find(&*v1)).map(|at| at as i32 + 0i32).unwrap_or(-1);
    let mut v3: i32 = v2.wrapping_add(10i32);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = (v0).get(v3 as usize..).and_then(|rest| rest.find(&*v4)).map(|at| at as i32 + v3).unwrap_or(-1);
    let mut v6: Rc<str> = std::rc::Rc::<str>::from((v0).get(v3 as usize..v5 as usize).unwrap_or(""));
    let mut v7: US2 = kind_of_case_6(v6.clone());
    match &v7 {
        US2::US2_KindKnown(v13) => {
            let mut v13: US0 = v13.clone();
            let (mut v14, mut v15): (Rc<str>, i32) = quoted_at_7(v0.clone(), v5);
            let (mut v16, mut v17): (Rc<str>, i32) = quoted_at_7(v0.clone(), v15);
            let (mut v18, mut v19): (Rc<str>, i32) = quoted_at_7(v0.clone(), v17);
            let (mut v20, mut v21): (Rc<str>, i32) = quoted_at_7(v0.clone(), v19);
            let (mut v22, mut v23): (Rc<str>, i32) = quoted_at_7(v0.clone(), v21);
            let (mut v24, mut v25): (Rc<str>, i32) = quoted_at_7(v0.clone(), v23);
            let (mut v26, mut v27): (Rc<str>, i32) = quoted_at_7(v0.clone(), v25);
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
fn store_from_3(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>) -> (Rc<UH0>, i32, Rc<str>, Rc<str>) {
    let mut v6: i32 = (v0.clone().len() as i32);
    let mut v7: bool = v1 >= v6;
    let (mut v60, mut v61, mut v62, mut v63): (Rc<UH0>, i32, Rc<str>, Rc<str>) = if v7 {
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<UH0> = memories_flipped_4(v2.clone(), v8.clone());
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
            let mut v20: US1 = entry_of_line_5(v14.clone());
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
                let (mut v45, mut v46): (Rc<str>, i32) = quoted_at_7(v14.clone(), v44);
                (v2.clone(), v3, v45.clone(), v5.clone())
            } else {
                (v2.clone(), v3, v4.clone(), v5.clone())
            };
            (v47.clone(), v48, v49.clone(), v50.clone())
        };
        let mut v55: i32 = v13.wrapping_add(1i32);
        let (mut v56, mut v57, mut v58, mut v59): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_3(v0.clone(), v55, v51.clone(), v52, v53.clone(), v54.clone());
        (v56.clone(), v57, v58.clone(), v59.clone())
    };
    (v60.clone(), v61, v62.clone(), v63.clone())
}
fn rendered_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US0, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("---\nname: "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ndescription: "); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nmetadata:\n"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v1, &*v9, &*v2, &*v10].concat());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_type"); } LIT.with(|lit| lit.clone()) };
    let mut v13: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v14.clone()
    } else {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v12, &*v16, &*v4, &*v17].concat());
        v18.clone()
    };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = match &v3 {
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
    let mut v27: bool = v26.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = if v27 {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v28.clone()
    } else {
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v20, &*v30, &*v26, &*v31].concat());
        v32.clone()
    };
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("originSessionId"); } LIT.with(|lit| lit.clone()) };
    let mut v35: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = if v35 {
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v36.clone()
    } else {
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v34, &*v38, &*v5, &*v39].concat());
        v40.clone()
    };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modified"); } LIT.with(|lit| lit.clone()) };
    let mut v43: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = if v43 {
        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v44.clone()
    } else {
        let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
        let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v42, &*v46, &*v6, &*v47].concat());
        v48.clone()
    };
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v33, &*v41, &*v49, &*v50].concat());
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("---\n"); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v51, &*v52, &*v7, &*v50].concat());
    v53.clone()
}
fn rendered_into_9(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
                let mut v12: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v3).to_string_lossy().as_ref());
                let mut v13: Rc<str> = rendered_10(v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone());
                let mut v14: bool = { let path = std::path::Path::new(&*v12); let _ = path.parent().map(std::fs::create_dir_all); std::fs::write(path, (v13).as_bytes()).is_ok() };
                let mut v16: i32 = if v14 {
                    let mut v15: i32 = v2.wrapping_add(1i32);
                    v15
                } else {
                    v2
                };
                (v0, v1, v2) = (v11.clone(), v1.clone(), v16);
                continue;
            }
            UH0::UH0_NoMemories => {
                return v2;
            }
        }
    }
}
fn memory_render_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: bool = file_present_2(v0.clone());
    let mut v3: bool = v2 == false;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("render needs a readable data file: "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v0, &*v5, &*v5, &*v5].concat());
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v6, &*v5, &*v5, &*v5].concat());
        eprintln!("{}", &*v8);
        2i32
    } else {
        let mut v9: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
        let mut v10: i32 = 0i32;
        let mut v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
        let mut v12: i32 = 0i32;
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let (mut v14, mut v15, mut v16, mut v17): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_3(v9.clone(), v10, v11.clone(), v12, v13.clone(), v13.clone());
        let mut v18: i32 = 0i32;
        let mut v19: i32 = rendered_into_9(v14.clone(), v1.clone(), v18);
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MEMORY.md"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v20).to_string_lossy().as_ref());
        let mut v22: bool = { let path = std::path::Path::new(&*v21); let _ = path.parent().map(std::fs::create_dir_all); std::fs::write(path, (v16).as_bytes()).is_ok() };
        let mut v23: bool = v19 == v15;
        let mut v24: bool = v23 && v22;
        if v24 {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie memory render ok memories="); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = std::rc::Rc::<str>::from((v19).to_string().as_str());
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v26, &*v27, &*v1, &*v13].concat());
            println!("{}", &*v28);
            0i32
        } else {
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("render wrote "); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = std::rc::Rc::<str>::from((v19).to_string().as_str());
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = std::rc::Rc::<str>::from((v15).to_string().as_str());
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" memories"); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v30, &*v31, &*v32, &*v33].concat());
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v34, &*v13, &*v13, &*v13].concat());
            eprintln!("{}", &*v36);
            2i32
        }
    }
}
fn command_line_argument_count_12() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_13(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_11(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_12();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US3 = if v5 {
        let mut v6: Rc<str> = command_line_argument_13(v1);
        US3::US3_Some(v6.clone())
    } else {
        US3::US3_None
    };
    match &v9 {
        US3::US3_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US3::US3_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn memory_usage_14() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: eoie memory import <memory-dir> <data.spi> | render <data.spi> <out-dir> | check <data.spi> | list <data.spi> [--kind user|feedback|project|reference] | recall <data.spi> \"<words>\" [--limit n]"); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn memory_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_11(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Import"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from((v2).to_ascii_lowercase().as_str());
    let mut v4: bool = v1.clone() == v3.clone();
    let mut v32: US4 = if v4 {
        let mut v5: US5 = US5::US5_VerbImport;
        US4::US4_VerbKnown(v5.clone())
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Render"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from((v7).to_ascii_lowercase().as_str());
        let mut v9: bool = v1.clone() == v8.clone();
        if v9 {
            let mut v10: US5 = US5::US5_VerbRender;
            US4::US4_VerbKnown(v10.clone())
        } else {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Check"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from((v12).to_ascii_lowercase().as_str());
            let mut v14: bool = v1.clone() == v13.clone();
            if v14 {
                let mut v15: US5 = US5::US5_VerbCheck;
                US4::US4_VerbKnown(v15.clone())
            } else {
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("List"); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = std::rc::Rc::<str>::from((v17).to_ascii_lowercase().as_str());
                let mut v19: bool = v1.clone() == v18.clone();
                if v19 {
                    let mut v20: US5 = US5::US5_VerbList;
                    US4::US4_VerbKnown(v20.clone())
                } else {
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Recall"); } LIT.with(|lit| lit.clone()) };
                    let mut v23: Rc<str> = std::rc::Rc::<str>::from((v22).to_ascii_lowercase().as_str());
                    let mut v24: bool = v1.clone() == v23.clone();
                    if v24 {
                        let mut v25: US5 = US5::US5_VerbRecall;
                        US4::US4_VerbKnown(v25.clone())
                    } else {
                        US4::US4_VerbUnknown
                    }
                }
            }
        }
    };
    match &v32 {
        US4::US4_VerbKnown(v37) => {
            let mut v37: US5 = v37.clone();
            match &v37 {
                US5::US5_VerbCheck => {
                    let mut v62: i32 = 3i32;
                    let mut v63: Rc<str> = argument_11(v62);
                    let mut v64: bool = v63.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v64 {
                        let mut v65: Rc<str> = memory_usage_14();
                        let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
                        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v68: Rc<str> = std::rc::Rc::<str>::from([&*v66, &*v65, &*v67, &*v67, &*v67].concat());
                        eprintln!("{}", &*v68);
                        2i32
                    } else {
                        let mut v69: i32 = eoie_memory_query::eoie_memory_check(&*v63);
                        v69
                    }
                }
                US5::US5_VerbImport => {
                    let mut v38: i32 = 3i32;
                    let mut v39: Rc<str> = argument_11(v38);
                    let mut v40: i32 = 4i32;
                    let mut v41: Rc<str> = argument_11(v40);
                    let mut v42: bool = v41.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v42 {
                        let mut v43: Rc<str> = memory_usage_14();
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
                        let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v43, &*v45, &*v45, &*v45].concat());
                        eprintln!("{}", &*v46);
                        2i32
                    } else {
                        let mut v47: i32 = eoie_memory_import::eoie_memory_import(&*v39, &*v41);
                        v47
                    }
                }
                US5::US5_VerbList => {
                    let mut v71: i32 = 3i32;
                    let mut v72: Rc<str> = argument_11(v71);
                    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--kind"); } LIT.with(|lit| lit.clone()) };
                    let mut v74: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v73).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
                    let mut v75: bool = v72.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v75 {
                        let mut v76: Rc<str> = memory_usage_14();
                        let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
                        let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v79: Rc<str> = std::rc::Rc::<str>::from([&*v77, &*v76, &*v78, &*v78, &*v78].concat());
                        eprintln!("{}", &*v79);
                        2i32
                    } else {
                        let mut v80: i32 = eoie_memory_query::eoie_memory_list(&*v72, &*v74);
                        v80
                    }
                }
                US5::US5_VerbRecall => {
                    let mut v82: i32 = 3i32;
                    let mut v83: Rc<str> = argument_11(v82);
                    let mut v84: i32 = 4i32;
                    let mut v85: Rc<str> = argument_11(v84);
                    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit"); } LIT.with(|lit| lit.clone()) };
                    let mut v87: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v86).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
                    let mut v88: i32 = (v87).trim().parse::<i32>().ok().filter(|value| *value > 0).unwrap_or(10i32);
                    let mut v89: bool = v85.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v89 {
                        let mut v90: Rc<str> = memory_usage_14();
                        let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
                        let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v93: Rc<str> = std::rc::Rc::<str>::from([&*v91, &*v90, &*v92, &*v92, &*v92].concat());
                        eprintln!("{}", &*v93);
                        2i32
                    } else {
                        let mut v94: i32 = eoie_memory_query::eoie_memory_recall(&*v83, &*v85, v88);
                        v94
                    }
                }
                US5::US5_VerbRender => {
                    let mut v49: i32 = 4i32;
                    let mut v50: Rc<str> = argument_11(v49);
                    let mut v51: bool = v50.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v51 {
                        let mut v52: Rc<str> = memory_usage_14();
                        let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
                        let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v52, &*v54, &*v54, &*v54].concat());
                        eprintln!("{}", &*v55);
                        2i32
                    } else {
                        let mut v56: i32 = 3i32;
                        let mut v57: Rc<str> = argument_11(v56);
                        let mut v58: i32 = 4i32;
                        let mut v59: Rc<str> = argument_11(v58);
                        memory_render_1(v57.clone(), v59.clone())
                    }
                }
            }
        }
        US4::US4_VerbUnknown => {
            let mut v33: Rc<str> = memory_usage_14();
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v33, &*v35, &*v35, &*v35].concat());
            eprintln!("{}", &*v36);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        memory_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_memory_run() -> i32 {
    closure0()()
}
