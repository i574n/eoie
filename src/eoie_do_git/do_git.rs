#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_GitInit(Rc<str>),
    US1_GitCommit(Rc<str>, Rc<str>, Rc<str>),
    US1_GitHistory(Rc<str>, Rc<str>, Rc<str>),
    US1_GitStatus(Rc<str>),
    US1_GitDays(Rc<str>, Rc<str>),
    US1_Blank,
}
#[derive(Clone)]
enum US0 {
    US0_GitParsed(US1),
    US0_GitRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Succeeded,
    US2_Failed,
}
fn now_ms_1() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_index_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn words_span_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = { let fields = v0.get(v1 as usize).cloned().unwrap_or_default(); let until = if v3 < 0 { fields.len() } else { (v3 as usize).min(fields.len()) }; std::rc::Rc::<str>::from(fields.iter().take(until).skip(v2 as usize).filter(|raw| raw.len() > 1).flat_map(|raw| raw[1..].split(|c: char| c == char::from(44u8) || c.is_whitespace()).filter(|item| !item.is_empty()).map(str::to_owned).collect::<Vec<_>>()).collect::<Vec<_>>().join("
")) };
    v4.clone()
}
fn body_at_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3.clone()
}
fn status_parsed_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = field_index_6(v0.clone(), v1, v2.clone());
    let mut v4: i32 = 3i32;
    let mut v5: Rc<str> = words_span_7(v0.clone(), v1, v4, v3);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git status: expected git status <repo>[, <repo>...] group by state"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v7.clone())
    } else {
        let mut v9: bool = v3 < 0i32;
        let mut v13: bool = if v9 {
            true
        } else {
            let mut v10: i32 = v3.wrapping_add(2i32);
            let mut v11: Rc<str> = body_at_8(v0.clone(), v1, v10);
            let mut v12: bool = v11.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
            v12
        };
        if v13 {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git status: expected group by state"); } LIT.with(|lit| lit.clone()) };
            US0::US0_GitRejected(v14.clone())
        } else {
            let mut v16: US1 = US1::US1_GitStatus(v5.clone());
            US0::US0_GitParsed(v16.clone())
        }
    }
}
fn quoted_at_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> bool {
    let mut v3: bool = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).map(|field| field.starts_with(char::from(113u8))).unwrap_or(false);
    v3
}
fn commit_parsed_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("at"); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = field_index_6(v0.clone(), v1, v3.clone());
    let mut v5: i32 = 4i32;
    let mut v6: Rc<str> = body_at_8(v0.clone(), v1, v5);
    let mut v7: bool = v6.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git commit: expected git commit <repo> all \"message\" [at <iso time>]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v8.clone())
    } else {
        let mut v10: i32 = 5i32;
        let mut v11: bool = quoted_at_10(v0.clone(), v1, v10);
        let mut v12: bool = v11 == false;
        if v12 {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git commit: expected the quoted message after all"); } LIT.with(|lit| lit.clone()) };
            US0::US0_GitRejected(v13.clone())
        } else {
            let mut v15: i32 = 5i32;
            let mut v16: Rc<str> = body_at_8(v0.clone(), v1, v15);
            let mut v17: bool = v4 < 0i32;
            let mut v21: Rc<str> = if v17 {
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v18.clone()
            } else {
                let mut v19: i32 = v4.wrapping_add(1i32);
                body_at_8(v0.clone(), v1, v19)
            };
            let mut v22: US1 = US1::US1_GitCommit(v2.clone(), v16.clone(), v21.clone());
            US0::US0_GitParsed(v22.clone())
        }
    }
}
fn history_parsed_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in"); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = field_index_6(v0.clone(), v1, v3.clone());
    let mut v5: i32 = 4i32;
    let mut v6: bool = quoted_at_10(v0.clone(), v1, v5);
    let mut v7: bool = v6 == false;
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git history: expected git history <repo> \"text\" in <file>"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v8.clone())
    } else {
        let mut v10: bool = v4 < 0i32;
        if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git history: expected in <file>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_GitRejected(v11.clone())
        } else {
            let mut v13: i32 = 4i32;
            let mut v14: Rc<str> = body_at_8(v0.clone(), v1, v13);
            let mut v15: i32 = v4.wrapping_add(1i32);
            let mut v16: Rc<str> = body_at_8(v0.clone(), v1, v15);
            let mut v17: US1 = US1::US1_GitHistory(v2.clone(), v14.clone(), v16.clone());
            US0::US0_GitParsed(v17.clone())
        }
    }
}
fn days_parsed_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("since"); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = field_index_6(v0.clone(), v1, v3.clone());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by"); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = field_index_6(v0.clone(), v1, v5.clone());
    let mut v7: bool = v6 < 0i32;
    let mut v11: bool = if v7 {
        true
    } else {
        let mut v8: i32 = v6.wrapping_add(1i32);
        let mut v9: Rc<str> = body_at_8(v0.clone(), v1, v8);
        let mut v10: bool = v9.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("day"); } LIT.with(|lit| lit.clone()) };
        v10
    };
    if v11 {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git log: expected git log <repo> [since <date>] count by day"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v12.clone())
    } else {
        let mut v14: bool = v4 < 0i32;
        let mut v18: Rc<str> = if v14 {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v15.clone()
        } else {
            let mut v16: i32 = v4.wrapping_add(1i32);
            body_at_8(v0.clone(), v1, v16)
        };
        let mut v19: US1 = US1::US1_GitDays(v2.clone(), v18.clone());
        US0::US0_GitParsed(v19.clone())
    }
}
fn field_at_13(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn git_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 0i32;
    let mut v3: Rc<str> = field_at_13(v0.clone(), v1, v2);
    let mut v4: i32 = 2i32;
    let mut v5: Rc<str> = body_at_8(v0.clone(), v1, v4);
    let mut v6: i32 = 3i32;
    let mut v7: Rc<str> = body_at_8(v0.clone(), v1, v6);
    let mut v8: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v8 {
        let mut v9: US1 = US1::US1_Blank;
        US0::US0_GitParsed(v9.clone())
    } else {
        let mut v11: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
        let mut v15: bool = if v11 {
            true
        } else {
            let mut v12: i32 = 1i32;
            let mut v13: Rc<str> = field_at_13(v0.clone(), v1, v12);
            let mut v14: bool = v13.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
            v14
        };
        if v15 {
            let mut v16: i32 = 1i32;
            let mut v17: Rc<str> = field_at_13(v0.clone(), v1, v16);
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with git yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v19, &*v19, &*v19].concat());
            US0::US0_GitRejected(v20.clone())
        } else {
            let mut v22: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
            if v22 {
                status_parsed_5(v0.clone(), v1)
            } else {
                let mut v24: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v24 {
                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git "); } LIT.with(|lit| lit.clone()) };
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": expected a repo folder"); } LIT.with(|lit| lit.clone()) };
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v5, &*v26, &*v27, &*v27].concat());
                    US0::US0_GitRejected(v28.clone())
                } else {
                    let mut v30: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init"); } LIT.with(|lit| lit.clone()) };
                    if v30 {
                        let mut v31: US1 = US1::US1_GitInit(v7.clone());
                        US0::US0_GitParsed(v31.clone())
                    } else {
                        let mut v33: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commit"); } LIT.with(|lit| lit.clone()) };
                        if v33 {
                            commit_parsed_9(v0.clone(), v1, v7.clone())
                        } else {
                            let mut v35: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("history"); } LIT.with(|lit| lit.clone()) };
                            if v35 {
                                history_parsed_11(v0.clone(), v1, v7.clone())
                            } else {
                                let mut v37: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("log"); } LIT.with(|lit| lit.clone()) };
                                if v37 {
                                    days_parsed_12(v0.clone(), v1, v7.clone())
                                } else {
                                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git: unknown action "); } LIT.with(|lit| lit.clone()) };
                                    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; expected init, commit, history, status or log"); } LIT.with(|lit| lit.clone()) };
                                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v5, &*v40, &*v41, &*v41].concat());
                                    US0::US0_GitRejected(v42.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn first_rejection_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: US0 = git_parsed_4(v0.clone(), v1);
            match &v4 {
                US0::US0_GitParsed(v6) => {
                    let mut v6: US1 = v6.clone();
                    let mut v7: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v7);
                    continue;
                }
                US0::US0_GitRejected(v5) => {
                    let mut v5: Rc<str> = v5.clone();
                    return v5.clone();
                }
            }
        } else {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v11.clone();
        }
    }
}
fn empty_git_tally_14() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn git_run_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = { let mut command = std::process::Command::new("git"); command.current_dir(std::path::Path::new(&*v0).join(&*v1)).args(["-c", "user.name=eoie", "-c"]).arg(format!("user.email=eoie{}localhost", char::from(64u8))).args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"]).args((v2).split(char::from(10u8))).stdin(std::process::Stdio::null()); if !(v3).is_empty() { command.env("GIT_AUTHOR_DATE", &*v3).env("GIT_COMMITTER_DATE", &*v3); } let mark = char::from(30u8); std::rc::Rc::<str>::from(match command.output() { Ok(output) => format!("{}{mark}{}{mark}{}", output.status.code().unwrap_or(1), String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr).trim()), Err(error) => format!("1{mark}{mark}git: {error}") }) };
    v4.clone()
}
fn part_at_20(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn succeeded_19(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<str> = part_at_20(v0.clone(), v1);
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
    v3
}
fn lines_joined_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v0.clone()
    } else {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            v1.clone()
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v1, &*v5, &*v5].concat());
            v6.clone()
        }
    }
}
fn reported_21(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: bool) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v15: Rc<str> = lines_joined_22(v0.clone(), v13.clone());
    let mut v16: bool = v1 || v14;
    (v15.clone(), v16, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
}
fn failure_row_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed: "); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = 2i32;
    let mut v4: Rc<str> = part_at_20(v1.clone(), v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v4, &*v5, &*v5].concat());
    v6.clone()
}
fn init_run_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init\n-q\n-b\nmain\n"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v1, &*v15, &*v15, &*v15].concat());
    let mut v18: Rc<str> = git_run_18(v0.clone(), v15.clone(), v17.clone(), v15.clone());
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git init "); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v1, &*v15, &*v15, &*v15].concat());
    let mut v21: bool = succeeded_19(v18.clone());
    let (mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v21 {
        let mut v22: u64 = v4.wrapping_add(1u64);
        let mut v23: bool = false;
        let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v2.clone(), v3, v22, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v20.clone(), v23);
        (v24.clone(), v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36)
    } else {
        let mut v37: Rc<str> = failure_row_23(v20.clone(), v18.clone());
        let mut v38: bool = true;
        let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v37.clone(), v38);
        (v39.clone(), v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51)
    };
    (v52.clone(), v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64)
}
fn commit_run_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git commit "); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v1, &*v18, &*v2, &*v19].concat());
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add\n-A"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = git_run_18(v0.clone(), v1.clone(), v21.clone(), v19.clone());
    let mut v23: bool = succeeded_19(v22.clone());
    let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v23 {
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commit\n-q\n-m\n"); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v2, &*v19, &*v19, &*v19].concat());
        let mut v26: Rc<str> = git_run_18(v0.clone(), v1.clone(), v25.clone(), v3.clone());
        let mut v27: bool = succeeded_19(v26.clone());
        let (mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v27 {
            let mut v28: u64 = v7.wrapping_add(1u64);
            let mut v29: bool = false;
            let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v28, v8, v9, v10, v11, v12, v13, v14, v15, v16, v20.clone(), v29);
            (v30.clone(), v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42)
        } else {
            let mut v43: Rc<str> = failure_row_23(v20.clone(), v26.clone());
            let mut v44: bool = true;
            let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v43.clone(), v44);
            (v45.clone(), v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57)
        };
        (v58.clone(), v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70)
    } else {
        let mut v71: Rc<str> = failure_row_23(v20.clone(), v22.clone());
        let mut v72: bool = true;
        let (mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v71.clone(), v72);
        (v73.clone(), v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85)
    };
    (v86.clone(), v87, v88, v89, v90, v91, v92, v93, v94, v95, v96, v97, v98)
}
fn row_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v1, &*v3, &*v3].concat());
    v4.clone()
}
fn line_count_28(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).lines().filter(|line| !line.is_empty()).count() as u64;
    v1
}
fn line_at_29(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().filter(|line| !line.is_empty()).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn tab_part_30(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn occurrences_31(mut v0: Rc<str>, mut v1: Rc<str>) -> u64 {
    let mut v2: u64 = (v0).matches(&*v1).count() as u64;
    v2
}
fn history_rows_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>) -> Rc<str> {
    loop {
        let mut v7: u64 = line_count_28(v4.clone());
        let mut v8: bool = v5 < v7;
        if v8 {
            let mut v9: Rc<str> = line_at_29(v4.clone(), v5);
            let mut v10: i32 = 0i32;
            let mut v11: Rc<str> = tab_part_30(v9.clone(), v10);
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show\n"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v11, &*v13, &*v3, &*v14].concat());
            let mut v16: Rc<str> = git_run_18(v0.clone(), v1.clone(), v15.clone(), v14.clone());
            let mut v17: i32 = 1i32;
            let mut v18: Rc<str> = part_at_20(v16.clone(), v17);
            let mut v19: u64 = occurrences_31(v18.clone(), v2.clone());
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^:"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v11, &*v20, &*v3, &*v14].concat());
            let mut v22: Rc<str> = git_run_18(v0.clone(), v1.clone(), v21.clone(), v14.clone());
            let mut v23: i32 = 1i32;
            let mut v24: Rc<str> = part_at_20(v22.clone(), v23);
            let mut v25: u64 = occurrences_31(v24.clone(), v2.clone());
            let mut v26: bool = v19 < v25;
            let mut v29: Rc<str> = if v26 {
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("removed"); } LIT.with(|lit| lit.clone()) };
                v27.clone()
            } else {
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("added"); } LIT.with(|lit| lit.clone()) };
                v28.clone()
            };
            let mut v30: i32 = 1i32;
            let mut v31: Rc<str> = tab_part_30(v9.clone(), v30);
            let mut v32: i32 = 2i32;
            let mut v33: Rc<str> = tab_part_30(v9.clone(), v32);
            let mut v34: Rc<str> = row_26(v29.clone(), v33.clone());
            let mut v35: Rc<str> = row_26(v31.clone(), v34.clone());
            let mut v36: Rc<str> = row_26(v11.clone(), v35.clone());
            let mut v37: u64 = v5.wrapping_add(1u64);
            let mut v38: Rc<str> = lines_joined_22(v6.clone(), v36.clone());
            (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v37, v38.clone());
            continue;
        } else {
            return v6.clone();
        }
    }
}
fn matching_lines_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(std::path::Path::new(&*v0).join(&*v1)).unwrap_or_default().lines().enumerate().filter(|(_, line)| line.contains(&*v2)).map(|(index, line)| format!("{}:{}  {}", &*v1, index + 1, line)).collect::<Vec<_>>().join("
"));
    v3.clone()
}
fn aligned_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let separator = char::from(31u8); let rows = (&*v0).lines().collect::<Vec<_>>(); let mut widths: Vec<usize> = Vec::new(); for row in &rows { if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); for (index, cell) in cells[..cells.len() - 1].iter().enumerate() { let length = cell.chars().count(); if widths.len() <= index { widths.push(length); } else if widths[index] < length { widths[index] = length; } } } } let shown = rows.iter().map(|row| if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); let last = cells.len() - 1; cells.iter().enumerate().map(|(index, cell)| if index == last { cell.to_string() } else { format!("{:<width$}  ", cell, width = widths[index]) }).collect::<String>() } else { row.to_string() }).collect::<Vec<_>>(); std::rc::Rc::<str>::from(shown.join("
")) };
    v1.clone()
}
fn history_run_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("log\n--abbrev=8\n--format=%h%x09%ad%x09%s\n--date=short\n-S"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n--\n"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v2, &*v18, &*v3, &*v19].concat());
    let mut v21: Rc<str> = git_run_18(v0.clone(), v1.clone(), v20.clone(), v19.clone());
    let mut v22: bool = succeeded_19(v21.clone());
    let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v22 {
        let mut v23: i32 = 1i32;
        let mut v24: Rc<str> = part_at_20(v21.clone(), v23);
        let mut v25: u64 = 0u64;
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commit"); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("date"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("change"); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("subject"); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = row_26(v28.clone(), v29.clone());
        let mut v31: Rc<str> = row_26(v27.clone(), v30.clone());
        let mut v32: Rc<str> = row_26(v26.clone(), v31.clone());
        let mut v33: Rc<str> = history_rows_27(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v24.clone(), v25, v32.clone());
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v34, &*v3, &*v19, &*v19].concat());
        let mut v36: Rc<str> = matching_lines_32(v0.clone(), v35.clone(), v2.clone());
        let mut v37: u64 = line_count_28(v24.clone());
        let mut v38: u64 = v8.wrapping_add(v37);
        let mut v39: u64 = line_count_28(v36.clone());
        let mut v40: u64 = v9.wrapping_add(v39);
        let mut v41: u64 = v10.wrapping_add(1u64);
        let mut v42: Rc<str> = aligned_33(v33.clone());
        let mut v43: Rc<str> = lines_joined_22(v42.clone(), v36.clone());
        let mut v44: bool = false;
        let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v7, v38, v40, v41, v11, v12, v13, v14, v15, v16, v43.clone(), v44);
        (v45.clone(), v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57)
    } else {
        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git history "); } LIT.with(|lit| lit.clone()) };
        let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v1, &*v19, &*v19, &*v19].concat());
        let mut v60: Rc<str> = failure_row_23(v59.clone(), v21.clone());
        let mut v61: bool = true;
        let (mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v60.clone(), v61);
        (v62.clone(), v63, v64, v65, v66, v67, v68, v69, v70, v71, v72, v73, v74)
    };
    (v75.clone(), v76, v77, v78, v79, v80, v81, v82, v83, v84, v85, v86, v87)
}
fn porcelain_paths_35(mut v0: Rc<str>, mut v1: bool) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().filter(|line| line.len() > 3 && line.starts_with("??") == v1).map(|line| line[3..].trim_matches(char::from(34u8)).to_owned()).collect::<Vec<_>>().join("
"));
    v2.clone()
}
fn decimal_text_37(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn state_row_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        let mut v5: u64 = line_count_28(v2.clone());
        let mut v6: Rc<str> = decimal_text_37(v5);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v7, &v8));
        let mut v10: Rc<str> = row_26(v6.clone(), v9.clone());
        let mut v11: Rc<str> = row_26(v1.clone(), v10.clone());
        row_26(v0.clone(), v11.clone())
    }
}
fn repos_status_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v17: u64 = line_count_28(v1.clone());
    let mut v18: bool = v2 < v17;
    let (mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v18 {
        let mut v19: Rc<str> = line_at_29(v1.clone(), v2);
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status\n--porcelain\n--untracked-files=all"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = git_run_18(v0.clone(), v19.clone(), v20.clone(), v21.clone());
        let mut v23: bool = succeeded_19(v22.clone());
        let (mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v23 {
            let mut v24: i32 = 1i32;
            let mut v25: Rc<str> = part_at_20(v22.clone(), v24);
            let mut v26: bool = false;
            let mut v27: Rc<str> = porcelain_paths_35(v25.clone(), v26);
            let mut v28: bool = true;
            let mut v29: Rc<str> = porcelain_paths_35(v25.clone(), v28);
            let mut v30: bool = v25.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v40: Rc<str> = if v30 {
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("clean"); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = row_26(v31.clone(), v32.clone());
                row_26(v19.clone(), v33.clone())
            } else {
                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modified"); } LIT.with(|lit| lit.clone()) };
                let mut v36: Rc<str> = state_row_36(v19.clone(), v35.clone(), v27.clone());
                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("untracked"); } LIT.with(|lit| lit.clone()) };
                let mut v38: Rc<str> = state_row_36(v19.clone(), v37.clone(), v29.clone());
                lines_joined_22(v36.clone(), v38.clone())
            };
            let mut v41: u64 = v11.wrapping_add(1u64);
            let mut v42: u64 = line_count_28(v27.clone());
            let mut v43: u64 = v12.wrapping_add(v42);
            let mut v44: u64 = line_count_28(v29.clone());
            let mut v45: u64 = v13.wrapping_add(v44);
            let mut v46: u64 = v2.wrapping_add(1u64);
            let mut v47: Rc<str> = lines_joined_22(v3.clone(), v40.clone());
            let (mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = repos_status_34(v0.clone(), v1.clone(), v46, v47.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v41, v43, v45, v14, v15, v16);
            (v48.clone(), v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59, v60)
        } else {
            let mut v61: u64 = v2.wrapping_add(1u64);
            let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v64: i32 = 2i32;
            let mut v65: Rc<str> = part_at_20(v22.clone(), v64);
            let mut v66: Rc<str> = row_26(v63.clone(), v65.clone());
            let mut v67: Rc<str> = row_26(v62.clone(), v66.clone());
            let mut v68: Rc<str> = row_26(v19.clone(), v67.clone());
            let mut v69: Rc<str> = lines_joined_22(v3.clone(), v68.clone());
            let mut v70: bool = true;
            let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = repos_status_34(v0.clone(), v1.clone(), v61, v69.clone(), v4.clone(), v70, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16);
            (v71.clone(), v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83)
        };
        (v84.clone(), v85, v86, v87, v88, v89, v90, v91, v92, v93, v94, v95, v96)
    } else {
        let mut v97: u64 = v14.wrapping_add(1u64);
        let mut v98: Rc<str> = aligned_33(v3.clone());
        let mut v99: bool = false;
        let (mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v97, v15, v16, v98.clone(), v99);
        (v100.clone(), v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, v111, v112)
    };
    (v113.clone(), v114, v115, v116, v117, v118, v119, v120, v121, v122, v123, v124, v125)
}
fn days_counted_39(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut days = std::collections::BTreeMap::<&str, u64>::new(); for day in (v0).lines().map(str::trim).filter(|day| !day.is_empty()) { *days.entry(day).or_default() += 1; } std::rc::Rc::<str>::from(days.iter().rev().map(|(day, count)| format!("{day}{}{count}", char::from(31u8))).collect::<Vec<_>>().join("
")) };
    v1.clone()
}
fn days_run_38(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v16: bool = (v2).len() == 10;
    let mut v17: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = if v17 {
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v18.clone()
    } else {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n--since="); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = if v16 {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" 00:00:00"); } LIT.with(|lit| lit.clone()) };
            v20.clone()
        } else {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v21.clone()
        };
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v2, &*v22, &*v23, &*v23].concat());
        v24.clone()
    };
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("log\n--format=%ad\n--date=short"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v25, &*v27, &*v27, &*v27].concat());
    let mut v29: Rc<str> = git_run_18(v0.clone(), v1.clone(), v28.clone(), v27.clone());
    let mut v30: bool = succeeded_19(v29.clone());
    let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v30 {
        let mut v31: i32 = 1i32;
        let mut v32: Rc<str> = part_at_20(v29.clone(), v31);
        let mut v33: Rc<str> = days_counted_39(v32.clone());
        let mut v34: u64 = line_count_28(v32.clone());
        let mut v35: u64 = v7.wrapping_add(v34);
        let mut v36: u64 = line_count_28(v33.clone());
        let mut v37: u64 = v14.wrapping_add(v36);
        let mut v38: u64 = v15.wrapping_add(1u64);
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("day"); } LIT.with(|lit| lit.clone()) };
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commits"); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = row_26(v39.clone(), v40.clone());
        let mut v42: Rc<str> = lines_joined_22(v41.clone(), v33.clone());
        let mut v43: Rc<str> = aligned_33(v42.clone());
        let mut v44: bool = false;
        let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v3.clone(), v4, v5, v6, v35, v8, v9, v10, v11, v12, v13, v37, v38, v43.clone(), v44);
        (v45.clone(), v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57)
    } else {
        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git log "); } LIT.with(|lit| lit.clone()) };
        let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v1, &*v27, &*v27, &*v27].concat());
        let mut v60: Rc<str> = failure_row_23(v59.clone(), v29.clone());
        let mut v61: bool = true;
        let (mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v60.clone(), v61);
        (v62.clone(), v63, v64, v65, v66, v67, v68, v69, v70, v71, v72, v73, v74)
    };
    (v75.clone(), v76, v77, v78, v79, v80, v81, v82, v83, v84, v85, v86, v87)
}
fn step_run_16(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v163, mut v164, mut v165, mut v166, mut v167, mut v168, mut v169, mut v170, mut v171, mut v172, mut v173, mut v174, mut v175): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
        US1::US1_GitInit(v15) => {
            let mut v15: Rc<str> = v15.clone();
            init_run_17(v0.clone(), v15.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
        }
        _ => {
            let (mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                US1::US1_GitCommit(v29, v30, v31) => {
                    let mut v29: Rc<str> = v29.clone();
                    let mut v30: Rc<str> = v30.clone();
                    let mut v31: Rc<str> = v31.clone();
                    commit_run_24(v0.clone(), v29.clone(), v30.clone(), v31.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                }
                _ => {
                    let (mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145, mut v146, mut v147, mut v148, mut v149): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                        US1::US1_GitHistory(v45, v46, v47) => {
                            let mut v45: Rc<str> = v45.clone();
                            let mut v46: Rc<str> = v46.clone();
                            let mut v47: Rc<str> = v47.clone();
                            history_run_25(v0.clone(), v45.clone(), v46.clone(), v47.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                        }
                        _ => {
                            let (mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                                US1::US1_GitStatus(v61) => {
                                    let mut v61: Rc<str> = v61.clone();
                                    let mut v62: u64 = 0u64;
                                    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repo"); } LIT.with(|lit| lit.clone()) };
                                    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
                                    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                                    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths"); } LIT.with(|lit| lit.clone()) };
                                    let mut v67: Rc<str> = row_26(v65.clone(), v66.clone());
                                    let mut v68: Rc<str> = row_26(v64.clone(), v67.clone());
                                    let mut v69: Rc<str> = row_26(v63.clone(), v68.clone());
                                    repos_status_34(v0.clone(), v61.clone(), v62, v69.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                                }
                                _ => {
                                    let (mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                                        US1::US1_GitDays(v83, v84) => {
                                            let mut v83: Rc<str> = v83.clone();
                                            let mut v84: Rc<str> = v84.clone();
                                            days_run_38(v0.clone(), v83.clone(), v84.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                                        }
                                        _ => {
                                            let (mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                                                US1::US1_Blank => {
                                                    (v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v98.clone(), v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110)
                                        }
                                    };
                                    (v111.clone(), v112, v113, v114, v115, v116, v117, v118, v119, v120, v121, v122, v123)
                                }
                            };
                            (v124.clone(), v125, v126, v127, v128, v129, v130, v131, v132, v133, v134, v135, v136)
                        }
                    };
                    (v137.clone(), v138, v139, v140, v141, v142, v143, v144, v145, v146, v147, v148, v149)
                }
            };
            (v150.clone(), v151, v152, v153, v154, v155, v156, v157, v158, v159, v160, v161, v162)
        }
    };
    (v163.clone(), v164, v165, v166, v167, v168, v169, v170, v171, v172, v173, v174, v175)
}
fn gits_run_15(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v16: i32 = v1.len() as i32;
    let mut v17: bool = v2 < v16;
    let (mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let mut v18: US0 = git_parsed_4(v1.clone(), v2);
        let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
            US0::US0_GitParsed(v19) => {
                let mut v19: US1 = v19.clone();
                let mut v20: i32 = v2.wrapping_add(1i32);
                let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = step_run_16(v0.clone(), v19.clone(), v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15);
                gits_run_15(v0.clone(), v1.clone(), v20, v21.clone(), v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33)
            }
            _ => {
                let (mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
                    US0::US0_GitRejected(v47) => {
                        let mut v47: Rc<str> = v47.clone();
                        let mut v48: i32 = v2.wrapping_add(1i32);
                        gits_run_15(v0.clone(), v1.clone(), v48, v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
                    }
                    _ => unreachable!(),
                };
                (v62.clone(), v63, v64, v65, v66, v67, v68, v69, v70, v71, v72, v73, v74)
            }
        };
        (v75.clone(), v76, v77, v78, v79, v80, v81, v82, v83, v84, v85, v86, v87)
    } else {
        (v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    };
    (v88.clone(), v89, v90, v91, v92, v93, v94, v95, v96, v97, v98, v99, v100)
}
fn segments_joined_41(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v1.clone()
    } else {
        let mut v3: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            v0.clone()
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v1, &*v5, &*v5].concat());
            v6.clone()
        }
    }
}
fn receipt_counts_40(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> Rc<str> {
    let mut v13: u64 = v2.wrapping_add(v3);
    let mut v14: bool = v13 > 0u64;
    let mut v22: Rc<str> = if v14 {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("initialized "); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = decimal_text_37(v2);
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  committed "); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = decimal_text_37(v3);
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v17, &*v18, &*v19].concat());
        v20.clone()
    } else {
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v21.clone()
    };
    let mut v23: bool = v6 > 0u64;
    let mut v31: Rc<str> = if v23 {
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commits "); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = decimal_text_37(v4);
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  matches "); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = decimal_text_37(v5);
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v25, &*v26, &*v27, &*v28].concat());
        v29.clone()
    } else {
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v30.clone()
    };
    let mut v32: bool = v10 > 0u64;
    let mut v43: Rc<str> = if v32 {
        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repos "); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = decimal_text_37(v7);
        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  changed "); } LIT.with(|lit| lit.clone()) };
        let mut v36: Rc<str> = decimal_text_37(v8);
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  untracked "); } LIT.with(|lit| lit.clone()) };
        let mut v38: Rc<str> = decimal_text_37(v9);
        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v38, &*v39, &*v39, &*v39].concat());
        let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v36, &*v40].concat());
        v41.clone()
    } else {
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v42.clone()
    };
    let mut v44: bool = v12 > 0u64;
    let mut v46: bool = if v44 {
        let mut v45: bool = v6 == 0u64;
        v45
    } else {
        false
    };
    let mut v54: Rc<str> = if v46 {
        let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commits "); } LIT.with(|lit| lit.clone()) };
        let mut v48: Rc<str> = decimal_text_37(v4);
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  days "); } LIT.with(|lit| lit.clone()) };
        let mut v50: Rc<str> = decimal_text_37(v11);
        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v48, &*v49, &*v50, &*v51].concat());
        v52.clone()
    } else {
        let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v53.clone()
    };
    let mut v55: Rc<str> = segments_joined_41(v22.clone(), v31.clone());
    let mut v56: Rc<str> = segments_joined_41(v43.clone(), v54.clone());
    segments_joined_41(v55.clone(), v56.clone())
}
fn elapsed_text_43(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_45() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_44() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_45();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_42(mut v0: US2, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US2::US2_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = elapsed_text_43(v2);
    let mut v9: Rc<str> = clock_text_44();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn git_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_1();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1.clone());
    let mut v4: i32 = 1i32;
    let mut v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_git_tally_14();
        let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = gits_run_15(v0.clone(), v3.clone(), v7, v8.clone(), v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
        let mut v34: bool = v21.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v34 {
            ()
        } else {
            println!("{}", &*v21);
            ()
        };
        if v22 {
            let mut v35: US2 = US2::US2_Failed;
            let mut v36: Rc<str> = receipt_counts_40(v21.clone(), v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33);
            receipt_line_42(v35.clone(), v36.clone(), v2);
            1i32
        } else {
            let mut v37: US2 = US2::US2_Succeeded;
            let mut v38: Rc<str> = receipt_counts_40(v21.clone(), v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33);
            receipt_line_42(v37.clone(), v38.clone(), v2);
            0i32
        }
    } else {
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v5, &*v41, &*v41, &*v41].concat());
        println!("{}", &*v42);
        let mut v43: US2 = US2::US2_Failed;
        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repos 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_42(v43.clone(), v44.clone(), v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        git_program_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_git_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
