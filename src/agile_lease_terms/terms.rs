#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_0,
    US0_1,
    US0_2,
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_0 => 0,
            US0::US0_1 => 1,
            US0::US0_2 => 2,
        }
    }
}
fn method0() -> i32 {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(1i32 as usize).unwrap_or_default());
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2i32 as usize).unwrap_or_default());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v3].concat());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::env::args().count().saturating_sub(3).to_string());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v5, &*v6, &*v3, &*v3].concat());
    let mut v8: u64 = command_spec_domain::eoie_command_subcommand_effect_code(&v7);
    let mut v9: bool = v8 <= 3;
    if v9 {
        let mut v10: i32 = v8 as i32;
        v10
    } else {
        let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(1i32 as usize).unwrap_or_default());
        let mut v12: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2i32 as usize).unwrap_or_default());
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v1, &*v12, &*v3, &*v3].concat());
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(3i32 as usize).unwrap_or_default());
        let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v1, &*v14, &*v3, &*v3].concat());
        let mut v16: u64 = command_spec_domain::eoie_command_subcommand_effect_code(&v15);
        let mut v17: bool = v16 <= 3;
        if v17 {
            let mut v18: i32 = v16 as i32;
            v18
        } else {
            let mut v19: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(1i32 as usize).unwrap_or_default());
            let mut v20: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2i32 as usize).unwrap_or_default());
            let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v1, &*v20, &*v3, &*v3].concat());
            let mut v22: u64 = command_spec_domain::eoie_command_subcommand_effect_code(&v21);
            let mut v23: bool = v22 <= 3;
            if v23 {
                let mut v24: i32 = v22 as i32;
                v24
            } else {
                let mut v25: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(1i32 as usize).unwrap_or_default());
                let mut v26: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2i32 as usize).unwrap_or_default());
                let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v1, &*v26, &*v3, &*v3].concat());
                let mut v28: u64 = command_spec_domain::eoie_command_effect_code(&v25);
                let mut v29: bool = v28 <= 3;
                if v29 {
                    let mut v30: i32 = v28 as i32;
                    v30
                } else {
                    let mut v31: i32 = 4u64 as i32;
                    v31
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        method0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method1() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE_LEASE_POLICY"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    let mut v2: bool = v1.is_empty() == false;
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease-policy"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(std::env::var(&*v4).unwrap_or_default());
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("XDG_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(std::env::var(&*v6).unwrap_or_default());
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from(std::env::var(&*v8).unwrap_or_default());
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("USERPROFILE"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::env::var(&*v10).unwrap_or_default());
        let mut v12: bool = v5.is_empty() == false;
        let mut v36: Rc<str> = if v12 {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v13, &*v3, &*v14, &*v14].concat());
            v15.clone()
        } else {
            let mut v16: bool = v7.is_empty() == false;
            if v16 {
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/eoie/"); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v17, &*v3, &*v18, &*v18].concat());
                v19.clone()
            } else {
                let mut v20: bool = v9.is_empty() == false;
                if v20 {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/eoie/"); } LIT.with(|lit| lit.clone()) };
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v21, &*v3, &*v22, &*v22].concat());
                    v23.clone()
                } else {
                    let mut v24: bool = v11.is_empty() == false;
                    if v24 {
                        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/.config/eoie/"); } LIT.with(|lit| lit.clone()) };
                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v25, &*v3, &*v26, &*v26].concat());
                        v27.clone()
                    } else {
                        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
                        let mut v29: Rc<str> = std::rc::Rc::<str>::from(std::env::var(&*v28).unwrap_or_default());
                        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/.config/eoie/"); } LIT.with(|lit| lit.clone()) };
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v30, &*v3, &*v31, &*v31].concat());
                        v32.clone()
                    }
                }
            }
        };
        let mut v37: Rc<str> = std::fs::read_to_string(&*v36).map(|text| std::rc::Rc::<str>::from(text.trim())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        v37.clone()
    }
}
fn closure1() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        method1()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method2(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool) -> Rc<str> {
    let mut v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile lease status root="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" phase="); } LIT.with(|lit| lit.clone()) };
    let mut v14: US0 = if v6 {
        US0::US0_2
    } else {
        if v5 {
            US0::US0_1
        } else {
            US0::US0_0
        }
    };
    let mut v20: Rc<str> = match &v14 {
        US0::US0_2 => {
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired"); } LIT.with(|lit| lit.clone()) };
            v17.clone()
        }
        US0::US0_0 => {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v15.clone()
        }
        US0::US0_1 => {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wrapping"); } LIT.with(|lit| lit.clone()) };
            v16.clone()
        }
    };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" policy="); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v0, &*v9, &*v20, &*v21].concat());
    let mut v23: Rc<str> = method1();
    let mut v24: bool = v23.is_empty() == false;
    let mut v26: Rc<str> = if v24 {
        v23.clone()
    } else {
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bounded"); } LIT.with(|lit| lit.clone()) };
        v25.clone()
    };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" started="); } LIT.with(|lit| lit.clone()) };
    let mut v28: bool = v1 <= v7;
    let mut v111: Rc<str> = if v28 {
        let mut v29: u64 = v7.saturating_sub(v1);
        let mut v30: u64 = v29 / 1000u64;
        let mut v31: u64 = v30 / 3600u64;
        let mut v32: u64 = v30 / 60u64;
        let mut v33: u64 = v32 % 60u64;
        let mut v34: u64 = v30 % 60u64;
        let mut v35: bool = 0 < v31;
        let mut v66: Rc<str> = if v35 {
            let mut v36: Rc<str> = std::rc::Rc::<str>::from((v31).to_string());
            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v38: bool = v33 <= 9u64;
            let mut v44: Rc<str> = if v38 {
                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v40: Rc<str> = std::rc::Rc::<str>::from((v33).to_string());
                let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v40, &*v41, &*v41, &*v41].concat());
                v42.clone()
            } else {
                let mut v43: Rc<str> = std::rc::Rc::<str>::from((v33).to_string());
                v43.clone()
            };
            let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v37, &*v44, &*v45, &*v46].concat());
            v47.clone()
        } else {
            let mut v48: bool = 0 < v33;
            if v48 {
                let mut v49: Rc<str> = std::rc::Rc::<str>::from((v33).to_string());
                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v51: bool = v34 <= 9u64;
                let mut v57: Rc<str> = if v51 {
                    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v53: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
                    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v52, &*v53, &*v54, &*v54, &*v54].concat());
                    v55.clone()
                } else {
                    let mut v56: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
                    v56.clone()
                };
                let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v60: Rc<str> = std::rc::Rc::<str>::from([&*v49, &*v50, &*v57, &*v58, &*v59].concat());
                v60.clone()
            } else {
                let mut v61: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
                let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v64: Rc<str> = std::rc::Rc::<str>::from([&*v61, &*v62, &*v63, &*v63, &*v63].concat());
                v64.clone()
            }
        };
        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v69: Rc<str> = std::rc::Rc::<str>::from([&*v66, &*v67, &*v68, &*v68, &*v68].concat());
        v69.clone()
    } else {
        let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let mut v71: u64 = v1.saturating_sub(v7);
        let mut v72: u64 = v71 / 1000u64;
        let mut v73: u64 = v72 / 3600u64;
        let mut v74: u64 = v72 / 60u64;
        let mut v75: u64 = v74 % 60u64;
        let mut v76: u64 = v72 % 60u64;
        let mut v77: bool = 0 < v73;
        let mut v108: Rc<str> = if v77 {
            let mut v78: Rc<str> = std::rc::Rc::<str>::from((v73).to_string());
            let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v80: bool = v75 <= 9u64;
            let mut v86: Rc<str> = if v80 {
                let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v82: Rc<str> = std::rc::Rc::<str>::from((v75).to_string());
                let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v84: Rc<str> = std::rc::Rc::<str>::from([&*v81, &*v82, &*v83, &*v83, &*v83].concat());
                v84.clone()
            } else {
                let mut v85: Rc<str> = std::rc::Rc::<str>::from((v75).to_string());
                v85.clone()
            };
            let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v89: Rc<str> = std::rc::Rc::<str>::from([&*v78, &*v79, &*v86, &*v87, &*v88].concat());
            v89.clone()
        } else {
            let mut v90: bool = 0 < v75;
            if v90 {
                let mut v91: Rc<str> = std::rc::Rc::<str>::from((v75).to_string());
                let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v93: bool = v76 <= 9u64;
                let mut v99: Rc<str> = if v93 {
                    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v95: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                    let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v97: Rc<str> = std::rc::Rc::<str>::from([&*v94, &*v95, &*v96, &*v96, &*v96].concat());
                    v97.clone()
                } else {
                    let mut v98: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                    v98.clone()
                };
                let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v102: Rc<str> = std::rc::Rc::<str>::from([&*v91, &*v92, &*v99, &*v100, &*v101].concat());
                v102.clone()
            } else {
                let mut v103: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v106: Rc<str> = std::rc::Rc::<str>::from([&*v103, &*v104, &*v105, &*v105, &*v105].concat());
                v106.clone()
            }
        };
        let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v110: Rc<str> = std::rc::Rc::<str>::from([&*v70, &*v108, &*v109, &*v109, &*v109].concat());
        v110.clone()
    };
    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wrap_at="); } LIT.with(|lit| lit.clone()) };
    let mut v113: bool = v2 <= v7;
    let mut v196: Rc<str> = if v113 {
        let mut v114: u64 = v7.saturating_sub(v2);
        let mut v115: u64 = v114 / 1000u64;
        let mut v116: u64 = v115 / 3600u64;
        let mut v117: u64 = v115 / 60u64;
        let mut v118: u64 = v117 % 60u64;
        let mut v119: u64 = v115 % 60u64;
        let mut v120: bool = 0 < v116;
        let mut v151: Rc<str> = if v120 {
            let mut v121: Rc<str> = std::rc::Rc::<str>::from((v116).to_string());
            let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v123: bool = v118 <= 9u64;
            let mut v129: Rc<str> = if v123 {
                let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v125: Rc<str> = std::rc::Rc::<str>::from((v118).to_string());
                let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v127: Rc<str> = std::rc::Rc::<str>::from([&*v124, &*v125, &*v126, &*v126, &*v126].concat());
                v127.clone()
            } else {
                let mut v128: Rc<str> = std::rc::Rc::<str>::from((v118).to_string());
                v128.clone()
            };
            let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v132: Rc<str> = std::rc::Rc::<str>::from([&*v121, &*v122, &*v129, &*v130, &*v131].concat());
            v132.clone()
        } else {
            let mut v133: bool = 0 < v118;
            if v133 {
                let mut v134: Rc<str> = std::rc::Rc::<str>::from((v118).to_string());
                let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v136: bool = v119 <= 9u64;
                let mut v142: Rc<str> = if v136 {
                    let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v138: Rc<str> = std::rc::Rc::<str>::from((v119).to_string());
                    let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v140: Rc<str> = std::rc::Rc::<str>::from([&*v137, &*v138, &*v139, &*v139, &*v139].concat());
                    v140.clone()
                } else {
                    let mut v141: Rc<str> = std::rc::Rc::<str>::from((v119).to_string());
                    v141.clone()
                };
                let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v145: Rc<str> = std::rc::Rc::<str>::from([&*v134, &*v135, &*v142, &*v143, &*v144].concat());
                v145.clone()
            } else {
                let mut v146: Rc<str> = std::rc::Rc::<str>::from((v119).to_string());
                let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v149: Rc<str> = std::rc::Rc::<str>::from([&*v146, &*v147, &*v148, &*v148, &*v148].concat());
                v149.clone()
            }
        };
        let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v154: Rc<str> = std::rc::Rc::<str>::from([&*v151, &*v152, &*v153, &*v153, &*v153].concat());
        v154.clone()
    } else {
        let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let mut v156: u64 = v2.saturating_sub(v7);
        let mut v157: u64 = v156 / 1000u64;
        let mut v158: u64 = v157 / 3600u64;
        let mut v159: u64 = v157 / 60u64;
        let mut v160: u64 = v159 % 60u64;
        let mut v161: u64 = v157 % 60u64;
        let mut v162: bool = 0 < v158;
        let mut v193: Rc<str> = if v162 {
            let mut v163: Rc<str> = std::rc::Rc::<str>::from((v158).to_string());
            let mut v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v165: bool = v160 <= 9u64;
            let mut v171: Rc<str> = if v165 {
                let mut v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v167: Rc<str> = std::rc::Rc::<str>::from((v160).to_string());
                let mut v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v169: Rc<str> = std::rc::Rc::<str>::from([&*v166, &*v167, &*v168, &*v168, &*v168].concat());
                v169.clone()
            } else {
                let mut v170: Rc<str> = std::rc::Rc::<str>::from((v160).to_string());
                v170.clone()
            };
            let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v174: Rc<str> = std::rc::Rc::<str>::from([&*v163, &*v164, &*v171, &*v172, &*v173].concat());
            v174.clone()
        } else {
            let mut v175: bool = 0 < v160;
            if v175 {
                let mut v176: Rc<str> = std::rc::Rc::<str>::from((v160).to_string());
                let mut v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v178: bool = v161 <= 9u64;
                let mut v184: Rc<str> = if v178 {
                    let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v180: Rc<str> = std::rc::Rc::<str>::from((v161).to_string());
                    let mut v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v182: Rc<str> = std::rc::Rc::<str>::from([&*v179, &*v180, &*v181, &*v181, &*v181].concat());
                    v182.clone()
                } else {
                    let mut v183: Rc<str> = std::rc::Rc::<str>::from((v161).to_string());
                    v183.clone()
                };
                let mut v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v187: Rc<str> = std::rc::Rc::<str>::from([&*v176, &*v177, &*v184, &*v185, &*v186].concat());
                v187.clone()
            } else {
                let mut v188: Rc<str> = std::rc::Rc::<str>::from((v161).to_string());
                let mut v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v191: Rc<str> = std::rc::Rc::<str>::from([&*v188, &*v189, &*v190, &*v190, &*v190].concat());
                v191.clone()
            }
        };
        let mut v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v195: Rc<str> = std::rc::Rc::<str>::from([&*v155, &*v193, &*v194, &*v194, &*v194].concat());
        v195.clone()
    };
    let mut v197: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v27, &*v111, &*v112, &*v196].concat());
    let mut v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deadline="); } LIT.with(|lit| lit.clone()) };
    let mut v199: bool = v3 <= v7;
    let mut v282: Rc<str> = if v199 {
        let mut v200: u64 = v7.saturating_sub(v3);
        let mut v201: u64 = v200 / 1000u64;
        let mut v202: u64 = v201 / 3600u64;
        let mut v203: u64 = v201 / 60u64;
        let mut v204: u64 = v203 % 60u64;
        let mut v205: u64 = v201 % 60u64;
        let mut v206: bool = 0 < v202;
        let mut v237: Rc<str> = if v206 {
            let mut v207: Rc<str> = std::rc::Rc::<str>::from((v202).to_string());
            let mut v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v209: bool = v204 <= 9u64;
            let mut v215: Rc<str> = if v209 {
                let mut v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v211: Rc<str> = std::rc::Rc::<str>::from((v204).to_string());
                let mut v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v213: Rc<str> = std::rc::Rc::<str>::from([&*v210, &*v211, &*v212, &*v212, &*v212].concat());
                v213.clone()
            } else {
                let mut v214: Rc<str> = std::rc::Rc::<str>::from((v204).to_string());
                v214.clone()
            };
            let mut v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v218: Rc<str> = std::rc::Rc::<str>::from([&*v207, &*v208, &*v215, &*v216, &*v217].concat());
            v218.clone()
        } else {
            let mut v219: bool = 0 < v204;
            if v219 {
                let mut v220: Rc<str> = std::rc::Rc::<str>::from((v204).to_string());
                let mut v221: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v222: bool = v205 <= 9u64;
                let mut v228: Rc<str> = if v222 {
                    let mut v223: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v224: Rc<str> = std::rc::Rc::<str>::from((v205).to_string());
                    let mut v225: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v226: Rc<str> = std::rc::Rc::<str>::from([&*v223, &*v224, &*v225, &*v225, &*v225].concat());
                    v226.clone()
                } else {
                    let mut v227: Rc<str> = std::rc::Rc::<str>::from((v205).to_string());
                    v227.clone()
                };
                let mut v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v231: Rc<str> = std::rc::Rc::<str>::from([&*v220, &*v221, &*v228, &*v229, &*v230].concat());
                v231.clone()
            } else {
                let mut v232: Rc<str> = std::rc::Rc::<str>::from((v205).to_string());
                let mut v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v235: Rc<str> = std::rc::Rc::<str>::from([&*v232, &*v233, &*v234, &*v234, &*v234].concat());
                v235.clone()
            }
        };
        let mut v238: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let mut v239: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v240: Rc<str> = std::rc::Rc::<str>::from([&*v237, &*v238, &*v239, &*v239, &*v239].concat());
        v240.clone()
    } else {
        let mut v241: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let mut v242: u64 = v3.saturating_sub(v7);
        let mut v243: u64 = v242 / 1000u64;
        let mut v244: u64 = v243 / 3600u64;
        let mut v245: u64 = v243 / 60u64;
        let mut v246: u64 = v245 % 60u64;
        let mut v247: u64 = v243 % 60u64;
        let mut v248: bool = 0 < v244;
        let mut v279: Rc<str> = if v248 {
            let mut v249: Rc<str> = std::rc::Rc::<str>::from((v244).to_string());
            let mut v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v251: bool = v246 <= 9u64;
            let mut v257: Rc<str> = if v251 {
                let mut v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v253: Rc<str> = std::rc::Rc::<str>::from((v246).to_string());
                let mut v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v255: Rc<str> = std::rc::Rc::<str>::from([&*v252, &*v253, &*v254, &*v254, &*v254].concat());
                v255.clone()
            } else {
                let mut v256: Rc<str> = std::rc::Rc::<str>::from((v246).to_string());
                v256.clone()
            };
            let mut v258: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v260: Rc<str> = std::rc::Rc::<str>::from([&*v249, &*v250, &*v257, &*v258, &*v259].concat());
            v260.clone()
        } else {
            let mut v261: bool = 0 < v246;
            if v261 {
                let mut v262: Rc<str> = std::rc::Rc::<str>::from((v246).to_string());
                let mut v263: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v264: bool = v247 <= 9u64;
                let mut v270: Rc<str> = if v264 {
                    let mut v265: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v266: Rc<str> = std::rc::Rc::<str>::from((v247).to_string());
                    let mut v267: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v268: Rc<str> = std::rc::Rc::<str>::from([&*v265, &*v266, &*v267, &*v267, &*v267].concat());
                    v268.clone()
                } else {
                    let mut v269: Rc<str> = std::rc::Rc::<str>::from((v247).to_string());
                    v269.clone()
                };
                let mut v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v272: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v273: Rc<str> = std::rc::Rc::<str>::from([&*v262, &*v263, &*v270, &*v271, &*v272].concat());
                v273.clone()
            } else {
                let mut v274: Rc<str> = std::rc::Rc::<str>::from((v247).to_string());
                let mut v275: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v276: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v277: Rc<str> = std::rc::Rc::<str>::from([&*v274, &*v275, &*v276, &*v276, &*v276].concat());
                v277.clone()
            }
        };
        let mut v280: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v281: Rc<str> = std::rc::Rc::<str>::from([&*v241, &*v279, &*v280, &*v280, &*v280].concat());
        v281.clone()
    };
    let mut v283: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" remaining="); } LIT.with(|lit| lit.clone()) };
    let mut v284: u64 = v4 / 1000u64;
    let mut v285: u64 = v284 / 3600u64;
    let mut v286: u64 = v284 / 60u64;
    let mut v287: u64 = v286 % 60u64;
    let mut v288: u64 = v284 % 60u64;
    let mut v289: bool = 0 < v285;
    let mut v320: Rc<str> = if v289 {
        let mut v290: Rc<str> = std::rc::Rc::<str>::from((v285).to_string());
        let mut v291: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
        let mut v292: bool = v287 <= 9u64;
        let mut v298: Rc<str> = if v292 {
            let mut v293: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            let mut v294: Rc<str> = std::rc::Rc::<str>::from((v287).to_string());
            let mut v295: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v296: Rc<str> = std::rc::Rc::<str>::from([&*v293, &*v294, &*v295, &*v295, &*v295].concat());
            v296.clone()
        } else {
            let mut v297: Rc<str> = std::rc::Rc::<str>::from((v287).to_string());
            v297.clone()
        };
        let mut v299: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
        let mut v300: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v301: Rc<str> = std::rc::Rc::<str>::from([&*v290, &*v291, &*v298, &*v299, &*v300].concat());
        v301.clone()
    } else {
        let mut v302: bool = 0 < v287;
        if v302 {
            let mut v303: Rc<str> = std::rc::Rc::<str>::from((v287).to_string());
            let mut v304: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v305: bool = v288 <= 9u64;
            let mut v311: Rc<str> = if v305 {
                let mut v306: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v307: Rc<str> = std::rc::Rc::<str>::from((v288).to_string());
                let mut v308: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v309: Rc<str> = std::rc::Rc::<str>::from([&*v306, &*v307, &*v308, &*v308, &*v308].concat());
                v309.clone()
            } else {
                let mut v310: Rc<str> = std::rc::Rc::<str>::from((v288).to_string());
                v310.clone()
            };
            let mut v312: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
            let mut v313: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v314: Rc<str> = std::rc::Rc::<str>::from([&*v303, &*v304, &*v311, &*v312, &*v313].concat());
            v314.clone()
        } else {
            let mut v315: Rc<str> = std::rc::Rc::<str>::from((v288).to_string());
            let mut v316: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
            let mut v317: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v318: Rc<str> = std::rc::Rc::<str>::from([&*v315, &*v316, &*v317, &*v317, &*v317].concat());
            v318.clone()
        }
    };
    let mut v321: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v322: Rc<str> = std::rc::Rc::<str>::from([&*v198, &*v282, &*v283, &*v320, &*v321].concat());
    let mut v323: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" should_wrap="); } LIT.with(|lit| lit.clone()) };
    let mut v326: Rc<str> = if v5 {
        let mut v324: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v324.clone()
    } else {
        let mut v325: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v325.clone()
    };
    let mut v327: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" should_yield="); } LIT.with(|lit| lit.clone()) };
    let mut v330: Rc<str> = if v6 {
        let mut v328: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v328.clone()
    } else {
        let mut v329: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v329.clone()
    };
    let mut v331: Rc<str> = std::rc::Rc::<str>::from([&*v323, &*v326, &*v327, &*v330, &*v321].concat());
    let mut v332: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" started_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v333: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v334: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wrap_at_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v335: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v336: Rc<str> = std::rc::Rc::<str>::from([&*v332, &*v333, &*v334, &*v335, &*v321].concat());
    let mut v337: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deadline_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v338: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v339: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" remaining_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v340: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v341: Rc<str> = std::rc::Rc::<str>::from([&*v337, &*v338, &*v339, &*v340, &*v321].concat());
    let mut v342: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v197, &*v322, &*v321, &*v321].concat());
    let mut v343: Rc<str> = std::rc::Rc::<str>::from([&*v342, &*v331, &*v336, &*v341, &*v321].concat());
    v343.clone()
}
fn closure2() -> Rc<dyn Fn(Rc<str>, u64, u64, u64, u64, bool, bool) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, u64, u64, u64, bool, bool) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool| -> Rc<str> {
        method2(v0.clone(), v1, v2, v3, v4, v5, v6)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_lease_effect_code() -> i32 {
    closure0()()
}
pub fn eoie_lease_policy_text() -> Rc<str> {
    closure1()()
}
pub fn eoie_lease_status_text(v0: &str, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: bool) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), v1, v2, v3, v4, v5, v6)
}
