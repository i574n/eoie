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
fn starts_with_loop_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
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
fn find_char_loop_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
    loop {
        let mut v4: bool = v1 == v2 ;
        if v4 {
            return v2;
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == v3 ;
            if v6 {
                return v1;
            } else {
                let mut v7: i32 = v1 + 1i32 ;
                (v0, v1, v2, v3) = (v0.clone(), v7, v2, v3);
                continue;
            }
        }
    }
}
fn block_end_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return -1i32;
        } else {
            let mut v5: i32 = v1 + 2i32 ;
            let mut v6: bool = v3 < v5;
            let mut v11: bool = if v6 {
                false
            } else {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/*"); } LIT.with(|lit| lit.clone()) };
                let mut v8: i32 = 0i32;
                let mut v9: i32 = 2i32;
                starts_with_loop_2(v0.clone(), v7.clone(), v1, v8, v9)
            };
            if v11 {
                let mut v12: bool = v2 == 256i32 ;
                if v12 {
                    return -1i32;
                } else {
                    let mut v13: i32 = v1 + 2i32 ;
                    let mut v14: i32 = v2 + 1i32 ;
                    (v0, v1, v2) = (v0.clone(), v13, v14);
                    continue;
                }
            } else {
                let mut v17: i32 = v1 + 2i32 ;
                let mut v18: bool = v3 < v17;
                let mut v23: bool = if v18 {
                    false
                } else {
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*/"); } LIT.with(|lit| lit.clone()) };
                    let mut v20: i32 = 0i32;
                    let mut v21: i32 = 2i32;
                    starts_with_loop_2(v0.clone(), v19.clone(), v1, v20, v21)
                };
                if v23 {
                    let mut v24: bool = v2 == 1i32 ;
                    if v24 {
                        let mut v25: i32 = v1 + 2i32 ;
                        return v25;
                    } else {
                        let mut v26: i32 = v1 + 2i32 ;
                        let mut v27: i32 = v2 - 1i32 ;
                        (v0, v1, v2) = (v0.clone(), v26, v27);
                        continue;
                    }
                } else {
                    let mut v30: i32 = v1 + 1i32 ;
                    (v0, v1, v2) = (v0.clone(), v30, v2);
                    continue;
                }
            }
        }
    }
}
fn skip_1(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v1 >= v2;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v5: bool = v4 == b' ' ;
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t' ;
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\n' ;
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\r' ;
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v1 + 1i32 ;
                (v0, v1) = (v0.clone(), v12);
                continue;
            } else {
                let mut v14: i32 = v1 + 2i32 ;
                let mut v15: bool = v2 < v14;
                let mut v20: bool = if v15 {
                    false
                } else {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                    let mut v17: i32 = 0i32;
                    let mut v18: i32 = 2i32;
                    starts_with_loop_2(v0.clone(), v16.clone(), v1, v17, v18)
                };
                if v20 {
                    let mut v21: u8 = b'\n';
                    let mut v22: i32 = find_char_loop_3(v0.clone(), v1, v2, v21);
                    (v0, v1) = (v0.clone(), v22);
                    continue;
                } else {
                    let mut v24: i32 = v1 + 2i32 ;
                    let mut v25: bool = v2 < v24;
                    let mut v30: bool = if v25 {
                        false
                    } else {
                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/*"); } LIT.with(|lit| lit.clone()) };
                        let mut v27: i32 = 0i32;
                        let mut v28: i32 = 2i32;
                        starts_with_loop_2(v0.clone(), v26.clone(), v1, v27, v28)
                    };
                    if v30 {
                        let mut v31: i32 = v1 + 2i32 ;
                        let mut v32: i32 = 1i32;
                        let mut v33: i32 = block_end_4(v0.clone(), v31, v32);
                        let mut v34: bool = v33 < 0i32;
                        if v34 {
                            return v33;
                        } else {
                            (v0, v1) = (v0.clone(), v33);
                            continue;
                        }
                    } else {
                        return v1;
                    }
                }
            }
        }
    }
}
fn find_text_loop_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        let mut v5: i32 = v2 + v4 ;
        let mut v6: bool = v3 < v5;
        if v6 {
            return v3;
        } else {
            let mut v7: i32 = (v0.clone().len() as i32);
            let mut v8: i32 = (v1.clone().len() as i32);
            let mut v9: i32 = v2 + v8 ;
            let mut v10: bool = v7 < v9;
            let mut v13: bool = if v10 {
                false
            } else {
                let mut v11: i32 = 0i32;
                starts_with_loop_2(v0.clone(), v1.clone(), v2, v11, v8)
            };
            if v13 {
                return v2;
            } else {
                let mut v14: i32 = v2 + 1i32 ;
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v14, v3, v4);
                continue;
            }
        }
    }
}
fn raw_end_5(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return -1i32;
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == b'#' ;
            if v6 {
                let mut v7: i32 = v1 + 1i32 ;
                let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) }));
                (v0, v1, v2) = (v0.clone(), v7, v8.clone());
                continue;
            } else {
                let mut v10: bool = v5 == b'"' ;
                if v10 {
                    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v2.clone()));
                    let mut v12: i32 = v1 + 1i32 ;
                    let mut v13: i32 = (v11.clone().len() as i32);
                    let mut v14: i32 = find_text_loop_6(v0.clone(), v11.clone(), v12, v3, v13);
                    let mut v15: bool = v14 == v3 ;
                    if v15 {
                        return -1i32;
                    } else {
                        let mut v16: i32 = v14 + v13 ;
                        return v16;
                    }
                } else {
                    return -1i32;
                }
            }
        }
    }
}
fn quoted_end_7(mut v0: Rc<str>, mut v1: i32, mut v2: u8) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return -1i32;
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == b'\\' ;
            if v6 {
                let mut v7: i32 = v1 + 2i32 ;
                (v0, v1, v2) = (v0.clone(), v7, v2);
                continue;
            } else {
                let mut v9: bool = v5 == v2 ;
                if v9 {
                    let mut v10: i32 = v1 + 1i32 ;
                    return v10;
                } else {
                    let mut v11: i32 = v1 + 1i32 ;
                    (v0, v1, v2) = (v0.clone(), v11, v2);
                    continue;
                }
            }
        }
    }
}
fn word_end_8(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v1 < v2;
        let mut v18: bool = if v3 {
            let mut v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v5: bool = b'a' <= v4;
            let mut v7: bool = if v5 {
                let mut v6: bool = v4 <= b'z';
                v6
            } else {
                false
            };
            let mut v13: bool = if v7 {
                true
            } else {
                let mut v8: bool = b'A' <= v4;
                let mut v10: bool = if v8 {
                    let mut v9: bool = v4 <= b'Z';
                    v9
                } else {
                    false
                };
                if v10 {
                    true
                } else {
                    let mut v11: bool = v4 == b'_' ;
                    v11
                }
            };
            if v13 {
                true
            } else {
                let mut v14: bool = b'0' <= v4;
                if v14 {
                    let mut v15: bool = v4 <= b'9';
                    v15
                } else {
                    false
                }
            }
        } else {
            false
        };
        if v18 {
            let mut v19: i32 = v1 + 1i32 ;
            (v0, v1) = (v0.clone(), v19);
            continue;
        } else {
            return v1;
        }
    }
}
fn method10() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_9() -> Rc<str> {
    method10()
}
fn token_0(mut v0: Rc<str>, mut v1: i32) -> (i32, i32, Rc<str>, bool) {
    let mut v2: i32 = skip_1(v0.clone(), v1);
    let mut v3: i32 = (v0.clone().len() as i32);
    let mut v4: bool = v2 < 0i32;
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v2, v2, v5.clone(), false)
    } else {
        let mut v6: bool = v2 == v3 ;
        if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v2, v2, v7.clone(), true)
        } else {
            let mut v8: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v9: i32 = v2 + 2i32 ;
            let mut v10: bool = v3 < v9;
            let mut v15: bool = if v10 {
                false
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("r\""); } LIT.with(|lit| lit.clone()) };
                let mut v12: i32 = 0i32;
                let mut v13: i32 = 2i32;
                starts_with_loop_2(v0.clone(), v11.clone(), v2, v12, v13)
            };
            let mut v23: bool = if v15 {
                true
            } else {
                let mut v16: i32 = v2 + 2i32 ;
                let mut v17: bool = v3 < v16;
                if v17 {
                    false
                } else {
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("r#"); } LIT.with(|lit| lit.clone()) };
                    let mut v19: i32 = 0i32;
                    let mut v20: i32 = 2i32;
                    starts_with_loop_2(v0.clone(), v18.clone(), v2, v19, v20)
                }
            };
            let mut v56: i32 = if v23 {
                1i32
            } else {
                let mut v24: i32 = v2 + 3i32 ;
                let mut v25: bool = v3 < v24;
                let mut v30: bool = if v25 {
                    false
                } else {
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("br\""); } LIT.with(|lit| lit.clone()) };
                    let mut v27: i32 = 0i32;
                    let mut v28: i32 = 3i32;
                    starts_with_loop_2(v0.clone(), v26.clone(), v2, v27, v28)
                };
                let mut v38: bool = if v30 {
                    true
                } else {
                    let mut v31: i32 = v2 + 3i32 ;
                    let mut v32: bool = v3 < v31;
                    if v32 {
                        false
                    } else {
                        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("br#"); } LIT.with(|lit| lit.clone()) };
                        let mut v34: i32 = 0i32;
                        let mut v35: i32 = 3i32;
                        starts_with_loop_2(v0.clone(), v33.clone(), v2, v34, v35)
                    }
                };
                let mut v46: bool = if v38 {
                    true
                } else {
                    let mut v39: i32 = v2 + 3i32 ;
                    let mut v40: bool = v3 < v39;
                    if v40 {
                        false
                    } else {
                        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cr\""); } LIT.with(|lit| lit.clone()) };
                        let mut v42: i32 = 0i32;
                        let mut v43: i32 = 3i32;
                        starts_with_loop_2(v0.clone(), v41.clone(), v2, v42, v43)
                    }
                };
                let mut v54: bool = if v46 {
                    true
                } else {
                    let mut v47: i32 = v2 + 3i32 ;
                    let mut v48: bool = v3 < v47;
                    if v48 {
                        false
                    } else {
                        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cr#"); } LIT.with(|lit| lit.clone()) };
                        let mut v50: i32 = 0i32;
                        let mut v51: i32 = 3i32;
                        starts_with_loop_2(v0.clone(), v49.clone(), v2, v50, v51)
                    }
                };
                if v54 {
                    2i32
                } else {
                    0i32
                }
            };
            let mut v57: bool = v56 > 0i32;
            let mut v107: i32 = if v57 {
                let mut v58: i32 = v2 + v56 ;
                let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                raw_end_5(v0.clone(), v58, v59.clone())
            } else {
                let mut v61: bool = v8 == b'"' ;
                if v61 {
                    let mut v62: i32 = v2 + 1i32 ;
                    quoted_end_7(v0.clone(), v62, v8)
                } else {
                    let mut v64: bool = v8 == b'\'' ;
                    if v64 {
                        let mut v65: i32 = v2 + 1i32 ;
                        let mut v66: bool = v65 < v3;
                        let mut v77: bool = if v66 {
                            let mut v67: u8 = v0.clone().as_bytes()[v65 as usize];
                            let mut v68: bool = b'a' <= v67;
                            let mut v70: bool = if v68 {
                                let mut v69: bool = v67 <= b'z';
                                v69
                            } else {
                                false
                            };
                            if v70 {
                                true
                            } else {
                                let mut v71: bool = b'A' <= v67;
                                let mut v73: bool = if v71 {
                                    let mut v72: bool = v67 <= b'Z';
                                    v72
                                } else {
                                    false
                                };
                                if v73 {
                                    true
                                } else {
                                    let mut v74: bool = v67 == b'_' ;
                                    v74
                                }
                            }
                        } else {
                            false
                        };
                        if v77 {
                            let mut v78: i32 = word_end_8(v0.clone(), v65);
                            let mut v79: bool = v78 < v3;
                            let mut v82: bool = if v79 {
                                let mut v80: u8 = v0.clone().as_bytes()[v78 as usize];
                                let mut v81: bool = v80 == b'\'' ;
                                v81
                            } else {
                                false
                            };
                            if v82 {
                                let mut v83: i32 = v78 + 1i32 ;
                                v83
                            } else {
                                v78
                            }
                        } else {
                            quoted_end_7(v0.clone(), v65, v8)
                        }
                    } else {
                        let mut v87: bool = b'a' <= v8;
                        let mut v89: bool = if v87 {
                            let mut v88: bool = v8 <= b'z';
                            v88
                        } else {
                            false
                        };
                        let mut v95: bool = if v89 {
                            true
                        } else {
                            let mut v90: bool = b'A' <= v8;
                            let mut v92: bool = if v90 {
                                let mut v91: bool = v8 <= b'Z';
                                v91
                            } else {
                                false
                            };
                            if v92 {
                                true
                            } else {
                                let mut v93: bool = v8 == b'_' ;
                                v93
                            }
                        };
                        let mut v99: bool = if v95 {
                            true
                        } else {
                            let mut v96: bool = b'0' <= v8;
                            if v96 {
                                let mut v97: bool = v8 <= b'9';
                                v97
                            } else {
                                false
                            }
                        };
                        if v99 {
                            word_end_8(v0.clone(), v2)
                        } else {
                            let mut v101: bool = v8 > b'~';
                            if v101 {
                                -1i32
                            } else {
                                let mut v102: i32 = v2 + 1i32 ;
                                v102
                            }
                        }
                    }
                }
            };
            let mut v108: bool = v107 < 0i32;
            if v108 {
                let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v2, v2, v109.clone(), false)
            } else {
                let mut v110: bool = v2 == v107 ;
                let mut v114: Rc<str> = if v110 {
                    codec_empty_9()
                } else {
                    let mut v112: i32 = v107 - 1i32 ;
                    let mut v113: Rc<str> = string_slice(&v0.clone(), v2 as i64, v112 as i64);
                    v113.clone()
                };
                (v2, v107, v114.clone(), true)
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, i32) -> (i32, i32, Rc<str>, bool)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32) -> (i32, i32, Rc<str>, bool)> = Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> (i32, i32, Rc<str>, bool) {
        token_0(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> i32 {
        skip_1(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> i32 {
        word_end_8(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn canonical_loop_11(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> (Rc<str>, bool) {
    loop {
        let (mut v3, mut v4, mut v5, mut v6): (i32, i32, Rc<str>, bool) = token_0(v0.clone(), v1);
        let mut v7: bool = v6 == false ;
        if v7 {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return (v8.clone(), false);
        } else {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v10: bool = v5 == v9 ;
            if v10 {
                return (v2.clone(), true);
            } else {
                let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v5.clone()));
                let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                (v0, v1, v2) = (v0.clone(), v4, v12.clone());
                continue;
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, bool)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, bool)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, bool) {
        let mut v1: i32 = 0i32;
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        canonical_loop_11(v0.clone(), v1, v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_predicate_token(v0: &str, v1: i32) -> (i32, i32, Rc<str>, bool) {
    closure0()(Rc::<str>::from(v0), v1)
}
pub fn eoie_predicate_skip(v0: &str, v1: i32) -> i32 {
    closure1()(Rc::<str>::from(v0), v1)
}
pub fn eoie_predicate_word_end(v0: &str, v1: i32) -> i32 {
    closure2()(Rc::<str>::from(v0), v1)
}
pub fn eoie_predicate_canonical(v0: &str) -> (Rc<str>, bool) {
    closure3()(Rc::<str>::from(v0))
}
