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
    US0_Rejected(Rc<str>),
    US0_Candidate(Rc<str>),
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_Rejected(..) => 0,
            US0::US0_Candidate(..) => 1,
        }
    }
}
impl Default for US0 {
    fn default() -> Self {
        US0::US0_Rejected({ thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })
    }
}
#[derive(Clone)]
enum US1 {
    US1_Sha256,
    US1_ReceiptText,
    US1_Unsupported,
}
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_Sha256 => 0,
            US1::US1_ReceiptText => 1,
            US1::US1_Unsupported => 2,
        }
    }
}
impl Default for US1 {
    fn default() -> Self {
        US1::US1_Sha256
    }
}
fn method3() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_2() -> Rc<str> {
    method3()
}
fn starts_with_loop_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
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
fn codec_runtime_char_6(mut v0: Rc<str>) -> u8 {
    let mut v1: u8 = v0.clone().as_bytes()[0i32 as usize];
    v1
}
fn argument_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (i32, i32, i32, Rc<str>, bool) {
    let mut v3: i32 = predicate_lex_domain::eoie_predicate_skip(&*v0,v1);
    let mut v4: bool = v3 < 0i32;
    let mut v7: bool = if v4 {
        true
    } else {
        let mut v5: i32 = (v0.clone().len() as i32);
        let mut v6: bool = v3 >= v5;
        v6
    };
    let mut v9: bool = if v7 {
        true
    } else {
        let mut v8: bool = v2 > 16i32;
        v8
    };
    if v9 {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v1, v1, v1, v10.clone(), false)
    } else {
        let mut v11: u8 = v0.clone().as_bytes()[v3 as usize];
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v13: u8 = codec_runtime_char_6(v12.clone());
        let mut v14: bool = v11 == v13 ;
        if v14 {
            let (mut v15, mut v16, mut v17): (Rc<str>, i32, bool) = predicate_literal_domain::eoie_predicate_literal(&*v0,v3);
            (v16, v3, v16, v15.clone(), v17)
        } else {
            let mut v18: i32 = (v0.clone().len() as i32);
            let mut v19: i32 = v3 + 1i32 ;
            let mut v20: bool = v18 < v19;
            let mut v25: bool = if v20 {
                false
            } else {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
                let mut v22: i32 = 0i32;
                let mut v23: i32 = 1i32;
                starts_with_loop_4(v0.clone(), v21.clone(), v3, v22, v23)
            };
            if v25 {
                let mut v26: i32 = v3 + 1i32 ;
                let mut v27: i32 = predicate_lex_domain::eoie_predicate_skip(&*v0,v26);
                let mut v28: bool = v27 < 0i32;
                let mut v45: i32 = if v28 {
                    0i32
                } else {
                    let mut v29: i32 = v27 + 33i32 ;
                    let mut v30: bool = v18 < v29;
                    let mut v35: bool = if v30 {
                        false
                    } else {
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("typed_predicate.model.lift_sha256"); } LIT.with(|lit| lit.clone()) };
                        let mut v32: i32 = 0i32;
                        let mut v33: i32 = 33i32;
                        starts_with_loop_4(v0.clone(), v31.clone(), v27, v32, v33)
                    };
                    if v35 {
                        33i32
                    } else {
                        let mut v36: i32 = v27 + 39i32 ;
                        let mut v37: bool = v18 < v36;
                        let mut v42: bool = if v37 {
                            false
                        } else {
                            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("typed_predicate.model.lift_receipt_text"); } LIT.with(|lit| lit.clone()) };
                            let mut v39: i32 = 0i32;
                            let mut v40: i32 = 39i32;
                            starts_with_loop_4(v0.clone(), v38.clone(), v27, v39, v40)
                        };
                        if v42 {
                            39i32
                        } else {
                            0i32
                        }
                    }
                };
                let mut v46: bool = v45 == 0i32 ;
                if v46 {
                    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (v3, v3, v3, v47.clone(), false)
                } else {
                    let mut v48: i32 = v27 + v45 ;
                    let mut v49: i32 = predicate_lex_domain::eoie_predicate_skip(&*v0,v48);
                    let (mut v50, mut v51, mut v52): (Rc<str>, i32, bool) = predicate_literal_domain::eoie_predicate_literal(&*v0,v49);
                    let mut v53: bool = v52 == false ;
                    if v53 {
                        let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        (v3, v3, v3, v54.clone(), false)
                    } else {
                        let mut v55: i32 = v2 + 1i32 ;
                        let (mut v56, mut v57, mut v58, mut v59, mut v60): (i32, i32, i32, Rc<str>, bool) = argument_5(v0.clone(), v51, v55);
                        let mut v61: i32 = predicate_lex_domain::eoie_predicate_skip(&*v0,v56);
                        let mut v62: bool = v61 < 0i32;
                        let mut v64: bool = if v62 {
                            true
                        } else {
                            let mut v63: bool = v60 == false ;
                            v63
                        };
                        if v64 {
                            let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v3, v3, v3, v65.clone(), false)
                        } else {
                            let mut v66: i32 = v61 + 1i32 ;
                            let mut v67: bool = v18 < v66;
                            let mut v72: bool = if v67 {
                                false
                            } else {
                                let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                                let mut v69: i32 = 0i32;
                                let mut v70: i32 = 1i32;
                                starts_with_loop_4(v0.clone(), v68.clone(), v61, v69, v70)
                            };
                            if v72 {
                                let mut v73: i32 = v61 + 1i32 ;
                                (v73, v57, v58, v59.clone(), true)
                            } else {
                                let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                (v3, v3, v3, v74.clone(), false)
                            }
                        }
                    }
                }
            } else {
                let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v3, v3, v3, v95.clone(), false)
            }
        }
    }
}
fn function_end_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: bool) -> i32 {
    loop {
        let (mut v4, mut v5, mut v6, mut v7): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v1);
        let mut v8: bool = v7 == false ;
        let mut v11: bool = if v8 {
            true
        } else {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v10: bool = v6 == v9 ;
            v10
        };
        if v11 {
            return -1i32;
        } else {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
            let mut v13: bool = v6 == v12 ;
            if v13 {
                let mut v14: i32 = v2 + 1i32 ;
                let mut v15: bool = true;
                (v0, v1, v2, v3) = (v0.clone(), v5, v14, v15);
                continue;
            } else {
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
                let mut v18: bool = v6 == v17 ;
                if v18 {
                    let mut v19: bool = v2 == 1i32 ;
                    if v19 {
                        return v5;
                    } else {
                        let mut v20: bool = v2 <= 0i32;
                        if v20 {
                            return -1i32;
                        } else {
                            let mut v21: i32 = v2 - 1i32 ;
                            (v0, v1, v2, v3) = (v0.clone(), v5, v21, v3);
                            continue;
                        }
                    }
                } else {
                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                    let mut v26: bool = v6 == v25 ;
                    let mut v28: bool = if v26 {
                        let mut v27: bool = v3 == false ;
                        v27
                    } else {
                        false
                    };
                    if v28 {
                        return -1i32;
                    } else {
                        (v0, v1, v2, v3) = (v0.clone(), v5, v2, v3);
                        continue;
                    }
                }
            }
        }
    }
}
fn closure_name_10(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    loop {
        let (mut v2, mut v3, mut v4, mut v5): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v1);
        let mut v6: bool = v5 == false ;
        let mut v9: bool = if v6 {
            true
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v4 == v7 ;
            v8
        };
        if v9 {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v10.clone();
        } else {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
            let mut v12: bool = v4 == v11 ;
            if v12 {
                let (mut v13, mut v14, mut v15, mut v16): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v3);
                return v15.clone();
            } else {
                (v0, v1) = (v0.clone(), v3);
                continue;
            }
        }
    }
}
fn expected_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sha256"); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v0 == v4 ;
    let mut v12: US1 = if v5 {
        US1::US1_Sha256
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("receipt-text"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v0 == v7 ;
        if v8 {
            US1::US1_ReceiptText
        } else {
            US1::US1_Unsupported
        }
    };
    let mut v36: Rc<str> = match &v12 {
        US1::US1_ReceiptText => {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{!"); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".is_empty()&&"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".bytes().all(|"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v2, &*v23, &*v2, &*v24].concat());
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".is_ascii_alphanumeric()||matches!("); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v26, &*v3, &*v27, &*v3].concat());
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",b'.'|b'_'|b'-'|b'/'))}"); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v29, &*v30, &*v30, &*v30].concat());
            let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v31, &*v30, &*v30, &*v30].concat());
            v32.clone()
        }
        US1::US1_Sha256 => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".len()==64&&"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".bytes().all(|"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v2, &*v14, &*v2, &*v15].concat());
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".is_ascii_hexdigit())}"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v17, &*v3, &*v18, &*v19].concat());
            let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v20, &*v19, &*v19, &*v19].concat());
            v21.clone()
        }
        US1::US1_Unsupported => {
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v33.clone()
        }
    };
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v38: bool = v36 == v37 ;
    if v38 {
        v37.clone()
    } else {
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn "); } LIT.with(|lit| lit.clone()) };
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":&str)->bool"); } LIT.with(|lit| lit.clone()) };
        let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v1, &*v40, &*v2, &*v41].concat());
        let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v36, &*v37, &*v37, &*v37].concat());
        v43.clone()
    }
}
fn rust_loop_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: bool, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32, bool) {
    loop {
        let (mut v8, mut v9, mut v10, mut v11): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v3);
        let mut v12: bool = v11 == false ;
        if v12 {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return (v13.clone(), v7, false);
        } else {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: bool = v10 == v14 ;
            if v15 {
                let mut v16: i32 = (v0.clone().len() as i32);
                let mut v17: bool = v3 == v16 ;
                let mut v21: Rc<str> = if v17 {
                    codec_empty_2()
                } else {
                    let mut v19: i32 = v16 - 1i32 ;
                    let mut v20: Rc<str> = string_slice(&v0.clone(), v3 as i64, v19 as i64);
                    v20.clone()
                };
                let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v6.clone(), v21.clone()));
                let mut v23: bool = v4 == 0i32 ;
                return (v22.clone(), v7, v23);
            } else {
                let mut v24: bool = v4 == 0i32 ;
                let mut v27: bool = if v24 {
                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn"); } LIT.with(|lit| lit.clone()) };
                    let mut v26: bool = v10 == v25 ;
                    v26
                } else {
                    false
                };
                if v27 {
                    let (mut v28, mut v29, mut v30, mut v31): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v9);
                    let mut v32: bool = v30 == v1 ;
                    if v32 {
                        let mut v33: i32 = 0i32;
                        let mut v34: bool = false;
                        let mut v35: i32 = function_end_9(v0.clone(), v9, v33, v34);
                        let mut v36: bool = v35 < 0i32;
                        let mut v38: bool = if v36 {
                            true
                        } else {
                            let mut v37: bool = v5 == false ;
                            v37
                        };
                        if v38 {
                            return (v14.clone(), v7, false);
                        } else {
                            let mut v39: bool = v8 == v35 ;
                            let mut v43: Rc<str> = if v39 {
                                codec_empty_2()
                            } else {
                                let mut v41: i32 = v35 - 1i32 ;
                                let mut v42: Rc<str> = string_slice(&v0.clone(), v8 as i64, v41 as i64);
                                v42.clone()
                            };
                            let (mut v44, mut v45, mut v46, mut v47): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v43,0i32);
                            let (mut v48, mut v49, mut v50, mut v51): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v43,v45);
                            let (mut v52, mut v53, mut v54, mut v55): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v43,v49);
                            let (mut v56, mut v57, mut v58, mut v59): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v43,v53);
                            let mut v60: Rc<str> = closure_name_10(v43.clone(), v57);
                            let mut v61: Rc<str> = expected_11(v2.clone(), v1.clone(), v58.clone(), v60.clone());
                            let (mut v62, mut v63): (Rc<str>, bool) = predicate_lex_domain::eoie_predicate_canonical(&*v61);
                            let (mut v64, mut v65): (Rc<str>, bool) = predicate_lex_domain::eoie_predicate_canonical(&*v43);
                            let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
                            let mut v67: bool = v54 == v66 ;
                            let mut v68: bool = v67 && v63;
                            let mut v69: bool = v68 && v65;
                            let mut v71: bool = if v69 {
                                let mut v70: bool = v62 == v64 ;
                                v70
                            } else {
                                false
                            };
                            let mut v74: bool = if v71 {
                                let mut v72: i32 = (v61.clone().len() as i32);
                                let mut v73: bool = 0i32 < v72;
                                v73
                            } else {
                                false
                            };
                            if v74 {
                                let mut v75: i32 = 0i32;
                                let mut v76: bool = true;
                                let mut v77: bool = v3 == v8 ;
                                let mut v81: Rc<str> = if v77 {
                                    codec_empty_2()
                                } else {
                                    let mut v79: i32 = v8 - 1i32 ;
                                    let mut v80: Rc<str> = string_slice(&v0.clone(), v3 as i64, v79 as i64);
                                    v80.clone()
                                };
                                let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v6.clone(), v81.clone()));
                                let mut v83: i32 = v7 + 1i32 ;
                                (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1.clone(), v2.clone(), v35, v75, v76, v82.clone(), v83);
                                continue;
                            } else {
                                return (v14.clone(), v7, false);
                            }
                        }
                    } else {
                        let mut v93: bool = false;
                        return rust_loop_7(v0.clone(), v1.clone(), v2.clone(), v9, v4, v93, v3, v6.clone(), v7);
                    }
                } else {
                    let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
                    let mut v101: bool = v10 == v100 ;
                    let mut v104: bool = if v101 {
                        true
                    } else {
                        let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                        let mut v103: bool = v10 == v102 ;
                        v103
                    };
                    let mut v107: bool = if v104 {
                        true
                    } else {
                        let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
                        let mut v106: bool = v10 == v105 ;
                        v106
                    };
                    let mut v119: i32 = if v107 {
                        let mut v108: i32 = v4 + 1i32 ;
                        v108
                    } else {
                        let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                        let mut v110: bool = v10 == v109 ;
                        let mut v113: bool = if v110 {
                            true
                        } else {
                            let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
                            let mut v112: bool = v10 == v111 ;
                            v112
                        };
                        let mut v116: bool = if v113 {
                            true
                        } else {
                            let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
                            let mut v115: bool = v10 == v114 ;
                            v115
                        };
                        if v116 {
                            let mut v117: i32 = v4 - 1i32 ;
                            v117
                        } else {
                            v4
                        }
                    };
                    let mut v120: bool = v119 == 0i32 ;
                    let mut v126: bool = if v120 {
                        let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                        let mut v122: bool = v10 == v121 ;
                        if v122 {
                            true
                        } else {
                            let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
                            let mut v124: bool = v10 == v123 ;
                            v124
                        }
                    } else {
                        false
                    };
                    let mut v127: bool = v119 < 0i32;
                    if v127 {
                        return (v14.clone(), v7, false);
                    } else {
                        return rust_loop_7(v0.clone(), v1.clone(), v2.clone(), v9, v119, v126, v3, v6.clone(), v7);
                    }
                }
            }
        }
    }
}
fn rust_loop_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: bool, mut v6: i32, mut v7: Rc<str>, mut v8: i32) -> (Rc<str>, i32, bool) {
    loop {
        let (mut v9, mut v10, mut v11, mut v12): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v3);
        let mut v13: bool = v12 == false ;
        if v13 {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return (v14.clone(), v8, false);
        } else {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v16: bool = v11 == v15 ;
            if v16 {
                let mut v17: i32 = (v0.clone().len() as i32);
                let mut v18: bool = v6 == v17 ;
                let mut v22: Rc<str> = if v18 {
                    codec_empty_2()
                } else {
                    let mut v20: i32 = v17 - 1i32 ;
                    let mut v21: Rc<str> = string_slice(&v0.clone(), v6 as i64, v20 as i64);
                    v21.clone()
                };
                let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v7.clone(), v22.clone()));
                let mut v24: bool = v4 == 0i32 ;
                return (v23.clone(), v8, v24);
            } else {
                let mut v25: bool = v4 == 0i32 ;
                let mut v28: bool = if v25 {
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn"); } LIT.with(|lit| lit.clone()) };
                    let mut v27: bool = v11 == v26 ;
                    v27
                } else {
                    false
                };
                if v28 {
                    let (mut v29, mut v30, mut v31, mut v32): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v10);
                    let mut v33: bool = v31 == v1 ;
                    if v33 {
                        let mut v34: i32 = 0i32;
                        let mut v35: bool = false;
                        let mut v36: i32 = function_end_9(v0.clone(), v10, v34, v35);
                        let mut v37: bool = v36 < 0i32;
                        let mut v39: bool = if v37 {
                            true
                        } else {
                            let mut v38: bool = v5 == false ;
                            v38
                        };
                        if v39 {
                            return (v15.clone(), v8, false);
                        } else {
                            let mut v40: bool = v9 == v36 ;
                            let mut v44: Rc<str> = if v40 {
                                codec_empty_2()
                            } else {
                                let mut v42: i32 = v36 - 1i32 ;
                                let mut v43: Rc<str> = string_slice(&v0.clone(), v9 as i64, v42 as i64);
                                v43.clone()
                            };
                            let (mut v45, mut v46, mut v47, mut v48): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v44,0i32);
                            let (mut v49, mut v50, mut v51, mut v52): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v44,v46);
                            let (mut v53, mut v54, mut v55, mut v56): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v44,v50);
                            let (mut v57, mut v58, mut v59, mut v60): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v44,v54);
                            let mut v61: Rc<str> = closure_name_10(v44.clone(), v58);
                            let mut v62: Rc<str> = expected_11(v2.clone(), v1.clone(), v59.clone(), v61.clone());
                            let (mut v63, mut v64): (Rc<str>, bool) = predicate_lex_domain::eoie_predicate_canonical(&*v62);
                            let (mut v65, mut v66): (Rc<str>, bool) = predicate_lex_domain::eoie_predicate_canonical(&*v44);
                            let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
                            let mut v68: bool = v55 == v67 ;
                            let mut v69: bool = v68 && v64;
                            let mut v70: bool = v69 && v66;
                            let mut v72: bool = if v70 {
                                let mut v71: bool = v63 == v65 ;
                                v71
                            } else {
                                false
                            };
                            let mut v75: bool = if v72 {
                                let mut v73: i32 = (v62.clone().len() as i32);
                                let mut v74: bool = 0i32 < v73;
                                v74
                            } else {
                                false
                            };
                            if v75 {
                                let mut v76: i32 = 0i32;
                                let mut v77: bool = true;
                                let mut v78: bool = v6 == v9 ;
                                let mut v82: Rc<str> = if v78 {
                                    codec_empty_2()
                                } else {
                                    let mut v80: i32 = v9 - 1i32 ;
                                    let mut v81: Rc<str> = string_slice(&v0.clone(), v6 as i64, v80 as i64);
                                    v81.clone()
                                };
                                let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v7.clone(), v82.clone()));
                                let mut v84: i32 = v8 + 1i32 ;
                                return rust_loop_8(v0.clone(), v1.clone(), v2.clone(), v36, v76, v77, v83.clone(), v84);
                            } else {
                                return (v15.clone(), v8, false);
                            }
                        }
                    } else {
                        let mut v94: bool = false;
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2.clone(), v10, v4, v94, v6, v7.clone(), v8);
                        continue;
                    }
                } else {
                    let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
                    let mut v102: bool = v11 == v101 ;
                    let mut v105: bool = if v102 {
                        true
                    } else {
                        let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                        let mut v104: bool = v11 == v103 ;
                        v104
                    };
                    let mut v108: bool = if v105 {
                        true
                    } else {
                        let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
                        let mut v107: bool = v11 == v106 ;
                        v107
                    };
                    let mut v120: i32 = if v108 {
                        let mut v109: i32 = v4 + 1i32 ;
                        v109
                    } else {
                        let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                        let mut v111: bool = v11 == v110 ;
                        let mut v114: bool = if v111 {
                            true
                        } else {
                            let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
                            let mut v113: bool = v11 == v112 ;
                            v113
                        };
                        let mut v117: bool = if v114 {
                            true
                        } else {
                            let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
                            let mut v116: bool = v11 == v115 ;
                            v116
                        };
                        if v117 {
                            let mut v118: i32 = v4 - 1i32 ;
                            v118
                        } else {
                            v4
                        }
                    };
                    let mut v121: bool = v120 == 0i32 ;
                    let mut v127: bool = if v121 {
                        let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                        let mut v123: bool = v11 == v122 ;
                        if v123 {
                            true
                        } else {
                            let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
                            let mut v125: bool = v11 == v124 ;
                            v125
                        }
                    } else {
                        false
                    };
                    let mut v128: bool = v120 < 0i32;
                    if v128 {
                        return (v15.clone(), v8, false);
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2.clone(), v10, v120, v127, v6, v7.clone(), v8);
                        continue;
                    }
                }
            }
        }
    }
}
fn wrapper_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sha256"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v0 == v3 ;
    let mut v11: US1 = if v4 {
        US1::US1_Sha256
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("receipt-text"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v0 == v6 ;
        if v7 {
            US1::US1_ReceiptText
        } else {
            US1::US1_Unsupported
        }
    };
    let mut v17: Rc<str> = match &v11 {
        US1::US1_ReceiptText => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lift_receipt_text"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US1::US1_Sha256 => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lift_sha256"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        US1::US1_Unsupported => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unsupported"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
    };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(typed_predicate.model."); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" \""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\" "); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v17, &*v19, &*v1, &*v20].concat());
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v22, &*v23, &*v23, &*v23].concat());
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v24, &*v23, &*v23, &*v23].concat());
    v25.clone()
}
fn source_loop_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: i32) -> US0 {
    loop {
        let (mut v7, mut v8, mut v9, mut v10): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v3);
        let mut v11: bool = v10 == false ;
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unsupported or malformed source syntax; input unchanged"); } LIT.with(|lit| lit.clone()) };
            return US0::US0_Rejected(v12.clone());
        } else {
            let mut v14: bool = v6 > 1i32;
            if v14 {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Ambiguous function: multiple definitions; input unchanged"); } LIT.with(|lit| lit.clone()) };
                return US0::US0_Rejected(v15.clone());
            } else {
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v18: bool = v9 == v17 ;
                if v18 {
                    let mut v19: bool = v6 == 1i32 ;
                    if v19 {
                        let mut v20: i32 = (v0.clone().len() as i32);
                        let mut v21: bool = v4 == v20 ;
                        let mut v25: Rc<str> = if v21 {
                            codec_empty_2()
                        } else {
                            let mut v23: i32 = v20 - 1i32 ;
                            let mut v24: Rc<str> = string_slice(&v0.clone(), v4 as i64, v23 as i64);
                            v24.clone()
                        };
                        let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), v25.clone()));
                        return US0::US0_Candidate(v26.clone());
                    } else {
                        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("No supported top-level predicate definition; input unchanged"); } LIT.with(|lit| lit.clone()) };
                        return US0::US0_Rejected(v28.clone());
                    }
                } else {
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_global"); } LIT.with(|lit| lit.clone()) };
                    let mut v32: bool = v9 == v31 ;
                    if v32 {
                        let mut v33: i32 = predicate_lex_domain::eoie_predicate_skip(&*v0,v8);
                        let mut v34: bool = v33 < 0i32;
                        if v34 {
                            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Malformed comment after RustGlobal; input unchanged"); } LIT.with(|lit| lit.clone()) };
                            return US0::US0_Rejected(v35.clone());
                        } else {
                            let mut v37: i32 = (v0.clone().len() as i32);
                            let mut v38: i32 = v33 + 12i32 ;
                            let mut v39: bool = v37 < v38;
                            let mut v44: bool = if v39 {
                                false
                            } else {
                                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(x : string)"); } LIT.with(|lit| lit.clone()) };
                                let mut v41: i32 = 0i32;
                                let mut v42: i32 = 12i32;
                                starts_with_loop_4(v0.clone(), v40.clone(), v33, v41, v42)
                            };
                            if v44 {
                                (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v8, v4, v5.clone(), v6);
                                continue;
                            } else {
                                let mut v46: i32 = 0i32;
                                let (mut v47, mut v48, mut v49, mut v50, mut v51): (i32, i32, i32, Rc<str>, bool) = argument_5(v0.clone(), v33, v46);
                                let mut v52: bool = v51 == false ;
                                if v52 {
                                    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unsupported RustGlobal argument; input unchanged"); } LIT.with(|lit| lit.clone()) };
                                    return US0::US0_Rejected(v53.clone());
                                } else {
                                    let mut v55: i32 = 0i32;
                                    let mut v56: i32 = 0i32;
                                    let mut v57: bool = true;
                                    let mut v58: i32 = 0i32;
                                    let mut v59: i32 = 0i32;
                                    let (mut v60, mut v61, mut v62): (Rc<str>, i32, bool) = rust_loop_7(v50.clone(), v1.clone(), v2.clone(), v55, v56, v57, v58, v17.clone(), v59);
                                    let mut v63: bool = v62 == false ;
                                    if v63 {
                                        let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Predicate or syntax differs from the supported token grammar; input unchanged"); } LIT.with(|lit| lit.clone()) };
                                        return US0::US0_Rejected(v64.clone());
                                    } else {
                                        let mut v66: bool = v61 == 0i32 ;
                                        if v66 {
                                            (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v47, v4, v5.clone(), v6);
                                            continue;
                                        } else {
                                            let mut v68: Rc<str> = predicate_literal_domain::eoie_predicate_encode(&*v60);
                                            let mut v69: Rc<str> = wrapper_12(v2.clone(), v1.clone(), v68.clone());
                                            let mut v70: bool = v4 == v48 ;
                                            let mut v74: Rc<str> = if v70 {
                                                codec_empty_2()
                                            } else {
                                                let mut v72: i32 = v48 - 1i32 ;
                                                let mut v73: Rc<str> = string_slice(&v0.clone(), v4 as i64, v72 as i64);
                                                v73.clone()
                                            };
                                            let mut v75: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), v74.clone()));
                                            let mut v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75.clone(), v69.clone()));
                                            let mut v77: i32 = v6 + v61 ;
                                            (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v47, v49, v76.clone(), v77);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v8, v4, v5.clone(), v6);
                        continue;
                    }
                }
            }
        }
    }
}
fn rewrite_result_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (bool, Rc<str>) {
    let mut v3: i32 = (v1.clone().len() as i32);
    let mut v4: bool = v3 == 0i32 ;
    let mut v40: US0 = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Empty function name"); } LIT.with(|lit| lit.clone()) };
        US0::US0_Rejected(v5.clone())
    } else {
        let mut v7: i32 = predicate_lex_domain::eoie_predicate_word_end(&*v1,0i32);
        let mut v8: bool = v7 == v3 ;
        let mut v19: bool = if v8 {
            let mut v9: u8 = v1.clone().as_bytes()[0i32 as usize];
            let mut v10: bool = b'a' <= v9;
            let mut v12: bool = if v10 {
                let mut v11: bool = v9 <= b'z';
                v11
            } else {
                false
            };
            if v12 {
                true
            } else {
                let mut v13: bool = b'A' <= v9;
                let mut v15: bool = if v13 {
                    let mut v14: bool = v9 <= b'Z';
                    v14
                } else {
                    false
                };
                if v15 {
                    true
                } else {
                    let mut v16: bool = v9 == b'_' ;
                    v16
                }
            }
        } else {
            false
        };
        if v19 {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sha256"); } LIT.with(|lit| lit.clone()) };
            let mut v21: bool = v2 == v20 ;
            let mut v28: US1 = if v21 {
                US1::US1_Sha256
            } else {
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("receipt-text"); } LIT.with(|lit| lit.clone()) };
                let mut v24: bool = v2 == v23 ;
                if v24 {
                    US1::US1_ReceiptText
                } else {
                    US1::US1_Unsupported
                }
            };
            match &v28 {
                US1::US1_Unsupported => {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unsupported predicate family"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_Rejected(v29.clone())
                }
                _ => {
                    let mut v31: i32 = 0i32;
                    let mut v32: i32 = 0i32;
                    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v34: i32 = 0i32;
                    source_loop_1(v0.clone(), v1.clone(), v2.clone(), v31, v32, v33.clone(), v34)
                }
            }
        } else {
            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Function name must be an ASCII identifier"); } LIT.with(|lit| lit.clone()) };
            US0::US0_Rejected(v37.clone())
        }
    };
    match &v40 {
        US0::US0_Candidate(v41) => {
            let mut v41: Rc<str> = v41.clone();
            (true, v41.clone())
        }
        US0::US0_Rejected(v42) => {
            let mut v42: Rc<str> = v42.clone();
            (false, v42.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (bool, Rc<str>) {
        rewrite_result_0(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_predicate_rewrite(v0: &str, v1: &str, v2: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
