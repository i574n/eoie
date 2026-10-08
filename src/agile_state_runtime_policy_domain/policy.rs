#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)] #![recursion_limit = "256"]
use std::cell::RefCell;
use std::rc::Rc;
fn method0(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 1i32;
    if v2 {
        let mut v3: bool = v1 == 1i32;
        if v3 {
            2i32
        } else {
            1i32
        }
    } else {
        0i32
    }
}
fn closure0() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method1(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 1i32;
    if v2 {
        let mut v3: bool = v1 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn closure1() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method1(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method2(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 1i32;
    if v2 {
        let mut v3: bool = v1 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn closure2() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method2(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method3(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        0u64
    } else {
        1u64
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method3(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method4(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned|Active|Blocked|Paused|Done"); } LIT.with(|lit| lit.clone()) };
    let mut v2: u64 = v1.split("|").position(|item| item == &*v0).map(|index| index as u64).unwrap_or(u64::MAX);
    v2
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method5(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.trim());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v1.contains(&*v4);
        if v5 {
            1u64
        } else {
            0u64
        }
    } else {
        0u64
    }
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method5(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method7(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("snapshot"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        1u64
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("module"); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v0 == v3 ;
        if v4 {
            2u64
        } else {
            0u64
        }
    }
}
fn method6(mut v0: Rc<str>) -> u64 {
    method7(v0.clone())
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method6(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("snapshot"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2"); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0 == v4 ;
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("module"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
    }
}
fn method8(mut v0: Rc<str>) -> Rc<str> {
    method9(v0.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        method8(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
    loop {
        let mut v5: bool = v3 == v4 ;
        if v5 {
            return true;
        } else {
            let mut v6: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v7: u8 = v1.clone().as_bytes()[v3 as usize];
            let mut v8: bool = v6 == v7 ;
            if v8 {
                let mut v9: i32 = v2 + 1i32 ;
                let mut v10: i32 = v3 + 1i32 ;
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v9, v10, v4);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn method11(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runtime"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        1u64
    } else {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v3 < 6i32;
        let mut v10: bool = if v4 {
            false
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("build:"); } LIT.with(|lit| lit.clone()) };
            let mut v6: i32 = 0i32;
            let mut v7: i32 = 0i32;
            let mut v8: i32 = 6i32;
            method12(v0.clone(), v5.clone(), v6, v7, v8)
        };
        if v10 {
            2u64
        } else {
            0u64
        }
    }
}
fn method10(mut v0: Rc<str>) -> u64 {
    method11(v0.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method10(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runtime"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("build:"); } LIT.with(|lit| lit.clone()) }, v0.clone()));
        v4.clone()
    }
}
fn method13(mut v0: Rc<str>) -> Rc<str> {
    method14(v0.clone())
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        method13(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LeaseActive"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        1u64
    } else {
        0u64
    }
}
fn closure10() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method15(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PromptLease ("); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    }
}
fn closure11() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        method16(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 7i32;
    if v2 {
        let mut v3: bool = v1 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn closure12() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method17(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v1 == 1i32;
    if v2 {
        2i32
    } else {
        let mut v3: bool = v0 == 1i32;
        if v3 {
            1i32
        } else {
            0i32
        }
    }
}
fn closure13() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method18(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method19(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.trim());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("renewing"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1 == v2 ;
    if v3 {
        1u64
    } else {
        0u64
    }
}
fn closure14() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method19(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method20(mut v0: i32, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == 1i32;
    if v2 {
        0i32
    } else {
        v1
    }
}
fn closure15() -> Rc<dyn Fn(i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32) -> i32> = Rc::new(move |mut v0: i32, mut v1: i32| -> i32 {
        method20(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0 == v4 ;
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2"); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v0 == v7 ;
            if v8 {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            } else {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("3"); } LIT.with(|lit| lit.clone()) };
                let mut v11: bool = v0 == v10 ;
                if v11 {
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) };
                    v12.clone()
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("4"); } LIT.with(|lit| lit.clone()) };
                    let mut v14: bool = v0 == v13 ;
                    if v14 {
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
                        v15.clone()
                    } else {
                        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v16.clone()
                    }
                }
            }
        }
    }
}
fn closure16() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        method21(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v1, &v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v4, &v5));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v6.replace(&*v7, &v8));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(v9.replace(&*v10, &v11));
    v12.clone()
}
fn closure17() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        method22(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method23(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.trim());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1 == v2 ;
    if v3 {
        1u64
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0.starts_with(&*v4);
        let mut v8: bool = if v5 {
            true
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v0.starts_with(&*v6);
            v7
        };
        if v8 {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v10: bool = v1 == v9 ;
            let mut v13: bool = if v10 {
                true
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                let mut v12: bool = v1.starts_with(&*v11);
                v12
            };
            if v13 {
                0u64
            } else {
                2u64
            }
        } else {
            3u64
        }
    }
}
fn closure18() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        method23(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.trim());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v1.trim_end_matches(&*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from(v3.trim_end_matches(&*v4));
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(v5.trim());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: bool = v6 == v7 ;
    let mut v11: bool = if v8 {
        true
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v10: bool = v6.contains(&*v9);
        v10
    };
    let mut v14: bool = if v11 {
        true
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v13: bool = v6.contains(&*v12);
        v13
    };
    if v14 {
        v7.clone()
    } else {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v15, &*v7, &*v7, &*v7].concat());
        v16.clone()
    }
}
fn closure19() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        method24(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_state_mutation_target_decision_binding(v0: i32, v1: i32) -> i32 {
    closure0()(v0, v1)
}
pub fn eoie_agile_lease_budget_decision_binding(v0: i32, v1: i32) -> i32 {
    closure1()(v0, v1)
}
pub fn eoie_agile_lease_readback_decision_binding(v0: i32, v1: i32) -> i32 {
    closure2()(v0, v1)
}
pub fn eoie_agile_prompt_title_code(v0: &str) -> u64 {
    closure3()(Rc::<str>::from(v0))
}
pub fn eoie_agile_status_name_code(v0: &str) -> u64 {
    closure4()(Rc::<str>::from(v0))
}
pub fn eoie_agile_task_line_candidate_code(v0: &str) -> u64 {
    closure5()(Rc::<str>::from(v0))
}
pub fn eoie_agile_receipt_row_kind_code(v0: &str) -> u64 {
    closure6()(Rc::<str>::from(v0))
}
pub fn eoie_agile_receipt_row_kind_text(v0: &str) -> Rc<str> {
    closure7()(Rc::<str>::from(v0))
}
pub fn eoie_agile_receipt_attestation_kind_code(v0: &str) -> u64 {
    closure8()(Rc::<str>::from(v0))
}
pub fn eoie_agile_receipt_attestation_text(v0: &str) -> Rc<str> {
    closure9()(Rc::<str>::from(v0))
}
pub fn eoie_agile_prompt_lease_status_code(v0: &str) -> u64 {
    closure10()(Rc::<str>::from(v0))
}
pub fn eoie_agile_prompt_lease_marker_text(v0: &str) -> Rc<str> {
    closure11()(Rc::<str>::from(v0))
}
pub fn eoie_agile_prompt_lease_shape_binding(v0: i32, v1: i32) -> i32 {
    closure12()(v0, v1)
}
pub fn eoie_agile_lease_clock_phase_binding(v0: i32, v1: i32) -> i32 {
    closure13()(v0, v1)
}
pub fn eoie_agile_lease_policy_from_text(v0: &str) -> u64 {
    closure14()(Rc::<str>::from(v0))
}
pub fn eoie_agile_lease_effective_phase_binding(v0: i32, v1: i32) -> i32 {
    closure15()(v0, v1)
}
pub fn eoie_agile_status_text(v0: &str) -> Rc<str> {
    closure16()(Rc::<str>::from(v0))
}
pub fn eoie_agile_spi_escape_text(v0: &str) -> Rc<str> {
    closure17()(Rc::<str>::from(v0))
}
pub fn eoie_agile_state_package_line_code(v0: &str) -> u64 {
    closure18()(Rc::<str>::from(v0))
}
pub fn eoie_agile_state_package_module_name(v0: &str) -> Rc<str> {
    closure19()(Rc::<str>::from(v0))
}
