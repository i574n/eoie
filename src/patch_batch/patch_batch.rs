#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn spiral_main() -> i32 {
    let mut v0: Rc<str> = std::env::temp_dir().join("eoie patch batch ü ".to_owned() + &std::process::id().to_string() + "-" + &std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos().to_string()).display().to_string().into();
    let mut v1: bool = std::fs::create_dir(&*v0).is_ok();
    if v1 {
        let mut v2: Rc<str> = Rc::<str>::from("target-a.txt");
        let mut v3: Rc<str> = std::path::Path::new(&*v0).join(&*v2).display().to_string().into();
        let mut v4: Rc<str> = Rc::<str>::from("stage-a.txt");
        let mut v5: Rc<str> = std::path::Path::new(&*v0).join(&*v4).display().to_string().into();
        let mut v6: Rc<str> = Rc::<str>::from("backup-a.txt");
        let mut v7: Rc<str> = std::path::Path::new(&*v0).join(&*v6).display().to_string().into();
        let mut v8: Rc<str> = Rc::<str>::from("target-b.txt");
        let mut v9: Rc<str> = std::path::Path::new(&*v0).join(&*v8).display().to_string().into();
        let mut v10: Rc<str> = Rc::<str>::from("stage-b.txt");
        let mut v11: Rc<str> = std::path::Path::new(&*v0).join(&*v10).display().to_string().into();
        let mut v12: Rc<str> = Rc::<str>::from("backup-b.txt");
        let mut v13: Rc<str> = std::path::Path::new(&*v0).join(&*v12).display().to_string().into();
        let mut v14: bool = std::fs::remove_file(&*v3).is_ok() || !std::path::Path::new(&*v3).exists();
        let mut v16: bool = if v14 {
            let mut v15: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
            v15
        } else {
            false
        };
        let mut v18: bool = if v16 {
            let mut v17: bool = std::fs::remove_file(&*v7).is_ok() || !std::path::Path::new(&*v7).exists();
            v17
        } else {
            false
        };
        let mut v20: bool = if v18 {
            let mut v19: bool = std::fs::remove_file(&*v9).is_ok() || !std::path::Path::new(&*v9).exists();
            v19
        } else {
            false
        };
        let mut v22: bool = if v20 {
            let mut v21: bool = std::fs::remove_file(&*v11).is_ok() || !std::path::Path::new(&*v11).exists();
            v21
        } else {
            false
        };
        let mut v24: bool = if v22 {
            let mut v23: bool = std::fs::remove_file(&*v13).is_ok() || !std::path::Path::new(&*v13).exists();
            v23
        } else {
            false
        };
        let mut v68: bool = if v24 {
            let mut v25: Rc<str> = Rc::<str>::from("original-a");
            let mut v26: bool = std::fs::write(&*v3, &*v25).is_ok();
            let mut v29: bool = if v26 {
                let mut v27: Rc<str> = Rc::<str>::from("original-b");
                let mut v28: bool = std::fs::write(&*v9, &*v27).is_ok();
                v28
            } else {
                false
            };
            let mut v31: bool = if v29 {
                let mut v30: bool = std::fs::write(&*v7, &*v25).is_ok();
                v30
            } else {
                false
            };
            let mut v34: bool = if v31 {
                let mut v32: Rc<str> = Rc::<str>::from("original-b");
                let mut v33: bool = std::fs::write(&*v13, &*v32).is_ok();
                v33
            } else {
                false
            };
            let mut v37: bool = if v34 {
                let mut v35: Rc<str> = Rc::<str>::from("changed-a");
                let mut v36: bool = std::fs::write(&*v5, &*v35).is_ok();
                v36
            } else {
                false
            };
            let mut v40: bool = if v37 {
                let mut v38: Rc<str> = Rc::<str>::from("changed-b");
                let mut v39: bool = std::fs::write(&*v11, &*v38).is_ok();
                v39
            } else {
                false
            };
            let mut v42: bool = if v40 {
                let mut v41: bool = std::fs::rename(&*v5, &*v3).is_ok();
                v41
            } else {
                false
            };
            let mut v45: bool = if v42 {
                let mut v43: Rc<str> = Rc::<str>::from("changed-a");
                let mut v44: bool = std::fs::read_to_string(&*v3).is_ok_and(|value| value == *v43);
                v44
            } else {
                false
            };
            let mut v48: bool = if v45 {
                let mut v46: Rc<str> = Rc::<str>::from("original-b");
                let mut v47: bool = std::fs::read_to_string(&*v9).is_ok_and(|value| value == *v46);
                v47
            } else {
                false
            };
            if v48 {
                let mut v49: bool = std::fs::rename(&*v7, &*v3).is_ok();
                let mut v51: bool = if v49 {
                    let mut v50: bool = std::fs::remove_file(&*v11).is_ok() || !std::path::Path::new(&*v11).exists();
                    v50
                } else {
                    false
                };
                let mut v53: bool = if v51 {
                    let mut v52: bool = std::fs::remove_file(&*v13).is_ok() || !std::path::Path::new(&*v13).exists();
                    v52
                } else {
                    false
                };
                let mut v58: bool = if v53 {
                    let mut v54: bool = std::fs::read_to_string(&*v3).is_ok_and(|value| value == *v25);
                    if v54 {
                        let mut v55: Rc<str> = Rc::<str>::from("original-b");
                        let mut v56: bool = std::fs::read_to_string(&*v9).is_ok_and(|value| value == *v55);
                        v56
                    } else {
                        false
                    }
                } else {
                    false
                };
                if v58 {
                    let mut v59: bool = !std::path::Path::new(&*v5).is_file();
                    let mut v61: bool = if v59 {
                        let mut v60: bool = !std::path::Path::new(&*v11).is_file();
                        v60
                    } else {
                        false
                    };
                    let mut v63: bool = if v61 {
                        let mut v62: bool = !std::path::Path::new(&*v7).is_file();
                        v62
                    } else {
                        false
                    };
                    if v63 {
                        let mut v64: bool = !std::path::Path::new(&*v13).is_file();
                        v64
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
        };
        let mut v69: Rc<str> = Rc::<str>::from("missing.txt");
        let mut v70: Rc<str> = std::path::Path::new(&*v0).join(&*v69).display().to_string().into();
        let mut v71: Rc<str> = Rc::<str>::from("missing-promoted.txt");
        let mut v72: Rc<str> = std::path::Path::new(&*v0).join(&*v71).display().to_string().into();
        let mut v73: bool = std::fs::rename(&*v72, &*v70).is_ok();
        let mut v74: bool = v73 == false;
        let mut v75: bool = std::fs::remove_file(&*v3).is_ok() || !std::path::Path::new(&*v3).exists();
        let mut v77: bool = if v75 {
            let mut v76: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
            v76
        } else {
            false
        };
        let mut v79: bool = if v77 {
            let mut v78: bool = std::fs::remove_file(&*v7).is_ok() || !std::path::Path::new(&*v7).exists();
            v78
        } else {
            false
        };
        let mut v81: bool = if v79 {
            let mut v80: bool = std::fs::remove_file(&*v9).is_ok() || !std::path::Path::new(&*v9).exists();
            v80
        } else {
            false
        };
        let mut v83: bool = if v81 {
            let mut v82: bool = std::fs::remove_file(&*v11).is_ok() || !std::path::Path::new(&*v11).exists();
            v82
        } else {
            false
        };
        let mut v85: bool = if v83 {
            let mut v84: bool = std::fs::remove_file(&*v13).is_ok() || !std::path::Path::new(&*v13).exists();
            v84
        } else {
            false
        };
        let mut v86: bool = std::fs::remove_file(&*v3).is_ok() || !std::path::Path::new(&*v3).exists();
        let mut v88: bool = if v86 {
            let mut v87: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
            v87
        } else {
            false
        };
        let mut v90: bool = if v88 {
            let mut v89: bool = std::fs::remove_file(&*v7).is_ok() || !std::path::Path::new(&*v7).exists();
            v89
        } else {
            false
        };
        let mut v92: bool = if v90 {
            let mut v91: bool = std::fs::remove_file(&*v9).is_ok() || !std::path::Path::new(&*v9).exists();
            v91
        } else {
            false
        };
        let mut v94: bool = if v92 {
            let mut v93: bool = std::fs::remove_file(&*v11).is_ok() || !std::path::Path::new(&*v11).exists();
            v93
        } else {
            false
        };
        let mut v96: bool = if v94 {
            let mut v95: bool = std::fs::remove_file(&*v13).is_ok() || !std::path::Path::new(&*v13).exists();
            v95
        } else {
            false
        };
        let mut v97: bool = std::fs::remove_dir(&*v0).is_ok();
        let mut v98: bool = v68 && v74;
        let mut v99: bool = v98 && v85;
        let mut v100: bool = v99 && v96;
        let mut v101: bool = v100 && v97;
        if v101 {
            0i32
        } else {
            1i32
        }
    } else {
        10i32
    }
}
fn main() {
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(main.join().unwrap());
}
