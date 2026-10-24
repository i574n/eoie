#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Rendered(Rc<str>),
    US0_Refused(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_StateMissing,
    US1_StateText(Rc<str>),
    US1_StateUnreadable(Rc<str>),
}
fn state_host_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (i32, std::rc::Rc<str>) {
    let mut v2: (i32, std::rc::Rc<str>) = { let path = std::path::Path::new(&*v0).join(&*v1); match std::fs::read_to_string(&path) { Ok(source) => (1, std::rc::Rc::<str>::from(source)), Err(error) if error.kind() == std::io::ErrorKind::NotFound => (0, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }), Err(error) => (2, std::rc::Rc::<str>::from(format!("read {}: {error}", path.display()))) } };
    v2.clone()
}
fn state_read_of_2(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let mut v2: (i32, std::rc::Rc<str>) = state_host_of_3(v0.clone(), v1.clone());
    let mut v3: i32 = v2.clone().0;
    let mut v4: bool = v3 == 1i32;
    if v4 {
        let mut v5: Rc<str> = v2.1.clone();
        US1::US1_StateText(v5.clone())
    } else {
        let mut v7: bool = v3 == 0i32;
        if v7 {
            US1::US1_StateMissing
        } else {
            let mut v9: Rc<str> = v2.1.clone();
            US1::US1_StateUnreadable(v9.clone())
        }
    }
}
fn sections_block_4(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff sections"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = handoff_sections_projection_domain::eoie_handoff_sections_projection(&*v0);
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: US0 = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff sections projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
        US0::US0_Refused(v5.clone())
    } else {
        US0::US0_Rendered(v2.clone())
    };
    match &v8 {
        US0::US0_Refused(v9) => {
            let mut v9: Rc<str> = v9.clone();
            US0::US0_Refused(v9.clone())
        }
        US0::US0_Rendered(v11) => {
            let mut v11: Rc<str> = v11.clone();
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            let mut v13: bool = v11.starts_with(&*v12);
            if v13 {
                let mut v14: Rc<str> = std::rc::Rc::<str>::from(&(&*v11)[2i32 as usize..]);
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
                US0::US0_Rendered(v16.clone())
            } else {
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff sections projection returned invalid envelope"); } LIT.with(|lit| lit.clone()) };
                US0::US0_Refused(v18.clone())
            }
        }
    }
}
fn recorded_lines_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let mut v3: i32 = authority_state_domain::eoie_authority_declared_mask(0, 0);
    let mut v4: i32 = authority_state_domain::eoie_authority_state(v3, 0);
    let mut v5: bool = v3 < 0i32;
    let mut v7: bool = if v5 {
        true
    } else {
        let mut v6: bool = v4 < 0i32;
        v6
    };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiled authority projection rejected"); } LIT.with(|lit| lit.clone()) };
        US0::US0_Refused(v8.clone())
    } else {
        let mut v10: bool = v4 == 1i32;
        let mut v13: Rc<str> = if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exclusive"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        } else {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nauthority="); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" blocker_mask="); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
        let mut v18: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
        let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v2));
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
        let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v23));
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n## Recorded state\n\nSaved ratings, census and receipts are shown below with compiled authority declarations. This command does not revalidate their freshness or certify a release.\n"); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n## Recorded state\n\nSaved ratings, census and receipts are shown below with compiled authority declarations. This command does not revalidate their freshness or certify a release.\n\n"); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v1));
        let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v22));
        let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v24));
        let mut v30: US0 = US0::US0_Rendered(v29.clone());
        let mut v61: US0 = match &v30 {
            US0::US0_Refused(v31) => {
                let mut v31: Rc<str> = v31.clone();
                US0::US0_Refused(v31.clone())
            }
            US0::US0_Rendered(v33) => {
                let mut v33: Rc<str> = v33.clone();
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/toolchain_identity.spi"); } LIT.with(|lit| lit.clone()) };
                let mut v35: US1 = state_read_of_2(v0.clone(), v34.clone());
                match &v35 {
                    US1::US1_StateMissing => {
                        US0::US0_Rendered(v33.clone())
                    }
                    US1::US1_StateText(v39) => {
                        let mut v39: Rc<str> = v39.clone();
                        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) };
                        let mut v41: Rc<str> = handoff_evidence_projection_domain::eoie_handoff_toolchain_projection(&*v39);
                        let mut v42: bool = v41.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v47: US0 = if v42 {
                            let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                            let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                            US0::US0_Refused(v44.clone())
                        } else {
                            US0::US0_Rendered(v41.clone())
                        };
                        match &v47 {
                            US0::US0_Refused(v48) => {
                                let mut v48: Rc<str> = v48.clone();
                                US0::US0_Refused(v48.clone())
                            }
                            US0::US0_Rendered(v50) => {
                                let mut v50: Rc<str> = v50.clone();
                                let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v22));
                                let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v50));
                                let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v22));
                                US0::US0_Rendered(v53.clone())
                            }
                        }
                    }
                    US1::US1_StateUnreadable(v37) => {
                        let mut v37: Rc<str> = v37.clone();
                        US0::US0_Refused(v37.clone())
                    }
                }
            }
        };
        match &v61 {
            US0::US0_Refused(v62) => {
                let mut v62: Rc<str> = v62.clone();
                US0::US0_Refused(v62.clone())
            }
            US0::US0_Rendered(v64) => {
                let mut v64: Rc<str> = v64.clone();
                let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/cold_proof.spi"); } LIT.with(|lit| lit.clone()) };
                let mut v66: US1 = state_read_of_2(v0.clone(), v65.clone());
                match &v66 {
                    US1::US1_StateMissing => {
                        US0::US0_Rendered(v64.clone())
                    }
                    US1::US1_StateText(v70) => {
                        let mut v70: Rc<str> = v70.clone();
                        let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cold proof"); } LIT.with(|lit| lit.clone()) };
                        let mut v72: Rc<str> = handoff_evidence_projection_domain::eoie_handoff_cold_projection(&*v70);
                        let mut v73: bool = v72.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v78: US0 = if v73 {
                            let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                            let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cold proof projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                            US0::US0_Refused(v75.clone())
                        } else {
                            US0::US0_Rendered(v72.clone())
                        };
                        match &v78 {
                            US0::US0_Refused(v79) => {
                                let mut v79: Rc<str> = v79.clone();
                                US0::US0_Refused(v79.clone())
                            }
                            US0::US0_Rendered(v81) => {
                                let mut v81: Rc<str> = v81.clone();
                                let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v22));
                                let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v81));
                                let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v22));
                                US0::US0_Rendered(v84.clone())
                            }
                        }
                    }
                    US1::US1_StateUnreadable(v68) => {
                        let mut v68: Rc<str> = v68.clone();
                        US0::US0_Refused(v68.clone())
                    }
                }
            }
        }
    }
}
fn recorded_block_5(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/ratings.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: US1 = state_read_of_2(v0.clone(), v1.clone());
    match &v2 {
        US1::US1_StateMissing => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            US0::US0_Rendered(v3.clone())
        }
        US1::US1_StateText(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/authority_census.spi"); } LIT.with(|lit| lit.clone()) };
            let mut v9: US1 = state_read_of_2(v0.clone(), v8.clone());
            match &v9 {
                US1::US1_StateMissing => {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    US0::US0_Rendered(v10.clone())
                }
                US1::US1_StateText(v14) => {
                    let mut v14: Rc<str> = v14.clone();
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ratings"); } LIT.with(|lit| lit.clone()) };
                    let mut v16: Rc<str> = handoff_state_projection_domain::eoie_handoff_ratings_projection(&*v7);
                    let mut v17: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v22: US0 = if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ratings projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                        US0::US0_Refused(v19.clone())
                    } else {
                        US0::US0_Rendered(v16.clone())
                    };
                    match &v22 {
                        US0::US0_Refused(v23) => {
                            let mut v23: Rc<str> = v23.clone();
                            US0::US0_Refused(v23.clone())
                        }
                        US0::US0_Rendered(v25) => {
                            let mut v25: Rc<str> = v25.clone();
                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority census"); } LIT.with(|lit| lit.clone()) };
                            let mut v27: Rc<str> = handoff_state_projection_domain::eoie_handoff_census_projection(&*v14);
                            let mut v28: bool = v27.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v33: US0 = if v28 {
                                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority census projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                                US0::US0_Refused(v30.clone())
                            } else {
                                US0::US0_Rendered(v27.clone())
                            };
                            match &v33 {
                                US0::US0_Refused(v34) => {
                                    let mut v34: Rc<str> = v34.clone();
                                    US0::US0_Refused(v34.clone())
                                }
                                US0::US0_Rendered(v36) => {
                                    let mut v36: Rc<str> = v36.clone();
                                    recorded_lines_6(v0.clone(), v25.clone(), v36.clone())
                                }
                            }
                        }
                    }
                }
                US1::US1_StateUnreadable(v12) => {
                    let mut v12: Rc<str> = v12.clone();
                    US0::US0_Refused(v12.clone())
                }
            }
        }
        US1::US1_StateUnreadable(v5) => {
            let mut v5: Rc<str> = v5.clone();
            US0::US0_Refused(v5.clone())
        }
    }
}
fn bulleted_8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: i32 = (&*v0).lines().count() as i32;
        let mut v4: bool = v1 < v3;
        if v4 {
            let mut v5: i32 = v1.wrapping_add(1i32);
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n- "); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
            let mut v8: Rc<str> = std::rc::Rc::<str>::from((&*v0).lines().nth(v1 as usize).unwrap_or(""));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
            (v0, v1, v2) = (v0.clone(), v5, v11.clone());
            continue;
        } else {
            return v2.clone();
        }
    }
}
fn active_block_7(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: US1 = state_read_of_2(v0.clone(), v1.clone());
    match &v2 {
        US1::US1_StateMissing => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            US0::US0_Rendered(v3.clone())
        }
        US1::US1_StateText(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active work"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = handoff_active_work_projection_domain::eoie_handoff_active_work_projection(&*v7);
            let mut v10: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: US0 = if v10 {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active work projection rejected typed state"); } LIT.with(|lit| lit.clone()) };
                US0::US0_Refused(v12.clone())
            } else {
                US0::US0_Rendered(v9.clone())
            };
            match &v15 {
                US0::US0_Refused(v16) => {
                    let mut v16: Rc<str> = v16.clone();
                    US0::US0_Refused(v16.clone())
                }
                US0::US0_Rendered(v18) => {
                    let mut v18: Rc<str> = v18.clone();
                    let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                    if v19 {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        US0::US0_Rendered(v20.clone())
                    } else {
                        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v23: bool = v18.starts_with(&*v22);
                        if v23 {
                            let mut v24: Rc<str> = std::rc::Rc::<str>::from(&(&*v18)[3i32 as usize..]);
                            let mut v25: Rc<str> = { let mut items = (&*v24).lines().filter(|line| !line.is_empty()).collect::<Vec<_>>(); items.sort(); std::rc::Rc::<str>::from(items.join("
")) };
                            let mut v26: bool = v25.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v31: Rc<str> = if v26 {
                                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v27.clone()
                            } else {
                                let mut v28: i32 = 0i32;
                                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n## Active work\n"); } LIT.with(|lit| lit.clone()) };
                                bulleted_8(v25.clone(), v28, v29.clone())
                            };
                            US0::US0_Rendered(v31.clone())
                        } else {
                            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active work projection returned invalid envelope"); } LIT.with(|lit| lit.clone()) };
                            US0::US0_Refused(v33.clone())
                        }
                    }
                }
            }
        }
        US1::US1_StateUnreadable(v5) => {
            let mut v5: Rc<str> = v5.clone();
            US0::US0_Refused(v5.clone())
        }
    }
}
fn handoff_text_1(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/handoff.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: US1 = state_read_of_2(v0.clone(), v1.clone());
    match &v2 {
        US1::US1_StateMissing => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/handoff.spi is missing"); } LIT.with(|lit| lit.clone()) };
            US0::US0_Refused(v3.clone())
        }
        US1::US1_StateText(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: US0 = sections_block_4(v7.clone());
            match &v8 {
                US0::US0_Refused(v9) => {
                    let mut v9: Rc<str> = v9.clone();
                    US0::US0_Refused(v9.clone())
                }
                US0::US0_Rendered(v11) => {
                    let mut v11: Rc<str> = v11.clone();
                    let mut v12: US0 = recorded_block_5(v0.clone());
                    match &v12 {
                        US0::US0_Refused(v13) => {
                            let mut v13: Rc<str> = v13.clone();
                            US0::US0_Refused(v13.clone())
                        }
                        US0::US0_Rendered(v15) => {
                            let mut v15: Rc<str> = v15.clone();
                            let mut v16: US0 = active_block_7(v0.clone());
                            match &v16 {
                                US0::US0_Refused(v17) => {
                                    let mut v17: Rc<str> = v17.clone();
                                    US0::US0_Refused(v17.clone())
                                }
                                US0::US0_Rendered(v19) => {
                                    let mut v19: Rc<str> = v19.clone();
                                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# EOIE handoff\n"); } LIT.with(|lit| lit.clone()) };
                                    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v15));
                                    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v11));
                                    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v19));
                                    US0::US0_Rendered(v23.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
        US1::US1_StateUnreadable(v5) => {
            let mut v5: Rc<str> = v5.clone();
            US0::US0_Refused(v5.clone())
        }
    }
}
fn handoff_render_owned_0(mut v0: Rc<str>) -> i32 {
    let mut v1: US0 = handoff_text_1(v0.clone());
    match &v1 {
        US0::US0_Refused(v2) => {
            let mut v2: Rc<str> = v2.clone();
            eprintln!("eoie error: {}", &*v2);
            2i32
        }
        US0::US0_Rendered(v3) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = { let mut stdout = std::io::stdout().lock(); match std::io::Write::write_all(&mut stdout, (&*v3).as_bytes()).and_then(|()| std::io::Write::flush(&mut stdout)) { Err(error) if error.kind() != std::io::ErrorKind::BrokenPipe => std::rc::Rc::<str>::from(format!("write handoff: {error}")), _ => { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) } } };
            let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v5 {
                0i32
            } else {
                eprintln!("eoie error: {}", &*v4);
                2i32
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        handoff_render_owned_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn handoff_render_owned(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
