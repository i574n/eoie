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
fn digits_in_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v2 <= v1;
        if v3 {
            return true;
        } else {
            let mut v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v5: bool = 48i32 <= v4;
            let mut v7: bool = if v5 {
                let mut v6: bool = v4 <= 57i32;
                v6
            } else {
                false
            };
            if v7 {
                let mut v8: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v8, v2);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn dated_clock_2(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = 16i32 <= v1;
    if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: i32 = 4i32;
        let mut v5: bool = digits_in_3(v0.clone(), v3, v4);
        if v5 {
            let mut v6: i32 = v0.as_bytes().get(4i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v7: bool = v6 == 45i32;
            if v7 {
                let mut v8: i32 = 5i32;
                let mut v9: i32 = 7i32;
                let mut v10: bool = digits_in_3(v0.clone(), v8, v9);
                if v10 {
                    let mut v11: i32 = v0.as_bytes().get(7i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let mut v12: bool = v11 == 45i32;
                    if v12 {
                        let mut v13: i32 = 8i32;
                        let mut v14: i32 = 10i32;
                        let mut v15: bool = digits_in_3(v0.clone(), v13, v14);
                        if v15 {
                            let mut v16: i32 = v0.as_bytes().get(10i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                            let mut v17: bool = v16 == 32i32;
                            if v17 {
                                let mut v18: i32 = 11i32;
                                let mut v19: i32 = 13i32;
                                let mut v20: bool = digits_in_3(v0.clone(), v18, v19);
                                if v20 {
                                    let mut v21: i32 = v0.as_bytes().get(13i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                                    let mut v22: bool = v21 == 58i32;
                                    if v22 {
                                        let mut v23: i32 = 14i32;
                                        let mut v24: i32 = 16i32;
                                        let mut v25: bool = digits_in_3(v0.clone(), v23, v24);
                                        if v25 {
                                            let mut v26: bool = v1 == 16i32;
                                            if v26 {
                                                true
                                            } else {
                                                let mut v27: i32 = v0.as_bytes().get(16i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                                                let mut v28: bool = v27 == 32i32;
                                                v28
                                            }
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
fn dated_day_4(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = 10i32 <= v1;
    if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: i32 = 4i32;
        let mut v5: bool = digits_in_3(v0.clone(), v3, v4);
        if v5 {
            let mut v6: i32 = v0.as_bytes().get(4i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v7: bool = v6 == 45i32;
            if v7 {
                let mut v8: i32 = 5i32;
                let mut v9: i32 = 7i32;
                let mut v10: bool = digits_in_3(v0.clone(), v8, v9);
                if v10 {
                    let mut v11: i32 = v0.as_bytes().get(7i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                    let mut v12: bool = v11 == 45i32;
                    if v12 {
                        let mut v13: i32 = 8i32;
                        let mut v14: i32 = 10i32;
                        let mut v15: bool = digits_in_3(v0.clone(), v13, v14);
                        if v15 {
                            let mut v16: bool = v1 == 10i32;
                            if v16 {
                                true
                            } else {
                                let mut v17: i32 = v0.as_bytes().get(10i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                                let mut v18: bool = v17 == 32i32;
                                v18
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
fn bare_clock_5(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = 5i32 <= v1;
    if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: i32 = 2i32;
        let mut v5: bool = digits_in_3(v0.clone(), v3, v4);
        if v5 {
            let mut v6: i32 = v0.as_bytes().get(2i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v7: bool = v6 == 58i32;
            if v7 {
                let mut v8: i32 = 3i32;
                let mut v9: i32 = 5i32;
                let mut v10: bool = digits_in_3(v0.clone(), v8, v9);
                if v10 {
                    let mut v11: bool = v1 == 5i32;
                    if v11 {
                        true
                    } else {
                        let mut v12: i32 = v0.as_bytes().get(5i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                        let mut v13: bool = v12 == 32i32;
                        v13
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
fn stamped_separator_from_1(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
        let mut v3: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v3 + 2;
            let mut v6: i32 = v5 + 17i32;
            let mut v7: i32 = (v0.clone().len() as i32);
            let mut v8: bool = v7 <= v6;
            let mut v9: i32 = if v8 {
                v7
            } else {
                v6
            };
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v9 as usize).unwrap_or(""));
            let mut v11: bool = dated_clock_2(v10.clone());
            let mut v15: bool = if v11 {
                true
            } else {
                let mut v12: bool = dated_day_4(v10.clone());
                if v12 {
                    true
                } else {
                    bare_clock_5(v10.clone())
                }
            };
            if v15 {
                return v3;
            } else {
                (v0, v1) = (v0.clone(), v5);
                continue;
            }
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method8(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn stamped_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v4: bool = dated_clock_2(v0.clone());
    let (mut v55, mut v56, mut v57): (Rc<str>, Rc<str>, Rc<str>) = if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..16i32 as usize).unwrap_or(""));
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..10i32 as usize).unwrap_or(""));
        let mut v7: i32 = (v0.clone().len() as i32);
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(16i32 as usize..v7 as usize).unwrap_or(""));
        let mut v9: i32 = (v8.clone().len() as i32);
        let mut v10: i32 = 0i32;
        let mut v11: i32 = method7(v8.clone(), v9, v10);
        let mut v12: i32 = v9.wrapping_sub(1i32);
        let mut v13: Rc<str> = string_slice(&v8.clone(), v11 as i64, v12 as i64);
        let mut v14: i32 = (v13.clone().len() as i32);
        let mut v15: i32 = method8(v13.clone(), v14);
        let mut v16: Rc<str> = string_slice(&v13.clone(), 0i32 as i64, v15 as i64);
        (v5.clone(), v6.clone(), v16.clone())
    } else {
        let mut v17: bool = dated_day_4(v0.clone());
        let (mut v52, mut v53, mut v54): (Rc<str>, Rc<str>, Rc<str>) = if v17 {
            let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..10i32 as usize).unwrap_or(""));
            let mut v19: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..10i32 as usize).unwrap_or(""));
            let mut v20: i32 = (v0.clone().len() as i32);
            let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(10i32 as usize..v20 as usize).unwrap_or(""));
            let mut v22: i32 = (v21.clone().len() as i32);
            let mut v23: i32 = 0i32;
            let mut v24: i32 = method7(v21.clone(), v22, v23);
            let mut v25: i32 = v22.wrapping_sub(1i32);
            let mut v26: Rc<str> = string_slice(&v21.clone(), v24 as i64, v25 as i64);
            let mut v27: i32 = (v26.clone().len() as i32);
            let mut v28: i32 = method8(v26.clone(), v27);
            let mut v29: Rc<str> = string_slice(&v26.clone(), 0i32 as i64, v28 as i64);
            (v18.clone(), v19.clone(), v29.clone())
        } else {
            let mut v30: bool = bare_clock_5(v0.clone());
            let (mut v49, mut v50, mut v51): (Rc<str>, Rc<str>, Rc<str>) = if v30 {
                let mut v31: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..5i32 as usize).unwrap_or(""));
                let mut v32: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v36: Rc<str> = if v32 {
                    v31.clone()
                } else {
                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v33));
                    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v31));
                    v35.clone()
                };
                let mut v37: i32 = (v0.clone().len() as i32);
                let mut v38: Rc<str> = std::rc::Rc::<str>::from(v0.get(5i32 as usize..v37 as usize).unwrap_or(""));
                let mut v39: i32 = (v38.clone().len() as i32);
                let mut v40: i32 = 0i32;
                let mut v41: i32 = method7(v38.clone(), v39, v40);
                let mut v42: i32 = v39.wrapping_sub(1i32);
                let mut v43: Rc<str> = string_slice(&v38.clone(), v41 as i64, v42 as i64);
                let mut v44: i32 = (v43.clone().len() as i32);
                let mut v45: i32 = method8(v43.clone(), v44);
                let mut v46: Rc<str> = string_slice(&v43.clone(), 0i32 as i64, v45 as i64);
                (v36.clone(), v1.clone(), v46.clone())
            } else {
                let mut v47: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v48: Rc<str> = if v47 {
                    v3.clone()
                } else {
                    v2.clone()
                };
                (v48.clone(), v1.clone(), v0.clone())
            };
            (v49.clone(), v50.clone(), v51.clone())
        };
        (v52.clone(), v53.clone(), v54.clone())
    };
    (v55.clone(), v56.clone(), v57.clone())
}
fn note_row_10(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v7, &v8));
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v10, &*v8, &*v11, &*v12].concat());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v16, &v17));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v18.replace(&*v15, &v19));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v20.replace(&*v21, &v22));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v23.replace(&*v24, &v25));
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v26.replace(&*v27, &v28));
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v29, &*v15, &*v12, &*v12].concat());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v13, &*v14, &*v30, &*v31].concat());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v16, &v17));
    let mut v36: Rc<str> = std::rc::Rc::<str>::from(v35.replace(&*v15, &v19));
    let mut v37: Rc<str> = std::rc::Rc::<str>::from(v36.replace(&*v21, &v22));
    let mut v38: Rc<str> = std::rc::Rc::<str>::from(v37.replace(&*v24, &v25));
    let mut v39: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v27, &v28));
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v39, &*v15, &*v12, &*v12].concat());
    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v16, &v17));
    let mut v42: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v15, &v19));
    let mut v43: Rc<str> = std::rc::Rc::<str>::from(v42.replace(&*v21, &v22));
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v43.replace(&*v24, &v25));
    let mut v45: Rc<str> = std::rc::Rc::<str>::from(v44.replace(&*v27, &v28));
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v45, &*v15, &*v12, &*v12].concat());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v40, &*v31, &*v46].concat());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v16, &v17));
    let mut v49: Rc<str> = std::rc::Rc::<str>::from(v48.replace(&*v15, &v19));
    let mut v50: Rc<str> = std::rc::Rc::<str>::from(v49.replace(&*v21, &v22));
    let mut v51: Rc<str> = std::rc::Rc::<str>::from(v50.replace(&*v24, &v25));
    let mut v52: Rc<str> = std::rc::Rc::<str>::from(v51.replace(&*v27, &v28));
    let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v52, &*v15, &*v12, &*v12].concat());
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v47, &*v31, &*v53, &*v54].concat());
    v55.clone()
}
fn moved_from_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32) -> (Rc<str>, i32) {
    let mut v10: bool = v2 <= v1;
    let (mut v47, mut v48): (Rc<str>, i32) = if v10 {
        (v8.clone(), v9)
    } else {
        let mut v11: i32 = stamped_separator_from_1(v0.clone(), v1);
        let mut v12: bool = v11 < 0i32;
        let mut v13: i32 = if v12 {
            v2
        } else {
            v11
        };
        let mut v15: i32 = if v12 {
            v2
        } else {
            let mut v14: i32 = v11 + 2;
            v14
        };
        let mut v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v13 as usize).unwrap_or(""));
        let mut v17: i32 = (v16.clone().len() as i32);
        let mut v18: i32 = 0i32;
        let mut v19: i32 = method7(v16.clone(), v17, v18);
        let mut v20: i32 = v17.wrapping_sub(1i32);
        let mut v21: Rc<str> = string_slice(&v16.clone(), v19 as i64, v20 as i64);
        let mut v22: i32 = (v21.clone().len() as i32);
        let mut v23: i32 = method8(v21.clone(), v22);
        let mut v24: Rc<str> = string_slice(&v21.clone(), 0i32 as i64, v23 as i64);
        let mut v25: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let (mut v45, mut v46): (Rc<str>, i32) = if v25 {
            let (mut v26, mut v27): (Rc<str>, i32) = moved_from_6(v0.clone(), v15, v2, v3.clone(), v4, v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9);
            (v26.clone(), v27)
        } else {
            let (mut v28, mut v29, mut v30): (Rc<str>, Rc<str>, Rc<str>) = stamped_9(v24.clone(), v5.clone(), v6.clone(), v7.clone());
            let mut v31: bool = v30.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = if v31 {
                v24.clone()
            } else {
                v30.clone()
            };
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrated"); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = note_row_10(v3.clone(), v4, v28.clone(), v33.clone(), v32.clone());
            let mut v35: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v38: Rc<str> = if v35 {
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v36.clone()
            } else {
                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                v37.clone()
            };
            let mut v39: i32 = v4.wrapping_add(1i32);
            let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v38));
            let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v34));
            let mut v42: i32 = v9.wrapping_add(1i32);
            let (mut v43, mut v44): (Rc<str>, i32) = moved_from_6(v0.clone(), v15, v2, v3.clone(), v39, v29.clone(), v28.clone(), v7.clone(), v41.clone(), v42);
            (v43.clone(), v44)
        };
        (v45.clone(), v46)
    };
    (v47.clone(), v48)
}
fn tail_rows_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let mut v4: i32 = 0i32;
    let mut v5: i32 = stamped_separator_from_1(v0.clone(), v4);
    let mut v6: bool = v5 < 0i32;
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (0i32, v7.clone())
    } else {
        let mut v8: i32 = v5 + 2;
        let mut v9: i32 = (v0.clone().len() as i32);
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: i32 = 0i32;
        let (mut v12, mut v13): (Rc<str>, i32) = moved_from_6(v0.clone(), v8, v9, v1.clone(), v2, v10.clone(), v10.clone(), v3.clone(), v10.clone(), v11);
        (v13, v12.clone())
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>| -> (i32, Rc<str>) {
        tail_rows_0(v0.clone(), v1.clone(), v2, v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        let mut v1: i32 = 0i32;
        stamped_separator_from_1(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_title_tail_rows(v0: &str, v1: &str, v2: i32, v3: &str) -> (i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, Rc::<str>::from(v3))
}
pub fn eoie_agile_title_kept_end(v0: &str) -> i32 {
    closure1()(Rc::<str>::from(v0))
}
