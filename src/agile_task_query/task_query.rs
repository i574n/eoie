#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_OnTeam(Rc<str>),
    US1_WithTag(Rc<str>),
    US1_WithPrefix(Rc<str>),
    US1_OnlyOpen,
    US1_OnlyReady,
    US1_OnlyBlocked,
}
#[derive(Clone)]
enum US0 {
    US0_FilterRead(US1, i32),
    US0_FlagRefused(Rc<str>),
    US0_NotQueryFlag,
}
#[derive(Clone)]
enum US2 {
    US2_ValueRead(Rc<str>),
    US2_ValueRefused(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_NoFilters,
    UH0_FilterThen(US1, Rc<UH0>),
}
fn flag_read_of_1(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team"); } LIT.with(|lit| lit.clone()) };
    let mut v28: US0 = if v2 {
        let mut v3: bool = v1.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = if v3 {
            let mut v4: Rc<str> = Rc::<str>::from(v1.to_lowercase());
            let mut v5: bool = v4.clone() == v1.clone();
            v5
        } else {
            false
        };
        let mut v10: bool = if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v1.find(&*v7).map(|i| i as i32).unwrap_or(-1);
            let mut v9: bool = v8 < 0i32;
            v9
        } else {
            false
        };
        let mut v14: bool = if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v12: i32 = v1.find(&*v11).map(|i| i as i32).unwrap_or(-1);
            let mut v13: bool = v12 < 0i32;
            v13
        } else {
            false
        };
        let mut v19: US2 = if v14 {
            US2::US2_ValueRead(v1.clone())
        } else {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team expects a lower-case team word, got "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v1));
            US2::US2_ValueRefused(v17.clone())
        };
        match &v19 {
            US2::US2_ValueRead(v20) => {
                let mut v20: Rc<str> = v20.clone();
                let mut v21: US1 = US1::US1_OnTeam(v20.clone());
                US0::US0_FilterRead(v21.clone(), 2i32)
            }
            US2::US2_ValueRefused(v23) => {
                let mut v23: Rc<str> = v23.clone();
                US0::US0_FlagRefused(v23.clone())
            }
        }
    } else {
        US0::US0_NotQueryFlag
    };
    let mut v43: US0 = match &v28 {
        US0::US0_NotQueryFlag => {
            let mut v29: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tag"); } LIT.with(|lit| lit.clone()) };
            if v29 {
                let mut v30: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v34: US2 = if v30 {
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tag expects a tag"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ValueRefused(v31.clone())
                } else {
                    US2::US2_ValueRead(v1.clone())
                };
                match &v34 {
                    US2::US2_ValueRead(v35) => {
                        let mut v35: Rc<str> = v35.clone();
                        let mut v36: US1 = US1::US1_WithTag(v35.clone());
                        US0::US0_FilterRead(v36.clone(), 2i32)
                    }
                    US2::US2_ValueRefused(v38) => {
                        let mut v38: Rc<str> = v38.clone();
                        US0::US0_FlagRefused(v38.clone())
                    }
                }
            } else {
                v28.clone()
            }
        }
        _ => {
            v28.clone()
        }
    };
    let mut v58: US0 = match &v43 {
        US0::US0_NotQueryFlag => {
            let mut v44: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prefix"); } LIT.with(|lit| lit.clone()) };
            if v44 {
                let mut v45: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v49: US2 = if v45 {
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prefix expects an ID prefix"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ValueRefused(v46.clone())
                } else {
                    US2::US2_ValueRead(v1.clone())
                };
                match &v49 {
                    US2::US2_ValueRead(v50) => {
                        let mut v50: Rc<str> = v50.clone();
                        let mut v51: US1 = US1::US1_WithPrefix(v50.clone());
                        US0::US0_FilterRead(v51.clone(), 2i32)
                    }
                    US2::US2_ValueRefused(v53) => {
                        let mut v53: Rc<str> = v53.clone();
                        US0::US0_FlagRefused(v53.clone())
                    }
                }
            } else {
                v43.clone()
            }
        }
        _ => {
            v43.clone()
        }
    };
    let mut v63: US0 = match &v58 {
        US0::US0_NotQueryFlag => {
            let mut v59: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--open"); } LIT.with(|lit| lit.clone()) };
            if v59 {
                let mut v60: US1 = US1::US1_OnlyOpen;
                US0::US0_FilterRead(v60.clone(), 1i32)
            } else {
                v58.clone()
            }
        }
        _ => {
            v58.clone()
        }
    };
    let mut v68: US0 = match &v63 {
        US0::US0_NotQueryFlag => {
            let mut v64: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ready"); } LIT.with(|lit| lit.clone()) };
            if v64 {
                let mut v65: US1 = US1::US1_OnlyReady;
                US0::US0_FilterRead(v65.clone(), 1i32)
            } else {
                v63.clone()
            }
        }
        _ => {
            v63.clone()
        }
    };
    match &v68 {
        US0::US0_NotQueryFlag => {
            let mut v69: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--blocked"); } LIT.with(|lit| lit.clone()) };
            if v69 {
                let mut v70: US1 = US1::US1_OnlyBlocked;
                US0::US0_FilterRead(v70.clone(), 1i32)
            } else {
                v68.clone()
            }
        }
        _ => {
            v68.clone()
        }
    }
}
fn flag_width_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: US0 = flag_read_of_1(v0.clone(), v1.clone());
    match &v2 {
        US0::US0_FilterRead(v3, v4) => {
            let mut v3: US1 = v3.clone();
            let mut v4: i32 = *v4;
            v4
        }
        US0::US0_FlagRefused(v5) => {
            let mut v5: Rc<str> = v5.clone();
            -1i32
        }
        US0::US0_NotQueryFlag => {
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        flag_width_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn flag_refusal_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US0 = flag_read_of_1(v0.clone(), v1.clone());
    match &v2 {
        US0::US0_FlagRefused(v3) => {
            let mut v3: Rc<str> = v3.clone();
            v3.clone()
        }
        _ => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        flag_refusal_2(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tag_listed_7(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(v0.replace(&*v5, &*v6));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v7));
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v2));
    let mut v10: i32 = v9.find(&*v4).map(|i| i as i32).unwrap_or(-1);
    let mut v11: bool = v10 >= 0i32;
    v11
}
fn on_team_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = tag_listed_7(v2.clone(), v0.clone());
    if v3 {
        true
    } else {
        let mut v4: Rc<str> = Rc::<str>::from(v0.to_uppercase());
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
        let mut v7: bool = v1.starts_with(&*v6);
        v7
    }
}
fn filter_passes_5(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool) -> bool {
    match &v0 {
        US1::US1_OnTeam(v5) => {
            let mut v5: Rc<str> = v5.clone();
            on_team_6(v5.clone(), v1.clone(), v2.clone())
        }
        US1::US1_OnlyBlocked => {
            let mut v18: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
            if v18 {
                true
            } else {
                let mut v19: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
                let mut v21: bool = if v19 {
                    true
                } else {
                    let mut v20: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
                    v20
                };
                if v21 {
                    let mut v22: bool = v4 == false;
                    v22
                } else {
                    false
                }
            }
        }
        US1::US1_OnlyOpen => {
            let mut v11: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
            if v11 {
                true
            } else {
                let mut v12: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
                v12
            }
        }
        US1::US1_OnlyReady => {
            let mut v14: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
            let mut v16: bool = if v14 {
                true
            } else {
                let mut v15: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
                v15
            };
            let mut v17: bool = v16 && v4;
            v17
        }
        US1::US1_WithPrefix(v9) => {
            let mut v9: Rc<str> = v9.clone();
            let mut v10: bool = v1.starts_with(&*v9);
            v10
        }
        US1::US1_WithTag(v7) => {
            let mut v7: Rc<str> = v7.clone();
            tag_listed_7(v2.clone(), v7.clone())
        }
    }
}
fn filters_pass_4(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v5, v6) => {
                let mut v5: US1 = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: bool = filter_passes_5(v5.clone(), v1.clone(), v2.clone(), v3.clone(), v4);
                if v7 {
                    (v0, v1, v2, v3, v4) = (v6.clone(), v1.clone(), v2.clone(), v3.clone(), v4);
                    continue;
                } else {
                    return false;
                }
            }
            UH0::UH0_NoFilters => {
                return true;
            }
        }
    }
}
fn query_token_9(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).split_whitespace().nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn filters_of_query_8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: Rc<str> = query_token_9(v0.clone(), v1);
        let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            return v2.clone();
        } else {
            let mut v5: i32 = v1.wrapping_add(1i32);
            let mut v6: Rc<str> = query_token_9(v0.clone(), v5);
            let mut v7: US0 = flag_read_of_1(v3.clone(), v6.clone());
            match &v7 {
                US0::US0_FilterRead(v8, v9) => {
                    let mut v8: US1 = v8.clone();
                    let mut v9: i32 = *v9;
                    let mut v10: i32 = v1.wrapping_add(v9);
                    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_FilterThen(v8.clone(), v2.clone()));
                    (v0, v1, v2) = (v0.clone(), v10, v11.clone());
                    continue;
                }
                _ => {
                    (v0, v1, v2) = (v0.clone(), v5, v2.clone());
                    continue;
                }
            }
        }
    }
}
fn query_passes_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool) -> bool {
    let mut v5: i32 = 0i32;
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = filters_of_query_8(v0.clone(), v5, v6.clone());
    filters_pass_4(v7.clone(), v1.clone(), v2.clone(), v3.clone(), v4)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool| -> bool {
        query_passes_3(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn filters_read_deps_11(mut v0: Rc<UH0>) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v1, v2) => {
                let mut v1: US1 = v1.clone();
                let mut v2: Rc<UH0> = v2.clone();
                let mut v4: bool = match &v1 {
                    US1::US1_OnlyBlocked => {
                        true
                    }
                    US1::US1_OnlyReady => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v4 {
                    return true;
                } else {
                    v0 = v2.clone();
                    continue;
                }
            }
            UH0::UH0_NoFilters => {
                return false;
            }
        }
    }
}
fn query_reads_deps_10(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = filters_of_query_8(v0.clone(), v1, v2.clone());
    filters_read_deps_11(v3.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        query_reads_deps_10(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn filters_text_13(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v2, v3) => {
                let mut v2: US1 = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v21: Rc<str> = match &v2 {
                    US1::US1_OnTeam(v4) => {
                        let mut v4: Rc<str> = v4.clone();
                        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team="); } LIT.with(|lit| lit.clone()) };
                        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
                        v6.clone()
                    }
                    US1::US1_OnlyBlocked => {
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
                        v15.clone()
                    }
                    US1::US1_OnlyOpen => {
                        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open"); } LIT.with(|lit| lit.clone()) };
                        v13.clone()
                    }
                    US1::US1_OnlyReady => {
                        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ready"); } LIT.with(|lit| lit.clone()) };
                        v14.clone()
                    }
                    US1::US1_WithPrefix(v10) => {
                        let mut v10: Rc<str> = v10.clone();
                        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prefix="); } LIT.with(|lit| lit.clone()) };
                        let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10));
                        v12.clone()
                    }
                    US1::US1_WithTag(v7) => {
                        let mut v7: Rc<str> = v7.clone();
                        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tag="); } LIT.with(|lit| lit.clone()) };
                        let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
                        v9.clone()
                    }
                };
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v21));
                let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v1));
                (v0, v1) = (v3.clone(), v24.clone());
                continue;
            }
            UH0::UH0_NoFilters => {
                return v1.clone();
            }
        }
    }
}
fn query_shown_of_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = filters_of_query_8(v0.clone(), v1, v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    filters_text_13(v3.clone(), v4.clone())
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        query_shown_of_12(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>"); } LIT.with(|lit| lit.clone()) };
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>, "); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tag <tag>"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>, --tag <tag>"); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prefix <ID prefix>"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>, --tag <tag>, "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prefix <ID prefix>"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team <team>, --tag <tag>, --prefix <ID prefix>"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        };
        let mut v11: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--open"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        } else {
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v1));
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--open"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            v15.clone()
        };
        let mut v17: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = if v17 {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ready"); } LIT.with(|lit| lit.clone()) };
            v18.clone()
        } else {
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v1));
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ready"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
            v21.clone()
        };
        let mut v23: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v23 {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--blocked"); } LIT.with(|lit| lit.clone()) };
            v24.clone()
        } else {
            let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v1));
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--blocked"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            v27.clone()
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_query_flag_width(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_query_flag_refusal(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_query_passes(v0: &str, v1: &str, v2: &str, v3: &str, v4: bool) -> bool {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4)
}
pub fn eoie_query_reads_deps(v0: &str) -> bool {
    closure3()(Rc::<str>::from(v0))
}
pub fn eoie_query_shown(v0: &str) -> Rc<str> {
    closure4()(Rc::<str>::from(v0))
}
pub fn eoie_query_usage() -> Rc<str> {
    closure5()()
}
