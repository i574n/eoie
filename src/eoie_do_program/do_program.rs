#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US2 {
    US2_Apply,
    US2_Preview,
}
#[derive(Clone)]
enum US3 {
    US3_Unchecked,
    US3_Expected(u64, u64),
}
#[derive(Clone)]
enum US4 {
    US4_Once,
    US4_Every,
    US4_Exactly(u64),
}
#[derive(Clone)]
enum US5 {
    US5_Literal,
    US5_Pattern,
}
#[derive(Clone)]
enum UH1 {
    UH1_NoRules,
    UH1_Rule(Rc<str>, Rc<str>, US4, US5, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_EditStep(Rc<str>, US2, US3, Rc<str>, Rc<UH1>),
    US1_ShowStep(Rc<str>),
    US1_QueryStep(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Done,
    UH0_Step(US1, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Parsed(Rc<UH0>, i32),
    US0_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_Word(Rc<str>),
    US6_Quoted(Rc<str>),
    US6_StatementEnd,
    US6_Unterminated(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_Parsed(US1, i32),
    US7_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US8 {
    US8_Parsed(Rc<str>, US2, US3, Rc<str>, Rc<UH1>, i32),
    US8_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US9 {
    US9_Parsed(Rc<UH1>, i32),
    US9_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US10 {
    US10_Parsed(Rc<str>, Rc<str>, US4, US5, i32),
    US10_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US11 {
    US11_Parsed(US4, i32),
    US11_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US12 {
    US12_Parsed(std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>, i32),
    US12_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US13 {
    US13_Parsed(Rc<str>, i32),
    US13_Rejected(Rc<str>),
}
fn token_at_4(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US6 {
    let mut v2: i32 = v0.len() as i32;
    let mut v3: bool = v1 >= v2;
    if v3 {
        US6::US6_StatementEnd
    } else {
        let mut v5: i32 = v0[v1 as usize].0;
        let mut v6: Rc<str> = v0[v1 as usize].1.clone();
        let mut v7: bool = v5 == 0i32;
        if v7 {
            US6::US6_Word(v6.clone())
        } else {
            let mut v9: bool = v5 == 1i32;
            if v9 {
                US6::US6_Quoted(v6.clone())
            } else {
                let mut v11: bool = v5 == 2i32;
                if v11 {
                    US6::US6_StatementEnd
                } else {
                    US6::US6_Unterminated(v6.clone())
                }
            }
        }
    }
}
fn skip_ends_3(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: US6 = token_at_4(v0.clone(), v1);
            match &v4 {
                US6::US6_StatementEnd => {
                    let mut v5: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v5);
                    continue;
                }
                _ => {
                    return v1;
                }
            }
        } else {
            return v1;
        }
    }
}
fn word_is_9(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>) -> bool {
    let mut v3: US6 = token_at_4(v0.clone(), v1);
    match &v3 {
        US6::US6_Word(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: bool = v4.clone() == v2.clone();
            v5
        }
        _ => {
            false
        }
    }
}
fn header_options_8(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>, mut v3: US2, mut v4: US3, mut v5: Rc<str>, mut v6: Rc<UH1>) -> US8 {
    loop {
        let mut v7: US6 = token_at_4(v0.clone(), v1);
        match &v7 {
            US6::US6_StatementEnd => {
                return US8::US8_Parsed(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v1);
            }
            US6::US6_Word(v9) => {
                let mut v9: Rc<str> = v9.clone();
                let mut v10: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dry-run"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: i32 = v1.wrapping_add(1i32);
                    let mut v12: US2 = US2::US2_Preview;
                    (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v11, v2.clone(), v12.clone(), v4.clone(), v5.clone(), v6.clone());
                    continue;
                } else {
                    let mut v14: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except"); } LIT.with(|lit| lit.clone()) };
                    if v14 {
                        let mut v15: i32 = v1.wrapping_add(1i32);
                        let mut v16: US6 = token_at_4(v0.clone(), v15);
                        match &v16 {
                            US6::US6_Quoted(v20) => {
                                let mut v20: Rc<str> = v20.clone();
                                let mut v21: i32 = v1.wrapping_add(2i32);
                                (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v21, v2.clone(), v3.clone(), v4.clone(), v20.clone(), v6.clone());
                                continue;
                            }
                            US6::US6_Word(v17) => {
                                let mut v17: Rc<str> = v17.clone();
                                let mut v18: i32 = v1.wrapping_add(2i32);
                                (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v18, v2.clone(), v3.clone(), v4.clone(), v17.clone(), v6.clone());
                                continue;
                            }
                            _ => {
                                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: except expects a glob or a comma list of globs"); } LIT.with(|lit| lit.clone()) };
                                return US8::US8_Rejected(v23.clone());
                            }
                        }
                    } else {
                        let mut v27: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expect"); } LIT.with(|lit| lit.clone()) };
                        let mut v31: bool = if v27 {
                            let mut v28: i32 = v1.wrapping_add(1i32);
                            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
                            word_is_9(v0.clone(), v28, v29.clone())
                        } else {
                            false
                        };
                        let mut v35: bool = if v31 {
                            let mut v32: i32 = v1.wrapping_add(3i32);
                            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacements"); } LIT.with(|lit| lit.clone()) };
                            word_is_9(v0.clone(), v32, v33.clone())
                        } else {
                            false
                        };
                        if v35 {
                            let mut v36: i32 = v1.wrapping_add(2i32);
                            let mut v37: US6 = token_at_4(v0.clone(), v36);
                            match &v37 {
                                US6::US6_Word(v38) => {
                                    let mut v38: Rc<str> = v38.clone();
                                    let mut v39: i32 = v1.wrapping_add(4i32);
                                    let mut v40: US6 = token_at_4(v0.clone(), v39);
                                    match &v40 {
                                        US6::US6_Word(v41) => {
                                            let mut v41: Rc<str> = v41.clone();
                                            let mut v42: bool = !v38.is_empty() && v38.bytes().all(|byte| byte.is_ascii_digit());
                                            let mut v44: bool = if v42 {
                                                let mut v43: bool = !v41.is_empty() && v41.bytes().all(|byte| byte.is_ascii_digit());
                                                v43
                                            } else {
                                                false
                                            };
                                            if v44 {
                                                let mut v45: i32 = v1.wrapping_add(5i32);
                                                let mut v46: u64 = v38.trim().parse::<u64>().unwrap_or(0);
                                                let mut v47: u64 = v41.trim().parse::<u64>().unwrap_or(0);
                                                let mut v48: US3 = US3::US3_Expected(v46, v47);
                                                (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v45, v2.clone(), v3.clone(), v48.clone(), v5.clone(), v6.clone());
                                                continue;
                                            } else {
                                                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expect files <n> replacements <n>"); } LIT.with(|lit| lit.clone()) };
                                                return US8::US8_Rejected(v50.clone());
                                            }
                                        }
                                        _ => {
                                            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expect files <n> replacements <n>"); } LIT.with(|lit| lit.clone()) };
                                            return US8::US8_Rejected(v53.clone());
                                        }
                                    }
                                }
                                _ => {
                                    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expect files <n> replacements <n>"); } LIT.with(|lit| lit.clone()) };
                                    return US8::US8_Rejected(v56.clone());
                                }
                            }
                        } else {
                            let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: unknown option "); } LIT.with(|lit| lit.clone()) };
                            let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v61: Rc<str> = std::rc::Rc::<str>::from([&*v59, &*v9, &*v60, &*v60, &*v60].concat());
                            return US8::US8_Rejected(v61.clone());
                        }
                    }
                }
            }
            _ => {
                let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: unexpected quoted text"); } LIT.with(|lit| lit.clone()) };
                return US8::US8_Rejected(v66.clone());
            }
        }
    }
}
fn edit_header_at_7(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US8 {
    let mut v2: i32 = v1.wrapping_add(1i32);
    let mut v3: US6 = token_at_4(v0.clone(), v2);
    match &v3 {
        US6::US6_Quoted(v11) => {
            let mut v11: Rc<str> = v11.clone();
            let mut v12: i32 = v1.wrapping_add(2i32);
            let mut v13: US2 = US2::US2_Apply;
            let mut v14: US3 = US3::US3_Unchecked;
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRules); } CASE.with(|case| case.clone()) };
            header_options_8(v0.clone(), v12, v11.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone())
        }
        US6::US6_Word(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: i32 = v1.wrapping_add(2i32);
            let mut v6: US2 = US2::US2_Apply;
            let mut v7: US3 = US3::US3_Unchecked;
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRules); } CASE.with(|case| case.clone()) };
            header_options_8(v0.clone(), v5, v4.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone())
        }
        _ => {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expected a path or glob"); } LIT.with(|lit| lit.clone()) };
            US8::US8_Rejected(v18.clone())
        }
    }
}
fn count_at_12(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US11 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("once"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = word_is_9(v0.clone(), v1, v2.clone());
    if v3 {
        let mut v4: i32 = v1.wrapping_add(1i32);
        let mut v5: US4 = US4::US4_Once;
        US11::US11_Parsed(v5.clone(), v4)
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = word_is_9(v0.clone(), v1, v7.clone());
        if v8 {
            let mut v9: i32 = v1.wrapping_add(1i32);
            let mut v10: US4 = US4::US4_Every;
            US11::US11_Parsed(v10.clone(), v9)
        } else {
            let mut v12: i32 = v1.wrapping_add(1i32);
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("times"); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = word_is_9(v0.clone(), v12, v13.clone());
            if v14 {
                let mut v15: US6 = token_at_4(v0.clone(), v1);
                match &v15 {
                    US6::US6_Word(v16) => {
                        let mut v16: Rc<str> = v16.clone();
                        let mut v17: bool = !v16.is_empty() && v16.bytes().all(|byte| byte.is_ascii_digit());
                        if v17 {
                            let mut v18: u64 = v16.trim().parse::<u64>().unwrap_or(0);
                            let mut v19: i32 = v1.wrapping_add(2i32);
                            let mut v20: US4 = US4::US4_Exactly(v18);
                            US11::US11_Parsed(v20.clone(), v19)
                        } else {
                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: not a count: "); } LIT.with(|lit| lit.clone()) };
                            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v16, &*v23, &*v23, &*v23].concat());
                            US11::US11_Rejected(v24.clone())
                        }
                    }
                    _ => {
                        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected once, all or <n> times"); } LIT.with(|lit| lit.clone()) };
                        US11::US11_Rejected(v27.clone())
                    }
                }
            } else {
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected once, all or <n> times"); } LIT.with(|lit| lit.clone()) };
                US11::US11_Rejected(v30.clone())
            }
        }
    }
}
fn quoted_problem_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": \""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v1, &*v3, &*v4].concat());
    v5.clone()
}
fn rule_at_11(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US10 {
    let mut v2: i32 = v1.wrapping_add(1i32);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pattern"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = word_is_9(v0.clone(), v2, v3.clone());
    let mut v7: US5 = if v4 {
        US5::US5_Pattern
    } else {
        US5::US5_Literal
    };
    let mut v9: i32 = if v4 {
        let mut v8: i32 = v1.wrapping_add(2i32);
        v8
    } else {
        v2
    };
    let mut v10: US11 = count_at_12(v0.clone(), v9);
    match &v10 {
        US11::US11_Parsed(v13, v14) => {
            let mut v13: US4 = v13.clone();
            let mut v14: i32 = *v14;
            let mut v15: US6 = token_at_4(v0.clone(), v14);
            match &v15 {
                US6::US6_Quoted(v16) => {
                    let mut v16: Rc<str> = v16.clone();
                    let mut v17: i32 = v14.wrapping_add(1i32);
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("with"); } LIT.with(|lit| lit.clone()) };
                    let mut v19: bool = word_is_9(v0.clone(), v17, v18.clone());
                    if v19 {
                        let mut v20: i32 = v14.wrapping_add(2i32);
                        let mut v21: US6 = token_at_4(v0.clone(), v20);
                        match &v21 {
                            US6::US6_Quoted(v22) => {
                                let mut v22: Rc<str> = v22.clone();
                                let mut v23: i32 = v14.wrapping_add(3i32);
                                let mut v24: US6 = token_at_4(v0.clone(), v23);
                                match &v24 {
                                    US6::US6_StatementEnd => {
                                        US10::US10_Parsed(v16.clone(), v22.clone(), v13.clone(), v7.clone(), v23)
                                    }
                                    _ => {
                                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: unexpected text after the replacement"); } LIT.with(|lit| lit.clone()) };
                                        US10::US10_Rejected(v26.clone())
                                    }
                                }
                            }
                            US6::US6_Unterminated(v29) => {
                                let mut v29: Rc<str> = v29.clone();
                                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
                                let mut v31: Rc<str> = quoted_problem_13(v30.clone(), v29.clone());
                                US10::US10_Rejected(v31.clone())
                            }
                            _ => {
                                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected a quoted replacement after with"); } LIT.with(|lit| lit.clone()) };
                                US10::US10_Rejected(v33.clone())
                            }
                        }
                    } else {
                        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected with after the anchor"); } LIT.with(|lit| lit.clone()) };
                        US10::US10_Rejected(v37.clone())
                    }
                }
                US6::US6_Unterminated(v40) => {
                    let mut v40: Rc<str> = v40.clone();
                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
                    let mut v42: Rc<str> = quoted_problem_13(v41.clone(), v40.clone());
                    US10::US10_Rejected(v42.clone())
                }
                _ => {
                    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected a quoted anchor"); } LIT.with(|lit| lit.clone()) };
                    US10::US10_Rejected(v44.clone())
                }
            }
        }
        US11::US11_Rejected(v11) => {
            let mut v11: Rc<str> = v11.clone();
            US10::US10_Rejected(v11.clone())
        }
    }
}
fn patch_stray_line_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((&*v0).split(char::from(10u8)).find(|line| !(line.is_empty() || line.starts_with("@") || line.starts_with(char::from(32u8)) || line.starts_with(char::from(45u8)) || line.starts_with(char::from(43u8)))).unwrap_or(""));
    v1.clone()
}
fn hunks_of_16(mut v0: Rc<str>) -> std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>> {
    let mut v1: std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>> = { let newline = char::from(10u8).to_string(); let mut hunks: Vec<(std::rc::Rc<str>, std::rc::Rc<str>)> = Vec::new(); let mut old: Vec<&str> = Vec::new(); let mut new: Vec<&str> = Vec::new(); for line in (&*v0).split(char::from(10u8)).chain(std::iter::once("@")) { if line.starts_with("@") { if !old.is_empty() || !new.is_empty() { hunks.push((std::rc::Rc::from(old.join(newline.as_str())), std::rc::Rc::from(new.join(newline.as_str())))); } old.clear(); new.clear(); } else if line.is_empty() { old.push(line); new.push(line); } else if let Some(kept) = line.strip_prefix(char::from(32u8)) { old.push(kept); new.push(kept); } else if let Some(removed) = line.strip_prefix(char::from(45u8)) { old.push(removed); } else if let Some(added) = line.strip_prefix(char::from(43u8)) { new.push(added); } } std::rc::Rc::new(hunks) };
    v1.clone()
}
fn patch_at_14(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US12 {
    let mut v2: i32 = v1.wrapping_add(1i32);
    let mut v3: US6 = token_at_4(v0.clone(), v2);
    match &v3 {
        US6::US6_Quoted(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: i32 = v1.wrapping_add(2i32);
            let mut v6: US6 = token_at_4(v0.clone(), v5);
            match &v6 {
                US6::US6_StatementEnd => {
                    let mut v7: Rc<str> = patch_stray_line_15(v4.clone());
                    let mut v8: std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>> = hunks_of_16(v4.clone());
                    let mut v9: bool = v7.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v9 {
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: not a hunk line"); } LIT.with(|lit| lit.clone()) };
                        let mut v11: Rc<str> = quoted_problem_13(v10.clone(), v7.clone());
                        US12::US12_Rejected(v11.clone())
                    } else {
                        let mut v13: bool = v8.iter().any(|hunk| hunk.0 == hunk.1);
                        if v13 {
                            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: a hunk changes nothing"); } LIT.with(|lit| lit.clone()) };
                            US12::US12_Rejected(v14.clone())
                        } else {
                            let mut v16: bool = v8.iter().any(|hunk| hunk.0.is_empty());
                            if v16 {
                                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: a hunk needs context or removed lines to anchor it"); } LIT.with(|lit| lit.clone()) };
                                US12::US12_Rejected(v17.clone())
                            } else {
                                US12::US12_Parsed(v8.clone(), v5)
                            }
                        }
                    }
                }
                _ => {
                    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: unexpected text after the hunks"); } LIT.with(|lit| lit.clone()) };
                    US12::US12_Rejected(v23.clone())
                }
            }
        }
        US6::US6_Unterminated(v26) => {
            let mut v26: Rc<str> = v26.clone();
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = quoted_problem_13(v27.clone(), v26.clone());
            US12::US12_Rejected(v28.clone())
        }
        _ => {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: expected a <<< block of hunks"); } LIT.with(|lit| lit.clone()) };
            US12::US12_Rejected(v30.clone())
        }
    }
}
fn hunk_rules_17(mut v0: std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    let mut v3: i32 = v0.len() as i32;
    let mut v4: bool = v1 < v3;
    if v4 {
        let mut v5: Rc<str> = v0[v1 as usize].0.clone();
        let mut v6: Rc<str> = v0[v1 as usize].1.clone();
        let mut v7: i32 = v1.wrapping_add(1i32);
        let mut v8: Rc<UH1> = hunk_rules_17(v0.clone(), v7, v2.clone());
        let mut v9: US4 = US4::US4_Once;
        let mut v10: US5 = US5::US5_Literal;
        Rc::new(UH1::UH1_Rule(v5.clone(), v6.clone(), v9.clone(), v10.clone(), v8.clone()))
    } else {
        v2.clone()
    }
}
fn rules_at_10(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US9 {
    let mut v2: i32 = skip_ends_3(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = word_is_9(v0.clone(), v2, v3.clone());
    if v4 {
        let mut v5: US10 = rule_at_11(v0.clone(), v2);
        match &v5 {
            US10::US10_Parsed(v8, v9, v10, v11, v12) => {
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: US4 = v10.clone();
                let mut v11: US5 = v11.clone();
                let mut v12: i32 = *v12;
                let mut v13: US9 = rules_at_10(v0.clone(), v12);
                match &v13 {
                    US9::US9_Parsed(v16, v17) => {
                        let mut v16: Rc<UH1> = v16.clone();
                        let mut v17: i32 = *v17;
                        let mut v18: Rc<UH1> = Rc::new(UH1::UH1_Rule(v8.clone(), v9.clone(), v10.clone(), v11.clone(), v16.clone()));
                        US9::US9_Parsed(v18.clone(), v17)
                    }
                    US9::US9_Rejected(v14) => {
                        let mut v14: Rc<str> = v14.clone();
                        US9::US9_Rejected(v14.clone())
                    }
                }
            }
            US10::US10_Rejected(v6) => {
                let mut v6: Rc<str> = v6.clone();
                US9::US9_Rejected(v6.clone())
            }
        }
    } else {
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch"); } LIT.with(|lit| lit.clone()) };
        let mut v25: bool = word_is_9(v0.clone(), v2, v24.clone());
        if v25 {
            let mut v26: US12 = patch_at_14(v0.clone(), v2);
            match &v26 {
                US12::US12_Parsed(v29, v30) => {
                    let mut v29: std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>> = v29.clone();
                    let mut v30: i32 = *v30;
                    let mut v31: US9 = rules_at_10(v0.clone(), v30);
                    match &v31 {
                        US9::US9_Parsed(v34, v35) => {
                            let mut v34: Rc<UH1> = v34.clone();
                            let mut v35: i32 = *v35;
                            let mut v36: i32 = 0i32;
                            let mut v37: Rc<UH1> = hunk_rules_17(v29.clone(), v36, v34.clone());
                            US9::US9_Parsed(v37.clone(), v35)
                        }
                        US9::US9_Rejected(v32) => {
                            let mut v32: Rc<str> = v32.clone();
                            US9::US9_Rejected(v32.clone())
                        }
                    }
                }
                US12::US12_Rejected(v27) => {
                    let mut v27: Rc<str> = v27.clone();
                    US9::US9_Rejected(v27.clone())
                }
            }
        } else {
            let mut v43: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRules); } CASE.with(|case| case.clone()) };
            US9::US9_Parsed(v43.clone(), v2)
        }
    }
}
fn edit_at_6(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US7 {
    let mut v2: US8 = edit_header_at_7(v0.clone(), v1);
    match &v2 {
        US8::US8_Parsed(v5, v6, v7, v8, v9, v10) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: US2 = v6.clone();
            let mut v7: US3 = v7.clone();
            let mut v8: Rc<str> = v8.clone();
            let mut v9: Rc<UH1> = v9.clone();
            let mut v10: i32 = *v10;
            let mut v11: US9 = rules_at_10(v0.clone(), v10);
            match &v11 {
                US9::US9_Parsed(v14, v15) => {
                    let mut v14: Rc<UH1> = v14.clone();
                    let mut v15: i32 = *v15;
                    match &*v14 {
                        UH1::UH1_NoRules => {
                            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit "); } LIT.with(|lit| lit.clone()) };
                            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": no replace or patch lines under it"); } LIT.with(|lit| lit.clone()) };
                            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v5, &*v17, &*v18, &*v18].concat());
                            US7::US7_Rejected(v19.clone())
                        }
                        _ => {
                            let mut v21: US1 = US1::US1_EditStep(v5.clone(), v6.clone(), v7.clone(), v8.clone(), v14.clone());
                            US7::US7_Parsed(v21.clone(), v15)
                        }
                    }
                }
                US9::US9_Rejected(v12) => {
                    let mut v12: Rc<str> = v12.clone();
                    US7::US7_Rejected(v12.clone())
                }
            }
        }
        US8::US8_Rejected(v3) => {
            let mut v3: Rc<str> = v3.clone();
            US7::US7_Rejected(v3.clone())
        }
    }
}
fn show_at_18(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US7 {
    let mut v2: i32 = v1.wrapping_add(1i32);
    let mut v3: US6 = token_at_4(v0.clone(), v2);
    match &v3 {
        US6::US6_Word(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: i32 = v1.wrapping_add(2i32);
            let mut v6: US6 = token_at_4(v0.clone(), v5);
            match &v6 {
                US6::US6_StatementEnd => {
                    let mut v7: US1 = US1::US1_ShowStep(v4.clone());
                    US7::US7_Parsed(v7.clone(), v5)
                }
                _ => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show: expected one path"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_Rejected(v9.clone())
                }
            }
        }
        _ => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show: expected a path"); } LIT.with(|lit| lit.clone()) };
            US7::US7_Rejected(v12.clone())
        }
    }
}
fn query_words_20(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>) -> US13 {
    loop {
        let mut v3: US6 = token_at_4(v0.clone(), v1);
        match &v3 {
            US6::US6_Quoted(v15) => {
                let mut v15: Rc<str> = v15.clone();
                let mut v16: i32 = v1.wrapping_add(1i32);
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("q"); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v15, &*v18, &*v18, &*v18].concat());
                let mut v20: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = if v20 {
                    v19.clone()
                } else {
                    let mut v21: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
                    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v21, &*v19, &*v18, &*v18].concat());
                    v22.clone()
                };
                (v0, v1, v2) = (v0.clone(), v16, v23.clone());
                continue;
            }
            US6::US6_StatementEnd => {
                return US13::US13_Parsed(v2.clone(), v1);
            }
            US6::US6_Unterminated(v25) => {
                let mut v25: Rc<str> = v25.clone();
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = quoted_problem_13(v26.clone(), v25.clone());
                return US13::US13_Rejected(v27.clone());
            }
            US6::US6_Word(v5) => {
                let mut v5: Rc<str> = v5.clone();
                let mut v6: i32 = v1.wrapping_add(1i32);
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("w"); } LIT.with(|lit| lit.clone()) };
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v5, &*v8, &*v8, &*v8].concat());
                let mut v10: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = if v10 {
                    v9.clone()
                } else {
                    let mut v11: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
                    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v11, &*v9, &*v8, &*v8].concat());
                    v12.clone()
                };
                (v0, v1, v2) = (v0.clone(), v6, v13.clone());
                continue;
            }
        }
    }
}
fn query_at_19(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>) -> US7 {
    let mut v3: i32 = v1.wrapping_add(1i32);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: US13 = query_words_20(v0.clone(), v3, v4.clone());
    match &v5 {
        US13::US13_Parsed(v8, v9) => {
            let mut v8: Rc<str> = v8.clone();
            let mut v9: i32 = *v9;
            let mut v10: US1 = US1::US1_QueryStep(v2.clone(), v8.clone());
            US7::US7_Parsed(v10.clone(), v9)
        }
        US13::US13_Rejected(v6) => {
            let mut v6: Rc<str> = v6.clone();
            US7::US7_Rejected(v6.clone())
        }
    }
}
fn verb_family_word_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = eoie_do_verbs::eoie_do_verb_family(&*v0);
    v1.clone()
}
fn step_at_5(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US7 {
    let mut v2: US6 = token_at_4(v0.clone(), v1);
    match &v2 {
        US6::US6_Quoted(v30) => {
            let mut v30: Rc<str> = v30.clone();
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected quoted text"); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = quoted_problem_13(v31.clone(), v30.clone());
            US7::US7_Rejected(v32.clone())
        }
        US6::US6_StatementEnd => {
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("empty statement"); } LIT.with(|lit| lit.clone()) };
            US7::US7_Rejected(v34.clone())
        }
        US6::US6_Unterminated(v26) => {
            let mut v26: Rc<str> = v26.clone();
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = quoted_problem_13(v27.clone(), v26.clone());
            US7::US7_Rejected(v28.clone())
        }
        US6::US6_Word(v3) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            if v4 {
                edit_at_6(v0.clone(), v1)
            } else {
                let mut v6: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                if v6 {
                    show_at_18(v0.clone(), v1)
                } else {
                    let mut v8: Rc<str> = verb_family_word_21(v3.clone());
                    let mut v9: bool = v8.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v9 {
                        query_at_19(v0.clone(), v1, v3.clone())
                    } else {
                        let mut v11: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
                        if v11 {
                            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: needs an edit line above it"); } LIT.with(|lit| lit.clone()) };
                            US7::US7_Rejected(v12.clone())
                        } else {
                            let mut v14: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch"); } LIT.with(|lit| lit.clone()) };
                            if v14 {
                                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: needs an edit line above it"); } LIT.with(|lit| lit.clone()) };
                                US7::US7_Rejected(v15.clone())
                            } else {
                                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unsupported statement: "); } LIT.with(|lit| lit.clone()) };
                                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v3, &*v18, &*v18, &*v18].concat());
                                US7::US7_Rejected(v19.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn steps_at_2(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US0 {
    let mut v2: i32 = skip_ends_3(v0.clone(), v1);
    let mut v3: i32 = v0.len() as i32;
    let mut v4: bool = v2 >= v3;
    if v4 {
        let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Done); } CASE.with(|case| case.clone()) };
        US0::US0_Parsed(v5.clone(), v2)
    } else {
        let mut v7: US7 = step_at_5(v0.clone(), v2);
        match &v7 {
            US7::US7_Parsed(v10, v11) => {
                let mut v10: US1 = v10.clone();
                let mut v11: i32 = *v11;
                let mut v12: US0 = steps_at_2(v0.clone(), v11);
                match &v12 {
                    US0::US0_Parsed(v15, v16) => {
                        let mut v15: Rc<UH0> = v15.clone();
                        let mut v16: i32 = *v16;
                        let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Step(v10.clone(), v15.clone()));
                        US0::US0_Parsed(v17.clone(), v16)
                    }
                    US0::US0_Rejected(v13) => {
                        let mut v13: Rc<str> = v13.clone();
                        US0::US0_Rejected(v13.clone())
                    }
                }
            }
            US7::US7_Rejected(v8) => {
                let mut v8: Rc<str> = v8.clone();
                US0::US0_Rejected(v8.clone())
            }
        }
    }
}
fn tokens_of_22(mut v0: Rc<str>) -> std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>> {
    let mut v1: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>> = { let newline = char::from(10u8); let semicolon = char::from(59u8); let quote = char::from(34u8); let backslash = char::from(92u8); let angle_open = char::from(60u8); let angle_close = char::from(62u8); let carriage = char::from(13u8); let chars: Vec<char> = (&*v0).chars().collect(); let length = chars.len(); let mut found: Vec<(i32, std::rc::Rc<str>)> = Vec::new(); let mut index = 0usize; while index < length { let character = chars[index]; if character == newline || character == semicolon { found.push((2, std::rc::Rc::from(""))); index += 1; } else if character.is_whitespace() { index += 1; } else if character == angle_open && index + 3 < length && chars[index + 1] == angle_open && chars[index + 2] == angle_open && (chars[index + 3] == newline || (chars[index + 3] == carriage && index + 4 < length && chars[index + 4] == newline)) { let mut content = index + 3; while chars[content] != newline { content += 1; } content += 1; let mut line = content; let mut closed: Option<(usize, usize)> = None; while line < length { let mut first = line; while first < length && (chars[first] == char::from(32u8) || chars[first] == char::from(9u8)) { first += 1; } if first + 3 <= length && chars[first] == angle_close && chars[first + 1] == angle_close && chars[first + 2] == angle_close { closed = Some((line, first + 3)); break; } while line < length && chars[line] != newline { line += 1; } line += 1; } match closed { Some((close_line, after)) => { let end = if close_line > content { close_line - 1 } else { content }; let text: String = chars[content..end].iter().filter(|c| **c != carriage).collect(); found.push((1, std::rc::Rc::from(text))); index = after; } None => { found.push((3, std::rc::Rc::from("<<<"))); index = length; } } } else if character == quote { let mut text = String::new(); let mut closed = false; index += 1; while index < length { let inner = chars[index]; index += 1; if inner == backslash { if index < length { let escaped = chars[index]; index += 1; if escaped == char::from(110u8) { text.push(newline); } else if escaped == char::from(116u8) { text.push(char::from(9u8)); } else if escaped == quote || escaped == backslash { text.push(escaped); } else { text.push(backslash); text.push(escaped); } } } else if inner == quote { closed = true; break; } else { text.push(inner); } } found.push((if closed { 1 } else { 3 }, std::rc::Rc::from(text))); } else { let start = index; while index < length && chars[index].is_whitespace() == false && chars[index] != semicolon && chars[index] != quote { index += 1; } let text: String = chars[start..index].iter().collect(); found.push((0, std::rc::Rc::from(text))); } } std::rc::Rc::new(found) };
    v1.clone()
}
fn program_of_1(mut v0: Rc<str>) -> US0 {
    let mut v1: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>> = tokens_of_22(v0.clone());
    let mut v2: i32 = 0i32;
    steps_at_2(v1.clone(), v2)
}
fn fields_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v4].join(&char::from(31u8).to_string()));
    v5.clone()
}
fn decimal_text_26(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn edit_record_24(mut v0: Rc<str>, mut v1: US2, mut v2: US3, mut v3: Rc<str>, mut v4: Rc<UH1>) -> Rc<str> {
    let mut v8: Rc<str> = match &v1 {
        US2::US2_Apply => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US2::US2_Preview => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    };
    match &v2 {
        US3::US3_Expected(v12, v13) => {
            let mut v12: u64 = *v12;
            let mut v13: u64 = *v13;
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = decimal_text_26(v12);
            let mut v16: Rc<str> = decimal_text_26(v13);
            fields_25(v14.clone(), v0.clone(), v8.clone(), v15.clone(), v16.clone())
        }
        US3::US3_Unchecked => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            fields_25(v9.clone(), v0.clone(), v8.clone(), v10.clone(), v10.clone())
        }
    }
}
fn records_joined_27(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v1, &*v4, &*v4].concat());
        v5.clone()
    }
}
fn excluded_encoded_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = fields_25(v3.clone(), v0.clone(), v4.clone(), v4.clone(), v4.clone());
        records_joined_27(v1.clone(), v5.clone())
    }
}
fn rule_record_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US4, mut v3: US5) -> Rc<str> {
    let mut v7: Rc<str> = match &v3 {
        US5::US5_Literal => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US5::US5_Pattern => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pattern"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
    };
    match &v2 {
        US4::US4_Every => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            fields_25(v7.clone(), v11.clone(), v12.clone(), v0.clone(), v1.clone())
        }
        US4::US4_Exactly(v14) => {
            let mut v14: u64 = *v14;
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("times"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = decimal_text_26(v14);
            fields_25(v7.clone(), v15.clone(), v16.clone(), v0.clone(), v1.clone())
        }
        US4::US4_Once => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("once"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
            fields_25(v7.clone(), v8.clone(), v9.clone(), v0.clone(), v1.clone())
        }
    }
}
fn rules_encoded_29(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH1::UH1_NoRules => {
                return v1.clone();
            }
            UH1::UH1_Rule(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: US4 = v4.clone();
                let mut v5: US5 = v5.clone();
                let mut v6: Rc<UH1> = v6.clone();
                let mut v7: Rc<str> = rule_record_30(v2.clone(), v3.clone(), v4.clone(), v5.clone());
                let mut v8: Rc<str> = records_joined_27(v1.clone(), v7.clone());
                (v0, v1) = (v6.clone(), v8.clone());
                continue;
            }
        }
    }
}
fn steps_encoded_23(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Done => {
                return v1.clone();
            }
            UH0::UH0_Step(v2, v3) => {
                let mut v2: US1 = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                match &v2 {
                    US1::US1_EditStep(v4, v5, v6, v7, v8) => {
                        let mut v4: Rc<str> = v4.clone();
                        let mut v5: US2 = v5.clone();
                        let mut v6: US3 = v6.clone();
                        let mut v7: Rc<str> = v7.clone();
                        let mut v8: Rc<UH1> = v8.clone();
                        let mut v9: Rc<str> = edit_record_24(v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone());
                        let mut v10: Rc<str> = records_joined_27(v1.clone(), v9.clone());
                        let mut v11: Rc<str> = excluded_encoded_28(v7.clone(), v10.clone());
                        let mut v12: Rc<str> = rules_encoded_29(v8.clone(), v11.clone());
                        (v0, v1) = (v3.clone(), v12.clone());
                        continue;
                    }
                    US1::US1_QueryStep(v20, v21) => {
                        let mut v20: Rc<str> = v20.clone();
                        let mut v21: Rc<str> = v21.clone();
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
                        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v24: Rc<str> = fields_25(v22.clone(), v20.clone(), v21.clone(), v23.clone(), v23.clone());
                        let mut v25: Rc<str> = records_joined_27(v1.clone(), v24.clone());
                        (v0, v1) = (v3.clone(), v25.clone());
                        continue;
                    }
                    US1::US1_ShowStep(v14) => {
                        let mut v14: Rc<str> = v14.clone();
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v17: Rc<str> = fields_25(v15.clone(), v14.clone(), v16.clone(), v16.clone(), v16.clone());
                        let mut v18: Rc<str> = records_joined_27(v1.clone(), v17.clone());
                        (v0, v1) = (v3.clone(), v18.clone());
                        continue;
                    }
                }
            }
        }
    }
}
fn compiled_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = !v0.contains(char::from(30u8)) && !v0.contains(char::from(31u8));
    if v1 {
        let mut v2: US0 = program_of_1(v0.clone());
        match &v2 {
            US0::US0_Parsed(v8, v9) => {
                let mut v8: Rc<UH0> = v8.clone();
                let mut v9: i32 = *v9;
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = steps_encoded_23(v8.clone(), v12.clone());
                let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v13, &*v12, &*v12].concat());
                v14.clone()
            }
            US0::US0_Rejected(v3) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rejected"); } LIT.with(|lit| lit.clone()) };
                let mut v5: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v5, &*v3, &*v6, &*v6].concat());
                v7.clone()
            }
        }
    } else {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rejected"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("control characters 30 and 31 are reserved"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v19, &*v20, &*v20].concat());
        v21.clone()
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        compiled_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_program_compiled(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
