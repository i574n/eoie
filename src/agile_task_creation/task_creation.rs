#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Valid(Rc<str>),
    US0_Invalid,
}
impl US0 {
    fn tag(&self) -> i32 {
        match self {
            US0::US0_Valid(..) => 0,
            US0::US0_Invalid => 1,
        }
    }
}
impl Default for US0 {
    fn default() -> Self {
        US0::US0_Valid({ thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })
    }
}
#[derive(Clone)]
enum US2 {
    US2_AgileEpic,
    US2_AgileFeature,
    US2_AgileStory,
    US2_AgileTaskKind,
}
impl US2 {
    fn tag(&self) -> i32 {
        match self {
            US2::US2_AgileEpic => 0,
            US2::US2_AgileFeature => 1,
            US2::US2_AgileStory => 2,
            US2::US2_AgileTaskKind => 3,
        }
    }
}
impl Default for US2 {
    fn default() -> Self {
        US2::US2_AgileEpic
    }
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
impl US1 {
    fn tag(&self) -> i32 {
        match self {
            US1::US1_Valid(..) => 0,
            US1::US1_Invalid => 1,
        }
    }
}
impl Default for US1 {
    fn default() -> Self {
        US1::US1_Valid(<US2>::default())
    }
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
impl US4 {
    fn tag(&self) -> i32 {
        match self {
            US4::US4_AgileP0 => 0,
            US4::US4_AgileP1 => 1,
            US4::US4_AgileP2 => 2,
            US4::US4_AgileP3 => 3,
            US4::US4_AgileP4 => 4,
            US4::US4_AgileP5 => 5,
        }
    }
}
impl Default for US4 {
    fn default() -> Self {
        US4::US4_AgileP0
    }
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US4),
    US3_Invalid,
}
impl US3 {
    fn tag(&self) -> i32 {
        match self {
            US3::US3_Valid(..) => 0,
            US3::US3_Invalid => 1,
        }
    }
}
impl Default for US3 {
    fn default() -> Self {
        US3::US3_Valid(<US4>::default())
    }
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
impl US6 {
    fn tag(&self) -> i32 {
        match self {
            US6::US6_AgileD0 => 0,
            US6::US6_AgileD1 => 1,
            US6::US6_AgileD2 => 2,
            US6::US6_AgileD3 => 3,
            US6::US6_AgileD4 => 4,
            US6::US6_AgileD5 => 5,
            US6::US6_AgileD6 => 6,
            US6::US6_AgileD7 => 7,
            US6::US6_AgileD8 => 8,
        }
    }
}
impl Default for US6 {
    fn default() -> Self {
        US6::US6_AgileD0
    }
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US6),
    US5_Invalid,
}
impl US5 {
    fn tag(&self) -> i32 {
        match self {
            US5::US5_Valid(..) => 0,
            US5::US5_Invalid => 1,
        }
    }
}
impl Default for US5 {
    fn default() -> Self {
        US5::US5_Valid(<US6>::default())
    }
}
#[derive(Clone)]
enum US7 {
    US7_FileReadable(Rc<str>),
    US7_FileMissing(Rc<str>),
}
impl US7 {
    fn tag(&self) -> i32 {
        match self {
            US7::US7_FileReadable(..) => 0,
            US7::US7_FileMissing(..) => 1,
        }
    }
}
impl Default for US7 {
    fn default() -> Self {
        US7::US7_FileReadable({ thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })
    }
}
#[derive(Clone)]
enum US8 {
    US8_Replaced,
    US8_TemporaryWriteFailed,
    US8_RenameFailed,
    US8_ReadbackMismatch,
}
impl US8 {
    fn tag(&self) -> i32 {
        match self {
            US8::US8_Replaced => 0,
            US8::US8_TemporaryWriteFailed => 1,
            US8::US8_RenameFailed => 2,
            US8::US8_ReadbackMismatch => 3,
        }
    }
}
impl Default for US8 {
    fn default() -> Self {
        US8::US8_Replaced
    }
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
                let mut v14: i32 = v0 + 1;
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
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic|Feature|Story|TaskKind"); } LIT.with(|lit| lit.clone()) };
        let mut v29: u64 = v28.split("|").position(|item| item == &*v27).map(|index| index as u64).unwrap_or(u64::MAX);
        let mut v30: bool = 0u64 == v29;
        let mut v46: US1 = if v30 {
            let mut v31: US2 = US2::US2_AgileEpic;
            US1::US1_Valid(v31.clone())
        } else {
            let mut v33: bool = 1u64 == v29;
            if v33 {
                let mut v34: US2 = US2::US2_AgileFeature;
                US1::US1_Valid(v34.clone())
            } else {
                let mut v36: bool = 2u64 == v29;
                if v36 {
                    let mut v37: US2 = US2::US2_AgileStory;
                    US1::US1_Valid(v37.clone())
                } else {
                    let mut v39: bool = 3u64 == v29;
                    if v39 {
                        let mut v40: US2 = US2::US2_AgileTaskKind;
                        US1::US1_Valid(v40.clone())
                    } else {
                        US1::US1_Invalid
                    }
                }
            }
        };
        let mut v47: i32 = 4i32;
        let mut v48: Rc<str> = usize::try_from(v47).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0|P1|P2|P3|P4|P5"); } LIT.with(|lit| lit.clone()) };
        let mut v50: u64 = v49.split("|").position(|item| item == &*v48).map(|index| index as u64).unwrap_or(u64::MAX);
        let mut v51: bool = 0u64 == v50;
        let mut v75: US3 = if v51 {
            let mut v52: US4 = US4::US4_AgileP0;
            US3::US3_Valid(v52.clone())
        } else {
            let mut v54: bool = 1u64 == v50;
            if v54 {
                let mut v55: US4 = US4::US4_AgileP1;
                US3::US3_Valid(v55.clone())
            } else {
                let mut v57: bool = 2u64 == v50;
                if v57 {
                    let mut v58: US4 = US4::US4_AgileP2;
                    US3::US3_Valid(v58.clone())
                } else {
                    let mut v60: bool = 3u64 == v50;
                    if v60 {
                        let mut v61: US4 = US4::US4_AgileP3;
                        US3::US3_Valid(v61.clone())
                    } else {
                        let mut v63: bool = 4u64 == v50;
                        if v63 {
                            let mut v64: US4 = US4::US4_AgileP4;
                            US3::US3_Valid(v64.clone())
                        } else {
                            let mut v66: bool = 5u64 == v50;
                            if v66 {
                                let mut v67: US4 = US4::US4_AgileP5;
                                US3::US3_Valid(v67.clone())
                            } else {
                                US3::US3_Invalid
                            }
                        }
                    }
                }
            }
        };
        let mut v76: i32 = 5i32;
        let mut v77: Rc<str> = usize::try_from(v76).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0|D1|D2|D3|D4|D5|D6|D7|D8"); } LIT.with(|lit| lit.clone()) };
        let mut v79: u64 = v78.split("|").position(|item| item == &*v77).map(|index| index as u64).unwrap_or(u64::MAX);
        let mut v80: bool = 0u64 == v79;
        let mut v116: US5 = if v80 {
            let mut v81: US6 = US6::US6_AgileD0;
            US5::US5_Valid(v81.clone())
        } else {
            let mut v83: bool = 1u64 == v79;
            if v83 {
                let mut v84: US6 = US6::US6_AgileD1;
                US5::US5_Valid(v84.clone())
            } else {
                let mut v86: bool = 2u64 == v79;
                if v86 {
                    let mut v87: US6 = US6::US6_AgileD2;
                    US5::US5_Valid(v87.clone())
                } else {
                    let mut v89: bool = 3u64 == v79;
                    if v89 {
                        let mut v90: US6 = US6::US6_AgileD3;
                        US5::US5_Valid(v90.clone())
                    } else {
                        let mut v92: bool = 4u64 == v79;
                        if v92 {
                            let mut v93: US6 = US6::US6_AgileD4;
                            US5::US5_Valid(v93.clone())
                        } else {
                            let mut v95: bool = 5u64 == v79;
                            if v95 {
                                let mut v96: US6 = US6::US6_AgileD5;
                                US5::US5_Valid(v96.clone())
                            } else {
                                let mut v98: bool = 6u64 == v79;
                                if v98 {
                                    let mut v99: US6 = US6::US6_AgileD6;
                                    US5::US5_Valid(v99.clone())
                                } else {
                                    let mut v101: bool = 7u64 == v79;
                                    if v101 {
                                        let mut v102: US6 = US6::US6_AgileD7;
                                        US5::US5_Valid(v102.clone())
                                    } else {
                                        let mut v104: bool = 8u64 == v79;
                                        if v104 {
                                            let mut v105: US6 = US6::US6_AgileD8;
                                            US5::US5_Valid(v105.clone())
                                        } else {
                                            US5::US5_Invalid
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        match &v25 {
            US0::US0_Invalid => {
                let mut v308: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must be uppercase letters, digits and dashes"); } LIT.with(|lit| lit.clone()) };
                let mut v309: i32 = 2i32;
                let mut v310: Rc<str> = usize::try_from(v309).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v311: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                let mut v312: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v313: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v314: Rc<str> = std::rc::Rc::<str>::from([&*v311, &*v310, &*v312, &*v308, &*v313].concat());
                let mut v315: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v314 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v315
            }
            US0::US0_Valid(v117) => {
                let mut v117: Rc<str> = v117.clone();
                match &v46 {
                    US1::US1_Invalid => {
                        let mut v298: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                        let mut v299: i32 = 2i32;
                        let mut v300: Rc<str> = usize::try_from(v299).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v301: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                        let mut v302: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v303: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v304: Rc<str> = std::rc::Rc::<str>::from([&*v301, &*v300, &*v302, &*v298, &*v303].concat());
                        let mut v305: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v304 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v305
                    }
                    US1::US1_Valid(v118) => {
                        let mut v118: US2 = v118.clone();
                        match &v75 {
                            US3::US3_Invalid => {
                                let mut v288: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                                let mut v289: i32 = 2i32;
                                let mut v290: Rc<str> = usize::try_from(v289).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v291: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                let mut v292: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let mut v293: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v294: Rc<str> = std::rc::Rc::<str>::from([&*v291, &*v290, &*v292, &*v288, &*v293].concat());
                                let mut v295: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v294 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v295
                            }
                            US3::US3_Valid(v119) => {
                                let mut v119: US4 = v119.clone();
                                match &v116 {
                                    US5::US5_Invalid => {
                                        let mut v278: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                                        let mut v279: i32 = 2i32;
                                        let mut v280: Rc<str> = usize::try_from(v279).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v281: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                        let mut v282: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                        let mut v283: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v284: Rc<str> = std::rc::Rc::<str>::from([&*v281, &*v280, &*v282, &*v278, &*v283].concat());
                                        let mut v285: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v284 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v285
                                    }
                                    US5::US5_Valid(v120) => {
                                        let mut v120: US6 = v120.clone();
                                        let mut v121: i32 = 1i32;
                                        let mut v122: Rc<str> = usize::try_from(v121).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v123: i32 = 6i32;
                                        let mut v124: Rc<str> = usize::try_from(v123).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v125: i32 = 7i32;
                                        let mut v126: Rc<str> = usize::try_from(v125).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v127: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                        let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                        let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v130: Rc<str> = std::rc::Rc::<str>::from([&*v122, &*v128, &*v129, &*v129, &*v129].concat());
                                        let mut v131: bool = std::path::Path::new(&*v130).is_file();
                                        let mut v134: US7 = if v131 {
                                            US7::US7_FileReadable(v130.clone())
                                        } else {
                                            US7::US7_FileMissing(v130.clone())
                                        };
                                        match &v134 {
                                            US7::US7_FileMissing(v135) => {
                                                let mut v135: Rc<str> = v135.clone();
                                                let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                                let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                let mut v139: Rc<str> = std::rc::Rc::<str>::from([&*v137, &*v117, &*v138, &*v136, &*v129].concat());
                                                let mut v140: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v139 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                v140
                                            }
                                            US7::US7_FileReadable(v141) => {
                                                let mut v141: Rc<str> = v141.clone();
                                                let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                                                let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                                                let mut v144: Rc<str> = std::rc::Rc::<str>::from([&*v142, &*v117, &*v143, &*v129, &*v129].concat());
                                                let mut v145: bool = std::fs::read_to_string(&*v130).is_ok_and(|value| value.contains(v144.as_ref()));
                                                if v145 {
                                                    let mut v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v149: Rc<str> = std::rc::Rc::<str>::from([&*v147, &*v117, &*v148, &*v146, &*v129].concat());
                                                    let mut v150: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v149 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                    v150
                                                } else {
                                                    let mut v151: Rc<str> = std::fs::read_to_string(&*v141).unwrap_or_default().into();
                                                    let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v153: Rc<str> = std::rc::Rc::<str>::from(v151.trim_end_matches(&*v152));
                                                    let mut v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v157: Rc<str> = std::rc::Rc::<str>::from(v117.replace(&*v155, &v156));
                                                    let mut v158: Rc<str> = std::rc::Rc::<str>::from(v157.to_ascii_lowercase());
                                                    let mut v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Task ("); } LIT.with(|lit| lit.clone()) };
                                                    let mut v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                    let mut v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v163: Rc<str> = std::rc::Rc::<str>::from(v117.replace(&*v161, &v162));
                                                    let mut v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                                    let mut v165: Rc<str> = std::rc::Rc::<str>::from(v163.replace(&*v160, &v164));
                                                    let mut v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v167: Rc<str> = std::rc::Rc::<str>::from(v165.replace(&*v152, &v166));
                                                    let mut v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v170: Rc<str> = std::rc::Rc::<str>::from(v167.replace(&*v168, &v169));
                                                    let mut v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v173: Rc<str> = std::rc::Rc::<str>::from(v170.replace(&*v171, &v172));
                                                    let mut v174: Rc<str> = std::rc::Rc::<str>::from([&*v160, &*v173, &*v160, &*v129, &*v129].concat());
                                                    let mut v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v176: Rc<str> = std::rc::Rc::<str>::from([&*v154, &*v158, &*v159, &*v174, &*v175].concat());
                                                    let mut v177: Rc<str> = std::rc::Rc::<str>::from(v124.replace(&*v161, &v162));
                                                    let mut v178: Rc<str> = std::rc::Rc::<str>::from(v177.replace(&*v160, &v164));
                                                    let mut v179: Rc<str> = std::rc::Rc::<str>::from(v178.replace(&*v152, &v166));
                                                    let mut v180: Rc<str> = std::rc::Rc::<str>::from(v179.replace(&*v168, &v169));
                                                    let mut v181: Rc<str> = std::rc::Rc::<str>::from(v180.replace(&*v171, &v172));
                                                    let mut v182: Rc<str> = std::rc::Rc::<str>::from([&*v160, &*v181, &*v160, &*v129, &*v129].concat());
                                                    let mut v190: Rc<str> = match &v118 {
                                                        US2::US2_AgileEpic => {
                                                            let mut v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                                                            v183.clone()
                                                        }
                                                        US2::US2_AgileFeature => {
                                                            let mut v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                                                            v184.clone()
                                                        }
                                                        US2::US2_AgileStory => {
                                                            let mut v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                                                            v185.clone()
                                                        }
                                                        US2::US2_AgileTaskKind => {
                                                            let mut v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                                                            v186.clone()
                                                        }
                                                    };
                                                    let mut v202: Rc<str> = match &v119 {
                                                        US4::US4_AgileP0 => {
                                                            let mut v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                                                            v191.clone()
                                                        }
                                                        US4::US4_AgileP1 => {
                                                            let mut v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                                                            v192.clone()
                                                        }
                                                        US4::US4_AgileP2 => {
                                                            let mut v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                                                            v193.clone()
                                                        }
                                                        US4::US4_AgileP3 => {
                                                            let mut v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                                                            v194.clone()
                                                        }
                                                        US4::US4_AgileP4 => {
                                                            let mut v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                                                            v195.clone()
                                                        }
                                                        US4::US4_AgileP5 => {
                                                            let mut v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                                                            v196.clone()
                                                        }
                                                    };
                                                    let mut v203: Rc<str> = std::rc::Rc::<str>::from([&*v182, &*v175, &*v190, &*v175, &*v202].concat());
                                                    let mut v221: Rc<str> = match &v120 {
                                                        US6::US6_AgileD0 => {
                                                            let mut v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                                                            v204.clone()
                                                        }
                                                        US6::US6_AgileD1 => {
                                                            let mut v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                                                            v205.clone()
                                                        }
                                                        US6::US6_AgileD2 => {
                                                            let mut v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                                                            v206.clone()
                                                        }
                                                        US6::US6_AgileD3 => {
                                                            let mut v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                                                            v207.clone()
                                                        }
                                                        US6::US6_AgileD4 => {
                                                            let mut v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                                                            v208.clone()
                                                        }
                                                        US6::US6_AgileD5 => {
                                                            let mut v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                                                            v209.clone()
                                                        }
                                                        US6::US6_AgileD6 => {
                                                            let mut v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                                                            v210.clone()
                                                        }
                                                        US6::US6_AgileD7 => {
                                                            let mut v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                                                            v211.clone()
                                                        }
                                                        US6::US6_AgileD8 => {
                                                            let mut v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                                                            v212.clone()
                                                        }
                                                    };
                                                    let mut v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", 0u32, Planned, \"\", "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v223: Rc<str> = std::rc::Rc::<str>::from(v126.replace(&*v161, &v162));
                                                    let mut v224: Rc<str> = std::rc::Rc::<str>::from(v223.replace(&*v160, &v164));
                                                    let mut v225: Rc<str> = std::rc::Rc::<str>::from(v224.replace(&*v152, &v166));
                                                    let mut v226: Rc<str> = std::rc::Rc::<str>::from(v225.replace(&*v168, &v169));
                                                    let mut v227: Rc<str> = std::rc::Rc::<str>::from(v226.replace(&*v171, &v172));
                                                    let mut v228: Rc<str> = std::rc::Rc::<str>::from([&*v160, &*v227, &*v160, &*v129, &*v129].concat());
                                                    let mut v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", \"current\", \"current\")"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v230: Rc<str> = std::rc::Rc::<str>::from([&*v175, &*v221, &*v222, &*v228, &*v229].concat());
                                                    let mut v231: Rc<str> = std::rc::Rc::<str>::from([&*v176, &*v203, &*v230, &*v129, &*v129].concat());
                                                    let mut v232: Rc<str> = std::rc::Rc::<str>::from([&*v153, &*v152, &*v231, &*v152, &*v129].concat());
                                                    let mut v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v234: Rc<str> = std::rc::Rc::<str>::from([&*v130, &*v233, &*v129, &*v129, &*v129].concat());
                                                    let mut v235: bool = std::fs::write(&*v234, &*v232).is_ok();
                                                    let mut v244: US8 = if v235 {
                                                        let mut v236: bool = std::fs::rename(&*v234, &*v130).is_ok();
                                                        if v236 {
                                                            let mut v237: bool = std::fs::read_to_string(&*v130).is_ok_and(|value| value == *v232);
                                                            if v237 {
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
                                                    match &v244 {
                                                        US8::US8_Replaced => {
                                                            let mut v245: Rc<str> = std::rc::Rc::<str>::from(v117.replace(&*v155, &v156));
                                                            let mut v246: Rc<str> = std::rc::Rc::<str>::from(v245.to_ascii_lowercase());
                                                            let mut v247: u64 = v151.len() as u64;
                                                            let mut v248: u64 = v232.len() as u64;
                                                            let mut v249: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v127);
                                                            let mut v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile add ok id="); } LIT.with(|lit| lit.clone()) };
                                                            let mut v251: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" binding="); } LIT.with(|lit| lit.clone()) };
                                                            let mut v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status=Planned progress=0"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v253: Rc<str> = std::rc::Rc::<str>::from([&*v250, &*v117, &*v251, &*v246, &*v252].concat());
                                                            let mut v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                            let mut v255: Rc<str> = std::rc::Rc::<str>::from((v247).to_string());
                                                            let mut v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                            let mut v257: Rc<str> = std::rc::Rc::<str>::from((v248).to_string());
                                                            let mut v258: Rc<str> = std::rc::Rc::<str>::from([&*v254, &*v255, &*v256, &*v257, &*v129].concat());
                                                            let mut v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                            let mut v260: Rc<str> = std::rc::Rc::<str>::from((v249).to_string());
                                                            let mut v261: Rc<str> = std::rc::Rc::<str>::from([&*v258, &*v259, &*v260, &*v129, &*v129].concat());
                                                            let mut v262: Rc<str> = std::rc::Rc::<str>::from([&*v253, &*v261, &*v129, &*v129, &*v129].concat());
                                                            let mut v263: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v262.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                                            v263
                                                        }
                                                        _ => {
                                                            let mut v269: Rc<str> = match &v244 {
                                                                US8::US8_ReadbackMismatch => {
                                                                    let mut v266: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                    v266.clone()
                                                                }
                                                                US8::US8_RenameFailed => {
                                                                    let mut v265: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                    v265.clone()
                                                                }
                                                                US8::US8_TemporaryWriteFailed => {
                                                                    let mut v264: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                    v264.clone()
                                                                }
                                                                _ => unreachable!(),
                                                            };
                                                            let mut v270: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                            let mut v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                            let mut v272: Rc<str> = std::rc::Rc::<str>::from([&*v270, &*v117, &*v271, &*v269, &*v129].concat());
                                                            let mut v273: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v272 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                            v273
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
