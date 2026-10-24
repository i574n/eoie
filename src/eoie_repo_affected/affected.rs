#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH0>, bool, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<UH0>, bool),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
struct Mut0 { l0: Rc<str> }
fn command_line_argument_count_1() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_2(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn arguments_from_3(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v0 >= v1;
        if v3 {
            let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method4(v2.clone(), v4.clone());
        } else {
            let mut v6: i32 = v0.wrapping_add(1i32);
            let mut v7: bool = v0 >= 0i32;
            let mut v10: bool = if v7 {
                let mut v8: i32 = command_line_argument_count_1();
                let mut v9: bool = v0 < v8;
                v9
            } else {
                false
            };
            let mut v14: US0 = if v10 {
                let mut v11: Rc<str> = command_line_argument_2(v0);
                US0::US0_Some(v11.clone())
            } else {
                US0::US0_None
            };
            let mut v18: Rc<str> = match &v14 {
                US0::US0_None => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                }
                US0::US0_Some(v15) => {
                    let mut v15: Rc<str> = v15.clone();
                    v15.clone()
                }
            };
            let mut v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18.clone(), v2.clone()));
            (v0, v1, v2) = (v6, v1, v19.clone());
            continue;
        }
    }
}
fn directory_exists_6(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn directory_names_5(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: bool = directory_exists_6(v0.clone());
    if v1 {
        let mut v3: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v5: Vec<std::string::String> = v3.unwrap();
        let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v8: bool = true; let _vec_fold_ = v5.into_iter().fold(v6, |acc, x| { //;
        let mut v10: Rc<UH0> = acc;
        let mut v12: std::string::String = x;
        let mut v14: Rc<str> = Rc::<str>::from(String::as_str(&v12));
        let mut v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14.clone(), v10.clone()));
        let mut v17: bool = true; v16 });
        let mut v19: Rc<UH0> = _vec_fold_;
        let mut v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method4(v19.clone(), v20.clone())
    } else {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    }
}
fn file_exists_9(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method10(mut v0: Vec<u8>) -> Vec<u8> {
    v0.clone()
}
fn method13(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method14(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn loop_15(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(1i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn section_uses_12(mut v0: Rc<UH0>, mut v1: bool, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: i32 = (v5.clone().len() as i32);
                let mut v8: i32 = 0i32;
                let mut v9: i32 = method13(v5.clone(), v7, v8);
                let mut v10: i32 = v7.wrapping_sub(1i32);
                let mut v11: Rc<str> = string_slice(&v5.clone(), v9 as i64, v10 as i64);
                let mut v12: i32 = (v11.clone().len() as i32);
                let mut v13: i32 = method14(v11.clone(), v12);
                let mut v14: Rc<str> = string_slice(&v11.clone(), 0i32 as i64, v13 as i64);
                let mut v15: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packages:"); } LIT.with(|lit| lit.clone()) };
                if v15 {
                    let mut v16: bool = true;
                    (v0, v1, v2) = (v6.clone(), v16, v2.clone());
                    continue;
                } else {
                    let mut v18: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:"); } LIT.with(|lit| lit.clone()) };
                    if v18 {
                        let mut v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        return method4(v2.clone(), v19.clone());
                    } else {
                        let mut v22: bool = if v1 {
                            let mut v21: bool = v14.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v21
                        } else {
                            false
                        };
                        if v22 {
                            let mut v23: bool = true;
                            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                            let mut v25: bool = v14.starts_with(&*v24);
                            let mut v29: Rc<str> = if v25 {
                                let mut v26: i32 = (v14.clone().len() as i32);
                                let mut v27: i32 = v26.wrapping_sub(1i32);
                                let mut v28: Rc<str> = Rc::<str>::from(&(v14)[((1i32) as usize)..(((1i32) + (v27)) as usize)]);
                                v28.clone()
                            } else {
                                v14.clone()
                            };
                            let mut v30: Rc<str> = loop_15(v29.clone());
                            let mut v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30.clone(), v2.clone()));
                            (v0, v1, v2) = (v6.clone(), v23, v31.clone());
                            continue;
                        } else {
                            (v0, v1, v2) = (v6.clone(), v1, v2.clone());
                            continue;
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method4(v2.clone(), v3.clone());
            }
        }
    }
}
fn lines_list_16(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2.clone();
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let mut v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v2.clone()));
            (v0, v1, v2) = (v0.clone(), v4, v6.clone());
            continue;
        }
    }
}
fn project_uses_11(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: i32 = v3.wrapping_sub(1i32);
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = lines_list_16(v2.clone(), v4, v5.clone());
    let mut v7: bool = false;
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    section_uses_12(v6.clone(), v7, v8.clone())
}
fn node_at_8(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v2));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/package.spiproj"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
    let mut v7: bool = file_exists_9(v6.clone());
    if v7 {
        let mut v9: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
        let mut v11: Vec<u8> = v9.unwrap();
        let mut v12: Vec<u8> = method10(v11.clone());
        let mut v14: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v12);
        let mut v16: std::string::String = v14.unwrap();
        let mut v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
        let mut v19: Rc<UH0> = project_uses_11(v18.clone());
        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v2));
        let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v1));
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/Cargo.toml"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
        let mut v24: bool = file_exists_9(v23.clone());
        US1::US1_Some(v1.clone(), v19.clone(), v24)
    } else {
        US1::US1_None
    }
}
fn method17(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = *v4;
                let mut v5: Rc<UH1> = v5.clone();
                let mut v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v3.clone(), v4, v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn nodes_of_7(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: US1 = node_at_8(v0.clone(), v5.clone());
                match &v7 {
                    US1::US1_None => {
                        (v0, v1, v2) = (v0.clone(), v6.clone(), v2.clone());
                        continue;
                    }
                    US1::US1_Some(v8, v9, v10) => {
                        let mut v8: Rc<str> = v8.clone();
                        let mut v9: Rc<UH0> = v9.clone();
                        let mut v10: bool = *v10;
                        let mut v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v8.clone(), v9.clone(), v10, v2.clone()));
                        (v0, v1, v2) = (v0.clone(), v6.clone(), v11.clone());
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return method17(v2.clone(), v3.clone());
            }
        }
    }
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: bool = v4.clone() == v0.clone();
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method20(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method20(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure1(v0.clone(), v1)
}
fn node_named_21(mut v0: Rc<str>, mut v1: Rc<UH1>) -> US1 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = *v5;
                let mut v6: Rc<UH1> = v6.clone();
                let mut v7: bool = v3.clone() == v0.clone();
                if v7 {
                    return US1::US1_Some(v3.clone(), v4.clone(), v5);
                } else {
                    (v0, v1) = (v0.clone(), v6.clone());
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn prepended_22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = prepended_22(v3.clone(), v1.clone());
            Rc::new(UH0::UH0_Cons(v2.clone(), v4.clone()))
        }
        UH0::UH0_Nil => {
            v1.clone()
        }
    }
}
fn reaches_19(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> bool {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: i32 = 0i32;
                let mut v7: Rc<dyn Fn(Rc<UH0>) -> US2> = method20(v4.clone(), v6);
                let mut v8: US2 = v7(v1.clone());
                let mut v9: bool = match &v8 {
                    US2::US2_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v10: bool = v9 != true;
                if v10 {
                    return true;
                } else {
                    let mut v11: i32 = 0i32;
                    let mut v12: Rc<dyn Fn(Rc<UH0>) -> US2> = method20(v4.clone(), v11);
                    let mut v13: US2 = v12(v2.clone());
                    let mut v14: bool = match &v13 {
                        US2::US2_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let mut v15: bool = v14 != true;
                    if v15 {
                        (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2.clone(), v5.clone());
                        continue;
                    } else {
                        let mut v17: US1 = node_named_21(v4.clone(), v0.clone());
                        match &v17 {
                            US1::US1_None => {
                                let mut v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v2.clone()));
                                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v24.clone(), v5.clone());
                                continue;
                            }
                            US1::US1_Some(v18, v19, v20) => {
                                let mut v18: Rc<str> = v18.clone();
                                let mut v19: Rc<UH0> = v19.clone();
                                let mut v20: bool = *v20;
                                let mut v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v2.clone()));
                                let mut v22: Rc<UH0> = prepended_22(v19.clone(), v5.clone());
                                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v21.clone(), v22.clone());
                                continue;
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn affected_crates_18(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v6, v7, v8, v9) => {
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH0> = v7.clone();
                let mut v8: bool = *v8;
                let mut v9: Rc<UH1> = v9.clone();
                let mut v14: bool = if v8 {
                    let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v11.clone()));
                    reaches_19(v1.clone(), v2.clone(), v10.clone(), v12.clone())
                } else {
                    false
                };
                let mut v16: Rc<UH0> = if v14 {
                    Rc::new(UH0::UH0_Cons(v6.clone(), v3.clone()))
                } else {
                    v3.clone()
                };
                (v0, v1, v2, v3) = (v9.clone(), v1.clone(), v2.clone(), v16.clone());
                continue;
            }
            UH1::UH1_Nil => {
                let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method4(v3.clone(), v4.clone());
            }
        }
    }
}
fn lines_printed_23(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                println!("{}", v2);
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method25(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_24(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method25(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method26(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = *v4;
                let mut v5: Rc<UH1> = v5.clone();
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5.clone(), v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn affected_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_1();
    let mut v1: i32 = command_line_argument_count_1();
    let mut v2: bool = 2i32 < v1;
    let mut v7: US0 = if v2 {
        let mut v3: i32 = 2i32;
        let mut v4: Rc<str> = command_line_argument_2(v3);
        US0::US0_Some(v4.clone())
    } else {
        US0::US0_None
    };
    let mut v11: Rc<str> = match &v7 {
        US0::US0_None => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US0::US0_Some(v8) => {
            let mut v8: Rc<str> = v8.clone();
            v8.clone()
        }
    };
    let mut v12: i32 = 3i32;
    let mut v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v14: Rc<UH0> = arguments_from_3(v12, v0, v13.clone());
    match &*v14 {
        UH0::UH0_Nil => {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: repo usage: repo affected <src> <package>... (crates whose package graph reaches a changed package)"); } LIT.with(|lit| lit.clone()) };
            println!("{}", v15);
            2i32
        }
        _ => {
            let mut v16: Rc<UH0> = directory_names_5(v11.clone());
            let mut v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let mut v18: Rc<UH1> = nodes_of_7(v11.clone(), v16.clone(), v17.clone());
            let mut v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v20: Rc<UH0> = affected_crates_18(v18.clone(), v18.clone(), v14.clone(), v19.clone());
            let mut v21: i32 = 0i32;
            let mut v22: i32 = lines_printed_23(v20.clone(), v21);
            let mut v23: Rc<str> = format_real_24(v22);
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie repo affected ok crates="); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v23));
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" packages="); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            let mut v28: i32 = 0i32;
            let mut v29: i32 = method26(v18.clone(), v28);
            let mut v30: Rc<str> = format_real_24(v29);
            let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v30));
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" src="); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
            let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v11));
            println!("{}", v34);
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        affected_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_repo_affected_run() -> i32 {
    closure0()()
}
