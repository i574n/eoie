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
fn method3() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_2() -> Rc<str> {
    method3()
}
fn literal_loop_1(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> (Rc<str>, i32, bool) {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return (v5.clone(), v1, false);
        } else {
            let mut v6: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v7: bool = v6 == b'"' ;
            if v7 {
                let mut v8: bool = v1 == v1 ;
                let mut v12: Rc<str> = if v8 {
                    codec_empty_2()
                } else {
                    let mut v10: i32 = v1 - 1i32 ;
                    let mut v11: Rc<str> = string_slice(&v0.clone(), v1 as i64, v10 as i64);
                    v11.clone()
                };
                let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v12.clone()));
                let mut v14: i32 = v1 + 1i32 ;
                return (v13.clone(), v14, true);
            } else {
                let mut v15: bool = v6 == b'\\' ;
                if v15 {
                    let mut v16: i32 = v1 + 1i32 ;
                    let mut v17: bool = v16 >= v3;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        return (v18.clone(), v1, false);
                    } else {
                        let mut v19: u8 = v0.clone().as_bytes()[v16 as usize];
                        let mut v20: bool = v19 == b'n' ;
                        let mut v35: Rc<str> = if v20 {
                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            v21.clone()
                        } else {
                            let mut v22: bool = v19 == b'r' ;
                            if v22 {
                                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                v23.clone()
                            } else {
                                let mut v24: bool = v19 == b't' ;
                                if v24 {
                                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                    v25.clone()
                                } else {
                                    let mut v26: bool = v19 == b'"' ;
                                    if v26 {
                                        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                        v27.clone()
                                    } else {
                                        let mut v28: bool = v19 == b'\\' ;
                                        if v28 {
                                            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                            v29.clone()
                                        } else {
                                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v30.clone()
                                        }
                                    }
                                }
                            }
                        };
                        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v37: bool = v35 == v36 ;
                        if v37 {
                            return (v36.clone(), v1, false);
                        } else {
                            let mut v38: i32 = v1 + 2i32 ;
                            let mut v39: bool = v1 == v1 ;
                            let mut v43: Rc<str> = if v39 {
                                codec_empty_2()
                            } else {
                                let mut v41: i32 = v1 - 1i32 ;
                                let mut v42: Rc<str> = string_slice(&v0.clone(), v1 as i64, v41 as i64);
                                v42.clone()
                            };
                            let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v43.clone()));
                            let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44.clone(), v35.clone()));
                            (v0, v1, v2) = (v0.clone(), v38, v45.clone());
                            continue;
                        }
                    }
                } else {
                    let mut v55: i32 = v1 + 1i32 ;
                    return literal_loop_0(v0.clone(), v55, v1, v2.clone());
                }
            }
        }
    }
}
fn literal_loop_0(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> (Rc<str>, i32, bool) {
    loop {
        let mut v4: i32 = (v0.clone().len() as i32);
        let mut v5: bool = v1 >= v4;
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return (v6.clone(), v1, false);
        } else {
            let mut v7: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v8: bool = v7 == b'"' ;
            if v8 {
                let mut v9: bool = v2 == v1 ;
                let mut v13: Rc<str> = if v9 {
                    codec_empty_2()
                } else {
                    let mut v11: i32 = v1 - 1i32 ;
                    let mut v12: Rc<str> = string_slice(&v0.clone(), v2 as i64, v11 as i64);
                    v12.clone()
                };
                let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v13.clone()));
                let mut v15: i32 = v1 + 1i32 ;
                return (v14.clone(), v15, true);
            } else {
                let mut v16: bool = v7 == b'\\' ;
                if v16 {
                    let mut v17: i32 = v1 + 1i32 ;
                    let mut v18: bool = v17 >= v4;
                    if v18 {
                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        return (v19.clone(), v1, false);
                    } else {
                        let mut v20: u8 = v0.clone().as_bytes()[v17 as usize];
                        let mut v21: bool = v20 == b'n' ;
                        let mut v36: Rc<str> = if v21 {
                            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        } else {
                            let mut v23: bool = v20 == b'r' ;
                            if v23 {
                                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                v24.clone()
                            } else {
                                let mut v25: bool = v20 == b't' ;
                                if v25 {
                                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                    v26.clone()
                                } else {
                                    let mut v27: bool = v20 == b'"' ;
                                    if v27 {
                                        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                        v28.clone()
                                    } else {
                                        let mut v29: bool = v20 == b'\\' ;
                                        if v29 {
                                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                            v30.clone()
                                        } else {
                                            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v31.clone()
                                        }
                                    }
                                }
                            }
                        };
                        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v38: bool = v36 == v37 ;
                        if v38 {
                            return (v37.clone(), v1, false);
                        } else {
                            let mut v39: i32 = v1 + 2i32 ;
                            let mut v40: bool = v2 == v1 ;
                            let mut v44: Rc<str> = if v40 {
                                codec_empty_2()
                            } else {
                                let mut v42: i32 = v1 - 1i32 ;
                                let mut v43: Rc<str> = string_slice(&v0.clone(), v2 as i64, v42 as i64);
                                v43.clone()
                            };
                            let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v44.clone()));
                            let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45.clone(), v36.clone()));
                            return literal_loop_1(v0.clone(), v39, v46.clone());
                        }
                    }
                } else {
                    let mut v56: i32 = v1 + 1i32 ;
                    (v0, v1, v2, v3) = (v0.clone(), v56, v2, v3.clone());
                    continue;
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, i32) -> (Rc<str>, i32, bool)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32) -> (Rc<str>, i32, bool)> = Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> (Rc<str>, i32, bool) {
        let mut v2: bool = 0i32 <= v1;
        let mut v5: bool = if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: bool = v1 < v3;
            v4
        } else {
            false
        };
        let mut v8: bool = if v5 {
            let mut v6: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v7: bool = v6 == b'"' ;
            v7
        } else {
            false
        };
        if v8 {
            let mut v9: i32 = v1 + 1i32 ;
            let mut v10: i32 = v1 + 1i32 ;
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            literal_loop_0(v0.clone(), v9, v10, v11.clone())
        } else {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v15.clone(), v1, false)
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn encode_loop_5(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 == v3 ;
        if v4 {
            let mut v5: bool = v1 == v1 ;
            let mut v9: Rc<str> = if v5 {
                codec_empty_2()
            } else {
                let mut v7: i32 = v1 - 1i32 ;
                let mut v8: Rc<str> = string_slice(&v0.clone(), v1 as i64, v7 as i64);
                v8.clone()
            };
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v9.clone()));
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            return v11.clone();
        } else {
            let mut v12: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v13: bool = v12 == b'"' ;
            let mut v28: Rc<str> = if v13 {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                v14.clone()
            } else {
                let mut v15: bool = v12 == b'\\' ;
                if v15 {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let mut v17: bool = v12 == b'\n' ;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let mut v19: bool = v12 == b'\r' ;
                        if v19 {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let mut v21: bool = v12 == b'\t' ;
                            if v21 {
                                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                v22.clone()
                            } else {
                                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v23.clone()
                            }
                        }
                    }
                }
            };
            let mut v29: i32 = v1 + 1i32 ;
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v31: bool = v28 == v30 ;
            if v31 {
                return encode_loop_4(v0.clone(), v29, v1, v2.clone());
            } else {
                let mut v33: bool = v1 == v1 ;
                let mut v37: Rc<str> = if v33 {
                    codec_empty_2()
                } else {
                    let mut v35: i32 = v1 - 1i32 ;
                    let mut v36: Rc<str> = string_slice(&v0.clone(), v1 as i64, v35 as i64);
                    v36.clone()
                };
                let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v37.clone()));
                let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38.clone(), v28.clone()));
                (v0, v1, v2) = (v0.clone(), v29, v39.clone());
                continue;
            }
        }
    }
}
fn encode_loop_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: i32 = (v0.clone().len() as i32);
        let mut v5: bool = v1 == v4 ;
        if v5 {
            let mut v6: bool = v2 == v1 ;
            let mut v10: Rc<str> = if v6 {
                codec_empty_2()
            } else {
                let mut v8: i32 = v1 - 1i32 ;
                let mut v9: Rc<str> = string_slice(&v0.clone(), v2 as i64, v8 as i64);
                v9.clone()
            };
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v10.clone()));
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            return v12.clone();
        } else {
            let mut v13: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v14: bool = v13 == b'"' ;
            let mut v29: Rc<str> = if v14 {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                v15.clone()
            } else {
                let mut v16: bool = v13 == b'\\' ;
                if v16 {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                    v17.clone()
                } else {
                    let mut v18: bool = v13 == b'\n' ;
                    if v18 {
                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                        v19.clone()
                    } else {
                        let mut v20: bool = v13 == b'\r' ;
                        if v20 {
                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                            v21.clone()
                        } else {
                            let mut v22: bool = v13 == b'\t' ;
                            if v22 {
                                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                v23.clone()
                            } else {
                                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v24.clone()
                            }
                        }
                    }
                }
            };
            let mut v30: i32 = v1 + 1i32 ;
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v32: bool = v29 == v31 ;
            if v32 {
                (v0, v1, v2, v3) = (v0.clone(), v30, v2, v3.clone());
                continue;
            } else {
                let mut v34: bool = v2 == v1 ;
                let mut v38: Rc<str> = if v34 {
                    codec_empty_2()
                } else {
                    let mut v36: i32 = v1 - 1i32 ;
                    let mut v37: Rc<str> = string_slice(&v0.clone(), v2 as i64, v36 as i64);
                    v37.clone()
                };
                let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v38.clone()));
                let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39.clone(), v29.clone()));
                return encode_loop_5(v0.clone(), v30, v40.clone());
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let mut v1: i32 = 0i32;
        let mut v2: i32 = 0i32;
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        encode_loop_4(v0.clone(), v1, v2, v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_predicate_literal(v0: &str, v1: i32) -> (Rc<str>, i32, bool) {
    closure0()(Rc::<str>::from(v0), v1)
}
pub fn eoie_predicate_encode(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
