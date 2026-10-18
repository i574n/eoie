#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_TableQuery,
    US0_TableDiff,
    US0_TableUnknown,
}
#[derive(Clone)]
enum US1 {
    US1_StepTaken(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64),
    US1_StepRejected(Rc<str>),
}
fn diff_rows_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let mut v5: i32 = (|| -> i32 { let read = |path: &str| -> Result<(Vec<String>, Vec<Vec<String>>), String> { let text = std::fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?; let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; let mut lines = text.lines(); let header = lines.next().unwrap_or("").split(separator).map(str::to_owned).collect::<Vec<_>>(); Ok((header, lines.map(|line| line.split(separator).map(str::to_owned).collect()).collect())) }; let (left_header, left_rows) = match read(&*v0) { Ok(table) => table, Err(error) => { eprintln!("eoie error: table {error}"); return 2; } }; let (right_header, right_rows) = match read(&*v1) { Ok(table) => table, Err(error) => { eprintln!("eoie error: table {error}"); return 2; } }; let keys = (&*v2).split(char::from(44u8)).filter(|name| !name.is_empty()).map(str::to_owned).collect::<Vec<_>>(); if keys.is_empty() { eprintln!("eoie error: table diff needs --key a,b"); return 2; } let position = |header: &Vec<String>, name: &str| header.iter().position(|item| item == name); let mut missing = keys.iter().filter(|name| position(&left_header, name).is_none() || position(&right_header, name).is_none()).cloned().collect::<Vec<_>>(); let compared = if (&*v3).is_empty() { left_header.iter().filter(|name| !keys.contains(name) && position(&right_header, name).is_some()).cloned().collect::<Vec<_>>() } else { (&*v3).split(char::from(44u8)).map(str::to_owned).collect::<Vec<_>>() }; missing.extend(compared.iter().filter(|name| position(&left_header, name).is_none() || position(&right_header, name).is_none()).cloned()); if !missing.is_empty() { eprintln!("eoie error: table diff unknown columns: {}", missing.join(",")); return 2; } let key_of = |header: &Vec<String>, row: &Vec<String>| keys.iter().map(|name| row.get(position(header, name).unwrap()).cloned().unwrap_or_default()).collect::<Vec<_>>().join("|"); let right_index = right_rows.iter().map(|row| (key_of(&right_header, row), row)).collect::<std::collections::HashMap<_, _>>(); let mut counts: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new(); let mut samples = Vec::new(); let mut only_left = 0usize; let mut seen = std::collections::HashSet::new(); for row in &left_rows { let key_text = key_of(&left_header, row); seen.insert(key_text.clone()); match right_index.get(&key_text) { None => only_left += 1, Some(other) => { for name in &compared { let a = row.get(position(&left_header, name).unwrap()).cloned().unwrap_or_default(); let b = other.get(position(&right_header, name).unwrap()).cloned().unwrap_or_default(); if a != b { *counts.entry(name.clone()).or_insert(0) += 1; if (samples.len() as u64) < v4 { samples.push(format!("{key_text}	{name}	{a}	{b}")); } } } } } } let only_right = right_rows.iter().filter(|row| !seen.contains(&key_of(&right_header, row))).count(); for (name, count) in &counts { println!("column {name} differs={count}"); } if !samples.is_empty() { println!("key	column	left	right"); for sample in &samples { println!("{sample}"); } } println!("eoie table diff ok left_rows={} right_rows={} only_left={only_left} only_right={only_right} differing_cells={}", left_rows.len(), right_rows.len(), counts.values().sum::<usize>()); 0 })();
    v5
}
fn query_rows_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let mut v5: i32 = (|| -> i32 { let path = &*v0; let text = match std::fs::read_to_string(path) { Ok(text) => text, Err(error) => { eprintln!("eoie error: table {path}: {error}"); return 2; } }; let separator = if path.ends_with(".csv") { char::from(44u8) } else { char::from(9u8) }; let mut lines = text.lines(); let header = lines.next().unwrap_or("").split(separator).map(str::to_owned).collect::<Vec<_>>(); let column = |name: &str| header.iter().position(|item| item == name); let mut missing = Vec::new(); let mut filters = Vec::new(); for clause in (&*v1).lines() { if let Some((name, value)) = clause.split_once(char::from(61u8)) { match column(name) { Some(index) => filters.push((index, value.to_owned())), None => missing.push(name.to_owned()) } } } let projection = if (&*v2).is_empty() { (0..header.len()).collect::<Vec<_>>() } else { (&*v2).split(char::from(44u8)).filter_map(|name| match column(name) { Some(index) => Some(index), None => { missing.push(name.to_owned()); None } }).collect::<Vec<_>>() }; let grouping = if (&*v3).is_empty() { None } else { match column(&*v3) { Some(index) => Some(index), None => { missing.push((&*v3).to_owned()); None } } }; if !missing.is_empty() { eprintln!("eoie error: table unknown columns: {} (header: {})", missing.join(","), header.join(",")); return 2; } let mut total = 0usize; let mut matched = 0usize; let mut shown = 0usize; let mut groups: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new(); if grouping.is_none() { println!("{}", projection.iter().map(|index| header[*index].as_str()).collect::<Vec<_>>().join("	")); } for line in lines { total += 1; let cells = line.split(separator).collect::<Vec<_>>(); if !filters.iter().all(|(index, value)| cells.get(*index).map(|cell| cell == value).unwrap_or(false)) { continue; } matched += 1; match grouping { Some(index) => { *groups.entry(cells.get(index).unwrap_or(&"").to_string()).or_insert(0) += 1; } None => { if (shown as u64) < v4 { println!("{}", projection.iter().map(|index| *cells.get(*index).unwrap_or(&"")).collect::<Vec<_>>().join("	")); shown += 1; } } } } if grouping.is_some() { let mut counted = groups.into_iter().collect::<Vec<_>>(); counted.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0))); println!("count	{}", &*v3); for (value, count) in counted.iter().take(v4 as usize) { println!("{count}	{value}"); shown += 1; } println!("eoie table query ok total={total} matched={matched} groups={} shown={shown}", counted.len()); } else { println!("eoie table query ok total={total} matched={matched} shown={shown}"); } 0 })();
    v5
}
fn option_applied_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>) -> US1 {
    let mut v7: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where"); } LIT.with(|lit| lit.clone()) };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
        let mut v9: bool = v6.contains(&*v8);
        if v9 {
            let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = if v10 {
                v6.clone()
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v11, &*v6, &*v12, &*v12].concat());
                v13.clone()
            };
            US1::US1_StepTaken(v14.clone(), v1.clone(), v2.clone(), v3.clone(), v4)
        } else {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where expects col=value, got "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v6, &*v17, &*v17, &*v17].concat());
            US1::US1_StepRejected(v18.clone())
        }
    } else {
        let mut v21: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--cols"); } LIT.with(|lit| lit.clone()) };
        if v21 {
            US1::US1_StepTaken(v0.clone(), v6.clone(), v2.clone(), v3.clone(), v4)
        } else {
            let mut v23: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--group"); } LIT.with(|lit| lit.clone()) };
            if v23 {
                US1::US1_StepTaken(v0.clone(), v1.clone(), v6.clone(), v3.clone(), v4)
            } else {
                let mut v25: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--key"); } LIT.with(|lit| lit.clone()) };
                if v25 {
                    US1::US1_StepTaken(v0.clone(), v1.clone(), v2.clone(), v6.clone(), v4)
                } else {
                    let mut v27: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit"); } LIT.with(|lit| lit.clone()) };
                    if v27 {
                        let mut v28: u64 = v6.trim().parse::<u64>().unwrap_or(0);
                        let mut v29: bool = v28 == 0u64;
                        if v29 {
                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit expects a positive integer"); } LIT.with(|lit| lit.clone()) };
                            US1::US1_StepRejected(v30.clone())
                        } else {
                            US1::US1_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v28)
                        }
                    } else {
                        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v5, &*v35, &*v35, &*v35].concat());
                        US1::US1_StepRejected(v36.clone())
                    }
                }
            }
        }
    }
}
fn options_from_4(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64) -> US1 {
    loop {
        let mut v7: bool = v0 < v1;
        if v7 {
            let mut v8: i32 = v0.wrapping_add(1i32);
            let mut v9: bool = v8 < v1;
            if v9 {
                let mut v10: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
                let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v8 as usize).unwrap_or_default());
                let mut v12: US1 = option_applied_5(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v10.clone(), v11.clone());
                match &v12 {
                    US1::US1_StepRejected(v20) => {
                        let mut v20: Rc<str> = v20.clone();
                        return US1::US1_StepRejected(v20.clone());
                    }
                    US1::US1_StepTaken(v13, v14, v15, v16, v17) => {
                        let mut v13: Rc<str> = v13.clone();
                        let mut v14: Rc<str> = v14.clone();
                        let mut v15: Rc<str> = v15.clone();
                        let mut v16: Rc<str> = v16.clone();
                        let mut v17: u64 = *v17;
                        let mut v18: i32 = v0.wrapping_add(2i32);
                        (v0, v1, v2, v3, v4, v5, v6) = (v18, v1, v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17);
                        continue;
                    }
                }
            } else {
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) };
                let mut v25: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v25, &*v26, &*v27, &*v27].concat());
                return US1::US1_StepRejected(v28.clone());
            }
        } else {
            return US1::US1_StepTaken(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6);
        }
    }
}
fn option_applied_6(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>) -> US1 {
    let mut v4: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where"); } LIT.with(|lit| lit.clone()) };
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v3.contains(&*v5);
        if v6 {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = if v7 {
                v3.clone()
            } else {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v8, &*v3, &*v9, &*v9].concat());
                v10.clone()
            };
            US1::US1_StepTaken(v11.clone(), v0.clone(), v0.clone(), v0.clone(), v1)
        } else {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where expects col=value, got "); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v3, &*v14, &*v14, &*v14].concat());
            US1::US1_StepRejected(v15.clone())
        }
    } else {
        let mut v18: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--cols"); } LIT.with(|lit| lit.clone()) };
        if v18 {
            US1::US1_StepTaken(v0.clone(), v3.clone(), v0.clone(), v0.clone(), v1)
        } else {
            let mut v20: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--group"); } LIT.with(|lit| lit.clone()) };
            if v20 {
                US1::US1_StepTaken(v0.clone(), v0.clone(), v3.clone(), v0.clone(), v1)
            } else {
                let mut v22: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--key"); } LIT.with(|lit| lit.clone()) };
                if v22 {
                    US1::US1_StepTaken(v0.clone(), v0.clone(), v0.clone(), v3.clone(), v1)
                } else {
                    let mut v24: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit"); } LIT.with(|lit| lit.clone()) };
                    if v24 {
                        let mut v25: u64 = v3.trim().parse::<u64>().unwrap_or(0);
                        let mut v26: bool = v25 == 0u64;
                        if v26 {
                            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit expects a positive integer"); } LIT.with(|lit| lit.clone()) };
                            US1::US1_StepRejected(v27.clone())
                        } else {
                            US1::US1_StepTaken(v0.clone(), v0.clone(), v0.clone(), v0.clone(), v25)
                        }
                    } else {
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v2, &*v32, &*v32, &*v32].concat());
                        US1::US1_StepRejected(v33.clone())
                    }
                }
            }
        }
    }
}
fn options_from_3(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64) -> US1 {
    let mut v4: bool = v0 < v1;
    if v4 {
        let mut v5: i32 = v0.wrapping_add(1i32);
        let mut v6: bool = v5 < v1;
        if v6 {
            let mut v7: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v5 as usize).unwrap_or_default());
            let mut v9: US1 = option_applied_6(v2.clone(), v3, v7.clone(), v8.clone());
            match &v9 {
                US1::US1_StepRejected(v17) => {
                    let mut v17: Rc<str> = v17.clone();
                    US1::US1_StepRejected(v17.clone())
                }
                US1::US1_StepTaken(v10, v11, v12, v13, v14) => {
                    let mut v10: Rc<str> = v10.clone();
                    let mut v11: Rc<str> = v11.clone();
                    let mut v12: Rc<str> = v12.clone();
                    let mut v13: Rc<str> = v13.clone();
                    let mut v14: u64 = *v14;
                    let mut v15: i32 = v0.wrapping_add(2i32);
                    options_from_4(v15, v1, v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14)
                }
            }
        } else {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v22, &*v23, &*v24, &*v24].concat());
            US1::US1_StepRejected(v25.clone())
        }
    } else {
        US1::US1_StepTaken(v2.clone(), v2.clone(), v2.clone(), v2.clone(), v3)
    }
}
fn table_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
    let mut v8: US0 = if v2 {
        US0::US0_TableQuery
    } else {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            US0::US0_TableDiff
        } else {
            US0::US0_TableUnknown
        }
    };
    match &v8 {
        US0::US0_TableDiff => {
            let mut v34: i32 = std::env::args().count() as i32;
            let mut v35: bool = v34 < 5i32;
            if v35 {
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value]... [--cols a,b] [--group col] [--limit n] | table diff <left> <right> --key a,b [--cols x,y] [--limit n]"); } LIT.with(|lit| lit.clone()) };
                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v36, &*v38, &*v38, &*v38].concat());
                eprintln!("{}", &*v39);
                2i32
            } else {
                let mut v40: i32 = 5i32;
                let mut v41: i32 = std::env::args().count() as i32;
                let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v43: u64 = 20u64;
                let mut v44: US1 = options_from_3(v40, v41, v42.clone(), v43);
                match &v44 {
                    US1::US1_StepRejected(v55) => {
                        let mut v55: Rc<str> = v55.clone();
                        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
                        let mut v57: Rc<str> = std::rc::Rc::<str>::from([&*v56, &*v55, &*v42, &*v42, &*v42].concat());
                        eprintln!("{}", &*v57);
                        2i32
                    }
                    US1::US1_StepTaken(v45, v46, v47, v48, v49) => {
                        let mut v45: Rc<str> = v45.clone();
                        let mut v46: Rc<str> = v46.clone();
                        let mut v47: Rc<str> = v47.clone();
                        let mut v48: Rc<str> = v48.clone();
                        let mut v49: u64 = *v49;
                        let mut v50: i32 = 3i32;
                        let mut v51: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v50 as usize).unwrap_or_default());
                        let mut v52: i32 = 4i32;
                        let mut v53: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v52 as usize).unwrap_or_default());
                        diff_rows_1(v51.clone(), v53.clone(), v48.clone(), v46.clone(), v49)
                    }
                }
            }
        }
        US0::US0_TableQuery => {
            let mut v9: i32 = std::env::args().count() as i32;
            let mut v10: bool = v9 < 4i32;
            if v10 {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value]... [--cols a,b] [--group col] [--limit n] | table diff <left> <right> --key a,b [--cols x,y] [--limit n]"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v11, &*v13, &*v13, &*v13].concat());
                eprintln!("{}", &*v14);
                2i32
            } else {
                let mut v15: i32 = 4i32;
                let mut v16: i32 = std::env::args().count() as i32;
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v18: u64 = 20u64;
                let mut v19: US1 = options_from_3(v15, v16, v17.clone(), v18);
                match &v19 {
                    US1::US1_StepRejected(v28) => {
                        let mut v28: Rc<str> = v28.clone();
                        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
                        let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v28, &*v17, &*v17, &*v17].concat());
                        eprintln!("{}", &*v30);
                        2i32
                    }
                    US1::US1_StepTaken(v20, v21, v22, v23, v24) => {
                        let mut v20: Rc<str> = v20.clone();
                        let mut v21: Rc<str> = v21.clone();
                        let mut v22: Rc<str> = v22.clone();
                        let mut v23: Rc<str> = v23.clone();
                        let mut v24: u64 = *v24;
                        let mut v25: i32 = 3i32;
                        let mut v26: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v25 as usize).unwrap_or_default());
                        query_rows_2(v26.clone(), v20.clone(), v21.clone(), v22.clone(), v24)
                    }
                }
            }
        }
        US0::US0_TableUnknown => {
            let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value]... [--cols a,b] [--group col] [--limit n] | table diff <left> <right> --key a,b [--cols x,y] [--limit n]"); } LIT.with(|lit| lit.clone()) };
            let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table "); } LIT.with(|lit| lit.clone()) };
            let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v64: Rc<str> = std::rc::Rc::<str>::from([&*v62, &*v61, &*v63, &*v63, &*v63].concat());
            eprintln!("{}", &*v64);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        table_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_table_run() -> i32 {
    closure0()()
}
