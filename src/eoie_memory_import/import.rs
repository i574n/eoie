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
fn file_present_1(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn kind_of_word_4(mut v0: Rc<str>) -> US2 {
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
fn rendered_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US0, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> Rc<str> {
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
fn entry_of_file_3(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v2).unwrap_or_default().as_str());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n---\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = (v3).get(3i32 as usize..).and_then(|rest| rest.find(&*v4)).map(|at| at as i32 + 3i32).unwrap_or(-1);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("---\n"); } LIT.with(|lit| lit.clone()) };
    let mut v7: bool = v3.starts_with(&*v6);
    let mut v8: bool = v7 == false;
    let mut v10: bool = if v8 {
        true
    } else {
        let mut v9: bool = v5 < 0i32;
        v9
    };
    if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": no frontmatter"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v11, &*v12, &*v12, &*v12].concat());
        US1::US1_EntryRefused(v13.clone())
    } else {
        let mut v15: i32 = v5.wrapping_add(1i32);
        let mut v16: Rc<str> = std::rc::Rc::<str>::from((v3).get(4i32 as usize..v15 as usize).unwrap_or(""));
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  type: "); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v17)).unwrap_or(""));
        let mut v19: US2 = kind_of_word_4(v18.clone());
        match &v19 {
            US2::US2_KindKnown(v25) => {
                let mut v25: US0 = v25.clone();
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("name: "); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v26)).unwrap_or(""));
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("description: "); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v28)).unwrap_or(""));
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  node_type: "); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v30)).unwrap_or(""));
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  originSessionId: "); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v32)).unwrap_or(""));
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  modified: "); } LIT.with(|lit| lit.clone()) };
                let mut v35: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v34)).unwrap_or(""));
                let mut v36: i32 = v5.wrapping_add(5i32);
                let mut v37: i32 = (v3.clone().len() as i32);
                let mut v38: Rc<str> = std::rc::Rc::<str>::from((v3).get(v36 as usize..v37 as usize).unwrap_or(""));
                let mut v39: Rc<str> = rendered_5(v1.clone(), v27.clone(), v29.clone(), v25.clone(), v31.clone(), v33.clone(), v35.clone(), v38.clone());
                let mut v40: bool = v39.clone() == v3.clone();
                if v40 {
                    US1::US1_EntryRead(v1.clone(), v27.clone(), v29.clone(), v25.clone(), v31.clone(), v33.clone(), v35.clone(), v38.clone())
                } else {
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": does not render back byte-identical"); } LIT.with(|lit| lit.clone()) };
                    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v42, &*v43, &*v43, &*v43].concat());
                    US1::US1_EntryRefused(v44.clone())
                }
            }
            US2::US2_KindUnknown => {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": unknown metadata type "); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v17)).unwrap_or(""));
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v20, &*v21, &*v22, &*v22].concat());
                US1::US1_EntryRefused(v23.clone())
            }
        }
    }
}
fn entries_from_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: i32, mut v4: Rc<str>) -> (Rc<UH0>, i32, Rc<str>) {
    let mut v5: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v38, mut v39, mut v40): (Rc<UH0>, i32, Rc<str>) = if v5 {
        (v2.clone(), v3, v4.clone())
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(10u8)).next().unwrap_or(""));
        let mut v7: US1 = entry_of_file_3(v0.clone(), v6.clone());
        let (mut v35, mut v36, mut v37): (Rc<UH0>, i32, Rc<str>) = match &v7 {
            US1::US1_EntryRead(v8, v9, v10, v11, v12, v13, v14, v15) => {
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<str> = v10.clone();
                let mut v11: US0 = v11.clone();
                let mut v12: Rc<str> = v12.clone();
                let mut v13: Rc<str> = v13.clone();
                let mut v14: Rc<str> = v14.clone();
                let mut v15: Rc<str> = v15.clone();
                let mut v16: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                let mut v17: i32 = v3.wrapping_add(1i32);
                let mut v18: Rc<UH0> = Rc::new(UH0::UH0_MemoryThen(v8.clone(), v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14.clone(), v15.clone(), v2.clone()));
                entries_from_2(v0.clone(), v16.clone(), v18.clone(), v17, v4.clone())
            }
            _ => {
                let (mut v32, mut v33, mut v34): (Rc<UH0>, i32, Rc<str>) = match &v7 {
                    US1::US1_EntryRefused(v22) => {
                        let mut v22: Rc<str> = v22.clone();
                        let mut v23: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                        let mut v24: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v28: Rc<str> = if v24 {
                            v22.clone()
                        } else {
                            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v25, &*v22, &*v26, &*v26].concat());
                            v27.clone()
                        };
                        entries_from_2(v0.clone(), v23.clone(), v2.clone(), v3, v28.clone())
                    }
                    _ => unreachable!(),
                };
                (v32.clone(), v33, v34.clone())
            }
        };
        (v35.clone(), v36, v37.clone())
    };
    (v38.clone(), v39, v40.clone())
}
fn memories_flipped_6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn memory_line_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US0, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v10, &v11));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(v12.replace(&*v13, &v11));
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v14, &*v15, &*v15, &*v15].concat());
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Memory ("); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = match &v3 {
        US0::US0_KindFeedback => {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
            v19.clone()
        }
        US0::US0_KindProject => {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
            v20.clone()
        }
        US0::US0_KindReference => {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
            v21.clone()
        }
        US0::US0_KindUser => {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
            v18.clone()
        }
    };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v16, &*v17, &*v22, &*v23].concat());
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v26, &v27));
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = std::rc::Rc::<str>::from(v28.replace(&*v25, &v29));
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = std::rc::Rc::<str>::from(v30.replace(&*v31, &v32));
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = std::rc::Rc::<str>::from(v33.replace(&*v34, &v35));
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = std::rc::Rc::<str>::from(v36.replace(&*v37, &v38));
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v39, &*v25, &*v15, &*v15].concat());
    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v26, &v27));
    let mut v42: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v25, &v29));
    let mut v43: Rc<str> = std::rc::Rc::<str>::from(v42.replace(&*v31, &v32));
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v43.replace(&*v34, &v35));
    let mut v45: Rc<str> = std::rc::Rc::<str>::from(v44.replace(&*v37, &v38));
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v45, &*v25, &*v15, &*v15].concat());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v26, &v27));
    let mut v48: Rc<str> = std::rc::Rc::<str>::from(v47.replace(&*v25, &v29));
    let mut v49: Rc<str> = std::rc::Rc::<str>::from(v48.replace(&*v31, &v32));
    let mut v50: Rc<str> = std::rc::Rc::<str>::from(v49.replace(&*v34, &v35));
    let mut v51: Rc<str> = std::rc::Rc::<str>::from(v50.replace(&*v37, &v38));
    let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v51, &*v25, &*v15, &*v15].concat());
    let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v23, &*v46, &*v23, &*v52].concat());
    let mut v54: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v26, &v27));
    let mut v55: Rc<str> = std::rc::Rc::<str>::from(v54.replace(&*v25, &v29));
    let mut v56: Rc<str> = std::rc::Rc::<str>::from(v55.replace(&*v31, &v32));
    let mut v57: Rc<str> = std::rc::Rc::<str>::from(v56.replace(&*v34, &v35));
    let mut v58: Rc<str> = std::rc::Rc::<str>::from(v57.replace(&*v37, &v38));
    let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v58, &*v25, &*v15, &*v15].concat());
    let mut v60: Rc<str> = std::rc::Rc::<str>::from(v5.replace(&*v26, &v27));
    let mut v61: Rc<str> = std::rc::Rc::<str>::from(v60.replace(&*v25, &v29));
    let mut v62: Rc<str> = std::rc::Rc::<str>::from(v61.replace(&*v31, &v32));
    let mut v63: Rc<str> = std::rc::Rc::<str>::from(v62.replace(&*v34, &v35));
    let mut v64: Rc<str> = std::rc::Rc::<str>::from(v63.replace(&*v37, &v38));
    let mut v65: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v64, &*v25, &*v15, &*v15].concat());
    let mut v66: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v59, &*v23, &*v65, &*v23].concat());
    let mut v67: Rc<str> = std::rc::Rc::<str>::from(v6.replace(&*v26, &v27));
    let mut v68: Rc<str> = std::rc::Rc::<str>::from(v67.replace(&*v25, &v29));
    let mut v69: Rc<str> = std::rc::Rc::<str>::from(v68.replace(&*v31, &v32));
    let mut v70: Rc<str> = std::rc::Rc::<str>::from(v69.replace(&*v34, &v35));
    let mut v71: Rc<str> = std::rc::Rc::<str>::from(v70.replace(&*v37, &v38));
    let mut v72: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v71, &*v25, &*v15, &*v15].concat());
    let mut v73: Rc<str> = std::rc::Rc::<str>::from(v7.replace(&*v26, &v27));
    let mut v74: Rc<str> = std::rc::Rc::<str>::from(v73.replace(&*v25, &v29));
    let mut v75: Rc<str> = std::rc::Rc::<str>::from(v74.replace(&*v31, &v32));
    let mut v76: Rc<str> = std::rc::Rc::<str>::from(v75.replace(&*v34, &v35));
    let mut v77: Rc<str> = std::rc::Rc::<str>::from(v76.replace(&*v37, &v38));
    let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v77, &*v25, &*v15, &*v15].concat());
    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")\n"); } LIT.with(|lit| lit.clone()) };
    let mut v80: Rc<str> = std::rc::Rc::<str>::from([&*v72, &*v23, &*v78, &*v79, &*v15].concat());
    let mut v81: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v53, &*v66, &*v80, &*v15].concat());
    v81.clone()
}
fn memory_lines_8(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
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
                let mut v11: Rc<str> = memory_line_9(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone());
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v11, &*v12, &*v12, &*v12].concat());
                (v0, v1) = (v10.clone(), v13.clone());
                continue;
            }
            UH0::UH0_NoMemories => {
                return v1.clone();
            }
        }
    }
}
fn data_text_7(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union memory_kind =\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | User\n    | Feedback\n    | Project\n    | Reference"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\nunion memory_item =\n    | Memory :: memory_kind * string * string * string * string * string * string * string -> memory_item\n    | MemoryIndex :: string -> memory_item\n\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v4, &*v5, &*v5].concat());
    let mut v7: Rc<str> = memory_lines_8(v0.clone(), v5.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl index_of_memory () = MemoryIndex ("); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v10, &v11));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(v12.replace(&*v9, &v13));
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from(v14.replace(&*v15, &v16));
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v17.replace(&*v18, &v19));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v20.replace(&*v21, &v22));
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v23, &*v9, &*v5, &*v5].concat());
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v8, &*v24, &*v25].concat());
    v26.clone()
}
fn kind_of_case_12(mut v0: Rc<str>) -> US2 {
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
fn unescaped_from_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> (Rc<str>, i32) {
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
fn quoted_at_13(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, i32) {
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
        unescaped_from_14(v0.clone(), v8, v8, v9.clone())
    }
}
fn entry_of_line_11(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= Memory ("); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = (v0).get(0i32 as usize..).and_then(|rest| rest.find(&*v1)).map(|at| at as i32 + 0i32).unwrap_or(-1);
    let mut v3: i32 = v2.wrapping_add(10i32);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = (v0).get(v3 as usize..).and_then(|rest| rest.find(&*v4)).map(|at| at as i32 + v3).unwrap_or(-1);
    let mut v6: Rc<str> = std::rc::Rc::<str>::from((v0).get(v3 as usize..v5 as usize).unwrap_or(""));
    let mut v7: US2 = kind_of_case_12(v6.clone());
    match &v7 {
        US2::US2_KindKnown(v13) => {
            let mut v13: US0 = v13.clone();
            let (mut v14, mut v15): (Rc<str>, i32) = quoted_at_13(v0.clone(), v5);
            let (mut v16, mut v17): (Rc<str>, i32) = quoted_at_13(v0.clone(), v15);
            let (mut v18, mut v19): (Rc<str>, i32) = quoted_at_13(v0.clone(), v17);
            let (mut v20, mut v21): (Rc<str>, i32) = quoted_at_13(v0.clone(), v19);
            let (mut v22, mut v23): (Rc<str>, i32) = quoted_at_13(v0.clone(), v21);
            let (mut v24, mut v25): (Rc<str>, i32) = quoted_at_13(v0.clone(), v23);
            let (mut v26, mut v27): (Rc<str>, i32) = quoted_at_13(v0.clone(), v25);
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
fn store_from_10(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>) -> (Rc<UH0>, i32, Rc<str>, Rc<str>) {
    let mut v6: i32 = (v0.clone().len() as i32);
    let mut v7: bool = v1 >= v6;
    let (mut v60, mut v61, mut v62, mut v63): (Rc<UH0>, i32, Rc<str>, Rc<str>) = if v7 {
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<UH0> = memories_flipped_6(v2.clone(), v8.clone());
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
            let mut v20: US1 = entry_of_line_11(v14.clone());
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
                let (mut v45, mut v46): (Rc<str>, i32) = quoted_at_13(v14.clone(), v44);
                (v2.clone(), v3, v45.clone(), v5.clone())
            } else {
                (v2.clone(), v3, v4.clone(), v5.clone())
            };
            (v47.clone(), v48, v49.clone(), v50.clone())
        };
        let mut v55: i32 = v13.wrapping_add(1i32);
        let (mut v56, mut v57, mut v58, mut v59): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_10(v0.clone(), v55, v51.clone(), v52, v53.clone(), v54.clone());
        (v56.clone(), v57, v58.clone(), v59.clone())
    };
    (v60.clone(), v61, v62.clone(), v63.clone())
}
fn identical_count_15(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
                let mut v12: Rc<str> = rendered_5(v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone());
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v3).to_string_lossy().as_ref());
                let mut v14: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v13).unwrap_or_default().as_str());
                let mut v15: bool = v12.clone() == v14.clone();
                let mut v17: i32 = if v15 {
                    let mut v16: i32 = v2.wrapping_add(1i32);
                    v16
                } else {
                    v2
                };
                (v0, v1, v2) = (v11.clone(), v1.clone(), v17);
                continue;
            }
            UH0::UH0_NoMemories => {
                return v2;
            }
        }
    }
}
fn import_receipt_16(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = std::rc::Rc::<str>::from((v0).to_string().as_str());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie memory import ok memories="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" identical="); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let mut v9: Rc<str> = std::rc::Rc::<str>::from((v1).to_string().as_str());
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" index_identical="); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v2));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" data="); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v3));
    v16.clone()
}
fn memory_import_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MEMORY.md"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v2).to_string_lossy().as_ref());
    let mut v4: bool = file_present_1(v3.clone());
    let mut v5: bool = v4 == false;
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import needs "); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v0));
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/MEMORY.md"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v9, &*v11, &*v11, &*v11].concat());
        eprintln!("{}", &*v12);
        2i32
    } else {
        let mut v13: Rc<str> = std::rc::Rc::<str>::from({ let mut names = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.ends_with(".md") && name != "MEMORY.md").collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.join("
") }.as_str());
        let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
        let mut v15: i32 = 0i32;
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let (mut v17, mut v18, mut v19): (Rc<UH0>, i32, Rc<str>) = entries_from_2(v0.clone(), v13.clone(), v14.clone(), v15, v16.clone());
        let mut v20: bool = v19.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v20 {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import refused, nothing written:\n"); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v19));
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v22, &*v16, &*v16, &*v16].concat());
            eprintln!("{}", &*v24);
            2i32
        } else {
            let mut v25: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v2).to_string_lossy().as_ref());
            let mut v26: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v25).unwrap_or_default().as_str());
            let mut v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
            let mut v28: Rc<UH0> = memories_flipped_6(v17.clone(), v27.clone());
            let mut v29: Rc<str> = data_text_7(v28.clone(), v26.clone());
            let mut v30: bool = { let path = std::path::Path::new(&*v1); let _ = path.parent().map(std::fs::create_dir_all); std::fs::write(path, (v29).as_bytes()).is_ok() };
            if v30 {
                let mut v31: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v1).unwrap_or_default().as_str());
                let mut v32: i32 = 0i32;
                let mut v33: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMemories); } CASE.with(|case| case.clone()) };
                let mut v34: i32 = 0i32;
                let (mut v35, mut v36, mut v37, mut v38): (Rc<UH0>, i32, Rc<str>, Rc<str>) = store_from_10(v31.clone(), v32, v33.clone(), v34, v16.clone(), v16.clone());
                let mut v39: bool = v37.clone() == v26.clone();
                let mut v42: Rc<str> = if v39 {
                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                    v40.clone()
                } else {
                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no"); } LIT.with(|lit| lit.clone()) };
                    v41.clone()
                };
                let mut v43: i32 = 0i32;
                let mut v44: i32 = identical_count_15(v35.clone(), v0.clone(), v43);
                let mut v45: Rc<str> = import_receipt_16(v18, v44, v42.clone(), v1.clone());
                println!("{}", &*v45);
                0i32
            } else {
                let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import could not write "); } LIT.with(|lit| lit.clone()) };
                let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v1));
                let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: memory "); } LIT.with(|lit| lit.clone()) };
                let mut v49: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v47, &*v16, &*v16, &*v16].concat());
                eprintln!("{}", &*v49);
                2i32
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        memory_import_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_memory_import(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
