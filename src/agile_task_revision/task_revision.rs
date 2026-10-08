#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_0,
    US1_1,
    US1_2,
    US1_3,
    US1_4,
    US1_5,
    US1_6,
}
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_0 => 0,
            US1::US1_1 => 1,
            US1::US1_2 => 2,
            US1::US1_3 => 3,
            US1::US1_4 => 4,
            US1::US1_5 => 5,
            US1::US1_6 => 6,
        }
    }
}
#[derive(Clone)]
enum US0 {
    US0_0(US1),
    US0_1,
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_0(..) => 0,
            US0::US0_1 => 1,
        }
    }
}
#[derive(Clone)]
enum US2 {
    US2_0(Rc<str>),
    US2_1(Rc<str>),
}
impl US2 {
    fn tag(&self) -> i32 {
        match self {
            US2::US2_0(..) => 0,
            US2::US2_1(..) => 1,
        }
    }
}
#[derive(Clone)]
enum US3 {
    US3_0,
    US3_1,
    US3_2,
}
impl US3 {
    fn tag(&self) -> i32 {
        match self {
            US3::US3_0 => 0,
            US3::US3_1 => 1,
            US3::US3_2 => 2,
        }
    }
}
#[derive(Clone)]
enum US4 {
    US4_0(Rc<str>),
    US4_1,
}
impl US4 {
    fn tag(&self) -> i32 {
        match self {
            US4::US4_0(..) => 0,
            US4::US4_1 => 1,
        }
    }
}
#[derive(Clone)]
enum US6 {
    US6_0,
    US6_1,
    US6_2,
    US6_3,
    US6_4,
    US6_5,
    US6_6,
    US6_7,
    US6_8,
}
impl US6 {
    fn tag(&self) -> i32 {
        match self {
            US6::US6_0 => 0,
            US6::US6_1 => 1,
            US6::US6_2 => 2,
            US6::US6_3 => 3,
            US6::US6_4 => 4,
            US6::US6_5 => 5,
            US6::US6_6 => 6,
            US6::US6_7 => 7,
            US6::US6_8 => 8,
        }
    }
}
#[derive(Clone)]
enum US5 {
    US5_0(US6),
    US5_1,
}
impl US5 {
    fn tag(&self) -> i32 {
        match self {
            US5::US5_0(..) => 0,
            US5::US5_1 => 1,
        }
    }
}
#[derive(Clone)]
enum US8 {
    US8_0,
    US8_1,
    US8_2,
    US8_3,
}
impl US8 {
    fn tag(&self) -> i32 {
        match self {
            US8::US8_0 => 0,
            US8::US8_1 => 1,
            US8::US8_2 => 2,
            US8::US8_3 => 3,
        }
    }
}
#[derive(Clone)]
enum US7 {
    US7_0(US8),
    US7_1,
}
impl US7 {
    fn tag(&self) -> i32 {
        match self {
            US7::US7_0(..) => 0,
            US7::US7_1 => 1,
        }
    }
}
#[derive(Clone)]
enum US10 {
    US10_0,
    US10_1,
    US10_2,
    US10_3,
    US10_4,
    US10_5,
}
impl US10 {
    fn tag(&self) -> i32 {
        match self {
            US10::US10_0 => 0,
            US10::US10_1 => 1,
            US10::US10_2 => 2,
            US10::US10_3 => 3,
            US10::US10_4 => 4,
            US10::US10_5 => 5,
        }
    }
}
#[derive(Clone)]
enum US9 {
    US9_0(US10),
    US9_1,
}
impl US9 {
    fn tag(&self) -> i32 {
        match self {
            US9::US9_0(..) => 0,
            US9::US9_1 => 1,
        }
    }
}
#[derive(Clone)]
enum US11 {
    US11_0,
    US11_1,
    US11_2,
    US11_3,
}
impl US11 {
    fn tag(&self) -> i32 {
        match self {
            US11::US11_0 => 0,
            US11::US11_1 => 1,
            US11::US11_2 => 2,
            US11::US11_3 => 3,
        }
    }
}
fn method2(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US3) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US3::US3_0 => {
                    let mut v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let mut v10: i32 = v1 + 1;
                let mut v25: US3 = match &v3 {
                    US3::US3_2 => {
                        US3::US3_1
                    }
                    US3::US3_0 => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US3::US3_1
                        } else {
                            US3::US3_0
                        }
                    }
                    US3::US3_1 => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US3::US3_2
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US3::US3_0
                            } else {
                                US3::US3_1
                            }
                        }
                    }
                };
                (v0, v1, v2, v3) = (v0.clone(), v10, v2, v25.clone());
                continue;
            }
        }
    }
}
fn method3(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1 + 1;
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn method1(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US3 = US3::US3_0;
            let mut v6: i32 = method2(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6 + 1;
            let mut v8: i32 = method3(v0.clone(), v7);
            let mut v9: i32 = v3 - 1;
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn method0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title|title-append|kind|priority|difficulty|deps|tags"); } LIT.with(|lit| lit.clone()) };
    let mut v3: u64 = v2.split("|").position(|item| item == &*v1).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v4: bool = 0u64 == v3;
    let mut v32: US0 = if v4 {
        let mut v5: US1 = US1::US1_0;
        US0::US0_0(v5.clone())
    } else {
        let mut v7: bool = 1u64 == v3;
        if v7 {
            let mut v8: US1 = US1::US1_1;
            US0::US0_0(v8.clone())
        } else {
            let mut v10: bool = 2u64 == v3;
            if v10 {
                let mut v11: US1 = US1::US1_2;
                US0::US0_0(v11.clone())
            } else {
                let mut v13: bool = 3u64 == v3;
                if v13 {
                    let mut v14: US1 = US1::US1_3;
                    US0::US0_0(v14.clone())
                } else {
                    let mut v16: bool = 4u64 == v3;
                    if v16 {
                        let mut v17: US1 = US1::US1_4;
                        US0::US0_0(v17.clone())
                    } else {
                        let mut v19: bool = 5u64 == v3;
                        if v19 {
                            let mut v20: US1 = US1::US1_5;
                            US0::US0_0(v20.clone())
                        } else {
                            let mut v22: bool = 6u64 == v3;
                            if v22 {
                                let mut v23: US1 = US1::US1_6;
                                US0::US0_0(v23.clone())
                            } else {
                                US0::US0_1
                            }
                        }
                    }
                }
            }
        }
    };
    match &v32 {
        US0::US0_1 => {
            let mut v356: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be title, title-append, kind, priority, difficulty, deps or tags"); } LIT.with(|lit| lit.clone()) };
            let mut v357: i32 = 2i32;
            let mut v358: Rc<str> = usize::try_from(v357).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v359: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile edit "); } LIT.with(|lit| lit.clone()) };
            let mut v360: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v361: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v362: Rc<str> = std::rc::Rc::<str>::from([&*v359, &*v358, &*v360, &*v356, &*v361].concat());
            let mut v363: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v362 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v363
        }
        US0::US0_0(v33) => {
            let mut v33: US1 = v33.clone();
            let mut v34: i32 = 1i32;
            let mut v35: Rc<str> = usize::try_from(v34).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v36: i32 = 2i32;
            let mut v37: Rc<str> = usize::try_from(v36).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v38: i32 = 3i32;
            let mut v39: Rc<str> = usize::try_from(v38).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v40: i32 = 4i32;
            let mut v41: Rc<str> = usize::try_from(v40).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v42: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
            let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
            let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v43, &*v44, &*v44, &*v44].concat());
            let mut v46: bool = std::path::Path::new(&*v45).is_file();
            let mut v49: US2 = if v46 {
                US2::US2_0(v45.clone())
            } else {
                US2::US2_1(v45.clone())
            };
            match &v49 {
                US2::US2_1(v50) => {
                    let mut v50: Rc<str> = v50.clone();
                    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile edit "); } LIT.with(|lit| lit.clone()) };
                    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v52, &*v37, &*v53, &*v51, &*v44].concat());
                    let mut v55: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v54 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v55
                }
                US2::US2_0(v56) => {
                    let mut v56: Rc<str> = v56.clone();
                    let mut v57: Rc<str> = std::fs::read_to_string(&*v56).unwrap_or_default().into();
                    let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                    let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                    let mut v60: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v37, &*v59, &*v44, &*v44].concat());
                    let mut v61: i32 = v57.find(&*v60).map(|offset| offset as i32).unwrap_or(-1);
                    let mut v62: bool = v61 < 0;
                    if v62 {
                        let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                        let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile edit "); } LIT.with(|lit| lit.clone()) };
                        let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v66: Rc<str> = std::rc::Rc::<str>::from([&*v64, &*v37, &*v65, &*v63, &*v44].concat());
                        let mut v67: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v66 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v67
                    } else {
                        let mut v68: i32 = v57.len() as i32;
                        let mut v69: i32 = v61 + 6i32;
                        let mut v76: i32 = match &v33 {
                            US1::US1_1 => {
                                1i32
                            }
                            US1::US1_5 => {
                                7i32
                            }
                            US1::US1_4 => {
                                4i32
                            }
                            US1::US1_2 => {
                                2i32
                            }
                            US1::US1_3 => {
                                3i32
                            }
                            US1::US1_6 => {
                                8i32
                            }
                            US1::US1_0 => {
                                1i32
                            }
                        };
                        let mut v77: i32 = method1(v57.clone(), v69, v68, v76);
                        let mut v78: US3 = US3::US3_0;
                        let mut v79: i32 = method2(v57.clone(), v77, v68, v78.clone());
                        let mut v80: Rc<str> = std::rc::Rc::<str>::from(v57.get(v77 as usize..v79 as usize).unwrap_or(""));
                        let mut v296: US4 = match &v33 {
                            US1::US1_1 => {
                                let mut v132: i32 = v80.len() as i32;
                                let mut v133: i32 = v132 - 1;
                                let mut v134: Rc<str> = std::rc::Rc::<str>::from(v80.get(0i32 as usize..v133 as usize).unwrap_or(""));
                                let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                let mut v137: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v135, &v136));
                                let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                let mut v140: Rc<str> = std::rc::Rc::<str>::from(v137.replace(&*v138, &v139));
                                let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v143: Rc<str> = std::rc::Rc::<str>::from(v140.replace(&*v141, &v142));
                                let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                let mut v146: Rc<str> = std::rc::Rc::<str>::from(v143.replace(&*v144, &v145));
                                let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                let mut v149: Rc<str> = std::rc::Rc::<str>::from(v146.replace(&*v147, &v148));
                                let mut v150: Rc<str> = std::rc::Rc::<str>::from([&*v134, &*v149, &*v138, &*v44, &*v44].concat());
                                US4::US4_0(v150.clone())
                            }
                            US1::US1_5 => {
                                let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                let mut v101: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v99, &v100));
                                let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                let mut v103: Rc<str> = std::rc::Rc::<str>::from(v101.replace(&*v98, &v102));
                                let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v106: Rc<str> = std::rc::Rc::<str>::from(v103.replace(&*v104, &v105));
                                let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                let mut v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                let mut v109: Rc<str> = std::rc::Rc::<str>::from(v106.replace(&*v107, &v108));
                                let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                let mut v112: Rc<str> = std::rc::Rc::<str>::from(v109.replace(&*v110, &v111));
                                let mut v113: Rc<str> = std::rc::Rc::<str>::from([&*v98, &*v112, &*v98, &*v44, &*v44].concat());
                                US4::US4_0(v113.clone())
                            }
                            US1::US1_4 => {
                                let mut v228: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0|D1|D2|D3|D4|D5|D6|D7|D8"); } LIT.with(|lit| lit.clone()) };
                                let mut v229: u64 = v228.split("|").position(|item| item == &*v41).map(|index| index as u64).unwrap_or(u64::MAX);
                                let mut v230: bool = 0u64 == v229;
                                let mut v266: US5 = if v230 {
                                    let mut v231: US6 = US6::US6_0;
                                    US5::US5_0(v231.clone())
                                } else {
                                    let mut v233: bool = 1u64 == v229;
                                    if v233 {
                                        let mut v234: US6 = US6::US6_1;
                                        US5::US5_0(v234.clone())
                                    } else {
                                        let mut v236: bool = 2u64 == v229;
                                        if v236 {
                                            let mut v237: US6 = US6::US6_2;
                                            US5::US5_0(v237.clone())
                                        } else {
                                            let mut v239: bool = 3u64 == v229;
                                            if v239 {
                                                let mut v240: US6 = US6::US6_3;
                                                US5::US5_0(v240.clone())
                                            } else {
                                                let mut v242: bool = 4u64 == v229;
                                                if v242 {
                                                    let mut v243: US6 = US6::US6_4;
                                                    US5::US5_0(v243.clone())
                                                } else {
                                                    let mut v245: bool = 5u64 == v229;
                                                    if v245 {
                                                        let mut v246: US6 = US6::US6_5;
                                                        US5::US5_0(v246.clone())
                                                    } else {
                                                        let mut v248: bool = 6u64 == v229;
                                                        if v248 {
                                                            let mut v249: US6 = US6::US6_6;
                                                            US5::US5_0(v249.clone())
                                                        } else {
                                                            let mut v251: bool = 7u64 == v229;
                                                            if v251 {
                                                                let mut v252: US6 = US6::US6_7;
                                                                US5::US5_0(v252.clone())
                                                            } else {
                                                                let mut v254: bool = 8u64 == v229;
                                                                if v254 {
                                                                    let mut v255: US6 = US6::US6_8;
                                                                    US5::US5_0(v255.clone())
                                                                } else {
                                                                    US5::US5_1
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                match &v266 {
                                    US5::US5_1 => {
                                        US4::US4_1
                                    }
                                    US5::US5_0(v267) => {
                                        let mut v267: US6 = v267.clone();
                                        let mut v285: Rc<str> = match &v267 {
                                            US6::US6_0 => {
                                                let mut v268: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                                                v268.clone()
                                            }
                                            US6::US6_1 => {
                                                let mut v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                                                v269.clone()
                                            }
                                            US6::US6_2 => {
                                                let mut v270: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                                                v270.clone()
                                            }
                                            US6::US6_3 => {
                                                let mut v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                                                v271.clone()
                                            }
                                            US6::US6_4 => {
                                                let mut v272: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                                                v272.clone()
                                            }
                                            US6::US6_5 => {
                                                let mut v273: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                                                v273.clone()
                                            }
                                            US6::US6_6 => {
                                                let mut v274: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                                                v274.clone()
                                            }
                                            US6::US6_7 => {
                                                let mut v275: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                                                v275.clone()
                                            }
                                            US6::US6_8 => {
                                                let mut v276: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                                                v276.clone()
                                            }
                                        };
                                        US4::US4_0(v285.clone())
                                    }
                                }
                            }
                            US1::US1_2 => {
                                let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic|Feature|Story|TaskKind"); } LIT.with(|lit| lit.clone()) };
                                let mut v153: u64 = v152.split("|").position(|item| item == &*v41).map(|index| index as u64).unwrap_or(u64::MAX);
                                let mut v154: bool = 0u64 == v153;
                                let mut v170: US7 = if v154 {
                                    let mut v155: US8 = US8::US8_0;
                                    US7::US7_0(v155.clone())
                                } else {
                                    let mut v157: bool = 1u64 == v153;
                                    if v157 {
                                        let mut v158: US8 = US8::US8_1;
                                        US7::US7_0(v158.clone())
                                    } else {
                                        let mut v160: bool = 2u64 == v153;
                                        if v160 {
                                            let mut v161: US8 = US8::US8_2;
                                            US7::US7_0(v161.clone())
                                        } else {
                                            let mut v163: bool = 3u64 == v153;
                                            if v163 {
                                                let mut v164: US8 = US8::US8_3;
                                                US7::US7_0(v164.clone())
                                            } else {
                                                US7::US7_1
                                            }
                                        }
                                    }
                                };
                                match &v170 {
                                    US7::US7_1 => {
                                        US4::US4_1
                                    }
                                    US7::US7_0(v171) => {
                                        let mut v171: US8 = v171.clone();
                                        let mut v179: Rc<str> = match &v171 {
                                            US8::US8_0 => {
                                                let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                                                v172.clone()
                                            }
                                            US8::US8_1 => {
                                                let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                                                v173.clone()
                                            }
                                            US8::US8_2 => {
                                                let mut v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                                                v174.clone()
                                            }
                                            US8::US8_3 => {
                                                let mut v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                                                v175.clone()
                                            }
                                        };
                                        US4::US4_0(v179.clone())
                                    }
                                }
                            }
                            US1::US1_3 => {
                                let mut v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0|P1|P2|P3|P4|P5"); } LIT.with(|lit| lit.clone()) };
                                let mut v185: u64 = v184.split("|").position(|item| item == &*v41).map(|index| index as u64).unwrap_or(u64::MAX);
                                let mut v186: bool = 0u64 == v185;
                                let mut v210: US9 = if v186 {
                                    let mut v187: US10 = US10::US10_0;
                                    US9::US9_0(v187.clone())
                                } else {
                                    let mut v189: bool = 1u64 == v185;
                                    if v189 {
                                        let mut v190: US10 = US10::US10_1;
                                        US9::US9_0(v190.clone())
                                    } else {
                                        let mut v192: bool = 2u64 == v185;
                                        if v192 {
                                            let mut v193: US10 = US10::US10_2;
                                            US9::US9_0(v193.clone())
                                        } else {
                                            let mut v195: bool = 3u64 == v185;
                                            if v195 {
                                                let mut v196: US10 = US10::US10_3;
                                                US9::US9_0(v196.clone())
                                            } else {
                                                let mut v198: bool = 4u64 == v185;
                                                if v198 {
                                                    let mut v199: US10 = US10::US10_4;
                                                    US9::US9_0(v199.clone())
                                                } else {
                                                    let mut v201: bool = 5u64 == v185;
                                                    if v201 {
                                                        let mut v202: US10 = US10::US10_5;
                                                        US9::US9_0(v202.clone())
                                                    } else {
                                                        US9::US9_1
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                match &v210 {
                                    US9::US9_1 => {
                                        US4::US4_1
                                    }
                                    US9::US9_0(v211) => {
                                        let mut v211: US10 = v211.clone();
                                        let mut v223: Rc<str> = match &v211 {
                                            US10::US10_0 => {
                                                let mut v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                                                v212.clone()
                                            }
                                            US10::US10_1 => {
                                                let mut v213: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                                                v213.clone()
                                            }
                                            US10::US10_2 => {
                                                let mut v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                                                v214.clone()
                                            }
                                            US10::US10_3 => {
                                                let mut v215: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                                                v215.clone()
                                            }
                                            US10::US10_4 => {
                                                let mut v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                                                v216.clone()
                                            }
                                            US10::US10_5 => {
                                                let mut v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                                                v217.clone()
                                            }
                                        };
                                        US4::US4_0(v223.clone())
                                    }
                                }
                            }
                            US1::US1_6 => {
                                let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                let mut v118: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v116, &v117));
                                let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                let mut v120: Rc<str> = std::rc::Rc::<str>::from(v118.replace(&*v115, &v119));
                                let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v123: Rc<str> = std::rc::Rc::<str>::from(v120.replace(&*v121, &v122));
                                let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                let mut v126: Rc<str> = std::rc::Rc::<str>::from(v123.replace(&*v124, &v125));
                                let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                let mut v129: Rc<str> = std::rc::Rc::<str>::from(v126.replace(&*v127, &v128));
                                let mut v130: Rc<str> = std::rc::Rc::<str>::from([&*v115, &*v129, &*v115, &*v44, &*v44].concat());
                                US4::US4_0(v130.clone())
                            }
                            US1::US1_0 => {
                                let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                let mut v84: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v82, &v83));
                                let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                let mut v86: Rc<str> = std::rc::Rc::<str>::from(v84.replace(&*v81, &v85));
                                let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v89: Rc<str> = std::rc::Rc::<str>::from(v86.replace(&*v87, &v88));
                                let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                let mut v92: Rc<str> = std::rc::Rc::<str>::from(v89.replace(&*v90, &v91));
                                let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                let mut v95: Rc<str> = std::rc::Rc::<str>::from(v92.replace(&*v93, &v94));
                                let mut v96: Rc<str> = std::rc::Rc::<str>::from([&*v81, &*v95, &*v81, &*v44, &*v44].concat());
                                US4::US4_0(v96.clone())
                            }
                        };
                        match &v296 {
                            US4::US4_1 => {
                                let mut v303: Rc<str> = match &v33 {
                                    US1::US1_4 => {
                                        let mut v299: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                                        v299.clone()
                                    }
                                    US1::US1_2 => {
                                        let mut v297: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                                        v297.clone()
                                    }
                                    US1::US1_3 => {
                                        let mut v298: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                                        v298.clone()
                                    }
                                    _ => {
                                        let mut v300: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the value was rejected"); } LIT.with(|lit| lit.clone()) };
                                        v300.clone()
                                    }
                                };
                                let mut v304: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile edit "); } LIT.with(|lit| lit.clone()) };
                                let mut v305: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let mut v306: Rc<str> = std::rc::Rc::<str>::from([&*v304, &*v37, &*v305, &*v303, &*v44].concat());
                                let mut v307: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v306 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v307
                            }
                            US4::US4_0(v308) => {
                                let mut v308: Rc<str> = v308.clone();
                                let mut v309: Rc<str> = std::rc::Rc::<str>::from(v57.get(0i32 as usize..v77 as usize).unwrap_or(""));
                                let mut v310: Rc<str> = std::rc::Rc::<str>::from(v57.get(v79 as usize..v68 as usize).unwrap_or(""));
                                let mut v311: Rc<str> = std::rc::Rc::<str>::from([&*v309, &*v308, &*v310, &*v44, &*v44].concat());
                                let mut v312: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                                let mut v313: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v312, &*v44, &*v44, &*v44].concat());
                                let mut v314: bool = std::fs::write(&*v313, &*v311).is_ok();
                                let mut v323: US11 = if v314 {
                                    let mut v315: bool = std::fs::rename(&*v313, &*v45).is_ok();
                                    if v315 {
                                        let mut v316: bool = std::fs::read_to_string(&*v45).is_ok_and(|value| value == *v311);
                                        if v316 {
                                            US11::US11_0
                                        } else {
                                            US11::US11_3
                                        }
                                    } else {
                                        US11::US11_2
                                    }
                                } else {
                                    US11::US11_1
                                };
                                match &v323 {
                                    US11::US11_0 => {
                                        let mut v324: u64 = v57.len() as u64;
                                        let mut v325: u64 = v311.len() as u64;
                                        let mut v326: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v42);
                                        let mut v327: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile edit ok id="); } LIT.with(|lit| lit.clone()) };
                                        let mut v328: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" field="); } LIT.with(|lit| lit.clone()) };
                                        let mut v329: Rc<str> = std::rc::Rc::<str>::from([&*v327, &*v37, &*v328, &*v39, &*v44].concat());
                                        let mut v330: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                        let mut v331: Rc<str> = std::rc::Rc::<str>::from((v324).to_string());
                                        let mut v332: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                        let mut v333: Rc<str> = std::rc::Rc::<str>::from((v325).to_string());
                                        let mut v334: Rc<str> = std::rc::Rc::<str>::from([&*v330, &*v331, &*v332, &*v333, &*v44].concat());
                                        let mut v335: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                        let mut v336: Rc<str> = std::rc::Rc::<str>::from((v326).to_string());
                                        let mut v337: Rc<str> = std::rc::Rc::<str>::from([&*v334, &*v335, &*v336, &*v44, &*v44].concat());
                                        let mut v338: Rc<str> = std::rc::Rc::<str>::from([&*v329, &*v337, &*v44, &*v44, &*v44].concat());
                                        let mut v339: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v338.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                        v339
                                    }
                                    _ => {
                                        let mut v345: Rc<str> = match &v323 {
                                            US11::US11_3 => {
                                                let mut v342: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                v342.clone()
                                            }
                                            US11::US11_2 => {
                                                let mut v341: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                v341.clone()
                                            }
                                            US11::US11_1 => {
                                                let mut v340: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                v340.clone()
                                            }
                                            _ => unreachable!(),
                                        };
                                        let mut v346: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile edit "); } LIT.with(|lit| lit.clone()) };
                                        let mut v347: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                        let mut v348: Rc<str> = std::rc::Rc::<str>::from([&*v346, &*v37, &*v347, &*v345, &*v44].concat());
                                        let mut v349: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v348 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v349
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
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        method0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_edit_run() -> i32 {
    closure0()()
}
