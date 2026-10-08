#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_0(Rc<str>),
    US0_1(Rc<str>),
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_0(..) => 0,
            US0::US0_1(..) => 1,
        }
    }
}
#[derive(Clone)]
enum US2 {
    US2_0,
    US2_1,
}
impl US2 {
    fn tag(&self) -> i32 {
        match self {
            US2::US2_0 => 0,
            US2::US2_1 => 1,
        }
    }
}
#[derive(Clone)]
enum US1 {
    US1_0(US2),
    US1_1,
}
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_0(..) => 0,
            US1::US1_1 => 1,
        }
    }
}
#[derive(Clone)]
enum US4 {
    US4_0,
    US4_1(u64),
    US4_2,
}
impl US4 {
    fn tag(&self) -> i32 {
        match self {
            US4::US4_0 => 0,
            US4::US4_1(..) => 1,
            US4::US4_2 => 2,
        }
    }
}
#[derive(Clone)]
enum US3 {
    US3_0(US4),
    US3_1,
}
impl US3 {
    fn tag(&self) -> i32 {
        match self {
            US3::US3_0(..) => 0,
            US3::US3_1 => 1,
        }
    }
}
#[derive(Clone)]
enum US5 {
    US5_0,
    US5_1,
    US5_2,
    US5_3,
}
impl US5 {
    fn tag(&self) -> i32 {
        match self {
            US5::US5_0 => 0,
            US5::US5_1 => 1,
            US5::US5_2 => 2,
            US5::US5_3 => 3,
        }
    }
}
fn method1() -> US0 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1 == v2 ;
    let mut v12: US1 = if v3 {
        let mut v4: US2 = US2::US2_0;
        US1::US1_0(v4.clone())
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v1 == v6 ;
        if v7 {
            let mut v8: US2 = US2::US2_1;
            US1::US1_0(v8.clone())
        } else {
            US1::US1_1
        }
    };
    let mut v13: i32 = std::env::args().skip(2).count() as i32;
    let mut v14: bool = 6i32 == v13;
    let mut v41: US3 = if v14 {
        let mut v15: US4 = US4::US4_0;
        US3::US3_0(v15.clone())
    } else {
        let mut v17: bool = 7i32 == v13;
        if v17 {
            let mut v18: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 6i32 as usize).unwrap_or_default());
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--all"); } LIT.with(|lit| lit.clone()) };
            let mut v20: bool = v18 == v19 ;
            if v20 {
                let mut v21: US4 = US4::US4_2;
                US3::US3_0(v21.clone())
            } else {
                US3::US3_1
            }
        } else {
            let mut v25: bool = 8i32 == v13;
            if v25 {
                let mut v26: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 7i32 as usize).unwrap_or_default());
                let mut v27: u64 = v26.parse::<u64>().unwrap_or(0);
                let mut v28: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 6i32 as usize).unwrap_or_default());
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--expect"); } LIT.with(|lit| lit.clone()) };
                let mut v30: bool = v28 == v29 ;
                if v30 {
                    let mut v31: bool = 0 < v27;
                    if v31 {
                        let mut v32: US4 = US4::US4_1(v27);
                        US3::US3_0(v32.clone())
                    } else {
                        US3::US3_1
                    }
                } else {
                    US3::US3_1
                }
            } else {
                US3::US3_1
            }
        }
    };
    match &v12 {
        US1::US1_1 => {
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace mode must be check or apply"); } LIT.with(|lit| lit.clone()) };
            US0::US0_1(v42.clone())
        }
        _ => {
            match &v41 {
                US3::US3_1 => {
                    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace expects <check|apply> <root> <relative> <old> <new> [--expect N | --all]"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_1(v44.clone())
                }
                _ => {
                    match &v12 {
                        US1::US1_0(v46) => {
                            let mut v46: US2 = v46.clone();
                            match &v41 {
                                US3::US3_0(v47) => {
                                    let mut v47: US4 = v47.clone();
                                    let mut v48: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 4i32 as usize).unwrap_or_default());
                                    let mut v49: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 5i32 as usize).unwrap_or_default());
                                    let mut v50: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
                                    let mut v51: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 3i32 as usize).unwrap_or_default());
                                    let mut v52: Rc<str> = eoie_rust_std_fs::rooted_path(std::path::Path::new(&*v50), &*v51).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v54: bool = v52 == v53 ;
                                    if v54 {
                                        let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace target escapes the root"); } LIT.with(|lit| lit.clone()) };
                                        US0::US0_1(v55.clone())
                                    } else {
                                        let mut v57: bool = v48 == v53 ;
                                        if v57 {
                                            let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace old text is empty"); } LIT.with(|lit| lit.clone()) };
                                            US0::US0_1(v58.clone())
                                        } else {
                                            let mut v60: bool = v48 == v49 ;
                                            if v60 {
                                                let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace old and new text are identical"); } LIT.with(|lit| lit.clone()) };
                                                US0::US0_1(v61.clone())
                                            } else {
                                                let mut v63: bool = std::path::Path::new(&*v52).is_file();
                                                if v63 {
                                                    let mut v64: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v52).unwrap_or_default());
                                                    let mut v65: u64 = v64.match_indices(&*v48).count() as u64;
                                                    let mut v72: bool = match &v47 {
                                                        US4::US4_2 => {
                                                            let mut v69: bool = 0 < v65;
                                                            v69
                                                        }
                                                        US4::US4_1(v67) => {
                                                            let mut v67: u64 = *v67;
                                                            let mut v68: bool = v65 == v67;
                                                            v68
                                                        }
                                                        US4::US4_0 => {
                                                            let mut v66: bool = v65 == 1u64;
                                                            v66
                                                        }
                                                    };
                                                    if v72 {
                                                        let mut v75: Rc<str> = match &v47 {
                                                            US4::US4_2 => {
                                                                let mut v73: Rc<str> = std::rc::Rc::<str>::from(v64.replace(&*v48, &*v49));
                                                                v73.clone()
                                                            }
                                                            _ => {
                                                                let mut v74: Rc<str> = std::rc::Rc::<str>::from(v64.replacen(&*v48, &*v49, v65 as usize));
                                                                v74.clone()
                                                            }
                                                        };
                                                        match &v46 {
                                                            US2::US2_1 => {
                                                                let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                                                                let mut v104: Rc<str> = std::rc::Rc::<str>::from([&*v52, &*v103, &*v53, &*v53, &*v53].concat());
                                                                let mut v105: bool = std::fs::write(&*v104, &*v75).is_ok();
                                                                let mut v114: US5 = if v105 {
                                                                    let mut v106: bool = std::fs::rename(&*v104, &*v52).is_ok();
                                                                    if v106 {
                                                                        let mut v107: bool = std::fs::read_to_string(&*v52).is_ok_and(|value| value == *v75);
                                                                        if v107 {
                                                                            US5::US5_0
                                                                        } else {
                                                                            US5::US5_3
                                                                        }
                                                                    } else {
                                                                        US5::US5_2
                                                                    }
                                                                } else {
                                                                    US5::US5_1
                                                                };
                                                                match &v114 {
                                                                    US5::US5_0 => {
                                                                        let mut v115: u64 = v64.len() as u64;
                                                                        let mut v116: u64 = v75.len() as u64;
                                                                        let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy text-replace ok mode="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matches="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v121: Rc<str> = std::rc::Rc::<str>::from([&*v117, &*v118, &*v119, &*v52, &*v120].concat());
                                                                        let mut v122: Rc<str> = std::rc::Rc::<str>::from((v65).to_string());
                                                                        let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expected="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v130: Rc<str> = match &v47 {
                                                                            US4::US4_2 => {
                                                                                let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                                v127.clone()
                                                                            }
                                                                            US4::US4_1(v125) => {
                                                                                let mut v125: u64 = *v125;
                                                                                let mut v126: Rc<str> = std::rc::Rc::<str>::from((v125).to_string());
                                                                                v126.clone()
                                                                            }
                                                                            US4::US4_0 => {
                                                                                let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                                v124.clone()
                                                                            }
                                                                        };
                                                                        let mut v131: Rc<str> = std::rc::Rc::<str>::from([&*v122, &*v123, &*v130, &*v53, &*v53].concat());
                                                                        let mut v132: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                                                        let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v134: Rc<str> = std::rc::Rc::<str>::from((v115).to_string());
                                                                        let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v136: Rc<str> = std::rc::Rc::<str>::from((v116).to_string());
                                                                        let mut v137: Rc<str> = std::rc::Rc::<str>::from([&*v133, &*v134, &*v135, &*v136, &*v53].concat());
                                                                        let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                                        let mut v139: Rc<str> = std::rc::Rc::<str>::from((v132).to_string());
                                                                        let mut v140: Rc<str> = std::rc::Rc::<str>::from([&*v137, &*v138, &*v139, &*v53, &*v53].concat());
                                                                        let mut v141: Rc<str> = std::rc::Rc::<str>::from([&*v121, &*v131, &*v140, &*v53, &*v53].concat());
                                                                        US0::US0_0(v141.clone())
                                                                    }
                                                                    _ => {
                                                                        let mut v148: Rc<str> = match &v114 {
                                                                            US5::US5_3 => {
                                                                                let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                                v145.clone()
                                                                            }
                                                                            US5::US5_2 => {
                                                                                let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                                v144.clone()
                                                                            }
                                                                            US5::US5_1 => {
                                                                                let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                                v143.clone()
                                                                            }
                                                                            _ => unreachable!(),
                                                                        };
                                                                        US0::US0_1(v148.clone())
                                                                    }
                                                                }
                                                            }
                                                            US2::US2_0 => {
                                                                let mut v76: u64 = v64.len() as u64;
                                                                let mut v77: u64 = v75.len() as u64;
                                                                let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie proxy text-replace ok mode="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matches="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v81: Rc<str> = std::rc::Rc::<str>::from([&*v78, &*v2, &*v79, &*v52, &*v80].concat());
                                                                let mut v82: Rc<str> = std::rc::Rc::<str>::from((v65).to_string());
                                                                let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expected="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v90: Rc<str> = match &v47 {
                                                                    US4::US4_2 => {
                                                                        let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                        v87.clone()
                                                                    }
                                                                    US4::US4_1(v85) => {
                                                                        let mut v85: u64 = *v85;
                                                                        let mut v86: Rc<str> = std::rc::Rc::<str>::from((v85).to_string());
                                                                        v86.clone()
                                                                    }
                                                                    US4::US4_0 => {
                                                                        let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                        v84.clone()
                                                                    }
                                                                };
                                                                let mut v91: Rc<str> = std::rc::Rc::<str>::from([&*v82, &*v83, &*v90, &*v53, &*v53].concat());
                                                                let mut v92: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                                                let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v94: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                                                                let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v96: Rc<str> = std::rc::Rc::<str>::from((v77).to_string());
                                                                let mut v97: Rc<str> = std::rc::Rc::<str>::from([&*v93, &*v94, &*v95, &*v96, &*v53].concat());
                                                                let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                                let mut v99: Rc<str> = std::rc::Rc::<str>::from((v92).to_string());
                                                                let mut v100: Rc<str> = std::rc::Rc::<str>::from([&*v97, &*v98, &*v99, &*v53, &*v53].concat());
                                                                let mut v101: Rc<str> = std::rc::Rc::<str>::from([&*v81, &*v91, &*v100, &*v53, &*v53].concat());
                                                                US0::US0_0(v101.clone())
                                                            }
                                                        }
                                                    } else {
                                                        let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace expected "); } LIT.with(|lit| lit.clone()) };
                                                        let mut v160: Rc<str> = match &v47 {
                                                            US4::US4_2 => {
                                                                let mut v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                v157.clone()
                                                            }
                                                            US4::US4_1(v155) => {
                                                                let mut v155: u64 = *v155;
                                                                let mut v156: Rc<str> = std::rc::Rc::<str>::from((v155).to_string());
                                                                v156.clone()
                                                            }
                                                            US4::US4_0 => {
                                                                let mut v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                v154.clone()
                                                            }
                                                        };
                                                        let mut v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" match(es), observed matches="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v162: Rc<str> = std::rc::Rc::<str>::from((v65).to_string());
                                                        let mut v163: Rc<str> = std::rc::Rc::<str>::from([&*v153, &*v160, &*v161, &*v162, &*v53].concat());
                                                        let mut v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (use --expect N or --all for repeated text)"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v165: Rc<str> = std::rc::Rc::<str>::from([&*v163, &*v164, &*v53, &*v53, &*v53].concat());
                                                        US0::US0_1(v165.clone())
                                                    }
                                                } else {
                                                    let mut v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace target is not a regular file"); } LIT.with(|lit| lit.clone()) };
                                                    US0::US0_1(v168.clone())
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
fn method0() -> (i32, Rc<str>) {
    let mut v0: US0 = method1();
    match &v0 {
        US0::US0_0(v1) => {
            let mut v1: Rc<str> = v1.clone();
            (0i32, v1.clone())
        }
        US0::US0_1(v2) => {
            let mut v2: Rc<str> = v2.clone();
            (2i32, v2.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        method0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_text_replace_outcome() -> (i32, Rc<str>) {
    closure0()()
}
