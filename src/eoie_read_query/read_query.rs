#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Succeeded,
    US0_Failed,
}
#[derive(Clone)]
enum US1 {
    US1_ReadFiles,
    US1_CountLines,
    US1_SearchText,
    US1_ReadUnknown,
}
#[derive(Clone)]
enum US2 {
    US2_WholeFile,
    US2_FromTo(u64, u64),
    US2_OneLine(u64),
}
fn decimal_text_2(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn elapsed_text_4(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_6() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_5() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_6();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_3(mut v0: US0, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US0::US0_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = elapsed_text_4(v2);
    let mut v9: Rc<str> = clock_text_5();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn lines_receipt_1(mut v0: [u64; 4], mut v1: u64) -> i32 {
    let mut v2: u64 = 0u64;
    let mut v3: u64 = v0[v2 as usize];
    let mut v4: Rc<str> = decimal_text_2(v3);
    let mut v5: u64 = 1u64;
    let mut v6: u64 = v0[v5 as usize];
    let mut v7: Rc<str> = decimal_text_2(v6);
    let mut v8: u64 = 2u64;
    let mut v9: u64 = v0[v8 as usize];
    let mut v10: Rc<str> = decimal_text_2(v9);
    let mut v11: u64 = 3u64;
    let mut v12: u64 = v0[v11 as usize];
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  bytes "); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v7, &*v15, &*v10, &*v16].concat());
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v4, &*v17, &*v16, &*v16].concat());
    let mut v19: bool = v12 == 0u64;
    if v19 {
        let mut v20: US0 = US0::US0_Succeeded;
        receipt_line_3(v20.clone(), v18.clone(), v1);
        0i32
    } else {
        let mut v21: US0 = US0::US0_Failed;
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  unreadable "); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = decimal_text_2(v12);
        let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v22, &*v23, &*v16, &*v16].concat());
        receipt_line_3(v21.clone(), v24.clone(), v1);
        2i32
    }
}
fn read_receipt_7(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> i32 {
    let mut v4: Rc<str> = decimal_text_2(v0);
    let mut v5: Rc<str> = decimal_text_2(v2);
    let mut v6: bool = v1 == 0u64;
    if v6 {
        let mut v7: US0 = US0::US0_Succeeded;
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v4, &*v9, &*v5, &*v10].concat());
        receipt_line_3(v7.clone(), v11.clone(), v3);
        0i32
    } else {
        let mut v12: US0 = US0::US0_Failed;
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  unreadable "); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = decimal_text_2(v1);
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v5, &*v17, &*v17, &*v17].concat());
        let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v4, &*v14, &*v15, &*v18].concat());
        receipt_line_3(v12.clone(), v19.clone(), v3);
        2i32
    }
}
fn search_receipt_8(mut v0: [u64; 4], mut v1: u64) -> i32 {
    let mut v2: u64 = 0u64;
    let mut v3: u64 = v0[v2 as usize];
    let mut v4: Rc<str> = decimal_text_2(v3);
    let mut v5: u64 = 1u64;
    let mut v6: u64 = v0[v5 as usize];
    let mut v7: Rc<str> = decimal_text_2(v6);
    let mut v8: u64 = 2u64;
    let mut v9: u64 = v0[v8 as usize];
    let mut v10: u64 = 3u64;
    let mut v11: u64 = v0[v10 as usize];
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" searched  "); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched  matches "); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = decimal_text_2(v9);
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v16, &*v16, &*v16].concat());
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v4, &*v13, &*v7, &*v17].concat());
    let mut v19: US0 = US0::US0_Succeeded;
    let mut v20: bool = v11 < v9;
    let mut v24: Rc<str> = if v20 {
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  shown "); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = decimal_text_2(v11);
        let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v21, &*v22, &*v16, &*v16].concat());
        v23.clone()
    } else {
        v18.clone()
    };
    receipt_line_3(v19.clone(), v24.clone(), v1);
    0i32
}
fn now_ms_9() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn patterns_from_10(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
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
fn lines_counted_11(mut v0: Rc<str>) -> [u64; 4] {
    let mut v1: [u64; 4] = (|| -> [u64; 4] { let expand = |pattern: &str| -> Vec<(String, std::path::PathBuf)> { let split = pattern.rfind(|character: char| character == char::from(47u8) || character == char::from(92u8)); if !pattern.contains(char::from(42u8)) { return vec![(pattern.to_owned(), std::path::PathBuf::from(pattern))]; } let (folder_text, separator, mask) = match split { Some(at) => (&pattern[..at], &pattern[at..at + 1], &pattern[at + 1..]), None => (".", "", pattern) }; let (head, tail) = mask.split_once(char::from(42u8)).unwrap_or((mask, "")); let mut found = std::fs::read_dir(folder_text).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_file()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with(head) && name.ends_with(tail) && name.len() >= head.len() + tail.len()).collect::<Vec<_>>()).unwrap_or_default(); found.sort(); found.into_iter().map(|name| (if separator.is_empty() { name.clone() } else { format!("{folder_text}{separator}{name}") }, std::path::Path::new(folder_text).join(&name))).collect() }; let mut rows = Vec::new(); for pattern in (&*v0).lines() { for (shown, path) in expand(pattern) { rows.push((shown, std::fs::read(&path).map(|bytes| (bytes.len() as u64, String::from_utf8_lossy(&bytes).lines().count() as u64)).map_err(|error| error.to_string()))); } } let path_width = rows.iter().map(|row| row.0.len()).max().unwrap_or(0).max(4); let bytes_width = rows.iter().map(|row| row.1.as_ref().map(|counts| counts.0.to_string().len()).unwrap_or(1)).max().unwrap_or(0).max(5); println!("{:<path_width$}  {:<bytes_width$}  {}", "path", "bytes", "lines"); let mut tally = [0u64; 4]; for (shown, counts) in &rows { tally[0] += 1; match counts { Ok((bytes, lines)) => { tally[1] += bytes; tally[2] += lines; println!("{:<path_width$}  {:<bytes_width$}  {}", shown, bytes, lines); } Err(error) => { tally[3] += 1; println!("{:<path_width$}  {:<bytes_width$}  -  (unreadable: {error})", shown, "-"); } } } tally })();
    v1.clone()
}
fn max_lines_from_12(mut v0: i32, mut v1: i32) -> u64 {
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
fn read_spec_of_14(mut v0: Rc<str>) -> (Rc<str>, US2) {
    let mut v1: i64 = { let bytes = v0.as_bytes(); bytes.iter().rposition(|byte| *byte == 58u8).filter(|index| *index > 1 || !(bytes.len() > 1 && bytes[1] == 58u8)).map(|index| index as i64).unwrap_or(-1) };
    let mut v2: bool = v1 < 0i64;
    let (mut v36, mut v37): (Rc<str>, US2) = if v2 {
        let mut v3: US2 = US2::US2_WholeFile;
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
        let (mut v34, mut v35): (Rc<str>, US2) = if v12 {
            let mut v13: Rc<str> = std::rc::Rc::<str>::from(&(&*v0)[..v1 as usize]);
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v15: bool = v4.contains(&*v14);
            let mut v32: US2 = if v15 {
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
                    US2::US2_WholeFile
                } else {
                    US2::US2_FromTo(v18, v20)
                }
            } else {
                let mut v27: bool = !v4.is_empty() && v4.bytes().all(|byte| byte.is_ascii_digit());
                if v27 {
                    let mut v28: u64 = v4.trim().parse::<u64>().unwrap_or(0);
                    US2::US2_OneLine(v28)
                } else {
                    US2::US2_WholeFile
                }
            };
            (v13.clone(), v32.clone())
        } else {
            let mut v33: US2 = US2::US2_WholeFile;
            (v0.clone(), v33.clone())
        };
        (v34.clone(), v35.clone())
    };
    (v36.clone(), v37.clone())
}
fn file_slice_15(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> i64 {
    let mut v4: i64 = (|| -> i64 { let text = match std::fs::read(&*v0) { Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(), Err(error) => { println!("{} (unreadable: {error})", &*v0); return -1; } }; let lines = text.lines().collect::<Vec<_>>(); let total = lines.len() as u64; let first = (v1).max(1); let last = (v2).min(total); let shown_last = last.min(first.saturating_add(v3).saturating_sub(1)); println!("{} (lines {}-{} of {})", &*v0, first, shown_last.max(first.saturating_sub(1)), total); let width = shown_last.to_string().len(); let mut shown = 0i64; if first <= shown_last { for index in first..=shown_last { println!("{:>width$}  {}", index, lines[(index - 1) as usize]); shown += 1; } } if shown_last < last { println!("... {} more lines (raise --max-lines or pass a range)", last - shown_last); } shown })();
    v4
}
fn read_from_13(mut v0: i32, mut v1: i32, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64) -> (u64, u64, u64) {
    let mut v6: bool = v0 < v1;
    let (mut v44, mut v45, mut v46): (u64, u64, u64) = if v6 {
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-lines"); } LIT.with(|lit| lit.clone()) };
        let (mut v41, mut v42, mut v43): (u64, u64, u64) = if v8 {
            let mut v9: i32 = v0.wrapping_add(2i32);
            let (mut v10, mut v11, mut v12): (u64, u64, u64) = read_from_13(v9, v1, v2, v3, v4, v5);
            (v10, v11, v12)
        } else {
            let (mut v13, mut v14): (Rc<str>, US2) = read_spec_of_14(v7.clone());
            let mut v20: u64 = match &v14 {
                US2::US2_FromTo(v15, v16) => {
                    let mut v15: u64 = *v15;
                    let mut v16: u64 = *v16;
                    v15
                }
                US2::US2_OneLine(v17) => {
                    let mut v17: u64 = *v17;
                    v17
                }
                US2::US2_WholeFile => {
                    1u64
                }
            };
            let mut v21: u64 = 18446744073709551615u64;
            let mut v27: u64 = match &v14 {
                US2::US2_FromTo(v22, v23) => {
                    let mut v22: u64 = *v22;
                    let mut v23: u64 = *v23;
                    v23
                }
                US2::US2_OneLine(v24) => {
                    let mut v24: u64 = *v24;
                    v24
                }
                US2::US2_WholeFile => {
                    v21
                }
            };
            let mut v28: i64 = file_slice_15(v13.clone(), v20, v27, v2);
            let mut v29: u64 = v3.wrapping_add(1u64);
            let mut v30: bool = v28 < 0i64;
            let (mut v34, mut v35, mut v36): (u64, u64, u64) = if v30 {
                let mut v31: u64 = v4.wrapping_add(1u64);
                (v29, v31, v5)
            } else {
                let mut v32: u64 = v28 as u64;
                let mut v33: u64 = v5.wrapping_add(v32);
                (v29, v4, v33)
            };
            let mut v37: i32 = v0.wrapping_add(1i32);
            let (mut v38, mut v39, mut v40): (u64, u64, u64) = read_from_13(v37, v1, v2, v34, v35, v36);
            (v38, v39, v40)
        };
        (v41, v42, v43)
    } else {
        (v3, v4, v5)
    };
    (v44, v45, v46)
}
fn search_options_from_16(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: bool) -> (Rc<str>, u64, u64, bool) {
    let mut v6: bool = v0 < v1;
    let (mut v62, mut v63, mut v64, mut v65): (Rc<str>, u64, u64, bool) = if v6 {
        let mut v7: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ignore-case"); } LIT.with(|lit| lit.clone()) };
        let (mut v58, mut v59, mut v60, mut v61): (Rc<str>, u64, u64, bool) = if v8 {
            let mut v9: i32 = v0.wrapping_add(1i32);
            let mut v10: bool = true;
            let (mut v11, mut v12, mut v13, mut v14): (Rc<str>, u64, u64, bool) = search_options_from_16(v9, v1, v2.clone(), v3, v4, v10);
            (v11.clone(), v12, v13, v14)
        } else {
            let mut v15: i32 = v0.wrapping_add(1i32);
            let mut v16: bool = v15 < v1;
            let (mut v54, mut v55, mut v56, mut v57): (Rc<str>, u64, u64, bool) = if v16 {
                let mut v17: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v15 as usize).unwrap_or_default());
                let mut v18: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--glob"); } LIT.with(|lit| lit.clone()) };
                let (mut v50, mut v51, mut v52, mut v53): (Rc<str>, u64, u64, bool) = if v18 {
                    let mut v19: i32 = v0.wrapping_add(2i32);
                    let (mut v20, mut v21, mut v22, mut v23): (Rc<str>, u64, u64, bool) = search_options_from_16(v19, v1, v17.clone(), v3, v4, v5);
                    (v20.clone(), v21, v22, v23)
                } else {
                    let mut v24: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--context"); } LIT.with(|lit| lit.clone()) };
                    let (mut v46, mut v47, mut v48, mut v49): (Rc<str>, u64, u64, bool) = if v24 {
                        let mut v25: i32 = v0.wrapping_add(2i32);
                        let mut v26: u64 = v17.trim().parse::<u64>().unwrap_or(0);
                        let (mut v27, mut v28, mut v29, mut v30): (Rc<str>, u64, u64, bool) = search_options_from_16(v25, v1, v2.clone(), v26, v4, v5);
                        (v27.clone(), v28, v29, v30)
                    } else {
                        let mut v31: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max"); } LIT.with(|lit| lit.clone()) };
                        let (mut v42, mut v43, mut v44, mut v45): (Rc<str>, u64, u64, bool) = if v31 {
                            let mut v32: i32 = v0.wrapping_add(2i32);
                            let mut v33: u64 = v17.trim().parse::<u64>().unwrap_or(0);
                            let (mut v34, mut v35, mut v36, mut v37): (Rc<str>, u64, u64, bool) = search_options_from_16(v32, v1, v2.clone(), v3, v33, v5);
                            (v34.clone(), v35, v36, v37)
                        } else {
                            let (mut v38, mut v39, mut v40, mut v41): (Rc<str>, u64, u64, bool) = search_options_from_16(v15, v1, v2.clone(), v3, v4, v5);
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
fn text_searched_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: bool) -> [u64; 4] {
    let mut v6: [u64; 4] = (|| -> [u64; 4] { use rayon::prelude::*; let skip = ["target", "node_modules", ".git", "_build", "deps", "dist", ".cache", "obj"]; let root = std::path::PathBuf::from(&*v0); let mut stack = vec![root.clone()]; let mut files = Vec::new(); while let Some(folder) = stack.pop() { let Ok(entries) = std::fs::read_dir(&folder) else { continue }; for entry in entries.filter_map(|entry| entry.ok()) { let path = entry.path(); let Ok(kind) = entry.file_type() else { continue }; if kind.is_symlink() { continue; } if kind.is_dir() { if !skip.iter().any(|name| path.file_name().map(|value| value == *name).unwrap_or(false)) { stack.push(path); } } else if kind.is_file() { files.push(path); } } } let glob = &*v2; let (head, tail) = glob.split_once(char::from(42u8)).unwrap_or((glob, "")); let wanted = |path: &std::path::Path| glob.is_empty() || path.file_name().map(|name| { let name = name.to_string_lossy(); if glob.contains(char::from(42u8)) { name.starts_with(head) && name.ends_with(tail) } else { name == glob } }).unwrap_or(false); files.retain(|path| wanted(path)); let ignore_case = v5; let needle = if ignore_case { (&*v1).to_lowercase() } else { (&*v1).to_owned() }; let mut hits = files.par_iter().filter_map(|path| { let bytes = std::fs::read(path).ok()?; let text = String::from_utf8_lossy(&bytes); let lines = text.lines().map(str::to_owned).collect::<Vec<_>>(); let found = lines.iter().enumerate().filter(|(_, line)| if ignore_case { line.to_lowercase().contains(&needle) } else { line.contains(needle.as_str()) }).map(|(index, _)| index).collect::<Vec<_>>(); if found.is_empty() { return None; } Some((path.clone(), found, lines)) }).collect::<Vec<_>>(); hits.sort_by(|left, right| left.0.cmp(&right.0)); let shown_path = |path: &std::path::Path| path.strip_prefix(&root).unwrap_or(path).to_string_lossy().replace(char::from(92u8), "/"); let clip = |line: &str| if line.chars().count() > 200 { format!("{}...", line.chars().take(200).collect::<String>()) } else { line.to_owned() }; let context = v3 as usize; let mut shown = 0u64; let mut total = 0u64; for (path, found, lines) in &hits { total += found.len() as u64; if shown >= v4 { continue; } let taken = found.iter().copied().take((v4 - shown) as usize).collect::<Vec<_>>(); shown += taken.len() as u64; let last_line = taken.iter().map(|index| (index + context).min(lines.len() - 1)).max().unwrap_or(0); let width = (last_line + 1).to_string().len(); println!("{}", shown_path(path)); let mut printed_until: Option<usize> = None; for index in &taken { let start = match printed_until { Some(printed) if printed + 1 > index.saturating_sub(context) => printed + 1, _ => index.saturating_sub(context) }; let end = (index + context).min(lines.len() - 1); for line in start..=end { let marker = if taken.contains(&line) { char::from(62u8) } else { char::from(32u8) }; println!("{:>width$}{} {}", line + 1, marker, clip(&lines[line])); } printed_until = Some(printed_until.map_or(end, |printed| printed.max(end))); } } [files.len() as u64, hits.len() as u64, total, shown] })();
    v6.clone()
}
fn read_run_0() -> i32 {
    let mut v0: u64 = now_ms_9();
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read"); } LIT.with(|lit| lit.clone()) };
    let mut v12: US1 = if v3 {
        US1::US1_ReadFiles
    } else {
        let mut v5: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
        if v5 {
            US1::US1_CountLines
        } else {
            let mut v7: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("grep"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                US1::US1_SearchText
            } else {
                US1::US1_ReadUnknown
            }
        }
    };
    match &v12 {
        US1::US1_CountLines => {
            let mut v32: i32 = std::env::args().count() as i32;
            let mut v33: bool = v32 < 3i32;
            if v33 {
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path>[:from-to]... [--max-lines n] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case]"); } LIT.with(|lit| lit.clone()) };
                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v34, &*v36, &*v36, &*v36].concat());
                eprintln!("{}", &*v37);
                2i32
            } else {
                let mut v38: i32 = 2i32;
                let mut v39: i32 = std::env::args().count() as i32;
                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v41: Rc<str> = patterns_from_10(v38, v39, v40.clone());
                let mut v42: [u64; 4] = lines_counted_11(v41.clone());
                lines_receipt_1(v42.clone(), v0)
            }
        }
        US1::US1_ReadFiles => {
            let mut v13: i32 = std::env::args().count() as i32;
            let mut v14: bool = v13 < 3i32;
            if v14 {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path>[:from-to]... [--max-lines n] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case]"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v15, &*v17, &*v17, &*v17].concat());
                eprintln!("{}", &*v18);
                2i32
            } else {
                let mut v19: i32 = 2i32;
                let mut v20: i32 = std::env::args().count() as i32;
                let mut v21: i32 = 2i32;
                let mut v22: i32 = std::env::args().count() as i32;
                let mut v23: u64 = max_lines_from_12(v21, v22);
                let mut v24: u64 = 0u64;
                let mut v25: u64 = 0u64;
                let mut v26: u64 = 0u64;
                let (mut v27, mut v28, mut v29): (u64, u64, u64) = read_from_13(v19, v20, v23, v24, v25, v26);
                read_receipt_7(v27, v28, v29, v0)
            }
        }
        US1::US1_ReadUnknown => {
            let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path>[:from-to]... [--max-lines n] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case]"); } LIT.with(|lit| lit.clone()) };
            let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
            let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v71: Rc<str> = std::rc::Rc::<str>::from([&*v69, &*v68, &*v70, &*v70, &*v70].concat());
            eprintln!("{}", &*v71);
            2i32
        }
        US1::US1_SearchText => {
            let mut v45: i32 = std::env::args().count() as i32;
            let mut v46: bool = v45 < 4i32;
            if v46 {
                let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path>[:from-to]... [--max-lines n] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case]"); } LIT.with(|lit| lit.clone()) };
                let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: "); } LIT.with(|lit| lit.clone()) };
                let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v50: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v47, &*v49, &*v49, &*v49].concat());
                eprintln!("{}", &*v50);
                2i32
            } else {
                let mut v51: i32 = 4i32;
                let mut v52: i32 = std::env::args().count() as i32;
                let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v54: u64 = 0u64;
                let mut v55: u64 = 200u64;
                let mut v56: bool = false;
                let (mut v57, mut v58, mut v59, mut v60): (Rc<str>, u64, u64, bool) = search_options_from_16(v51, v52, v53.clone(), v54, v55, v56);
                let mut v61: i32 = 2i32;
                let mut v62: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v61 as usize).unwrap_or_default());
                let mut v63: i32 = 3i32;
                let mut v64: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v63 as usize).unwrap_or_default());
                let mut v65: [u64; 4] = text_searched_17(v62.clone(), v64.clone(), v57.clone(), v58, v59, v60);
                search_receipt_8(v65.clone(), v0)
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
