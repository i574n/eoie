#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use std::path::{Component, Path, PathBuf};

pub fn tar_octal(field: &[u8], label: &str) -> Result<u64, String> {
    let end = field.iter().position(|byte| *byte == 0).unwrap_or(field.len());
    let text = std::str::from_utf8(&field[..end]).map_err(|_| format!("non-UTF8 tar {label}"))?;
    let error = eoie_tar_octal_error(text, label);
    if error.is_empty() { Ok(eoie_tar_octal_value(text)) } else { Err(error.to_string()) }
}

pub fn tar_text(field: &[u8], label: &str) -> Result<String, String> {
    let end = field.iter().position(|byte| *byte == 0).unwrap_or(field.len());
    Ok(std::str::from_utf8(&field[..end]).map_err(|_| format!("non-UTF8 tar {label}"))?.trim().to_owned())
}

pub fn tar_path(header: &[u8; 512]) -> Result<PathBuf, String> {
    let name = tar_text(&header[0..100], "name")?;
    let prefix = tar_text(&header[345..500], "prefix")?;
    let joined = if prefix.is_empty() { name } else { format!("{prefix}/{name}") };
    let normalized = joined.strip_prefix("./").unwrap_or(&joined);
    let path = PathBuf::from(normalized);
    if path.as_os_str().is_empty() || path.is_absolute() || !path.components().all(|component| matches!(component, Component::Normal(_))) {
        return Err(format!("unsafe tar entry path: {joined}"));
    }
    Ok(path)
}

pub fn tar_pax_path(payload: &[u8]) -> Result<PathBuf, String> {
    if payload.is_empty() || payload.len() > 65536 { return Err(format!("PAX extended header size rejected: {}", payload.len())); }
    let space = payload.iter().position(|byte| *byte == b' ').ok_or_else(|| "PAX record missing length separator".to_owned())?;
    let length_text = std::str::from_utf8(&payload[..space]).map_err(|_| "non-UTF8 PAX record length".to_owned())?;
    let declared = length_text.parse::<usize>().map_err(|_| format!("invalid PAX record length: {length_text}"))?;
    if declared != payload.len() || space + 2 >= declared { return Err(format!("PAX record length mismatch declared={declared} observed={}", payload.len())); }
    let record = &payload[space + 1..];
    if record.last() != Some(&b'\n') { return Err("PAX record missing newline".to_owned()); }
    let content = &record[..record.len() - 1];
    let equals = content.iter().position(|byte| *byte == b'=').ok_or_else(|| "PAX record missing equals".to_owned())?;
    let key = std::str::from_utf8(&content[..equals]).map_err(|_| "non-UTF8 PAX key".to_owned())?;
    if key != "path" { return Err(format!("unsupported PAX key: {key}")); }
    let value = std::str::from_utf8(&content[equals + 1..]).map_err(|_| "non-UTF8 PAX path".to_owned())?;
    let normalized = value.strip_prefix("./").unwrap_or(value);
    let path = PathBuf::from(normalized);
    let path_safe = !path.as_os_str().is_empty() && !path.is_absolute() && path.components().all(|component| matches!(component, Component::Normal(_)));
    if archive_deflate_domain::eoie_archive_tar_pax_path_binding(i32::from(path_safe), 1) != 1 { return Err(format!("unsafe PAX path: {value}")); }
    Ok(path)
}

pub fn tar_checksum(header: &[u8; 512]) -> Result<(), String> {
    let expected = tar_octal(&header[148..156], "checksum")?;
    let observed = header.iter().enumerate().map(|(index, byte)| if (148..156).contains(&index) { u64::from(b' ') } else { u64::from(*byte) }).sum::<u64>();
    if expected == observed { Ok(()) } else { Err(format!("tar checksum mismatch expected={expected} observed={observed}")) }
}

pub fn archive_format(path: &Path) -> Result<(i32, std::rc::Rc<str>), String> {
    let name = path.file_name().and_then(|value| value.to_str()).unwrap_or("");
    match eoie_archive_format_code(name) {
        code if code < 0 => Err(format!("unsupported archive extension: {}", path.display())),
        code => Ok((code, eoie_archive_format_label(name))),
    }
}

fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US1 {
    US1_ZipArchive,
    US1_TarArchive,
    US1_GzipTarArchive,
    US1_XzTarArchive,
    US1_Bzip2TarArchive,
}
#[derive(Clone)]
enum US0 {
    US0_Recognized(US1),
    US0_Unsupported,
}
#[derive(Clone)]
enum US2 {
    US2_EmptyField,
    US2_OctalValue(u64),
    US2_InvalidOctal(Rc<str>),
}
fn classify_archive_name_1(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".zip"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.ends_with(&*v2);
    if v3 {
        let mut v4: US1 = US1::US1_ZipArchive;
        US0::US0_Recognized(v4.clone())
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".tar.gz"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v1.ends_with(&*v6);
        let mut v10: bool = if v7 {
            true
        } else {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".tgz"); } LIT.with(|lit| lit.clone()) };
            let mut v9: bool = v1.ends_with(&*v8);
            v9
        };
        if v10 {
            let mut v11: US1 = US1::US1_GzipTarArchive;
            US0::US0_Recognized(v11.clone())
        } else {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".tar.xz"); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = v1.ends_with(&*v13);
            let mut v17: bool = if v14 {
                true
            } else {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".txz"); } LIT.with(|lit| lit.clone()) };
                let mut v16: bool = v1.ends_with(&*v15);
                v16
            };
            if v17 {
                let mut v18: US1 = US1::US1_XzTarArchive;
                US0::US0_Recognized(v18.clone())
            } else {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".tar.bz2"); } LIT.with(|lit| lit.clone()) };
                let mut v21: bool = v1.ends_with(&*v20);
                let mut v24: bool = if v21 {
                    true
                } else {
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".tbz2"); } LIT.with(|lit| lit.clone()) };
                    let mut v23: bool = v1.ends_with(&*v22);
                    v23
                };
                let mut v27: bool = if v24 {
                    true
                } else {
                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".tbz"); } LIT.with(|lit| lit.clone()) };
                    let mut v26: bool = v1.ends_with(&*v25);
                    v26
                };
                if v27 {
                    let mut v28: US1 = US1::US1_Bzip2TarArchive;
                    US0::US0_Recognized(v28.clone())
                } else {
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".tar"); } LIT.with(|lit| lit.clone()) };
                    let mut v31: bool = v1.ends_with(&*v30);
                    if v31 {
                        let mut v32: US1 = US1::US1_TarArchive;
                        US0::US0_Recognized(v32.clone())
                    } else {
                        US0::US0_Unsupported
                    }
                }
            }
        }
    }
}
fn archive_classification_code_0(mut v0: Rc<str>) -> i32 {
    let mut v1: US0 = classify_archive_name_1(v0.clone());
    match &v1 {
        US0::US0_Recognized(v2) => {
            let mut v2: US1 = v2.clone();
            match &v2 {
                US1::US1_Bzip2TarArchive => {
                    4i32
                }
                US1::US1_GzipTarArchive => {
                    2i32
                }
                US1::US1_TarArchive => {
                    1i32
                }
                US1::US1_XzTarArchive => {
                    3i32
                }
                US1::US1_ZipArchive => {
                    0i32
                }
            }
        }
        US0::US0_Unsupported => {
            -1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        archive_classification_code_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn archive_classification_label_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = classify_archive_name_1(v0.clone());
    match &v1 {
        US0::US0_Recognized(v2) => {
            let mut v2: US1 = v2.clone();
            match &v2 {
                US1::US1_Bzip2TarArchive => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tar.bz2"); } LIT.with(|lit| lit.clone()) };
                    v7.clone()
                }
                US1::US1_GzipTarArchive => {
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tar.gz"); } LIT.with(|lit| lit.clone()) };
                    v5.clone()
                }
                US1::US1_TarArchive => {
                    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tar"); } LIT.with(|lit| lit.clone()) };
                    v4.clone()
                }
                US1::US1_XzTarArchive => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tar.xz"); } LIT.with(|lit| lit.clone()) };
                    v6.clone()
                }
                US1::US1_ZipArchive => {
                    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zip"); } LIT.with(|lit| lit.clone()) };
                    v3.clone()
                }
            }
        }
        US0::US0_Unsupported => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        archive_classification_label_2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn read_octal_field_4(mut v0: Rc<str>) -> US2 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method5(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method6(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v9 {
        US2::US2_EmptyField
    } else {
        let mut v11: bool = u64::from_str_radix(&*v8, 8).is_err();
        if v11 {
            US2::US2_InvalidOctal(v8.clone())
        } else {
            let mut v13: u64 = u64::from_str_radix(&*v8, 8).unwrap_or(0);
            US2::US2_OctalValue(v13)
        }
    }
}
fn octal_field_value_3(mut v0: Rc<str>) -> u64 {
    let mut v1: US2 = read_octal_field_4(v0.clone());
    match &v1 {
        US2::US2_EmptyField => {
            0u64
        }
        US2::US2_InvalidOctal(v3) => {
            let mut v3: Rc<str> = v3.clone();
            0u64
        }
        US2::US2_OctalValue(v2) => {
            let mut v2: u64 = *v2;
            v2
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        octal_field_value_3(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn octal_field_error_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US2 = read_octal_field_4(v0.clone());
    match &v2 {
        US2::US2_InvalidOctal(v3) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid tar "); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v1));
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v3));
            v8.clone()
        }
        _ => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        octal_field_error_7(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_archive_format_code(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_archive_format_label(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
pub fn eoie_tar_octal_value(v0: &str) -> u64 {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_tar_octal_error(v0: &str, v1: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
