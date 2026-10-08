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
                                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit"); } LIT.with(|lit| lit.clone()) };
                                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>"); } LIT.with(|lit| lit.clone()) };
                                    let mut v26: Rc<str> = v24.split("|").zip(v25.split(char::from(10u8))).find_map(|(key,item)| if key == &*v23 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v28: bool = v26 == v27 ;
                                    if v28 {
                                        let mut v29: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>");
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
                let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin"); } LIT.with(|lit| lit.clone()) };
                let mut v54: bool = usize::try_from(v52).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v53.as_ref());
                if v54 {
                    let mut v55: i32 = 1i32;
                    let mut v56: Rc<str> = usize::try_from(v55).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v57: i32 = 2i32;
                    let mut v58: Rc<str> = usize::try_from(v57).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v59: i32 = eoie_lease_title_words();
                    let mut v60: bool = 0 < v59;
                    if v60 {
                        let mut v61: i32 = mutation_agile_begin_owned(v56.as_ref(), v58.as_ref());
                        v61
                    } else {
                        let mut v62: i32 = agile_mutation_error("begin title must contain 3..7 visible words");
                        v62
                    }
                } else {
                    let mut v64: i32 = 0i32;
                    let mut v65: Rc<str> = usize::try_from(v64).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit"); } LIT.with(|lit| lit.clone()) };
                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>"); } LIT.with(|lit| lit.clone()) };
                    let mut v68: Rc<str> = v66.split("|").zip(v67.split(char::from(10u8))).find_map(|(key,item)| if key == &*v65 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v70: bool = v68 == v69 ;
                    if v70 {
                        let mut v71: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>");
                        v71
                    } else {
                        let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                        let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                        let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                        let mut v75: Rc<str> = std::rc::Rc::<str>::from([&*v72, &*v65, &*v73, &*v68, &*v74].concat());
                        let mut v76: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                        let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                        let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v75, &*v76, &*v77, &*v69, &*v69].concat());
                        let mut v79: i32 = agile_mutation_error(&v78);
                        v79
                    }
                }
            }
        } else {
            let mut v83: i32 = 4i32;
            let mut v84: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v83 ;
            if v84 {
                let mut v85: i32 = 0i32;
                let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                let mut v87: bool = usize::try_from(v85).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v86.as_ref());
                if v87 {
                    let mut v88: i32 = agile_task_listing::eoie_agile_list_run();
                    v88
                } else {
                    let mut v89: i32 = 0i32;
                    let mut v90: Rc<str> = usize::try_from(v89).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit"); } LIT.with(|lit| lit.clone()) };
                    let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>"); } LIT.with(|lit| lit.clone()) };
                    let mut v93: Rc<str> = v91.split("|").zip(v92.split(char::from(10u8))).find_map(|(key,item)| if key == &*v90 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v95: bool = v93 == v94 ;
                    if v95 {
                        let mut v96: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>");
                        v96
                    } else {
                        let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                        let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                        let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                        let mut v100: Rc<str> = std::rc::Rc::<str>::from([&*v97, &*v90, &*v98, &*v93, &*v99].concat());
                        let mut v101: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                        let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                        let mut v103: Rc<str> = std::rc::Rc::<str>::from([&*v100, &*v101, &*v102, &*v94, &*v94].concat());
                        let mut v104: i32 = agile_mutation_error(&v103);
                        v104
                    }
                }
            } else {
                let mut v107: i32 = 8i32;
                let mut v108: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v107 ;
                if v108 {
                    let mut v109: i32 = 0i32;
                    let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
                    let mut v111: bool = usize::try_from(v109).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v110.as_ref());
                    if v111 {
                        let mut v112: i32 = agile_task_creation::eoie_agile_add_run();
                        v112
                    } else {
                        let mut v113: i32 = 0i32;
                        let mut v114: Rc<str> = usize::try_from(v113).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit"); } LIT.with(|lit| lit.clone()) };
                        let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>"); } LIT.with(|lit| lit.clone()) };
                        let mut v117: Rc<str> = v115.split("|").zip(v116.split(char::from(10u8))).find_map(|(key,item)| if key == &*v114 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                        let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v119: bool = v117 == v118 ;
                        if v119 {
                            let mut v120: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>");
                            v120
                        } else {
                            let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                            let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                            let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                            let mut v124: Rc<str> = std::rc::Rc::<str>::from([&*v121, &*v114, &*v122, &*v117, &*v123].concat());
                            let mut v125: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                            let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                            let mut v127: Rc<str> = std::rc::Rc::<str>::from([&*v124, &*v125, &*v126, &*v118, &*v118].concat());
                            let mut v128: i32 = agile_mutation_error(&v127);
                            v128
                        }
                    }
                } else {
                    let mut v131: i32 = 5i32;
                    let mut v132: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v131 ;
                    if v132 {
                        let mut v133: i32 = 0i32;
                        let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
                        let mut v135: bool = usize::try_from(v133).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v134.as_ref());
                        if v135 {
                            let mut v136: i32 = agile_task_revision::eoie_agile_edit_run();
                            v136
                        } else {
                            let mut v137: i32 = 0i32;
                            let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
                            let mut v139: bool = usize::try_from(v137).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v138.as_ref());
                            if v139 {
                                let mut v140: i32 = 1i32;
                                let mut v141: Rc<str> = usize::try_from(v140).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v142: i32 = 2i32;
                                let mut v143: Rc<str> = usize::try_from(v142).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v144: i32 = 3i32;
                                let mut v145: i32 = usize::try_from(v144).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
                                let mut v146: i32 = 3i32;
                                let mut v147: i32 = 0i32;
                                let mut v148: i32 = usize::try_from(v146).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v147).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v149: bool = 48i32 <= v148;
                                let mut v151: bool = if v149 {
                                    let mut v150: bool = v148 <= 57i32;
                                    v150
                                } else {
                                    false
                                };
                                let mut v154: i32 = if v151 {
                                    let mut v152: i32 = v148 - 48;
                                    v152
                                } else {
                                    let mut v153: i32 = -1;
                                    v153
                                };
                                let mut v155: i32 = 3i32;
                                let mut v156: i32 = 1i32;
                                let mut v157: i32 = usize::try_from(v155).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v156).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v158: bool = 48i32 <= v157;
                                let mut v160: bool = if v158 {
                                    let mut v159: bool = v157 <= 57i32;
                                    v159
                                } else {
                                    false
                                };
                                let mut v163: i32 = if v160 {
                                    let mut v161: i32 = v157 - 48;
                                    v161
                                } else {
                                    let mut v162: i32 = -1;
                                    v162
                                };
                                let mut v164: i32 = 3i32;
                                let mut v165: i32 = 2i32;
                                let mut v166: i32 = usize::try_from(v164).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v165).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v167: bool = 48i32 <= v166;
                                let mut v169: bool = if v167 {
                                    let mut v168: bool = v166 <= 57i32;
                                    v168
                                } else {
                                    false
                                };
                                let mut v172: i32 = if v169 {
                                    let mut v170: i32 = v166 - 48;
                                    v170
                                } else {
                                    let mut v171: i32 = -1;
                                    v171
                                };
                                let mut v173: bool = v145 == 1i32;
                                let mut v174: bool = 0 <= v154;
                                let mut v175: bool = v173 && v174;
                                let mut v199: US0 = if v175 {
                                    US0::US0_0(v154)
                                } else {
                                    let mut v177: bool = v145 == 2i32;
                                    let mut v178: bool = 0 <= v154;
                                    let mut v179: bool = 0 <= v163;
                                    let mut v181: bool = if v177 {
                                        let mut v180: bool = v178 && v179;
                                        v180
                                    } else {
                                        false
                                    };
                                    if v181 {
                                        let mut v182: i32 = v154 * 10 + v163;
                                        US0::US0_0(v182)
                                    } else {
                                        let mut v184: bool = v145 == 3i32;
                                        let mut v185: bool = 0 <= v154;
                                        let mut v186: bool = 0 <= v163;
                                        let mut v187: bool = 0 <= v172;
                                        let mut v190: bool = if v184 {
                                            if v185 {
                                                let mut v188: bool = v186 && v187;
                                                v188
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        };
                                        if v190 {
                                            let mut v191: i32 = v154 * 100 + v163 * 10 + v172;
                                            let mut v192: bool = v191 <= 100i32;
                                            if v192 {
                                                US0::US0_0(v191)
                                            } else {
                                                US0::US0_1
                                            }
                                        } else {
                                            US0::US0_1
                                        }
                                    }
                                };
                                let mut v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned|Active|Blocked|Paused|Done"); } LIT.with(|lit| lit.clone()) };
                                let mut v201: i32 = 4i32;
                                let mut v202: Rc<str> = usize::try_from(v201).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v203: u64 = v200.split("|").position(|item| item == &*v202).map(|index| index as u64).unwrap_or(u64::MAX);
                                let mut v204: bool = 0u64 == v203;
                                let mut v224: US1 = if v204 {
                                    let mut v205: US2 = US2::US2_0;
                                    US1::US1_0(v205.clone())
                                } else {
                                    let mut v207: bool = 1u64 == v203;
                                    if v207 {
                                        let mut v208: US2 = US2::US2_1;
                                        US1::US1_0(v208.clone())
                                    } else {
                                        let mut v210: bool = 2u64 == v203;
                                        if v210 {
                                            let mut v211: US2 = US2::US2_2;
                                            US1::US1_0(v211.clone())
                                        } else {
                                            let mut v213: bool = 3u64 == v203;
                                            if v213 {
                                                let mut v214: US2 = US2::US2_3;
                                                US1::US1_0(v214.clone())
                                            } else {
                                                let mut v216: bool = 4u64 == v203;
                                                if v216 {
                                                    let mut v217: US2 = US2::US2_4;
                                                    US1::US1_0(v217.clone())
                                                } else {
                                                    US1::US1_1
                                                }
                                            }
                                        }
                                    }
                                };
                                match &v199 {
                                    US0::US0_1 => {
                                        let mut v236: i32 = agile_mutation_error("progress must be 0..100");
                                        v236
                                    }
                                    US0::US0_0(v225) => {
                                        let mut v225: i32 = *v225;
                                        match &v224 {
                                            US1::US1_1 => {
                                                let mut v233: i32 = agile_mutation_error("invalid agile status");
                                                v233
                                            }
                                            US1::US1_0(v226) => {
                                                let mut v226: US2 = v226.clone();
                                                let mut v231: i32 = match &v226 {
                                                    US2::US2_1 => {
                                                        1i32
                                                    }
                                                    US2::US2_2 => {
                                                        2i32
                                                    }
                                                    US2::US2_4 => {
                                                        4i32
                                                    }
                                                    US2::US2_3 => {
                                                        3i32
                                                    }
                                                    US2::US2_0 => {
                                                        0i32
                                                    }
                                                };
                                                let mut v232: i32 = mutation_agile_set_owned(v141.as_ref(), v143.as_ref(), v225, v231);
                                                v232
                                            }
                                        }
                                    }
                                }
                            } else {
                                let mut v239: i32 = 0i32;
                                let mut v240: Rc<str> = usize::try_from(v239).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v241: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit"); } LIT.with(|lit| lit.clone()) };
                                let mut v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>"); } LIT.with(|lit| lit.clone()) };
                                let mut v243: Rc<str> = v241.split("|").zip(v242.split(char::from(10u8))).find_map(|(key,item)| if key == &*v240 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                let mut v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v245: bool = v243 == v244 ;
                                if v245 {
                                    let mut v246: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>");
                                    v246
                                } else {
                                    let mut v247: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                                    let mut v248: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                                    let mut v249: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                                    let mut v250: Rc<str> = std::rc::Rc::<str>::from([&*v247, &*v240, &*v248, &*v243, &*v249].concat());
                                    let mut v251: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                                    let mut v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                                    let mut v253: Rc<str> = std::rc::Rc::<str>::from([&*v250, &*v251, &*v252, &*v244, &*v244].concat());
                                    let mut v254: i32 = agile_mutation_error(&v253);
                                    v254
                                }
                            }
                        }
                    } else {
                        let mut v258: i32 = 0i32;
                        let mut v259: Rc<str> = usize::try_from(v258).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v260: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit"); } LIT.with(|lit| lit.clone()) };
                        let mut v261: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>"); } LIT.with(|lit| lit.clone()) };
                        let mut v262: Rc<str> = v260.split("|").zip(v261.split(char::from(10u8))).find_map(|(key,item)| if key == &*v259 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                        let mut v263: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v264: bool = v262 == v263 ;
                        if v264 {
                            let mut v265: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags> <value>");
                            v265
                        } else {
                            let mut v266: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                            let mut v267: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                            let mut v268: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                            let mut v269: Rc<str> = std::rc::Rc::<str>::from([&*v266, &*v259, &*v267, &*v262, &*v268].concat());
                            let mut v270: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                            let mut v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                            let mut v272: Rc<str> = std::rc::Rc::<str>::from([&*v269, &*v270, &*v271, &*v263, &*v263].concat());
                            let mut v273: i32 = agile_mutation_error(&v272);
                            v273
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
pub fn eoie_agile_run() -> i32 {
    closure0()()
}
