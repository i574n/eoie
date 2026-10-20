#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_AppendLine(Rc<str>, Rc<str>),
    US1_CopyTree(Rc<str>, Rc<str>, Rc<str>),
    US1_ReadAfter,
    US1_Blank,
}
#[derive(Clone)]
enum US0 {
    US0_WriteParsed(US1),
    US0_WriteRejected(Rc<str>),
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
fn body_at_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3.clone()
}
fn quoted_at_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> bool {
    let mut v3: bool = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).map(|field| field.starts_with(char::from(113u8))).unwrap_or(false);
    v3
}
fn append_parsed_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 2i32;
    let mut v3: Rc<str> = body_at_6(v0.clone(), v1, v2);
    let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: bool = if v4 {
        true
    } else {
        let mut v5: i32 = 2i32;
        quoted_at_7(v0.clone(), v1, v5)
    };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append: expected a file, then the quoted line"); } LIT.with(|lit| lit.clone()) };
        US0::US0_WriteRejected(v8.clone())
    } else {
        let mut v10: i32 = 3i32;
        let mut v11: bool = quoted_at_7(v0.clone(), v1, v10);
        let mut v12: bool = v11 == false;
        if v12 {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append: expected the quoted line after the file"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v13.clone())
        } else {
            let mut v15: i32 = 3i32;
            let mut v16: Rc<str> = body_at_6(v0.clone(), v1, v15);
            let mut v17: US1 = US1::US1_AppendLine(v3.clone(), v16.clone());
            US0::US0_WriteParsed(v17.clone())
        }
    }
}
fn field_index_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn items_span_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize).map(|fields| fields.iter().skip(v2 as usize).filter(|raw| raw.len() > 1).flat_map(|raw| raw[1..].split(|c: char| c == char::from(44u8) || c.is_whitespace()).filter(|item| !item.is_empty()).map(str::to_owned).collect::<Vec<_>>()).collect::<Vec<_>>().join("
")).unwrap_or_default());
    v3.clone()
}
fn copy_parsed_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 2i32;
    let mut v3: Rc<str> = body_at_6(v0.clone(), v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = field_index_9(v0.clone(), v1, v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except"); } LIT.with(|lit| lit.clone()) };
    let mut v7: i32 = field_index_9(v0.clone(), v1, v6.clone());
    let mut v8: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = if v8 {
        true
    } else {
        let mut v9: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to"); } LIT.with(|lit| lit.clone()) };
        v9
    };
    if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy: expected copy <glob> to <folder> [except <glob,...>]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_WriteRejected(v11.clone())
    } else {
        let mut v13: bool = v5 < 0i32;
        if v13 {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy: expected to <folder>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v14.clone())
        } else {
            let mut v16: i32 = v5.wrapping_add(1i32);
            let mut v17: Rc<str> = body_at_6(v0.clone(), v1, v16);
            let mut v18: bool = v7 < 0i32;
            let mut v22: Rc<str> = if v18 {
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v19.clone()
            } else {
                let mut v20: i32 = v7.wrapping_add(1i32);
                items_span_10(v0.clone(), v1, v20)
            };
            let mut v23: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v25: bool = if v23 {
                true
            } else {
                let mut v24: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except"); } LIT.with(|lit| lit.clone()) };
                v24
            };
            if v25 {
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to: expected a folder after it"); } LIT.with(|lit| lit.clone()) };
                US0::US0_WriteRejected(v26.clone())
            } else {
                let mut v28: bool = v7 > 0i32;
                let mut v30: bool = if v28 {
                    let mut v29: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v29
                } else {
                    false
                };
                if v30 {
                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except: expected a glob after it"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_WriteRejected(v31.clone())
                } else {
                    let mut v33: US1 = US1::US1_CopyTree(v3.clone(), v17.clone(), v22.clone());
                    US0::US0_WriteParsed(v33.clone())
                }
            }
        }
    }
}
fn field_at_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn write_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 0i32;
    let mut v3: Rc<str> = field_at_11(v0.clone(), v1, v2);
    let mut v4: i32 = 1i32;
    let mut v5: Rc<str> = field_at_11(v0.clone(), v1, v4);
    let mut v6: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: US1 = US1::US1_Blank;
        US0::US0_WriteParsed(v7.clone())
    } else {
        let mut v9: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
        if v9 {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit and write statements cannot share one program yet; run them as two calls"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v10.clone())
        } else {
            let mut v12: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append"); } LIT.with(|lit| lit.clone()) };
            if v12 {
                append_parsed_5(v0.clone(), v1)
            } else {
                let mut v14: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy"); } LIT.with(|lit| lit.clone()) };
                if v14 {
                    copy_parsed_8(v0.clone(), v1)
                } else {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search|read|files|head|tail|wait"); } LIT.with(|lit| lit.clone()) };
                    let mut v17: bool = v16.split("|").any(|item| item == &*v5);
                    if v17 {
                        let mut v18: US1 = US1::US1_ReadAfter;
                        US0::US0_WriteParsed(v18.clone())
                    } else {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with append or copy yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
                        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v20, &*v21, &*v21, &*v21].concat());
                        US0::US0_WriteRejected(v22.clone())
                    }
                }
            }
        }
    }
}
fn read_rejection_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = eoie_do_read::eoie_do_read_rejection(&*v0);
    v1.clone()
}
fn record_alone_14(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(format!("ok{}{}", char::from(30u8), v0.get(v1 as usize).map(|fields| fields.iter().map(|field| field.to_string()).collect::<Vec<_>>().join(&char::from(31u8).to_string())).unwrap_or_default()));
    v2.clone()
}
fn read_rejected_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: US1) -> Rc<str> {
    match &v2 {
        US1::US1_ReadAfter => {
            let mut v3: Rc<str> = record_alone_14(v0.clone(), v1);
            read_rejection_13(v3.clone())
        }
        _ => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
    }
}
fn first_rejection_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: US0 = write_parsed_4(v0.clone(), v1);
            match &v4 {
                US0::US0_WriteParsed(v6) => {
                    let mut v6: US1 = v6.clone();
                    let mut v7: Rc<str> = read_rejected_12(v0.clone(), v1, v6.clone());
                    let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v8 {
                        let mut v9: i32 = v1.wrapping_add(1i32);
                        (v0, v1) = (v0.clone(), v9);
                        continue;
                    } else {
                        return v7.clone();
                    }
                }
                US0::US0_WriteRejected(v5) => {
                    let mut v5: Rc<str> = v5.clone();
                    return v5.clone();
                }
            }
        } else {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v14.clone();
        }
    }
}
fn empty_write_tally_15() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn file_text_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(std::path::Path::new(&*v0).join(&*v1)).unwrap_or_default());
    v2.clone()
}
fn uses_crlf_20(mut v0: Rc<str>) -> bool {
    let mut v1: bool = (v0).contains(&format!("{}{}", char::from(13u8), char::from(10u8)));
    v1
}
fn crlf_21() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(format!("{}{}", char::from(13u8), char::from(10u8)));
    v0.clone()
}
fn local_offset_minutes_22() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_filled_23(mut v0: Rc<str>, mut v1: i64) -> Rc<str> {
    let mut v2: Rc<str> = { let seconds = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs() as i64).unwrap_or(0) + v1 * 60; let (days, day_seconds) = (seconds.div_euclid(86400), seconds.rem_euclid(86400)); let shifted = days + 719468; let era = shifted.div_euclid(146097); let of_era = shifted - era * 146097; let year_of_era = (of_era - of_era / 1460 + of_era / 36524 - of_era / 146096) / 365; let of_year = of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100); let month_index = (5 * of_year + 2) / 153; let day = of_year - (153 * month_index + 2) / 5 + 1; let month = if month_index < 10 { month_index + 3 } else { month_index - 9 }; let year = year_of_era + era * 400 + if month <= 2 { 1 } else { 0 }; let formatted = |format: &str| format.replace("yyyy", &format!("{year:04}")).replace("MM", &format!("{month:02}")).replace("dd", &format!("{day:02}")).replace("HH", &format!("{:02}", day_seconds / 3600)).replace("mm", &format!("{:02}", day_seconds / 60 % 60)).replace("ss", &format!("{:02}", day_seconds % 60)); let pattern = regex::Regex::new("[{]clock(?::([^}]*))?[}]").unwrap(); std::rc::Rc::<str>::from(pattern.replace_all(&*v0, |captures: &regex::Captures| formatted(captures.get(1).map(|format| format.as_str()).unwrap_or("HH:mm:ss"))).into_owned()) };
    v2.clone()
}
fn ended_with_newline_24(mut v0: Rc<str>) -> bool {
    let mut v1: bool = (v0).is_empty() || (v0).ends_with(char::from(10u8));
    v1
}
fn appended_to_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = { use std::io::Write; std::rc::Rc::<str>::from(std::fs::OpenOptions::new().create(true).append(true).open(std::path::Path::new(&*v0).join(&*v1)).and_then(|mut file| file.write_all((v2).as_bytes())).err().map(|error| format!("append {}: {error}", &*v1)).unwrap_or_default()) };
    v3.clone()
}
fn text_bytes_26(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).len() as u64;
    v1
}
fn line_count_27(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).lines().count() as u64;
    v1
}
fn decimal_text_29(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn counted_28(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = decimal_text_29(v0);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v0 == 1u64;
    let mut v6: Rc<str> = if v5 {
        v1.clone()
    } else {
        v2.clone()
    };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v6, &*v7, &*v7].concat());
    v8.clone()
}
fn lines_joined_30(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
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
fn append_run_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let mut v12: Rc<str> = file_text_19(v0.clone(), v1.clone());
    let mut v13: bool = uses_crlf_20(v12.clone());
    let mut v16: Rc<str> = if v13 {
        crlf_21()
    } else {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        v15.clone()
    };
    let mut v17: i64 = local_offset_minutes_22();
    let mut v18: Rc<str> = clock_filled_23(v2.clone(), v17);
    let mut v19: bool = ended_with_newline_24(v12.clone());
    let mut v21: Rc<str> = if v19 {
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v20.clone()
    } else {
        v16.clone()
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v18, &*v16, &*v22, &*v22].concat());
    let mut v24: Rc<str> = appended_to_25(v0.clone(), v1.clone(), v23.clone());
    let mut v25: bool = v24.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v25 {
        let mut v26: Rc<str> = file_text_19(v0.clone(), v1.clone());
        let mut v27: u64 = text_bytes_26(v23.clone());
        let mut v28: u64 = line_count_27(v18.clone());
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  now "); } LIT.with(|lit| lit.clone()) };
        let mut v30: u64 = line_count_27(v26.clone());
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line"); } LIT.with(|lit| lit.clone()) };
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = counted_28(v30, v31.clone(), v32.clone());
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let mut v35: u64 = text_bytes_26(v26.clone());
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
        let mut v38: Rc<str> = counted_28(v35, v36.clone(), v37.clone());
        let mut v39: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v33, &*v34, &*v38, &*v22].concat());
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append "); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v42: Rc<str> = counted_28(v28, v31.clone(), v32.clone());
        let mut v43: Rc<str> = counted_28(v27, v36.clone(), v37.clone());
        let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v43, &*v39, &*v22, &*v22].concat());
        let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v1, &*v41, &*v42, &*v44].concat());
        let mut v46: Rc<str> = lines_joined_30(v3.clone(), v45.clone());
        let mut v47: u64 = v5.wrapping_add(1u64);
        let mut v48: u64 = v6.wrapping_add(v28);
        let mut v49: u64 = v9.wrapping_add(v27);
        let mut v50: u64 = v10.wrapping_add(1u64);
        (v46.clone(), v4, v47, v48, v7, v8, v49, v50, v11)
    } else {
        let mut v51: Rc<str> = lines_joined_30(v3.clone(), v24.clone());
        let mut v52: u64 = v10.wrapping_add(1u64);
        (v51.clone(), true, v5, v6, v7, v8, v9, v52, v11)
    };
    (v53.clone(), v54, v55, v56, v57, v58, v59, v60, v61)
}
fn glob_base_32(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).replace(char::from(92u8), "/").split(char::from(47u8)).take_while(|part| !part.contains(char::from(42u8)) && !part.contains(char::from(63u8))).collect::<Vec<_>>().join("/"));
    v1.clone()
}
fn matching_33(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = eoie_do_read::eoie_do_read_matching(&*v0, &*v1);
    v2.clone()
}
fn line_at_35(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn excluded_by_36(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (v0).lines().any(|pattern| { let name = (v1).rsplit(char::from(47u8)).next().unwrap_or(""); let mut translated = String::from("^"); for c in pattern.chars() { if c == char::from(42u8) { translated.push_str("[^/]*") } else if c == char::from(63u8) { translated.push_str("[^/]") } else { translated.push_str(&regex::escape(&c.to_string())) } } let translated = format!("{}$", translated.replace("[^/]*[^/]*", ".*")); regex::Regex::new(&translated).map(|regex| regex.is_match(name) || regex.is_match(&*v1)).unwrap_or(false) });
    v2
}
fn relative_under_37(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(if (v0).is_empty() { &*v1 } else { (v1).strip_prefix(&*v0).map(|rest| rest.trim_start_matches(char::from(47u8))).unwrap_or(&*v1) });
    v2.clone()
}
fn copied_bytes_38(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i64 {
    let mut v3: i64 = { let root = std::path::Path::new(&*v0); let target = root.join(&*v2); target.parent().map(std::fs::create_dir_all).unwrap_or(Ok(())).and_then(|_| std::fs::copy(root.join(&*v1), &target)).map(|bytes| bytes as i64).unwrap_or(-1) };
    v3
}
fn copies_run_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: bool, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let mut v15: u64 = line_count_27(v4.clone());
    let mut v16: bool = v5 < v15;
    let (mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v16 {
        let mut v17: Rc<str> = line_at_35(v4.clone(), v5);
        let mut v18: bool = excluded_by_36(v3.clone(), v17.clone());
        let (mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v18 {
            let mut v19: u64 = v11.wrapping_add(1u64);
            (v6.clone(), v7, v8, v9, v10, v19, v12, v13, v14)
        } else {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = relative_under_37(v1.clone(), v17.clone());
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v20, &*v21, &*v22, &*v22].concat());
            let mut v24: i64 = copied_bytes_38(v0.clone(), v17.clone(), v23.clone());
            let mut v25: bool = v24 < 0i64;
            let (mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v25 {
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy "); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed"); } LIT.with(|lit| lit.clone()) };
                let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v17, &*v27, &*v22, &*v22].concat());
                let mut v29: Rc<str> = lines_joined_30(v6.clone(), v28.clone());
                (v29.clone(), true, v8, v9, v10, v11, v12, v13, v14)
            } else {
                let mut v30: u64 = v10.wrapping_add(1u64);
                let mut v31: u64 = v24 as u64;
                let mut v32: u64 = v12.wrapping_add(v31);
                (v6.clone(), v7, v8, v9, v30, v11, v32, v13, v14)
            };
            (v33.clone(), v34, v35, v36, v37, v38, v39, v40, v41)
        };
        let mut v51: u64 = v5.wrapping_add(1u64);
        let (mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = copies_run_34(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v51, v42.clone(), v43, v44, v45, v46, v47, v48, v49, v50);
        (v52.clone(), v53, v54, v55, v56, v57, v58, v59, v60)
    } else {
        (v6.clone(), v7, v8, v9, v10, v11, v12, v13, v14)
    };
    (v61.clone(), v62, v63, v64, v65, v66, v67, v68, v69)
}
fn copy_run_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let mut v13: Rc<str> = glob_base_32(v1.clone());
    let mut v14: Rc<str> = matching_33(v0.clone(), v1.clone());
    let mut v15: u64 = 0u64;
    let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = copies_run_34(v0.clone(), v13.clone(), v2.clone(), v3.clone(), v14.clone(), v15, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12);
    let mut v25: u64 = v20.wrapping_sub(v8);
    let mut v26: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = if v26 {
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v27.clone()
    } else {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v29: u64 = v21.wrapping_sub(v9);
        let mut v30: Rc<str> = decimal_text_29(v29);
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" skipped ("); } LIT.with(|lit| lit.clone()) };
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v32, &v33));
        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v30, &*v31, &*v34, &*v35].concat());
        v36.clone()
    };
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy "); } LIT.with(|lit| lit.clone()) };
    let mut v39: bool = v13.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = if v39 {
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        v40.clone()
    } else {
        v13.clone()
    };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file"); } LIT.with(|lit| lit.clone()) };
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = counted_28(v25, v44.clone(), v45.clone());
    let mut v47: u64 = v22.wrapping_sub(v10);
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
    let mut v50: Rc<str> = counted_28(v47, v48.clone(), v49.clone());
    let mut v51: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v46, &*v43, &*v50, &*v37].concat());
    let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v41, &*v42, &*v2, &*v51].concat());
    let mut v54: bool = if v17 {
        true
    } else {
        let mut v53: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v53
    };
    let mut v55: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v59: Rc<str> = if v55 {
        let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy: no file matched "); } LIT.with(|lit| lit.clone()) };
        let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v58: Rc<str> = std::rc::Rc::<str>::from([&*v56, &*v1, &*v57, &*v57, &*v57].concat());
        v58.clone()
    } else {
        v52.clone()
    };
    let mut v60: Rc<str> = lines_joined_30(v16.clone(), v59.clone());
    let mut v61: u64 = v24.wrapping_add(1u64);
    (v60.clone(), v54, v18, v19, v20, v21, v22, v23, v61)
}
fn read_report_40(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = eoie_do_read::eoie_do_read_report(&*v0, &*v1);
    v2.clone()
}
fn part_at_41(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn read_run_39(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let mut v12: Rc<str> = record_alone_14(v1.clone(), v2);
    let mut v13: Rc<str> = read_report_40(v0.clone(), v12.clone());
    let mut v14: i32 = 0i32;
    let mut v15: Rc<str> = part_at_41(v13.clone(), v14);
    let mut v16: Rc<str> = lines_joined_30(v3.clone(), v15.clone());
    let mut v20: bool = if v4 {
        true
    } else {
        let mut v17: i32 = 1i32;
        let mut v18: Rc<str> = part_at_41(v13.clone(), v17);
        let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v19
    };
    (v16.clone(), v20, v5, v6, v7, v8, v9, v10, v11)
}
fn step_run_17(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: US1, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
        US1::US1_AppendLine(v13, v14) => {
            let mut v13: Rc<str> = v13.clone();
            let mut v14: Rc<str> = v14.clone();
            append_run_18(v0.clone(), v13.clone(), v14.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12)
        }
        _ => {
            let (mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                US1::US1_CopyTree(v24, v25, v26) => {
                    let mut v24: Rc<str> = v24.clone();
                    let mut v25: Rc<str> = v25.clone();
                    let mut v26: Rc<str> = v26.clone();
                    copy_run_31(v0.clone(), v24.clone(), v25.clone(), v26.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12)
                }
                _ => {
                    let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                        US1::US1_ReadAfter => {
                            read_run_39(v0.clone(), v1.clone(), v2, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12)
                        }
                        _ => {
                            let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                                US1::US1_Blank => {
                                    (v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12)
                                }
                                _ => unreachable!(),
                            };
                            (v45.clone(), v46, v47, v48, v49, v50, v51, v52, v53)
                        }
                    };
                    (v54.clone(), v55, v56, v57, v58, v59, v60, v61, v62)
                }
            };
            (v63.clone(), v64, v65, v66, v67, v68, v69, v70, v71)
        }
    };
    (v72.clone(), v73, v74, v75, v76, v77, v78, v79, v80)
}
fn writes_run_16(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let mut v12: i32 = v1.len() as i32;
    let mut v13: bool = v2 < v12;
    let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v13 {
        let mut v14: US0 = write_parsed_4(v1.clone(), v2);
        let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v14 {
            US0::US0_WriteParsed(v15) => {
                let mut v15: US1 = v15.clone();
                let mut v16: i32 = v2.wrapping_add(1i32);
                let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = step_run_17(v0.clone(), v1.clone(), v2, v15.clone(), v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11);
                writes_run_16(v0.clone(), v1.clone(), v16, v17.clone(), v18, v19, v20, v21, v22, v23, v24, v25)
            }
            _ => {
                let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v14 {
                    US0::US0_WriteRejected(v35) => {
                        let mut v35: Rc<str> = v35.clone();
                        let mut v36: i32 = v2.wrapping_add(1i32);
                        writes_run_16(v0.clone(), v1.clone(), v36, v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11)
                    }
                    _ => unreachable!(),
                };
                (v46.clone(), v47, v48, v49, v50, v51, v52, v53, v54)
            }
        };
        (v55.clone(), v56, v57, v58, v59, v60, v61, v62, v63)
    } else {
        (v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11)
    };
    (v64.clone(), v65, v66, v67, v68, v69, v70, v71, v72)
}
fn receipt_counts_42(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64) -> Rc<str> {
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = decimal_text_29(v2);
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  appended "); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = decimal_text_29(v3);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v12, &*v13].concat());
    let mut v15: bool = v8 == 0u64;
    let mut v24: Rc<str> = if v15 {
        v14.clone()
    } else {
        let mut v16: bool = v7 == 0u64;
        if v16 {
            let mut v17: Rc<str> = decimal_text_29(v4);
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" copied"); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v17, &*v18, &*v13, &*v13].concat());
            v19.clone()
        } else {
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  copied "); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = decimal_text_29(v4);
            let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v20, &*v21, &*v13, &*v13].concat());
            v22.clone()
        }
    };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  bytes "); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = decimal_text_29(v6);
    let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v25, &*v26, &*v13, &*v13].concat());
    if v15 {
        v27.clone()
    } else {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  skipped "); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = decimal_text_29(v5);
        let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v13, &*v13].concat());
        v30.clone()
    }
}
fn elapsed_text_44(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn clock_text_45() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_22();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_43(mut v0: US2, mut v1: Rc<str>, mut v2: u64) -> () {
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
    let mut v8: Rc<str> = elapsed_text_44(v2);
    let mut v9: Rc<str> = clock_text_45();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v10, &*v10].concat());
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v1, &*v7, &*v11].concat());
    println!("{}", &*v12);
    ()
}
fn write_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_1();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1.clone());
    let mut v4: i32 = 1i32;
    let mut v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v6 {
        let mut v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = empty_write_tally_15();
        let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = writes_run_16(v0.clone(), v3.clone(), v7, v8.clone(), v9, v10, v11, v12, v13, v14, v15, v16);
        let mut v26: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v26 {
            ()
        } else {
            println!("{}", &*v17);
            ()
        };
        if v18 {
            let mut v27: US2 = US2::US2_Failed;
            let mut v28: Rc<str> = receipt_counts_42(v17.clone(), v18, v19, v20, v21, v22, v23, v24, v25);
            receipt_line_43(v27.clone(), v28.clone(), v2);
            1i32
        } else {
            let mut v29: US2 = US2::US2_Succeeded;
            let mut v30: Rc<str> = receipt_counts_42(v17.clone(), v18, v19, v20, v21, v22, v23, v24, v25);
            receipt_line_43(v29.clone(), v30.clone(), v2);
            0i32
        }
    } else {
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v5, &*v33, &*v33, &*v33].concat());
        println!("{}", &*v34);
        let mut v35: US2 = US2::US2_Failed;
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_43(v35.clone(), v36.clone(), v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        write_program_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_write_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
