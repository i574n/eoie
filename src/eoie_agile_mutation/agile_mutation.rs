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
                                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags|note|notes|rename"); } LIT.with(|lit| lit.clone()) };
                                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>\n<root> <id> <text> [--as <author>]\nfrom-title <root> <id>\n<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                                    let mut v26: Rc<str> = v24.split("|").zip(v25.split(char::from(10u8))).find_map(|(key,item)| if key == &*v23 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v28: bool = v26 == v27 ;
                                    if v28 {
                                        let mut v29: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], notes from-title <root> <id>, rename <root> <old-id> <new-id>");
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
                    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags|note|notes|rename"); } LIT.with(|lit| lit.clone()) };
                    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>\n<root> <id> <text> [--as <author>]\nfrom-title <root> <id>\n<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                    let mut v68: Rc<str> = v66.split("|").zip(v67.split(char::from(10u8))).find_map(|(key,item)| if key == &*v65 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v70: bool = v68 == v69 ;
                    if v70 {
                        let mut v71: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], notes from-title <root> <id>, rename <root> <old-id> <new-id>");
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
                let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show|note|notes|rename"); } LIT.with(|lit| lit.clone()) };
                let mut v86: i32 = 0i32;
                let mut v87: Rc<str> = usize::try_from(v86).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v88: u64 = v85.split("|").position(|item| item == &*v87).map(|index| index as u64).unwrap_or(u64::MAX);
                let mut v89: bool = 0u64 == v88;
                if v89 {
                    let mut v90: i32 = agile_task_inspection::eoie_agile_show_run();
                    v90
                } else {
                    let mut v91: bool = 1u64 == v88;
                    if v91 {
                        let mut v92: i32 = agile_task_notes::eoie_agile_note_run();
                        v92
                    } else {
                        let mut v93: bool = 2u64 == v88;
                        if v93 {
                            let mut v94: i32 = agile_title_notes::eoie_agile_notes_from_title_run();
                            v94
                        } else {
                            let mut v95: bool = 3u64 == v88;
                            if v95 {
                                let mut v96: i32 = agile_task_rename::eoie_agile_rename_run();
                                v96
                            } else {
                                let mut v97: i32 = 0i32;
                                let mut v98: Rc<str> = usize::try_from(v97).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags|note|notes|rename"); } LIT.with(|lit| lit.clone()) };
                                let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>\n<root> <id> <text> [--as <author>]\nfrom-title <root> <id>\n<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                                let mut v101: Rc<str> = v99.split("|").zip(v100.split(char::from(10u8))).find_map(|(key,item)| if key == &*v98 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v103: bool = v101 == v102 ;
                                if v103 {
                                    let mut v104: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], notes from-title <root> <id>, rename <root> <old-id> <new-id>");
                                    v104
                                } else {
                                    let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                                    let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                                    let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                                    let mut v108: Rc<str> = std::rc::Rc::<str>::from([&*v105, &*v98, &*v106, &*v101, &*v107].concat());
                                    let mut v109: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                                    let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                                    let mut v111: Rc<str> = std::rc::Rc::<str>::from([&*v108, &*v109, &*v110, &*v102, &*v102].concat());
                                    let mut v112: i32 = agile_mutation_error(&v111);
                                    v112
                                }
                            }
                        }
                    }
                }
            } else {
                let mut v118: i32 = 6i32;
                let mut v119: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v118 ;
                if v119 {
                    let mut v120: i32 = 0i32;
                    let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                    let mut v122: bool = usize::try_from(v120).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v121.as_ref());
                    if v122 {
                        let mut v123: i32 = agile_task_notes::eoie_agile_note_run();
                        v123
                    } else {
                        let mut v124: i32 = 0i32;
                        let mut v125: Rc<str> = usize::try_from(v124).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags|note|notes|rename"); } LIT.with(|lit| lit.clone()) };
                        let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>\n<root> <id> <text> [--as <author>]\nfrom-title <root> <id>\n<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                        let mut v128: Rc<str> = v126.split("|").zip(v127.split(char::from(10u8))).find_map(|(key,item)| if key == &*v125 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                        let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v130: bool = v128 == v129 ;
                        if v130 {
                            let mut v131: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], notes from-title <root> <id>, rename <root> <old-id> <new-id>");
                            v131
                        } else {
                            let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                            let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                            let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                            let mut v135: Rc<str> = std::rc::Rc::<str>::from([&*v132, &*v125, &*v133, &*v128, &*v134].concat());
                            let mut v136: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                            let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                            let mut v138: Rc<str> = std::rc::Rc::<str>::from([&*v135, &*v136, &*v137, &*v129, &*v129].concat());
                            let mut v139: i32 = agile_mutation_error(&v138);
                            v139
                        }
                    }
                } else {
                    let mut v142: i32 = 8i32;
                    let mut v143: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v142 ;
                    if v143 {
                        let mut v144: i32 = 0i32;
                        let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
                        let mut v146: bool = usize::try_from(v144).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v145.as_ref());
                        if v146 {
                            let mut v147: i32 = agile_task_creation::eoie_agile_add_run();
                            v147
                        } else {
                            let mut v148: i32 = 0i32;
                            let mut v149: Rc<str> = usize::try_from(v148).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags|note|notes|rename"); } LIT.with(|lit| lit.clone()) };
                            let mut v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>\n<root> <id> <text> [--as <author>]\nfrom-title <root> <id>\n<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                            let mut v152: Rc<str> = v150.split("|").zip(v151.split(char::from(10u8))).find_map(|(key,item)| if key == &*v149 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                            let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v154: bool = v152 == v153 ;
                            if v154 {
                                let mut v155: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], notes from-title <root> <id>, rename <root> <old-id> <new-id>");
                                v155
                            } else {
                                let mut v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                                let mut v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                                let mut v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                                let mut v159: Rc<str> = std::rc::Rc::<str>::from([&*v156, &*v149, &*v157, &*v152, &*v158].concat());
                                let mut v160: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                                let mut v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                                let mut v162: Rc<str> = std::rc::Rc::<str>::from([&*v159, &*v160, &*v161, &*v153, &*v153].concat());
                                let mut v163: i32 = agile_mutation_error(&v162);
                                v163
                            }
                        }
                    } else {
                        let mut v166: i32 = 5i32;
                        let mut v167: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v166 ;
                        if v167 {
                            let mut v168: i32 = 0i32;
                            let mut v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
                            let mut v170: bool = usize::try_from(v168).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v169.as_ref());
                            if v170 {
                                let mut v171: i32 = 3i32;
                                let mut v172: Rc<str> = usize::try_from(v171).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate|load|assignee"); } LIT.with(|lit| lit.clone()) };
                                let mut v174: bool = v173.split("|").any(|item| item == &*v172);
                                if v174 {
                                    let mut v175: i32 = agile_planning_revision::eoie_agile_planning_edit_run();
                                    v175
                                } else {
                                    let mut v176: i32 = agile_task_revision::eoie_agile_edit_run();
                                    v176
                                }
                            } else {
                                let mut v178: i32 = 0i32;
                                let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
                                let mut v180: bool = usize::try_from(v178).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v179.as_ref());
                                if v180 {
                                    let mut v181: i32 = 1i32;
                                    let mut v182: Rc<str> = usize::try_from(v181).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v183: i32 = 2i32;
                                    let mut v184: Rc<str> = usize::try_from(v183).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v185: i32 = 3i32;
                                    let mut v186: i32 = usize::try_from(v185).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
                                    let mut v187: i32 = 3i32;
                                    let mut v188: i32 = 0i32;
                                    let mut v189: i32 = usize::try_from(v187).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v188).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                    let mut v190: bool = 48i32 <= v189;
                                    let mut v192: bool = if v190 {
                                        let mut v191: bool = v189 <= 57i32;
                                        v191
                                    } else {
                                        false
                                    };
                                    let mut v195: i32 = if v192 {
                                        let mut v193: i32 = v189 - 48;
                                        v193
                                    } else {
                                        let mut v194: i32 = -1;
                                        v194
                                    };
                                    let mut v196: i32 = 3i32;
                                    let mut v197: i32 = 1i32;
                                    let mut v198: i32 = usize::try_from(v196).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v197).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                    let mut v199: bool = 48i32 <= v198;
                                    let mut v201: bool = if v199 {
                                        let mut v200: bool = v198 <= 57i32;
                                        v200
                                    } else {
                                        false
                                    };
                                    let mut v204: i32 = if v201 {
                                        let mut v202: i32 = v198 - 48;
                                        v202
                                    } else {
                                        let mut v203: i32 = -1;
                                        v203
                                    };
                                    let mut v205: i32 = 3i32;
                                    let mut v206: i32 = 2i32;
                                    let mut v207: i32 = usize::try_from(v205).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v206).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                    let mut v208: bool = 48i32 <= v207;
                                    let mut v210: bool = if v208 {
                                        let mut v209: bool = v207 <= 57i32;
                                        v209
                                    } else {
                                        false
                                    };
                                    let mut v213: i32 = if v210 {
                                        let mut v211: i32 = v207 - 48;
                                        v211
                                    } else {
                                        let mut v212: i32 = -1;
                                        v212
                                    };
                                    let mut v214: bool = v186 == 1i32;
                                    let mut v215: bool = 0 <= v195;
                                    let mut v216: bool = v214 && v215;
                                    let mut v240: US0 = if v216 {
                                        US0::US0_Valid(v195)
                                    } else {
                                        let mut v218: bool = v186 == 2i32;
                                        let mut v219: bool = 0 <= v195;
                                        let mut v220: bool = 0 <= v204;
                                        let mut v222: bool = if v218 {
                                            let mut v221: bool = v219 && v220;
                                            v221
                                        } else {
                                            false
                                        };
                                        if v222 {
                                            let mut v223: i32 = v195 * 10 + v204;
                                            US0::US0_Valid(v223)
                                        } else {
                                            let mut v225: bool = v186 == 3i32;
                                            let mut v226: bool = 0 <= v195;
                                            let mut v227: bool = 0 <= v204;
                                            let mut v228: bool = 0 <= v213;
                                            let mut v231: bool = if v225 {
                                                if v226 {
                                                    let mut v229: bool = v227 && v228;
                                                    v229
                                                } else {
                                                    false
                                                }
                                            } else {
                                                false
                                            };
                                            if v231 {
                                                let mut v232: i32 = v195 * 100 + v204 * 10 + v213;
                                                let mut v233: bool = v232 <= 100i32;
                                                if v233 {
                                                    US0::US0_Valid(v232)
                                                } else {
                                                    US0::US0_Invalid
                                                }
                                            } else {
                                                US0::US0_Invalid
                                            }
                                        }
                                    };
                                    let mut v241: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned|Active|Blocked|Paused|Done"); } LIT.with(|lit| lit.clone()) };
                                    let mut v242: i32 = 4i32;
                                    let mut v243: Rc<str> = usize::try_from(v242).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v244: u64 = v241.split("|").position(|item| item == &*v243).map(|index| index as u64).unwrap_or(u64::MAX);
                                    let mut v245: bool = 0u64 == v244;
                                    let mut v265: US1 = if v245 {
                                        let mut v246: US2 = US2::US2_AgilePlanned;
                                        US1::US1_Valid(v246.clone())
                                    } else {
                                        let mut v248: bool = 1u64 == v244;
                                        if v248 {
                                            let mut v249: US2 = US2::US2_AgileActive;
                                            US1::US1_Valid(v249.clone())
                                        } else {
                                            let mut v251: bool = 2u64 == v244;
                                            if v251 {
                                                let mut v252: US2 = US2::US2_AgileBlocked;
                                                US1::US1_Valid(v252.clone())
                                            } else {
                                                let mut v254: bool = 3u64 == v244;
                                                if v254 {
                                                    let mut v255: US2 = US2::US2_AgilePaused;
                                                    US1::US1_Valid(v255.clone())
                                                } else {
                                                    let mut v257: bool = 4u64 == v244;
                                                    if v257 {
                                                        let mut v258: US2 = US2::US2_AgileDone;
                                                        US1::US1_Valid(v258.clone())
                                                    } else {
                                                        US1::US1_Invalid
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    match &v240 {
                                        US0::US0_Invalid => {
                                            let mut v277: i32 = agile_mutation_error("progress must be 0..100");
                                            v277
                                        }
                                        US0::US0_Valid(v266) => {
                                            let mut v266: i32 = *v266;
                                            match &v265 {
                                                US1::US1_Invalid => {
                                                    let mut v274: i32 = agile_mutation_error("invalid agile status");
                                                    v274
                                                }
                                                US1::US1_Valid(v267) => {
                                                    let mut v267: US2 = v267.clone();
                                                    let mut v272: i32 = match &v267 {
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
                                                    let mut v273: i32 = mutation_agile_set_owned(v182.as_ref(), v184.as_ref(), v266, v272);
                                                    v273
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let mut v280: i32 = 0i32;
                                    let mut v281: Rc<str> = usize::try_from(v280).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                    let mut v282: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags|note|notes|rename"); } LIT.with(|lit| lit.clone()) };
                                    let mut v283: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>\n<root> <id> <text> [--as <author>]\nfrom-title <root> <id>\n<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                                    let mut v284: Rc<str> = v282.split("|").zip(v283.split(char::from(10u8))).find_map(|(key,item)| if key == &*v281 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                    let mut v285: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v286: bool = v284 == v285 ;
                                    if v286 {
                                        let mut v287: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], notes from-title <root> <id>, rename <root> <old-id> <new-id>");
                                        v287
                                    } else {
                                        let mut v288: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                                        let mut v289: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                                        let mut v290: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                                        let mut v291: Rc<str> = std::rc::Rc::<str>::from([&*v288, &*v281, &*v289, &*v284, &*v290].concat());
                                        let mut v292: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                                        let mut v293: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                                        let mut v294: Rc<str> = std::rc::Rc::<str>::from([&*v291, &*v292, &*v293, &*v285, &*v285].concat());
                                        let mut v295: i32 = agile_mutation_error(&v294);
                                        v295
                                    }
                                }
                            }
                        } else {
                            let mut v299: i32 = 0i32;
                            let mut v300: Rc<str> = usize::try_from(v299).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v301: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags|note|notes|rename"); } LIT.with(|lit| lit.clone()) };
                            let mut v302: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load <load>] [--assignee <assignee>] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>\n<root> <id> <text> [--as <author>]\nfrom-title <root> <id>\n<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                            let mut v303: Rc<str> = v301.split("|").zip(v302.split(char::from(10u8))).find_map(|(key,item)| if key == &*v300 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                            let mut v304: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v305: bool = v303 == v304 ;
                            if v305 {
                                let mut v306: i32 = agile_mutation_error("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root>, record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], notes from-title <root> <id>, rename <root> <old-id> <new-id>");
                                v306
                            } else {
                                let mut v307: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                                let mut v308: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
                                let mut v309: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
                                let mut v310: Rc<str> = std::rc::Rc::<str>::from([&*v307, &*v300, &*v308, &*v303, &*v309].concat());
                                let mut v311: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
                                let mut v312: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
                                let mut v313: Rc<str> = std::rc::Rc::<str>::from([&*v310, &*v311, &*v312, &*v304, &*v304].concat());
                                let mut v314: i32 = agile_mutation_error(&v313);
                                v314
                            }
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
