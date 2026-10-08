#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_0,
    US0_1(Rc<str>),
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_0 => 0,
            US0::US0_1(..) => 1,
        }
    }
}
#[derive(Clone)]
enum US1 {
    US1_0,
    US1_1,
}
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_0 => 0,
            US1::US1_1 => 1,
        }
    }
}
#[derive(Clone)]
enum US2 {
    US2_0,
    US2_1,
    US2_2,
}
impl US2 {
    fn tag(&self) -> i32 {
        match self {
            US2::US2_0 => 0,
            US2::US2_1 => 1,
            US2::US2_2 => 2,
        }
    }
}
#[derive(Clone)]
enum US3 {
    US3_0(Rc<str>),
    US3_1(Rc<str>),
}
impl US3 {
    fn tag(&self) -> i32 {
        match self {
            US3::US3_0(..) => 0,
            US3::US3_1(..) => 1,
        }
    }
}
#[derive(Clone)]
enum US4 {
    US4_0(US0),
    US4_1,
}
impl US4 {
    fn tag(&self) -> i32 {
        match self {
            US4::US4_0(..) => 0,
            US4::US4_1 => 1,
        }
    }
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US2) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US2::US2_0 => {
                    let mut v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let mut v10: i32 = v1 + 1;
                let mut v25: US2 = match &v3 {
                    US2::US2_2 => {
                        US2::US2_1
                    }
                    US2::US2_0 => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US2::US2_1
                        } else {
                            US2::US2_0
                        }
                    }
                    US2::US2_1 => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US2::US2_2
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US2::US2_0
                            } else {
                                US2::US2_1
                            }
                        }
                    }
                };
                (v0, v1, v2, v3) = (v0.clone(), v10, v2, v25.clone());
                continue;
            }
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1 + 1;
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US2 = US2::US2_0;
            let mut v6: i32 = method6(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6 + 1;
            let mut v8: i32 = method7(v0.clone(), v7);
            let mut v9: i32 = v3 - 1;
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn method4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v3: i32 = 0i32;
    let mut v4: i32 = method5(v0.clone(), v1, v2, v3);
    let mut v5: US2 = US2::US2_0;
    let mut v6: i32 = method6(v0.clone(), v4, v2, v5.clone());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let mut v8: i32 = v7.len() as i32;
    let mut v9: i32 = v8 - 1;
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let mut v11: i32 = 1i32;
    let mut v12: i32 = method5(v0.clone(), v1, v2, v11);
    let mut v13: US2 = US2::US2_0;
    let mut v14: i32 = method6(v0.clone(), v12, v2, v13.clone());
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: i32 = v15.len() as i32;
    let mut v17: i32 = v16 - 1;
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let mut v19: i32 = 2i32;
    let mut v20: i32 = method5(v0.clone(), v1, v2, v19);
    let mut v21: US2 = US2::US2_0;
    let mut v22: i32 = method6(v0.clone(), v20, v2, v21.clone());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let mut v24: i32 = 3i32;
    let mut v25: i32 = method5(v0.clone(), v1, v2, v24);
    let mut v26: US2 = US2::US2_0;
    let mut v27: i32 = method6(v0.clone(), v25, v2, v26.clone());
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let mut v29: i32 = 4i32;
    let mut v30: i32 = method5(v0.clone(), v1, v2, v29);
    let mut v31: US2 = US2::US2_0;
    let mut v32: i32 = method6(v0.clone(), v30, v2, v31.clone());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v30 as usize..v32 as usize).unwrap_or(""));
    let mut v34: i32 = 5i32;
    let mut v35: i32 = method5(v0.clone(), v1, v2, v34);
    let mut v36: US2 = US2::US2_0;
    let mut v37: i32 = method6(v0.clone(), v35, v2, v36.clone());
    let mut v38: Rc<str> = std::rc::Rc::<str>::from(v0.get(v35 as usize..v37 as usize).unwrap_or(""));
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v39, &v40));
    let mut v42: i32 = 6i32;
    let mut v43: i32 = method5(v0.clone(), v1, v2, v42);
    let mut v44: US2 = US2::US2_0;
    let mut v45: i32 = method6(v0.clone(), v43, v2, v44.clone());
    let mut v46: Rc<str> = std::rc::Rc::<str>::from(v0.get(v43 as usize..v45 as usize).unwrap_or(""));
    (v10.clone(), v18.clone(), v23.clone(), v28.clone(), v33.clone(), v41.clone(), v46.clone())
}
fn method8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool) -> Rc<str> {
    let mut v10: Rc<str> = if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v8.clone()
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v0, &*v13, &*v14, &*v14].concat());
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v12, &*v16, &*v15, &*v14].concat());
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v1, &*v13, &*v14, &*v14].concat());
    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v19, &*v16, &*v20, &*v14].concat());
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v17, &*v18, &*v21].concat());
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v2, &*v13, &*v14, &*v14].concat());
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v23, &*v16, &*v24, &*v14].concat());
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v3, &*v13, &*v14, &*v14].concat());
    let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v26, &*v16, &*v27, &*v14].concat());
    let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v25, &*v18, &*v28, &*v18].concat());
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v4, &*v13, &*v14, &*v14].concat());
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v30, &*v16, &*v31, &*v14].concat());
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v33, &*v16, &*v5, &*v14].concat());
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v6, &*v13, &*v14, &*v14].concat());
    let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v35, &*v16, &*v36, &*v14].concat());
    let mut v38: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v18, &*v34, &*v18, &*v37].concat());
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v29, &*v38, &*v39, &*v14].concat());
    v40.clone()
}
fn method9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> Rc<str> {
    let mut v7: Rc<str> = { let text = v0; let filler = (26i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v8, &*v3, &*v8, &*v4].concat());
    let mut v10: Rc<str> = { let text = v5; let filler = (4i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &*text) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { let text = v6; let filler = (8i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v13: Rc<str> = { let text = v2; let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v10, &*v11, &*v12, &*v13].concat());
    let mut v15: Rc<str> = { let text = v1; if text.chars().count() <= 96i32 as usize { text } else { std::rc::Rc::<str>::from(text.chars().take((96i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v14, &*v15, &*v16, &*v16].concat());
    v17.clone()
}
fn method3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: US0, mut v5: US1, mut v6: i32, mut v7: i32) -> i32 {
    loop {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let mut v9: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v8)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v10: bool = v9 < 0;
        if v10 {
            return v7;
        } else {
            let mut v11: i32 = v9 + 6i32;
            let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = method4(v0.clone(), v11, v2);
            let mut v19: i32 = v0.get(..v9 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v9 as usize).unwrap_or(""));
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v22: bool = v20.starts_with(&*v21);
            if v22 {
                let mut v23: bool = v15 == v3 ;
                if v23 {
                    let mut v27: bool = match &v4 {
                        US0::US0_0 => {
                            true
                        }
                        US0::US0_1(v24) => {
                            let mut v24: Rc<str> = v24.clone();
                            let mut v25: bool = v24 == v18 ;
                            v25
                        }
                    };
                    if v27 {
                        let mut v28: i32 = v6 + v7;
                        let mut v29: bool = v28 == 0i32;
                        let mut v33: Rc<str> = match &v5 {
                            US1::US1_1 => {
                                method8(v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v29)
                            }
                            US1::US1_0 => {
                                method9(v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone())
                            }
                        };
                        let mut v34: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v33.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        let mut v35: i32 = v7 + 1;
                        (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v11, v2, v3.clone(), v4.clone(), v5.clone(), v6, v35);
                        continue;
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v11, v2, v3.clone(), v4.clone(), v5.clone(), v6, v7);
                        continue;
                    }
                } else {
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v11, v2, v3.clone(), v4.clone(), v5.clone(), v6, v7);
                    continue;
                }
            } else {
                (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v11, v2, v3.clone(), v4.clone(), v5.clone(), v6, v7);
                continue;
            }
        }
    }
}
fn method2(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: US1, mut v4: i32, mut v5: i32) -> i32 {
    loop {
        let mut v6: bool = v4 <= 5i32;
        if v6 {
            let mut v7: i32 = 0i32;
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v10, &*v10, &*v10].concat());
            let mut v12: i32 = 0i32;
            let mut v13: i32 = method3(v0.clone(), v7, v1, v11.clone(), v2.clone(), v3.clone(), v5, v12);
            let mut v14: i32 = v4 + 1;
            let mut v15: i32 = v5 + v13;
            (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1, v2.clone(), v3.clone(), v14, v15);
            continue;
        } else {
            return v5;
        }
    }
}
fn method1(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: u64) -> i32 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"tasks\":["); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v4.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v6: US1 = US1::US1_1;
    let mut v7: i32 = 0i32;
    let mut v8: i32 = 0i32;
    let mut v9: i32 = method2(v0.clone(), v1, v2.clone(), v6.clone(), v7, v8);
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"shown\":"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"filter\":"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = match &v2 {
        US0::US0_0 => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US0::US0_1(v14) => {
            let mut v14: Rc<str> = v14.clone();
            v14.clone()
        }
    };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v16, &*v17, &*v18, &*v18].concat());
    let mut v20: Rc<str> = agile_task_census::eoie_agile_status_counts_json(&v0);
    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v19, &*v20].concat());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
    let mut v23: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let mut v24: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v22, &*v24, &*v25, &*v18].concat());
    let mut v27: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v26.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v27
}
fn method10(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: u64) -> i32 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ID"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { let text = v4; let filler = (26i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" P  D     %  "); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("STATUS"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { let text = v7; let filler = (8i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIND"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { let text = v9; let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TITLE"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v6, &*v8, &*v10, &*v11].concat());
    let mut v13: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v12.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let mut v14: US1 = US1::US1_0;
    let mut v15: i32 = 0i32;
    let mut v16: i32 = 0i32;
    let mut v17: i32 = method2(v0.clone(), v1, v2.clone(), v14.clone(), v15, v16);
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile list ok shown="); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" filter="); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = match &v2 {
        US0::US0_0 => {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v21.clone()
        }
        US0::US0_1(v22) => {
            let mut v22: Rc<str> = v22.clone();
            v22.clone()
        }
    };
    let mut v25: Rc<str> = agile_task_census::eoie_agile_status_counts(&v0);
    let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v24, &*v25].concat());
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v28: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let mut v29: Rc<str> = std::rc::Rc::<str>::from((v28).to_string());
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v27, &*v29, &*v30, &*v30].concat());
    let mut v32: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v31.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v32
}
fn method12(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: US2 = US2::US2_0;
        let mut v5: i32 = method6(v0.clone(), v1, v2, v4.clone());
        let mut v6: bool = v2 <= v5;
        if v6 {
            return -1i32;
        } else {
            let mut v7: i32 = v0.as_bytes().get(v5 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v8: bool = v7 == 41i32;
            if v8 {
                let mut v9: i32 = v3 + 1;
                return v9;
            } else {
                let mut v10: i32 = v5 + 1;
                let mut v11: i32 = method7(v0.clone(), v10);
                let mut v12: i32 = v3 + 1;
                (v0, v1, v2, v3) = (v0.clone(), v11, v2, v12);
                continue;
            }
        }
    }
}
fn method11(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0;
        if v5 {
            return -1i32;
        } else {
            let mut v6: i32 = v4 + 6i32;
            let mut v7: i32 = v0.get(..v4 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v7 as usize..v4 as usize).unwrap_or(""));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v10: bool = v8.starts_with(&*v9);
            if v10 {
                let mut v11: i32 = 0i32;
                let mut v12: i32 = method12(v0.clone(), v6, v2, v11);
                let mut v13: bool = v12 == 11i32;
                if v13 {
                    (v0, v1, v2) = (v0.clone(), v6, v2);
                    continue;
                } else {
                    return v4;
                }
            } else {
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            }
        }
    }
}
fn method0() -> i32 {
    let mut v0: i32 = std::env::args().skip(2).count() as i32;
    let mut v1: bool = v0 == 2i32;
    if v1 {
        let mut v2: i32 = 1i32;
        let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: US0 = US0::US0_0;
        let mut v5: US1 = US1::US1_0;
        let mut v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v7, &*v8, &*v8, &*v8].concat());
        let mut v10: bool = std::path::Path::new(&*v9).is_file();
        let mut v13: US3 = if v10 {
            US3::US3_0(v9.clone())
        } else {
            US3::US3_1(v9.clone())
        };
        match &v13 {
            US3::US3_1(v14) => {
                let mut v14: Rc<str> = v14.clone();
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v15, &*v8, &*v8, &*v8].concat());
                let mut v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v18
            }
            US3::US3_0(v19) => {
                let mut v19: Rc<str> = v19.clone();
                let mut v20: Rc<str> = std::fs::read_to_string(&*v19).unwrap_or_default().into();
                let mut v21: i32 = v20.len() as i32;
                let mut v22: i32 = 0i32;
                let mut v23: i32 = method11(v20.clone(), v22, v21);
                let mut v24: bool = v23 < 0;
                if v24 {
                    match &v5 {
                        US1::US1_1 => {
                            method1(v20.clone(), v21, v4.clone(), v6)
                        }
                        US1::US1_0 => {
                            method10(v20.clone(), v21, v4.clone(), v6)
                        }
                    }
                } else {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                    let mut v30: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                    let mut v32: Rc<str> = std::rc::Rc::<str>::from((11i32).to_string());
                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fields)"); } LIT.with(|lit| lit.clone()) };
                    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v30, &*v31, &*v32, &*v33].concat());
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v34, &*v8, &*v8, &*v8].concat());
                    let mut v37: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v36 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v37
                }
            }
        }
    } else {
        let mut v41: bool = v0 == 3i32;
        if v41 {
            let mut v42: i32 = 2i32;
            let mut v43: Rc<str> = usize::try_from(v42).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
            let mut v45: bool = v43 == v44 ;
            if v45 {
                let mut v46: i32 = 1i32;
                let mut v47: Rc<str> = usize::try_from(v46).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v48: US0 = US0::US0_0;
                let mut v49: US1 = US1::US1_1;
                let mut v50: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v51, &*v52, &*v52, &*v52].concat());
                let mut v54: bool = std::path::Path::new(&*v53).is_file();
                let mut v57: US3 = if v54 {
                    US3::US3_0(v53.clone())
                } else {
                    US3::US3_1(v53.clone())
                };
                match &v57 {
                    US3::US3_1(v58) => {
                        let mut v58: Rc<str> = v58.clone();
                        let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                        let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v61: Rc<str> = std::rc::Rc::<str>::from([&*v60, &*v59, &*v52, &*v52, &*v52].concat());
                        let mut v62: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v61 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v62
                    }
                    US3::US3_0(v63) => {
                        let mut v63: Rc<str> = v63.clone();
                        let mut v64: Rc<str> = std::fs::read_to_string(&*v63).unwrap_or_default().into();
                        let mut v65: i32 = v64.len() as i32;
                        let mut v66: i32 = 0i32;
                        let mut v67: i32 = method11(v64.clone(), v66, v65);
                        let mut v68: bool = v67 < 0;
                        if v68 {
                            match &v49 {
                                US1::US1_1 => {
                                    method1(v64.clone(), v65, v48.clone(), v50)
                                }
                                US1::US1_0 => {
                                    method10(v64.clone(), v65, v48.clone(), v50)
                                }
                            }
                        } else {
                            let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                            let mut v74: Rc<str> = std::rc::Rc::<str>::from((v67).to_string());
                            let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                            let mut v76: Rc<str> = std::rc::Rc::<str>::from((11i32).to_string());
                            let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fields)"); } LIT.with(|lit| lit.clone()) };
                            let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v73, &*v74, &*v75, &*v76, &*v77].concat());
                            let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                            let mut v80: Rc<str> = std::rc::Rc::<str>::from([&*v79, &*v78, &*v52, &*v52, &*v52].concat());
                            let mut v81: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v80 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v81
                        }
                    }
                }
            } else {
                let mut v85: US1 = US1::US1_0;
                let mut v86: i32 = 2i32;
                let mut v87: Rc<str> = usize::try_from(v86).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                let mut v89: bool = v87 == v88 ;
                let mut v98: US4 = if v89 {
                    let mut v90: US0 = US0::US0_0;
                    US4::US4_0(v90.clone())
                } else {
                    let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned|Active|Blocked|Paused|Done"); } LIT.with(|lit| lit.clone()) };
                    let mut v93: bool = v92.split("|").any(|item| item == &*v87);
                    if v93 {
                        let mut v94: US0 = US0::US0_1(v87.clone());
                        US4::US4_0(v94.clone())
                    } else {
                        US4::US4_1
                    }
                };
                match &v98 {
                    US4::US4_1 => {
                        let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid agile status filter: "); } LIT.with(|lit| lit.clone()) };
                        let mut v138: i32 = 2i32;
                        let mut v139: Rc<str> = usize::try_from(v138).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected all, Planned, Active, Blocked, Paused or Done)"); } LIT.with(|lit| lit.clone()) };
                        let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v142: Rc<str> = std::rc::Rc::<str>::from([&*v137, &*v139, &*v140, &*v141, &*v141].concat());
                        let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v144: Rc<str> = std::rc::Rc::<str>::from([&*v143, &*v142, &*v141, &*v141, &*v141].concat());
                        let mut v145: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v144 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v145
                    }
                    US4::US4_0(v99) => {
                        let mut v99: US0 = v99.clone();
                        let mut v100: i32 = 1i32;
                        let mut v101: Rc<str> = usize::try_from(v100).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v102: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                        let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                        let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v105: Rc<str> = std::rc::Rc::<str>::from([&*v101, &*v103, &*v104, &*v104, &*v104].concat());
                        let mut v106: bool = std::path::Path::new(&*v105).is_file();
                        let mut v109: US3 = if v106 {
                            US3::US3_0(v105.clone())
                        } else {
                            US3::US3_1(v105.clone())
                        };
                        match &v109 {
                            US3::US3_1(v110) => {
                                let mut v110: Rc<str> = v110.clone();
                                let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                                let mut v113: Rc<str> = std::rc::Rc::<str>::from([&*v112, &*v111, &*v104, &*v104, &*v104].concat());
                                let mut v114: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v113 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v114
                            }
                            US3::US3_0(v115) => {
                                let mut v115: Rc<str> = v115.clone();
                                let mut v116: Rc<str> = std::fs::read_to_string(&*v115).unwrap_or_default().into();
                                let mut v117: i32 = v116.len() as i32;
                                let mut v118: i32 = 0i32;
                                let mut v119: i32 = method11(v116.clone(), v118, v117);
                                let mut v120: bool = v119 < 0;
                                if v120 {
                                    match &v85 {
                                        US1::US1_1 => {
                                            method1(v116.clone(), v117, v99.clone(), v102)
                                        }
                                        US1::US1_0 => {
                                            method10(v116.clone(), v117, v99.clone(), v102)
                                        }
                                    }
                                } else {
                                    let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                                    let mut v126: Rc<str> = std::rc::Rc::<str>::from((v119).to_string());
                                    let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                                    let mut v128: Rc<str> = std::rc::Rc::<str>::from((11i32).to_string());
                                    let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fields)"); } LIT.with(|lit| lit.clone()) };
                                    let mut v130: Rc<str> = std::rc::Rc::<str>::from([&*v125, &*v126, &*v127, &*v128, &*v129].concat());
                                    let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                                    let mut v132: Rc<str> = std::rc::Rc::<str>::from([&*v131, &*v130, &*v104, &*v104, &*v104].concat());
                                    let mut v133: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v132 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v133
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let mut v149: i32 = 3i32;
            let mut v150: Rc<str> = usize::try_from(v149).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
            let mut v152: bool = v150 == v151 ;
            if v152 {
                let mut v153: US1 = US1::US1_1;
                let mut v154: i32 = 2i32;
                let mut v155: Rc<str> = usize::try_from(v154).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                let mut v157: bool = v155 == v156 ;
                let mut v166: US4 = if v157 {
                    let mut v158: US0 = US0::US0_0;
                    US4::US4_0(v158.clone())
                } else {
                    let mut v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned|Active|Blocked|Paused|Done"); } LIT.with(|lit| lit.clone()) };
                    let mut v161: bool = v160.split("|").any(|item| item == &*v155);
                    if v161 {
                        let mut v162: US0 = US0::US0_1(v155.clone());
                        US4::US4_0(v162.clone())
                    } else {
                        US4::US4_1
                    }
                };
                match &v166 {
                    US4::US4_1 => {
                        let mut v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid agile status filter: "); } LIT.with(|lit| lit.clone()) };
                        let mut v206: i32 = 2i32;
                        let mut v207: Rc<str> = usize::try_from(v206).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected all, Planned, Active, Blocked, Paused or Done)"); } LIT.with(|lit| lit.clone()) };
                        let mut v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v210: Rc<str> = std::rc::Rc::<str>::from([&*v205, &*v207, &*v208, &*v209, &*v209].concat());
                        let mut v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v212: Rc<str> = std::rc::Rc::<str>::from([&*v211, &*v210, &*v209, &*v209, &*v209].concat());
                        let mut v213: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v212 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v213
                    }
                    US4::US4_0(v167) => {
                        let mut v167: US0 = v167.clone();
                        let mut v168: i32 = 1i32;
                        let mut v169: Rc<str> = usize::try_from(v168).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v170: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                        let mut v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                        let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v173: Rc<str> = std::rc::Rc::<str>::from([&*v169, &*v171, &*v172, &*v172, &*v172].concat());
                        let mut v174: bool = std::path::Path::new(&*v173).is_file();
                        let mut v177: US3 = if v174 {
                            US3::US3_0(v173.clone())
                        } else {
                            US3::US3_1(v173.clone())
                        };
                        match &v177 {
                            US3::US3_1(v178) => {
                                let mut v178: Rc<str> = v178.clone();
                                let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                let mut v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                                let mut v181: Rc<str> = std::rc::Rc::<str>::from([&*v180, &*v179, &*v172, &*v172, &*v172].concat());
                                let mut v182: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v181 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v182
                            }
                            US3::US3_0(v183) => {
                                let mut v183: Rc<str> = v183.clone();
                                let mut v184: Rc<str> = std::fs::read_to_string(&*v183).unwrap_or_default().into();
                                let mut v185: i32 = v184.len() as i32;
                                let mut v186: i32 = 0i32;
                                let mut v187: i32 = method11(v184.clone(), v186, v185);
                                let mut v188: bool = v187 < 0;
                                if v188 {
                                    match &v153 {
                                        US1::US1_1 => {
                                            method1(v184.clone(), v185, v167.clone(), v170)
                                        }
                                        US1::US1_0 => {
                                            method10(v184.clone(), v185, v167.clone(), v170)
                                        }
                                    }
                                } else {
                                    let mut v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                                    let mut v194: Rc<str> = std::rc::Rc::<str>::from((v187).to_string());
                                    let mut v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                                    let mut v196: Rc<str> = std::rc::Rc::<str>::from((11i32).to_string());
                                    let mut v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fields)"); } LIT.with(|lit| lit.clone()) };
                                    let mut v198: Rc<str> = std::rc::Rc::<str>::from([&*v193, &*v194, &*v195, &*v196, &*v197].concat());
                                    let mut v199: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                                    let mut v200: Rc<str> = std::rc::Rc::<str>::from([&*v199, &*v198, &*v172, &*v172, &*v172].concat());
                                    let mut v201: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v200 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v201
                                }
                            }
                        }
                    }
                }
            } else {
                let mut v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list expects <root> [all|Planned|Active|Blocked|Paused|Done] [--json]"); } LIT.with(|lit| lit.clone()) };
                let mut v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v219: Rc<str> = std::rc::Rc::<str>::from([&*v217, &*v216, &*v218, &*v218, &*v218].concat());
                let mut v220: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v219 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v220
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        method0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_list_run() -> i32 {
    closure0()()
}
