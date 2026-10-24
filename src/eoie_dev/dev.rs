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
fn command_line_argument_count_1() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn count_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_1();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn validate_2(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let mut v2: bool = v0 == v1;
        if v2 {
            return true;
        } else {
            let mut v3: Rc<str> = std::env::args().nth(v0 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
            let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = if v4 {
                true
            } else {
                let mut v5: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--skip-release"); } LIT.with(|lit| lit.clone()) };
                if v5 {
                    true
                } else {
                    let mut v6: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--offline"); } LIT.with(|lit| lit.clone()) };
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compiler-contracts"); } LIT.with(|lit| lit.clone()) };
                        if v7 {
                            true
                        } else {
                            let mut v8: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dry-run"); } LIT.with(|lit| lit.clone()) };
                            if v8 {
                                true
                            } else {
                                let mut v9: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--help"); } LIT.with(|lit| lit.clone()) };
                                v9
                            }
                        }
                    }
                }
            };
            if v14 {
                let mut v15: i32 = v0.wrapping_add(1i32);
                (v0, v1) = (v15, v1);
                continue;
            } else {
                let mut v17: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--root"); } LIT.with(|lit| lit.clone()) };
                let mut v25: bool = if v17 {
                    true
                } else {
                    let mut v18: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
                    if v18 {
                        true
                    } else {
                        let mut v19: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--target-dir"); } LIT.with(|lit| lit.clone()) };
                        if v19 {
                            true
                        } else {
                            let mut v20: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compiler"); } LIT.with(|lit| lit.clone()) };
                            if v20 {
                                true
                            } else {
                                let mut v21: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout-ms"); } LIT.with(|lit| lit.clone()) };
                                v21
                            }
                        }
                    }
                };
                if v25 {
                    let mut v26: i32 = v0.wrapping_add(1i32);
                    let mut v27: bool = v26 < v1;
                    if v27 {
                        let mut v28: Rc<str> = std::env::args().nth(v26 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
                        let mut v29: bool = v28.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        if v29 {
                            return false;
                        } else {
                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                            let mut v31: bool = v28.starts_with(&*v30);
                            if v31 {
                                return false;
                            } else {
                                let mut v32: i32 = v0.wrapping_add(2i32);
                                (v0, v1) = (v32, v1);
                                continue;
                            }
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
}
fn has_from_3(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> bool {
    loop {
        let mut v3: bool = v0 == v1;
        if v3 {
            return false;
        } else {
            let mut v4: Rc<str> = std::env::args().nth(v0 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
            let mut v5: bool = v2.clone() == v4.clone();
            if v5 {
                return true;
            } else {
                let mut v6: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--root"); } LIT.with(|lit| lit.clone()) };
                let mut v14: bool = if v6 {
                    true
                } else {
                    let mut v7: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--target-dir"); } LIT.with(|lit| lit.clone()) };
                        if v8 {
                            true
                        } else {
                            let mut v9: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compiler"); } LIT.with(|lit| lit.clone()) };
                            if v9 {
                                true
                            } else {
                                let mut v10: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout-ms"); } LIT.with(|lit| lit.clone()) };
                                v10
                            }
                        }
                    }
                };
                let mut v15: i32 = if v14 {
                    2i32
                } else {
                    1i32
                };
                let mut v16: i32 = v0.wrapping_add(v15);
                (v0, v1, v2) = (v16, v1, v2.clone());
                continue;
            }
        }
    }
}
fn arg_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v2));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
    v4.clone()
}
fn packages_from_4(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: bool) -> Rc<str> {
    loop {
        let mut v4: bool = v0 == v1;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: Rc<str> = std::env::args().nth(v0 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
            let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--root"); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = if v6 {
                true
            } else {
                let mut v7: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
                if v7 {
                    true
                } else {
                    let mut v8: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--target-dir"); } LIT.with(|lit| lit.clone()) };
                    if v8 {
                        true
                    } else {
                        let mut v9: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compiler"); } LIT.with(|lit| lit.clone()) };
                        if v9 {
                            true
                        } else {
                            let mut v10: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout-ms"); } LIT.with(|lit| lit.clone()) };
                            v10
                        }
                    }
                }
            };
            let mut v15: i32 = if v14 {
                2i32
            } else {
                1i32
            };
            let mut v16: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = if v16 {
                let mut v17: i32 = v0.wrapping_add(1i32);
                let mut v18: Rc<str> = std::env::args().nth(v17 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
                if v3 {
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
                    let mut v20: Rc<str> = arg_5(v2.clone(), v19.clone());
                    arg_5(v20.clone(), v18.clone())
                } else {
                    let mut v22: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v22 {
                        v18.clone()
                    } else {
                        arg_5(v2.clone(), v18.clone())
                    }
                }
            } else {
                v2.clone()
            };
            let mut v27: i32 = v0.wrapping_add(v15);
            (v0, v1, v2, v3) = (v27, v1, v26.clone(), v3);
            continue;
        }
    }
}
fn value_from_6(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v0 == v1;
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = std::env::args().nth(v0 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
            let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--root"); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = if v6 {
                true
            } else {
                let mut v7: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
                if v7 {
                    true
                } else {
                    let mut v8: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--target-dir"); } LIT.with(|lit| lit.clone()) };
                    if v8 {
                        true
                    } else {
                        let mut v9: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compiler"); } LIT.with(|lit| lit.clone()) };
                        if v9 {
                            true
                        } else {
                            let mut v10: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout-ms"); } LIT.with(|lit| lit.clone()) };
                            v10
                        }
                    }
                }
            };
            let mut v15: i32 = if v14 {
                2i32
            } else {
                1i32
            };
            let mut v16: bool = v2.clone() == v5.clone();
            let mut v19: Rc<str> = if v16 {
                let mut v17: i32 = v0.wrapping_add(1i32);
                let mut v18: Rc<str> = std::env::args().nth(v17 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
                v18.clone()
            } else {
                v3.clone()
            };
            let mut v20: i32 = v0.wrapping_add(v15);
            (v0, v1, v2, v3) = (v20, v1, v2.clone(), v19.clone());
            continue;
        }
    }
}
fn append_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    v2.clone()
}
fn argument_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v2));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
    v4.clone()
}
fn requirements_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: u64) -> i32 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metadata"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--no-deps"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = argument_9(v4.clone(), v5.clone());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--format-version"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = argument_9(v6.clone(), v7.clone());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = argument_9(v8.clone(), v9.clone());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--locked"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = argument_9(v10.clone(), v11.clone());
    let mut v15: Rc<str> = if v2 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--offline"); } LIT.with(|lit| lit.clone()) };
        argument_9(v12.clone(), v13.clone())
    } else {
        v12.clone()
    };
    let (mut v16, mut v17): (bool, Rc<str>) = { let mut command=std::process::Command::new("cargo"); command.current_dir(std::path::Path::new(&*v0).join("src")).args(v15.split(char::from(0u8))); match eoie_process::run_bounded_capture_with_input(&mut command,v3,None) { Ok(capture) => match String::from_utf8(capture.stdout) { Ok(text) => (capture.termination == eoie_process::ProcessTermination::Exited(0), std::rc::Rc::<str>::from(text)), Err(error) => (false,std::rc::Rc::<str>::from(error.to_string())) }, Err(error) => (false,std::rc::Rc::<str>::from(error)) } };
    let mut v20: i32 = if v16 {
        let mut v18: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
        let mut v19: i32 = developer_metadata_domain::eoie_developer_requirements(&*v17,&*v1,&*v18);
        v19
    } else {
        -1i32
    };
    let mut v21: bool = v20 < 0i32;
    if v21 {
        let mut v22: bool = v20 == -2i32 ;
        let mut v31: Rc<str> = if v22 {
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unknown selected Cargo package or ambiguous workspace name\n"); } LIT.with(|lit| lit.clone()) };
            v23.clone()
        } else {
            let mut v24: bool = v20 == -3i32 ;
            if v24 {
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie test metadata must be a table\n"); } LIT.with(|lit| lit.clone()) };
                v25.clone()
            } else {
                let mut v26: bool = v20 == -4i32 ;
                if v26 {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-requires-cli must be a boolean\n"); } LIT.with(|lit| lit.clone()) };
                    v27.clone()
                } else {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cannot inspect selected Cargo test requirements\n"); } LIT.with(|lit| lit.clone()) };
                    v28.clone()
                }
            }
        };
        std::io::Write::write_all(&mut std::io::stdout(),v31.as_bytes()).unwrap();
        ()
    };
    v20
}
fn find_char_loop_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
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
fn method13() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_12() -> Rc<str> {
    method13()
}
fn quoted_arguments_10(mut v0: Rc<str>, mut v1: i32, mut v2: u8) -> Rc<str> {
    let mut v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = find_char_loop_11(v0.clone(), v1, v3, v2);
    let mut v5: bool = v1 == v4 ;
    let mut v9: Rc<str> = if v5 {
        codec_empty_12()
    } else {
        let mut v7: i32 = v4 - 1i32 ;
        let mut v8: Rc<str> = string_slice(&v0.clone(), v1 as i64, v7 as i64);
        v8.clone()
    };
    let mut v10: Rc<str> = predicate_literal_domain::eoie_predicate_encode(&*v9);
    let mut v11: bool = v4 == v3 ;
    if v11 {
        v10.clone()
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v12));
        let mut v14: i32 = v4.wrapping_add(1i32);
        let mut v15: Rc<str> = quoted_arguments_10(v0.clone(), v14, v2);
        let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v15));
        v16.clone()
    }
}
fn spiral_main() -> i32 {
    let mut v0: i32 = count_0();
    let mut v1: i32 = 1i32;
    let mut v2: bool = validate_2(v1, v0);
    if v2 {
        let mut v3: i32 = 1i32;
        let mut v4: i32 = count_0();
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--help"); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = has_from_3(v3, v4, v5.clone());
        if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-dev [--test] [--skip-release] [--package NAME] [--offline] [--root PATH] [--target-dir PATH] [--compiler-contracts] [--compiler PATH] [--timeout-ms N] [--dry-run]\nBuilds a candidate; installation remains a separately validated operation.\n"); } LIT.with(|lit| lit.clone()) };
            std::io::Write::write_all(&mut std::io::stdout(),v7.as_bytes()).unwrap();
            0i32
        } else {
            let mut v8: i32 = 1i32;
            let mut v9: i32 = count_0();
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
            let mut v11: bool = has_from_3(v8, v9, v10.clone());
            let mut v12: i32 = 1i32;
            let mut v13: i32 = count_0();
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--skip-release"); } LIT.with(|lit| lit.clone()) };
            let mut v15: bool = has_from_3(v12, v13, v14.clone());
            let mut v16: i32 = 1i32;
            let mut v17: i32 = count_0();
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compiler-contracts"); } LIT.with(|lit| lit.clone()) };
            let mut v19: bool = has_from_3(v16, v17, v18.clone());
            let mut v20: i32 = 1i32;
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v22: bool = false;
            let mut v23: Rc<str> = packages_from_4(v20, v0, v21.clone(), v22);
            let mut v24: i32 = 1i32;
            let mut v25: i32 = count_0();
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compiler"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = value_from_6(v24, v25, v26.clone(), v21.clone());
            let mut v28: bool = v27.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = if v28 {
                let mut v29: Rc<str> = std::env::var("EOIE_SPIRAL_COMPILE").map(std::rc::Rc::<str>::from).unwrap_or_default();
                v29.clone()
            } else {
                let mut v30: Rc<str> = std::fs::canonicalize(&*v27).ok().filter(|p| p.is_file()).and_then(|p| p.to_str().map(std::rc::Rc::<str>::from)).unwrap_or_default();
                v30.clone()
            };
            let mut v32: i32 = 1i32;
            let mut v33: i32 = count_0();
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout-ms"); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("900000"); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = value_from_6(v32, v33, v34.clone(), v35.clone());
            let mut v37: u64 = v36.parse::<u64>().unwrap_or(0);
            let mut v38: i32 = 1i32;
            let mut v39: i32 = count_0();
            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--root"); } LIT.with(|lit| lit.clone()) };
            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
            let mut v42: Rc<str> = value_from_6(v38, v39, v40.clone(), v41.clone());
            let mut v43: Rc<str> = std::path::absolute(&*v42).ok().and_then(|p| p.to_str().map(std::rc::Rc::<str>::from)).unwrap_or_default();
            let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/src/target"); } LIT.with(|lit| lit.clone()) };
            let mut v45: Rc<str> = append_7(v43.clone(), v44.clone());
            let mut v46: i32 = 1i32;
            let mut v47: i32 = count_0();
            let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--target-dir"); } LIT.with(|lit| lit.clone()) };
            let mut v49: Rc<str> = value_from_6(v46, v47, v48.clone(), v45.clone());
            let mut v50: Rc<str> = std::path::absolute(&*v49).ok().and_then(|p| p.to_str().map(std::rc::Rc::<str>::from)).unwrap_or_default();
            let mut v51: i32 = 1i32;
            let mut v52: i32 = count_0();
            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dry-run"); } LIT.with(|lit| lit.clone()) };
            let mut v54: bool = has_from_3(v51, v52, v53.clone());
            let mut v55: bool = v11 == false;
            let mut v60: bool = if v55 {
                let mut v56: bool = v15 || v19;
                if v56 {
                    true
                } else {
                    let mut v57: bool = v23.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v58: bool = v57 == false;
                    v58
                }
            } else {
                false
            };
            if v60 {
                let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--skip-release, --package and --compiler-contracts require --test"); } LIT.with(|lit| lit.clone()) };
                let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v63: Rc<str> = append_7(v61.clone(), v62.clone());
                std::io::Write::write_all(&mut std::io::stdout(),v63.as_bytes()).unwrap();
                2i32
            } else {
                let mut v64: bool = v28 == false;
                let mut v66: bool = if v64 {
                    let mut v65: bool = v31.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v65
                } else {
                    false
                };
                if v66 {
                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compiler must identify an existing file"); } LIT.with(|lit| lit.clone()) };
                    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v69: Rc<str> = append_7(v67.clone(), v68.clone());
                    std::io::Write::write_all(&mut std::io::stdout(),v69.as_bytes()).unwrap();
                    2i32
                } else {
                    let mut v71: bool = if v19 {
                        let mut v70: bool = v31.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v70
                    } else {
                        false
                    };
                    if v71 {
                        let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compiler-contracts requires --compiler or EOIE_SPIRAL_COMPILE"); } LIT.with(|lit| lit.clone()) };
                        let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v74: Rc<str> = append_7(v72.clone(), v73.clone());
                        std::io::Write::write_all(&mut std::io::stdout(),v74.as_bytes()).unwrap();
                        2i32
                    } else {
                        let mut v75: bool = v37 == 0u64;
                        let mut v77: bool = if v75 {
                            true
                        } else {
                            let mut v76: bool = v37 > 3600000u64;
                            v76
                        };
                        if v77 {
                            let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout-ms must be between 1 and 3600000"); } LIT.with(|lit| lit.clone()) };
                            let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v80: Rc<str> = append_7(v78.clone(), v79.clone());
                            std::io::Write::write_all(&mut std::io::stdout(),v80.as_bytes()).unwrap();
                            2i32
                        } else {
                            let mut v81: bool = std::path::Path::new(&*v43).join("src/Cargo.toml").is_file();
                            let mut v82: bool = v81 == false;
                            if v82 {
                                let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--root must contain src/Cargo.toml"); } LIT.with(|lit| lit.clone()) };
                                let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v85: Rc<str> = append_7(v83.clone(), v84.clone());
                                std::io::Write::write_all(&mut std::io::stdout(),v85.as_bytes()).unwrap();
                                2i32
                            } else {
                                let mut v86: bool = v54 == false;
                                let mut v88: bool = if v86 {
                                    let mut v87: bool = std::env::current_exe().ok().and_then(|p| p.parent().and_then(std::path::Path::parent).map(std::path::Path::to_owned)).and_then(|p| std::fs::canonicalize(p).ok()).zip(std::fs::canonicalize(&*v50).ok()).is_some_and(|(running,target)| running==target);
                                    v87
                                } else {
                                    false
                                };
                                if v88 {
                                    let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("The workflow executable is running from the build target; bootstrap eoie-dev with a separate --target-dir"); } LIT.with(|lit| lit.clone()) };
                                    let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                    let mut v91: Rc<str> = append_7(v89.clone(), v90.clone());
                                    std::io::Write::write_all(&mut std::io::stdout(),v91.as_bytes()).unwrap();
                                    2i32
                                } else {
                                    let mut v92: i32 = 1i32;
                                    let mut v93: i32 = count_0();
                                    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--offline"); } LIT.with(|lit| lit.clone()) };
                                    let mut v95: bool = has_from_3(v92, v93, v94.clone());
                                    let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--locked"); } LIT.with(|lit| lit.clone()) };
                                    let mut v97: Rc<str> = arg_5(v96.clone(), v48.clone());
                                    let mut v98: Rc<str> = arg_5(v97.clone(), v50.clone());
                                    let mut v100: Rc<str> = if v95 {
                                        arg_5(v98.clone(), v94.clone())
                                    } else {
                                        v98.clone()
                                    };
                                    let mut v101: bool = v23.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v103: i32 = if v101 {
                                        1i32
                                    } else {
                                        requirements_8(v43.clone(), v23.clone(), v95, v37)
                                    };
                                    let mut v104: bool = v103 < 0i32;
                                    if v104 {
                                        2i32
                                    } else {
                                        let mut v106: bool = if v11 {
                                            let mut v105: bool = v103 == 1i32;
                                            v105
                                        } else {
                                            false
                                        };
                                        let mut v123: i32 = if v106 {
                                            let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("build"); } LIT.with(|lit| lit.clone()) };
                                            let mut v108: Rc<str> = arg_5(v107.clone(), v100.clone());
                                            let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
                                            let mut v110: Rc<str> = arg_5(v108.clone(), v109.clone());
                                            let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-cli"); } LIT.with(|lit| lit.clone()) };
                                            let mut v112: Rc<str> = arg_5(v110.clone(), v111.clone());
                                            if v54 {
                                                let mut v113: i32 = 0i32;
                                                let mut v114: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                                                let mut v115: u8 = v114.clone().as_bytes()[0i32 as usize];
                                                let mut v116: Rc<str> = quoted_arguments_10(v112.clone(), v113, v115);
                                                let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cargo "); } LIT.with(|lit| lit.clone()) };
                                                let mut v118: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v116));
                                                let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                let mut v120: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v119));
                                                std::io::Write::write_all(&mut std::io::stdout(),v120.as_bytes()).unwrap();
                                                0i32
                                            } else {
                                                let mut v121: i32 = { let mut command=std::process::Command::new("cargo"); command.current_dir(std::path::Path::new(&*v43).join("src")); command.args(v112.split(char::from(0u8))); if v31.len() > 0 { command.env("EOIE_SPIRAL_COMPILE", &*v31); } if std::env::var_os("CARGO_BUILD_JOBS").is_none() { command.env("CARGO_BUILD_JOBS", "2"); } if std::env::var_os("RUST_TEST_THREADS").is_none() { command.env("RUST_TEST_THREADS", "1"); } match eoie_process::run_bounded(&mut command,v37) { Ok(status) => status.code().unwrap_or(1), Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); 2 } } };
                                                v121
                                            }
                                        } else {
                                            0i32
                                        };
                                        let mut v124: bool = v123 == 0i32;
                                        if v124 {
                                            let mut v151: i32 = if v11 {
                                                let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
                                                let mut v126: Rc<str> = arg_5(v125.clone(), v100.clone());
                                                let mut v132: Rc<str> = if v101 {
                                                    let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--workspace"); } LIT.with(|lit| lit.clone()) };
                                                    arg_5(v126.clone(), v127.clone())
                                                } else {
                                                    let mut v129: i32 = 1i32;
                                                    let mut v130: bool = true;
                                                    packages_from_4(v129, v0, v126.clone(), v130)
                                                };
                                                let mut v137: Rc<str> = if v19 {
                                                    let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v134: Rc<str> = arg_5(v132.clone(), v133.clone());
                                                    let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--include-ignored"); } LIT.with(|lit| lit.clone()) };
                                                    arg_5(v134.clone(), v135.clone())
                                                } else {
                                                    v132.clone()
                                                };
                                                let mut v147: i32 = if v54 {
                                                    let mut v138: i32 = 0i32;
                                                    let mut v139: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                                                    let mut v140: u8 = v139.clone().as_bytes()[0i32 as usize];
                                                    let mut v141: Rc<str> = quoted_arguments_10(v137.clone(), v138, v140);
                                                    let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cargo "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v143: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v141));
                                                    let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v145: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v144));
                                                    std::io::Write::write_all(&mut std::io::stdout(),v145.as_bytes()).unwrap();
                                                    0i32
                                                } else {
                                                    let mut v146: i32 = { let mut command=std::process::Command::new("cargo"); command.current_dir(std::path::Path::new(&*v43).join("src")); command.args(v137.split(char::from(0u8))); if v31.len() > 0 { command.env("EOIE_SPIRAL_COMPILE", &*v31); } if std::env::var_os("CARGO_BUILD_JOBS").is_none() { command.env("CARGO_BUILD_JOBS", "2"); } if std::env::var_os("RUST_TEST_THREADS").is_none() { command.env("RUST_TEST_THREADS", "1"); } match eoie_process::run_bounded(&mut command,v37) { Ok(status) => status.code().unwrap_or(1), Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); 2 } } };
                                                    v146
                                                };
                                                let mut v148: bool = v147 == 0i32;
                                                if v148 {
                                                    let mut v149: i32 = eoie_dev_spiral_tests::eoie_dev_spiral_tests_run(&*v43,&*v50,&*v23,&char::from(0u8).to_string(),v54,v37);
                                                    v149
                                                } else {
                                                    v147
                                                }
                                            } else {
                                                0i32
                                            };
                                            let mut v152: bool = v151 == 0i32;
                                            if v152 {
                                                if v15 {
                                                    0i32
                                                } else {
                                                    let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("build"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v154: Rc<str> = arg_5(v153.clone(), v100.clone());
                                                    let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--release"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v156: Rc<str> = arg_5(v154.clone(), v155.clone());
                                                    let mut v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--package"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v158: Rc<str> = arg_5(v156.clone(), v157.clone());
                                                    let mut v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-cli"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v160: Rc<str> = arg_5(v158.clone(), v159.clone());
                                                    if v54 {
                                                        let mut v161: i32 = 0i32;
                                                        let mut v162: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                                                        let mut v163: u8 = v162.clone().as_bytes()[0i32 as usize];
                                                        let mut v164: Rc<str> = quoted_arguments_10(v160.clone(), v161, v163);
                                                        let mut v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cargo "); } LIT.with(|lit| lit.clone()) };
                                                        let mut v166: Rc<str> = Rc::<str>::from(format!("{}{}", v165, v164));
                                                        let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v168: Rc<str> = Rc::<str>::from(format!("{}{}", v166, v167));
                                                        std::io::Write::write_all(&mut std::io::stdout(),v168.as_bytes()).unwrap();
                                                        0i32
                                                    } else {
                                                        let mut v169: i32 = { let mut command=std::process::Command::new("cargo"); command.current_dir(std::path::Path::new(&*v43).join("src")); command.args(v160.split(char::from(0u8))); if v31.len() > 0 { command.env("EOIE_SPIRAL_COMPILE", &*v31); } if std::env::var_os("CARGO_BUILD_JOBS").is_none() { command.env("CARGO_BUILD_JOBS", "2"); } if std::env::var_os("RUST_TEST_THREADS").is_none() { command.env("RUST_TEST_THREADS", "1"); } match eoie_process::run_bounded(&mut command,v37) { Ok(status) => status.code().unwrap_or(1), Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); 2 } } };
                                                        v169
                                                    }
                                                }
                                            } else {
                                                v151
                                            }
                                        } else {
                                            v123
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Invalid option or missing value; use --help"); } LIT.with(|lit| lit.clone()) };
        let mut v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v184: Rc<str> = append_7(v182.clone(), v183.clone());
        std::io::Write::write_all(&mut std::io::stdout(),v184.as_bytes()).unwrap();
        2i32
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
