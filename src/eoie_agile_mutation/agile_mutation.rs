#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use eoie_agile_lease::eoie_lease_title_words;
use eoie_agile_state::{agile_begin_owned as mutation_agile_begin_owned, agile_check_owned as mutation_agile_check_owned, agile_lease_status_owned as mutation_agile_lease_status_owned, agile_record_owned as mutation_agile_record_owned, agile_set_validated_owned as mutation_agile_set_owned};
use eoie_handoff::handoff_render_owned;

#[must_use]
fn agile_mutation_error(message: &str) -> i32 {
    eprintln!("eoie error: {message}");
    2
}

#[derive(Clone)]
enum US0 {
    US0_Valid(i32),
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
        US0::US0_Valid(<i32>::default())
    }
}
#[derive(Clone)]
enum US2 {
    US2_AgilePlanned,
    US2_AgileActive,
    US2_AgileBlocked,
    US2_AgilePaused,
    US2_AgileDone,
}
impl US2 {
    fn tag(&self) -> i32 {
        match self {
            US2::US2_AgilePlanned => 0,
            US2::US2_AgileActive => 1,
            US2::US2_AgileBlocked => 2,
            US2::US2_AgilePaused => 3,
            US2::US2_AgileDone => 4,
        }
    }
}
impl Default for US2 {
    fn default() -> Self {
        US2::US2_AgilePlanned
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
fn agile_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v0 ;
    if v1 {
        let mut v2: i32 = 1i32;
        let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list|check|lease|status|handoff|record|init"); } LIT.with(|lit| lit.clone()) };
        let mut v5: i32 = 0i32;
        let mut v6: Rc<str> = usize::try_from(v5).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v7: u64 = v4.split("|").position(|item| item == &*v6).map(|index| index as u64).unwrap_or(u64::MAX);
        let mut v8: bool = 0u64 == v7;
        if v8 {
            let mut v9: i32 = agile_task_listing::eoie_agile_list_run();
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
                                let mut v20: bool = 6u64 == v7;
                                if v20 {
                                    let mut v21: i32 = agile_backlog_scaffold::eoie_agile_init_run();
                                    v21
                                } else {
                                    let mut v22: i32 = 0i32;
                                    let mut v23: Rc<str> = usize::try_from(v22).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show"); } LIT.with(|lit| lit.clone()) };
                                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>\n<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
                                    let mut v26: Rc<str> = v24.split("|").zip(v25.split(char::from(10u8))).find_map(|(key,item)| if key == &*v23 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v28: bool = v26 == v27 ;
                                    if v28 {
                                        let mut v29: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>, show <root> <id> [--json]");
                                        v29
                                    } else {
                                        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                                        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                                        let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v23, &*v31, &*v26, &*v32].concat());
                                        let mut v34: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                                        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                                        let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v27, &*v27].concat());
                                        let mut v37: i32 = agile_mutation_error(&v36);
                                        v37
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut v46: i32 = 3i32;
        let mut v47: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v46 ;
        if v47 {
            let mut v48: i32 = 0i32;
            let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
            let mut v50: bool = usize::try_from(v48).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v49.as_ref());
            if v50 {
                let mut v51: i32 = agile_task_listing::eoie_agile_list_run();
                v51
            } else {
                let mut v52: i32 = 0i32;
                let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                let mut v54: bool = usize::try_from(v52).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v53.as_ref());
                if v54 {
                    let mut v55: i32 = agile_task_inspection::eoie_agile_show_run();
                    v55
                } else {
                    let mut v56: i32 = 0i32;
                    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin"); } LIT.with(|lit| lit.clone()) };
                    let mut v58: bool = usize::try_from(v56).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v57.as_ref());
                    if v58 {
                        let mut v59: i32 = 1i32;
                        let mut v60: Rc<str> = usize::try_from(v59).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v61: i32 = 2i32;
                        let mut v62: Rc<str> = usize::try_from(v61).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v63: i32 = eoie_lease_title_words();
                        let mut v64: bool = 0 < v63;
                        if v64 {
                            let mut v65: i32 = mutation_agile_begin_owned(v60.as_ref(), v62.as_ref());
                            v65
                        } else {
                            let mut v66: i32 = agile_mutation_error("begin title must contain 3..7 visible words");
                            v66
                        }
                    } else {
                        let mut v68: i32 = 0i32;
                        let mut v69: Rc<str> = usize::try_from(v68).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show"); } LIT.with(|lit| lit.clone()) };
                        let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>\n<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
                        let mut v72: Rc<str> = v70.split("|").zip(v71.split(char::from(10u8))).find_map(|(key,item)| if key == &*v69 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                        let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v74: bool = v72 == v73 ;
                        if v74 {
                            let mut v75: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>, show <root> <id> [--json]");
                            v75
                        } else {
                            let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                            let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                            let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                            let mut v79: Rc<str> = std::rc::Rc::<str>::from([&*v76, &*v69, &*v77, &*v72, &*v78].concat());
                            let mut v80: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                            let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                            let mut v82: Rc<str> = std::rc::Rc::<str>::from([&*v79, &*v80, &*v81, &*v73, &*v73].concat());
                            let mut v83: i32 = agile_mutation_error(&v82);
                            v83
                        }
                    }
                }
            }
        } else {
            let mut v88: i32 = 4i32;
            let mut v89: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v88 ;
            if v89 {
                let mut v90: i32 = 0i32;
                let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                let mut v92: bool = usize::try_from(v90).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v91.as_ref());
                if v92 {
                    let mut v93: i32 = agile_task_listing::eoie_agile_list_run();
                    v93
                } else {
                    let mut v94: i32 = 0i32;
                    let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                    let mut v96: bool = usize::try_from(v94).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v95.as_ref());
                    if v96 {
                        let mut v97: i32 = agile_task_inspection::eoie_agile_show_run();
                        v97
                    } else {
                        let mut v98: i32 = 0i32;
                        let mut v99: Rc<str> = usize::try_from(v98).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show"); } LIT.with(|lit| lit.clone()) };
                        let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>\n<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
                        let mut v102: Rc<str> = v100.split("|").zip(v101.split(char::from(10u8))).find_map(|(key,item)| if key == &*v99 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                        let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v104: bool = v102 == v103 ;
                        if v104 {
                            let mut v105: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>, show <root> <id> [--json]");
                            v105
                        } else {
                            let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                            let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                            let mut v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                            let mut v109: Rc<str> = std::rc::Rc::<str>::from([&*v106, &*v99, &*v107, &*v102, &*v108].concat());
                            let mut v110: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                            let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                            let mut v112: Rc<str> = std::rc::Rc::<str>::from([&*v109, &*v110, &*v111, &*v103, &*v103].concat());
                            let mut v113: i32 = agile_mutation_error(&v112);
                            v113
                        }
                    }
                }
            } else {
                let mut v117: i32 = 8i32;
                let mut v118: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v117 ;
                if v118 {
                    let mut v119: i32 = 0i32;
                    let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
                    let mut v121: bool = usize::try_from(v119).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v120.as_ref());
                    if v121 {
                        let mut v122: i32 = agile_task_creation::eoie_agile_add_run();
                        v122
                    } else {
                        let mut v123: i32 = 0i32;
                        let mut v124: Rc<str> = usize::try_from(v123).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show"); } LIT.with(|lit| lit.clone()) };
                        let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>\n<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
                        let mut v127: Rc<str> = v125.split("|").zip(v126.split(char::from(10u8))).find_map(|(key,item)| if key == &*v124 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                        let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v129: bool = v127 == v128 ;
                        if v129 {
                            let mut v130: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>, show <root> <id> [--json]");
                            v130
                        } else {
                            let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                            let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                            let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                            let mut v134: Rc<str> = std::rc::Rc::<str>::from([&*v131, &*v124, &*v132, &*v127, &*v133].concat());
                            let mut v135: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                            let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                            let mut v137: Rc<str> = std::rc::Rc::<str>::from([&*v134, &*v135, &*v136, &*v128, &*v128].concat());
                            let mut v138: i32 = agile_mutation_error(&v137);
                            v138
                        }
                    }
                } else {
                    let mut v141: i32 = 5i32;
                    let mut v142: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v141 ;
                    if v142 {
                        let mut v143: i32 = 0i32;
                        let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
                        let mut v145: bool = usize::try_from(v143).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v144.as_ref());
                        if v145 {
                            let mut v146: i32 = agile_task_revision::eoie_agile_edit_run();
                            v146
                        } else {
                            let mut v147: i32 = 0i32;
                            let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
                            let mut v149: bool = usize::try_from(v147).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v148.as_ref());
                            if v149 {
                                let mut v150: i32 = 1i32;
                                let mut v151: Rc<str> = usize::try_from(v150).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v152: i32 = 2i32;
                                let mut v153: Rc<str> = usize::try_from(v152).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v154: i32 = 3i32;
                                let mut v155: i32 = usize::try_from(v154).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
                                let mut v156: i32 = 3i32;
                                let mut v157: i32 = 0i32;
                                let mut v158: i32 = usize::try_from(v156).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v157).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v159: bool = 48i32 <= v158;
                                let mut v161: bool = if v159 {
                                    let mut v160: bool = v158 <= 57i32;
                                    v160
                                } else {
                                    false
                                };
                                let mut v164: i32 = if v161 {
                                    let mut v162: i32 = v158 - 48;
                                    v162
                                } else {
                                    let mut v163: i32 = -1;
                                    v163
                                };
                                let mut v165: i32 = 3i32;
                                let mut v166: i32 = 1i32;
                                let mut v167: i32 = usize::try_from(v165).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v166).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v168: bool = 48i32 <= v167;
                                let mut v170: bool = if v168 {
                                    let mut v169: bool = v167 <= 57i32;
                                    v169
                                } else {
                                    false
                                };
                                let mut v173: i32 = if v170 {
                                    let mut v171: i32 = v167 - 48;
                                    v171
                                } else {
                                    let mut v172: i32 = -1;
                                    v172
                                };
                                let mut v174: i32 = 3i32;
                                let mut v175: i32 = 2i32;
                                let mut v176: i32 = usize::try_from(v174).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v175).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v177: bool = 48i32 <= v176;
                                let mut v179: bool = if v177 {
                                    let mut v178: bool = v176 <= 57i32;
                                    v178
                                } else {
                                    false
                                };
                                let mut v182: i32 = if v179 {
                                    let mut v180: i32 = v176 - 48;
                                    v180
                                } else {
                                    let mut v181: i32 = -1;
                                    v181
                                };
                                let mut v183: bool = v155 == 1i32;
                                let mut v184: bool = 0 <= v164;
                                let mut v185: bool = v183 && v184;
                                let mut v209: US0 = if v185 {
                                    US0::US0_Valid(v164)
                                } else {
                                    let mut v187: bool = v155 == 2i32;
                                    let mut v188: bool = 0 <= v164;
                                    let mut v189: bool = 0 <= v173;
                                    let mut v191: bool = if v187 {
                                        let mut v190: bool = v188 && v189;
                                        v190
                                    } else {
                                        false
                                    };
                                    if v191 {
                                        let mut v192: i32 = v164 * 10 + v173;
                                        US0::US0_Valid(v192)
                                    } else {
                                        let mut v194: bool = v155 == 3i32;
                                        let mut v195: bool = 0 <= v164;
                                        let mut v196: bool = 0 <= v173;
                                        let mut v197: bool = 0 <= v182;
                                        let mut v200: bool = if v194 {
                                            if v195 {
                                                let mut v198: bool = v196 && v197;
                                                v198
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        };
                                        if v200 {
                                            let mut v201: i32 = v164 * 100 + v173 * 10 + v182;
                                            let mut v202: bool = v201 <= 100i32;
                                            if v202 {
                                                US0::US0_Valid(v201)
                                            } else {
                                                US0::US0_Invalid
                                            }
                                        } else {
                                            US0::US0_Invalid
                                        }
                                    }
                                };
                                let mut v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned|Active|Blocked|Paused|Done"); } LIT.with(|lit| lit.clone()) };
                                let mut v211: i32 = 4i32;
                                let mut v212: Rc<str> = usize::try_from(v211).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v213: u64 = v210.split("|").position(|item| item == &*v212).map(|index| index as u64).unwrap_or(u64::MAX);
                                let mut v214: bool = 0u64 == v213;
                                let mut v234: US1 = if v214 {
                                    let mut v215: US2 = US2::US2_AgilePlanned;
                                    US1::US1_Valid(v215.clone())
                                } else {
                                    let mut v217: bool = 1u64 == v213;
                                    if v217 {
                                        let mut v218: US2 = US2::US2_AgileActive;
                                        US1::US1_Valid(v218.clone())
                                    } else {
                                        let mut v220: bool = 2u64 == v213;
                                        if v220 {
                                            let mut v221: US2 = US2::US2_AgileBlocked;
                                            US1::US1_Valid(v221.clone())
                                        } else {
                                            let mut v223: bool = 3u64 == v213;
                                            if v223 {
                                                let mut v224: US2 = US2::US2_AgilePaused;
                                                US1::US1_Valid(v224.clone())
                                            } else {
                                                let mut v226: bool = 4u64 == v213;
                                                if v226 {
                                                    let mut v227: US2 = US2::US2_AgileDone;
                                                    US1::US1_Valid(v227.clone())
                                                } else {
                                                    US1::US1_Invalid
                                                }
                                            }
                                        }
                                    }
                                };
                                match &v209 {
                                    US0::US0_Invalid => {
                                        let mut v246: i32 = agile_mutation_error("progress must be 0..100");
                                        v246
                                    }
                                    US0::US0_Valid(v235) => {
                                        let mut v235: i32 = *v235;
                                        match &v234 {
                                            US1::US1_Invalid => {
                                                let mut v243: i32 = agile_mutation_error("invalid agile status");
                                                v243
                                            }
                                            US1::US1_Valid(v236) => {
                                                let mut v236: US2 = v236.clone();
                                                let mut v241: i32 = match &v236 {
                                                    US2::US2_AgileActive => {
                                                        1i32
                                                    }
                                                    US2::US2_AgileBlocked => {
                                                        2i32
                                                    }
                                                    US2::US2_AgileDone => {
                                                        4i32
                                                    }
                                                    US2::US2_AgilePaused => {
                                                        3i32
                                                    }
                                                    US2::US2_AgilePlanned => {
                                                        0i32
                                                    }
                                                };
                                                let mut v242: i32 = mutation_agile_set_owned(v151.as_ref(), v153.as_ref(), v235, v241);
                                                v242
                                            }
                                        }
                                    }
                                }
                            } else {
                                let mut v249: i32 = 0i32;
                                let mut v250: Rc<str> = usize::try_from(v249).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v251: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show"); } LIT.with(|lit| lit.clone()) };
                                let mut v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>\n<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
                                let mut v253: Rc<str> = v251.split("|").zip(v252.split(char::from(10u8))).find_map(|(key,item)| if key == &*v250 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                let mut v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v255: bool = v253 == v254 ;
                                if v255 {
                                    let mut v256: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>, show <root> <id> [--json]");
                                    v256
                                } else {
                                    let mut v257: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                                    let mut v258: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                                    let mut v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                                    let mut v260: Rc<str> = std::rc::Rc::<str>::from([&*v257, &*v250, &*v258, &*v253, &*v259].concat());
                                    let mut v261: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                                    let mut v262: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                                    let mut v263: Rc<str> = std::rc::Rc::<str>::from([&*v260, &*v261, &*v262, &*v254, &*v254].concat());
                                    let mut v264: i32 = agile_mutation_error(&v263);
                                    v264
                                }
                            }
                        }
                    } else {
                        let mut v268: i32 = 0i32;
                        let mut v269: Rc<str> = usize::try_from(v268).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v270: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show"); } LIT.with(|lit| lit.clone()) };
                        let mut v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>\n<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
                        let mut v272: Rc<str> = v270.split("|").zip(v271.split(char::from(10u8))).find_map(|(key,item)| if key == &*v269 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                        let mut v273: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v274: bool = v272 == v273 ;
                        if v274 {
                            let mut v275: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>, show <root> <id> [--json]");
                            v275
                        } else {
                            let mut v276: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                            let mut v277: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                            let mut v278: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                            let mut v279: Rc<str> = std::rc::Rc::<str>::from([&*v276, &*v269, &*v277, &*v272, &*v278].concat());
                            let mut v280: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                            let mut v281: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                            let mut v282: Rc<str> = std::rc::Rc::<str>::from([&*v279, &*v280, &*v281, &*v273, &*v273].concat());
                            let mut v283: i32 = agile_mutation_error(&v282);
                            v283
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_run() -> i32 {
    closure0()()
}
