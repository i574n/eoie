#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalPlanRow { pub op: String, pub target: String, pub expected: String, pub payload: String, pub effect: String }

fn plan_hex_nibble(byte: u8) -> Option<u8> { match byte { b'0'..=b'9' => Some(byte-b'0'), b'a'..=b'f' => Some(byte-b'a'+10), b'A'..=b'F' => Some(byte-b'A'+10), _ => None } }
fn plan_hex_decode(value: &str) -> Result<String,String> { if !value.len().is_multiple_of(2) { return Err("canonical plan IR hex field has odd length".to_owned()); } let bytes=value.as_bytes(); let mut out=Vec::with_capacity(bytes.len()/2); let mut index=0usize; while index<bytes.len() { let hi=plan_hex_nibble(bytes[index]).ok_or_else(|| "canonical plan IR contains non-hex data".to_owned())?; let lo=plan_hex_nibble(bytes[index+1]).ok_or_else(|| "canonical plan IR contains non-hex data".to_owned())?; out.push((hi<<4)|lo); index+=2; } String::from_utf8(out).map_err(|error| format!("canonical plan IR field is not UTF-8: {error}")) }

pub fn parse_plan_ir_manifest(text: &str) -> Result<Vec<CanonicalPlanRow>,String> { let mut lines=text.lines(); let header=lines.next().unwrap_or_default(); if eoie_plan_ir_header_projection(header).as_ref()!="1" { return Err("canonical plan IR header mismatch".to_owned()); } let mut rows=Vec::new(); for (index,line) in lines.enumerate() { if line.is_empty() { continue; } let (op_hex,target_hex,expected_hex,payload_hex,effect_hex)=eoie_plan_ir_row_hex_tuple(line); if op_hex.is_empty() { return Err(format!("canonical plan IR row {} has invalid shape", index+1)); } let row=CanonicalPlanRow { op:plan_hex_decode(op_hex.as_ref())?, target:plan_hex_decode(target_hex.as_ref())?, expected:plan_hex_decode(expected_hex.as_ref())?, payload:plan_hex_decode(payload_hex.as_ref())?, effect:plan_hex_decode(effect_hex.as_ref())? }; let code=eoie_plan_ir_decode_code(&row.op,&row.target,&row.expected,&row.payload,&row.effect); if code>=8 { return Err(format!("canonical plan IR row {} was rejected by Spiral decoder", index+1)); } rows.push(row); } if rows.is_empty() { return Err("canonical plan IR manifest contains no operations".to_owned()); } Ok(rows) }
pub fn plan_ir_manifest_count(value: &str) -> i32 { let header=value.lines().next().unwrap_or_default(); if eoie_plan_ir_header_projection(header).as_ref()=="1" { match parse_plan_ir_manifest(value) { Ok(rows) => i32::try_from(rows.len()).unwrap_or(-1), Err(_) => -1 } } else if header.starts_with("EOIE-PLAN-IR") { -1 } else { 0 } }

fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
fn plan_schema_tag_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("schema"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("canonical-plan-ir"); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    }
}
fn plan_descriptor_by_index_1(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v1: i32 = (v0).parse::<u8>().ok().filter(|index| index.to_string() == &*v0).map(|index| index as i32).unwrap_or(-1);
    let mut v2: bool = 0i32 == v1;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("create"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/example.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("typed-payload"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
        (v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone())
    } else {
        let mut v8: bool = 1i32 == v1;
        if v8 {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/example.spi"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacement"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
            (v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone())
        } else {
            let mut v14: bool = 2i32 == v1;
            if v14 {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch-exact"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/example.spi"); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("old"); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("new"); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                (v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone())
            } else {
                let mut v20: bool = 3i32 == v1;
                if v20 {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mkdir"); } LIT.with(|lit| lit.clone()) };
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/generated"); } LIT.with(|lit| lit.clone()) };
                    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                    (v21.clone(), v22.clone(), v23.clone(), v23.clone(), v24.clone())
                } else {
                    let mut v25: bool = 4i32 == v1;
                    if v25 {
                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) };
                        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cargo-check"); } LIT.with(|lit| lit.clone()) };
                        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        (v26.clone(), v27.clone(), v28.clone(), v28.clone(), v26.clone())
                    } else {
                        let mut v29: bool = 5i32 == v1;
                        if v29 {
                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy"); } LIT.with(|lit| lit.clone()) };
                            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/copied.spi"); } LIT.with(|lit| lit.clone()) };
                            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected-sha"); } LIT.with(|lit| lit.clone()) };
                            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/source.spi"); } LIT.with(|lit| lit.clone()) };
                            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                            (v30.clone(), v31.clone(), v32.clone(), v33.clone(), v34.clone())
                        } else {
                            let mut v35: bool = 6i32 == v1;
                            if v35 {
                                let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chmod"); } LIT.with(|lit| lit.clone()) };
                                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/tool"); } LIT.with(|lit| lit.clone()) };
                                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0755"); } LIT.with(|lit| lit.clone()) };
                                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0644"); } LIT.with(|lit| lit.clone()) };
                                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                                (v36.clone(), v37.clone(), v38.clone(), v39.clone(), v40.clone())
                            } else {
                                let mut v41: bool = 7i32 == v1;
                                if v41 {
                                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("symlink"); } LIT.with(|lit| lit.clone()) };
                                    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/current"); } LIT.with(|lit| lit.clone()) };
                                    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/target"); } LIT.with(|lit| lit.clone()) };
                                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                                    (v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46.clone())
                                } else {
                                    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid"); } LIT.with(|lit| lit.clone()) };
                                    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    (v47.clone(), v48.clone(), v48.clone(), v48.clone(), v47.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method4() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_3() -> Rc<str> {
    method4()
}
fn plan_manifest_header_projection_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE-PLAN-IR\t1"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        codec_empty_3()
    }
}
fn codec_runtime_char_6(mut v0: Rc<str>) -> u8 {
    let mut v1: u8 = v0.clone().as_bytes()[0i32 as usize];
    v1
}
fn find_char_loop_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: u8) -> i32 {
    loop {
        let mut v4: bool = v1 == v2 ;
        if v4 {
            return v2;
        } else {
            let mut v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == v3 ;
            if v6 {
                return v1;
            } else {
                let mut v7: i32 = v1 + 1i32 ;
                (v0, v1, v2, v3) = (v0.clone(), v7, v2, v3);
                continue;
            }
        }
    }
}
fn plan_manifest_row_hex_tuple_5(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v3: u8 = codec_runtime_char_6(v2.clone());
    let mut v4: i32 = 0i32;
    let mut v5: i32 = find_char_loop_7(v0.clone(), v4, v1, v3);
    let mut v6: bool = v5 == v1 ;
    if v6 {
        let mut v7: Rc<str> = codec_empty_3();
        let mut v8: Rc<str> = codec_empty_3();
        let mut v9: Rc<str> = codec_empty_3();
        let mut v10: Rc<str> = codec_empty_3();
        let mut v11: Rc<str> = codec_empty_3();
        (v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone())
    } else {
        let mut v12: i32 = v5 + 1i32 ;
        let mut v13: i32 = find_char_loop_7(v0.clone(), v12, v1, v3);
        let mut v14: bool = v13 == v1 ;
        if v14 {
            let mut v15: Rc<str> = codec_empty_3();
            let mut v16: Rc<str> = codec_empty_3();
            let mut v17: Rc<str> = codec_empty_3();
            let mut v18: Rc<str> = codec_empty_3();
            let mut v19: Rc<str> = codec_empty_3();
            (v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone())
        } else {
            let mut v20: i32 = v13 + 1i32 ;
            let mut v21: i32 = find_char_loop_7(v0.clone(), v20, v1, v3);
            let mut v22: bool = v21 == v1 ;
            if v22 {
                let mut v23: Rc<str> = codec_empty_3();
                let mut v24: Rc<str> = codec_empty_3();
                let mut v25: Rc<str> = codec_empty_3();
                let mut v26: Rc<str> = codec_empty_3();
                let mut v27: Rc<str> = codec_empty_3();
                (v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone())
            } else {
                let mut v28: i32 = v21 + 1i32 ;
                let mut v29: i32 = find_char_loop_7(v0.clone(), v28, v1, v3);
                let mut v30: bool = v29 == v1 ;
                if v30 {
                    let mut v31: Rc<str> = codec_empty_3();
                    let mut v32: Rc<str> = codec_empty_3();
                    let mut v33: Rc<str> = codec_empty_3();
                    let mut v34: Rc<str> = codec_empty_3();
                    let mut v35: Rc<str> = codec_empty_3();
                    (v31.clone(), v32.clone(), v33.clone(), v34.clone(), v35.clone())
                } else {
                    let mut v36: i32 = v29 + 1i32 ;
                    let mut v37: i32 = find_char_loop_7(v0.clone(), v36, v1, v3);
                    let mut v38: bool = v37 == v1 ;
                    if v38 {
                        let mut v39: Rc<str> = codec_empty_3();
                        let mut v40: Rc<str> = codec_empty_3();
                        let mut v41: Rc<str> = codec_empty_3();
                        let mut v42: Rc<str> = codec_empty_3();
                        let mut v43: Rc<str> = codec_empty_3();
                        (v39.clone(), v40.clone(), v41.clone(), v42.clone(), v43.clone())
                    } else {
                        let mut v44: i32 = v37 + 1i32 ;
                        let mut v45: i32 = find_char_loop_7(v0.clone(), v44, v1, v3);
                        let mut v46: bool = 0i32 == v5 ;
                        let mut v50: Rc<str> = if v46 {
                            codec_empty_3()
                        } else {
                            let mut v48: i32 = v5 - 1i32 ;
                            let mut v49: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v48 as i64);
                            v49.clone()
                        };
                        let mut v51: bool = v45 < v1;
                        if v51 {
                            let mut v52: Rc<str> = codec_empty_3();
                            let mut v53: Rc<str> = codec_empty_3();
                            let mut v54: Rc<str> = codec_empty_3();
                            let mut v55: Rc<str> = codec_empty_3();
                            let mut v56: Rc<str> = codec_empty_3();
                            (v52.clone(), v53.clone(), v54.clone(), v55.clone(), v56.clone())
                        } else {
                            let mut v57: bool = v50.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("op"); } LIT.with(|lit| lit.clone()) };
                            if v57 {
                                let mut v58: i32 = v5 + 1i32 ;
                                let mut v59: bool = v58 == v13 ;
                                let mut v63: Rc<str> = if v59 {
                                    codec_empty_3()
                                } else {
                                    let mut v61: i32 = v13 - 1i32 ;
                                    let mut v62: Rc<str> = string_slice(&v0.clone(), v58 as i64, v61 as i64);
                                    v62.clone()
                                };
                                let mut v64: i32 = v13 + 1i32 ;
                                let mut v65: bool = v64 == v21 ;
                                let mut v69: Rc<str> = if v65 {
                                    codec_empty_3()
                                } else {
                                    let mut v67: i32 = v21 - 1i32 ;
                                    let mut v68: Rc<str> = string_slice(&v0.clone(), v64 as i64, v67 as i64);
                                    v68.clone()
                                };
                                let mut v70: i32 = v21 + 1i32 ;
                                let mut v71: bool = v70 == v29 ;
                                let mut v75: Rc<str> = if v71 {
                                    codec_empty_3()
                                } else {
                                    let mut v73: i32 = v29 - 1i32 ;
                                    let mut v74: Rc<str> = string_slice(&v0.clone(), v70 as i64, v73 as i64);
                                    v74.clone()
                                };
                                let mut v76: i32 = v29 + 1i32 ;
                                let mut v77: bool = v76 == v37 ;
                                let mut v81: Rc<str> = if v77 {
                                    codec_empty_3()
                                } else {
                                    let mut v79: i32 = v37 - 1i32 ;
                                    let mut v80: Rc<str> = string_slice(&v0.clone(), v76 as i64, v79 as i64);
                                    v80.clone()
                                };
                                let mut v82: i32 = v37 + 1i32 ;
                                let mut v83: bool = v82 == v1 ;
                                let mut v87: Rc<str> = if v83 {
                                    codec_empty_3()
                                } else {
                                    let mut v85: i32 = v1 - 1i32 ;
                                    let mut v86: Rc<str> = string_slice(&v0.clone(), v82 as i64, v85 as i64);
                                    v86.clone()
                                };
                                (v63.clone(), v69.clone(), v75.clone(), v81.clone(), v87.clone())
                            } else {
                                let mut v88: Rc<str> = codec_empty_3();
                                let mut v89: Rc<str> = codec_empty_3();
                                let mut v90: Rc<str> = codec_empty_3();
                                let mut v91: Rc<str> = codec_empty_3();
                                let mut v92: Rc<str> = codec_empty_3();
                                (v88.clone(), v89.clone(), v90.clone(), v91.clone(), v92.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn plan_manifest_projection_witness_8() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE-PLAN-IR\t1"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = plan_manifest_header_projection_2(v0.clone());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE-PLAN-IR\t2"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = plan_manifest_header_projection_2(v3.clone());
        let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("op\t01\t02\t03\t04\t05"); } LIT.with(|lit| lit.clone()) };
            let (mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = plan_manifest_row_hex_tuple_5(v6.clone());
            let mut v12: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("01"); } LIT.with(|lit| lit.clone()) };
            if v12 {
                let mut v13: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("02"); } LIT.with(|lit| lit.clone()) };
                if v13 {
                    let mut v14: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("03"); } LIT.with(|lit| lit.clone()) };
                    if v14 {
                        let mut v15: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("04"); } LIT.with(|lit| lit.clone()) };
                        if v15 {
                            let mut v16: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("05"); } LIT.with(|lit| lit.clone()) };
                            if v16 {
                                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("op\t01\t02\t03\t04\t05\t06"); } LIT.with(|lit| lit.clone()) };
                                let (mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = plan_manifest_row_hex_tuple_5(v17.clone());
                                let mut v23: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                if v23 {
                                    let mut v24: bool = v19.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    if v24 {
                                        let mut v25: bool = v20.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        if v25 {
                                            let mut v26: bool = v21.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            if v26 {
                                                let mut v27: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                if v27 {
                                                    1i32
                                                } else {
                                                    0i32
                                                }
                                            } else {
                                                0i32
                                            }
                                        } else {
                                            0i32
                                        }
                                    } else {
                                        0i32
                                    }
                                } else {
                                    0i32
                                }
                            } else {
                                0i32
                            }
                        } else {
                            0i32
                        }
                    } else {
                        0i32
                    }
                } else {
                    0i32
                }
            } else {
                0i32
            }
        } else {
            0i32
        }
    } else {
        0i32
    }
}
fn plan_provider_count_9() -> i32 {
    8i32
}
fn plan_stage_witness_10() -> i32 {
    1i32
}
fn plan_index_valid_11(mut v0: Rc<str>) -> u64 {
    let mut v1: i32 = (v0).parse::<u8>().ok().filter(|index| index.to_string() == &*v0).map(|index| index as i32).unwrap_or(-1);
    let mut v2: bool = 0i32 <= v1;
    let mut v4: bool = if v2 {
        let mut v3: bool = v1 < 8i32;
        v3
    } else {
        false
    };
    if v4 {
        1u64
    } else {
        0u64
    }
}
fn plan_decode_code_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> u64 {
    let mut v5: u64 = 8i32 as u64;
    let mut v6: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("create"); } LIT.with(|lit| lit.clone()) } == v0.clone();
    let mut v8: bool = if v6 {
        let mut v7: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) } == v4.clone();
        v7
    } else {
        false
    };
    if v8 {
        let mut v9: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: bool = if v9 {
            true
        } else {
            let mut v10: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v10
        };
        let mut v12: bool = v11 == false;
        let mut v14: bool = if v12 {
            let mut v13: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v13
        } else {
            false
        };
        if v14 {
            let mut v15: u64 = 0i32 as u64;
            v15
        } else {
            v5
        }
    } else {
        let mut v17: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) } == v0.clone();
        let mut v19: bool = if v17 {
            let mut v18: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) } == v4.clone();
            v18
        } else {
            false
        };
        if v19 {
            let mut v20: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v22: bool = if v20 {
                true
            } else {
                let mut v21: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                v21
            };
            let mut v23: bool = v22 == false;
            let mut v28: bool = if v23 {
                let mut v24: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v26: bool = if v24 {
                    true
                } else {
                    let mut v25: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    v25
                };
                let mut v27: bool = v26 == false;
                v27
            } else {
                false
            };
            if v28 {
                let mut v29: u64 = 1i32 as u64;
                v29
            } else {
                v5
            }
        } else {
            let mut v31: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch-exact"); } LIT.with(|lit| lit.clone()) } == v0.clone();
            let mut v33: bool = if v31 {
                let mut v32: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) } == v4.clone();
                v32
            } else {
                false
            };
            if v33 {
                let mut v34: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v36: bool = if v34 {
                    true
                } else {
                    let mut v35: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    v35
                };
                let mut v37: bool = v36 == false;
                if v37 {
                    let mut v38: u64 = 2i32 as u64;
                    v38
                } else {
                    v5
                }
            } else {
                let mut v40: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mkdir"); } LIT.with(|lit| lit.clone()) } == v0.clone();
                let mut v42: bool = if v40 {
                    let mut v41: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) } == v4.clone();
                    v41
                } else {
                    false
                };
                if v42 {
                    let mut v43: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v45: bool = if v43 {
                        true
                    } else {
                        let mut v44: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v44
                    };
                    let mut v46: bool = v45 == false;
                    let mut v48: bool = if v46 {
                        let mut v47: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v47
                    } else {
                        false
                    };
                    let mut v50: bool = if v48 {
                        let mut v49: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v49
                    } else {
                        false
                    };
                    if v50 {
                        let mut v51: u64 = 3i32 as u64;
                        v51
                    } else {
                        v5
                    }
                } else {
                    let mut v53: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) } == v0.clone();
                    let mut v55: bool = if v53 {
                        let mut v54: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) } == v4.clone();
                        v54
                    } else {
                        false
                    };
                    if v55 {
                        let mut v56: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v58: bool = if v56 {
                            true
                        } else {
                            let mut v57: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                            v57
                        };
                        let mut v59: bool = v58 == false;
                        let mut v61: bool = if v59 {
                            let mut v60: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                            v60
                        } else {
                            false
                        };
                        let mut v63: bool = if v61 {
                            let mut v62: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                            v62
                        } else {
                            false
                        };
                        if v63 {
                            let mut v64: u64 = 4i32 as u64;
                            v64
                        } else {
                            v5
                        }
                    } else {
                        let mut v66: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy"); } LIT.with(|lit| lit.clone()) } == v0.clone();
                        let mut v68: bool = if v66 {
                            let mut v67: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) } == v4.clone();
                            v67
                        } else {
                            false
                        };
                        if v68 {
                            let mut v69: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v71: bool = if v69 {
                                true
                            } else {
                                let mut v70: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                v70
                            };
                            let mut v72: bool = v71 == false;
                            let mut v77: bool = if v72 {
                                let mut v73: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v75: bool = if v73 {
                                    true
                                } else {
                                    let mut v74: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                    v74
                                };
                                let mut v76: bool = v75 == false;
                                v76
                            } else {
                                false
                            };
                            let mut v82: bool = if v77 {
                                let mut v78: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v80: bool = if v78 {
                                    true
                                } else {
                                    let mut v79: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                    v79
                                };
                                let mut v81: bool = v80 == false;
                                v81
                            } else {
                                false
                            };
                            if v82 {
                                let mut v83: u64 = 5i32 as u64;
                                v83
                            } else {
                                v5
                            }
                        } else {
                            let mut v85: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chmod"); } LIT.with(|lit| lit.clone()) } == v0.clone();
                            let mut v87: bool = if v85 {
                                let mut v86: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) } == v4.clone();
                                v86
                            } else {
                                false
                            };
                            if v87 {
                                let mut v88: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v90: bool = if v88 {
                                    true
                                } else {
                                    let mut v89: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                    v89
                                };
                                let mut v91: bool = v90 == false;
                                let mut v96: bool = if v91 {
                                    let mut v92: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v94: bool = if v92 {
                                        true
                                    } else {
                                        let mut v93: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                        v93
                                    };
                                    let mut v95: bool = v94 == false;
                                    v95
                                } else {
                                    false
                                };
                                let mut v101: bool = if v96 {
                                    let mut v97: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v99: bool = if v97 {
                                        true
                                    } else {
                                        let mut v98: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                        v98
                                    };
                                    let mut v100: bool = v99 == false;
                                    v100
                                } else {
                                    false
                                };
                                if v101 {
                                    let mut v102: u64 = 6i32 as u64;
                                    v102
                                } else {
                                    v5
                                }
                            } else {
                                let mut v104: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("symlink"); } LIT.with(|lit| lit.clone()) } == v0.clone();
                                let mut v106: bool = if v104 {
                                    let mut v105: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) } == v4.clone();
                                    v105
                                } else {
                                    false
                                };
                                if v106 {
                                    let mut v107: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let mut v109: bool = if v107 {
                                        true
                                    } else {
                                        let mut v108: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                        v108
                                    };
                                    let mut v110: bool = v109 == false;
                                    let mut v112: bool = if v110 {
                                        let mut v111: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                        v111
                                    } else {
                                        false
                                    };
                                    let mut v117: bool = if v112 {
                                        let mut v113: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let mut v115: bool = if v113 {
                                            true
                                        } else {
                                            let mut v114: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                            v114
                                        };
                                        let mut v116: bool = v115 == false;
                                        v116
                                    } else {
                                        false
                                    };
                                    if v117 {
                                        let mut v118: u64 = 7i32 as u64;
                                        v118
                                    } else {
                                        v5
                                    }
                                } else {
                                    v5
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        plan_schema_tag_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        plan_descriptor_by_index_1(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        plan_manifest_header_projection_2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        plan_manifest_row_hex_tuple_5(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        plan_manifest_projection_witness_8()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        plan_provider_count_9()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        plan_stage_witness_10()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        plan_index_valid_11(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>| -> u64 {
        plan_decode_code_12(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_plan_ir_schema(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn eoie_plan_ir_descriptor(v0: &str) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure1()(Rc::<str>::from(v0))
}
pub fn eoie_plan_ir_header_projection(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_plan_ir_row_hex_tuple(v0: &str) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure3()(Rc::<str>::from(v0))
}
pub fn eoie_plan_ir_projection_witness() -> i32 {
    closure4()()
}
pub fn eoie_plan_ir_count() -> i32 {
    closure5()()
}
pub fn eoie_plan_ir_stage_witness() -> i32 {
    closure6()()
}
pub fn eoie_plan_ir_index_valid(v0: &str) -> u64 {
    closure7()(Rc::<str>::from(v0))
}
pub fn eoie_plan_ir_decode_code(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str) -> u64 {
    closure8()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4))
}
