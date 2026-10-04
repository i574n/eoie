#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_patterns, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use eoie_agile_lease::eoie_lease_title_words;
use eoie_agile_state::{agile_begin_owned as mutation_agile_begin_owned, agile_check_owned as mutation_agile_check_owned, agile_lease_status_owned as mutation_agile_lease_status_owned, agile_list_owned as mutation_agile_list_owned, agile_list_filtered_owned as mutation_agile_list_filtered_owned, agile_record_owned as mutation_agile_record_owned, agile_set_validated_owned as mutation_agile_set_owned};
use eoie_handoff::handoff_render_owned;

#[must_use]
fn agile_mutation_error(message: &str) -> i32 {
    eprintln!("eoie error: {message}");
    2
}

#[derive(Clone)]
enum US0 {
    US0_0(i32),
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
    US2_4,
}
impl US2 {
    fn tag(&self) -> i32 {
        match self {
            US2::US2_0 => 0,
            US2::US2_1 => 1,
            US2::US2_2 => 2,
            US2::US2_3 => 3,
            US2::US2_4 => 4,
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
fn method0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v0 ;
    if v1 {
        let mut v2: i32 = 1i32;
        let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: Rc<str> = Rc::<str>::from("list|check|lease|status|handoff|record");
        let mut v5: i32 = 0i32;
        let mut v6: Rc<str> = usize::try_from(v5).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v7: u64 = v4.split("|").position(|item| item == &*v6).map(|index| index as u64).unwrap_or(u64::MAX);
        let mut v8: bool = 0u64 == v7;
        if v8 {
            let mut v9: i32 = mutation_agile_list_owned(v3.as_ref());
            v9
        } else {
            let mut v10: bool = 1u64 == v7;
            if v10 {
                let mut v11: i32 = mutation_agile_check_owned(v3.as_ref());
                v11
            } else {
                let mut v12: bool = 2u64 == v7;
                if v12 {
                    let mut v13: i32 = mutation_agile_lease_status_owned(v3.as_ref());
                    v13
                } else {
                    let mut v14: bool = 3u64 == v7;
                    if v14 {
                        let mut v15: i32 = mutation_agile_lease_status_owned(v3.as_ref());
                        v15
                    } else {
                        let mut v16: bool = 4u64 == v7;
                        if v16 {
                            let mut v17: i32 = handoff_render_owned(v3.as_ref());
                            v17
                        } else {
                            let mut v18: bool = 5u64 == v7;
                            if v18 {
                                let mut v19: i32 = mutation_agile_record_owned(v3.as_ref());
                                v19
                            } else {
                                let mut v20: i32 = agile_mutation_error("agile supports begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>");
                                v20
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut v27: i32 = 3i32;
        let mut v28: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v27 ;
        if v28 {
            let mut v29: i32 = 0i32;
            let mut v30: Rc<str> = Rc::<str>::from("list");
            let mut v31: bool = usize::try_from(v29).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v30.as_ref());
            if v31 {
                let mut v32: i32 = 1i32;
                let mut v33: Rc<str> = usize::try_from(v32).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v34: i32 = 2i32;
                let mut v35: Rc<str> = usize::try_from(v34).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v36: i32 = mutation_agile_list_filtered_owned(v33.as_ref(), v35.as_ref());
                v36
            } else {
                let mut v37: i32 = 0i32;
                let mut v38: Rc<str> = Rc::<str>::from("begin");
                let mut v39: bool = usize::try_from(v37).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v38.as_ref());
                if v39 {
                    let mut v40: i32 = 1i32;
                    let mut v41: Rc<str> = usize::try_from(v40).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v42: i32 = 2i32;
                    let mut v43: Rc<str> = usize::try_from(v42).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v44: i32 = eoie_lease_title_words();
                    let mut v45: bool = 0 < v44;
                    if v45 {
                        let mut v46: i32 = mutation_agile_begin_owned(v41.as_ref(), v43.as_ref());
                        v46
                    } else {
                        let mut v47: i32 = agile_mutation_error("begin title must contain 3..7 visible words");
                        v47
                    }
                } else {
                    let mut v49: i32 = agile_mutation_error("agile supports begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>");
                    v49
                }
            }
        } else {
            let mut v52: i32 = 5i32;
            let mut v53: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v52 ;
            if v53 {
                let mut v54: i32 = 0i32;
                let mut v55: Rc<str> = Rc::<str>::from("set");
                let mut v56: bool = usize::try_from(v54).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v55.as_ref());
                if v56 {
                    let mut v57: i32 = 1i32;
                    let mut v58: Rc<str> = usize::try_from(v57).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v59: i32 = 2i32;
                    let mut v60: Rc<str> = usize::try_from(v59).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v61: i32 = 3i32;
                    let mut v62: i32 = usize::try_from(v61).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
                    let mut v63: i32 = 3i32;
                    let mut v64: i32 = 0i32;
                    let mut v65: i32 = usize::try_from(v63).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v64).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                    let mut v66: bool = 48i32 <= v65;
                    let mut v68: bool = if v66 {
                        let mut v67: bool = v65 <= 57i32;
                        v67
                    } else {
                        false
                    };
                    let mut v71: i32 = if v68 {
                        let mut v69: i32 = v65 - 48;
                        v69
                    } else {
                        let mut v70: i32 = -1;
                        v70
                    };
                    let mut v72: i32 = 3i32;
                    let mut v73: i32 = 1i32;
                    let mut v74: i32 = usize::try_from(v72).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v73).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                    let mut v75: bool = 48i32 <= v74;
                    let mut v77: bool = if v75 {
                        let mut v76: bool = v74 <= 57i32;
                        v76
                    } else {
                        false
                    };
                    let mut v80: i32 = if v77 {
                        let mut v78: i32 = v74 - 48;
                        v78
                    } else {
                        let mut v79: i32 = -1;
                        v79
                    };
                    let mut v81: i32 = 3i32;
                    let mut v82: i32 = 2i32;
                    let mut v83: i32 = usize::try_from(v81).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v82).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                    let mut v84: bool = 48i32 <= v83;
                    let mut v86: bool = if v84 {
                        let mut v85: bool = v83 <= 57i32;
                        v85
                    } else {
                        false
                    };
                    let mut v89: i32 = if v86 {
                        let mut v87: i32 = v83 - 48;
                        v87
                    } else {
                        let mut v88: i32 = -1;
                        v88
                    };
                    let mut v90: bool = v62 == 1i32;
                    let mut v91: bool = 0 <= v71;
                    let mut v92: bool = v90 && v91;
                    let mut v116: US0 = if v92 {
                        US0::US0_0(v71)
                    } else {
                        let mut v94: bool = v62 == 2i32;
                        let mut v95: bool = 0 <= v71;
                        let mut v96: bool = 0 <= v80;
                        let mut v98: bool = if v94 {
                            let mut v97: bool = v95 && v96;
                            v97
                        } else {
                            false
                        };
                        if v98 {
                            let mut v99: i32 = v71 * 10 + v80;
                            US0::US0_0(v99)
                        } else {
                            let mut v101: bool = v62 == 3i32;
                            let mut v102: bool = 0 <= v71;
                            let mut v103: bool = 0 <= v80;
                            let mut v104: bool = 0 <= v89;
                            let mut v107: bool = if v101 {
                                if v102 {
                                    let mut v105: bool = v103 && v104;
                                    v105
                                } else {
                                    false
                                }
                            } else {
                                false
                            };
                            if v107 {
                                let mut v108: i32 = v71 * 100 + v80 * 10 + v89;
                                let mut v109: bool = v108 <= 100i32;
                                if v109 {
                                    US0::US0_0(v108)
                                } else {
                                    US0::US0_1
                                }
                            } else {
                                US0::US0_1
                            }
                        }
                    };
                    let mut v117: Rc<str> = Rc::<str>::from("Planned|Active|Blocked|Paused|Done");
                    let mut v118: i32 = 4i32;
                    let mut v119: Rc<str> = usize::try_from(v118).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v120: u64 = v117.split("|").position(|item| item == &*v119).map(|index| index as u64).unwrap_or(u64::MAX);
                    let mut v121: bool = 0u64 == v120;
                    let mut v141: US1 = if v121 {
                        let mut v122: US2 = US2::US2_0;
                        US1::US1_0(v122.clone())
                    } else {
                        let mut v124: bool = 1u64 == v120;
                        if v124 {
                            let mut v125: US2 = US2::US2_1;
                            US1::US1_0(v125.clone())
                        } else {
                            let mut v127: bool = 2u64 == v120;
                            if v127 {
                                let mut v128: US2 = US2::US2_2;
                                US1::US1_0(v128.clone())
                            } else {
                                let mut v130: bool = 3u64 == v120;
                                if v130 {
                                    let mut v131: US2 = US2::US2_3;
                                    US1::US1_0(v131.clone())
                                } else {
                                    let mut v133: bool = 4u64 == v120;
                                    if v133 {
                                        let mut v134: US2 = US2::US2_4;
                                        US1::US1_0(v134.clone())
                                    } else {
                                        US1::US1_1
                                    }
                                }
                            }
                        }
                    };
                    match &v116 {
                        US0::US0_1 => { // Invalid
                            let mut v153: i32 = agile_mutation_error("progress must be 0..100");
                            v153
                        }
                        US0::US0_0(v142) => { // Valid
                            let mut v142: i32 = v142.clone();
                            match &v141 {
                                US1::US1_1 => { // Invalid
                                    let mut v150: i32 = agile_mutation_error("invalid agile status");
                                    v150
                                }
                                US1::US1_0(v143) => { // Valid
                                    let mut v143: US2 = v143.clone();
                                    let mut v148: i32 = match &v143 {
                                        US2::US2_1 => { // AgileActive
                                            1i32
                                        }
                                        US2::US2_2 => { // AgileBlocked
                                            2i32
                                        }
                                        US2::US2_4 => { // AgileDone
                                            4i32
                                        }
                                        US2::US2_3 => { // AgilePaused
                                            3i32
                                        }
                                        US2::US2_0 => { // AgilePlanned
                                            0i32
                                        }
                                        _ => unreachable!(),
                                    };
                                    let mut v149: i32 = mutation_agile_set_owned(v58.as_ref(), v60.as_ref(), v142, v148);
                                    v149
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    let mut v156: i32 = agile_mutation_error("agile supports begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>");
                    v156
                }
            } else {
                let mut v158: i32 = agile_mutation_error("agile supports begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>");
                v158
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    Rc::new(move || -> i32 {
        method0()
    })
}
pub fn eoie_agile_run() -> i32 {
    closure0()()
}
