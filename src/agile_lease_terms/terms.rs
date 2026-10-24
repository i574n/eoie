#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_LeaseRunning,
    US1_LeaseWrapping,
    US1_LeaseExpired,
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn process_argument_1(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_2();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_3(v1);
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn lease_effect_entry_0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = process_argument_1(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v2));
    let mut v4: i32 = 2i32;
    let mut v5: Rc<str> = process_argument_1(v4);
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v5));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(std::env::args().count().saturating_sub(3).to_string());
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let mut v11: u64 = command_spec_domain::eoie_command_subcommand_effect_code(&v10);
    let mut v12: bool = v11 <= 3;
    if v12 {
        let mut v13: i32 = v11 as i32;
        v13
    } else {
        let mut v14: i32 = 1i32;
        let mut v15: Rc<str> = process_argument_1(v14);
        let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v2));
        let mut v17: i32 = 2i32;
        let mut v18: Rc<str> = process_argument_1(v17);
        let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v18));
        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v2));
        let mut v21: i32 = 3i32;
        let mut v22: Rc<str> = process_argument_1(v21);
        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v22));
        let mut v24: u64 = command_spec_domain::eoie_command_subcommand_effect_code(&v23);
        let mut v25: bool = v24 <= 3;
        if v25 {
            let mut v26: i32 = v24 as i32;
            v26
        } else {
            let mut v27: i32 = 1i32;
            let mut v28: Rc<str> = process_argument_1(v27);
            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v2));
            let mut v30: i32 = 2i32;
            let mut v31: Rc<str> = process_argument_1(v30);
            let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v31));
            let mut v33: u64 = command_spec_domain::eoie_command_subcommand_effect_code(&v32);
            let mut v34: bool = v33 <= 3;
            if v34 {
                let mut v35: i32 = v33 as i32;
                v35
            } else {
                let mut v36: i32 = 1i32;
                let mut v37: Rc<str> = process_argument_1(v36);
                let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v2));
                let mut v39: i32 = 2i32;
                let mut v40: Rc<str> = process_argument_1(v39);
                let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v40));
                let mut v42: u64 = command_spec_domain::eoie_command_effect_code(&v37);
                let mut v43: bool = v42 <= 3;
                if v43 {
                    let mut v44: i32 = v42 as i32;
                    v44
                } else {
                    let mut v45: i32 = 4u64 as i32;
                    v45
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        lease_effect_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn lease_policy_entry_4() -> Rc<str> {
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
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v13));
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v3));
            v15.clone()
        } else {
            let mut v16: bool = v7.is_empty() == false;
            if v16 {
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/eoie/"); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v17));
                let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v3));
                v19.clone()
            } else {
                let mut v20: bool = v9.is_empty() == false;
                if v20 {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/eoie/"); } LIT.with(|lit| lit.clone()) };
                    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v21));
                    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v3));
                    v23.clone()
                } else {
                    let mut v24: bool = v11.is_empty() == false;
                    if v24 {
                        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/.config/eoie/"); } LIT.with(|lit| lit.clone()) };
                        let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v25));
                        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v3));
                        v27.clone()
                    } else {
                        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
                        let mut v29: Rc<str> = std::rc::Rc::<str>::from(std::env::var(&*v28).unwrap_or_default());
                        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/.config/eoie/"); } LIT.with(|lit| lit.clone()) };
                        let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
                        let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v3));
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
        lease_policy_entry_4()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn lease_status_entry_5(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool) -> Rc<str> {
    let mut v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile lease status root="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v0));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" phase="); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let mut v16: US1 = if v6 {
        US1::US1_LeaseExpired
    } else {
        if v5 {
            US1::US1_LeaseWrapping
        } else {
            US1::US1_LeaseRunning
        }
    };
    let mut v22: Rc<str> = match &v16 {
        US1::US1_LeaseExpired => {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired"); } LIT.with(|lit| lit.clone()) };
            v19.clone()
        }
        US1::US1_LeaseRunning => {
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v17.clone()
        }
        US1::US1_LeaseWrapping => {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wrapping"); } LIT.with(|lit| lit.clone()) };
            v18.clone()
        }
    };
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v22));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" policy="); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let mut v26: Rc<str> = lease_policy_entry_4();
    let mut v27: bool = v26.is_empty() == false;
    let mut v29: Rc<str> = if v27 {
        v26.clone()
    } else {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bounded"); } LIT.with(|lit| lit.clone()) };
        v28.clone()
    };
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" started="); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
    let mut v32: bool = v1 <= v7;
    let mut v111: Rc<str> = if v32 {
        let mut v33: u64 = v7.saturating_sub(v1);
        let mut v34: u64 = v33.wrapping_div(1000u64);
        let mut v35: u64 = v34.wrapping_div(3600u64);
        let mut v36: u64 = v34.wrapping_div(60u64);
        let mut v37: u64 = v36.wrapping_rem(60u64);
        let mut v38: u64 = v34.wrapping_rem(60u64);
        let mut v39: bool = 0 < v35;
        let mut v69: Rc<str> = if v39 {
            let mut v40: Rc<str> = std::rc::Rc::<str>::from((v35).to_string());
            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
            let mut v43: bool = v37 <= 9u64;
            let mut v48: Rc<str> = if v43 {
                let mut v44: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
                let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v44));
                v46.clone()
            } else {
                let mut v47: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
                v47.clone()
            };
            let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v48));
            let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
            v51.clone()
        } else {
            let mut v52: bool = 0 < v37;
            if v52 {
                let mut v53: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
                let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
                let mut v56: bool = v38 <= 9u64;
                let mut v61: Rc<str> = if v56 {
                    let mut v57: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
                    let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v57));
                    v59.clone()
                } else {
                    let mut v60: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
                    v60.clone()
                };
                let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v61));
                let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v63));
                v64.clone()
            } else {
                let mut v65: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
                let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v67: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v66));
                v67.clone()
            }
        };
        let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
        v71.clone()
    } else {
        let mut v72: u64 = v1.saturating_sub(v7);
        let mut v73: u64 = v72.wrapping_div(1000u64);
        let mut v74: u64 = v73.wrapping_div(3600u64);
        let mut v75: u64 = v73.wrapping_div(60u64);
        let mut v76: u64 = v75.wrapping_rem(60u64);
        let mut v77: u64 = v73.wrapping_rem(60u64);
        let mut v78: bool = 0 < v74;
        let mut v108: Rc<str> = if v78 {
            let mut v79: Rc<str> = std::rc::Rc::<str>::from((v74).to_string());
            let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v80));
            let mut v82: bool = v76 <= 9u64;
            let mut v87: Rc<str> = if v82 {
                let mut v83: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v83));
                v85.clone()
            } else {
                let mut v86: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                v86.clone()
            };
            let mut v88: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v87));
            let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v90: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v89));
            v90.clone()
        } else {
            let mut v91: bool = 0 < v76;
            if v91 {
                let mut v92: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v94: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v93));
                let mut v95: bool = v77 <= 9u64;
                let mut v100: Rc<str> = if v95 {
                    let mut v96: Rc<str> = std::rc::Rc::<str>::from((v77).to_string());
                    let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v96));
                    v98.clone()
                } else {
                    let mut v99: Rc<str> = std::rc::Rc::<str>::from((v77).to_string());
                    v99.clone()
                };
                let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v100));
                let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v103: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v102));
                v103.clone()
            } else {
                let mut v104: Rc<str> = std::rc::Rc::<str>::from((v77).to_string());
                let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
                v106.clone()
            }
        };
        let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let mut v110: Rc<str> = Rc::<str>::from(format!("{}{}", v109, v108));
        v110.clone()
    };
    let mut v112: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v111));
    let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wrap_at="); } LIT.with(|lit| lit.clone()) };
    let mut v114: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v113));
    let mut v115: bool = v2 <= v7;
    let mut v194: Rc<str> = if v115 {
        let mut v116: u64 = v7.saturating_sub(v2);
        let mut v117: u64 = v116.wrapping_div(1000u64);
        let mut v118: u64 = v117.wrapping_div(3600u64);
        let mut v119: u64 = v117.wrapping_div(60u64);
        let mut v120: u64 = v119.wrapping_rem(60u64);
        let mut v121: u64 = v117.wrapping_rem(60u64);
        let mut v122: bool = 0 < v118;
        let mut v152: Rc<str> = if v122 {
            let mut v123: Rc<str> = std::rc::Rc::<str>::from((v118).to_string());
            let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v125: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v124));
            let mut v126: bool = v120 <= 9u64;
            let mut v131: Rc<str> = if v126 {
                let mut v127: Rc<str> = std::rc::Rc::<str>::from((v120).to_string());
                let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v129: Rc<str> = Rc::<str>::from(format!("{}{}", v128, v127));
                v129.clone()
            } else {
                let mut v130: Rc<str> = std::rc::Rc::<str>::from((v120).to_string());
                v130.clone()
            };
            let mut v132: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v131));
            let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v134: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v133));
            v134.clone()
        } else {
            let mut v135: bool = 0 < v120;
            if v135 {
                let mut v136: Rc<str> = std::rc::Rc::<str>::from((v120).to_string());
                let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v138: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v137));
                let mut v139: bool = v121 <= 9u64;
                let mut v144: Rc<str> = if v139 {
                    let mut v140: Rc<str> = std::rc::Rc::<str>::from((v121).to_string());
                    let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v142: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v140));
                    v142.clone()
                } else {
                    let mut v143: Rc<str> = std::rc::Rc::<str>::from((v121).to_string());
                    v143.clone()
                };
                let mut v145: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v144));
                let mut v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v147: Rc<str> = Rc::<str>::from(format!("{}{}", v145, v146));
                v147.clone()
            } else {
                let mut v148: Rc<str> = std::rc::Rc::<str>::from((v121).to_string());
                let mut v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v150: Rc<str> = Rc::<str>::from(format!("{}{}", v148, v149));
                v150.clone()
            }
        };
        let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let mut v154: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v153));
        v154.clone()
    } else {
        let mut v155: u64 = v2.saturating_sub(v7);
        let mut v156: u64 = v155.wrapping_div(1000u64);
        let mut v157: u64 = v156.wrapping_div(3600u64);
        let mut v158: u64 = v156.wrapping_div(60u64);
        let mut v159: u64 = v158.wrapping_rem(60u64);
        let mut v160: u64 = v156.wrapping_rem(60u64);
        let mut v161: bool = 0 < v157;
        let mut v191: Rc<str> = if v161 {
            let mut v162: Rc<str> = std::rc::Rc::<str>::from((v157).to_string());
            let mut v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v164: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v163));
            let mut v165: bool = v159 <= 9u64;
            let mut v170: Rc<str> = if v165 {
                let mut v166: Rc<str> = std::rc::Rc::<str>::from((v159).to_string());
                let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v168: Rc<str> = Rc::<str>::from(format!("{}{}", v167, v166));
                v168.clone()
            } else {
                let mut v169: Rc<str> = std::rc::Rc::<str>::from((v159).to_string());
                v169.clone()
            };
            let mut v171: Rc<str> = Rc::<str>::from(format!("{}{}", v164, v170));
            let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v173: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v172));
            v173.clone()
        } else {
            let mut v174: bool = 0 < v159;
            if v174 {
                let mut v175: Rc<str> = std::rc::Rc::<str>::from((v159).to_string());
                let mut v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v177: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v176));
                let mut v178: bool = v160 <= 9u64;
                let mut v183: Rc<str> = if v178 {
                    let mut v179: Rc<str> = std::rc::Rc::<str>::from((v160).to_string());
                    let mut v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v181: Rc<str> = Rc::<str>::from(format!("{}{}", v180, v179));
                    v181.clone()
                } else {
                    let mut v182: Rc<str> = std::rc::Rc::<str>::from((v160).to_string());
                    v182.clone()
                };
                let mut v184: Rc<str> = Rc::<str>::from(format!("{}{}", v177, v183));
                let mut v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v186: Rc<str> = Rc::<str>::from(format!("{}{}", v184, v185));
                v186.clone()
            } else {
                let mut v187: Rc<str> = std::rc::Rc::<str>::from((v160).to_string());
                let mut v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v189: Rc<str> = Rc::<str>::from(format!("{}{}", v187, v188));
                v189.clone()
            }
        };
        let mut v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let mut v193: Rc<str> = Rc::<str>::from(format!("{}{}", v192, v191));
        v193.clone()
    };
    let mut v195: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v194));
    let mut v196: bool = v3 <= v7;
    let mut v275: Rc<str> = if v196 {
        let mut v197: u64 = v7.saturating_sub(v3);
        let mut v198: u64 = v197.wrapping_div(1000u64);
        let mut v199: u64 = v198.wrapping_div(3600u64);
        let mut v200: u64 = v198.wrapping_div(60u64);
        let mut v201: u64 = v200.wrapping_rem(60u64);
        let mut v202: u64 = v198.wrapping_rem(60u64);
        let mut v203: bool = 0 < v199;
        let mut v233: Rc<str> = if v203 {
            let mut v204: Rc<str> = std::rc::Rc::<str>::from((v199).to_string());
            let mut v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v206: Rc<str> = Rc::<str>::from(format!("{}{}", v204, v205));
            let mut v207: bool = v201 <= 9u64;
            let mut v212: Rc<str> = if v207 {
                let mut v208: Rc<str> = std::rc::Rc::<str>::from((v201).to_string());
                let mut v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v210: Rc<str> = Rc::<str>::from(format!("{}{}", v209, v208));
                v210.clone()
            } else {
                let mut v211: Rc<str> = std::rc::Rc::<str>::from((v201).to_string());
                v211.clone()
            };
            let mut v213: Rc<str> = Rc::<str>::from(format!("{}{}", v206, v212));
            let mut v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v215: Rc<str> = Rc::<str>::from(format!("{}{}", v213, v214));
            v215.clone()
        } else {
            let mut v216: bool = 0 < v201;
            if v216 {
                let mut v217: Rc<str> = std::rc::Rc::<str>::from((v201).to_string());
                let mut v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v219: Rc<str> = Rc::<str>::from(format!("{}{}", v217, v218));
                let mut v220: bool = v202 <= 9u64;
                let mut v225: Rc<str> = if v220 {
                    let mut v221: Rc<str> = std::rc::Rc::<str>::from((v202).to_string());
                    let mut v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v223: Rc<str> = Rc::<str>::from(format!("{}{}", v222, v221));
                    v223.clone()
                } else {
                    let mut v224: Rc<str> = std::rc::Rc::<str>::from((v202).to_string());
                    v224.clone()
                };
                let mut v226: Rc<str> = Rc::<str>::from(format!("{}{}", v219, v225));
                let mut v227: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v228: Rc<str> = Rc::<str>::from(format!("{}{}", v226, v227));
                v228.clone()
            } else {
                let mut v229: Rc<str> = std::rc::Rc::<str>::from((v202).to_string());
                let mut v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v231: Rc<str> = Rc::<str>::from(format!("{}{}", v229, v230));
                v231.clone()
            }
        };
        let mut v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let mut v235: Rc<str> = Rc::<str>::from(format!("{}{}", v233, v234));
        v235.clone()
    } else {
        let mut v236: u64 = v3.saturating_sub(v7);
        let mut v237: u64 = v236.wrapping_div(1000u64);
        let mut v238: u64 = v237.wrapping_div(3600u64);
        let mut v239: u64 = v237.wrapping_div(60u64);
        let mut v240: u64 = v239.wrapping_rem(60u64);
        let mut v241: u64 = v237.wrapping_rem(60u64);
        let mut v242: bool = 0 < v238;
        let mut v272: Rc<str> = if v242 {
            let mut v243: Rc<str> = std::rc::Rc::<str>::from((v238).to_string());
            let mut v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let mut v245: Rc<str> = Rc::<str>::from(format!("{}{}", v243, v244));
            let mut v246: bool = v240 <= 9u64;
            let mut v251: Rc<str> = if v246 {
                let mut v247: Rc<str> = std::rc::Rc::<str>::from((v240).to_string());
                let mut v248: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v249: Rc<str> = Rc::<str>::from(format!("{}{}", v248, v247));
                v249.clone()
            } else {
                let mut v250: Rc<str> = std::rc::Rc::<str>::from((v240).to_string());
                v250.clone()
            };
            let mut v252: Rc<str> = Rc::<str>::from(format!("{}{}", v245, v251));
            let mut v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v254: Rc<str> = Rc::<str>::from(format!("{}{}", v252, v253));
            v254.clone()
        } else {
            let mut v255: bool = 0 < v240;
            if v255 {
                let mut v256: Rc<str> = std::rc::Rc::<str>::from((v240).to_string());
                let mut v257: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let mut v258: Rc<str> = Rc::<str>::from(format!("{}{}", v256, v257));
                let mut v259: bool = v241 <= 9u64;
                let mut v264: Rc<str> = if v259 {
                    let mut v260: Rc<str> = std::rc::Rc::<str>::from((v241).to_string());
                    let mut v261: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let mut v262: Rc<str> = Rc::<str>::from(format!("{}{}", v261, v260));
                    v262.clone()
                } else {
                    let mut v263: Rc<str> = std::rc::Rc::<str>::from((v241).to_string());
                    v263.clone()
                };
                let mut v265: Rc<str> = Rc::<str>::from(format!("{}{}", v258, v264));
                let mut v266: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v267: Rc<str> = Rc::<str>::from(format!("{}{}", v265, v266));
                v267.clone()
            } else {
                let mut v268: Rc<str> = std::rc::Rc::<str>::from((v241).to_string());
                let mut v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let mut v270: Rc<str> = Rc::<str>::from(format!("{}{}", v268, v269));
                v270.clone()
            }
        };
        let mut v273: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let mut v274: Rc<str> = Rc::<str>::from(format!("{}{}", v273, v272));
        v274.clone()
    };
    let mut v276: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deadline="); } LIT.with(|lit| lit.clone()) };
    let mut v277: Rc<str> = Rc::<str>::from(format!("{}{}", v276, v275));
    let mut v278: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" remaining="); } LIT.with(|lit| lit.clone()) };
    let mut v279: Rc<str> = Rc::<str>::from(format!("{}{}", v277, v278));
    let mut v280: u64 = v4.wrapping_div(1000u64);
    let mut v281: u64 = v280.wrapping_div(3600u64);
    let mut v282: u64 = v280.wrapping_div(60u64);
    let mut v283: u64 = v282.wrapping_rem(60u64);
    let mut v284: u64 = v280.wrapping_rem(60u64);
    let mut v285: bool = 0 < v281;
    let mut v315: Rc<str> = if v285 {
        let mut v286: Rc<str> = std::rc::Rc::<str>::from((v281).to_string());
        let mut v287: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
        let mut v288: Rc<str> = Rc::<str>::from(format!("{}{}", v286, v287));
        let mut v289: bool = v283 <= 9u64;
        let mut v294: Rc<str> = if v289 {
            let mut v290: Rc<str> = std::rc::Rc::<str>::from((v283).to_string());
            let mut v291: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            let mut v292: Rc<str> = Rc::<str>::from(format!("{}{}", v291, v290));
            v292.clone()
        } else {
            let mut v293: Rc<str> = std::rc::Rc::<str>::from((v283).to_string());
            v293.clone()
        };
        let mut v295: Rc<str> = Rc::<str>::from(format!("{}{}", v288, v294));
        let mut v296: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
        let mut v297: Rc<str> = Rc::<str>::from(format!("{}{}", v295, v296));
        v297.clone()
    } else {
        let mut v298: bool = 0 < v283;
        if v298 {
            let mut v299: Rc<str> = std::rc::Rc::<str>::from((v283).to_string());
            let mut v300: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let mut v301: Rc<str> = Rc::<str>::from(format!("{}{}", v299, v300));
            let mut v302: bool = v284 <= 9u64;
            let mut v307: Rc<str> = if v302 {
                let mut v303: Rc<str> = std::rc::Rc::<str>::from((v284).to_string());
                let mut v304: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v305: Rc<str> = Rc::<str>::from(format!("{}{}", v304, v303));
                v305.clone()
            } else {
                let mut v306: Rc<str> = std::rc::Rc::<str>::from((v284).to_string());
                v306.clone()
            };
            let mut v308: Rc<str> = Rc::<str>::from(format!("{}{}", v301, v307));
            let mut v309: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
            let mut v310: Rc<str> = Rc::<str>::from(format!("{}{}", v308, v309));
            v310.clone()
        } else {
            let mut v311: Rc<str> = std::rc::Rc::<str>::from((v284).to_string());
            let mut v312: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
            let mut v313: Rc<str> = Rc::<str>::from(format!("{}{}", v311, v312));
            v313.clone()
        }
    };
    let mut v316: Rc<str> = Rc::<str>::from(format!("{}{}", v279, v315));
    let mut v319: Rc<str> = if v5 {
        let mut v317: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v317.clone()
    } else {
        let mut v318: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v318.clone()
    };
    let mut v320: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" should_wrap="); } LIT.with(|lit| lit.clone()) };
    let mut v321: Rc<str> = Rc::<str>::from(format!("{}{}", v320, v319));
    let mut v322: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" should_yield="); } LIT.with(|lit| lit.clone()) };
    let mut v323: Rc<str> = Rc::<str>::from(format!("{}{}", v321, v322));
    let mut v326: Rc<str> = if v6 {
        let mut v324: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v324.clone()
    } else {
        let mut v325: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v325.clone()
    };
    let mut v327: Rc<str> = Rc::<str>::from(format!("{}{}", v323, v326));
    let mut v328: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v329: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" started_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v330: Rc<str> = Rc::<str>::from(format!("{}{}", v329, v328));
    let mut v331: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wrap_at_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v332: Rc<str> = Rc::<str>::from(format!("{}{}", v330, v331));
    let mut v333: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let mut v334: Rc<str> = Rc::<str>::from(format!("{}{}", v332, v333));
    let mut v335: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v336: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deadline_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v337: Rc<str> = Rc::<str>::from(format!("{}{}", v336, v335));
    let mut v338: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" remaining_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v339: Rc<str> = Rc::<str>::from(format!("{}{}", v337, v338));
    let mut v340: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let mut v341: Rc<str> = Rc::<str>::from(format!("{}{}", v339, v340));
    let mut v342: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v195));
    let mut v343: Rc<str> = Rc::<str>::from(format!("{}{}", v342, v316));
    let mut v344: Rc<str> = Rc::<str>::from(format!("{}{}", v343, v327));
    let mut v345: Rc<str> = Rc::<str>::from(format!("{}{}", v344, v334));
    let mut v346: Rc<str> = Rc::<str>::from(format!("{}{}", v345, v341));
    v346.clone()
}
fn closure2() -> Rc<dyn Fn(Rc<str>, u64, u64, u64, u64, bool, bool) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, u64, u64, u64, bool, bool) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool| -> Rc<str> {
        lease_status_entry_5(v0.clone(), v1, v2, v3, v4, v5, v6)
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
