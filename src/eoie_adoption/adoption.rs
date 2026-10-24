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
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_ClaudeCode,
    US2_Codex,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoDays,
    UH0_DayThen(Rc<str>, u64, u64, u64, Rc<UH0>),
}
#[derive(Clone)]
enum US3 {
    US3_EoieCall,
    US3_HabitCall,
    US3_OtherCall,
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
fn argument_count_5() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn option_value_1(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v1.wrapping_add(1i32);
        let mut v3: i32 = argument_count_5();
        let mut v4: bool = v2 >= v3;
        if v4 {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let mut v6: Rc<str> = argument_2(v1);
            let mut v7: bool = v6.clone() == v0.clone();
            if v7 {
                return argument_2(v2);
            } else {
                (v0, v1) = (v0.clone(), v2);
                continue;
            }
        }
    }
}
fn method7(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_6(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method7(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v4: Option<US1> = v2.map(|x| v3(x));
    let mut v5: US1 = US1::US1_None;
    let mut v6: US1 = v4.unwrap_or(v5);
    match &v6 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn session_files_8(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = { let since = std::time::SystemTime::now() - std::time::Duration::from_secs(v1 * 86400); let mut found = Vec::new(); let mut stack = vec![std::path::PathBuf::from(&*v0)]; while let Some(dir) = stack.pop() { let Ok(entries) = std::fs::read_dir(&dir) else { continue }; for entry in entries.flatten() { let path = entry.path(); if path.is_dir() { stack.push(path); } else if path.extension().map(|extension| extension == "jsonl").unwrap_or(false) && entry.metadata().and_then(|meta| meta.modified()).map(|modified| modified >= since).unwrap_or(false) { found.push(path.to_string_lossy().into_owned()); } } } found.sort(); std::rc::Rc::<str>::from(found.join("
").as_str()) };
    v2.clone()
}
fn cutoff_day_9(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0).saturating_sub(v0 * 86400) as i64; let z = secs.div_euclid(86400) + 719468; let era = z.div_euclid(146097); let doe = z - era * 146097; let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); let mp = (5 * doy + 2) / 153; let day = doy - (153 * mp + 2) / 5 + 1; let month = if mp < 10 { mp + 3 } else { mp - 9 }; let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 }; std::rc::Rc::<str>::from(format!("{year:04}-{month:02}-{day:02}").as_str()) };
    v1.clone()
}
fn file_calls_11(mut v0: Rc<str>, mut v1: US2, mut v2: Rc<str>) -> Rc<str> {
    let mut v6: Rc<str> = match &v1 {
        US2::US2_ClaudeCode => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tool_use"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US2::US2_Codex => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("function_call"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
    };
    let mut v10: Rc<str> = match &v1 {
        US2::US2_ClaudeCode => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x22type\\x22:\\x22tool_use\\x22,\\x22id\\x22:\\x22[^\\x22]*\\x22,\\x22name\\x22:\\x22(?:PowerShell|Bash)\\x22,\\x22input\\x22:\\{\\x22command\\x22:\\x22((?:[^\\x22\\\\]|\\\\.)*)\\x22"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US2::US2_Codex => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x22type\\x22:\\x22function_call\\x22,\\x22name\\x22:\\x22(?:shell|exec_command|local_shell)\\x22,\\x22arguments\\x22:\\x22((?:[^\\x22\\\\]|\\\\.)*)\\x22"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
    };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x22timestamp\\x22:\\x22(\\d{4}-\\d\\d-\\d\\d)"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { let quote = char::from(34u8); let backslash = char::from(92u8); let stamp = regex::Regex::new(&*v11).unwrap(); let command = regex::Regex::new(&*v10).unwrap(); let text = std::fs::read_to_string(&*v0).unwrap_or_default(); let mut out = String::new(); for line in text.lines().filter(|line| line.contains(&*v6)) { let Some(day) = stamp.captures(line).map(|found| found[1].to_owned()) else { continue }; if day.as_str() < &*v2 { continue; } for found in command.captures_iter(line) { let decoded = found[1].replace(&format!("{backslash}{quote}"), &quote.to_string()).replace(&format!("{backslash}{backslash}"), &backslash.to_string()).replace(char::from(10u8), " ").replace(char::from(9u8), " "); out.push_str(&format!("{day}	{decoded}
")); } } std::rc::Rc::<str>::from(out.as_str()) };
    v12.clone()
}
fn eoie_named_14(mut v0: Rc<str>) -> bool {
    let mut v1: bool = { let text = &*v0; let bytes = text.as_bytes(); let separators = [32u8, 9, 13, 10, 38, 39, 34, 92, 47, 40, 59, 124]; text.match_indices("eoie").any(|(at, _)| { let before = at == 0 || separators.contains(&bytes[at - 1]); let mut rest = &text[at + 4..]; if let Some(stripped) = rest.strip_prefix(".exe") { rest = stripped; } if let Some(stripped) = rest.strip_prefix(|c: char| c == char::from(39u8) || c == char::from(34u8)) { rest = stripped; } let trimmed = rest.trim_start(); before && trimmed.len() < rest.len() && trimmed.chars().next().is_some_and(|c| c.is_ascii_lowercase()) }) };
    v1
}
fn habit_named_15(mut v0: Rc<str>) -> bool {
    let mut v1: bool = { let lower = (&*v0).to_ascii_lowercase(); let bytes = lower.as_bytes(); let word_byte = |byte: u8| byte.is_ascii_alphanumeric() || byte == 95u8; let words = ["get-content", "select-string", "get-childitem", "set-content", "add-content", "out-file", "foreach-object", "measure-object", "copy-item", "remove-item", "move-item", "test-path", "start-sleep", "robocopy", "findstr", "python", "python3", "py.exe"]; lower.contains("[io.file]") || words.iter().any(|word| lower.match_indices(word).any(|(at, _)| { let end = at + word.len(); (at == 0 || !word_byte(bytes[at - 1])) && (end >= bytes.len() || !word_byte(bytes[end])) })) };
    v1
}
fn call_of_command_13(mut v0: Rc<str>) -> US3 {
    let mut v1: bool = eoie_named_14(v0.clone());
    if v1 {
        US3::US3_EoieCall
    } else {
        let mut v3: bool = habit_named_15(v0.clone());
        if v3 {
            US3::US3_HabitCall
        } else {
            US3::US3_OtherCall
        }
    }
}
fn tally_added_16(mut v0: Rc<str>, mut v1: US3, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v2 {
        UH0::UH0_DayThen(v14, v15, v16, v17, v18) => {
            let mut v14: Rc<str> = v14.clone();
            let mut v15: u64 = *v15;
            let mut v16: u64 = *v16;
            let mut v17: u64 = *v17;
            let mut v18: Rc<UH0> = v18.clone();
            let mut v19: bool = v14.clone() == v0.clone();
            if v19 {
                let (mut v31, mut v32, mut v33): (u64, u64, u64) = match &v1 {
                    US3::US3_EoieCall => {
                        let mut v20: u64 = v15.wrapping_add(1u64);
                        let mut v21: u64 = v16.wrapping_add(1u64);
                        (v20, v21, v17)
                    }
                    _ => {
                        let (mut v28, mut v29, mut v30): (u64, u64, u64) = match &v1 {
                            US3::US3_HabitCall => {
                                let mut v22: u64 = v15.wrapping_add(1u64);
                                let mut v23: u64 = v17.wrapping_add(1u64);
                                (v22, v16, v23)
                            }
                            _ => {
                                let (mut v25, mut v26, mut v27): (u64, u64, u64) = match &v1 {
                                    US3::US3_OtherCall => {
                                        let mut v24: u64 = v15.wrapping_add(1u64);
                                        (v24, v16, v17)
                                    }
                                    _ => unreachable!(),
                                };
                                (v25, v26, v27)
                            }
                        };
                        (v28, v29, v30)
                    }
                };
                Rc::new(UH0::UH0_DayThen(v14.clone(), v31, v32, v33, v18.clone()))
            } else {
                let mut v35: Rc<UH0> = tally_added_16(v0.clone(), v1.clone(), v18.clone());
                Rc::new(UH0::UH0_DayThen(v14.clone(), v15, v16, v17, v35.clone()))
            }
        }
        UH0::UH0_NoDays => {
            let (mut v9, mut v10, mut v11): (u64, u64, u64) = match &v1 {
                US3::US3_EoieCall => {
                    (1u64, 1u64, 0u64)
                }
                _ => {
                    let (mut v6, mut v7, mut v8): (u64, u64, u64) = match &v1 {
                        US3::US3_HabitCall => {
                            (1u64, 0u64, 1u64)
                        }
                        _ => {
                            let (mut v3, mut v4, mut v5): (u64, u64, u64) = match &v1 {
                                US3::US3_OtherCall => {
                                    (1u64, 0u64, 0u64)
                                }
                                _ => unreachable!(),
                            };
                            (v3, v4, v5)
                        }
                    };
                    (v6, v7, v8)
                }
            };
            let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoDays); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_DayThen(v0.clone(), v9, v10, v11, v12.clone()))
        }
    }
}
fn calls_tallied_12(mut v0: US2, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: u64 = (&*v1).len() as u64;
        let mut v5: bool = v2 >= v4;
        if v5 {
            return v3.clone();
        } else {
            let mut v6: u64 = (&*v1)[v2 as usize..].find(char::from(10u8)).map(|found| v2 + found as u64).unwrap_or((&*v1).len() as u64);
            let mut v7: Rc<str> = std::rc::Rc::<str>::from(&(&*v1)[v2 as usize..v6 as usize]);
            let mut v11: Rc<str> = match &v0 {
                US2::US2_ClaudeCode => {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                }
                US2::US2_Codex => {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("codex"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                }
            };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
            let mut v14: Rc<str> = std::rc::Rc::<str>::from((&*v7).split(char::from(9u8)).nth(0i32 as usize).unwrap_or(""));
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            let mut v16: u64 = v6.wrapping_add(1u64);
            let mut v17: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<UH0> = if v17 {
                v3.clone()
            } else {
                let mut v18: Rc<str> = std::rc::Rc::<str>::from((&*v7).split(char::from(9u8)).nth(1i32 as usize).unwrap_or(""));
                let mut v19: US3 = call_of_command_13(v18.clone());
                tally_added_16(v15.clone(), v19.clone(), v3.clone())
            };
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v16, v21.clone());
            continue;
        }
    }
}
fn files_tallied_10(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v5: u64 = (&*v1).len() as u64;
        let mut v6: bool = v3 >= v5;
        if v6 {
            return v4.clone();
        } else {
            let mut v7: u64 = (&*v1)[v3 as usize..].find(char::from(10u8)).map(|found| v3 + found as u64).unwrap_or((&*v1).len() as u64);
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(&(&*v1)[v3 as usize..v7 as usize]);
            let mut v9: u64 = v7.wrapping_add(1u64);
            let mut v10: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<UH0> = if v10 {
                v4.clone()
            } else {
                let mut v11: Rc<str> = file_calls_11(v8.clone(), v0.clone(), v2.clone());
                let mut v12: u64 = 0u64;
                calls_tallied_12(v0.clone(), v11.clone(), v12, v4.clone())
            };
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v9, v14.clone());
            continue;
        }
    }
}
fn eoie_share_18(mut v0: u64, mut v1: u64, mut v2: u64) -> f64 {
    let mut v3: bool = v0 == 0u64;
    if v3 {
        0.0f64
    } else {
        let mut v4: f64 = (v1 as f64) / (v0 as f64);
        v4
    }
}
fn rows_of_17(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64) -> (Rc<str>, u64, u64, u64) {
    loop {
        match &*v0 {
            UH0::UH0_DayThen(v5, v6, v7, v8, v9) => {
                let mut v5: Rc<str> = v5.clone();
                let mut v6: u64 = *v6;
                let mut v7: u64 = *v7;
                let mut v8: u64 = *v8;
                let mut v9: Rc<UH0> = v9.clone();
                let mut v10: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v42: Rc<str> = if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v11));
                    let mut v13: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
                    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v11));
                    let mut v16: Rc<str> = std::rc::Rc::<str>::from((v7).to_string());
                    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v11));
                    let mut v18: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
                    let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
                    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v11));
                    let mut v21: f64 = eoie_share_18(v6, v7, v8);
                    let mut v22: Rc<str> = std::rc::Rc::<str>::from(format!("{:.3}", v21));
                    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v22));
                    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v23));
                    v24.clone()
                } else {
                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v25));
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v27));
                    let mut v29: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
                    let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
                    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v27));
                    let mut v32: Rc<str> = std::rc::Rc::<str>::from((v7).to_string());
                    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v27));
                    let mut v34: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
                    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v27));
                    let mut v37: f64 = eoie_share_18(v6, v7, v8);
                    let mut v38: Rc<str> = std::rc::Rc::<str>::from(format!("{:.3}", v37));
                    let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v38));
                    let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v39));
                    let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v40));
                    v41.clone()
                };
                let mut v43: u64 = v2.wrapping_add(v6);
                let mut v44: u64 = v3.wrapping_add(v7);
                let mut v45: u64 = v4.wrapping_add(v8);
                (v0, v1, v2, v3, v4) = (v9.clone(), v42.clone(), v43, v44, v45);
                continue;
            }
            UH0::UH0_NoDays => {
                return (v1.clone(), v2, v3, v4);
            }
        }
    }
}
fn adoption_run_0() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--days"); } LIT.with(|lit| lit.clone()) };
    let mut v1: i32 = 2i32;
    let mut v2: Rc<str> = option_value_1(v0.clone(), v1);
    let mut v3: u64 = decimal_u64_6(v2.clone());
    let mut v4: bool = v3 == 0u64;
    let mut v5: u64 = if v4 {
        7u64
    } else {
        v3
    };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--out"); } LIT.with(|lit| lit.clone()) };
    let mut v7: i32 = 2i32;
    let mut v8: Rc<str> = option_value_1(v6.clone(), v7);
    let mut v9: US2 = US2::US2_ClaudeCode;
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".claude/projects"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(std::path::PathBuf::from).unwrap_or_default().join(&*v10).to_string_lossy().as_ref());
    let mut v12: Rc<str> = session_files_8(v11.clone(), v5);
    let mut v13: Rc<str> = cutoff_day_9(v5);
    let mut v14: u64 = 0u64;
    let mut v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoDays); } CASE.with(|case| case.clone()) };
    let mut v16: Rc<UH0> = files_tallied_10(v9.clone(), v12.clone(), v13.clone(), v14, v15.clone());
    let mut v17: US2 = US2::US2_Codex;
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".codex/sessions"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(std::path::PathBuf::from).unwrap_or_default().join(&*v18).to_string_lossy().as_ref());
    let mut v20: Rc<str> = session_files_8(v19.clone(), v5);
    let mut v21: Rc<str> = cutoff_day_9(v5);
    let mut v22: u64 = 0u64;
    let mut v23: Rc<UH0> = files_tallied_10(v17.clone(), v20.clone(), v21.clone(), v22, v16.clone());
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v25: u64 = 0u64;
    let mut v26: u64 = 0u64;
    let mut v27: u64 = 0u64;
    let (mut v28, mut v29, mut v30, mut v31): (Rc<str>, u64, u64, u64) = rows_of_17(v23.clone(), v24.clone(), v25, v26, v27);
    let mut v32: Rc<str> = { let mut lines = (&*v28).lines().filter(|line| !line.is_empty()).collect::<Vec<_>>(); lines.sort(); std::rc::Rc::<str>::from(lines.join("
")) };
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("client\tday\tshell_calls\teoie_calls\thabit_calls\teoie_share\n"); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
    let mut v37: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = if v37 {
        v24.clone()
    } else {
        let mut v38: Rc<str> = std::rc::Rc::<str>::from(std::fs::write(&*v8, (&*v36).as_bytes()).err().map(|error| format!("write {}: {error}", &*v8)).unwrap_or_default().as_str());
        v38.clone()
    };
    let mut v40: bool = v39.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v40 {
        eprintln!("eoie error: adoption: {}", &*v39);
        2i32
    } else {
        if v37 {
            print!("{}", &*v36);
            ()
        };
        let mut v41: Rc<str> = std::rc::Rc::<str>::from((v29).to_string());
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shell_calls="); } LIT.with(|lit| lit.clone()) };
        let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v41));
        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" eoie_calls="); } LIT.with(|lit| lit.clone()) };
        let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
        let mut v46: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
        let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
        let mut v48: Rc<str> = std::rc::Rc::<str>::from((v31).to_string());
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" habit_calls="); } LIT.with(|lit| lit.clone()) };
        let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v48));
        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" eoie_share="); } LIT.with(|lit| lit.clone()) };
        let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v51));
        let mut v53: f64 = eoie_share_18(v29, v30, v31);
        let mut v54: Rc<str> = std::rc::Rc::<str>::from(format!("{:.3}", v53));
        let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v54));
        let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v55));
        let mut v57: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie adoption ok days="); } LIT.with(|lit| lit.clone()) };
        let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v57));
        let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v56));
        let mut v62: Rc<str> = if v37 {
            v24.clone()
        } else {
            let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
            v61.clone()
        };
        let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v62));
        let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v8));
        println!("{}", &*v64);
        0i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        adoption_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_adoption_run() -> i32 {
    closure0()()
}
