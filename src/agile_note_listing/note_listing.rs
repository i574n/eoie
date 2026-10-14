#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ScanPlain,
    US0_ScanQuoted,
    US0_ScanEscaped,
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_ScanPlain => 0,
            US0::US0_ScanQuoted => 1,
            US0::US0_ScanEscaped => 2,
        }
    }
}
impl Default for US0 {
    fn default() -> Self {
        US0::US0_ScanPlain
    }
}
fn declared_from_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let mut v9: i32 = v3 + 1;
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
}
fn field_end_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US0::US0_ScanPlain => {
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
                let mut v25: US0 = match &v3 {
                    US0::US0_ScanEscaped => {
                        US0::US0_ScanQuoted
                    }
                    US0::US0_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US0::US0_ScanQuoted
                        } else {
                            US0::US0_ScanPlain
                        }
                    }
                    US0::US0_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US0::US0_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US0::US0_ScanPlain
                            } else {
                                US0::US0_ScanQuoted
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
fn skip_spaces_6(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US0 = US0::US0_ScanPlain;
            let mut v6: i32 = field_end_5(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6 + 1;
            let mut v8: i32 = skip_spaces_6(v0.clone(), v7);
            let mut v9: i32 = v3 - 1;
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn unescaped_from_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v2 as usize).unwrap_or(""));
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v5, &*v6, &*v6, &*v6].concat());
            return v7.clone();
        } else {
            let mut v8: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = v8 == 92i32;
            if v9 {
                let mut v10: i32 = v1 + 1;
                let mut v11: i32 = v10 + 1;
                let mut v12: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v1 as usize).unwrap_or(""));
                let mut v13: i32 = v0.as_bytes().get(v10 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v14: bool = v13 == 110i32;
                let mut v24: Rc<str> = if v14 {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v15.clone()
                } else {
                    let mut v16: bool = v13 == 114i32;
                    if v16 {
                        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v17.clone()
                    } else {
                        let mut v18: bool = v13 == 116i32;
                        if v18 {
                            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v19.clone()
                        } else {
                            let mut v20: i32 = v10 + 1;
                            let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v20 as usize).unwrap_or(""));
                            v21.clone()
                        }
                    }
                };
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v12, &*v24, &*v25, &*v25].concat());
                (v0, v1, v2, v3) = (v0.clone(), v11, v2, v26.clone());
                continue;
            } else {
                let mut v28: i32 = v1 + 1;
                return unescaped_from_8(v0.clone(), v28, v1, v2, v3.clone());
            }
        }
    }
}
fn unescaped_from_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v6, &*v7, &*v7, &*v7].concat());
            return v8.clone();
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v10: bool = v9 == 92i32;
            if v10 {
                let mut v11: i32 = v1 + 1;
                let mut v12: i32 = v11 + 1;
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v15: bool = v14 == 110i32;
                let mut v25: Rc<str> = if v15 {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let mut v19: bool = v14 == 116i32;
                        if v19 {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let mut v21: i32 = v11 + 1;
                            let mut v22: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v21 as usize).unwrap_or(""));
                            v22.clone()
                        }
                    }
                };
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v13, &*v25, &*v26, &*v26].concat());
                return unescaped_from_9(v0.clone(), v12, v3, v27.clone());
            } else {
                let mut v29: i32 = v1 + 1;
                (v0, v1, v2, v3, v4) = (v0.clone(), v29, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn unquoted_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = v0.len() as i32;
    let mut v2: i32 = v1 - 1;
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = v3.len() as i32;
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_8(v3.clone(), v4, v5, v6, v7.clone())
}
fn note_at_3(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = v2.len() as i32;
    let mut v4: i32 = v1 + v3;
    let mut v5: i32 = v0.len() as i32;
    let mut v6: i32 = 2i32;
    let mut v7: i32 = field_start_4(v0.clone(), v4, v5, v6);
    let mut v8: US0 = US0::US0_ScanPlain;
    let mut v9: i32 = field_end_5(v0.clone(), v7, v5, v8.clone());
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(v7 as usize..v9 as usize).unwrap_or(""));
    let mut v11: Rc<str> = unquoted_7(v10.clone());
    let mut v12: i32 = v2.len() as i32;
    let mut v13: i32 = v1 + v12;
    let mut v14: i32 = v0.len() as i32;
    let mut v15: i32 = 3i32;
    let mut v16: i32 = field_start_4(v0.clone(), v13, v14, v15);
    let mut v17: US0 = US0::US0_ScanPlain;
    let mut v18: i32 = field_end_5(v0.clone(), v16, v14, v17.clone());
    let mut v19: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v18 as usize).unwrap_or(""));
    let mut v20: Rc<str> = unquoted_7(v19.clone());
    let mut v21: i32 = v2.len() as i32;
    let mut v22: i32 = v1 + v21;
    let mut v23: i32 = v0.len() as i32;
    let mut v24: i32 = 4i32;
    let mut v25: i32 = field_start_4(v0.clone(), v22, v23, v24);
    let mut v26: US0 = US0::US0_ScanPlain;
    let mut v27: i32 = field_end_5(v0.clone(), v25, v23, v26.clone());
    let mut v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let mut v29: Rc<str> = unquoted_7(v28.clone());
    (v11.clone(), v20.clone(), v29.clone())
}
fn note_lines_from_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0;
        if v5 {
            return v3.clone();
        } else {
            let mut v6: i32 = v4 + 1;
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let (mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, Rc<str>) = note_at_3(v0.clone(), v4);
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v11, &*v9, &*v11, &*v10].concat());
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v7, &*v12, &*v13, &*v13].concat());
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v14.clone());
            continue;
        }
    }
}
fn declared_count_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4 + 1;
            let mut v7: i32 = v3 + 1;
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v7);
            continue;
        }
    }
}
fn notes_text_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v7: i32 = 0i32;
    let mut v8: i32 = 0i32;
    let mut v9: i32 = declared_count_10(v0.clone(), v6.clone(), v7, v8);
    let mut v10: bool = v9 == 0i32;
    if v10 {
        v5.clone()
    } else {
        let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v1, &*v4, &*v5].concat());
        let mut v12: i32 = 0i32;
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes ("); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v5, &*v5].concat());
        note_lines_from_1(v0.clone(), v11.clone(), v12, v16.clone())
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        notes_text_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_json_at_14(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"at\":"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = v3.len() as i32;
    let mut v5: i32 = v1 + v4;
    let mut v6: i32 = v0.len() as i32;
    let mut v7: i32 = 2i32;
    let mut v8: i32 = field_start_4(v0.clone(), v5, v6, v7);
    let mut v9: US0 = US0::US0_ScanPlain;
    let mut v10: i32 = field_end_5(v0.clone(), v8, v6, v9.clone());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v10 as usize).unwrap_or(""));
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"author\":"); } LIT.with(|lit| lit.clone()) };
    let mut v13: i32 = v3.len() as i32;
    let mut v14: i32 = v1 + v13;
    let mut v15: i32 = v0.len() as i32;
    let mut v16: i32 = 3i32;
    let mut v17: i32 = field_start_4(v0.clone(), v14, v15, v16);
    let mut v18: US0 = US0::US0_ScanPlain;
    let mut v19: i32 = field_end_5(v0.clone(), v17, v15, v18.clone());
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v11, &*v12, &*v20, &*v21].concat());
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"text\":"); } LIT.with(|lit| lit.clone()) };
    let mut v24: i32 = v3.len() as i32;
    let mut v25: i32 = v1 + v24;
    let mut v26: i32 = v0.len() as i32;
    let mut v27: i32 = 4i32;
    let mut v28: i32 = field_start_4(v0.clone(), v25, v26, v27);
    let mut v29: US0 = US0::US0_ScanPlain;
    let mut v30: i32 = field_end_5(v0.clone(), v28, v26, v29.clone());
    let mut v31: Rc<str> = std::rc::Rc::<str>::from(v0.get(v28 as usize..v30 as usize).unwrap_or(""));
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v23, &*v31, &*v32, &*v33].concat());
    v34.clone()
}
fn note_json_from_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let mut v6: bool = v5 < 0;
        if v6 {
            return v3.clone();
        } else {
            let mut v7: i32 = v5 + 1;
            let mut v8: Rc<str> = note_json_at_14(v0.clone(), v5);
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v8, &*v9, &*v9].concat());
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v7, v10.clone(), v11.clone());
            continue;
        }
    }
}
fn note_json_from_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
    let mut v5: bool = v4 < 0;
    if v5 {
        v3.clone()
    } else {
        let mut v6: i32 = v4 + 1;
        let mut v7: Rc<str> = note_json_at_14(v0.clone(), v4);
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v3, &*v7, &*v8, &*v8].concat());
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        note_json_from_13(v0.clone(), v1.clone(), v6, v9.clone(), v10.clone())
    }
}
fn notes_json_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"notes\":["); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v1, &*v5, &*v6].concat());
    let mut v8: i32 = 0i32;
    let mut v9: Rc<str> = note_json_from_12(v0.clone(), v7.clone(), v8, v6.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v9, &*v10, &*v6, &*v6].concat());
    v11.clone()
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        notes_json_11(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_notes_text(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_agile_notes_json(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
