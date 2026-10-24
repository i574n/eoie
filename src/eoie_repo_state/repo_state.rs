#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_TrackedChange,
    US1_RenamedChange,
    US1_UntrackedEntry,
    US1_UnmergedEntry,
    US1_HeaderLine,
    US1_OtherLine,
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_2(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_3();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_4(v1);
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn git_output_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::process::Command::new("git").arg("-C").arg(&*v0).args(v1.split(char::from(10u8))).output().ok().filter(|output| output.status.success()).map(|output| String::from_utf8_lossy(&output.stdout).into_owned()).unwrap_or_default());
    v2.clone()
}
fn path_of_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1 "); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let mut v21: US1 = if v2 {
        US1::US1_TrackedChange
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2 "); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0.starts_with(&*v4);
        if v5 {
            US1::US1_RenamedChange
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("? "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v0.starts_with(&*v7);
            if v8 {
                US1::US1_UntrackedEntry
            } else {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u "); } LIT.with(|lit| lit.clone()) };
                let mut v11: bool = v0.starts_with(&*v10);
                if v11 {
                    US1::US1_UnmergedEntry
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# "); } LIT.with(|lit| lit.clone()) };
                    let mut v14: bool = v0.starts_with(&*v13);
                    if v14 {
                        US1::US1_HeaderLine
                    } else {
                        US1::US1_OtherLine
                    }
                }
            }
        }
    };
    match &v21 {
        US1::US1_HeaderLine => {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v30.clone()
        }
        US1::US1_OtherLine => {
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v31.clone()
        }
        US1::US1_RenamedChange => {
            let mut v24: u64 = 9u64;
            let mut v25: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v24 as usize) + 1, char::from(32u8)).nth(v24 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v25.clone()
        }
        US1::US1_TrackedChange => {
            let mut v22: u64 = 8u64;
            let mut v23: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v22 as usize) + 1, char::from(32u8)).nth(v22 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v23.clone()
        }
        US1::US1_UnmergedEntry => {
            let mut v28: u64 = 10u64;
            let mut v29: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v28 as usize) + 1, char::from(32u8)).nth(v28 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v29.clone()
        }
        US1::US1_UntrackedEntry => {
            let mut v26: u64 = 1u64;
            let mut v27: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v26 as usize) + 1, char::from(32u8)).nth(v26 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v27.clone()
        }
    }
}
fn counts_of_9(mut v0: Rc<str>) -> (u64, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1 "); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let mut v21: US1 = if v2 {
        US1::US1_TrackedChange
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2 "); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0.starts_with(&*v4);
        if v5 {
            US1::US1_RenamedChange
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("? "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v0.starts_with(&*v7);
            if v8 {
                US1::US1_UntrackedEntry
            } else {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u "); } LIT.with(|lit| lit.clone()) };
                let mut v11: bool = v0.starts_with(&*v10);
                if v11 {
                    US1::US1_UnmergedEntry
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# "); } LIT.with(|lit| lit.clone()) };
                    let mut v14: bool = v0.starts_with(&*v13);
                    if v14 {
                        US1::US1_HeaderLine
                    } else {
                        US1::US1_OtherLine
                    }
                }
            }
        }
    };
    let (mut v54, mut v55, mut v56, mut v57): (u64, u64, u64, u64) = match &v21 {
        US1::US1_TrackedChange => {
            let mut v22: u64 = 1u64;
            let mut v23: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(32u8)).nth(v22 as usize).unwrap_or(""));
            let mut v24: bool = v23.chars().next().map(|c| c == char::from(46u8)).unwrap_or(true) == false;
            let mut v25: u64 = if v24 {
                1u64
            } else {
                0u64
            };
            let mut v26: bool = v23.chars().nth(1).map(|c| c == char::from(46u8)).unwrap_or(true) == false;
            let mut v27: u64 = if v26 {
                1u64
            } else {
                0u64
            };
            (v25, v27, 0u64, 0u64)
        }
        _ => {
            let (mut v50, mut v51, mut v52, mut v53): (u64, u64, u64, u64) = match &v21 {
                US1::US1_RenamedChange => {
                    let mut v28: u64 = 1u64;
                    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(32u8)).nth(v28 as usize).unwrap_or(""));
                    let mut v30: bool = v29.chars().next().map(|c| c == char::from(46u8)).unwrap_or(true) == false;
                    let mut v31: u64 = if v30 {
                        1u64
                    } else {
                        0u64
                    };
                    let mut v32: bool = v29.chars().nth(1).map(|c| c == char::from(46u8)).unwrap_or(true) == false;
                    let mut v33: u64 = if v32 {
                        1u64
                    } else {
                        0u64
                    };
                    (v31, v33, 0u64, 0u64)
                }
                _ => {
                    let (mut v46, mut v47, mut v48, mut v49): (u64, u64, u64, u64) = match &v21 {
                        US1::US1_UntrackedEntry => {
                            (0u64, 0u64, 1u64, 0u64)
                        }
                        _ => {
                            let (mut v42, mut v43, mut v44, mut v45): (u64, u64, u64, u64) = match &v21 {
                                US1::US1_UnmergedEntry => {
                                    (0u64, 0u64, 0u64, 1u64)
                                }
                                _ => {
                                    let (mut v38, mut v39, mut v40, mut v41): (u64, u64, u64, u64) = match &v21 {
                                        US1::US1_HeaderLine => {
                                            (0u64, 0u64, 0u64, 0u64)
                                        }
                                        _ => {
                                            let (mut v34, mut v35, mut v36, mut v37): (u64, u64, u64, u64) = match &v21 {
                                                US1::US1_OtherLine => {
                                                    (0u64, 0u64, 0u64, 0u64)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v34, v35, v36, v37)
                                        }
                                    };
                                    (v38, v39, v40, v41)
                                }
                            };
                            (v42, v43, v44, v45)
                        }
                    };
                    (v46, v47, v48, v49)
                }
            };
            (v50, v51, v52, v53)
        }
    };
    (v54, v55, v56, v57)
}
fn tally_status_7(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::BTreeMap<String, [u64; 4]>>>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64) -> () {
    loop {
        let mut v5: bool = v2 < v3;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.lines().nth(v2 as usize).unwrap_or(""));
            let mut v7: Rc<str> = path_of_8(v6.clone());
            let mut v8: bool = v7.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v8 {
                let mut v9: Rc<str> = { let parts = v7.split(char::from(47u8)).collect::<Vec<_>>(); let keep = (v4 as usize).min(parts.len().saturating_sub(1)).max(1).min(parts.len()); std::rc::Rc::<str>::from(parts[..keep].join("/")) };
                let (mut v10, mut v11, mut v12, mut v13): (u64, u64, u64, u64) = counts_of_9(v6.clone());
                { let mut map = v0.borrow_mut(); let entry = map.entry(v9.to_string()).or_insert([0; 4]); entry[0] += v10; entry[1] += v11; entry[2] += v12; entry[3] += v13; };
                ()
            };
            let mut v14: u64 = v2.wrapping_add(1u64);
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v14, v3, v4);
            continue;
        }
        return ();
    }
}
fn repo_state_of_5(mut v0: Rc<str>, mut v1: u64) -> i32 {
    let mut v2: bool = std::path::Path::new(&*v0).join(".git").exists();
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status\n--porcelain=v2\n--branch\n--untracked-files=all"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = git_output_6(v0.clone(), v3.clone());
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("log\n-1\n--format=%h %cI %s"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = git_output_6(v0.clone(), v5.clone());
        let mut v7: std::rc::Rc<std::cell::RefCell<std::collections::BTreeMap<String, [u64; 4]>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));
        let mut v8: u64 = 0u64;
        let mut v9: u64 = v4.lines().count() as u64;
        tally_status_7(v7.clone(), v4.clone(), v8, v9, v1);
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# branch.head "); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from(v4.lines().find_map(|line| line.strip_prefix(&*v10)).unwrap_or("").trim());
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# branch.ab "); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from(v4.lines().find_map(|line| line.strip_prefix(&*v12)).unwrap_or("").trim());
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie repo state ok root="); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v0));
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" branch="); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
        let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v11));
        let mut v19: bool = v13.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = if v19 {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
            v20.clone()
        } else {
            v13.clone()
        };
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ab="); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v21));
        let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v23));
        let mut v25: u64 = 0u64;
        let mut v26: u64 = v7.borrow().values().map(|counts| counts[v25 as usize]).sum::<u64>();
        let mut v27: Rc<str> = std::rc::Rc::<str>::from((v26).to_string());
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" staged="); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v27));
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" modified="); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
        let mut v32: u64 = 1u64;
        let mut v33: u64 = v7.borrow().values().map(|counts| counts[v32 as usize]).sum::<u64>();
        let mut v34: Rc<str> = std::rc::Rc::<str>::from((v33).to_string());
        let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v34));
        let mut v36: u64 = 2u64;
        let mut v37: u64 = v7.borrow().values().map(|counts| counts[v36 as usize]).sum::<u64>();
        let mut v38: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" untracked="); } LIT.with(|lit| lit.clone()) };
        let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v38));
        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unmerged="); } LIT.with(|lit| lit.clone()) };
        let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
        let mut v43: u64 = 3u64;
        let mut v44: u64 = v7.borrow().values().map(|counts| counts[v43 as usize]).sum::<u64>();
        let mut v45: Rc<str> = std::rc::Rc::<str>::from((v44).to_string());
        let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v45));
        let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v46));
        let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v47));
        let mut v49: u64 = v7.borrow().len() as u64;
        let mut v50: Rc<str> = std::rc::Rc::<str>::from((v49).to_string());
        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" areas="); } LIT.with(|lit| lit.clone()) };
        let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v50));
        let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v52));
        let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" last="); } LIT.with(|lit| lit.clone()) };
        let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
        let mut v56: u64 = 0u64;
        let mut v57: Rc<str> = std::rc::Rc::<str>::from(v6.lines().nth(v56 as usize).unwrap_or(""));
        let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v57));
        println!("{}", &*v58);
        let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v60: u64 = 12u64;
        let mut v61: Rc<str> = { let map = v7.borrow(); let mut areas = map.iter().collect::<Vec<_>>(); areas.sort_by(|left, right| right.1.iter().sum::<u64>().cmp(&left.1.iter().sum::<u64>()).then_with(|| left.0.cmp(right.0))); let shown = areas.iter().take(v60 as usize).map(|(area, counts)| format!("{}area={} staged={} modified={} untracked={} unmerged={}", &*v59, area, counts[0], counts[1], counts[2], counts[3])).collect::<Vec<_>>(); let hidden = areas.len().saturating_sub(shown.len()); let mut text = shown.join("
"); if hidden > 0 { text.push_str(&format!("
{}... {hidden} more areas (eoie table query or git status for the rest)", &*v59)); } std::rc::Rc::<str>::from(text) };
        let mut v62: bool = v61.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v62 {
            println!("{}", &*v61);
            ()
        };
        0i32
    } else {
        let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie repo state rejected root="); } LIT.with(|lit| lit.clone()) };
        let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v0));
        let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason=not-a-git-repository"); } LIT.with(|lit| lit.clone()) };
        let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v65));
        println!("{}", &*v66);
        2i32
    }
}
fn states_from_1(mut v0: i32, mut v1: i32, mut v2: u64, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v0 < v1;
        if v4 {
            let mut v5: Rc<str> = argument_2(v0);
            let mut v6: i32 = repo_state_of_5(v5.clone(), v2);
            let mut v7: i32 = v0.wrapping_add(1i32);
            let mut v8: bool = v6 > v3;
            let mut v9: i32 = if v8 {
                v6
            } else {
                v3
            };
            (v0, v1, v2, v3) = (v7, v1, v2, v9);
            continue;
        } else {
            return v3;
        }
    }
}
fn argument_count_10() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn repo_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_2(v0);
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = if v2 {
        let mut v3: i32 = argument_count_10();
        let mut v4: bool = v3 > 3i32;
        v4
    } else {
        false
    };
    if v5 {
        let mut v6: i32 = 3i32;
        let mut v7: i32 = argument_count_10();
        let mut v8: u64 = 2u64;
        let mut v9: i32 = 0i32;
        states_from_1(v6, v7, v8, v9)
    } else {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: repo usage: repo state <root> [<root>...]"); } LIT.with(|lit| lit.clone()) };
        println!("{}", &*v11);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        repo_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_repo_run() -> i32 {
    closure0()()
}
