#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_WholeFile,
    US0_FromTo(u64, u64),
    US0_OneLine(u64),
}
#[derive(Clone)]
enum US1 {
    US1_ReadFiles,
    US1_CountLines,
    US1_SearchText,
    US1_ReadUnknown,
}
fn lines_counted_1(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = (|| -> i32 { let expand = |pattern: &str| -> Vec<std::path::PathBuf> { if !pattern.contains(char::from(42u8)) { return vec![std::path::PathBuf::from(pattern)]; } let path = std::path::Path::new(pattern); let folder = path.parent().filter(|parent| !parent.as_os_str().is_empty()).unwrap_or(std::path::Path::new(".")); let mask = path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(); let (head, tail) = mask.split_once(char::from(42u8)).unwrap_or((mask.as_str(), "")); let mut found = std::fs::read_dir(folder).map(|entries| entries.filter_map(|entry| entry.ok()).map(|entry| entry.path()).filter(|candidate| candidate.is_file() && candidate.file_name().map(|name| { let name = name.to_string_lossy(); name.starts_with(head) && name.ends_with(tail) && name.len() >= head.len() + tail.len() }).unwrap_or(false)).collect::<Vec<_>>()).unwrap_or_default(); found.sort(); found }; let mut total = 0usize; let mut files = 0usize; for pattern in (&*v0).lines() { for path in expand(pattern) { match std::fs::read(&path) { Ok(bytes) => { let count = String::from_utf8_lossy(&bytes).lines().count(); total += count; files += 1; println!("{:>8}  {}", count, path.display()); } Err(error) => println!("{:>8}  {} (unreadable: {error})", "-", path.display()) } } } println!("eoie lines ok files={files} total={total}"); 0 })();
    v1
}
fn read_spec_of_3(mut v0: Rc<str>) -> (Rc<str>, US0) {
    let mut v1: i64 = { let bytes = v0.as_bytes(); bytes.iter().rposition(|byte| *byte == 58u8).filter(|index| *index > 1 || !(bytes.len() > 1 && bytes[1] == 58u8)).map(|index| index as i64).unwrap_or(-1) };
    let mut v2: bool = v1 < 0i64;
    let (mut v36, mut v37): (Rc<str>, US0) = if v2 {
        let mut v3: US0 = US0::US0_WholeFile;
        (v0.clone(), v3.clone())
    } else {
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(&(&*v0)[(v1 as usize) + 1..]);
        let mut v5: bool = !v4.is_empty() && v4.bytes().all(|byte| byte.is_ascii_digit());
        let mut v12: bool = if v5 {
            true
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v4.contains(&*v6);
            if v7 {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v6, &v8));
                let mut v10: bool = !v9.is_empty() && v9.bytes().all(|byte| byte.is_ascii_digit());
                v10
            } else {
                false
            }
        };
        let (mut v34, mut v35): (Rc<str>, US0) = if v12 {
            let mut v13: Rc<str> = std::rc::Rc::<str>::from(&(&*v0)[..v1 as usize]);
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v15: bool = v4.contains(&*v14);
            let mut v32: US0 = if v15 {
                let mut v16: i64 = v4.find(char::from(45u8)).map(|index| index as i64).unwrap_or(-1);
                let mut v17: Rc<str> = std::rc::Rc::<str>::from(&(&*v4)[..v16 as usize]);
                let mut v18: u64 = v17.trim().parse::<u64>().unwrap_or(0);
                let mut v19: Rc<str> = std::rc::Rc::<str>::from(&(&*v4)[(v16 as usize) + 1..]);
                let mut v20: u64 = v19.trim().parse::<u64>().unwrap_or(0);
                let mut v21: bool = v18 == 0u64;
                let mut v23: bool = if v21 {
                    true
                } else {
                    let mut v22: bool = v20 < v18;
                    v22
                };
                if v23 {
                    US0::US0_WholeFile
                } else {
                    US0::US0_FromTo(v18, v20)
                }
            } else {
                let mut v27: bool = !v4.is_empty() && v4.bytes().all(|byte| byte.is_ascii_digit());
                if v27 {
                    let mut v28: u64 = v4.trim().parse::<u64>().unwrap_or(0);
                    US0::US0_OneLine(v28)
                } else {
                    US0::US0_WholeFile
                }
            };
            (v13.clone(), v32.clone())
        } else {
            let mut v33: US0 = US0::US0_WholeFile;
            (v0.clone(), v33.clone())
        };
        (v34.clone(), v35.clone())
    };
    (v36.clone(), v37.clone())
}
fn file_slice_4(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> i32 {
    let mut v4: i32 = (|| -> i32 { let text = match std::fs::read(&*v0) { Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(), Err(error) => { println!("== {} (unreadable: {error})", &*v0); return 2; } }; let lines = text.lines().collect::<Vec<_>>(); let total = lines.len() as u64; let first = (v1).max(1); let last = (v2).min(total); let shown_last = last.min(first.saturating_add(v3).saturating_sub(1)); println!("== {} (lines {}-{} of {})", &*v0, first, shown_last.max(first.saturating_sub(1)), total); if first <= shown_last { for index in first..=shown_last { println!("{:>6}  {}", index, lines[(index - 1) as usize]); } } if shown_last < last { println!("   ... {} more lines (raise --max-lines or pass a range)", last - shown_last); } 0 })();
    v4
}
fn read_from_2(mut v0: i32, mut v1: i32, mut v2: u64, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v0 < v1;
        if v4 {
            let mut v5: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
            let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-lines"); } LIT.with(|lit| lit.clone()) };
            if v6 {
                let mut v7: i32 = v0.wrapping_add(2i32);
                (v0, v1, v2, v3) = (v7, v1, v2, v3);
                continue;
            } else {
                let (mut v9, mut v10): (Rc<str>, US0) = read_spec_of_3(v5.clone());
                let mut v16: u64 = match &v10 {
                    US0::US0_FromTo(v11, v12) => {
                        let mut v11: u64 = *v11;
                        let mut v12: u64 = *v12;
                        v11
                    }
                    US0::US0_OneLine(v13) => {
                        let mut v13: u64 = *v13;
                        v13
                    }
                    US0::US0_WholeFile => {
                        1u64
                    }
                };
                let mut v17: u64 = 18446744073709551615u64;
                let mut v23: u64 = match &v10 {
                    US0::US0_FromTo(v18, v19) => {
                        let mut v18: u64 = *v18;
                        let mut v19: u64 = *v19;
                        v19
                    }
                    US0::US0_OneLine(v20) => {
                        let mut v20: u64 = *v20;
                        v20
                    }
                    US0::US0_WholeFile => {
                        v17
                    }
                };
                let mut v24: i32 = file_slice_4(v9.clone(), v16, v23, v2);
                let mut v25: i32 = v0.wrapping_add(1i32);
                let mut v26: bool = v24 > v3;
                let mut v27: i32 = if v26 {
                    v24
                } else {
                    v3
                };
                (v0, v1, v2, v3) = (v25, v1, v2, v27);
                continue;
            }
        } else {
            return v3;
        }
    }
}
fn text_searched_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: bool) -> i32 {
    let mut v6: i32 = (|| -> i32 { use rayon::prelude::*; let skip = ["target", "node_modules", ".git", "_build", "deps", "dist", ".cache", "obj"]; let mut stack = vec![std::path::PathBuf::from(&*v0)]; let mut files = Vec::new(); while let Some(folder) = stack.pop() { let Ok(entries) = std::fs::read_dir(&folder) else { continue }; for entry in entries.filter_map(|entry| entry.ok()) { let path = entry.path(); let Ok(kind) = entry.file_type() else { continue }; if kind.is_symlink() { continue; } if kind.is_dir() { if !skip.iter().any(|name| path.file_name().map(|value| value == *name).unwrap_or(false)) { stack.push(path); } } else if kind.is_file() { files.push(path); } } } let glob = &*v2; let (head, tail) = glob.split_once(char::from(42u8)).unwrap_or((glob, "")); let wanted = |path: &std::path::Path| glob.is_empty() || path.file_name().map(|name| { let name = name.to_string_lossy(); if glob.contains(char::from(42u8)) { name.starts_with(head) && name.ends_with(tail) } else { name == glob } }).unwrap_or(false); let ignore_case = v5; let needle = if ignore_case { (&*v1).to_lowercase() } else { (&*v1).to_owned() }; let mut hits = files.par_iter().filter(|path| wanted(path)).filter_map(|path| { let bytes = std::fs::read(path).ok()?; let text = String::from_utf8_lossy(&bytes); let lines = text.lines().collect::<Vec<_>>(); let found = lines.iter().enumerate().filter(|(_, line)| if ignore_case { line.to_lowercase().contains(&needle) } else { line.contains(needle.as_str()) }).map(|(index, _)| index).collect::<Vec<_>>(); if found.is_empty() { return None; } Some((path.clone(), found, lines.iter().map(|line| line.to_string()).collect::<Vec<_>>())) }).collect::<Vec<_>>(); hits.sort_by(|left, right| left.0.cmp(&right.0)); let clip = |line: &str| if line.chars().count() > 200 { format!("{}...", line.chars().take(200).collect::<String>()) } else { line.to_owned() }; let mut shown = 0u64; let mut total = 0usize; for (path, found, lines) in &hits { total += found.len(); for index in found { if shown >= v4 { continue; } let first = index.saturating_sub(v3 as usize); let last = (index + v3 as usize).min(lines.len() - 1); for line in first..=last { let marker = if line == *index { char::from(58u8) } else { char::from(45u8) }; println!("{}{}{}{} {}", path.display(), marker, line + 1, marker, clip(&lines[line])); } shown += 1; } } println!("eoie grep ok files_scanned={} files_matched={} hits={total} shown={shown}", files.len(), hits.len()); 0 })();
    v6
}
fn patterns_from_6(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0 < v1;
        if v3 {
            let mut v4: i32 = v0.wrapping_add(1i32);
            let mut v5: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = if v5 {
                let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
                v6.clone()
            } else {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v7, &*v8, &*v9, &*v9].concat());
                v10.clone()
            };
            (v0, v1, v2) = (v4, v1, v11.clone());
            continue;
        } else {
            return v2.clone();
        }
    }
}
fn max_lines_from_7(mut v0: i32, mut v1: i32) -> u64 {
    loop {
        let mut v2: i32 = v0.wrapping_add(1i32);
        let mut v3: bool = v2 < v1;
        if v3 {
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
            let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-lines"); } LIT.with(|lit| lit.clone()) };
            if v5 {
                let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
                let mut v7: u64 = v6.trim().parse::<u64>().unwrap_or(0);
                return v7;
            } else {
                (v0, v1) = (v2, v1);
                continue;
            }
        } else {
            return 400u64;
        }
    }
}
fn search_options_from_8(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: bool) -> (Rc<str>, u64, u64, bool) {
    let mut v6: bool = v0 < v1;
    let (mut v62, mut v63, mut v64, mut v65): (Rc<str>, u64, u64, bool) = if v6 {
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ignore-case"); } LIT.with(|lit| lit.clone()) };
        let (mut v58, mut v59, mut v60, mut v61): (Rc<str>, u64, u64, bool) = if v8 {
            let mut v9: i32 = v0.wrapping_add(1i32);
            let mut v10: bool = true;
            let (mut v11, mut v12, mut v13, mut v14): (Rc<str>, u64, u64, bool) = search_options_from_8(v9, v1, v2.clone(), v3, v4, v10);
            (v11.clone(), v12, v13, v14)
        } else {
            let mut v15: i32 = v0.wrapping_add(1i32);
            let mut v16: bool = v15 < v1;
            let (mut v54, mut v55, mut v56, mut v57): (Rc<str>, u64, u64, bool) = if v16 {
                let mut v17: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v15 as usize).unwrap_or_default());
                let mut v18: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--glob"); } LIT.with(|lit| lit.clone()) };
                let (mut v50, mut v51, mut v52, mut v53): (Rc<str>, u64, u64, bool) = if v18 {
                    let mut v19: i32 = v0.wrapping_add(2i32);
                    let (mut v20, mut v21, mut v22, mut v23): (Rc<str>, u64, u64, bool) = search_options_from_8(v19, v1, v17.clone(), v3, v4, v5);
                    (v20.clone(), v21, v22, v23)
                } else {
                    let mut v24: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--context"); } LIT.with(|lit| lit.clone()) };
                    let (mut v46, mut v47, mut v48, mut v49): (Rc<str>, u64, u64, bool) = if v24 {
                        let mut v25: i32 = v0.wrapping_add(2i32);
                        let mut v26: u64 = v17.trim().parse::<u64>().unwrap_or(0);
                        let (mut v27, mut v28, mut v29, mut v30): (Rc<str>, u64, u64, bool) = search_options_from_8(v25, v1, v2.clone(), v26, v4, v5);
                        (v27.clone(), v28, v29, v30)
                    } else {
                        let mut v31: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max"); } LIT.with(|lit| lit.clone()) };
                        let (mut v42, mut v43, mut v44, mut v45): (Rc<str>, u64, u64, bool) = if v31 {
                            let mut v32: i32 = v0.wrapping_add(2i32);
                            let mut v33: u64 = v17.trim().parse::<u64>().unwrap_or(0);
                            let (mut v34, mut v35, mut v36, mut v37): (Rc<str>, u64, u64, bool) = search_options_from_8(v32, v1, v2.clone(), v3, v33, v5);
                            (v34.clone(), v35, v36, v37)
                        } else {
                            let (mut v38, mut v39, mut v40, mut v41): (Rc<str>, u64, u64, bool) = search_options_from_8(v15, v1, v2.clone(), v3, v4, v5);
                            (v38.clone(), v39, v40, v41)
                        };
                        (v42.clone(), v43, v44, v45)
                    };
                    (v46.clone(), v47, v48, v49)
                };
                (v50.clone(), v51, v52, v53)
            } else {
                (v2.clone(), v3, v4, v5)
            };
            (v54.clone(), v55, v56, v57)
        };
        (v58.clone(), v59, v60, v61)
    } else {
        (v2.clone(), v3, v4, v5)
    };
    (v62.clone(), v63, v64, v65)
}
fn read_run_0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read"); } LIT.with(|lit| lit.clone()) };
    let mut v11: US1 = if v2 {
        US1::US1_ReadFiles
    } else {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            US1::US1_CountLines
        } else {
            let mut v6: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("grep"); } LIT.with(|lit| lit.clone()) };
            if v6 {
                US1::US1_SearchText
            } else {
                US1::US1_ReadUnknown
            }
        }
    };
    match &v11 {
        US1::US1_CountLines => {
            let mut v26: i32 = std::env::args().count() as i32;
            let mut v27: bool = v26 < 3i32;
            if v27 {
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path>[:from-to]... [--max-lines n] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case]"); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v28, &*v30, &*v30, &*v30].concat());
                eprintln!("{}", &*v31);
                2i32
            } else {
                let mut v32: i32 = 2i32;
                let mut v33: i32 = std::env::args().count() as i32;
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v35: Rc<str> = patterns_from_6(v32, v33, v34.clone());
                lines_counted_1(v35.clone())
            }
        }
        US1::US1_ReadFiles => {
            let mut v12: i32 = std::env::args().count() as i32;
            let mut v13: bool = v12 < 3i32;
            if v13 {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path>[:from-to]... [--max-lines n] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case]"); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v14, &*v16, &*v16, &*v16].concat());
                eprintln!("{}", &*v17);
                2i32
            } else {
                let mut v18: i32 = 2i32;
                let mut v19: i32 = std::env::args().count() as i32;
                let mut v20: i32 = 2i32;
                let mut v21: i32 = std::env::args().count() as i32;
                let mut v22: u64 = max_lines_from_7(v20, v21);
                let mut v23: i32 = 0i32;
                read_from_2(v18, v19, v22, v23)
            }
        }
        US1::US1_ReadUnknown => {
            let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path>[:from-to]... [--max-lines n] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case]"); } LIT.with(|lit| lit.clone()) };
            let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
            let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v63: Rc<str> = std::rc::Rc::<str>::from([&*v61, &*v60, &*v62, &*v62, &*v62].concat());
            eprintln!("{}", &*v63);
            2i32
        }
        US1::US1_SearchText => {
            let mut v38: i32 = std::env::args().count() as i32;
            let mut v39: bool = v38 < 4i32;
            if v39 {
                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path>[:from-to]... [--max-lines n] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case]"); } LIT.with(|lit| lit.clone()) };
                let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
                let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v43: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v40, &*v42, &*v42, &*v42].concat());
                eprintln!("{}", &*v43);
                2i32
            } else {
                let mut v44: i32 = 4i32;
                let mut v45: i32 = std::env::args().count() as i32;
                let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v47: u64 = 0u64;
                let mut v48: u64 = 200u64;
                let mut v49: bool = false;
                let (mut v50, mut v51, mut v52, mut v53): (Rc<str>, u64, u64, bool) = search_options_from_8(v44, v45, v46.clone(), v47, v48, v49);
                let mut v54: i32 = 2i32;
                let mut v55: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v54 as usize).unwrap_or_default());
                let mut v56: i32 = 3i32;
                let mut v57: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v56 as usize).unwrap_or_default());
                text_searched_5(v55.clone(), v57.clone(), v50.clone(), v51, v52, v53)
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        read_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_read_run() -> i32 {
    closure0()()
}
