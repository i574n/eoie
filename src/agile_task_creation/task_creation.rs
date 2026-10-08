#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_0(Rc<str>),
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
    US2_0,
    US2_1,
    US2_2,
    US2_3,
}
impl US2 {
    fn tag(&self) -> i32 {
        match self {
            US2::US2_0 => 0,
            US2::US2_1 => 1,
            US2::US2_2 => 2,
            US2::US2_3 => 3,
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
    US4_1,
    US4_2,
    US4_3,
    US4_4,
    US4_5,
}
impl US4 {
    fn tag(&self) -> i32 {
        match self {
            US4::US4_0 => 0,
            US4::US4_1 => 1,
            US4::US4_2 => 2,
            US4::US4_3 => 3,
            US4::US4_4 => 4,
            US4::US4_5 => 5,
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
enum US7 {
    US7_0(Rc<str>),
    US7_1(Rc<str>),
}
impl US7 {
    fn tag(&self) -> i32 {
        match self {
            US7::US7_0(..) => 0,
            US7::US7_1(..) => 1,
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
fn method1(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let mut v2: bool = v0 == v1;
        if v2 {
            return true;
        } else {
            let mut v3: i32 = 2i32;
            let mut v4: i32 = usize::try_from(v3).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v0).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
            let mut v5: bool = 65i32 <= v4;
            let mut v7: bool = if v5 {
                let mut v6: bool = v4 <= 90i32;
                v6
            } else {
                false
            };
            let mut v13: bool = if v7 {
                true
            } else {
                let mut v8: bool = 48i32 <= v4;
                let mut v10: bool = if v8 {
                    let mut v9: bool = v4 <= 57i32;
                    v9
                } else {
                    false
                };
                if v10 {
                    true
                } else {
                    let mut v11: bool = v4 == 45i32;
                    v11
                }
            };
            if v13 {
                let mut v14: i32 = v0 + 1;
                (v0, v1) = (v14, v1);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn method0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
    let mut v2: bool = 0 < v1;
    let mut v11: US0 = if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: bool = method1(v3, v1);
        if v4 {
            let mut v5: i32 = 2i32;
            let mut v6: Rc<str> = usize::try_from(v5).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            US0::US0_0(v6.clone())
        } else {
            US0::US0_1
        }
    } else {
        US0::US0_1
    };
    let mut v12: i32 = 3i32;
    let mut v13: Rc<str> = usize::try_from(v12).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic|Feature|Story|TaskKind"); } LIT.with(|lit| lit.clone()) };
    let mut v15: u64 = v14.split("|").position(|item| item == &*v13).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v16: bool = 0u64 == v15;
    let mut v32: US1 = if v16 {
        let mut v17: US2 = US2::US2_0;
        US1::US1_0(v17.clone())
    } else {
        let mut v19: bool = 1u64 == v15;
        if v19 {
            let mut v20: US2 = US2::US2_1;
            US1::US1_0(v20.clone())
        } else {
            let mut v22: bool = 2u64 == v15;
            if v22 {
                let mut v23: US2 = US2::US2_2;
                US1::US1_0(v23.clone())
            } else {
                let mut v25: bool = 3u64 == v15;
                if v25 {
                    let mut v26: US2 = US2::US2_3;
                    US1::US1_0(v26.clone())
                } else {
                    US1::US1_1
                }
            }
        }
    };
    let mut v33: i32 = 4i32;
    let mut v34: Rc<str> = usize::try_from(v33).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0|P1|P2|P3|P4|P5"); } LIT.with(|lit| lit.clone()) };
    let mut v36: u64 = v35.split("|").position(|item| item == &*v34).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v37: bool = 0u64 == v36;
    let mut v61: US3 = if v37 {
        let mut v38: US4 = US4::US4_0;
        US3::US3_0(v38.clone())
    } else {
        let mut v40: bool = 1u64 == v36;
        if v40 {
            let mut v41: US4 = US4::US4_1;
            US3::US3_0(v41.clone())
        } else {
            let mut v43: bool = 2u64 == v36;
            if v43 {
                let mut v44: US4 = US4::US4_2;
                US3::US3_0(v44.clone())
            } else {
                let mut v46: bool = 3u64 == v36;
                if v46 {
                    let mut v47: US4 = US4::US4_3;
                    US3::US3_0(v47.clone())
                } else {
                    let mut v49: bool = 4u64 == v36;
                    if v49 {
                        let mut v50: US4 = US4::US4_4;
                        US3::US3_0(v50.clone())
                    } else {
                        let mut v52: bool = 5u64 == v36;
                        if v52 {
                            let mut v53: US4 = US4::US4_5;
                            US3::US3_0(v53.clone())
                        } else {
                            US3::US3_1
                        }
                    }
                }
            }
        }
    };
    let mut v62: i32 = 5i32;
    let mut v63: Rc<str> = usize::try_from(v62).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0|D1|D2|D3|D4|D5|D6|D7|D8"); } LIT.with(|lit| lit.clone()) };
    let mut v65: u64 = v64.split("|").position(|item| item == &*v63).map(|index| index as u64).unwrap_or(u64::MAX);
    let mut v66: bool = 0u64 == v65;
    let mut v102: US5 = if v66 {
        let mut v67: US6 = US6::US6_0;
        US5::US5_0(v67.clone())
    } else {
        let mut v69: bool = 1u64 == v65;
        if v69 {
            let mut v70: US6 = US6::US6_1;
            US5::US5_0(v70.clone())
        } else {
            let mut v72: bool = 2u64 == v65;
            if v72 {
                let mut v73: US6 = US6::US6_2;
                US5::US5_0(v73.clone())
            } else {
                let mut v75: bool = 3u64 == v65;
                if v75 {
                    let mut v76: US6 = US6::US6_3;
                    US5::US5_0(v76.clone())
                } else {
                    let mut v78: bool = 4u64 == v65;
                    if v78 {
                        let mut v79: US6 = US6::US6_4;
                        US5::US5_0(v79.clone())
                    } else {
                        let mut v81: bool = 5u64 == v65;
                        if v81 {
                            let mut v82: US6 = US6::US6_5;
                            US5::US5_0(v82.clone())
                        } else {
                            let mut v84: bool = 6u64 == v65;
                            if v84 {
                                let mut v85: US6 = US6::US6_6;
                                US5::US5_0(v85.clone())
                            } else {
                                let mut v87: bool = 7u64 == v65;
                                if v87 {
                                    let mut v88: US6 = US6::US6_7;
                                    US5::US5_0(v88.clone())
                                } else {
                                    let mut v90: bool = 8u64 == v65;
                                    if v90 {
                                        let mut v91: US6 = US6::US6_8;
                                        US5::US5_0(v91.clone())
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
    match &v11 {
        US0::US0_1 => {
            let mut v294: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must be uppercase letters, digits and dashes"); } LIT.with(|lit| lit.clone()) };
            let mut v295: i32 = 2i32;
            let mut v296: Rc<str> = usize::try_from(v295).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v297: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
            let mut v298: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v299: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v300: Rc<str> = std::rc::Rc::<str>::from([&*v297, &*v296, &*v298, &*v294, &*v299].concat());
            let mut v301: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v300 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v301
        }
        US0::US0_0(v103) => {
            let mut v103: Rc<str> = v103.clone();
            match &v32 {
                US1::US1_1 => {
                    let mut v284: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                    let mut v285: i32 = 2i32;
                    let mut v286: Rc<str> = usize::try_from(v285).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v287: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                    let mut v288: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v289: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v290: Rc<str> = std::rc::Rc::<str>::from([&*v287, &*v286, &*v288, &*v284, &*v289].concat());
                    let mut v291: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v290 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v291
                }
                US1::US1_0(v104) => {
                    let mut v104: US2 = v104.clone();
                    match &v61 {
                        US3::US3_1 => {
                            let mut v274: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                            let mut v275: i32 = 2i32;
                            let mut v276: Rc<str> = usize::try_from(v275).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v277: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                            let mut v278: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let mut v279: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v280: Rc<str> = std::rc::Rc::<str>::from([&*v277, &*v276, &*v278, &*v274, &*v279].concat());
                            let mut v281: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v280 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v281
                        }
                        US3::US3_0(v105) => {
                            let mut v105: US4 = v105.clone();
                            match &v102 {
                                US5::US5_1 => {
                                    let mut v264: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                                    let mut v265: i32 = 2i32;
                                    let mut v266: Rc<str> = usize::try_from(v265).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v267: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                    let mut v268: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                    let mut v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v270: Rc<str> = std::rc::Rc::<str>::from([&*v267, &*v266, &*v268, &*v264, &*v269].concat());
                                    let mut v271: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v270 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v271
                                }
                                US5::US5_0(v106) => {
                                    let mut v106: US6 = v106.clone();
                                    let mut v107: i32 = 1i32;
                                    let mut v108: Rc<str> = usize::try_from(v107).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v109: i32 = 6i32;
                                    let mut v110: Rc<str> = usize::try_from(v109).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v111: i32 = 7i32;
                                    let mut v112: Rc<str> = usize::try_from(v111).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v113: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                    let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                    let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v116: Rc<str> = std::rc::Rc::<str>::from([&*v108, &*v114, &*v115, &*v115, &*v115].concat());
                                    let mut v117: bool = std::path::Path::new(&*v116).is_file();
                                    let mut v120: US7 = if v117 {
                                        US7::US7_0(v116.clone())
                                    } else {
                                        US7::US7_1(v116.clone())
                                    };
                                    match &v120 {
                                        US7::US7_1(v121) => {
                                            let mut v121: Rc<str> = v121.clone();
                                            let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                            let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                            let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                            let mut v125: Rc<str> = std::rc::Rc::<str>::from([&*v123, &*v103, &*v124, &*v122, &*v115].concat());
                                            let mut v126: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v125 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                            v126
                                        }
                                        US7::US7_0(v127) => {
                                            let mut v127: Rc<str> = v127.clone();
                                            let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                                            let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                                            let mut v130: Rc<str> = std::rc::Rc::<str>::from([&*v128, &*v103, &*v129, &*v115, &*v115].concat());
                                            let mut v131: bool = std::fs::read_to_string(&*v116).is_ok_and(|value| value.contains(v130.as_ref()));
                                            if v131 {
                                                let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                                                let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                let mut v135: Rc<str> = std::rc::Rc::<str>::from([&*v133, &*v103, &*v134, &*v132, &*v115].concat());
                                                let mut v136: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v135 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                v136
                                            } else {
                                                let mut v137: Rc<str> = std::fs::read_to_string(&*v127).unwrap_or_default().into();
                                                let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                let mut v139: Rc<str> = std::rc::Rc::<str>::from(v137.trim_end_matches(&*v138));
                                                let mut v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                                                let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                                let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                                                let mut v143: Rc<str> = std::rc::Rc::<str>::from(v103.replace(&*v141, &v142));
                                                let mut v144: Rc<str> = std::rc::Rc::<str>::from(v143.to_ascii_lowercase());
                                                let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Task ("); } LIT.with(|lit| lit.clone()) };
                                                let mut v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                                let mut v149: Rc<str> = std::rc::Rc::<str>::from(v103.replace(&*v147, &v148));
                                                let mut v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                                let mut v151: Rc<str> = std::rc::Rc::<str>::from(v149.replace(&*v146, &v150));
                                                let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                                let mut v153: Rc<str> = std::rc::Rc::<str>::from(v151.replace(&*v138, &v152));
                                                let mut v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                                let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                                let mut v156: Rc<str> = std::rc::Rc::<str>::from(v153.replace(&*v154, &v155));
                                                let mut v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                                let mut v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                                let mut v159: Rc<str> = std::rc::Rc::<str>::from(v156.replace(&*v157, &v158));
                                                let mut v160: Rc<str> = std::rc::Rc::<str>::from([&*v146, &*v159, &*v146, &*v115, &*v115].concat());
                                                let mut v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                                let mut v162: Rc<str> = std::rc::Rc::<str>::from([&*v140, &*v144, &*v145, &*v160, &*v161].concat());
                                                let mut v163: Rc<str> = std::rc::Rc::<str>::from(v110.replace(&*v147, &v148));
                                                let mut v164: Rc<str> = std::rc::Rc::<str>::from(v163.replace(&*v146, &v150));
                                                let mut v165: Rc<str> = std::rc::Rc::<str>::from(v164.replace(&*v138, &v152));
                                                let mut v166: Rc<str> = std::rc::Rc::<str>::from(v165.replace(&*v154, &v155));
                                                let mut v167: Rc<str> = std::rc::Rc::<str>::from(v166.replace(&*v157, &v158));
                                                let mut v168: Rc<str> = std::rc::Rc::<str>::from([&*v146, &*v167, &*v146, &*v115, &*v115].concat());
                                                let mut v176: Rc<str> = match &v104 {
                                                    US2::US2_0 => {
                                                        let mut v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                                                        v169.clone()
                                                    }
                                                    US2::US2_1 => {
                                                        let mut v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                                                        v170.clone()
                                                    }
                                                    US2::US2_2 => {
                                                        let mut v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                                                        v171.clone()
                                                    }
                                                    US2::US2_3 => {
                                                        let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                                                        v172.clone()
                                                    }
                                                };
                                                let mut v188: Rc<str> = match &v105 {
                                                    US4::US4_0 => {
                                                        let mut v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                                                        v177.clone()
                                                    }
                                                    US4::US4_1 => {
                                                        let mut v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                                                        v178.clone()
                                                    }
                                                    US4::US4_2 => {
                                                        let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                                                        v179.clone()
                                                    }
                                                    US4::US4_3 => {
                                                        let mut v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                                                        v180.clone()
                                                    }
                                                    US4::US4_4 => {
                                                        let mut v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                                                        v181.clone()
                                                    }
                                                    US4::US4_5 => {
                                                        let mut v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                                                        v182.clone()
                                                    }
                                                };
                                                let mut v189: Rc<str> = std::rc::Rc::<str>::from([&*v168, &*v161, &*v176, &*v161, &*v188].concat());
                                                let mut v207: Rc<str> = match &v106 {
                                                    US6::US6_0 => {
                                                        let mut v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                                                        v190.clone()
                                                    }
                                                    US6::US6_1 => {
                                                        let mut v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                                                        v191.clone()
                                                    }
                                                    US6::US6_2 => {
                                                        let mut v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                                                        v192.clone()
                                                    }
                                                    US6::US6_3 => {
                                                        let mut v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                                                        v193.clone()
                                                    }
                                                    US6::US6_4 => {
                                                        let mut v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                                                        v194.clone()
                                                    }
                                                    US6::US6_5 => {
                                                        let mut v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                                                        v195.clone()
                                                    }
                                                    US6::US6_6 => {
                                                        let mut v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                                                        v196.clone()
                                                    }
                                                    US6::US6_7 => {
                                                        let mut v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                                                        v197.clone()
                                                    }
                                                    US6::US6_8 => {
                                                        let mut v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                                                        v198.clone()
                                                    }
                                                };
                                                let mut v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", 0u32, Planned, \"\", "); } LIT.with(|lit| lit.clone()) };
                                                let mut v209: Rc<str> = std::rc::Rc::<str>::from(v112.replace(&*v147, &v148));
                                                let mut v210: Rc<str> = std::rc::Rc::<str>::from(v209.replace(&*v146, &v150));
                                                let mut v211: Rc<str> = std::rc::Rc::<str>::from(v210.replace(&*v138, &v152));
                                                let mut v212: Rc<str> = std::rc::Rc::<str>::from(v211.replace(&*v154, &v155));
                                                let mut v213: Rc<str> = std::rc::Rc::<str>::from(v212.replace(&*v157, &v158));
                                                let mut v214: Rc<str> = std::rc::Rc::<str>::from([&*v146, &*v213, &*v146, &*v115, &*v115].concat());
                                                let mut v215: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", \"current\", \"current\")"); } LIT.with(|lit| lit.clone()) };
                                                let mut v216: Rc<str> = std::rc::Rc::<str>::from([&*v161, &*v207, &*v208, &*v214, &*v215].concat());
                                                let mut v217: Rc<str> = std::rc::Rc::<str>::from([&*v162, &*v189, &*v216, &*v115, &*v115].concat());
                                                let mut v218: Rc<str> = std::rc::Rc::<str>::from([&*v139, &*v138, &*v217, &*v138, &*v115].concat());
                                                let mut v219: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                                                let mut v220: Rc<str> = std::rc::Rc::<str>::from([&*v116, &*v219, &*v115, &*v115, &*v115].concat());
                                                let mut v221: bool = std::fs::write(&*v220, &*v218).is_ok();
                                                let mut v230: US8 = if v221 {
                                                    let mut v222: bool = std::fs::rename(&*v220, &*v116).is_ok();
                                                    if v222 {
                                                        let mut v223: bool = std::fs::read_to_string(&*v116).is_ok_and(|value| value == *v218);
                                                        if v223 {
                                                            US8::US8_0
                                                        } else {
                                                            US8::US8_3
                                                        }
                                                    } else {
                                                        US8::US8_2
                                                    }
                                                } else {
                                                    US8::US8_1
                                                };
                                                match &v230 {
                                                    US8::US8_0 => {
                                                        let mut v231: Rc<str> = std::rc::Rc::<str>::from(v103.replace(&*v141, &v142));
                                                        let mut v232: Rc<str> = std::rc::Rc::<str>::from(v231.to_ascii_lowercase());
                                                        let mut v233: u64 = v137.len() as u64;
                                                        let mut v234: u64 = v218.len() as u64;
                                                        let mut v235: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v113);
                                                        let mut v236: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile add ok id="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" binding="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v238: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status=Planned progress=0"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v239: Rc<str> = std::rc::Rc::<str>::from([&*v236, &*v103, &*v237, &*v232, &*v238].concat());
                                                        let mut v240: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v241: Rc<str> = std::rc::Rc::<str>::from((v233).to_string());
                                                        let mut v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v243: Rc<str> = std::rc::Rc::<str>::from((v234).to_string());
                                                        let mut v244: Rc<str> = std::rc::Rc::<str>::from([&*v240, &*v241, &*v242, &*v243, &*v115].concat());
                                                        let mut v245: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                        let mut v246: Rc<str> = std::rc::Rc::<str>::from((v235).to_string());
                                                        let mut v247: Rc<str> = std::rc::Rc::<str>::from([&*v244, &*v245, &*v246, &*v115, &*v115].concat());
                                                        let mut v248: Rc<str> = std::rc::Rc::<str>::from([&*v239, &*v247, &*v115, &*v115, &*v115].concat());
                                                        let mut v249: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v248.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                                        v249
                                                    }
                                                    _ => {
                                                        let mut v255: Rc<str> = match &v230 {
                                                            US8::US8_3 => {
                                                                let mut v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                v252.clone()
                                                            }
                                                            US8::US8_2 => {
                                                                let mut v251: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                v251.clone()
                                                            }
                                                            US8::US8_1 => {
                                                                let mut v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                v250.clone()
                                                            }
                                                            _ => unreachable!(),
                                                        };
                                                        let mut v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                        let mut v257: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                        let mut v258: Rc<str> = std::rc::Rc::<str>::from([&*v256, &*v103, &*v257, &*v255, &*v115].concat());
                                                        let mut v259: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v258 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                        v259
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
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        method0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_add_run() -> i32 {
    closure0()()
}
