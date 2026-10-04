#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return Rc::<str>::from(""); }
    // A slice that starts or ends inside a code point fails like the C and Delphi backends (abort / Halt(3)).
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
fn method2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
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
fn method3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
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
fn method4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                let mut v7: Rc<str> = Rc::<str>::from("/*");
                let mut v8: i32 = 0i32;
                let mut v9: i32 = 2i32;
                method2(v0.clone(), v7.clone(), v1, v8, v9)
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
                    let mut v19: Rc<str> = Rc::<str>::from("*/");
                    let mut v20: i32 = 0i32;
                    let mut v21: i32 = 2i32;
                    method2(v0.clone(), v19.clone(), v1, v20, v21)
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
fn method1(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
                    let mut v16: Rc<str> = Rc::<str>::from("//");
                    let mut v17: i32 = 0i32;
                    let mut v18: i32 = 2i32;
                    method2(v0.clone(), v16.clone(), v1, v17, v18)
                };
                if v20 {
                    let mut v21: u8 = b'\n';
                    let mut v22: i32 = method3(v0.clone(), v1, v2, v21);
                    (v0, v1) = (v0.clone(), v22);
                    continue;
                } else {
                    let mut v24: i32 = v1 + 2i32 ;
                    let mut v25: bool = v2 < v24;
                    let mut v30: bool = if v25 {
                        false
                    } else {
                        let mut v26: Rc<str> = Rc::<str>::from("/*");
                        let mut v27: i32 = 0i32;
                        let mut v28: i32 = 2i32;
                        method2(v0.clone(), v26.clone(), v1, v27, v28)
                    };
                    if v30 {
                        let mut v31: i32 = v1 + 2i32 ;
                        let mut v32: i32 = 1i32;
                        let mut v33: i32 = method4(v0.clone(), v31, v32);
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
fn method6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32) -> i32 {
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
                method2(v0.clone(), v1.clone(), v2, v11, v8)
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
fn method5(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> i32 {
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
                let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), Rc::<str>::from("#")));
                (v0, v1, v2) = (v0.clone(), v7, v8.clone());
                continue;
            } else {
                let mut v10: bool = v5 == b'"' ;
                if v10 {
                    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from("\""), v2.clone()));
                    let mut v12: i32 = v1 + 1i32 ;
                    let mut v13: i32 = (v11.clone().len() as i32);
                    let mut v14: i32 = method6(v0.clone(), v11.clone(), v12, v3, v13);
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
fn method7(mut v0: Rc<str>, mut v1: i32, mut v2: u8) -> i32 {
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
fn method8(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
    let mut v0: Rc<str> = Rc::<str>::from("");
    v0.clone()
}
fn method9() -> Rc<str> {
    method10()
}
fn method0(mut v0: Rc<str>, mut v1: i32) -> (i32, i32, Rc<str>, bool) {
    let mut v2: i32 = method1(v0.clone(), v1);
    let mut v3: i32 = (v0.clone().len() as i32);
    let mut v4: bool = v2 < 0i32;
    if v4 {
        let mut v5: Rc<str> = Rc::<str>::from("");
        (v2, v2, v5.clone(), false)
    } else {
        let mut v6: bool = v2 == v3 ;
        if v6 {
            let mut v7: Rc<str> = Rc::<str>::from("");
            (v2, v2, v7.clone(), true)
        } else {
            let mut v8: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v9: i32 = v2 + 2i32 ;
            let mut v10: bool = v3 < v9;
            let mut v15: bool = if v10 {
                false
            } else {
                let mut v11: Rc<str> = Rc::<str>::from("r\"");
                let mut v12: i32 = 0i32;
                let mut v13: i32 = 2i32;
                method2(v0.clone(), v11.clone(), v2, v12, v13)
            };
            let mut v23: bool = if v15 {
                true
            } else {
                let mut v16: i32 = v2 + 2i32 ;
                let mut v17: bool = v3 < v16;
                if v17 {
                    false
                } else {
                    let mut v18: Rc<str> = Rc::<str>::from("r#");
                    let mut v19: i32 = 0i32;
                    let mut v20: i32 = 2i32;
                    method2(v0.clone(), v18.clone(), v2, v19, v20)
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
                    let mut v26: Rc<str> = Rc::<str>::from("br\"");
                    let mut v27: i32 = 0i32;
                    let mut v28: i32 = 3i32;
                    method2(v0.clone(), v26.clone(), v2, v27, v28)
                };
                let mut v38: bool = if v30 {
                    true
                } else {
                    let mut v31: i32 = v2 + 3i32 ;
                    let mut v32: bool = v3 < v31;
                    if v32 {
                        false
                    } else {
                        let mut v33: Rc<str> = Rc::<str>::from("br#");
                        let mut v34: i32 = 0i32;
                        let mut v35: i32 = 3i32;
                        method2(v0.clone(), v33.clone(), v2, v34, v35)
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
                        let mut v41: Rc<str> = Rc::<str>::from("cr\"");
                        let mut v42: i32 = 0i32;
                        let mut v43: i32 = 3i32;
                        method2(v0.clone(), v41.clone(), v2, v42, v43)
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
                        let mut v49: Rc<str> = Rc::<str>::from("cr#");
                        let mut v50: i32 = 0i32;
                        let mut v51: i32 = 3i32;
                        method2(v0.clone(), v49.clone(), v2, v50, v51)
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
                let mut v59: Rc<str> = Rc::<str>::from("");
                method5(v0.clone(), v58, v59.clone())
            } else {
                let mut v61: bool = v8 == b'"' ;
                if v61 {
                    let mut v62: i32 = v2 + 1i32 ;
                    method7(v0.clone(), v62, v8)
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
                            let mut v78: i32 = method8(v0.clone(), v65);
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
                            method7(v0.clone(), v65, v8)
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
                            method8(v0.clone(), v2)
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
                let mut v109: Rc<str> = Rc::<str>::from("");
                (v2, v2, v109.clone(), false)
            } else {
                let mut v110: bool = v2 == v107 ;
                let mut v114: Rc<str> = if v110 {
                    method9()
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
    Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> (i32, i32, Rc<str>, bool) {
        method0(v0.clone(), v1)
    })
}
fn method12(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> (Rc<str>, i32, bool) {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            let mut v5: Rc<str> = Rc::<str>::from("");
            return (v5.clone(), v1, false);
        } else {
            let mut v6: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v7: bool = v6 == b'"' ;
            if v7 {
                let mut v8: bool = v1 == v1 ;
                let mut v12: Rc<str> = if v8 {
                    method9()
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
                        let mut v18: Rc<str> = Rc::<str>::from("");
                        return (v18.clone(), v1, false);
                    } else {
                        let mut v19: u8 = v0.clone().as_bytes()[v16 as usize];
                        let mut v20: bool = v19 == b'n' ;
                        let mut v35: Rc<str> = if v20 {
                            let mut v21: Rc<str> = Rc::<str>::from("\n");
                            v21.clone()
                        } else {
                            let mut v22: bool = v19 == b'r' ;
                            if v22 {
                                let mut v23: Rc<str> = Rc::<str>::from("\r");
                                v23.clone()
                            } else {
                                let mut v24: bool = v19 == b't' ;
                                if v24 {
                                    let mut v25: Rc<str> = Rc::<str>::from("\t");
                                    v25.clone()
                                } else {
                                    let mut v26: bool = v19 == b'"' ;
                                    if v26 {
                                        let mut v27: Rc<str> = Rc::<str>::from("\"");
                                        v27.clone()
                                    } else {
                                        let mut v28: bool = v19 == b'\\' ;
                                        if v28 {
                                            let mut v29: Rc<str> = Rc::<str>::from("\\");
                                            v29.clone()
                                        } else {
                                            let mut v30: Rc<str> = Rc::<str>::from("");
                                            v30.clone()
                                        }
                                    }
                                }
                            }
                        };
                        let mut v36: Rc<str> = Rc::<str>::from("");
                        let mut v37: bool = v35 == v36 ;
                        if v37 {
                            return (v36.clone(), v1, false);
                        } else {
                            let mut v38: i32 = v1 + 2i32 ;
                            let mut v39: bool = v1 == v1 ;
                            let mut v43: Rc<str> = if v39 {
                                method9()
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
                    return method11(v0.clone(), v55, v1, v2.clone());
                }
            }
        }
    }
}
fn method11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> (Rc<str>, i32, bool) {
    loop {
        let mut v4: i32 = (v0.clone().len() as i32);
        let mut v5: bool = v1 >= v4;
        if v5 {
            let mut v6: Rc<str> = Rc::<str>::from("");
            return (v6.clone(), v1, false);
        } else {
            let mut v7: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v8: bool = v7 == b'"' ;
            if v8 {
                let mut v9: bool = v2 == v1 ;
                let mut v13: Rc<str> = if v9 {
                    method9()
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
                        let mut v19: Rc<str> = Rc::<str>::from("");
                        return (v19.clone(), v1, false);
                    } else {
                        let mut v20: u8 = v0.clone().as_bytes()[v17 as usize];
                        let mut v21: bool = v20 == b'n' ;
                        let mut v36: Rc<str> = if v21 {
                            let mut v22: Rc<str> = Rc::<str>::from("\n");
                            v22.clone()
                        } else {
                            let mut v23: bool = v20 == b'r' ;
                            if v23 {
                                let mut v24: Rc<str> = Rc::<str>::from("\r");
                                v24.clone()
                            } else {
                                let mut v25: bool = v20 == b't' ;
                                if v25 {
                                    let mut v26: Rc<str> = Rc::<str>::from("\t");
                                    v26.clone()
                                } else {
                                    let mut v27: bool = v20 == b'"' ;
                                    if v27 {
                                        let mut v28: Rc<str> = Rc::<str>::from("\"");
                                        v28.clone()
                                    } else {
                                        let mut v29: bool = v20 == b'\\' ;
                                        if v29 {
                                            let mut v30: Rc<str> = Rc::<str>::from("\\");
                                            v30.clone()
                                        } else {
                                            let mut v31: Rc<str> = Rc::<str>::from("");
                                            v31.clone()
                                        }
                                    }
                                }
                            }
                        };
                        let mut v37: Rc<str> = Rc::<str>::from("");
                        let mut v38: bool = v36 == v37 ;
                        if v38 {
                            return (v37.clone(), v1, false);
                        } else {
                            let mut v39: i32 = v1 + 2i32 ;
                            let mut v40: bool = v2 == v1 ;
                            let mut v44: Rc<str> = if v40 {
                                method9()
                            } else {
                                let mut v42: i32 = v1 - 1i32 ;
                                let mut v43: Rc<str> = string_slice(&v0.clone(), v2 as i64, v42 as i64);
                                v43.clone()
                            };
                            let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v44.clone()));
                            let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45.clone(), v36.clone()));
                            return method12(v0.clone(), v39, v46.clone());
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
fn closure1() -> Rc<dyn Fn(Rc<str>, i32) -> (Rc<str>, i32, bool)> {
    Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> (Rc<str>, i32, bool) {
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
            let mut v11: Rc<str> = Rc::<str>::from("");
            method11(v0.clone(), v9, v10, v11.clone())
        } else {
            let mut v15: Rc<str> = Rc::<str>::from("");
            (v15.clone(), v1, false)
        }
    })
}
fn method14(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 == v3 ;
        if v4 {
            let mut v5: bool = v1 == v1 ;
            let mut v9: Rc<str> = if v5 {
                method9()
            } else {
                let mut v7: i32 = v1 - 1i32 ;
                let mut v8: Rc<str> = string_slice(&v0.clone(), v1 as i64, v7 as i64);
                v8.clone()
            };
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v9.clone()));
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10.clone(), Rc::<str>::from("\"")));
            return v11.clone();
        } else {
            let mut v12: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v13: bool = v12 == b'"' ;
            let mut v28: Rc<str> = if v13 {
                let mut v14: Rc<str> = Rc::<str>::from("\\\"");
                v14.clone()
            } else {
                let mut v15: bool = v12 == b'\\' ;
                if v15 {
                    let mut v16: Rc<str> = Rc::<str>::from("\\\\");
                    v16.clone()
                } else {
                    let mut v17: bool = v12 == b'\n' ;
                    if v17 {
                        let mut v18: Rc<str> = Rc::<str>::from("\\n");
                        v18.clone()
                    } else {
                        let mut v19: bool = v12 == b'\r' ;
                        if v19 {
                            let mut v20: Rc<str> = Rc::<str>::from("\\r");
                            v20.clone()
                        } else {
                            let mut v21: bool = v12 == b'\t' ;
                            if v21 {
                                let mut v22: Rc<str> = Rc::<str>::from("\\t");
                                v22.clone()
                            } else {
                                let mut v23: Rc<str> = Rc::<str>::from("");
                                v23.clone()
                            }
                        }
                    }
                }
            };
            let mut v29: i32 = v1 + 1i32 ;
            let mut v30: Rc<str> = Rc::<str>::from("");
            let mut v31: bool = v28 == v30 ;
            if v31 {
                return method13(v0.clone(), v29, v1, v2.clone());
            } else {
                let mut v33: bool = v1 == v1 ;
                let mut v37: Rc<str> = if v33 {
                    method9()
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
fn method13(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: i32 = (v0.clone().len() as i32);
        let mut v5: bool = v1 == v4 ;
        if v5 {
            let mut v6: bool = v2 == v1 ;
            let mut v10: Rc<str> = if v6 {
                method9()
            } else {
                let mut v8: i32 = v1 - 1i32 ;
                let mut v9: Rc<str> = string_slice(&v0.clone(), v2 as i64, v8 as i64);
                v9.clone()
            };
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v10.clone()));
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11.clone(), Rc::<str>::from("\"")));
            return v12.clone();
        } else {
            let mut v13: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v14: bool = v13 == b'"' ;
            let mut v29: Rc<str> = if v14 {
                let mut v15: Rc<str> = Rc::<str>::from("\\\"");
                v15.clone()
            } else {
                let mut v16: bool = v13 == b'\\' ;
                if v16 {
                    let mut v17: Rc<str> = Rc::<str>::from("\\\\");
                    v17.clone()
                } else {
                    let mut v18: bool = v13 == b'\n' ;
                    if v18 {
                        let mut v19: Rc<str> = Rc::<str>::from("\\n");
                        v19.clone()
                    } else {
                        let mut v20: bool = v13 == b'\r' ;
                        if v20 {
                            let mut v21: Rc<str> = Rc::<str>::from("\\r");
                            v21.clone()
                        } else {
                            let mut v22: bool = v13 == b'\t' ;
                            if v22 {
                                let mut v23: Rc<str> = Rc::<str>::from("\\t");
                                v23.clone()
                            } else {
                                let mut v24: Rc<str> = Rc::<str>::from("");
                                v24.clone()
                            }
                        }
                    }
                }
            };
            let mut v30: i32 = v1 + 1i32 ;
            let mut v31: Rc<str> = Rc::<str>::from("");
            let mut v32: bool = v29 == v31 ;
            if v32 {
                (v0, v1, v2, v3) = (v0.clone(), v30, v2, v3.clone());
                continue;
            } else {
                let mut v34: bool = v2 == v1 ;
                let mut v38: Rc<str> = if v34 {
                    method9()
                } else {
                    let mut v36: i32 = v1 - 1i32 ;
                    let mut v37: Rc<str> = string_slice(&v0.clone(), v2 as i64, v36 as i64);
                    v37.clone()
                };
                let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v38.clone()));
                let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39.clone(), v29.clone()));
                return method14(v0.clone(), v30, v40.clone());
            }
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let mut v1: i32 = 0i32;
        let mut v2: i32 = 0i32;
        let mut v3: Rc<str> = Rc::<str>::from("\"");
        method13(v0.clone(), v1, v2, v3.clone())
    })
}
fn closure3() -> Rc<dyn Fn(Rc<str>, i32) -> i32> {
    Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> i32 {
        method1(v0.clone(), v1)
    })
}
fn closure4() -> Rc<dyn Fn(Rc<str>, i32) -> i32> {
    Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> i32 {
        method8(v0.clone(), v1)
    })
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> (Rc<str>, bool) {
    loop {
        let (mut v3, mut v4, mut v5, mut v6): (i32, i32, Rc<str>, bool) = method0(v0.clone(), v1);
        let mut v7: bool = v6 == false ;
        if v7 {
            let mut v8: Rc<str> = Rc::<str>::from("");
            return (v8.clone(), false);
        } else {
            let mut v9: Rc<str> = Rc::<str>::from("");
            let mut v10: bool = v5 == v9 ;
            if v10 {
                return (v2.clone(), true);
            } else {
                let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), v5.clone()));
                let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11.clone(), Rc::<str>::from("\n")));
                (v0, v1, v2) = (v0.clone(), v4, v12.clone());
                continue;
            }
        }
    }
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, bool)> {
    Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, bool) {
        let mut v1: i32 = 0i32;
        let mut v2: Rc<str> = Rc::<str>::from("");
        method15(v0.clone(), v1, v2.clone())
    })
}
pub fn eoie_predicate_token(v0: &str, v1: i32) -> (i32, i32, Rc<str>, bool) {
    closure0()(Rc::<str>::from(v0), v1)
}
pub fn eoie_predicate_literal(v0: &str, v1: i32) -> (Rc<str>, i32, bool) {
    closure1()(Rc::<str>::from(v0), v1)
}
pub fn eoie_predicate_encode(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_predicate_skip(v0: &str, v1: i32) -> i32 {
    closure3()(Rc::<str>::from(v0), v1)
}
pub fn eoie_predicate_word_end(v0: &str, v1: i32) -> i32 {
    closure4()(Rc::<str>::from(v0), v1)
}
pub fn eoie_predicate_canonical(v0: &str) -> (Rc<str>, bool) {
    closure5()(Rc::<str>::from(v0))
}
