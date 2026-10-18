#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Valid(Rc<str>),
    US0_Invalid,
}
#[derive(Clone)]
enum US2 {
    US2_AgileEpic,
    US2_AgileFeature,
    US2_AgileStory,
    US2_AgileTaskKind,
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
#[derive(Clone)]
enum US4 {
    US4_AgileP0,
    US4_AgileP1,
    US4_AgileP2,
    US4_AgileP3,
    US4_AgileP4,
    US4_AgileP5,
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US4),
    US3_Invalid,
}
#[derive(Clone)]
enum US6 {
    US6_AgileD0,
    US6_AgileD1,
    US6_AgileD2,
    US6_AgileD3,
    US6_AgileD4,
    US6_AgileD5,
    US6_AgileD6,
    US6_AgileD7,
    US6_AgileD8,
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US6),
    US5_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_FileReadable(Rc<str>),
    US7_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US8 {
    US8_Replaced,
    US8_TemporaryWriteFailed,
    US8_RenameFailed,
    US8_ReadbackMismatch,
}
fn task_id_bytes_allowed_1(mut v0: i32, mut v1: i32) -> bool {
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
                let mut v14: i32 = v0.wrapping_add(1i32);
                (v0, v1) = (v14, v1);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn agile_add_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = 0i32;
    let mut v2: i32 = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v1).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
    let mut v3: bool = 48i32 <= v2;
    let mut v5: bool = if v3 {
        let mut v4: bool = v2 <= 57i32;
        v4
    } else {
        false
    };
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must start with a letter, so its inl name is a valid identifier"); } LIT.with(|lit| lit.clone()) };
        let mut v7: i32 = 2i32;
        let mut v8: Rc<str> = usize::try_from(v7).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v8, &*v10, &*v6, &*v11].concat());
        let mut v13: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v12 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v13
    } else {
        let mut v14: i32 = 2i32;
        let mut v15: i32 = usize::try_from(v14).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
        let mut v16: bool = 0 < v15;
        let mut v25: US0 = if v16 {
            let mut v17: i32 = 0i32;
            let mut v18: bool = task_id_bytes_allowed_1(v17, v15);
            if v18 {
                let mut v19: i32 = 2i32;
                let mut v20: Rc<str> = usize::try_from(v19).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                US0::US0_Valid(v20.clone())
            } else {
                US0::US0_Invalid
            }
        } else {
            US0::US0_Invalid
        };
        let mut v26: i32 = 3i32;
        let mut v27: Rc<str> = usize::try_from(v26).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
        let mut v29: bool = v27 == v28 ;
        let mut v33: US1 = if v29 {
            let mut v30: US2 = US2::US2_AgileEpic;
            US1::US1_Valid(v30.clone())
        } else {
            US1::US1_Invalid
        };
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
        let mut v35: bool = v27 == v34 ;
        let mut v38: US1 = if v35 {
            let mut v36: US2 = US2::US2_AgileFeature;
            US1::US1_Valid(v36.clone())
        } else {
            v33.clone()
        };
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
        let mut v40: bool = v27 == v39 ;
        let mut v43: US1 = if v40 {
            let mut v41: US2 = US2::US2_AgileStory;
            US1::US1_Valid(v41.clone())
        } else {
            v38.clone()
        };
        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
        let mut v45: bool = v27 == v44 ;
        let mut v48: US1 = if v45 {
            let mut v46: US2 = US2::US2_AgileTaskKind;
            US1::US1_Valid(v46.clone())
        } else {
            v43.clone()
        };
        let mut v49: i32 = 4i32;
        let mut v50: Rc<str> = usize::try_from(v49).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
        let mut v52: bool = v50 == v51 ;
        let mut v56: US3 = if v52 {
            let mut v53: US4 = US4::US4_AgileP0;
            US3::US3_Valid(v53.clone())
        } else {
            US3::US3_Invalid
        };
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
        let mut v58: bool = v50 == v57 ;
        let mut v61: US3 = if v58 {
            let mut v59: US4 = US4::US4_AgileP1;
            US3::US3_Valid(v59.clone())
        } else {
            v56.clone()
        };
        let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
        let mut v63: bool = v50 == v62 ;
        let mut v66: US3 = if v63 {
            let mut v64: US4 = US4::US4_AgileP2;
            US3::US3_Valid(v64.clone())
        } else {
            v61.clone()
        };
        let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
        let mut v68: bool = v50 == v67 ;
        let mut v71: US3 = if v68 {
            let mut v69: US4 = US4::US4_AgileP3;
            US3::US3_Valid(v69.clone())
        } else {
            v66.clone()
        };
        let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
        let mut v73: bool = v50 == v72 ;
        let mut v76: US3 = if v73 {
            let mut v74: US4 = US4::US4_AgileP4;
            US3::US3_Valid(v74.clone())
        } else {
            v71.clone()
        };
        let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
        let mut v78: bool = v50 == v77 ;
        let mut v81: US3 = if v78 {
            let mut v79: US4 = US4::US4_AgileP5;
            US3::US3_Valid(v79.clone())
        } else {
            v76.clone()
        };
        let mut v82: i32 = 5i32;
        let mut v83: Rc<str> = usize::try_from(v82).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
        let mut v85: bool = v83 == v84 ;
        let mut v89: US5 = if v85 {
            let mut v86: US6 = US6::US6_AgileD0;
            US5::US5_Valid(v86.clone())
        } else {
            US5::US5_Invalid
        };
        let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
        let mut v91: bool = v83 == v90 ;
        let mut v94: US5 = if v91 {
            let mut v92: US6 = US6::US6_AgileD1;
            US5::US5_Valid(v92.clone())
        } else {
            v89.clone()
        };
        let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
        let mut v96: bool = v83 == v95 ;
        let mut v99: US5 = if v96 {
            let mut v97: US6 = US6::US6_AgileD2;
            US5::US5_Valid(v97.clone())
        } else {
            v94.clone()
        };
        let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
        let mut v101: bool = v83 == v100 ;
        let mut v104: US5 = if v101 {
            let mut v102: US6 = US6::US6_AgileD3;
            US5::US5_Valid(v102.clone())
        } else {
            v99.clone()
        };
        let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
        let mut v106: bool = v83 == v105 ;
        let mut v109: US5 = if v106 {
            let mut v107: US6 = US6::US6_AgileD4;
            US5::US5_Valid(v107.clone())
        } else {
            v104.clone()
        };
        let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
        let mut v111: bool = v83 == v110 ;
        let mut v114: US5 = if v111 {
            let mut v112: US6 = US6::US6_AgileD5;
            US5::US5_Valid(v112.clone())
        } else {
            v109.clone()
        };
        let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
        let mut v116: bool = v83 == v115 ;
        let mut v119: US5 = if v116 {
            let mut v117: US6 = US6::US6_AgileD6;
            US5::US5_Valid(v117.clone())
        } else {
            v114.clone()
        };
        let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
        let mut v121: bool = v83 == v120 ;
        let mut v124: US5 = if v121 {
            let mut v122: US6 = US6::US6_AgileD7;
            US5::US5_Valid(v122.clone())
        } else {
            v119.clone()
        };
        let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
        let mut v126: bool = v83 == v125 ;
        let mut v129: US5 = if v126 {
            let mut v127: US6 = US6::US6_AgileD8;
            US5::US5_Valid(v127.clone())
        } else {
            v124.clone()
        };
        match &v25 {
            US0::US0_Invalid => {
                let mut v286: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must be uppercase letters, digits and dashes"); } LIT.with(|lit| lit.clone()) };
                let mut v287: i32 = 2i32;
                let mut v288: Rc<str> = usize::try_from(v287).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v289: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                let mut v290: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v291: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v292: Rc<str> = std::rc::Rc::<str>::from([&*v289, &*v288, &*v290, &*v286, &*v291].concat());
                let mut v293: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v292 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v293
            }
            US0::US0_Valid(v130) => {
                let mut v130: Rc<str> = v130.clone();
                match &v48 {
                    US1::US1_Invalid => {
                        let mut v276: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                        let mut v277: i32 = 2i32;
                        let mut v278: Rc<str> = usize::try_from(v277).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v279: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                        let mut v280: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v281: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v282: Rc<str> = std::rc::Rc::<str>::from([&*v279, &*v278, &*v280, &*v276, &*v281].concat());
                        let mut v283: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v282 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v283
                    }
                    US1::US1_Valid(v131) => {
                        let mut v131: US2 = v131.clone();
                        match &v81 {
                            US3::US3_Invalid => {
                                let mut v266: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                                let mut v267: i32 = 2i32;
                                let mut v268: Rc<str> = usize::try_from(v267).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                let mut v270: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let mut v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v272: Rc<str> = std::rc::Rc::<str>::from([&*v269, &*v268, &*v270, &*v266, &*v271].concat());
                                let mut v273: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v272 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v273
                            }
                            US3::US3_Valid(v132) => {
                                let mut v132: US4 = v132.clone();
                                match &v129 {
                                    US5::US5_Invalid => {
                                        let mut v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                                        let mut v257: i32 = 2i32;
                                        let mut v258: Rc<str> = usize::try_from(v257).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                        let mut v260: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                        let mut v261: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v262: Rc<str> = std::rc::Rc::<str>::from([&*v259, &*v258, &*v260, &*v256, &*v261].concat());
                                        let mut v263: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v262 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v263
                                    }
                                    US5::US5_Valid(v133) => {
                                        let mut v133: US6 = v133.clone();
                                        let mut v134: i32 = 1i32;
                                        let mut v135: Rc<str> = usize::try_from(v134).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v136: i32 = 6i32;
                                        let mut v137: Rc<str> = usize::try_from(v136).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v138: i32 = 7i32;
                                        let mut v139: Rc<str> = usize::try_from(v138).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v140: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                        let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                        let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v143: Rc<str> = std::rc::Rc::<str>::from([&*v135, &*v141, &*v142, &*v142, &*v142].concat());
                                        let mut v144: bool = std::path::Path::new(&*v143).is_file();
                                        let mut v147: US7 = if v144 {
                                            US7::US7_FileReadable(v143.clone())
                                        } else {
                                            US7::US7_FileMissing(v143.clone())
                                        };
                                        match &v147 {
                                            US7::US7_FileMissing(v148) => {
                                                let mut v148: Rc<str> = v148.clone();
                                                let mut v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                                let mut v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                let mut v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                let mut v152: Rc<str> = std::rc::Rc::<str>::from([&*v150, &*v130, &*v151, &*v149, &*v142].concat());
                                                let mut v153: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v152 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                v153
                                            }
                                            US7::US7_FileReadable(v154) => {
                                                let mut v154: Rc<str> = v154.clone();
                                                let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                                                let mut v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                                                let mut v157: Rc<str> = std::rc::Rc::<str>::from([&*v155, &*v130, &*v156, &*v142, &*v142].concat());
                                                let mut v158: bool = std::fs::read_to_string(&*v143).is_ok_and(|value| value.contains(v157.as_ref()));
                                                if v158 {
                                                    let mut v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v162: Rc<str> = std::rc::Rc::<str>::from([&*v160, &*v130, &*v161, &*v159, &*v142].concat());
                                                    let mut v163: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v162 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                    v163
                                                } else {
                                                    let mut v164: Rc<str> = std::fs::read_to_string(&*v154).unwrap_or_default().into();
                                                    let mut v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v166: Rc<str> = std::rc::Rc::<str>::from(v164.trim_end_matches(&*v165));
                                                    let mut v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v170: Rc<str> = std::rc::Rc::<str>::from(v130.replace(&*v168, &v169));
                                                    let mut v171: Rc<str> = std::rc::Rc::<str>::from(v170.to_ascii_lowercase());
                                                    let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Task ("); } LIT.with(|lit| lit.clone()) };
                                                    let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                    let mut v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v176: Rc<str> = std::rc::Rc::<str>::from(v130.replace(&*v174, &v175));
                                                    let mut v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                                    let mut v178: Rc<str> = std::rc::Rc::<str>::from(v176.replace(&*v173, &v177));
                                                    let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v180: Rc<str> = std::rc::Rc::<str>::from(v178.replace(&*v165, &v179));
                                                    let mut v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v183: Rc<str> = std::rc::Rc::<str>::from(v180.replace(&*v181, &v182));
                                                    let mut v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v186: Rc<str> = std::rc::Rc::<str>::from(v183.replace(&*v184, &v185));
                                                    let mut v187: Rc<str> = std::rc::Rc::<str>::from([&*v173, &*v186, &*v173, &*v142, &*v142].concat());
                                                    let mut v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v189: Rc<str> = std::rc::Rc::<str>::from([&*v167, &*v171, &*v172, &*v187, &*v188].concat());
                                                    let mut v190: Rc<str> = std::rc::Rc::<str>::from(v137.replace(&*v174, &v175));
                                                    let mut v191: Rc<str> = std::rc::Rc::<str>::from(v190.replace(&*v173, &v177));
                                                    let mut v192: Rc<str> = std::rc::Rc::<str>::from(v191.replace(&*v165, &v179));
                                                    let mut v193: Rc<str> = std::rc::Rc::<str>::from(v192.replace(&*v181, &v182));
                                                    let mut v194: Rc<str> = std::rc::Rc::<str>::from(v193.replace(&*v184, &v185));
                                                    let mut v195: Rc<str> = std::rc::Rc::<str>::from([&*v173, &*v194, &*v173, &*v142, &*v142].concat());
                                                    let mut v196: Rc<str> = match &v131 {
                                                        US2::US2_AgileEpic => {
                                                            v28.clone()
                                                        }
                                                        US2::US2_AgileFeature => {
                                                            v34.clone()
                                                        }
                                                        US2::US2_AgileStory => {
                                                            v39.clone()
                                                        }
                                                        US2::US2_AgileTaskKind => {
                                                            v44.clone()
                                                        }
                                                    };
                                                    let mut v197: Rc<str> = match &v132 {
                                                        US4::US4_AgileP0 => {
                                                            v51.clone()
                                                        }
                                                        US4::US4_AgileP1 => {
                                                            v57.clone()
                                                        }
                                                        US4::US4_AgileP2 => {
                                                            v62.clone()
                                                        }
                                                        US4::US4_AgileP3 => {
                                                            v67.clone()
                                                        }
                                                        US4::US4_AgileP4 => {
                                                            v72.clone()
                                                        }
                                                        US4::US4_AgileP5 => {
                                                            v77.clone()
                                                        }
                                                    };
                                                    let mut v198: Rc<str> = std::rc::Rc::<str>::from([&*v195, &*v188, &*v196, &*v188, &*v197].concat());
                                                    let mut v199: Rc<str> = match &v133 {
                                                        US6::US6_AgileD0 => {
                                                            v84.clone()
                                                        }
                                                        US6::US6_AgileD1 => {
                                                            v90.clone()
                                                        }
                                                        US6::US6_AgileD2 => {
                                                            v95.clone()
                                                        }
                                                        US6::US6_AgileD3 => {
                                                            v100.clone()
                                                        }
                                                        US6::US6_AgileD4 => {
                                                            v105.clone()
                                                        }
                                                        US6::US6_AgileD5 => {
                                                            v110.clone()
                                                        }
                                                        US6::US6_AgileD6 => {
                                                            v115.clone()
                                                        }
                                                        US6::US6_AgileD7 => {
                                                            v120.clone()
                                                        }
                                                        US6::US6_AgileD8 => {
                                                            v125.clone()
                                                        }
                                                    };
                                                    let mut v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", 0u32, Planned, \"\", "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v201: Rc<str> = std::rc::Rc::<str>::from(v139.replace(&*v174, &v175));
                                                    let mut v202: Rc<str> = std::rc::Rc::<str>::from(v201.replace(&*v173, &v177));
                                                    let mut v203: Rc<str> = std::rc::Rc::<str>::from(v202.replace(&*v165, &v179));
                                                    let mut v204: Rc<str> = std::rc::Rc::<str>::from(v203.replace(&*v181, &v182));
                                                    let mut v205: Rc<str> = std::rc::Rc::<str>::from(v204.replace(&*v184, &v185));
                                                    let mut v206: Rc<str> = std::rc::Rc::<str>::from([&*v173, &*v205, &*v173, &*v142, &*v142].concat());
                                                    let mut v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", \"current\", \"current\")"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v208: Rc<str> = std::rc::Rc::<str>::from([&*v188, &*v199, &*v200, &*v206, &*v207].concat());
                                                    let mut v209: Rc<str> = std::rc::Rc::<str>::from([&*v189, &*v198, &*v208, &*v142, &*v142].concat());
                                                    let mut v210: Rc<str> = std::rc::Rc::<str>::from([&*v166, &*v165, &*v209, &*v165, &*v142].concat());
                                                    let mut v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v212: Rc<str> = std::rc::Rc::<str>::from([&*v143, &*v211, &*v142, &*v142, &*v142].concat());
                                                    let mut v213: bool = std::fs::write(&*v212, &*v210).is_ok();
                                                    let mut v222: US8 = if v213 {
                                                        let mut v214: bool = std::fs::rename(&*v212, &*v143).is_ok();
                                                        if v214 {
                                                            let mut v215: bool = std::fs::read_to_string(&*v143).is_ok_and(|value| value == *v210);
                                                            if v215 {
                                                                US8::US8_Replaced
                                                            } else {
                                                                US8::US8_ReadbackMismatch
                                                            }
                                                        } else {
                                                            US8::US8_RenameFailed
                                                        }
                                                    } else {
                                                        US8::US8_TemporaryWriteFailed
                                                    };
                                                    match &v222 {
                                                        US8::US8_Replaced => {
                                                            let mut v223: Rc<str> = std::rc::Rc::<str>::from(v130.replace(&*v168, &v169));
                                                            let mut v224: Rc<str> = std::rc::Rc::<str>::from(v223.to_ascii_lowercase());
                                                            let mut v225: u64 = v164.len() as u64;
                                                            let mut v226: u64 = v210.len() as u64;
                                                            let mut v227: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v140);
                                                            let mut v228: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile add ok id="); } LIT.with(|lit| lit.clone()) };
                                                            let mut v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" binding="); } LIT.with(|lit| lit.clone()) };
                                                            let mut v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status=Planned progress=0"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v231: Rc<str> = std::rc::Rc::<str>::from([&*v228, &*v130, &*v229, &*v224, &*v230].concat());
                                                            let mut v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                            let mut v233: Rc<str> = std::rc::Rc::<str>::from((v225).to_string());
                                                            let mut v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                            let mut v235: Rc<str> = std::rc::Rc::<str>::from((v226).to_string());
                                                            let mut v236: Rc<str> = std::rc::Rc::<str>::from([&*v232, &*v233, &*v234, &*v235, &*v142].concat());
                                                            let mut v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                            let mut v238: Rc<str> = std::rc::Rc::<str>::from((v227).to_string());
                                                            let mut v239: Rc<str> = std::rc::Rc::<str>::from([&*v236, &*v237, &*v238, &*v142, &*v142].concat());
                                                            let mut v240: Rc<str> = std::rc::Rc::<str>::from([&*v231, &*v239, &*v142, &*v142, &*v142].concat());
                                                            let mut v241: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v240.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                                            v241
                                                        }
                                                        _ => {
                                                            let mut v247: Rc<str> = match &v222 {
                                                                US8::US8_ReadbackMismatch => {
                                                                    let mut v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                    v244.clone()
                                                                }
                                                                US8::US8_RenameFailed => {
                                                                    let mut v243: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                    v243.clone()
                                                                }
                                                                US8::US8_TemporaryWriteFailed => {
                                                                    let mut v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                    v242.clone()
                                                                }
                                                                _ => unreachable!(),
                                                            };
                                                            let mut v248: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                            let mut v249: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                            let mut v250: Rc<str> = std::rc::Rc::<str>::from([&*v248, &*v130, &*v249, &*v247, &*v142].concat());
                                                            let mut v251: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v250 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                            v251
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
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_add_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_add_run() -> i32 {
    closure0()()
}
