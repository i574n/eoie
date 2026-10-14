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
fn agile_counted_entry_1() -> i32 {
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
                                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags"); } LIT.with(|lit| lit.clone()) };
                                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                                    let mut v26: Rc<str> = v24.split("|").zip(v25.split(char::from(10u8))).find_map(|(key,item)| if key == &*v23 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v28: bool = v26 == v27 ;
                                    if v28 {
                                        let mut v29: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>");
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
            let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
            let mut v50: bool = usize::try_from(v48).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v49.as_ref());
            if v50 {
                let mut v51: i32 = agile_task_inspection::eoie_agile_show_run();
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
                    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags"); } LIT.with(|lit| lit.clone()) };
                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                    let mut v68: Rc<str> = v66.split("|").zip(v67.split(char::from(10u8))).find_map(|(key,item)| if key == &*v65 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v70: bool = v68 == v69 ;
                    if v70 {
                        let mut v71: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>");
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
                let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                let mut v87: bool = usize::try_from(v85).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v86.as_ref());
                if v87 {
                    let mut v88: i32 = agile_task_inspection::eoie_agile_show_run();
                    v88
                } else {
                    let mut v89: i32 = 0i32;
                    let mut v90: Rc<str> = usize::try_from(v89).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags"); } LIT.with(|lit| lit.clone()) };
                    let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                    let mut v93: Rc<str> = v91.split("|").zip(v92.split(char::from(10u8))).find_map(|(key,item)| if key == &*v90 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v95: bool = v93 == v94 ;
                    if v95 {
                        let mut v96: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>");
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
                        let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags"); } LIT.with(|lit| lit.clone()) };
                        let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                        let mut v117: Rc<str> = v115.split("|").zip(v116.split(char::from(10u8))).find_map(|(key,item)| if key == &*v114 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                        let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v119: bool = v117 == v118 ;
                        if v119 {
                            let mut v120: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>");
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
                            let mut v136: i32 = 3i32;
                            let mut v137: Rc<str> = usize::try_from(v136).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate|load|assignee"); } LIT.with(|lit| lit.clone()) };
                            let mut v139: bool = v138.split("|").any(|item| item == &*v137);
                            if v139 {
                                let mut v140: i32 = agile_planning_revision::eoie_agile_planning_edit_run();
                                v140
                            } else {
                                let mut v141: i32 = agile_task_revision::eoie_agile_edit_run();
                                v141
                            }
                        } else {
                            let mut v143: i32 = 0i32;
                            let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
                            let mut v145: bool = usize::try_from(v143).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v144.as_ref());
                            if v145 {
                                let mut v146: i32 = 1i32;
                                let mut v147: Rc<str> = usize::try_from(v146).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v148: i32 = 2i32;
                                let mut v149: Rc<str> = usize::try_from(v148).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v150: i32 = 3i32;
                                let mut v151: i32 = usize::try_from(v150).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
                                let mut v152: i32 = 3i32;
                                let mut v153: i32 = 0i32;
                                let mut v154: i32 = usize::try_from(v152).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v153).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v155: bool = 48i32 <= v154;
                                let mut v157: bool = if v155 {
                                    let mut v156: bool = v154 <= 57i32;
                                    v156
                                } else {
                                    false
                                };
                                let mut v160: i32 = if v157 {
                                    let mut v158: i32 = v154 - 48;
                                    v158
                                } else {
                                    let mut v159: i32 = -1;
                                    v159
                                };
                                let mut v161: i32 = 3i32;
                                let mut v162: i32 = 1i32;
                                let mut v163: i32 = usize::try_from(v161).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v162).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v164: bool = 48i32 <= v163;
                                let mut v166: bool = if v164 {
                                    let mut v165: bool = v163 <= 57i32;
                                    v165
                                } else {
                                    false
                                };
                                let mut v169: i32 = if v166 {
                                    let mut v167: i32 = v163 - 48;
                                    v167
                                } else {
                                    let mut v168: i32 = -1;
                                    v168
                                };
                                let mut v170: i32 = 3i32;
                                let mut v171: i32 = 2i32;
                                let mut v172: i32 = usize::try_from(v170).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v171).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v173: bool = 48i32 <= v172;
                                let mut v175: bool = if v173 {
                                    let mut v174: bool = v172 <= 57i32;
                                    v174
                                } else {
                                    false
                                };
                                let mut v178: i32 = if v175 {
                                    let mut v176: i32 = v172 - 48;
                                    v176
                                } else {
                                    let mut v177: i32 = -1;
                                    v177
                                };
                                let mut v179: bool = v151 == 1i32;
                                let mut v180: bool = 0 <= v160;
                                let mut v181: bool = v179 && v180;
                                let mut v205: US0 = if v181 {
                                    US0::US0_Valid(v160)
                                } else {
                                    let mut v183: bool = v151 == 2i32;
                                    let mut v184: bool = 0 <= v160;
                                    let mut v185: bool = 0 <= v169;
                                    let mut v187: bool = if v183 {
                                        let mut v186: bool = v184 && v185;
                                        v186
                                    } else {
                                        false
                                    };
                                    if v187 {
                                        let mut v188: i32 = v160 * 10 + v169;
                                        US0::US0_Valid(v188)
                                    } else {
                                        let mut v190: bool = v151 == 3i32;
                                        let mut v191: bool = 0 <= v160;
                                        let mut v192: bool = 0 <= v169;
                                        let mut v193: bool = 0 <= v178;
                                        let mut v196: bool = if v190 {
                                            if v191 {
                                                let mut v194: bool = v192 && v193;
                                                v194
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        };
                                        if v196 {
                                            let mut v197: i32 = v160 * 100 + v169 * 10 + v178;
                                            let mut v198: bool = v197 <= 100i32;
                                            if v198 {
                                                US0::US0_Valid(v197)
                                            } else {
                                                US0::US0_Invalid
                                            }
                                        } else {
                                            US0::US0_Invalid
                                        }
                                    }
                                };
                                let mut v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned|Active|Blocked|Paused|Done"); } LIT.with(|lit| lit.clone()) };
                                let mut v207: i32 = 4i32;
                                let mut v208: Rc<str> = usize::try_from(v207).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v209: u64 = v206.split("|").position(|item| item == &*v208).map(|index| index as u64).unwrap_or(u64::MAX);
                                let mut v210: bool = 0u64 == v209;
                                let mut v230: US1 = if v210 {
                                    let mut v211: US2 = US2::US2_AgilePlanned;
                                    US1::US1_Valid(v211.clone())
                                } else {
                                    let mut v213: bool = 1u64 == v209;
                                    if v213 {
                                        let mut v214: US2 = US2::US2_AgileActive;
                                        US1::US1_Valid(v214.clone())
                                    } else {
                                        let mut v216: bool = 2u64 == v209;
                                        if v216 {
                                            let mut v217: US2 = US2::US2_AgileBlocked;
                                            US1::US1_Valid(v217.clone())
                                        } else {
                                            let mut v219: bool = 3u64 == v209;
                                            if v219 {
                                                let mut v220: US2 = US2::US2_AgilePaused;
                                                US1::US1_Valid(v220.clone())
                                            } else {
                                                let mut v222: bool = 4u64 == v209;
                                                if v222 {
                                                    let mut v223: US2 = US2::US2_AgileDone;
                                                    US1::US1_Valid(v223.clone())
                                                } else {
                                                    US1::US1_Invalid
                                                }
                                            }
                                        }
                                    }
                                };
                                match &v205 {
                                    US0::US0_Invalid => {
                                        let mut v242: i32 = agile_mutation_error("progress must be 0..100");
                                        v242
                                    }
                                    US0::US0_Valid(v231) => {
                                        let mut v231: i32 = *v231;
                                        match &v230 {
                                            US1::US1_Invalid => {
                                                let mut v239: i32 = agile_mutation_error("invalid agile status");
                                                v239
                                            }
                                            US1::US1_Valid(v232) => {
                                                let mut v232: US2 = v232.clone();
                                                let mut v237: i32 = match &v232 {
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
                                                let mut v238: i32 = mutation_agile_set_owned(v147.as_ref(), v149.as_ref(), v231, v237);
                                                v238
                                            }
                                        }
                                    }
                                }
                            } else {
                                let mut v245: i32 = 0i32;
                                let mut v246: Rc<str> = usize::try_from(v245).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v247: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags"); } LIT.with(|lit| lit.clone()) };
                                let mut v248: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                                let mut v249: Rc<str> = v247.split("|").zip(v248.split(char::from(10u8))).find_map(|(key,item)| if key == &*v246 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                let mut v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v251: bool = v249 == v250 ;
                                if v251 {
                                    let mut v252: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>");
                                    v252
                                } else {
                                    let mut v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                                    let mut v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                                    let mut v255: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                                    let mut v256: Rc<str> = std::rc::Rc::<str>::from([&*v253, &*v246, &*v254, &*v249, &*v255].concat());
                                    let mut v257: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                                    let mut v258: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                                    let mut v259: Rc<str> = std::rc::Rc::<str>::from([&*v256, &*v257, &*v258, &*v250, &*v250].concat());
                                    let mut v260: i32 = agile_mutation_error(&v259);
                                    v260
                                }
                            }
                        }
                    } else {
                        let mut v264: i32 = 0i32;
                        let mut v265: Rc<str> = usize::try_from(v264).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v266: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags"); } LIT.with(|lit| lit.clone()) };
                        let mut v267: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                        let mut v268: Rc<str> = v266.split("|").zip(v267.split(char::from(10u8))).find_map(|(key,item)| if key == &*v265 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                        let mut v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v270: bool = v268 == v269 ;
                        if v270 {
                            let mut v271: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>");
                            v271
                        } else {
                            let mut v272: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                            let mut v273: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                            let mut v274: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                            let mut v275: Rc<str> = std::rc::Rc::<str>::from([&*v272, &*v265, &*v273, &*v268, &*v274].concat());
                            let mut v276: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                            let mut v277: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                            let mut v278: Rc<str> = std::rc::Rc::<str>::from([&*v275, &*v276, &*v277, &*v269, &*v269].concat());
                            let mut v279: i32 = agile_mutation_error(&v278);
                            v279
                        }
                    }
                }
            }
        }
    }
}
fn agile_entry_0() -> i32 {
    let mut v0: i32 = std::env::args().skip(2).count() as i32;
    let mut v1: bool = 2i32 <= v0;
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list|plan|migrate-tags"); } LIT.with(|lit| lit.clone()) };
        let mut v3: i32 = 0i32;
        let mut v4: Rc<str> = usize::try_from(v3).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v5: u64 = v2.split("|").position(|item| item == &*v4).map(|index| index as u64).unwrap_or(u64::MAX);
        let mut v6: bool = 0u64 == v5;
        if v6 {
            let mut v7: i32 = agile_task_listing::eoie_agile_list_run();
            v7
        } else {
            let mut v8: bool = 1u64 == v5;
            if v8 {
                let mut v9: i32 = agile_planning_rollup::eoie_agile_planning_rollup_run();
                v9
            } else {
                let mut v10: bool = 2u64 == v5;
                if v10 {
                    let mut v11: i32 = agile_planning_migration::eoie_agile_migrate_tags_run();
                    v11
                } else {
                    agile_counted_entry_1()
                }
            }
        }
    } else {
        agile_counted_entry_1()
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
