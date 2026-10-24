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
    US1_TableQuery,
    US1_TableDiff,
    US1_TableUnknown,
}
#[derive(Clone)]
enum US2 {
    US2_StepTaken(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool),
    US2_StepRejected(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US4_TableUnreadable(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_WhereEqual,
    US5_WhereNotEqual,
    US5_WhereGreater,
    US5_WhereGreaterEqual,
    US5_WhereLess,
    US5_WhereLessEqual,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoFilters,
    UH0_FilterThen(i32, US5, Rc<str>, Rc<UH0>),
}
fn measured_query_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let mut v9: i32 = eoie_table_measures::eoie_table_measured(&*v0, &*v1, &*v3, &*v6, v5, v8);
    v9
}
fn query_rows_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool) -> i32 {
    let mut v6: i32 = (|| -> i32 { let path = &*v0; let text = match std::fs::read_to_string(path) { Ok(text) => text, Err(error) => { eprintln!("eoie error: table {path}: {error}"); return 2; } }; let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; let mut lines = text.lines(); let header = lines.next().unwrap_or("").split(separator).map(str::to_owned).collect::<Vec<_>>(); let column = |name: &str| header.iter().position(|item| item == name); let mut missing = Vec::new(); let mut filters = Vec::new(); let compare = |cell: &str, op: &str, value: &str| -> bool { if op == "=" { return cell == value; } if op == "!=" { return cell != value; } let pair = cell.trim().parse::<f64>().ok().zip(value.trim().parse::<f64>().ok()); match (op, pair) { (">", Some((a, b))) => a > b, (">=", Some((a, b))) => a >= b, ("<", Some((a, b))) => a < b, ("<=", Some((a, b))) => a <= b, (">", None) => cell > value, (">=", None) => cell >= value, ("<", None) => cell < value, ("<=", None) => cell <= value, _ => false } }; for clause in (&*v1).lines() { if let Some(at) = clause.find(|c: char| c == char::from(60u8) || c == char::from(62u8) || c == char::from(61u8) || c == char::from(33u8)) { let bytes = clause.as_bytes(); let width = if at + 1 < bytes.len() && bytes[at + 1] == 61u8 && bytes[at] != 61u8 { 2 } else { 1 }; let name = &clause[..at]; match column(name) { Some(index) => filters.push((index, clause[at..at + width].to_owned(), clause[at + width..].to_owned())), None => missing.push(name.to_owned()) } } } let projection = if (&*v2).is_empty() { (0..header.len()).collect::<Vec<_>>() } else { (&*v2).split(char::from(44u8)).filter_map(|name| match column(name) { Some(index) => Some(index), None => { missing.push(name.to_owned()); None } }).collect::<Vec<_>>() }; let grouping = if (&*v3).is_empty() { None } else { match column(&*v3) { Some(index) => Some(index), None => { missing.push((&*v3).to_owned()); None } } }; if !missing.is_empty() { eprintln!("eoie error: table unknown columns: {} (header: {})", missing.join(","), header.join(",")); return 2; } let mut total = 0usize; let mut matched = 0usize; let mut shown = 0usize; let mut groups: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new(); if grouping.is_none() { println!("{}", projection.iter().map(|index| header[*index].as_str()).collect::<Vec<_>>().join("	")); } for line in lines { total += 1; let cells = line.split(separator).collect::<Vec<_>>(); if !filters.iter().all(|(index, op, value)| cells.get(*index).map(|cell| compare(cell, op, value)).unwrap_or(false)) { continue; } matched += 1; match grouping { Some(index) => { *groups.entry(cells.get(index).unwrap_or(&"").to_string()).or_insert(0) += 1; } None => { if (shown as u64) < v4 { println!("{}", projection.iter().map(|index| *cells.get(*index).unwrap_or(&"")).collect::<Vec<_>>().join("	")); shown += 1; } } } } if grouping.is_some() { let mut counted = groups.into_iter().collect::<Vec<_>>(); counted.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0))); println!("count	{}", &*v3); for (value, count) in counted.iter().take(v4 as usize) { println!("{count}	{value}"); shown += 1; } if !v5 { println!("eoie table query ok total={total} matched={matched} groups={} shown={shown}", counted.len()); } } else if !v5 { println!("eoie table query ok total={total} matched={matched} shown={shown}"); } 0 })();
    v6
}
fn command_line_argument_count_4() -> i32 {
    let mut v0: i32 = (std::env::args().count() as i32) - 1;
    v0
}
fn command_line_argument_5(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::args().nth((v0 + 1) as usize).unwrap_or_default());
    v1.clone()
}
fn argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_4();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_5(v1);
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
fn argument_count_6() -> i32 {
    let mut v0: i32 = command_line_argument_count_4();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn no_request_7() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0.clone(), false)
}
fn each_prefixed_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).map(|item| format!("{}{}", &*v0, item.trim())).collect::<Vec<_>>().join(",").as_str());
    v2.clone()
}
fn method12(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_11(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method12(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v5: US3 = US3::US3_None;
    let mut v6: US3 = v4.unwrap_or(v5);
    match &v6 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v7) => {
            let mut v7: u64 = *v7;
            v7
        }
    }
}
fn option_applied_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>) -> US2 {
    let mut v10: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where"); } LIT.with(|lit| lit.clone()) };
    if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
        let mut v12: bool = v9.contains(&*v11);
        let mut v15: bool = if v12 {
            true
        } else {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = v9.contains(&*v13);
            v14
        };
        let mut v18: bool = if v15 {
            true
        } else {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
            let mut v17: bool = v9.contains(&*v16);
            v17
        };
        if v18 {
            let mut v19: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = if v19 {
                v9.clone()
            } else {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v20, &*v9, &*v21, &*v21].concat());
                v22.clone()
            };
            US2::US2_StepTaken(v23.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
        } else {
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where expects col=value (or col!=value, col>value, col>=value, col<value, col<=value), got "); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v9, &*v26, &*v26, &*v26].concat());
            US2::US2_StepRejected(v27.clone())
        }
    } else {
        let mut v30: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--cols"); } LIT.with(|lit| lit.clone()) };
        if v30 {
            US2::US2_StepTaken(v0.clone(), v9.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
        } else {
            let mut v32: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--group"); } LIT.with(|lit| lit.clone()) };
            if v32 {
                US2::US2_StepTaken(v0.clone(), v1.clone(), v9.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
            } else {
                let mut v34: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--key"); } LIT.with(|lit| lit.clone()) };
                if v34 {
                    US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v9.clone(), v4, v5.clone(), v6.clone(), v7)
                } else {
                    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mean|--sd|--sum|--min|--max|--count"); } LIT.with(|lit| lit.clone()) };
                    let mut v37: bool = v36.split("|").any(|item| item == &*v8);
                    if v37 {
                        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        let mut v39: Rc<str> = std::rc::Rc::<str>::from((v8).trim_start_matches(&*v38));
                        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v40, &*v41, &*v41, &*v41].concat());
                        let mut v43: Rc<str> = each_prefixed_10(v42.clone(), v9.clone());
                        let mut v44: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v49: Rc<str> = if v44 {
                            v43.clone()
                        } else {
                            let mut v45: bool = v43.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            if v45 {
                                v5.clone()
                            } else {
                                let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v46, &*v43, &*v41, &*v41].concat());
                                v47.clone()
                            }
                        };
                        US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v49.clone(), v6.clone(), v7)
                    } else {
                        let mut v51: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--latest-by"); } LIT.with(|lit| lit.clone()) };
                        if v51 {
                            US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v9.clone(), v7)
                        } else {
                            let mut v53: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit"); } LIT.with(|lit| lit.clone()) };
                            if v53 {
                                let mut v54: u64 = decimal_u64_11(v9.clone());
                                let mut v55: bool = v54 == 0u64;
                                if v55 {
                                    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit expects a positive integer"); } LIT.with(|lit| lit.clone()) };
                                    US2::US2_StepRejected(v56.clone())
                                } else {
                                    US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v54, v5.clone(), v6.clone(), v7)
                                }
                            } else {
                                let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                                let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v62: Rc<str> = std::rc::Rc::<str>::from([&*v60, &*v8, &*v61, &*v61, &*v61].concat());
                                US2::US2_StepRejected(v62.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn options_from_8(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool) -> US2 {
    loop {
        let mut v10: bool = v0 < v1;
        if v10 {
            let mut v11: Rc<str> = argument_3(v0);
            let mut v12: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet"); } LIT.with(|lit| lit.clone()) };
            if v12 {
                let mut v13: i32 = v0.wrapping_add(1i32);
                let mut v14: bool = true;
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v13, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v14);
                continue;
            } else {
                let mut v16: i32 = v0.wrapping_add(1i32);
                let mut v17: bool = v16 < v1;
                if v17 {
                    let mut v18: Rc<str> = argument_3(v0);
                    let mut v19: Rc<str> = argument_3(v16);
                    let mut v20: US2 = option_applied_9(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9, v18.clone(), v19.clone());
                    match &v20 {
                        US2::US2_StepRejected(v31) => {
                            let mut v31: Rc<str> = v31.clone();
                            return US2::US2_StepRejected(v31.clone());
                        }
                        US2::US2_StepTaken(v21, v22, v23, v24, v25, v26, v27, v28) => {
                            let mut v21: Rc<str> = v21.clone();
                            let mut v22: Rc<str> = v22.clone();
                            let mut v23: Rc<str> = v23.clone();
                            let mut v24: Rc<str> = v24.clone();
                            let mut v25: u64 = *v25;
                            let mut v26: Rc<str> = v26.clone();
                            let mut v27: Rc<str> = v27.clone();
                            let mut v28: bool = *v28;
                            let mut v29: i32 = v0.wrapping_add(2i32);
                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v29, v1, v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25, v26.clone(), v27.clone(), v28);
                            continue;
                        }
                    }
                } else {
                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) };
                    let mut v36: Rc<str> = argument_3(v0);
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) };
                    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v36, &*v37, &*v38, &*v38].concat());
                    return US2::US2_StepRejected(v39.clone());
                }
            }
        } else {
            return US2::US2_StepTaken(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9);
        }
    }
}
fn table_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_3(v0);
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
    let mut v8: US1 = if v2 {
        US1::US1_TableQuery
    } else {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            US1::US1_TableDiff
        } else {
            US1::US1_TableUnknown
        }
    };
    match &v8 {
        US1::US1_TableDiff => {
            let mut v49: i32 = argument_count_6();
            let mut v50: bool = v49 < 5i32;
            if v50 {
                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
                let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
                let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v52, &*v51, &*v53, &*v53, &*v53].concat());
                eprintln!("{}", &*v54);
                2i32
            } else {
                let mut v55: i32 = 5i32;
                let mut v56: i32 = argument_count_6();
                let (mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_7();
                let mut v65: US2 = options_from_8(v55, v56, v57.clone(), v58.clone(), v59.clone(), v60.clone(), v61, v62.clone(), v63.clone(), v64);
                match &v65 {
                    US2::US2_StepRejected(v79) => {
                        let mut v79: Rc<str> = v79.clone();
                        let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
                        let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v82: Rc<str> = std::rc::Rc::<str>::from([&*v80, &*v79, &*v81, &*v81, &*v81].concat());
                        eprintln!("{}", &*v82);
                        2i32
                    }
                    US2::US2_StepTaken(v66, v67, v68, v69, v70, v71, v72, v73) => {
                        let mut v66: Rc<str> = v66.clone();
                        let mut v67: Rc<str> = v67.clone();
                        let mut v68: Rc<str> = v68.clone();
                        let mut v69: Rc<str> = v69.clone();
                        let mut v70: u64 = *v70;
                        let mut v71: Rc<str> = v71.clone();
                        let mut v72: Rc<str> = v72.clone();
                        let mut v73: bool = *v73;
                        let mut v74: i32 = 3i32;
                        let mut v75: Rc<str> = argument_3(v74);
                        let mut v76: i32 = 4i32;
                        let mut v77: Rc<str> = argument_3(v76);
                        let mut v78: i32 = eoie_table_diff::eoie_table_diff_run(&*v75, &*v77, &*v69, &*v67, &*v72, v70);
                        v78
                    }
                }
            }
        }
        US1::US1_TableQuery => {
            let mut v9: i32 = argument_count_6();
            let mut v10: bool = v9 < 4i32;
            if v10 {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v11, &*v13, &*v13, &*v13].concat());
                eprintln!("{}", &*v14);
                2i32
            } else {
                let mut v15: i32 = 4i32;
                let mut v16: i32 = argument_count_6();
                let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_7();
                let mut v25: US2 = options_from_8(v15, v16, v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21, v22.clone(), v23.clone(), v24);
                match &v25 {
                    US2::US2_StepRejected(v42) => {
                        let mut v42: Rc<str> = v42.clone();
                        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v42, &*v44, &*v44, &*v44].concat());
                        eprintln!("{}", &*v45);
                        2i32
                    }
                    US2::US2_StepTaken(v26, v27, v28, v29, v30, v31, v32, v33) => {
                        let mut v26: Rc<str> = v26.clone();
                        let mut v27: Rc<str> = v27.clone();
                        let mut v28: Rc<str> = v28.clone();
                        let mut v29: Rc<str> = v29.clone();
                        let mut v30: u64 = *v30;
                        let mut v31: Rc<str> = v31.clone();
                        let mut v32: Rc<str> = v32.clone();
                        let mut v33: bool = *v33;
                        let mut v34: bool = v31.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        if v34 {
                            let mut v35: i32 = 3i32;
                            let mut v36: Rc<str> = argument_3(v35);
                            measured_query_1(v36.clone(), v26.clone(), v27.clone(), v28.clone(), v29.clone(), v30, v31.clone(), v32.clone(), v33)
                        } else {
                            let mut v38: i32 = 3i32;
                            let mut v39: Rc<str> = argument_3(v38);
                            query_rows_2(v39.clone(), v26.clone(), v27.clone(), v28.clone(), v30, v33)
                        }
                    }
                }
            }
        }
        US1::US1_TableUnknown => {
            let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
            let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
            let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v89: Rc<str> = std::rc::Rc::<str>::from([&*v87, &*v86, &*v88, &*v88, &*v88].concat());
            eprintln!("{}", &*v89);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        table_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn filters_from_14(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v3.clone(), v4.clone())
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let mut v7: i32 = (v6).find(|c: char| c == char::from(60u8) || c == char::from(62u8) || c == char::from(61u8) || c == char::from(33u8)).map(|at| at as i32).unwrap_or(-1);
        let mut v8: bool = v7 < 0i32;
        let mut v10: i32 = if v8 {
            0i32
        } else {
            let mut v9: i32 = { let bytes = (v6).as_bytes(); let at = v7 as usize; if at + 1 < bytes.len() && bytes[at + 1] == 61u8 && bytes[at] != 61u8 { 2 } else { 1 } };
            v9
        };
        let mut v12: Rc<str> = if v8 {
            v6.clone()
        } else {
            let mut v11: Rc<str> = std::rc::Rc::<str>::from(&(v6)[0i32 as usize..v7 as usize]);
            v11.clone()
        };
        let mut v32: US5 = if v8 {
            US5::US5_WhereEqual
        } else {
            let mut v14: i32 = v7.wrapping_add(v10);
            let mut v15: Rc<str> = std::rc::Rc::<str>::from(&(v6)[v7 as usize..v14 as usize]);
            let mut v16: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!="); } LIT.with(|lit| lit.clone()) };
            if v16 {
                US5::US5_WhereNotEqual
            } else {
                let mut v18: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                if v18 {
                    US5::US5_WhereGreater
                } else {
                    let mut v20: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">="); } LIT.with(|lit| lit.clone()) };
                    if v20 {
                        US5::US5_WhereGreaterEqual
                    } else {
                        let mut v22: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
                        if v22 {
                            US5::US5_WhereLess
                        } else {
                            let mut v24: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<="); } LIT.with(|lit| lit.clone()) };
                            if v24 {
                                US5::US5_WhereLessEqual
                            } else {
                                US5::US5_WhereEqual
                            }
                        }
                    }
                }
            }
        };
        let mut v37: Rc<str> = if v8 {
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v33.clone()
        } else {
            let mut v34: i32 = v7.wrapping_add(v10);
            let mut v35: i32 = (v6.clone().len() as i32);
            let mut v36: Rc<str> = std::rc::Rc::<str>::from(&(v6)[v34 as usize..v35 as usize]);
            v36.clone()
        };
        let mut v38: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v12)).map(|index| index as i32).unwrap_or(-1);
        let mut v39: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v40, mut v41): (Rc<UH0>, Rc<str>) = filters_from_14(v0.clone(), v39.clone());
        let mut v42: bool = v38 < 0i32;
        if v42 {
            let mut v43: bool = v12.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v49: Rc<str> = if v43 {
                v41.clone()
            } else {
                let mut v44: bool = v41.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v44 {
                    v12.clone()
                } else {
                    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v45, &*v41, &*v46, &*v46].concat());
                    v47.clone()
                }
            };
            (v40.clone(), v49.clone())
        } else {
            let mut v50: Rc<UH0> = Rc::new(UH0::UH0_FilterThen(v38, v32.clone(), v37.clone(), v40.clone()));
            (v50.clone(), v41.clone())
        }
    }
}
fn cell_passes_17(mut v0: US5, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    match &v0 {
        US5::US5_WhereEqual => {
            let mut v3: bool = v1.clone() == v2.clone();
            v3
        }
        US5::US5_WhereNotEqual => {
            let mut v4: bool = v1.clone() != v2.clone();
            v4
        }
        _ => {
            let mut v5: i32 = match (v1).trim().parse::<f64>().ok().zip((v2).trim().parse::<f64>().ok()) { Some((a, b)) => if a < b { -1 } else if a > b { 1 } else { 0 }, None => 2 };
            let mut v6: bool = v5 == 2i32;
            let mut v8: i32 = if v6 {
                let mut v7: i32 = match (&*v1).cmp(&*v2) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 };
                v7
            } else {
                v5
            };
            match &v0 {
                US5::US5_WhereGreater => {
                    let mut v9: bool = v8 > 0i32;
                    v9
                }
                US5::US5_WhereGreaterEqual => {
                    let mut v10: bool = v8 >= 0i32;
                    v10
                }
                US5::US5_WhereLess => {
                    let mut v11: bool = v8 < 0i32;
                    v11
                }
                _ => {
                    let mut v12: bool = v8 <= 0i32;
                    v12
                }
            }
        }
    }
}
fn row_passes_16(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v3, v4, v5, v6) => {
                let mut v3: i32 = *v3;
                let mut v4: US5 = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: Rc<str> = std::rc::Rc::<str>::from((v1).get(v2 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
                let mut v8: bool = cell_passes_17(v4.clone(), v7.clone(), v5.clone());
                if v8 {
                    (v0, v1, v2) = (v6.clone(), v1.clone(), v2);
                    continue;
                } else {
                    return false;
                }
            }
            UH0::UH0_NoFilters => {
                return true;
            }
        }
    }
}
fn kept_mask_15(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v2 > v3;
        if v5 {
            return v4.clone();
        } else {
            let mut v6: i32 = v2.wrapping_add(1i32);
            let mut v7: bool = row_passes_16(v0.clone(), v1.clone(), v2);
            let mut v10: Rc<str> = if v7 {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                v8.clone()
            } else {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v10, &*v11, &*v11, &*v11].concat());
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v6, v3, v12.clone());
            continue;
        }
    }
}
fn rows_kept_13(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, Rc<str>) {
    let (mut v2, mut v3, mut v4): (bool, std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, Rc<str>) = { let path = &*v0; match std::fs::read_to_string(path) { Ok(text) => { let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; (true, std::rc::Rc::new(text.lines().map(|line| line.split(separator).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>()), { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) } Err(error) => (false, std::rc::Rc::new(Vec::new()), std::rc::Rc::<str>::from(format!("{path}: {error}").as_str())) } };
    let mut v7: US4 = if v2 {
        US4::US4_TableLoaded(v3.clone())
    } else {
        US4::US4_TableUnreadable(v4.clone())
    };
    match &v7 {
        US4::US4_TableLoaded(v9) => {
            let mut v9: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v9.clone();
            let (mut v10, mut v11): (Rc<UH0>, Rc<str>) = filters_from_14(v9.clone(), v1.clone());
            let mut v12: bool = v11.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v12 {
                (false, v11.clone())
            } else {
                let mut v13: i32 = 1i32;
                let mut v14: i32 = ((v9).len().max(1) - 1) as i32;
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = kept_mask_15(v10.clone(), v9.clone(), v13, v14, v15.clone());
                (true, v16.clone())
            }
        }
        US4::US4_TableUnreadable(v8) => {
            let mut v8: Rc<str> = v8.clone();
            (false, v8.clone())
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (bool, Rc<str>) {
        rows_kept_13(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_table_run() -> i32 {
    closure0()()
}
pub fn eoie_table_rows_kept(v0: &str, v1: &str) -> (bool, Rc<str>) {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
