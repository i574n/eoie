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
fn method2(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
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
fn method4() -> Rc<str> {
    let mut v0: Rc<str> = Rc::<str>::from("");
    v0.clone()
}
fn method3() -> Rc<str> {
    method4()
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let (mut v3, mut v4, mut v5, mut v6): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v1);
        let mut v7: bool = v6 == false ;
        let mut v10: bool = if v7 {
            true
        } else {
            let mut v8: Rc<str> = Rc::<str>::from("");
            let mut v9: bool = v5 == v8 ;
            v9
        };
        if v10 {
            return -2i32;
        } else {
            let mut v11: Rc<str> = Rc::<str>::from("{");
            let mut v12: bool = v5 == v11 ;
            let mut v15: bool = if v12 {
                true
            } else {
                let mut v13: Rc<str> = Rc::<str>::from("[");
                let mut v14: bool = v5 == v13 ;
                v14
            };
            if v15 {
                let mut v16: i32 = v2 + 1i32 ;
                (v0, v1, v2) = (v0.clone(), v4, v16);
                continue;
            } else {
                let mut v18: Rc<str> = Rc::<str>::from("}");
                let mut v19: bool = v5 == v18 ;
                let mut v22: bool = if v19 {
                    true
                } else {
                    let mut v20: Rc<str> = Rc::<str>::from("]");
                    let mut v21: bool = v5 == v20 ;
                    v21
                };
                if v22 {
                    let mut v23: bool = v2 == 1i32 ;
                    if v23 {
                        return v4;
                    } else {
                        let mut v24: i32 = v2 - 1i32 ;
                        (v0, v1, v2) = (v0.clone(), v4, v24);
                        continue;
                    }
                } else {
                    (v0, v1, v2) = (v0.clone(), v4, v2);
                    continue;
                }
            }
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let (mut v2, mut v3, mut v4, mut v5): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v1);
        let mut v6: bool = v5 == false ;
        if v6 {
            return -2i32;
        } else {
            let mut v7: Rc<str> = Rc::<str>::from("");
            let mut v8: bool = v4 == v7 ;
            let mut v11: bool = if v8 {
                true
            } else {
                let mut v9: Rc<str> = Rc::<str>::from(",");
                let mut v10: bool = v4 == v9 ;
                v10
            };
            let mut v14: bool = if v11 {
                true
            } else {
                let mut v12: Rc<str> = Rc::<str>::from("}");
                let mut v13: bool = v4 == v12 ;
                v13
            };
            let mut v17: bool = if v14 {
                true
            } else {
                let mut v15: Rc<str> = Rc::<str>::from("]");
                let mut v16: bool = v4 == v15 ;
                v16
            };
            if v17 {
                return v2;
            } else {
                (v0, v1) = (v0.clone(), v3);
                continue;
            }
        }
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    loop {
        let (mut v3, mut v4, mut v5, mut v6): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v1);
        let mut v7: bool = v6 == false ;
        if v7 {
            return -2i32;
        } else {
            let mut v8: Rc<str> = Rc::<str>::from("}");
            let mut v9: bool = v5 == v8 ;
            if v9 {
                return -1i32;
            } else {
                let (mut v10, mut v11, mut v12): (Rc<str>, i32, bool) = predicate_lex_domain::eoie_predicate_literal(&*v0,v3);
                let (mut v13, mut v14, mut v15, mut v16): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v4);
                let mut v17: bool = v12 == false ;
                let mut v19: bool = if v17 {
                    true
                } else {
                    let mut v18: bool = v16 == false ;
                    v18
                };
                let mut v23: bool = if v19 {
                    true
                } else {
                    let mut v20: Rc<str> = Rc::<str>::from(":");
                    let mut v21: bool = v15 == v20 ;
                    let mut v22: bool = v21 == false;
                    v22
                };
                if v23 {
                    return -2i32;
                } else {
                    let mut v24: bool = v10 == v2 ;
                    if v24 {
                        let mut v25: i32 = predicate_lex_domain::eoie_predicate_skip(&*v0,v14);
                        return v25;
                    } else {
                        let (mut v26, mut v27, mut v28, mut v29): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v14);
                        let mut v30: bool = v29 == false ;
                        let mut v33: bool = if v30 {
                            true
                        } else {
                            let mut v31: Rc<str> = Rc::<str>::from("");
                            let mut v32: bool = v28 == v31 ;
                            v32
                        };
                        let mut v43: i32 = if v33 {
                            -2i32
                        } else {
                            let mut v34: Rc<str> = Rc::<str>::from("{");
                            let mut v35: bool = v28 == v34 ;
                            let mut v38: bool = if v35 {
                                true
                            } else {
                                let mut v36: Rc<str> = Rc::<str>::from("[");
                                let mut v37: bool = v28 == v36 ;
                                v37
                            };
                            if v38 {
                                let mut v39: i32 = 1i32;
                                method6(v0.clone(), v27, v39)
                            } else {
                                method7(v0.clone(), v27)
                            }
                        };
                        let mut v44: bool = v43 < 0i32;
                        if v44 {
                            return -2i32;
                        } else {
                            let (mut v45, mut v46, mut v47, mut v48): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v43);
                            let mut v49: bool = v48 == false ;
                            if v49 {
                                return -2i32;
                            } else {
                                let mut v50: bool = v47 == v8 ;
                                if v50 {
                                    return -1i32;
                                } else {
                                    let mut v51: Rc<str> = Rc::<str>::from(",");
                                    let mut v52: bool = v47 == v51 ;
                                    if v52 {
                                        (v0, v1, v2) = (v0.clone(), v46, v2.clone());
                                        continue;
                                    } else {
                                        return -2i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method9(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    loop {
        let (mut v3, mut v4, mut v5, mut v6): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v1);
        let mut v7: bool = v6 == false ;
        if v7 {
            return -1i32;
        } else {
            let mut v8: Rc<str> = Rc::<str>::from("]");
            let mut v9: bool = v5 == v8 ;
            if v9 {
                return 0i32;
            } else {
                let mut v10: Rc<str> = Rc::<str>::from(",");
                let mut v11: bool = v5 == v10 ;
                if v11 {
                    (v0, v1, v2) = (v0.clone(), v4, v2.clone());
                    continue;
                } else {
                    let (mut v13, mut v14, mut v15): (Rc<str>, i32, bool) = predicate_lex_domain::eoie_predicate_literal(&*v0,v3);
                    let mut v16: bool = v15 == false ;
                    if v16 {
                        return -1i32;
                    } else {
                        let mut v17: bool = v13 == v2 ;
                        if v17 {
                            return 1i32;
                        } else {
                            (v0, v1, v2) = (v0.clone(), v14, v2.clone());
                            continue;
                        }
                    }
                }
            }
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: i32) -> i32 {
    loop {
        let (mut v6, mut v7, mut v8, mut v9): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v1);
        let mut v10: bool = v9 == false ;
        if v10 {
            return -5i32;
        } else {
            let mut v11: Rc<str> = Rc::<str>::from("]");
            let mut v12: bool = v8 == v11 ;
            if v12 {
                if v4 {
                    return v5;
                } else {
                    return -2i32;
                }
            } else {
                let mut v14: Rc<str> = Rc::<str>::from(",");
                let mut v15: bool = v8 == v14 ;
                if v15 {
                    (v0, v1, v2, v3, v4, v5) = (v0.clone(), v7, v2, v3.clone(), v4, v5);
                    continue;
                } else {
                    let mut v17: Rc<str> = Rc::<str>::from("{");
                    let mut v18: bool = v8 == v17 ;
                    let mut v19: bool = v18 == false;
                    if v19 {
                        return -5i32;
                    } else {
                        let (mut v20, mut v21, mut v22, mut v23): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v6);
                        let mut v24: bool = v23 == false ;
                        let mut v27: bool = if v24 {
                            true
                        } else {
                            let mut v25: Rc<str> = Rc::<str>::from("");
                            let mut v26: bool = v22 == v25 ;
                            v26
                        };
                        let mut v36: i32 = if v27 {
                            -2i32
                        } else {
                            let mut v28: bool = v22 == v17 ;
                            let mut v31: bool = if v28 {
                                true
                            } else {
                                let mut v29: Rc<str> = Rc::<str>::from("[");
                                let mut v30: bool = v22 == v29 ;
                                v30
                            };
                            if v31 {
                                let mut v32: i32 = 1i32;
                                method6(v0.clone(), v21, v32)
                            } else {
                                method7(v0.clone(), v21)
                            }
                        };
                        let mut v37: bool = v6 < 0i32;
                        let mut v47: i32 = if v37 {
                            -2i32
                        } else {
                            let (mut v38, mut v39, mut v40, mut v41): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v6);
                            let mut v43: bool = if v41 {
                                let mut v42: bool = v40 == v17 ;
                                v42
                            } else {
                                false
                            };
                            if v43 {
                                let mut v44: Rc<str> = Rc::<str>::from("name");
                                method8(v0.clone(), v39, v44.clone())
                            } else {
                                -2i32
                            }
                        };
                        let mut v57: i32 = if v37 {
                            -2i32
                        } else {
                            let (mut v48, mut v49, mut v50, mut v51): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v6);
                            let mut v53: bool = if v51 {
                                let mut v52: bool = v50 == v17 ;
                                v52
                            } else {
                                false
                            };
                            if v53 {
                                let mut v54: Rc<str> = Rc::<str>::from("id");
                                method8(v0.clone(), v49, v54.clone())
                            } else {
                                -2i32
                            }
                        };
                        let (mut v58, mut v59, mut v60): (Rc<str>, i32, bool) = predicate_lex_domain::eoie_predicate_literal(&*v0,v47);
                        let (mut v61, mut v62, mut v63): (Rc<str>, i32, bool) = predicate_lex_domain::eoie_predicate_literal(&*v0,v57);
                        let mut v64: bool = v36 < 0i32;
                        let mut v66: bool = if v64 {
                            true
                        } else {
                            let mut v65: bool = v60 == false ;
                            v65
                        };
                        let mut v68: bool = if v66 {
                            true
                        } else {
                            let mut v67: bool = v63 == false ;
                            v67
                        };
                        if v68 {
                            return -5i32;
                        } else {
                            let mut v69: bool = v58 == v3 ;
                            if v69 {
                                let mut v70: i32 = method9(v0.clone(), v2, v61.clone());
                                let mut v71: bool = v70 < 0i32;
                                if v71 {
                                    return -5i32;
                                } else {
                                    let mut v72: bool = v70 == 1i32 ;
                                    if v72 {
                                        if v4 {
                                            return -2i32;
                                        } else {
                                            let mut v82: i32 = if v37 {
                                                -2i32
                                            } else {
                                                let (mut v73, mut v74, mut v75, mut v76): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v6);
                                                let mut v78: bool = if v76 {
                                                    let mut v77: bool = v75 == v17 ;
                                                    v77
                                                } else {
                                                    false
                                                };
                                                if v78 {
                                                    let mut v79: Rc<str> = Rc::<str>::from("metadata");
                                                    method8(v0.clone(), v74, v79.clone())
                                                } else {
                                                    -2i32
                                                }
                                            };
                                            let mut v83: bool = v82 < 0i32;
                                            let mut v93: i32 = if v83 {
                                                -2i32
                                            } else {
                                                let (mut v84, mut v85, mut v86, mut v87): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v82);
                                                let mut v89: bool = if v87 {
                                                    let mut v88: bool = v86 == v17 ;
                                                    v88
                                                } else {
                                                    false
                                                };
                                                if v89 {
                                                    let mut v90: Rc<str> = Rc::<str>::from("eoie");
                                                    method8(v0.clone(), v85, v90.clone())
                                                } else {
                                                    -2i32
                                                }
                                            };
                                            let mut v94: bool = v93 < 0i32;
                                            let mut v133: i32 = if v94 {
                                                0i32
                                            } else {
                                                let (mut v95, mut v96, mut v97, mut v98): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v93);
                                                let mut v101: bool = if v98 {
                                                    let mut v99: Rc<str> = Rc::<str>::from("null");
                                                    let mut v100: bool = v97 == v99 ;
                                                    v100
                                                } else {
                                                    false
                                                };
                                                if v101 {
                                                    0i32
                                                } else {
                                                    let mut v102: bool = v98 == false ;
                                                    let mut v105: bool = if v102 {
                                                        true
                                                    } else {
                                                        let mut v103: bool = v97 == v17 ;
                                                        let mut v104: bool = v103 == false;
                                                        v104
                                                    };
                                                    if v105 {
                                                        -3i32
                                                    } else {
                                                        let (mut v106, mut v107, mut v108, mut v109): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v93);
                                                        let mut v111: bool = if v109 {
                                                            let mut v110: bool = v108 == v17 ;
                                                            v110
                                                        } else {
                                                            false
                                                        };
                                                        let mut v114: i32 = if v111 {
                                                            let mut v112: Rc<str> = Rc::<str>::from("test-requires-cli");
                                                            method8(v0.clone(), v107, v112.clone())
                                                        } else {
                                                            -2i32
                                                        };
                                                        let mut v115: bool = v114 == -1i32 ;
                                                        if v115 {
                                                            0i32
                                                        } else {
                                                            let mut v116: bool = v114 < 0i32;
                                                            if v116 {
                                                                -3i32
                                                            } else {
                                                                let (mut v117, mut v118, mut v119, mut v120): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v114);
                                                                let mut v123: bool = if v120 {
                                                                    let mut v121: Rc<str> = Rc::<str>::from("true");
                                                                    let mut v122: bool = v119 == v121 ;
                                                                    v122
                                                                } else {
                                                                    false
                                                                };
                                                                if v123 {
                                                                    1i32
                                                                } else {
                                                                    let mut v126: bool = if v120 {
                                                                        let mut v124: Rc<str> = Rc::<str>::from("false");
                                                                        let mut v125: bool = v119 == v124 ;
                                                                        v125
                                                                    } else {
                                                                        false
                                                                    };
                                                                    if v126 {
                                                                        0i32
                                                                    } else {
                                                                        -4i32
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            let mut v134: bool = v133 < 0i32;
                                            if v134 {
                                                return v133;
                                            } else {
                                                let mut v135: bool = true;
                                                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v36, v2, v3.clone(), v135, v133);
                                                continue;
                                            }
                                        }
                                    } else {
                                        (v0, v1, v2, v3, v4, v5) = (v0.clone(), v36, v2, v3.clone(), v4, v5);
                                        continue;
                                    }
                                }
                            } else {
                                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v36, v2, v3.clone(), v4, v5);
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: u8, mut v4: i32, mut v5: i32, mut v6: i32) -> i32 {
    loop {
        let mut v7: i32 = (v1.clone().len() as i32);
        let mut v8: i32 = method2(v1.clone(), v2, v7, v3);
        let mut v9: bool = v2 == v8 ;
        let mut v13: Rc<str> = if v9 {
            method3()
        } else {
            let mut v11: i32 = v8 - 1i32 ;
            let mut v12: Rc<str> = string_slice(&v1.clone(), v2 as i64, v11 as i64);
            v12.clone()
        };
        let mut v14: bool = false;
        let mut v15: i32 = 0i32;
        let mut v16: i32 = method5(v0.clone(), v4, v5, v13.clone(), v14, v15);
        let mut v17: bool = v16 < 0i32;
        if v17 {
            return v16;
        } else {
            let mut v18: bool = v16 == 1i32 ;
            let mut v19: i32 = if v18 {
                1i32
            } else {
                v6
            };
            let mut v20: bool = v8 == v7 ;
            if v20 {
                return v19;
            } else {
                let mut v21: i32 = v8 + 1i32 ;
                (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v21, v3, v4, v5, v19);
                continue;
            }
        }
    }
}
fn method0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let (mut v3, mut v4, mut v5, mut v6): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,0i32);
    let mut v9: bool = if v6 {
        let mut v7: Rc<str> = Rc::<str>::from("{");
        let mut v8: bool = v5 == v7 ;
        v8
    } else {
        false
    };
    let mut v12: i32 = if v9 {
        let mut v10: Rc<str> = Rc::<str>::from("packages");
        method8(v0.clone(), v4, v10.clone())
    } else {
        -2i32
    };
    let (mut v13, mut v14, mut v15, mut v16): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,0i32);
    let mut v19: bool = if v16 {
        let mut v17: Rc<str> = Rc::<str>::from("{");
        let mut v18: bool = v15 == v17 ;
        v18
    } else {
        false
    };
    let mut v22: i32 = if v19 {
        let mut v20: Rc<str> = Rc::<str>::from("workspace_members");
        method8(v0.clone(), v14, v20.clone())
    } else {
        -2i32
    };
    let mut v23: bool = v12 < 0i32;
    let mut v25: bool = if v23 {
        true
    } else {
        let mut v24: bool = v22 < 0i32;
        v24
    };
    if v25 {
        -5i32
    } else {
        let (mut v26, mut v27, mut v28, mut v29): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v12);
        let (mut v30, mut v31, mut v32, mut v33): (i32, i32, Rc<str>, bool) = predicate_lex_domain::eoie_predicate_token(&*v0,v22);
        let mut v34: bool = v29 && v33;
        let mut v37: bool = if v34 {
            let mut v35: Rc<str> = Rc::<str>::from("[");
            let mut v36: bool = v28 == v35 ;
            v36
        } else {
            false
        };
        let mut v40: bool = if v37 {
            let mut v38: Rc<str> = Rc::<str>::from("[");
            let mut v39: bool = v32 == v38 ;
            v39
        } else {
            false
        };
        if v40 {
            let mut v41: i32 = 0i32;
            let mut v42: u8 = v2.clone().as_bytes()[0i32 as usize];
            let mut v43: i32 = 0i32;
            method1(v0.clone(), v1.clone(), v41, v42, v27, v31, v43)
        } else {
            -5i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> i32> {
    Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> i32 {
        method0(v0.clone(), v1.clone(), v2.clone())
    })
}
pub fn eoie_developer_requirements(v0: &str, v1: &str, v2: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
