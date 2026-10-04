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
fn method0(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let mut v2: bool = v0 == v1;
        if v2 {
            return true;
        } else {
            let mut v3: Rc<str> = std::env::args().nth(v0 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
            let mut v4: Rc<str> = Rc::<str>::from("--test");
            let mut v5: bool = v3 == v4 ;
            let mut v20: bool = if v5 {
                true
            } else {
                let mut v6: Rc<str> = Rc::<str>::from("--skip-release");
                let mut v7: bool = v3 == v6 ;
                if v7 {
                    true
                } else {
                    let mut v8: Rc<str> = Rc::<str>::from("--offline");
                    let mut v9: bool = v3 == v8 ;
                    if v9 {
                        true
                    } else {
                        let mut v10: Rc<str> = Rc::<str>::from("--compiler-contracts");
                        let mut v11: bool = v3 == v10 ;
                        if v11 {
                            true
                        } else {
                            let mut v12: Rc<str> = Rc::<str>::from("--dry-run");
                            let mut v13: bool = v3 == v12 ;
                            if v13 {
                                true
                            } else {
                                let mut v14: Rc<str> = Rc::<str>::from("--help");
                                let mut v15: bool = v3 == v14 ;
                                v15
                            }
                        }
                    }
                }
            };
            if v20 {
                let mut v21: i32 = v0 + 1i32;
                (v0, v1) = (v21, v1);
                continue;
            } else {
                let mut v23: Rc<str> = Rc::<str>::from("--root");
                let mut v24: bool = v3 == v23 ;
                let mut v36: bool = if v24 {
                    true
                } else {
                    let mut v25: Rc<str> = Rc::<str>::from("--package");
                    let mut v26: bool = v3 == v25 ;
                    if v26 {
                        true
                    } else {
                        let mut v27: Rc<str> = Rc::<str>::from("--target-dir");
                        let mut v28: bool = v3 == v27 ;
                        if v28 {
                            true
                        } else {
                            let mut v29: Rc<str> = Rc::<str>::from("--compiler");
                            let mut v30: bool = v3 == v29 ;
                            if v30 {
                                true
                            } else {
                                let mut v31: Rc<str> = Rc::<str>::from("--timeout-ms");
                                let mut v32: bool = v3 == v31 ;
                                v32
                            }
                        }
                    }
                };
                if v36 {
                    let mut v37: i32 = v0 + 1i32;
                    let mut v38: bool = v37 < v1;
                    if v38 {
                        let mut v39: Rc<str> = std::env::args().nth(v37 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
                        let mut v40: Rc<str> = Rc::<str>::from("");
                        let mut v41: bool = v39 == v40 ;
                        if v41 {
                            return false;
                        } else {
                            let mut v42: Rc<str> = Rc::<str>::from("--");
                            let mut v43: bool = v39.starts_with(&*v42);
                            if v43 {
                                return false;
                            } else {
                                let mut v44: i32 = v0 + 2i32;
                                (v0, v1) = (v44, v1);
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
fn method1(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> bool {
    loop {
        let mut v3: bool = v0 == v1;
        if v3 {
            return false;
        } else {
            let mut v4: Rc<str> = std::env::args().nth(v0 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
            let mut v5: bool = v2 == v4 ;
            if v5 {
                return true;
            } else {
                let mut v6: Rc<str> = Rc::<str>::from("--root");
                let mut v7: bool = v4 == v6 ;
                let mut v19: bool = if v7 {
                    true
                } else {
                    let mut v8: Rc<str> = Rc::<str>::from("--package");
                    let mut v9: bool = v4 == v8 ;
                    if v9 {
                        true
                    } else {
                        let mut v10: Rc<str> = Rc::<str>::from("--target-dir");
                        let mut v11: bool = v4 == v10 ;
                        if v11 {
                            true
                        } else {
                            let mut v12: Rc<str> = Rc::<str>::from("--compiler");
                            let mut v13: bool = v4 == v12 ;
                            if v13 {
                                true
                            } else {
                                let mut v14: Rc<str> = Rc::<str>::from("--timeout-ms");
                                let mut v15: bool = v4 == v14 ;
                                v15
                            }
                        }
                    }
                };
                let mut v20: i32 = if v19 {
                    2i32
                } else {
                    1i32
                };
                let mut v21: i32 = v0 + v20;
                (v0, v1, v2) = (v21, v1, v2.clone());
                continue;
            }
        }
    }
}
fn method3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v3: Rc<str> = Rc::<str>::from("");
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v1, &*v3, &*v3].concat());
    v4.clone()
}
fn method2(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: bool) -> Rc<str> {
    loop {
        let mut v4: bool = v0 == v1;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: Rc<str> = std::env::args().nth(v0 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
            let mut v6: Rc<str> = Rc::<str>::from("--root");
            let mut v7: bool = v5 == v6 ;
            let mut v19: bool = if v7 {
                true
            } else {
                let mut v8: Rc<str> = Rc::<str>::from("--package");
                let mut v9: bool = v5 == v8 ;
                if v9 {
                    true
                } else {
                    let mut v10: Rc<str> = Rc::<str>::from("--target-dir");
                    let mut v11: bool = v5 == v10 ;
                    if v11 {
                        true
                    } else {
                        let mut v12: Rc<str> = Rc::<str>::from("--compiler");
                        let mut v13: bool = v5 == v12 ;
                        if v13 {
                            true
                        } else {
                            let mut v14: Rc<str> = Rc::<str>::from("--timeout-ms");
                            let mut v15: bool = v5 == v14 ;
                            v15
                        }
                    }
                }
            };
            let mut v20: i32 = if v19 {
                2i32
            } else {
                1i32
            };
            let mut v21: Rc<str> = Rc::<str>::from("--package");
            let mut v22: bool = v5 == v21 ;
            let mut v32: Rc<str> = if v22 {
                let mut v23: i32 = v0 + 1i32;
                let mut v24: Rc<str> = std::env::args().nth(v23 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
                if v3 {
                    let mut v25: Rc<str> = method3(v2.clone(), v21.clone());
                    method3(v25.clone(), v24.clone())
                } else {
                    let mut v27: Rc<str> = Rc::<str>::from("");
                    let mut v28: bool = v2 == v27 ;
                    if v28 {
                        v24.clone()
                    } else {
                        method3(v2.clone(), v24.clone())
                    }
                }
            } else {
                v2.clone()
            };
            let mut v33: i32 = v0 + v20;
            (v0, v1, v2, v3) = (v33, v1, v32.clone(), v3);
            continue;
        }
    }
}
fn method4(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v0 == v1;
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = std::env::args().nth(v0 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
            let mut v6: Rc<str> = Rc::<str>::from("--root");
            let mut v7: bool = v5 == v6 ;
            let mut v19: bool = if v7 {
                true
            } else {
                let mut v8: Rc<str> = Rc::<str>::from("--package");
                let mut v9: bool = v5 == v8 ;
                if v9 {
                    true
                } else {
                    let mut v10: Rc<str> = Rc::<str>::from("--target-dir");
                    let mut v11: bool = v5 == v10 ;
                    if v11 {
                        true
                    } else {
                        let mut v12: Rc<str> = Rc::<str>::from("--compiler");
                        let mut v13: bool = v5 == v12 ;
                        if v13 {
                            true
                        } else {
                            let mut v14: Rc<str> = Rc::<str>::from("--timeout-ms");
                            let mut v15: bool = v5 == v14 ;
                            v15
                        }
                    }
                }
            };
            let mut v20: i32 = if v19 {
                2i32
            } else {
                1i32
            };
            let mut v21: bool = v2 == v5 ;
            let mut v24: Rc<str> = if v21 {
                let mut v22: i32 = v0 + 1i32;
                let mut v23: Rc<str> = std::env::args().nth(v22 as usize).map(std::rc::Rc::<str>::from).unwrap_or_default();
                v23.clone()
            } else {
                v3.clone()
            };
            let mut v25: i32 = v0 + v20;
            (v0, v1, v2, v3) = (v25, v1, v2.clone(), v24.clone());
            continue;
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = Rc::<str>::from("");
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v2, &*v2].concat());
    v3.clone()
}
fn method7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let mut v3: Rc<str> = Rc::<str>::from("");
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v1, &*v3, &*v3].concat());
    v4.clone()
}
fn method6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: u64) -> i32 {
    let mut v4: Rc<str> = Rc::<str>::from("metadata");
    let mut v5: Rc<str> = Rc::<str>::from("--no-deps");
    let mut v6: Rc<str> = method7(v4.clone(), v5.clone());
    let mut v7: Rc<str> = Rc::<str>::from("--format-version");
    let mut v8: Rc<str> = method7(v6.clone(), v7.clone());
    let mut v9: Rc<str> = Rc::<str>::from("1");
    let mut v10: Rc<str> = method7(v8.clone(), v9.clone());
    let mut v11: Rc<str> = Rc::<str>::from("--locked");
    let mut v12: Rc<str> = method7(v10.clone(), v11.clone());
    let mut v15: Rc<str> = if v2 {
        let mut v13: Rc<str> = Rc::<str>::from("--offline");
        method7(v12.clone(), v13.clone())
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
            let mut v23: Rc<str> = Rc::<str>::from("Unknown selected Cargo package or ambiguous workspace name\n");
            v23.clone()
        } else {
            let mut v24: bool = v20 == -3i32 ;
            if v24 {
                let mut v25: Rc<str> = Rc::<str>::from("eoie test metadata must be a table\n");
                v25.clone()
            } else {
                let mut v26: bool = v20 == -4i32 ;
                if v26 {
                    let mut v27: Rc<str> = Rc::<str>::from("test-requires-cli must be a boolean\n");
                    v27.clone()
                } else {
                    let mut v28: Rc<str> = Rc::<str>::from("Cannot inspect selected Cargo test requirements\n");
                    v28.clone()
                }
            }
        };
        std::io::Write::write_all(&mut std::io::stdout(),v31.as_bytes()).unwrap();
        ()
    };
    v20
}
fn method9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
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
fn method11() -> Rc<str> {
    let mut v0: Rc<str> = Rc::<str>::from("");
    v0.clone()
}
fn method10() -> Rc<str> {
    method11()
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: u8) -> Rc<str> {
    let mut v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = method9(v0.clone(), v1, v3, v2);
    let mut v5: bool = v1 == v4 ;
    let mut v9: Rc<str> = if v5 {
        method10()
    } else {
        let mut v7: i32 = v4 - 1i32 ;
        let mut v8: Rc<str> = string_slice(&v0.clone(), v1 as i64, v7 as i64);
        v8.clone()
    };
    let mut v10: Rc<str> = predicate_lex_domain::eoie_predicate_encode(&*v9);
    let mut v11: bool = v4 == v3 ;
    if v11 {
        v10.clone()
    } else {
        let mut v12: Rc<str> = Rc::<str>::from(" ");
        let mut v13: i32 = v4 + 1i32 ;
        let mut v14: Rc<str> = method8(v0.clone(), v13, v2);
        let mut v15: Rc<str> = Rc::<str>::from("");
        let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v12, &*v14, &*v15, &*v15].concat());
        v16.clone()
    }
}
fn spiral_main() -> i32 {
    let mut v0: i32 = std::env::args().count() as i32;
    let mut v1: i32 = 1i32;
    let mut v2: bool = method0(v1, v0);
    if v2 {
        let mut v3: i32 = 1i32;
        let mut v4: i32 = std::env::args().count() as i32;
        let mut v5: Rc<str> = Rc::<str>::from("--help");
        let mut v6: bool = method1(v3, v4, v5.clone());
        if v6 {
            let mut v7: Rc<str> = Rc::<str>::from("eoie-dev [--test] [--skip-release] [--package NAME] [--offline] [--root PATH] [--target-dir PATH] [--compiler-contracts] [--compiler PATH] [--timeout-ms N] [--dry-run]\nBuilds a candidate; installation remains a separately validated operation.\n");
            std::io::Write::write_all(&mut std::io::stdout(),v7.as_bytes()).unwrap();
            0i32
        } else {
            let mut v8: i32 = 1i32;
            let mut v9: i32 = std::env::args().count() as i32;
            let mut v10: Rc<str> = Rc::<str>::from("--test");
            let mut v11: bool = method1(v8, v9, v10.clone());
            let mut v12: i32 = 1i32;
            let mut v13: i32 = std::env::args().count() as i32;
            let mut v14: Rc<str> = Rc::<str>::from("--skip-release");
            let mut v15: bool = method1(v12, v13, v14.clone());
            let mut v16: i32 = 1i32;
            let mut v17: i32 = std::env::args().count() as i32;
            let mut v18: Rc<str> = Rc::<str>::from("--compiler-contracts");
            let mut v19: bool = method1(v16, v17, v18.clone());
            let mut v20: i32 = 1i32;
            let mut v21: Rc<str> = Rc::<str>::from("");
            let mut v22: bool = false;
            let mut v23: Rc<str> = method2(v20, v0, v21.clone(), v22);
            let mut v24: i32 = 1i32;
            let mut v25: i32 = std::env::args().count() as i32;
            let mut v26: Rc<str> = Rc::<str>::from("--compiler");
            let mut v27: Rc<str> = method4(v24, v25, v26.clone(), v21.clone());
            let mut v28: bool = v27 == v21 ;
            let mut v31: Rc<str> = if v28 {
                let mut v29: Rc<str> = std::env::var("EOIE_SPIRAL_COMPILE").map(std::rc::Rc::<str>::from).unwrap_or_default();
                v29.clone()
            } else {
                let mut v30: Rc<str> = std::fs::canonicalize(&*v27).ok().filter(|p| p.is_file()).and_then(|p| p.to_str().map(std::rc::Rc::<str>::from)).unwrap_or_default();
                v30.clone()
            };
            let mut v32: i32 = 1i32;
            let mut v33: i32 = std::env::args().count() as i32;
            let mut v34: Rc<str> = Rc::<str>::from("--timeout-ms");
            let mut v35: Rc<str> = Rc::<str>::from("900000");
            let mut v36: Rc<str> = method4(v32, v33, v34.clone(), v35.clone());
            let mut v37: u64 = v36.parse::<u64>().unwrap_or(0);
            let mut v38: i32 = 1i32;
            let mut v39: i32 = std::env::args().count() as i32;
            let mut v40: Rc<str> = Rc::<str>::from("--root");
            let mut v41: Rc<str> = Rc::<str>::from(".");
            let mut v42: Rc<str> = method4(v38, v39, v40.clone(), v41.clone());
            let mut v43: Rc<str> = std::path::absolute(&*v42).ok().and_then(|p| p.to_str().map(std::rc::Rc::<str>::from)).unwrap_or_default();
            let mut v44: Rc<str> = Rc::<str>::from("/src/target");
            let mut v45: Rc<str> = method5(v43.clone(), v44.clone());
            let mut v46: i32 = 1i32;
            let mut v47: i32 = std::env::args().count() as i32;
            let mut v48: Rc<str> = Rc::<str>::from("--target-dir");
            let mut v49: Rc<str> = method4(v46, v47, v48.clone(), v45.clone());
            let mut v50: Rc<str> = std::path::absolute(&*v49).ok().and_then(|p| p.to_str().map(std::rc::Rc::<str>::from)).unwrap_or_default();
            let mut v51: i32 = 1i32;
            let mut v52: i32 = std::env::args().count() as i32;
            let mut v53: Rc<str> = Rc::<str>::from("--dry-run");
            let mut v54: bool = method1(v51, v52, v53.clone());
            let mut v55: bool = v11 == false;
            let mut v60: bool = if v55 {
                let mut v56: bool = v15 || v19;
                if v56 {
                    true
                } else {
                    let mut v57: bool = v23 == v21 ;
                    let mut v58: bool = v57 == false;
                    v58
                }
            } else {
                false
            };
            if v60 {
                let mut v61: Rc<str> = Rc::<str>::from("--skip-release, --package and --compiler-contracts require --test");
                let mut v62: Rc<str> = Rc::<str>::from("\n");
                let mut v63: Rc<str> = method5(v61.clone(), v62.clone());
                std::io::Write::write_all(&mut std::io::stdout(),v63.as_bytes()).unwrap();
                2i32
            } else {
                let mut v64: bool = v27 == v21 ;
                let mut v65: bool = v64 == false;
                let mut v67: bool = if v65 {
                    let mut v66: bool = v31 == v21 ;
                    v66
                } else {
                    false
                };
                if v67 {
                    let mut v68: Rc<str> = Rc::<str>::from("--compiler must identify an existing file");
                    let mut v69: Rc<str> = Rc::<str>::from("\n");
                    let mut v70: Rc<str> = method5(v68.clone(), v69.clone());
                    std::io::Write::write_all(&mut std::io::stdout(),v70.as_bytes()).unwrap();
                    2i32
                } else {
                    let mut v72: bool = if v19 {
                        let mut v71: bool = v31 == v21 ;
                        v71
                    } else {
                        false
                    };
                    if v72 {
                        let mut v73: Rc<str> = Rc::<str>::from("--compiler-contracts requires --compiler or EOIE_SPIRAL_COMPILE");
                        let mut v74: Rc<str> = Rc::<str>::from("\n");
                        let mut v75: Rc<str> = method5(v73.clone(), v74.clone());
                        std::io::Write::write_all(&mut std::io::stdout(),v75.as_bytes()).unwrap();
                        2i32
                    } else {
                        let mut v76: bool = v37 == 0u64;
                        let mut v78: bool = if v76 {
                            true
                        } else {
                            let mut v77: bool = v37 > 3600000u64;
                            v77
                        };
                        if v78 {
                            let mut v79: Rc<str> = Rc::<str>::from("--timeout-ms must be between 1 and 3600000");
                            let mut v80: Rc<str> = Rc::<str>::from("\n");
                            let mut v81: Rc<str> = method5(v79.clone(), v80.clone());
                            std::io::Write::write_all(&mut std::io::stdout(),v81.as_bytes()).unwrap();
                            2i32
                        } else {
                            let mut v82: bool = std::path::Path::new(&*v43).join("src/Cargo.toml").is_file();
                            let mut v83: bool = v82 == false;
                            if v83 {
                                let mut v84: Rc<str> = Rc::<str>::from("--root must contain src/Cargo.toml");
                                let mut v85: Rc<str> = Rc::<str>::from("\n");
                                let mut v86: Rc<str> = method5(v84.clone(), v85.clone());
                                std::io::Write::write_all(&mut std::io::stdout(),v86.as_bytes()).unwrap();
                                2i32
                            } else {
                                let mut v87: bool = v54 == false;
                                let mut v89: bool = if v87 {
                                    let mut v88: bool = std::env::current_exe().ok().and_then(|p| p.parent().and_then(std::path::Path::parent).map(std::path::Path::to_owned)).and_then(|p| std::fs::canonicalize(p).ok()).zip(std::fs::canonicalize(&*v50).ok()).is_some_and(|(running,target)| running==target);
                                    v88
                                } else {
                                    false
                                };
                                if v89 {
                                    let mut v90: Rc<str> = Rc::<str>::from("The workflow executable is running from the build target; bootstrap eoie-dev with a separate --target-dir");
                                    let mut v91: Rc<str> = Rc::<str>::from("\n");
                                    let mut v92: Rc<str> = method5(v90.clone(), v91.clone());
                                    std::io::Write::write_all(&mut std::io::stdout(),v92.as_bytes()).unwrap();
                                    2i32
                                } else {
                                    let mut v93: i32 = 1i32;
                                    let mut v94: i32 = std::env::args().count() as i32;
                                    let mut v95: Rc<str> = Rc::<str>::from("--offline");
                                    let mut v96: bool = method1(v93, v94, v95.clone());
                                    let mut v97: Rc<str> = Rc::<str>::from("--locked");
                                    let mut v98: Rc<str> = method3(v97.clone(), v48.clone());
                                    let mut v99: Rc<str> = method3(v98.clone(), v50.clone());
                                    let mut v101: Rc<str> = if v96 {
                                        method3(v99.clone(), v95.clone())
                                    } else {
                                        v99.clone()
                                    };
                                    let mut v102: bool = v23 == v21 ;
                                    let mut v104: i32 = if v102 {
                                        1i32
                                    } else {
                                        method6(v43.clone(), v23.clone(), v96, v37)
                                    };
                                    let mut v105: bool = v104 < 0i32;
                                    if v105 {
                                        2i32
                                    } else {
                                        let mut v107: bool = if v11 {
                                            let mut v106: bool = v104 == 1i32;
                                            v106
                                        } else {
                                            false
                                        };
                                        let mut v123: i32 = if v107 {
                                            let mut v108: Rc<str> = Rc::<str>::from("build");
                                            let mut v109: Rc<str> = method3(v108.clone(), v101.clone());
                                            let mut v110: Rc<str> = Rc::<str>::from("--package");
                                            let mut v111: Rc<str> = method3(v109.clone(), v110.clone());
                                            let mut v112: Rc<str> = Rc::<str>::from("eoie-cli");
                                            let mut v113: Rc<str> = method3(v111.clone(), v112.clone());
                                            if v54 {
                                                let mut v114: i32 = 0i32;
                                                let mut v115: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                                                let mut v116: u8 = v115.clone().as_bytes()[0i32 as usize];
                                                let mut v117: Rc<str> = method8(v113.clone(), v114, v116);
                                                let mut v118: Rc<str> = Rc::<str>::from("cargo ");
                                                let mut v119: Rc<str> = Rc::<str>::from("\n");
                                                let mut v120: Rc<str> = std::rc::Rc::<str>::from([&*v118, &*v117, &*v119, &*v21, &*v21].concat());
                                                std::io::Write::write_all(&mut std::io::stdout(),v120.as_bytes()).unwrap();
                                                0i32
                                            } else {
                                                let mut v121: i32 = { let mut command=std::process::Command::new("cargo"); command.current_dir(std::path::Path::new(&*v43).join("src")); command.args(v113.split(char::from(0u8))); if v31.len() > 0 { command.env("EOIE_SPIRAL_COMPILE", &*v31); } if std::env::var_os("CARGO_BUILD_JOBS").is_none() { command.env("CARGO_BUILD_JOBS", "2"); } if std::env::var_os("RUST_TEST_THREADS").is_none() { command.env("RUST_TEST_THREADS", "1"); } match eoie_process::run_bounded(&mut command,v37) { Ok(status) => status.code().unwrap_or(1), Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); 2 } } };
                                                v121
                                            }
                                        } else {
                                            0i32
                                        };
                                        let mut v124: bool = v123 == 0i32;
                                        if v124 {
                                            let mut v148: i32 = if v11 {
                                                let mut v125: Rc<str> = Rc::<str>::from("test");
                                                let mut v126: Rc<str> = method3(v125.clone(), v101.clone());
                                                let mut v127: bool = v23 == v21 ;
                                                let mut v133: Rc<str> = if v127 {
                                                    let mut v128: Rc<str> = Rc::<str>::from("--workspace");
                                                    method3(v126.clone(), v128.clone())
                                                } else {
                                                    let mut v130: i32 = 1i32;
                                                    let mut v131: bool = true;
                                                    method2(v130, v0, v126.clone(), v131)
                                                };
                                                let mut v138: Rc<str> = if v19 {
                                                    let mut v134: Rc<str> = Rc::<str>::from("--");
                                                    let mut v135: Rc<str> = method3(v133.clone(), v134.clone());
                                                    let mut v136: Rc<str> = Rc::<str>::from("--include-ignored");
                                                    method3(v135.clone(), v136.clone())
                                                } else {
                                                    v133.clone()
                                                };
                                                if v54 {
                                                    let mut v139: i32 = 0i32;
                                                    let mut v140: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                                                    let mut v141: u8 = v140.clone().as_bytes()[0i32 as usize];
                                                    let mut v142: Rc<str> = method8(v138.clone(), v139, v141);
                                                    let mut v143: Rc<str> = Rc::<str>::from("cargo ");
                                                    let mut v144: Rc<str> = Rc::<str>::from("\n");
                                                    let mut v145: Rc<str> = std::rc::Rc::<str>::from([&*v143, &*v142, &*v144, &*v21, &*v21].concat());
                                                    std::io::Write::write_all(&mut std::io::stdout(),v145.as_bytes()).unwrap();
                                                    0i32
                                                } else {
                                                    let mut v146: i32 = { let mut command=std::process::Command::new("cargo"); command.current_dir(std::path::Path::new(&*v43).join("src")); command.args(v138.split(char::from(0u8))); if v31.len() > 0 { command.env("EOIE_SPIRAL_COMPILE", &*v31); } if std::env::var_os("CARGO_BUILD_JOBS").is_none() { command.env("CARGO_BUILD_JOBS", "2"); } if std::env::var_os("RUST_TEST_THREADS").is_none() { command.env("RUST_TEST_THREADS", "1"); } match eoie_process::run_bounded(&mut command,v37) { Ok(status) => status.code().unwrap_or(1), Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); 2 } } };
                                                    v146
                                                }
                                            } else {
                                                0i32
                                            };
                                            let mut v149: bool = v148 == 0i32;
                                            if v149 {
                                                if v15 {
                                                    0i32
                                                } else {
                                                    let mut v150: Rc<str> = Rc::<str>::from("build");
                                                    let mut v151: Rc<str> = method3(v150.clone(), v101.clone());
                                                    let mut v152: Rc<str> = Rc::<str>::from("--release");
                                                    let mut v153: Rc<str> = method3(v151.clone(), v152.clone());
                                                    let mut v154: Rc<str> = Rc::<str>::from("--package");
                                                    let mut v155: Rc<str> = method3(v153.clone(), v154.clone());
                                                    let mut v156: Rc<str> = Rc::<str>::from("eoie-cli");
                                                    let mut v157: Rc<str> = method3(v155.clone(), v156.clone());
                                                    if v54 {
                                                        let mut v158: i32 = 0i32;
                                                        let mut v159: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
                                                        let mut v160: u8 = v159.clone().as_bytes()[0i32 as usize];
                                                        let mut v161: Rc<str> = method8(v157.clone(), v158, v160);
                                                        let mut v162: Rc<str> = Rc::<str>::from("cargo ");
                                                        let mut v163: Rc<str> = Rc::<str>::from("\n");
                                                        let mut v164: Rc<str> = std::rc::Rc::<str>::from([&*v162, &*v161, &*v163, &*v21, &*v21].concat());
                                                        std::io::Write::write_all(&mut std::io::stdout(),v164.as_bytes()).unwrap();
                                                        0i32
                                                    } else {
                                                        let mut v165: i32 = { let mut command=std::process::Command::new("cargo"); command.current_dir(std::path::Path::new(&*v43).join("src")); command.args(v157.split(char::from(0u8))); if v31.len() > 0 { command.env("EOIE_SPIRAL_COMPILE", &*v31); } if std::env::var_os("CARGO_BUILD_JOBS").is_none() { command.env("CARGO_BUILD_JOBS", "2"); } if std::env::var_os("RUST_TEST_THREADS").is_none() { command.env("RUST_TEST_THREADS", "1"); } match eoie_process::run_bounded(&mut command,v37) { Ok(status) => status.code().unwrap_or(1), Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); 2 } } };
                                                        v165
                                                    }
                                                }
                                            } else {
                                                v148
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
        let mut v178: Rc<str> = Rc::<str>::from("Invalid option or missing value; use --help");
        let mut v179: Rc<str> = Rc::<str>::from("\n");
        let mut v180: Rc<str> = method5(v178.clone(), v179.clone());
        std::io::Write::write_all(&mut std::io::stdout(),v180.as_bytes()).unwrap();
        2i32
    }
}
fn main() {
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(main.join().unwrap());
}
