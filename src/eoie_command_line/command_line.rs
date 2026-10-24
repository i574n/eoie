#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Bare(Rc<str>),
    US0_Quoted(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_NoMoreWords,
    UH0_WordThen(US0, Rc<UH0>),
}
fn byte_at_2(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: i32 = (v0).as_bytes().get(v1 as usize).map(|byte| *byte as i32).unwrap_or(-1);
    v2
}
fn word_end_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = byte_at_2(v0.clone(), v1);
        let mut v4: bool = v3 < 0i32;
        let mut v6: bool = if v4 {
            true
        } else {
            let mut v5: bool = v3 == v2;
            v5
        };
        if v6 {
            return v1;
        } else {
            let mut v7: i32 = v1.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v7, v2);
            continue;
        }
    }
}
fn text_between_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize..v2 as usize).unwrap_or(""));
    v3.clone()
}
fn words_scanned_1(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: i32 = byte_at_2(v0.clone(), v1);
        let mut v4: bool = v3 < 0i32;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: bool = v3 == 32i32;
            if v5 {
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v6, v2.clone());
                continue;
            } else {
                let mut v8: bool = v3 == 34i32;
                if v8 {
                    let mut v9: i32 = v1.wrapping_add(1i32);
                    let mut v10: i32 = 34i32;
                    let mut v11: i32 = word_end_3(v0.clone(), v9, v10);
                    let mut v12: i32 = v11.wrapping_add(1i32);
                    let mut v13: Rc<str> = text_between_4(v0.clone(), v9, v11);
                    let mut v14: US0 = US0::US0_Quoted(v13.clone());
                    let mut v15: Rc<UH0> = Rc::new(UH0::UH0_WordThen(v14.clone(), v2.clone()));
                    (v0, v1, v2) = (v0.clone(), v12, v15.clone());
                    continue;
                } else {
                    let mut v17: i32 = 32i32;
                    let mut v18: i32 = word_end_3(v0.clone(), v1, v17);
                    let mut v19: Rc<str> = text_between_4(v0.clone(), v1, v18);
                    let mut v20: US0 = US0::US0_Bare(v19.clone());
                    let mut v21: Rc<UH0> = Rc::new(UH0::UH0_WordThen(v20.clone(), v2.clone()));
                    (v0, v1, v2) = (v0.clone(), v18, v21.clone());
                    continue;
                }
            }
        }
    }
}
fn words_flipped_5(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_NoMoreWords => {
                return v1.clone();
            }
            UH0::UH0_WordThen(v2, v3) => {
                let mut v2: US0 = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_WordThen(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(US0) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(US0) -> Rc<str>> = Rc::new(move |mut v0: US0| -> Rc<str> {
        match &v0 {
            US0::US0_Bare(v1) => {
                let mut v1: Rc<str> = v1.clone();
                v1.clone()
            }
            US0::US0_Quoted(v2) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
                let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v3));
                v5.clone()
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn words_joined_7(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<dyn Fn(US0) -> Rc<str>>, mut v3: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoMoreWords => {
                return v3.clone();
            }
            UH0::UH0_WordThen(v4, v5) => {
                let mut v4: US0 = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = if v6 {
                    v2(v4.clone())
                } else {
                    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
                    let mut v9: Rc<str> = v2(v4.clone());
                    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
                    v10.clone()
                };
                (v0, v1, v2, v3) = (v5.clone(), v1.clone(), v2.clone(), v11.clone());
                continue;
            }
        }
    }
}
fn any_letter_of_10(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (v0).chars().any(|letter| (v1).contains(letter));
    v2
}
fn word_needs_shell_9(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_Bare(v1) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: bool = cfg!(windows);
            let mut v5: Rc<str> = if v2 {
                let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|&<>^%()\n"); } LIT.with(|lit| lit.clone()) };
                v3.clone()
            } else {
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|&<>;$`*?~(){}[]'\"\\\n"); } LIT.with(|lit| lit.clone()) };
                v4.clone()
            };
            any_letter_of_10(v1.clone(), v5.clone())
        }
        US0::US0_Quoted(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: bool = cfg!(windows);
            let mut v11: Rc<str> = if v8 {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%\"\n"); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            } else {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$`\"\\\n"); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            };
            any_letter_of_10(v7.clone(), v11.clone())
        }
    }
}
fn syntax_word_8(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoMoreWords => {
                let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v1.clone();
            }
            UH0::UH0_WordThen(v2, v3) => {
                let mut v2: US0 = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = word_needs_shell_9(v2.clone());
                if v4 {
                    match &v2 {
                        US0::US0_Bare(v5) => {
                            let mut v5: Rc<str> = v5.clone();
                            return v5.clone();
                        }
                        US0::US0_Quoted(v6) => {
                            let mut v6: Rc<str> = v6.clone();
                            return v6.clone();
                        }
                    }
                } else {
                    v0 = v3.clone();
                    continue;
                }
            }
        }
    }
}
fn named_in_11(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (v1).split(char::from(124u8)).any(|name| name.eq_ignore_ascii_case(&*v0));
    v2
}
fn names_dir_12(mut v0: Rc<str>) -> bool {
    let mut v1: bool = (v0).contains(char::from(47u8)) || (v0).contains(char::from(92u8));
    v1
}
fn path_entries_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()).map(|dir| dir.to_string_lossy().into_owned()).collect::<Vec<_>>().join(&*v0));
    v1.clone()
}
fn has_extension_14(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::path::Path::new(&*v0).extension().is_some();
    v1
}
fn pathext_15() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var("PATHEXT").unwrap_or_default().as_str());
    v0.clone()
}
fn first_line_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
    v1.clone()
}
fn path_joined_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(format!("{}{}", &*v1, &*v2)).to_string_lossy().as_ref());
    v3.clone()
}
fn is_file_20(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn rest_lines_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
    v1.clone()
}
fn candidate_in_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: Rc<str> = first_line_17(v2.clone());
        let mut v4: Rc<str> = path_joined_19(v0.clone(), v1.clone(), v3.clone());
        let mut v5: bool = is_file_20(v4.clone());
        if v5 {
            return v4.clone();
        } else {
            let mut v6: Rc<str> = rest_lines_21(v2.clone());
            let mut v7: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v8.clone();
            } else {
                let mut v9: Rc<str> = rest_lines_21(v2.clone());
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9.clone());
                continue;
            }
        }
    }
}
fn program_in_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v4.clone();
        } else {
            let mut v5: Rc<str> = first_line_17(v0.clone());
            let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = if v6 {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v7.clone()
            } else {
                candidate_in_18(v5.clone(), v1.clone(), v2.clone())
            };
            let mut v10: bool = v9.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v10 {
                return v9.clone();
            } else {
                let mut v11: Rc<str> = rest_lines_21(v0.clone());
                (v0, v1, v2) = (v11.clone(), v1.clone(), v2.clone());
                continue;
            }
        }
    }
}
fn lowered_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_lowercase().as_str());
    v1.clone()
}
fn closure2() -> Rc<dyn Fn(US0) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(US0) -> Rc<str>> = Rc::new(move |mut v0: US0| -> Rc<str> {
        match &v0 {
            US0::US0_Bare(v1) => {
                let mut v1: Rc<str> = v1.clone();
                v1.clone()
            }
            US0::US0_Quoted(v2) => {
                let mut v2: Rc<str> = v2.clone();
                v2.clone()
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn launch_parts_of_6(mut v0: Rc<str>, mut v1: Rc<UH0>) -> (Rc<str>, Rc<str>) {
    let (mut v153, mut v154): (Rc<str>, Rc<str>) = match &*v1 {
        UH0::UH0_NoMoreWords => {
            let mut v2: bool = cfg!(windows);
            let mut v15: Rc<str> = if v2 {
                let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v4: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = words_joined_7(v1.clone(), v3.clone(), v4.clone(), v5.clone());
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd.exe\n/c\n"); } LIT.with(|lit| lit.clone()) };
                let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
                v8.clone()
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v10: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = words_joined_7(v1.clone(), v9.clone(), v10.clone(), v11.clone());
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh\n-c\n"); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v12));
                v14.clone()
            };
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = words_joined_7(v1.clone(), v16.clone(), v17.clone(), v18.clone());
            (v15.clone(), v19.clone())
        }
        _ => {
            let (mut v151, mut v152): (Rc<str>, Rc<str>) = match &*v1 {
                UH0::UH0_WordThen(v20, v21) => {
                    let mut v20: US0 = v20.clone();
                    let mut v21: Rc<UH0> = v21.clone();
                    let mut v22: Rc<str> = syntax_word_8(v1.clone());
                    let mut v26: Rc<str> = match &v20 {
                        US0::US0_Bare(v23) => {
                            let mut v23: Rc<str> = v23.clone();
                            v23.clone()
                        }
                        US0::US0_Quoted(v24) => {
                            let mut v24: Rc<str> = v24.clone();
                            v24.clone()
                        }
                    };
                    let mut v27: bool = v22.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v27 {
                        let mut v28: bool = cfg!(windows);
                        let mut v41: Rc<str> = if v28 {
                            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                            let mut v30: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v32: Rc<str> = words_joined_7(v1.clone(), v29.clone(), v30.clone(), v31.clone());
                            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd.exe\n/c\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
                            v34.clone()
                        } else {
                            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                            let mut v36: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v38: Rc<str> = words_joined_7(v1.clone(), v35.clone(), v36.clone(), v37.clone());
                            let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh\n-c\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v38));
                            v40.clone()
                        };
                        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let mut v43: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v45: Rc<str> = words_joined_7(v1.clone(), v42.clone(), v43.clone(), v44.clone());
                        (v41.clone(), v45.clone())
                    } else {
                        let mut v46: bool = cfg!(windows);
                        let mut v49: bool = if v46 {
                            let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assoc|Break|Call|Cd|Chdir|Cls|Color|Copy|Date|Del|Dir|Echo|Endlocal|Erase|Exit|For|Ftype|Goto|If|Md|Mkdir|Mklink|Move|Path|Pause|Popd|Prompt|Pushd|Rd|Rem|Ren|Rename|Rmdir|Set|Setlocal|Shift|Start|Time|Title|Type|Ver|Verify|Vol"); } LIT.with(|lit| lit.clone()) };
                            named_in_11(v26.clone(), v47.clone())
                        } else {
                            false
                        };
                        let (mut v147, mut v148): (Rc<str>, Rc<str>) = if v49 {
                            let mut v50: bool = cfg!(windows);
                            let mut v63: Rc<str> = if v50 {
                                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                let mut v52: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v54: Rc<str> = words_joined_7(v1.clone(), v51.clone(), v52.clone(), v53.clone());
                                let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd.exe\n/c\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v54));
                                v56.clone()
                            } else {
                                let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                let mut v58: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v60: Rc<str> = words_joined_7(v1.clone(), v57.clone(), v58.clone(), v59.clone());
                                let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh\n-c\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v60));
                                v62.clone()
                            };
                            let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                            let mut v65: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                            let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v67: Rc<str> = words_joined_7(v1.clone(), v64.clone(), v65.clone(), v66.clone());
                            (v63.clone(), v67.clone())
                        } else {
                            let mut v68: bool = names_dir_12(v26.clone());
                            let mut v77: Rc<str> = if v68 {
                                v0.clone()
                            } else {
                                let mut v69: bool = cfg!(windows);
                                if v69 {
                                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v70));
                                    let mut v72: Rc<str> = path_entries_13(v70.clone());
                                    let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v72));
                                    v73.clone()
                                } else {
                                    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    path_entries_13(v74.clone())
                                }
                            };
                            let mut v78: bool = cfg!(windows);
                            let mut v81: bool = if v78 {
                                let mut v79: bool = has_extension_14(v26.clone());
                                let mut v80: bool = v79 == false;
                                v80
                            } else {
                                false
                            };
                            let mut v90: Rc<str> = if v81 {
                                let mut v82: Rc<str> = pathext_15();
                                let mut v83: bool = v82.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v85: Rc<str> = if v83 {
                                    let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".COM;.EXE;.BAT;.CMD"); } LIT.with(|lit| lit.clone()) };
                                    v84.clone()
                                } else {
                                    v82.clone()
                                };
                                let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                                let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v88: Rc<str> = Rc::<str>::from(v85.replace(&*v86, &*v87));
                                v88.clone()
                            } else {
                                let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v89.clone()
                            };
                            let mut v91: Rc<str> = program_in_16(v77.clone(), v26.clone(), v90.clone());
                            let mut v92: bool = v91.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let (mut v145, mut v146): (Rc<str>, Rc<str>) = if v92 {
                                let mut v93: bool = cfg!(windows);
                                let mut v106: Rc<str> = if v93 {
                                    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    let mut v95: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                    let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v97: Rc<str> = words_joined_7(v1.clone(), v94.clone(), v95.clone(), v96.clone());
                                    let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd.exe\n/c\n"); } LIT.with(|lit| lit.clone()) };
                                    let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v97));
                                    v99.clone()
                                } else {
                                    let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    let mut v101: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                    let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v103: Rc<str> = words_joined_7(v1.clone(), v100.clone(), v101.clone(), v102.clone());
                                    let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh\n-c\n"); } LIT.with(|lit| lit.clone()) };
                                    let mut v105: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v103));
                                    v105.clone()
                                };
                                let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                let mut v108: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v110: Rc<str> = words_joined_7(v1.clone(), v107.clone(), v108.clone(), v109.clone());
                                (v106.clone(), v110.clone())
                            } else {
                                let mut v111: bool = cfg!(windows);
                                let mut v118: bool = if v111 {
                                    let mut v112: Rc<str> = lowered_22(v91.clone());
                                    let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".exe"); } LIT.with(|lit| lit.clone()) };
                                    let mut v114: bool = v112.ends_with(&*v113);
                                    if v114 {
                                        true
                                    } else {
                                        let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".com"); } LIT.with(|lit| lit.clone()) };
                                        let mut v116: bool = v112.ends_with(&*v115);
                                        v116
                                    }
                                } else {
                                    true
                                };
                                let (mut v143, mut v144): (Rc<str>, Rc<str>) = if v118 {
                                    let mut v119: US0 = US0::US0_Bare(v91.clone());
                                    let mut v120: Rc<UH0> = Rc::new(UH0::UH0_WordThen(v119.clone(), v21.clone()));
                                    let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let mut v122: Rc<dyn Fn(US0) -> Rc<str>> = closure2();
                                    let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v124: Rc<str> = words_joined_7(v120.clone(), v121.clone(), v122.clone(), v123.clone());
                                    (v124.clone(), v123.clone())
                                } else {
                                    let mut v125: bool = cfg!(windows);
                                    let mut v138: Rc<str> = if v125 {
                                        let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                        let mut v127: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                        let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v129: Rc<str> = words_joined_7(v1.clone(), v126.clone(), v127.clone(), v128.clone());
                                        let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd.exe\n/c\n"); } LIT.with(|lit| lit.clone()) };
                                        let mut v131: Rc<str> = Rc::<str>::from(format!("{}{}", v130, v129));
                                        v131.clone()
                                    } else {
                                        let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                        let mut v133: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                        let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v135: Rc<str> = words_joined_7(v1.clone(), v132.clone(), v133.clone(), v134.clone());
                                        let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh\n-c\n"); } LIT.with(|lit| lit.clone()) };
                                        let mut v137: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v135));
                                        v137.clone()
                                    };
                                    let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    let mut v140: Rc<dyn Fn(US0) -> Rc<str>> = closure1();
                                    let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v142: Rc<str> = words_joined_7(v1.clone(), v139.clone(), v140.clone(), v141.clone());
                                    (v138.clone(), v142.clone())
                                };
                                (v143.clone(), v144.clone())
                            };
                            (v145.clone(), v146.clone())
                        };
                        (v147.clone(), v148.clone())
                    }
                }
                _ => unreachable!(),
            };
            (v151.clone(), v152.clone())
        }
    };
    (v153.clone(), v154.clone())
}
fn launch_argv_of_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: i32 = 0i32;
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMoreWords); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = words_scanned_1(v1.clone(), v2, v3.clone());
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMoreWords); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = words_flipped_5(v4.clone(), v5.clone());
    let (mut v7, mut v8): (Rc<str>, Rc<str>) = launch_parts_of_6(v0.clone(), v6.clone());
    v7.clone()
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        launch_argv_of_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn shell_line_of_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: i32 = 0i32;
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMoreWords); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = words_scanned_1(v1.clone(), v2, v3.clone());
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMoreWords); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = words_flipped_5(v4.clone(), v5.clone());
    let (mut v7, mut v8): (Rc<str>, Rc<str>) = launch_parts_of_6(v0.clone(), v6.clone());
    v8.clone()
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        shell_line_of_23(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_launch_argv(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_launch_line(v0: &str, v1: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
