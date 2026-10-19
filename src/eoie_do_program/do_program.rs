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
enum UH1 {
    UH1_NoRules,
    UH1_Rule(Rc<str>, Rc<str>, US4, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_EditStep(Rc<str>, US2, US3, Rc<UH1>),
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
enum US5 {
    US5_Word(Rc<str>),
    US5_Quoted(Rc<str>),
    US5_StatementEnd,
    US5_Unterminated(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_Parsed(US1, i32),
    US6_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_Parsed(Rc<str>, US2, US3, Rc<UH1>, i32),
    US7_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US8 {
    US8_Parsed(Rc<UH1>, i32),
    US8_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US9 {
    US9_Parsed(Rc<str>, Rc<str>, US4, i32),
    US9_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US10 {
    US10_Parsed(US4, i32),
    US10_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US11 {
    US11_Parsed(Rc<str>, i32),
    US11_Rejected(Rc<str>),
}
fn token_at_4(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US5 {
    let mut v2: i32 = v0.len() as i32;
    let mut v3: bool = v1 >= v2;
    if v3 {
        US5::US5_StatementEnd
    } else {
        let mut v5: i32 = v0[v1 as usize].0;
        let mut v6: Rc<str> = v0[v1 as usize].1.clone();
        let mut v7: bool = v5 == 0i32;
        if v7 {
            US5::US5_Word(v6.clone())
        } else {
            let mut v9: bool = v5 == 1i32;
            if v9 {
                US5::US5_Quoted(v6.clone())
            } else {
                let mut v11: bool = v5 == 2i32;
                if v11 {
                    US5::US5_StatementEnd
                } else {
                    US5::US5_Unterminated(v6.clone())
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
            let mut v4: US5 = token_at_4(v0.clone(), v1);
            match &v4 {
                US5::US5_StatementEnd => {
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
    let mut v3: US5 = token_at_4(v0.clone(), v1);
    match &v3 {
        US5::US5_Word(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: bool = v4.clone() == v2.clone();
            v5
        }
        _ => {
            false
        }
    }
}
fn header_options_8(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>, mut v3: US2, mut v4: US3, mut v5: Rc<UH1>) -> US7 {
    loop {
        let mut v6: US5 = token_at_4(v0.clone(), v1);
        match &v6 {
            US5::US5_StatementEnd => {
                return US7::US7_Parsed(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v1);
            }
            US5::US5_Word(v8) => {
                let mut v8: Rc<str> = v8.clone();
                let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dry-run"); } LIT.with(|lit| lit.clone()) };
                if v9 {
                    let mut v10: i32 = v1.wrapping_add(1i32);
                    let mut v11: US2 = US2::US2_Preview;
                    (v0, v1, v2, v3, v4, v5) = (v0.clone(), v10, v2.clone(), v11.clone(), v4.clone(), v5.clone());
                    continue;
                } else {
                    let mut v13: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expect"); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = if v13 {
                        let mut v14: i32 = v1.wrapping_add(1i32);
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
                        word_is_9(v0.clone(), v14, v15.clone())
                    } else {
                        false
                    };
                    let mut v21: bool = if v17 {
                        let mut v18: i32 = v1.wrapping_add(3i32);
                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacements"); } LIT.with(|lit| lit.clone()) };
                        word_is_9(v0.clone(), v18, v19.clone())
                    } else {
                        false
                    };
                    if v21 {
                        let mut v22: i32 = v1.wrapping_add(2i32);
                        let mut v23: US5 = token_at_4(v0.clone(), v22);
                        match &v23 {
                            US5::US5_Word(v24) => {
                                let mut v24: Rc<str> = v24.clone();
                                let mut v25: i32 = v1.wrapping_add(4i32);
                                let mut v26: US5 = token_at_4(v0.clone(), v25);
                                match &v26 {
                                    US5::US5_Word(v27) => {
                                        let mut v27: Rc<str> = v27.clone();
                                        let mut v28: bool = !v24.is_empty() && v24.bytes().all(|byte| byte.is_ascii_digit());
                                        let mut v30: bool = if v28 {
                                            let mut v29: bool = !v27.is_empty() && v27.bytes().all(|byte| byte.is_ascii_digit());
                                            v29
                                        } else {
                                            false
                                        };
                                        if v30 {
                                            let mut v31: i32 = v1.wrapping_add(5i32);
                                            let mut v32: u64 = v24.trim().parse::<u64>().unwrap_or(0);
                                            let mut v33: u64 = v27.trim().parse::<u64>().unwrap_or(0);
                                            let mut v34: US3 = US3::US3_Expected(v32, v33);
                                            (v0, v1, v2, v3, v4, v5) = (v0.clone(), v31, v2.clone(), v3.clone(), v34.clone(), v5.clone());
                                            continue;
                                        } else {
                                            let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expect files <n> replacements <n>"); } LIT.with(|lit| lit.clone()) };
                                            return US7::US7_Rejected(v36.clone());
                                        }
                                    }
                                    _ => {
                                        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expect files <n> replacements <n>"); } LIT.with(|lit| lit.clone()) };
                                        return US7::US7_Rejected(v39.clone());
                                    }
                                }
                            }
                            _ => {
                                let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expect files <n> replacements <n>"); } LIT.with(|lit| lit.clone()) };
                                return US7::US7_Rejected(v42.clone());
                            }
                        }
                    } else {
                        let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: unknown option "); } LIT.with(|lit| lit.clone()) };
                        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v8, &*v46, &*v46, &*v46].concat());
                        return US7::US7_Rejected(v47.clone());
                    }
                }
            }
            _ => {
                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: unexpected quoted text"); } LIT.with(|lit| lit.clone()) };
                return US7::US7_Rejected(v51.clone());
            }
        }
    }
}
fn edit_header_at_7(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US7 {
    let mut v2: i32 = v1.wrapping_add(1i32);
    let mut v3: US5 = token_at_4(v0.clone(), v2);
    match &v3 {
        US5::US5_Quoted(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: i32 = v1.wrapping_add(2i32);
            let mut v12: US2 = US2::US2_Apply;
            let mut v13: US3 = US3::US3_Unchecked;
            let mut v14: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRules); } CASE.with(|case| case.clone()) };
            header_options_8(v0.clone(), v11, v10.clone(), v12.clone(), v13.clone(), v14.clone())
        }
        US5::US5_Word(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: i32 = v1.wrapping_add(2i32);
            let mut v6: US2 = US2::US2_Apply;
            let mut v7: US3 = US3::US3_Unchecked;
            let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRules); } CASE.with(|case| case.clone()) };
            header_options_8(v0.clone(), v5, v4.clone(), v6.clone(), v7.clone(), v8.clone())
        }
        _ => {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expected a path or glob"); } LIT.with(|lit| lit.clone()) };
            US7::US7_Rejected(v16.clone())
        }
    }
}
fn count_at_12(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US10 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("once"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = word_is_9(v0.clone(), v1, v2.clone());
    if v3 {
        let mut v4: i32 = v1.wrapping_add(1i32);
        let mut v5: US4 = US4::US4_Once;
        US10::US10_Parsed(v5.clone(), v4)
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = word_is_9(v0.clone(), v1, v7.clone());
        if v8 {
            let mut v9: i32 = v1.wrapping_add(1i32);
            let mut v10: US4 = US4::US4_Every;
            US10::US10_Parsed(v10.clone(), v9)
        } else {
            let mut v12: i32 = v1.wrapping_add(1i32);
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("times"); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = word_is_9(v0.clone(), v12, v13.clone());
            if v14 {
                let mut v15: US5 = token_at_4(v0.clone(), v1);
                match &v15 {
                    US5::US5_Word(v16) => {
                        let mut v16: Rc<str> = v16.clone();
                        let mut v17: bool = !v16.is_empty() && v16.bytes().all(|byte| byte.is_ascii_digit());
                        if v17 {
                            let mut v18: u64 = v16.trim().parse::<u64>().unwrap_or(0);
                            let mut v19: i32 = v1.wrapping_add(2i32);
                            let mut v20: US4 = US4::US4_Exactly(v18);
                            US10::US10_Parsed(v20.clone(), v19)
                        } else {
                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: not a count: "); } LIT.with(|lit| lit.clone()) };
                            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v16, &*v23, &*v23, &*v23].concat());
                            US10::US10_Rejected(v24.clone())
                        }
                    }
                    _ => {
                        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected once, all or <n> times"); } LIT.with(|lit| lit.clone()) };
                        US10::US10_Rejected(v27.clone())
                    }
                }
            } else {
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected once, all or <n> times"); } LIT.with(|lit| lit.clone()) };
                US10::US10_Rejected(v30.clone())
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
fn rule_at_11(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US9 {
    let mut v2: i32 = v1.wrapping_add(1i32);
    let mut v3: US10 = count_at_12(v0.clone(), v2);
    match &v3 {
        US10::US10_Parsed(v6, v7) => {
            let mut v6: US4 = v6.clone();
            let mut v7: i32 = *v7;
            let mut v8: US5 = token_at_4(v0.clone(), v7);
            match &v8 {
                US5::US5_Quoted(v9) => {
                    let mut v9: Rc<str> = v9.clone();
                    let mut v10: i32 = v7.wrapping_add(1i32);
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("with"); } LIT.with(|lit| lit.clone()) };
                    let mut v12: bool = word_is_9(v0.clone(), v10, v11.clone());
                    if v12 {
                        let mut v13: i32 = v7.wrapping_add(2i32);
                        let mut v14: US5 = token_at_4(v0.clone(), v13);
                        match &v14 {
                            US5::US5_Quoted(v15) => {
                                let mut v15: Rc<str> = v15.clone();
                                let mut v16: i32 = v7.wrapping_add(3i32);
                                let mut v17: US5 = token_at_4(v0.clone(), v16);
                                match &v17 {
                                    US5::US5_StatementEnd => {
                                        US9::US9_Parsed(v9.clone(), v15.clone(), v6.clone(), v16)
                                    }
                                    _ => {
                                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: unexpected text after the replacement"); } LIT.with(|lit| lit.clone()) };
                                        US9::US9_Rejected(v19.clone())
                                    }
                                }
                            }
                            US5::US5_Unterminated(v22) => {
                                let mut v22: Rc<str> = v22.clone();
                                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
                                let mut v24: Rc<str> = quoted_problem_13(v23.clone(), v22.clone());
                                US9::US9_Rejected(v24.clone())
                            }
                            _ => {
                                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected a quoted replacement after with"); } LIT.with(|lit| lit.clone()) };
                                US9::US9_Rejected(v26.clone())
                            }
                        }
                    } else {
                        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected with after the anchor"); } LIT.with(|lit| lit.clone()) };
                        US9::US9_Rejected(v30.clone())
                    }
                }
                US5::US5_Unterminated(v33) => {
                    let mut v33: Rc<str> = v33.clone();
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
                    let mut v35: Rc<str> = quoted_problem_13(v34.clone(), v33.clone());
                    US9::US9_Rejected(v35.clone())
                }
                _ => {
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected a quoted anchor"); } LIT.with(|lit| lit.clone()) };
                    US9::US9_Rejected(v37.clone())
                }
            }
        }
        US10::US10_Rejected(v4) => {
            let mut v4: Rc<str> = v4.clone();
            US9::US9_Rejected(v4.clone())
        }
    }
}
fn rules_at_10(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US8 {
    let mut v2: i32 = skip_ends_3(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = word_is_9(v0.clone(), v2, v3.clone());
    if v4 {
        let mut v5: US9 = rule_at_11(v0.clone(), v2);
        match &v5 {
            US9::US9_Parsed(v8, v9, v10, v11) => {
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: US4 = v10.clone();
                let mut v11: i32 = *v11;
                let mut v12: US8 = rules_at_10(v0.clone(), v11);
                match &v12 {
                    US8::US8_Parsed(v15, v16) => {
                        let mut v15: Rc<UH1> = v15.clone();
                        let mut v16: i32 = *v16;
                        let mut v17: Rc<UH1> = Rc::new(UH1::UH1_Rule(v8.clone(), v9.clone(), v10.clone(), v15.clone()));
                        US8::US8_Parsed(v17.clone(), v16)
                    }
                    US8::US8_Rejected(v13) => {
                        let mut v13: Rc<str> = v13.clone();
                        US8::US8_Rejected(v13.clone())
                    }
                }
            }
            US9::US9_Rejected(v6) => {
                let mut v6: Rc<str> = v6.clone();
                US8::US8_Rejected(v6.clone())
            }
        }
    } else {
        let mut v23: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRules); } CASE.with(|case| case.clone()) };
        US8::US8_Parsed(v23.clone(), v2)
    }
}
fn edit_at_6(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US6 {
    let mut v2: US7 = edit_header_at_7(v0.clone(), v1);
    match &v2 {
        US7::US7_Parsed(v5, v6, v7, v8, v9) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: US2 = v6.clone();
            let mut v7: US3 = v7.clone();
            let mut v8: Rc<UH1> = v8.clone();
            let mut v9: i32 = *v9;
            let mut v10: US8 = rules_at_10(v0.clone(), v9);
            match &v10 {
                US8::US8_Parsed(v13, v14) => {
                    let mut v13: Rc<UH1> = v13.clone();
                    let mut v14: i32 = *v14;
                    match &*v13 {
                        UH1::UH1_NoRules => {
                            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit "); } LIT.with(|lit| lit.clone()) };
                            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": no replace lines under it"); } LIT.with(|lit| lit.clone()) };
                            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v5, &*v16, &*v17, &*v17].concat());
                            US6::US6_Rejected(v18.clone())
                        }
                        _ => {
                            let mut v20: US1 = US1::US1_EditStep(v5.clone(), v6.clone(), v7.clone(), v13.clone());
                            US6::US6_Parsed(v20.clone(), v14)
                        }
                    }
                }
                US8::US8_Rejected(v11) => {
                    let mut v11: Rc<str> = v11.clone();
                    US6::US6_Rejected(v11.clone())
                }
            }
        }
        US7::US7_Rejected(v3) => {
            let mut v3: Rc<str> = v3.clone();
            US6::US6_Rejected(v3.clone())
        }
    }
}
fn show_at_14(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US6 {
    let mut v2: i32 = v1.wrapping_add(1i32);
    let mut v3: US5 = token_at_4(v0.clone(), v2);
    match &v3 {
        US5::US5_Word(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: i32 = v1.wrapping_add(2i32);
            let mut v6: US5 = token_at_4(v0.clone(), v5);
            match &v6 {
                US5::US5_StatementEnd => {
                    let mut v7: US1 = US1::US1_ShowStep(v4.clone());
                    US6::US6_Parsed(v7.clone(), v5)
                }
                _ => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show: expected one path"); } LIT.with(|lit| lit.clone()) };
                    US6::US6_Rejected(v9.clone())
                }
            }
        }
        _ => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show: expected a path"); } LIT.with(|lit| lit.clone()) };
            US6::US6_Rejected(v12.clone())
        }
    }
}
fn query_words_16(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>) -> US11 {
    loop {
        let mut v3: US5 = token_at_4(v0.clone(), v1);
        match &v3 {
            US5::US5_Quoted(v15) => {
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
            US5::US5_StatementEnd => {
                return US11::US11_Parsed(v2.clone(), v1);
            }
            US5::US5_Unterminated(v25) => {
                let mut v25: Rc<str> = v25.clone();
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = quoted_problem_13(v26.clone(), v25.clone());
                return US11::US11_Rejected(v27.clone());
            }
            US5::US5_Word(v5) => {
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
fn query_at_15(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>) -> US6 {
    let mut v3: i32 = v1.wrapping_add(1i32);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: US11 = query_words_16(v0.clone(), v3, v4.clone());
    match &v5 {
        US11::US11_Parsed(v8, v9) => {
            let mut v8: Rc<str> = v8.clone();
            let mut v9: i32 = *v9;
            let mut v10: US1 = US1::US1_QueryStep(v2.clone(), v8.clone());
            US6::US6_Parsed(v10.clone(), v9)
        }
        US11::US11_Rejected(v6) => {
            let mut v6: Rc<str> = v6.clone();
            US6::US6_Rejected(v6.clone())
        }
    }
}
fn step_at_5(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US6 {
    let mut v2: US5 = token_at_4(v0.clone(), v1);
    match &v2 {
        US5::US5_Quoted(v65) => {
            let mut v65: Rc<str> = v65.clone();
            let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected quoted text"); } LIT.with(|lit| lit.clone()) };
            let mut v67: Rc<str> = quoted_problem_13(v66.clone(), v65.clone());
            US6::US6_Rejected(v67.clone())
        }
        US5::US5_StatementEnd => {
            let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("empty statement"); } LIT.with(|lit| lit.clone()) };
            US6::US6_Rejected(v69.clone())
        }
        US5::US5_Unterminated(v61) => {
            let mut v61: Rc<str> = v61.clone();
            let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
            let mut v63: Rc<str> = quoted_problem_13(v62.clone(), v61.clone());
            US6::US6_Rejected(v63.clone())
        }
        US5::US5_Word(v3) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            if v4 {
                edit_at_6(v0.clone(), v1)
            } else {
                let mut v6: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                if v6 {
                    show_at_14(v0.clone(), v1)
                } else {
                    let mut v8: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search"); } LIT.with(|lit| lit.clone()) };
                    let mut v10: bool = if v8 {
                        true
                    } else {
                        let mut v9: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read"); } LIT.with(|lit| lit.clone()) };
                        v9
                    };
                    let mut v12: bool = if v10 {
                        true
                    } else {
                        let mut v11: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
                        v11
                    };
                    let mut v14: bool = if v12 {
                        true
                    } else {
                        let mut v13: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("head"); } LIT.with(|lit| lit.clone()) };
                        v13
                    };
                    let mut v16: bool = if v14 {
                        true
                    } else {
                        let mut v15: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
                        v15
                    };
                    let mut v18: bool = if v16 {
                        true
                    } else {
                        let mut v17: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                        v17
                    };
                    let mut v20: bool = if v18 {
                        true
                    } else {
                        let mut v19: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash"); } LIT.with(|lit| lit.clone()) };
                        v19
                    };
                    let mut v22: bool = if v20 {
                        true
                    } else {
                        let mut v21: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
                        v21
                    };
                    let mut v24: bool = if v22 {
                        true
                    } else {
                        let mut v23: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exists"); } LIT.with(|lit| lit.clone()) };
                        v23
                    };
                    let mut v26: bool = if v24 {
                        true
                    } else {
                        let mut v25: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nearest"); } LIT.with(|lit| lit.clone()) };
                        v25
                    };
                    let mut v28: bool = if v26 {
                        true
                    } else {
                        let mut v27: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("json"); } LIT.with(|lit| lit.clone()) };
                        v27
                    };
                    let mut v30: bool = if v28 {
                        true
                    } else {
                        let mut v29: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
                        v29
                    };
                    let mut v32: bool = if v30 {
                        true
                    } else {
                        let mut v31: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table"); } LIT.with(|lit| lit.clone()) };
                        v31
                    };
                    let mut v34: bool = if v32 {
                        true
                    } else {
                        let mut v33: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dirs"); } LIT.with(|lit| lit.clone()) };
                        v33
                    };
                    let mut v36: bool = if v34 {
                        true
                    } else {
                        let mut v35: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff"); } LIT.with(|lit| lit.clone()) };
                        v35
                    };
                    let mut v38: bool = if v36 {
                        true
                    } else {
                        let mut v37: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tag"); } LIT.with(|lit| lit.clone()) };
                        v37
                    };
                    let mut v40: bool = if v38 {
                        true
                    } else {
                        let mut v39: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                        v39
                    };
                    let mut v42: bool = if v40 {
                        true
                    } else {
                        let mut v41: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
                        v41
                    };
                    let mut v44: bool = if v42 {
                        true
                    } else {
                        let mut v43: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("errors"); } LIT.with(|lit| lit.clone()) };
                        v43
                    };
                    let mut v46: bool = if v44 {
                        true
                    } else {
                        let mut v45: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("processes"); } LIT.with(|lit| lit.clone()) };
                        v45
                    };
                    let mut v48: bool = if v46 {
                        true
                    } else {
                        let mut v47: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
                        v47
                    };
                    if v48 {
                        query_at_15(v0.clone(), v1, v3.clone())
                    } else {
                        let mut v50: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
                        if v50 {
                            let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: needs an edit line above it"); } LIT.with(|lit| lit.clone()) };
                            US6::US6_Rejected(v51.clone())
                        } else {
                            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unsupported statement: "); } LIT.with(|lit| lit.clone()) };
                            let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v3, &*v54, &*v54, &*v54].concat());
                            US6::US6_Rejected(v55.clone())
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
        let mut v7: US6 = step_at_5(v0.clone(), v2);
        match &v7 {
            US6::US6_Parsed(v10, v11) => {
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
            US6::US6_Rejected(v8) => {
                let mut v8: Rc<str> = v8.clone();
                US0::US0_Rejected(v8.clone())
            }
        }
    }
}
fn tokens_of_17(mut v0: Rc<str>) -> std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>> {
    let mut v1: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>> = { let newline = char::from(10u8); let semicolon = char::from(59u8); let quote = char::from(34u8); let backslash = char::from(92u8); let mut found: Vec<(i32, std::rc::Rc<str>)> = Vec::new(); let mut characters = (&*v0).chars().peekable(); while let Some(character) = characters.next() { if character == newline || character == semicolon { found.push((2, std::rc::Rc::from(""))); } else if character.is_whitespace() { } else if character == quote { let mut text = String::new(); let mut closed = false; while let Some(inner) = characters.next() { if inner == backslash { match characters.next() { Some(escaped) if escaped == char::from(110u8) => text.push(newline), Some(escaped) if escaped == char::from(116u8) => text.push(char::from(9u8)), Some(escaped) if escaped == quote || escaped == backslash => text.push(escaped), Some(escaped) => { text.push(backslash); text.push(escaped); } None => {} } } else if inner == quote { closed = true; break; } else { text.push(inner); } } found.push((if closed { 1 } else { 3 }, std::rc::Rc::from(text))); } else { let mut text = String::from(character); while let Some(&next) = characters.peek() { if next.is_whitespace() || next == semicolon || next == quote { break; } text.push(next); characters.next(); } found.push((0, std::rc::Rc::from(text))); } } std::rc::Rc::new(found) };
    v1.clone()
}
fn program_of_1(mut v0: Rc<str>) -> US0 {
    let mut v1: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>> = tokens_of_17(v0.clone());
    let mut v2: i32 = 0i32;
    steps_at_2(v1.clone(), v2)
}
fn fields_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v4].join(&char::from(31u8).to_string()));
    v5.clone()
}
fn fields_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v3].join(&char::from(31u8).to_string()));
    v4.clone()
}
fn decimal_text_22(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn edit_record_19(mut v0: Rc<str>, mut v1: US2, mut v2: US3, mut v3: Rc<UH1>) -> Rc<str> {
    let mut v7: Rc<str> = match &v1 {
        US2::US2_Apply => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_Preview => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
    };
    match &v2 {
        US3::US3_Expected(v11, v12) => {
            let mut v11: u64 = *v11;
            let mut v12: u64 = *v12;
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = decimal_text_22(v11);
            let mut v15: Rc<str> = decimal_text_22(v12);
            fields_20(v13.clone(), v0.clone(), v7.clone(), v14.clone(), v15.clone())
        }
        US3::US3_Unchecked => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            fields_21(v8.clone(), v0.clone(), v7.clone(), v9.clone())
        }
    }
}
fn records_joined_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
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
fn rule_record_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US4) -> Rc<str> {
    match &v2 {
        US4::US4_Every => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            fields_20(v7.clone(), v8.clone(), v9.clone(), v0.clone(), v1.clone())
        }
        US4::US4_Exactly(v11) => {
            let mut v11: u64 = *v11;
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("times"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = decimal_text_22(v11);
            fields_20(v12.clone(), v13.clone(), v14.clone(), v0.clone(), v1.clone())
        }
        US4::US4_Once => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("once"); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
            fields_20(v3.clone(), v4.clone(), v5.clone(), v0.clone(), v1.clone())
        }
    }
}
fn rules_encoded_24(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH1::UH1_NoRules => {
                return v1.clone();
            }
            UH1::UH1_Rule(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: US4 = v4.clone();
                let mut v5: Rc<UH1> = v5.clone();
                let mut v6: Rc<str> = rule_record_25(v2.clone(), v3.clone(), v4.clone());
                let mut v7: Rc<str> = records_joined_23(v1.clone(), v6.clone());
                (v0, v1) = (v5.clone(), v7.clone());
                continue;
            }
        }
    }
}
fn fields_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v2, &*v2].join(&char::from(31u8).to_string()));
    v3.clone()
}
fn steps_encoded_18(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Done => {
                return v1.clone();
            }
            UH0::UH0_Step(v2, v3) => {
                let mut v2: US1 = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                match &v2 {
                    US1::US1_EditStep(v4, v5, v6, v7) => {
                        let mut v4: Rc<str> = v4.clone();
                        let mut v5: US2 = v5.clone();
                        let mut v6: US3 = v6.clone();
                        let mut v7: Rc<UH1> = v7.clone();
                        let mut v8: Rc<str> = edit_record_19(v4.clone(), v5.clone(), v6.clone(), v7.clone());
                        let mut v9: Rc<str> = records_joined_23(v1.clone(), v8.clone());
                        let mut v10: Rc<str> = rules_encoded_24(v7.clone(), v9.clone());
                        (v0, v1) = (v3.clone(), v10.clone());
                        continue;
                    }
                    US1::US1_QueryStep(v18, v19) => {
                        let mut v18: Rc<str> = v18.clone();
                        let mut v19: Rc<str> = v19.clone();
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
                        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v22: Rc<str> = fields_21(v20.clone(), v18.clone(), v19.clone(), v21.clone());
                        let mut v23: Rc<str> = records_joined_23(v1.clone(), v22.clone());
                        (v0, v1) = (v3.clone(), v23.clone());
                        continue;
                    }
                    US1::US1_ShowStep(v12) => {
                        let mut v12: Rc<str> = v12.clone();
                        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v15: Rc<str> = fields_26(v13.clone(), v12.clone(), v14.clone());
                        let mut v16: Rc<str> = records_joined_23(v1.clone(), v15.clone());
                        (v0, v1) = (v3.clone(), v16.clone());
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
                let mut v13: Rc<str> = steps_encoded_18(v8.clone(), v12.clone());
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
