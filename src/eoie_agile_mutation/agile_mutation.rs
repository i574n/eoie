#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
    US1_AgileNoteEdit,
    US1_AgileNotes,
    US1_AgileRename,
    US1_AgileTodos,
    US1_AgileNext,
    US1_AgileRoots,
    US1_AgileLinks,
    US1_AgileViews,
    US1_AgileView,
    US1_AgileViewAdd,
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
fn state_lock_refused_1() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile state/agile.spi stayed locked by another writer for 30 s; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
    let mut v1: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v0 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v1
}
fn agile_subcommand_shape_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init|begin|lease|status|handoff|list|check|record|set|add|edit|show|plan|migrate-tags|note|note-edit|notes|rename|todos|next|roots|links|views|view|view-add"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>\n<root> <title>\n<root>\n<root>\n<root>\n<root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json]\n<root> [--compiler <absolute>]\n<root>\n<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>\n<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>]\n<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>\n<root> <id> [--json]\n<root> [--json]\npreview|apply <root>\n<root> <id> <text> [--as <author>]\n<root> <id> <note-number> <text> [--as <author>]\nfrom-title <root> <id>\n<root> <old-id> <new-id>\n<root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record\n<root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--open] [--ready] [--blocked] [count]\n<root> [add|drop <path>...]\n<root> --memory <dir> [--json]\n<root>\n<root> <name> [extra flags...]\n<root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = v1.split("|").zip(v2.split(char::from(10u8))).find_map(|(key,item)| if key == &*v0 { Some(std::rc::Rc::<str>::from(item)) } else { None }).unwrap_or_else(|| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    v3.clone()
}
fn agile_usage_error_2() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: Rc<str> = agile_subcommand_shape_3(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2 == v3 ;
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile supports init <root>, begin <root> <title>, lease <root>, status <root>, handoff <root>, list <root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json], check <root> [--compiler <absolute>], record <root>, set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>, add <root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>], edit <root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>, show <root> <id> [--json], plan <root> [--json], migrate-tags preview|apply <root>, note <root> <id> <text> [--as <author>], note-edit <root> <id> <note-number> <text> [--as <author>], notes from-title <root> <id>, rename <root> <old-id> <new-id>, todos <root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record, next <root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--open] [--ready] [--blocked] [count], roots <root> [add|drop <path>...], links <root> --memory <dir> [--json], views <root>, view <root> <name> [extra flags...], view-add <root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v5 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v6
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v1, &*v8, &*v2, &*v9].concat());
        let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(3).count().to_string());
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" operands; see eoie agile --help"); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v3, &*v3].concat());
        let mut v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v13 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v14
    }
}
fn agile_subcommand_of_4(mut v0: Rc<str>) -> US0 {
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
    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note-edit"); } LIT.with(|lit| lit.clone()) };
    let mut v78: bool = v0 == v77 ;
    let mut v81: US0 = if v78 {
        let mut v79: US1 = US1::US1_AgileNoteEdit;
        US0::US0_AgileKnown(v79.clone())
    } else {
        v76.clone()
    };
    let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
    let mut v83: bool = v0 == v82 ;
    let mut v86: US0 = if v83 {
        let mut v84: US1 = US1::US1_AgileNotes;
        US0::US0_AgileKnown(v84.clone())
    } else {
        v81.clone()
    };
    let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
    let mut v88: bool = v0 == v87 ;
    let mut v91: US0 = if v88 {
        let mut v89: US1 = US1::US1_AgileRename;
        US0::US0_AgileKnown(v89.clone())
    } else {
        v86.clone()
    };
    let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
    let mut v93: bool = v0 == v92 ;
    let mut v96: US0 = if v93 {
        let mut v94: US1 = US1::US1_AgileTodos;
        US0::US0_AgileKnown(v94.clone())
    } else {
        v91.clone()
    };
    let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
    let mut v98: bool = v0 == v97 ;
    let mut v101: US0 = if v98 {
        let mut v99: US1 = US1::US1_AgileNext;
        US0::US0_AgileKnown(v99.clone())
    } else {
        v96.clone()
    };
    let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("roots"); } LIT.with(|lit| lit.clone()) };
    let mut v103: bool = v0 == v102 ;
    let mut v106: US0 = if v103 {
        let mut v104: US1 = US1::US1_AgileRoots;
        US0::US0_AgileKnown(v104.clone())
    } else {
        v101.clone()
    };
    let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("links"); } LIT.with(|lit| lit.clone()) };
    let mut v108: bool = v0 == v107 ;
    let mut v111: US0 = if v108 {
        let mut v109: US1 = US1::US1_AgileLinks;
        US0::US0_AgileKnown(v109.clone())
    } else {
        v106.clone()
    };
    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("views"); } LIT.with(|lit| lit.clone()) };
    let mut v113: bool = v0 == v112 ;
    let mut v116: US0 = if v113 {
        let mut v114: US1 = US1::US1_AgileViews;
        US0::US0_AgileKnown(v114.clone())
    } else {
        v111.clone()
    };
    let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view"); } LIT.with(|lit| lit.clone()) };
    let mut v118: bool = v0 == v117 ;
    let mut v121: US0 = if v118 {
        let mut v119: US1 = US1::US1_AgileView;
        US0::US0_AgileKnown(v119.clone())
    } else {
        v116.clone()
    };
    let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view-add"); } LIT.with(|lit| lit.clone()) };
    let mut v123: bool = v0 == v122 ;
    if v123 {
        let mut v124: US1 = US1::US1_AgileViewAdd;
        US0::US0_AgileKnown(v124.clone())
    } else {
        v121.clone()
    }
}
fn state_lock_taken_6(mut v0: Rc<str>) -> bool {
    let mut v1: bool = { let state = std::path::Path::new(&*v0).join("state"); let lock = state.join("agile.spi.lock"); let started = std::time::Instant::now(); loop { if std::fs::create_dir(&lock).is_ok() { break true; } if !state.is_dir() { break true; } let stale = std::fs::metadata(&lock).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() >= 10).unwrap_or(false); if stale { let _ = std::fs::remove_dir(&lock); continue; } if started.elapsed() >= std::time::Duration::from_secs(30) { break false; } std::thread::sleep(std::time::Duration::from_millis(25)); } };
    v1
}
fn state_lock_held_5(mut v0: bool, mut v1: Rc<str>) -> bool {
    if v0 {
        state_lock_taken_6(v1.clone())
    } else {
        true
    }
}
fn planning_field_named_7(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v1.to_ascii_lowercase());
    let mut v3: bool = v0.clone() == v2.clone();
    if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v4.to_ascii_lowercase());
        let mut v6: bool = v0.clone() == v5.clone();
        if v6 {
            true
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
            let mut v9: bool = v0.clone() == v8.clone();
            v9
        }
    }
}
fn state_lock_released_9(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("state").join("agile.spi.lock")); };
    ()
}
fn state_lock_done_8(mut v0: bool, mut v1: Rc<str>) -> () {
    if v0 {
        state_lock_released_9(v1.clone())
    }
}
fn agile_entry_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: US0 = agile_subcommand_of_4(v1.clone());
    match &v2 {
        US0::US0_AgileKnown(v3) => {
            let mut v3: US1 = v3.clone();
            let mut v28: bool = match &v3 {
                US1::US1_AgileAdd => {
                    true
                }
                US1::US1_AgileBegin => {
                    true
                }
                US1::US1_AgileCheck => {
                    false
                }
                US1::US1_AgileEdit => {
                    true
                }
                US1::US1_AgileHandoff => {
                    false
                }
                US1::US1_AgileInit => {
                    false
                }
                US1::US1_AgileLease => {
                    false
                }
                US1::US1_AgileLinks => {
                    false
                }
                US1::US1_AgileList => {
                    false
                }
                US1::US1_AgileMigrateTags => {
                    true
                }
                US1::US1_AgileNext => {
                    false
                }
                US1::US1_AgileNote => {
                    true
                }
                US1::US1_AgileNoteEdit => {
                    true
                }
                US1::US1_AgileNotes => {
                    true
                }
                US1::US1_AgilePlan => {
                    false
                }
                US1::US1_AgileRecord => {
                    true
                }
                US1::US1_AgileRename => {
                    true
                }
                US1::US1_AgileRoots => {
                    true
                }
                US1::US1_AgileSet => {
                    true
                }
                US1::US1_AgileShow => {
                    false
                }
                US1::US1_AgileStatus => {
                    false
                }
                US1::US1_AgileTodos => {
                    true
                }
                US1::US1_AgileView => {
                    false
                }
                US1::US1_AgileViewAdd => {
                    true
                }
                US1::US1_AgileViews => {
                    false
                }
            };
            let mut v29: i32 = 1i32;
            let mut v30: Rc<str> = usize::try_from(v29).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v31: bool = state_lock_held_5(v28, v30.clone());
            if v31 {
                let mut v370: i32 = match &v3 {
                    US1::US1_AgileAdd => {
                        let mut v249: i32 = std::env::args().skip(2).count() as i32;
                        let mut v250: bool = 8i32 <= v249;
                        if v250 {
                            let mut v251: i32 = agile_task_creation::eoie_agile_add_run();
                            v251
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileBegin => {
                        let mut v39: i32 = 3i32;
                        let mut v40: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v39 ;
                        if v40 {
                            let mut v41: i32 = 1i32;
                            let mut v42: Rc<str> = usize::try_from(v41).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v43: i32 = 2i32;
                            let mut v44: Rc<str> = usize::try_from(v43).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v45: i32 = eoie_agile_lease::eoie_lease_title_words();
                            let mut v46: bool = 0 < v45;
                            if v46 {
                                let mut v47: i32 = eoie_agile_state::agile_begin_owned(v42.as_ref(), v44.as_ref());
                                v47
                            } else {
                                let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin title must contain 3..7 visible words"); } LIT.with(|lit| lit.clone()) };
                                let mut v49: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v48 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v49
                            }
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileCheck => {
                        let mut v79: i32 = 2i32;
                        let mut v80: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v79 ;
                        if v80 {
                            let mut v81: i32 = 1i32;
                            let mut v82: Rc<str> = usize::try_from(v81).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v83: i32 = eoie_agile_state::agile_check_owned(v82.as_ref());
                            v83
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileEdit => {
                        let mut v254: i32 = 5i32;
                        let mut v255: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v254 ;
                        if v255 {
                            let mut v256: i32 = 3i32;
                            let mut v257: Rc<str> = usize::try_from(v256).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v258: bool = planning_field_named_7(v257.clone());
                            if v258 {
                                let mut v259: i32 = agile_planning_revision::eoie_agile_planning_edit_run();
                                v259
                            } else {
                                let mut v260: i32 = agile_task_revision::eoie_agile_edit_run();
                                v260
                            }
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileHandoff => {
                        let mut v67: i32 = 2i32;
                        let mut v68: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v67 ;
                        if v68 {
                            let mut v69: i32 = 1i32;
                            let mut v70: Rc<str> = usize::try_from(v69).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v71: i32 = eoie_handoff_render::handoff_render_owned(v70.as_ref());
                            v71
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileInit => {
                        let mut v32: i32 = 2i32;
                        let mut v33: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v32 ;
                        if v33 {
                            let mut v34: i32 = 1i32;
                            let mut v35: Rc<str> = usize::try_from(v34).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v36: i32 = agile_backlog_scaffold::eoie_agile_init_run();
                            v36
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileLease => {
                        let mut v53: i32 = 2i32;
                        let mut v54: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v53 ;
                        if v54 {
                            let mut v55: i32 = 1i32;
                            let mut v56: Rc<str> = usize::try_from(v55).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v57: i32 = eoie_agile_state::agile_lease_status_owned(v56.as_ref());
                            v57
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileLinks => {
                        let mut v326: i32 = std::env::args().skip(2).count() as i32;
                        let mut v327: bool = 2i32 <= v326;
                        if v327 {
                            let mut v328: i32 = agile_memory_links::eoie_agile_links_run();
                            v328
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileList => {
                        let mut v74: i32 = std::env::args().skip(2).count() as i32;
                        let mut v75: bool = 2i32 <= v74;
                        if v75 {
                            let mut v76: i32 = agile_task_listing::eoie_agile_list_run();
                            v76
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileMigrateTags => {
                        let mut v277: i32 = std::env::args().skip(2).count() as i32;
                        let mut v278: bool = 2i32 <= v277;
                        if v278 {
                            let mut v279: i32 = agile_planning_migration::eoie_agile_migrate_tags_run();
                            v279
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileNext => {
                        let mut v316: i32 = std::env::args().skip(2).count() as i32;
                        let mut v317: bool = 2i32 <= v316;
                        if v317 {
                            let mut v318: i32 = agile_task_next::eoie_agile_next_run();
                            v318
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileNote => {
                        let mut v282: i32 = 4i32;
                        let mut v283: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v282 ;
                        let mut v286: bool = if v283 {
                            true
                        } else {
                            let mut v284: i32 = 6i32;
                            let mut v285: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v284 ;
                            v285
                        };
                        if v286 {
                            let mut v287: i32 = agile_task_notes::eoie_agile_note_run();
                            v287
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileNoteEdit => {
                        let mut v290: i32 = 5i32;
                        let mut v291: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v290 ;
                        let mut v294: bool = if v291 {
                            true
                        } else {
                            let mut v292: i32 = 7i32;
                            let mut v293: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v292 ;
                            v293
                        };
                        if v294 {
                            let mut v295: i32 = agile_note_edit::eoie_agile_note_edit_run();
                            v295
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileNotes => {
                        let mut v298: i32 = 4i32;
                        let mut v299: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v298 ;
                        if v299 {
                            let mut v300: i32 = agile_title_notes::eoie_agile_notes_from_title_run();
                            v300
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgilePlan => {
                        let mut v272: i32 = std::env::args().skip(2).count() as i32;
                        let mut v273: bool = 2i32 <= v272;
                        if v273 {
                            let mut v274: i32 = agile_planning_rollup::eoie_agile_planning_rollup_run();
                            v274
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileRecord => {
                        let mut v86: i32 = 2i32;
                        let mut v87: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v86 ;
                        if v87 {
                            let mut v88: i32 = 1i32;
                            let mut v89: Rc<str> = usize::try_from(v88).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v90: i32 = eoie_agile_state::agile_record_owned(v89.as_ref());
                            v90
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileRename => {
                        let mut v303: i32 = 4i32;
                        let mut v304: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v303 ;
                        if v304 {
                            let mut v305: i32 = agile_task_rename::eoie_agile_rename_run();
                            v305
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileRoots => {
                        let mut v321: i32 = std::env::args().skip(2).count() as i32;
                        let mut v322: bool = 2i32 <= v321;
                        if v322 {
                            let mut v323: i32 = agile_todo_roots::eoie_agile_roots_run();
                            v323
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileSet => {
                        let mut v93: i32 = 7i32;
                        let mut v94: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v93 ;
                        let mut v99: bool = if v94 {
                            let mut v95: i32 = 5i32;
                            let mut v96: Rc<str> = usize::try_from(v95).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
                            let mut v98: bool = v96 == v97 ;
                            v98
                        } else {
                            false
                        };
                        if v99 {
                            let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set records no author (only agile note does); drop --as "); } LIT.with(|lit| lit.clone()) };
                            let mut v101: i32 = 6i32;
                            let mut v102: Rc<str> = usize::try_from(v101).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", or add a note with --as to say who moved "); } LIT.with(|lit| lit.clone()) };
                            let mut v104: i32 = 2i32;
                            let mut v105: Rc<str> = usize::try_from(v104).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v107: Rc<str> = std::rc::Rc::<str>::from([&*v100, &*v102, &*v103, &*v105, &*v106].concat());
                            let mut v108: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v107 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v108
                        } else {
                            let mut v109: i32 = 5i32;
                            let mut v110: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v109 ;
                            if v110 {
                                let mut v111: i32 = 1i32;
                                let mut v112: Rc<str> = usize::try_from(v111).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v113: i32 = 2i32;
                                let mut v114: Rc<str> = usize::try_from(v113).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v115: i32 = 3i32;
                                let mut v116: i32 = usize::try_from(v115).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
                                let mut v117: i32 = 3i32;
                                let mut v118: i32 = 0i32;
                                let mut v119: i32 = usize::try_from(v117).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v118).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v120: bool = 48i32 <= v119;
                                let mut v122: bool = if v120 {
                                    let mut v121: bool = v119 <= 57i32;
                                    v121
                                } else {
                                    false
                                };
                                let mut v125: i32 = if v122 {
                                    let mut v123: i32 = v119 - 48;
                                    v123
                                } else {
                                    let mut v124: i32 = -1;
                                    v124
                                };
                                let mut v126: i32 = 3i32;
                                let mut v127: i32 = 1i32;
                                let mut v128: i32 = usize::try_from(v126).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v127).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v129: bool = 48i32 <= v128;
                                let mut v131: bool = if v129 {
                                    let mut v130: bool = v128 <= 57i32;
                                    v130
                                } else {
                                    false
                                };
                                let mut v134: i32 = if v131 {
                                    let mut v132: i32 = v128 - 48;
                                    v132
                                } else {
                                    let mut v133: i32 = -1;
                                    v133
                                };
                                let mut v135: i32 = 3i32;
                                let mut v136: i32 = 2i32;
                                let mut v137: i32 = usize::try_from(v135).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v136).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
                                let mut v138: bool = 48i32 <= v137;
                                let mut v140: bool = if v138 {
                                    let mut v139: bool = v137 <= 57i32;
                                    v139
                                } else {
                                    false
                                };
                                let mut v143: i32 = if v140 {
                                    let mut v141: i32 = v137 - 48;
                                    v141
                                } else {
                                    let mut v142: i32 = -1;
                                    v142
                                };
                                let mut v144: bool = v116 == 1i32;
                                let mut v145: bool = 0 <= v125;
                                let mut v146: bool = v144 && v145;
                                let mut v170: US2 = if v146 {
                                    US2::US2_Valid(v125)
                                } else {
                                    let mut v148: bool = v116 == 2i32;
                                    let mut v149: bool = 0 <= v125;
                                    let mut v150: bool = 0 <= v134;
                                    let mut v152: bool = if v148 {
                                        let mut v151: bool = v149 && v150;
                                        v151
                                    } else {
                                        false
                                    };
                                    if v152 {
                                        let mut v153: i32 = v125 * 10 + v134;
                                        US2::US2_Valid(v153)
                                    } else {
                                        let mut v155: bool = v116 == 3i32;
                                        let mut v156: bool = 0 <= v125;
                                        let mut v157: bool = 0 <= v134;
                                        let mut v158: bool = 0 <= v143;
                                        let mut v161: bool = if v155 {
                                            if v156 {
                                                let mut v159: bool = v157 && v158;
                                                v159
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        };
                                        if v161 {
                                            let mut v162: i32 = v125 * 100 + v134 * 10 + v143;
                                            let mut v163: bool = v162 <= 100i32;
                                            if v163 {
                                                US2::US2_Valid(v162)
                                            } else {
                                                US2::US2_Invalid
                                            }
                                        } else {
                                            US2::US2_Invalid
                                        }
                                    }
                                };
                                let mut v171: i32 = 4i32;
                                let mut v172: Rc<str> = usize::try_from(v171).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let mut v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned"); } LIT.with(|lit| lit.clone()) };
                                let mut v174: bool = v172 == v173 ;
                                let mut v178: US3 = if v174 {
                                    let mut v175: US4 = US4::US4_AgilePlanned;
                                    US3::US3_Valid(v175.clone())
                                } else {
                                    US3::US3_Invalid
                                };
                                let mut v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Active"); } LIT.with(|lit| lit.clone()) };
                                let mut v180: bool = v172 == v179 ;
                                let mut v183: US3 = if v180 {
                                    let mut v181: US4 = US4::US4_AgileActive;
                                    US3::US3_Valid(v181.clone())
                                } else {
                                    v178.clone()
                                };
                                let mut v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Blocked"); } LIT.with(|lit| lit.clone()) };
                                let mut v185: bool = v172 == v184 ;
                                let mut v188: US3 = if v185 {
                                    let mut v186: US4 = US4::US4_AgileBlocked;
                                    US3::US3_Valid(v186.clone())
                                } else {
                                    v183.clone()
                                };
                                let mut v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused"); } LIT.with(|lit| lit.clone()) };
                                let mut v190: bool = v172 == v189 ;
                                let mut v193: US3 = if v190 {
                                    let mut v191: US4 = US4::US4_AgilePaused;
                                    US3::US3_Valid(v191.clone())
                                } else {
                                    v188.clone()
                                };
                                let mut v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Done"); } LIT.with(|lit| lit.clone()) };
                                let mut v195: bool = v172 == v194 ;
                                let mut v198: US3 = if v195 {
                                    let mut v196: US4 = US4::US4_AgileDone;
                                    US3::US3_Valid(v196.clone())
                                } else {
                                    v193.clone()
                                };
                                match &v170 {
                                    US2::US2_Invalid => {
                                        let mut v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
                                        let mut v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a whole number 0..100"); } LIT.with(|lit| lit.clone()) };
                                        let mut v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set: "); } LIT.with(|lit| lit.clone()) };
                                        let mut v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" must be "); } LIT.with(|lit| lit.clone()) };
                                        let mut v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                                        let mut v236: i32 = 3i32;
                                        let mut v237: Rc<str> = usize::try_from(v236).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let mut v238: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                        let mut v239: Rc<str> = std::rc::Rc::<str>::from([&*v234, &*v232, &*v235, &*v237, &*v238].concat());
                                        let mut v240: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; usage: agile set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
                                        let mut v241: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v242: Rc<str> = std::rc::Rc::<str>::from([&*v233, &*v231, &*v239, &*v240, &*v241].concat());
                                        let mut v243: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v242 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v243
                                    }
                                    US2::US2_Valid(v199) => {
                                        let mut v199: i32 = *v199;
                                        match &v198 {
                                            US3::US3_Invalid => {
                                                let mut v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                                                let mut v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned, Active, Blocked, Paused or Done"); } LIT.with(|lit| lit.clone()) };
                                                let mut v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set: "); } LIT.with(|lit| lit.clone()) };
                                                let mut v219: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" must be "); } LIT.with(|lit| lit.clone()) };
                                                let mut v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                                                let mut v221: i32 = 4i32;
                                                let mut v222: Rc<str> = usize::try_from(v221).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let mut v223: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                                let mut v224: Rc<str> = std::rc::Rc::<str>::from([&*v219, &*v217, &*v220, &*v222, &*v223].concat());
                                                let mut v225: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; usage: agile set <root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
                                                let mut v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                let mut v227: Rc<str> = std::rc::Rc::<str>::from([&*v218, &*v216, &*v224, &*v225, &*v226].concat());
                                                let mut v228: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v227 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                v228
                                            }
                                            US3::US3_Valid(v200) => {
                                                let mut v200: US4 = v200.clone();
                                                match &v200 {
                                                    US4::US4_AgileDone => {
                                                        let mut v201: Rc<str> = std::rc::Rc::<str>::from(agile_todo_scan::eoie_agile_todo_sites(&*v112, &*v114).trim_end());
                                                        let mut v202: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        let mut v203: bool = v201 == v202 ;
                                                        if v203 {
                                                            let mut v204: i32 = eoie_agile_state::agile_set_validated_owned(v112.as_ref(), v114.as_ref(), v199, 4i32);
                                                            v204
                                                        } else {
                                                            let mut v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile set "); } LIT.with(|lit| lit.clone()) };
                                                            let mut v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: TODO blocks still name it; cut them or keep the item open:\n"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v207: Rc<str> = std::rc::Rc::<str>::from([&*v205, &*v114, &*v206, &*v201, &*v202].concat());
                                                            let mut v208: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v207 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                            v208
                                                        }
                                                    }
                                                    _ => {
                                                        let mut v213: i32 = match &v200 {
                                                            US4::US4_AgileActive => {
                                                                1i32
                                                            }
                                                            US4::US4_AgileBlocked => {
                                                                2i32
                                                            }
                                                            US4::US4_AgilePaused => {
                                                                3i32
                                                            }
                                                            US4::US4_AgilePlanned => {
                                                                0i32
                                                            }
                                                            _ => unreachable!(),
                                                        };
                                                        let mut v214: i32 = eoie_agile_state::agile_set_validated_owned(v112.as_ref(), v114.as_ref(), v199, v213);
                                                        v214
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                agile_usage_error_2()
                            }
                        }
                    }
                    US1::US1_AgileShow => {
                        let mut v264: i32 = 3i32;
                        let mut v265: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v264 ;
                        let mut v268: bool = if v265 {
                            true
                        } else {
                            let mut v266: i32 = 4i32;
                            let mut v267: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v266 ;
                            v267
                        };
                        if v268 {
                            let mut v269: i32 = agile_task_inspection::eoie_agile_show_run();
                            v269
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileStatus => {
                        let mut v60: i32 = 2i32;
                        let mut v61: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v60 ;
                        if v61 {
                            let mut v62: i32 = 1i32;
                            let mut v63: Rc<str> = usize::try_from(v62).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let mut v64: i32 = eoie_agile_state::agile_lease_status_owned(v63.as_ref());
                            v64
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileTodos => {
                        let mut v308: i32 = std::env::args().skip(2).count() as i32;
                        let mut v309: bool = 2i32 <= v308;
                        if v309 {
                            let mut v310: bool = std::env::args().skip(4).any(|argument| argument == "--record");
                            if v310 {
                                let mut v311: i32 = agile_todo_record::eoie_agile_todos_record_run();
                                v311
                            } else {
                                let mut v312: i32 = agile_todo_scan::eoie_agile_todos_run();
                                v312
                            }
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileView => {
                        let mut v336: i32 = std::env::args().skip(2).count() as i32;
                        let mut v337: bool = 2i32 <= v336;
                        if v337 {
                            let mut v338: i32 = agile_task_views::eoie_agile_views_run();
                            v338
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileViewAdd => {
                        let mut v341: i32 = std::env::args().skip(2).count() as i32;
                        let mut v342: bool = 2i32 <= v341;
                        if v342 {
                            let mut v343: i32 = agile_task_views::eoie_agile_views_run();
                            v343
                        } else {
                            agile_usage_error_2()
                        }
                    }
                    US1::US1_AgileViews => {
                        let mut v331: i32 = std::env::args().skip(2).count() as i32;
                        let mut v332: bool = 2i32 <= v331;
                        if v332 {
                            let mut v333: i32 = agile_task_views::eoie_agile_views_run();
                            v333
                        } else {
                            agile_usage_error_2()
                        }
                    }
                };
                state_lock_done_8(v28, v30.clone());
                v370
            } else {
                state_lock_refused_1()
            }
        }
        US0::US0_AgileUnknownName => {
            agile_usage_error_2()
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
