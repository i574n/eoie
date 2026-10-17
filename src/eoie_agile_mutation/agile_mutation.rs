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
enum US1 {
    US1_AgileInit,
    US1_AgileBegin,
    US1_AgileLease,
    US1_AgileStatus,
    US1_AgileHandoff,
    US1_AgileList,
    US1_AgileCheck,
    US1_AgileRecord,
    US1_AgileSet,
    US1_AgileAdd,
    US1_AgileEdit,
    US1_AgileShow,
    US1_AgilePlan,
    US1_AgileMigrateTags,
    US1_AgileNote,
    US1_AgileNotes,
    US1_AgileRename,
    US1_AgileTodos,
}
#[derive(Clone)]
enum US0 {
    US0_AgileKnown(US1),
    US0_AgileUnknownName,
}
#[derive(Clone)]
enum US2 {
    US2_Valid(i32),
    US2_Invalid,
}
#[derive(Clone)]
enum US4 {
    US4_AgilePlanned,
    US4_AgileActive,
    US4_AgileBlocked,
    US4_AgilePaused,
    US4_AgileDone,
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US4),
    US3_Invalid,
}
fn agile_subcommand_shape_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags|note|notes|rename|todos"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>\n<root> <id> <text> [--as <author>]\nfrom-title <root> <id>\n<root> <old-id> <new-id>\n<root> [<path>...] [--json]"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = v1.split("|").zip(v2.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    v3.clone()
}
fn agile_usage_error_1() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: Rc<str> = agile_subcommand_shape_2(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2 == v3 ;
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root> [--compiler <absolute>], record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags>, edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], notes from-title <root> <id>, rename <root> <old-id> <new-id>, todos <root> [<path>...] [--json]"); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = agile_mutation_error(&v5);
        v6
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v1, &*v8, &*v2, &*v9].concat());
        let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v3, &*v3].concat());
        let mut v14: i32 = agile_mutation_error(&v13);
        v14
    }
}
fn agile_subcommand_of_3(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0 == v1 ;
    let mut v6: US0 = if v2 {
        let mut v3: US1 = US1::US1_AgileInit;
        US0::US0_AgileKnown(v3.clone())
    } else {
        US0::US0_AgileUnknownName
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin"); } LIT.with(|lit| lit.clone()) };
    let mut v8: bool = v0 == v7 ;
    let mut v11: US0 = if v8 {
        let mut v9: US1 = US1::US1_AgileBegin;
        US0::US0_AgileKnown(v9.clone())
    } else {
        v6.clone()
    };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease"); } LIT.with(|lit| lit.clone()) };
    let mut v13: bool = v0 == v12 ;
    let mut v16: US0 = if v13 {
        let mut v14: US1 = US1::US1_AgileLease;
        US0::US0_AgileKnown(v14.clone())
    } else {
        v11.clone()
    };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let mut v18: bool = v0 == v17 ;
    let mut v21: US0 = if v18 {
        let mut v19: US1 = US1::US1_AgileStatus;
        US0::US0_AgileKnown(v19.clone())
    } else {
        v16.clone()
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff"); } LIT.with(|lit| lit.clone()) };
    let mut v23: bool = v0 == v22 ;
    let mut v26: US0 = if v23 {
        let mut v24: US1 = US1::US1_AgileHandoff;
        US0::US0_AgileKnown(v24.clone())
    } else {
        v21.clone()
    };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
    let mut v28: bool = v0 == v27 ;
    let mut v31: US0 = if v28 {
        let mut v29: US1 = US1::US1_AgileList;
        US0::US0_AgileKnown(v29.clone())
    } else {
        v26.clone()
    };
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
    let mut v33: bool = v0 == v32 ;
    let mut v36: US0 = if v33 {
        let mut v34: US1 = US1::US1_AgileCheck;
        US0::US0_AgileKnown(v34.clone())
    } else {
        v31.clone()
    };
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record"); } LIT.with(|lit| lit.clone()) };
    let mut v38: bool = v0 == v37 ;
    let mut v41: US0 = if v38 {
        let mut v39: US1 = US1::US1_AgileRecord;
        US0::US0_AgileKnown(v39.clone())
    } else {
        v36.clone()
    };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
    let mut v43: bool = v0 == v42 ;
    let mut v46: US0 = if v43 {
        let mut v44: US1 = US1::US1_AgileSet;
        US0::US0_AgileKnown(v44.clone())
    } else {
        v41.clone()
    };
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
    let mut v48: bool = v0 == v47 ;
    let mut v51: US0 = if v48 {
        let mut v49: US1 = US1::US1_AgileAdd;
        US0::US0_AgileKnown(v49.clone())
    } else {
        v46.clone()
    };
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
    let mut v53: bool = v0 == v52 ;
    let mut v56: US0 = if v53 {
        let mut v54: US1 = US1::US1_AgileEdit;
        US0::US0_AgileKnown(v54.clone())
    } else {
        v51.clone()
    };
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
    let mut v58: bool = v0 == v57 ;
    let mut v61: US0 = if v58 {
        let mut v59: US1 = US1::US1_AgileShow;
        US0::US0_AgileKnown(v59.clone())
    } else {
        v56.clone()
    };
    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan"); } LIT.with(|lit| lit.clone()) };
    let mut v63: bool = v0 == v62 ;
    let mut v66: US0 = if v63 {
        let mut v64: US1 = US1::US1_AgilePlan;
        US0::US0_AgileKnown(v64.clone())
    } else {
        v61.clone()
    };
    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags"); } LIT.with(|lit| lit.clone()) };
    let mut v68: bool = v0 == v67 ;
    let mut v71: US0 = if v68 {
        let mut v69: US1 = US1::US1_AgileMigrateTags;
        US0::US0_AgileKnown(v69.clone())
    } else {
        v66.clone()
    };
    let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
    let mut v73: bool = v0 == v72 ;
    let mut v76: US0 = if v73 {
        let mut v74: US1 = US1::US1_AgileNote;
        US0::US0_AgileKnown(v74.clone())
    } else {
        v71.clone()
    };
    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
    let mut v78: bool = v0 == v77 ;
    let mut v81: US0 = if v78 {
        let mut v79: US1 = US1::US1_AgileNotes;
        US0::US0_AgileKnown(v79.clone())
    } else {
        v76.clone()
    };
    let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
    let mut v83: bool = v0 == v82 ;
    let mut v86: US0 = if v83 {
        let mut v84: US1 = US1::US1_AgileRename;
        US0::US0_AgileKnown(v84.clone())
    } else {
        v81.clone()
    };
    let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
    let mut v88: bool = v0 == v87 ;
    if v88 {
        let mut v89: US1 = US1::US1_AgileTodos;
        US0::US0_AgileKnown(v89.clone())
    } else {
        v86.clone()
    }
}
fn agile_entry_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: US0 = agile_subcommand_of_3(v1.clone());
    match &v2 {
        US0::US0_AgileKnown(v3) => {
            let mut v3: US1 = v3.clone();
            match &v3 {
                US1::US1_AgileAdd => {
                    let mut v170: i32 = 8i32;
                    let mut v171: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v170 ;
                    if v171 {
                        let mut v172: i32 = agile_task_creation::eoie_agile_add_run();
                        v172
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileBegin => {
                    let mut v11: i32 = 3i32;
                    let mut v12: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v11 ;
                    if v12 {
                        let mut v13: i32 = 1i32;
                        let mut v14: Rc<str> = usize::try_from(v13).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v15: i32 = 2i32;
                        let mut v16: Rc<str> = usize::try_from(v15).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v17: i32 = eoie_lease_title_words();
                        let mut v18: bool = 0 < v17;
                        if v18 {
                            let mut v19: i32 = mutation_agile_begin_owned(v14.as_ref(), v16.as_ref());
                            v19
                        } else {
                            let mut v20: i32 = agile_mutation_error("begin title must contain 3..7 visible words");
                            v20
                        }
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileCheck => {
                    let mut v50: i32 = 2i32;
                    let mut v51: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v50 ;
                    if v51 {
                        let mut v52: i32 = 1i32;
                        let mut v53: Rc<str> = usize::try_from(v52).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v54: i32 = mutation_agile_check_owned(v53.as_ref());
                        v54
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileEdit => {
                    let mut v175: i32 = 5i32;
                    let mut v176: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v175 ;
                    if v176 {
                        let mut v177: i32 = 3i32;
                        let mut v178: Rc<str> = usize::try_from(v177).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate|load|assignee"); } LIT.with(|lit| lit.clone()) };
                        let mut v180: bool = v179.split("|").any(|item| item == &*v178);
                        if v180 {
                            let mut v181: i32 = agile_planning_revision::eoie_agile_planning_edit_run();
                            v181
                        } else {
                            let mut v182: i32 = agile_task_revision::eoie_agile_edit_run();
                            v182
                        }
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileHandoff => {
                    let mut v38: i32 = 2i32;
                    let mut v39: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v38 ;
                    if v39 {
                        let mut v40: i32 = 1i32;
                        let mut v41: Rc<str> = usize::try_from(v40).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v42: i32 = handoff_render_owned(v41.as_ref());
                        v42
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileInit => {
                    let mut v4: i32 = 2i32;
                    let mut v5: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v4 ;
                    if v5 {
                        let mut v6: i32 = 1i32;
                        let mut v7: Rc<str> = usize::try_from(v6).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v8: i32 = agile_backlog_scaffold::eoie_agile_init_run();
                        v8
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileLease => {
                    let mut v24: i32 = 2i32;
                    let mut v25: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v24 ;
                    if v25 {
                        let mut v26: i32 = 1i32;
                        let mut v27: Rc<str> = usize::try_from(v26).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v28: i32 = mutation_agile_lease_status_owned(v27.as_ref());
                        v28
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileList => {
                    let mut v45: i32 = std::env::args().skip(2).count() as i32;
                    let mut v46: bool = 2i32 <= v45;
                    if v46 {
                        let mut v47: i32 = agile_task_listing::eoie_agile_list_run();
                        v47
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileMigrateTags => {
                    let mut v199: i32 = std::env::args().skip(2).count() as i32;
                    let mut v200: bool = 2i32 <= v199;
                    if v200 {
                        let mut v201: i32 = agile_planning_migration::eoie_agile_migrate_tags_run();
                        v201
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileNote => {
                    let mut v204: i32 = 4i32;
                    let mut v205: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v204 ;
                    let mut v208: bool = if v205 {
                        true
                    } else {
                        let mut v206: i32 = 6i32;
                        let mut v207: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v206 ;
                        v207
                    };
                    if v208 {
                        let mut v209: i32 = agile_task_notes::eoie_agile_note_run();
                        v209
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileNotes => {
                    let mut v212: i32 = 4i32;
                    let mut v213: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v212 ;
                    if v213 {
                        let mut v214: i32 = agile_title_notes::eoie_agile_notes_from_title_run();
                        v214
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgilePlan => {
                    let mut v194: i32 = std::env::args().skip(2).count() as i32;
                    let mut v195: bool = 2i32 <= v194;
                    if v195 {
                        let mut v196: i32 = agile_planning_rollup::eoie_agile_planning_rollup_run();
                        v196
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileRecord => {
                    let mut v57: i32 = 2i32;
                    let mut v58: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v57 ;
                    if v58 {
                        let mut v59: i32 = 1i32;
                        let mut v60: Rc<str> = usize::try_from(v59).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v61: i32 = mutation_agile_record_owned(v60.as_ref());
                        v61
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileRename => {
                    let mut v217: i32 = 4i32;
                    let mut v218: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v217 ;
                    if v218 {
                        let mut v219: i32 = agile_task_rename::eoie_agile_rename_run();
                        v219
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileSet => {
                    let mut v64: i32 = 5i32;
                    let mut v65: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v64 ;
                    if v65 {
                        let mut v66: i32 = 1i32;
                        let mut v67: Rc<str> = usize::try_from(v66).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v68: i32 = 2i32;
                        let mut v69: Rc<str> = usize::try_from(v68).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v70: i32 = 3i32;
                        let mut v71: i32 = usize::try_from(v70).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
                        let mut v72: i32 = 3i32;
                        let mut v73: i32 = 0i32;
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
                        let mut v82: i32 = 1i32;
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
                        let mut v90: i32 = 3i32;
                        let mut v91: i32 = 2i32;
                        let mut v92: i32 = usize::try_from(v90).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v91).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                        let mut v93: bool = 48i32 <= v92;
                        let mut v95: bool = if v93 {
                            let mut v94: bool = v92 <= 57i32;
                            v94
                        } else {
                            false
                        };
                        let mut v98: i32 = if v95 {
                            let mut v96: i32 = v92 - 48;
                            v96
                        } else {
                            let mut v97: i32 = -1;
                            v97
                        };
                        let mut v99: bool = v71 == 1i32;
                        let mut v100: bool = 0 <= v80;
                        let mut v101: bool = v99 && v100;
                        let mut v125: US2 = if v101 {
                            US2::US2_Valid(v80)
                        } else {
                            let mut v103: bool = v71 == 2i32;
                            let mut v104: bool = 0 <= v80;
                            let mut v105: bool = 0 <= v89;
                            let mut v107: bool = if v103 {
                                let mut v106: bool = v104 && v105;
                                v106
                            } else {
                                false
                            };
                            if v107 {
                                let mut v108: i32 = v80 * 10 + v89;
                                US2::US2_Valid(v108)
                            } else {
                                let mut v110: bool = v71 == 3i32;
                                let mut v111: bool = 0 <= v80;
                                let mut v112: bool = 0 <= v89;
                                let mut v113: bool = 0 <= v98;
                                let mut v116: bool = if v110 {
                                    if v111 {
                                        let mut v114: bool = v112 && v113;
                                        v114
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                };
                                if v116 {
                                    let mut v117: i32 = v80 * 100 + v89 * 10 + v98;
                                    let mut v118: bool = v117 <= 100i32;
                                    if v118 {
                                        US2::US2_Valid(v117)
                                    } else {
                                        US2::US2_Invalid
                                    }
                                } else {
                                    US2::US2_Invalid
                                }
                            }
                        };
                        let mut v126: i32 = 4i32;
                        let mut v127: Rc<str> = usize::try_from(v126).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
                        let mut v129: bool = v127 == v128 ;
                        let mut v133: US3 = if v129 {
                            let mut v130: US4 = US4::US4_AgilePlanned;
                            US3::US3_Valid(v130.clone())
                        } else {
                            US3::US3_Invalid
                        };
                        let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
                        let mut v135: bool = v127 == v134 ;
                        let mut v138: US3 = if v135 {
                            let mut v136: US4 = US4::US4_AgileActive;
                            US3::US3_Valid(v136.clone())
                        } else {
                            v133.clone()
                        };
                        let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
                        let mut v140: bool = v127 == v139 ;
                        let mut v143: US3 = if v140 {
                            let mut v141: US4 = US4::US4_AgileBlocked;
                            US3::US3_Valid(v141.clone())
                        } else {
                            v138.clone()
                        };
                        let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) };
                        let mut v145: bool = v127 == v144 ;
                        let mut v148: US3 = if v145 {
                            let mut v146: US4 = US4::US4_AgilePaused;
                            US3::US3_Valid(v146.clone())
                        } else {
                            v143.clone()
                        };
                        let mut v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
                        let mut v150: bool = v127 == v149 ;
                        let mut v153: US3 = if v150 {
                            let mut v151: US4 = US4::US4_AgileDone;
                            US3::US3_Valid(v151.clone())
                        } else {
                            v148.clone()
                        };
                        match &v125 {
                            US2::US2_Invalid => {
                                let mut v165: i32 = agile_mutation_error("progress must be 0..100");
                                v165
                            }
                            US2::US2_Valid(v154) => {
                                let mut v154: i32 = *v154;
                                match &v153 {
                                    US3::US3_Invalid => {
                                        let mut v162: i32 = agile_mutation_error("invalid agile status (expected Planned, Active, Blocked, Paused or Done)");
                                        v162
                                    }
                                    US3::US3_Valid(v155) => {
                                        let mut v155: US4 = v155.clone();
                                        let mut v160: i32 = match &v155 {
                                            US4::US4_AgileActive => {
                                                1i32
                                            }
                                            US4::US4_AgileBlocked => {
                                                2i32
                                            }
                                            US4::US4_AgileDone => {
                                                4i32
                                            }
                                            US4::US4_AgilePaused => {
                                                3i32
                                            }
                                            US4::US4_AgilePlanned => {
                                                0i32
                                            }
                                        };
                                        let mut v161: i32 = mutation_agile_set_owned(v67.as_ref(), v69.as_ref(), v154, v160);
                                        v161
                                    }
                                }
                            }
                        }
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileShow => {
                    let mut v186: i32 = 3i32;
                    let mut v187: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v186 ;
                    let mut v190: bool = if v187 {
                        true
                    } else {
                        let mut v188: i32 = 4i32;
                        let mut v189: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v188 ;
                        v189
                    };
                    if v190 {
                        let mut v191: i32 = agile_task_inspection::eoie_agile_show_run();
                        v191
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileStatus => {
                    let mut v31: i32 = 2i32;
                    let mut v32: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v31 ;
                    if v32 {
                        let mut v33: i32 = 1i32;
                        let mut v34: Rc<str> = usize::try_from(v33).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let mut v35: i32 = mutation_agile_lease_status_owned(v34.as_ref());
                        v35
                    } else {
                        agile_usage_error_1()
                    }
                }
                US1::US1_AgileTodos => {
                    let mut v222: i32 = std::env::args().skip(2).count() as i32;
                    let mut v223: bool = 2i32 <= v222;
                    if v223 {
                        let mut v224: i32 = agile_todo_scan::eoie_agile_todos_run();
                        v224
                    } else {
                        agile_usage_error_1()
                    }
                }
            }
        }
        US0::US0_AgileUnknownName => {
            agile_usage_error_1()
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
