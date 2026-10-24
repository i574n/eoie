#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_StateWritten,
    US1_StateChangedSinceRead,
    US1_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Replaced,
    US2_TemporaryWriteFailed,
    US2_RenameFailed,
    US2_ReadbackMismatch,
}
#[derive(Clone)]
enum US4 {
    US4_ViewList,
    US4_ViewNext,
}
#[derive(Clone)]
enum US3 {
    US3_VerbRead(US4),
    US3_VerbRefused,
}
#[derive(Clone)]
enum US5 {
    US5_ScanPlain,
    US5_ScanQuoted,
    US5_ScanEscaped,
}
#[derive(Clone)]
enum US7 {
    US7_ViewsCommand,
    US7_ViewCommand,
    US7_ViewAddCommand,
}
#[derive(Clone)]
enum US6 {
    US6_CommandRead(US7),
    US6_CommandUnknown,
}
fn views_loaded_3(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v2));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let mut v7: bool = std::path::Path::new(&*v6).is_file();
    let mut v10: US0 = if v7 {
        US0::US0_FileReadable(v6.clone())
    } else {
        US0::US0_FileMissing(v6.clone())
    };
    let (mut v16, mut v17): (bool, Rc<str>) = match &v10 {
        US0::US0_FileMissing(v11) => {
            let mut v11: Rc<str> = v11.clone();
            (false, v2.clone())
        }
        _ => {
            let (mut v14, mut v15): (bool, Rc<str>) = match &v10 {
                US0::US0_FileReadable(v12) => {
                    let mut v12: Rc<str> = v12.clone();
                    let mut v13: Rc<str> = std::fs::read_to_string(&*v12).unwrap_or_default().into();
                    (true, v13.clone())
                }
                _ => unreachable!(),
            };
            (v14, v15.clone())
        }
    };
    (v16, v17.clone())
}
fn declared_from_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let mut v9: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
}
fn views_written_5(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> US1 {
    let mut v7: bool = if v1 {
        let mut v4: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
        v4
    } else {
        let mut v5: bool = std::path::Path::new(&*v0).is_file();
        let mut v6: bool = v5 == false;
        v6
    };
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v8, &*v9, &*v10, &*v11].concat());
        let mut v13: bool = std::fs::write(&*v12, &*v3).is_ok();
        let mut v22: US2 = if v13 {
            let mut v14: bool = std::fs::rename(&*v12, &*v0).is_ok();
            if v14 {
                let mut v15: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v3);
                if v15 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                US2::US2_RenameFailed
            }
        } else {
            US2::US2_TemporaryWriteFailed
        };
        match &v22 {
            US2::US2_Replaced => {
                US1::US1_StateWritten
            }
            _ => {
                let mut v29: Rc<str> = match &v22 {
                    US2::US2_ReadbackMismatch => {
                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v26.clone()
                    }
                    US2::US2_RenameFailed => {
                        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v25.clone()
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v24.clone()
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v29.clone())
            }
        }
    } else {
        US1::US1_StateChangedSinceRead
    }
}
fn view_stored_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let (mut v4, mut v5): (bool, Rc<str>) = views_loaded_3(v0.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View ("); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v8, &v9));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from(v10.replace(&*v7, &v11));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v12.replace(&*v13, &v14));
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v15.replace(&*v16, &v17));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = std::rc::Rc::<str>::from(v18.replace(&*v19, &v20));
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v21, &*v7, &*v22, &*v22].concat());
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v23));
    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
    let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v22));
    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v22));
    let mut v29: i32 = 0i32;
    let mut v30: i32 = declared_from_4(v5.clone(), v28.clone(), v29);
    let mut v31: bool = v30 < 0i32;
    let mut v32: bool = v31 == false;
    let mut v130: Rc<str> = if v32 {
        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view_"); } LIT.with(|lit| lit.clone()) };
        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
        let mut v37: Rc<str> = Rc::<str>::from(v1.replace(&*v35, &*v36));
        let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v37));
        let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v22));
        let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v22));
        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v22));
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = View ("); } LIT.with(|lit| lit.clone()) };
        let mut v43: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v8, &v9));
        let mut v44: Rc<str> = std::rc::Rc::<str>::from(v43.replace(&*v7, &v11));
        let mut v45: Rc<str> = std::rc::Rc::<str>::from(v44.replace(&*v13, &v14));
        let mut v46: Rc<str> = std::rc::Rc::<str>::from(v45.replace(&*v16, &v17));
        let mut v47: Rc<str> = std::rc::Rc::<str>::from(v46.replace(&*v19, &v20));
        let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v47, &*v7, &*v22, &*v22].concat());
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v41));
        let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v42));
        let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v48));
        let mut v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v49));
        let mut v54: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v8, &v9));
        let mut v55: Rc<str> = std::rc::Rc::<str>::from(v54.replace(&*v7, &v11));
        let mut v56: Rc<str> = std::rc::Rc::<str>::from(v55.replace(&*v13, &v14));
        let mut v57: Rc<str> = std::rc::Rc::<str>::from(v56.replace(&*v16, &v17));
        let mut v58: Rc<str> = std::rc::Rc::<str>::from(v57.replace(&*v19, &v20));
        let mut v59: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v58, &*v7, &*v22, &*v22].concat());
        let mut v60: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v8, &v9));
        let mut v61: Rc<str> = std::rc::Rc::<str>::from(v60.replace(&*v7, &v11));
        let mut v62: Rc<str> = std::rc::Rc::<str>::from(v61.replace(&*v13, &v14));
        let mut v63: Rc<str> = std::rc::Rc::<str>::from(v62.replace(&*v16, &v17));
        let mut v64: Rc<str> = std::rc::Rc::<str>::from(v63.replace(&*v19, &v20));
        let mut v65: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v64, &*v7, &*v22, &*v22].concat());
        let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v67: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v59));
        let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v49));
        let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v65));
        let mut v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v66));
        let mut v71: i32 = v5.get(..v30 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
        let mut v72: i32 = v5.get(v30 as usize..).and_then(|rest| rest.find(&*v13)).map(|found| (found + v30 as usize) as i32).unwrap_or(-1);
        let mut v73: bool = v72 < 0i32;
        let mut v75: i32 = if v73 {
            let mut v74: i32 = (v5.clone().len() as i32);
            v74
        } else {
            v72
        };
        let mut v76: Rc<str> = std::rc::Rc::<str>::from(v5.get(0i32 as usize..v71 as usize).unwrap_or(""));
        let mut v77: i32 = (v5.clone().len() as i32);
        let mut v78: Rc<str> = std::rc::Rc::<str>::from(v5.get(v75 as usize..v77 as usize).unwrap_or(""));
        let mut v79: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v70));
        let mut v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v78));
        let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v22));
        let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v22));
        v82.clone()
    } else {
        let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view_"); } LIT.with(|lit| lit.clone()) };
        let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
        let mut v87: Rc<str> = Rc::<str>::from(v1.replace(&*v85, &*v86));
        let mut v88: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v87));
        let mut v89: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v22));
        let mut v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v22));
        let mut v91: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v22));
        let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = View ("); } LIT.with(|lit| lit.clone()) };
        let mut v93: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v8, &v9));
        let mut v94: Rc<str> = std::rc::Rc::<str>::from(v93.replace(&*v7, &v11));
        let mut v95: Rc<str> = std::rc::Rc::<str>::from(v94.replace(&*v13, &v14));
        let mut v96: Rc<str> = std::rc::Rc::<str>::from(v95.replace(&*v16, &v17));
        let mut v97: Rc<str> = std::rc::Rc::<str>::from(v96.replace(&*v19, &v20));
        let mut v98: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v97, &*v7, &*v22, &*v22].concat());
        let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let mut v100: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v91));
        let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v92));
        let mut v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v98));
        let mut v103: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v99));
        let mut v104: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v8, &v9));
        let mut v105: Rc<str> = std::rc::Rc::<str>::from(v104.replace(&*v7, &v11));
        let mut v106: Rc<str> = std::rc::Rc::<str>::from(v105.replace(&*v13, &v14));
        let mut v107: Rc<str> = std::rc::Rc::<str>::from(v106.replace(&*v16, &v17));
        let mut v108: Rc<str> = std::rc::Rc::<str>::from(v107.replace(&*v19, &v20));
        let mut v109: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v108, &*v7, &*v22, &*v22].concat());
        let mut v110: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v8, &v9));
        let mut v111: Rc<str> = std::rc::Rc::<str>::from(v110.replace(&*v7, &v11));
        let mut v112: Rc<str> = std::rc::Rc::<str>::from(v111.replace(&*v13, &v14));
        let mut v113: Rc<str> = std::rc::Rc::<str>::from(v112.replace(&*v16, &v17));
        let mut v114: Rc<str> = std::rc::Rc::<str>::from(v113.replace(&*v19, &v20));
        let mut v115: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v114, &*v7, &*v22, &*v22].concat());
        let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let mut v117: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v109));
        let mut v118: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v99));
        let mut v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v115));
        let mut v120: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v116));
        let mut v121: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v123: Rc<str> = if v121 {
            let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union agile_view =\n    | View :: string * string * string -> agile_view\n"); } LIT.with(|lit| lit.clone()) };
            v122.clone()
        } else {
            v5.clone()
        };
        let mut v124: bool = v123.ends_with(&*v13);
        let mut v125: Rc<str> = if v124 {
            v22.clone()
        } else {
            v13.clone()
        };
        let mut v126: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v125));
        let mut v127: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v120));
        let mut v128: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v13));
        let mut v129: Rc<str> = Rc::<str>::from(format!("{}{}", v128, v22));
        v129.clone()
    };
    let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v132: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v131));
    let mut v133: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v22));
    let mut v134: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v22));
    let mut v135: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v22));
    let mut v136: US1 = views_written_5(v135.clone(), v4, v5.clone(), v130.clone());
    match &v136 {
        US1::US1_StateWritten => {
            let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" verb="); } LIT.with(|lit| lit.clone()) };
            let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" flags='"); } LIT.with(|lit| lit.clone()) };
            let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
            let mut v140: Rc<str> = Rc::<str>::from(format!("{}{}", v137, v2));
            let mut v141: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v138));
            let mut v142: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v3));
            let mut v143: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v139));
            let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile view-add ok name="); } LIT.with(|lit| lit.clone()) };
            let mut v147: Rc<str> = if v32 {
                let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" replaced=yes"); } LIT.with(|lit| lit.clone()) };
                v145.clone()
            } else {
                let mut v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" replaced=no"); } LIT.with(|lit| lit.clone()) };
                v146.clone()
            };
            let mut v148: Rc<str> = Rc::<str>::from(format!("{}{}", v144, v1));
            let mut v149: Rc<str> = Rc::<str>::from(format!("{}{}", v148, v143));
            let mut v150: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v147));
            let mut v151: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v22));
            let mut v152: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v151.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v152
        }
        _ => {
            let mut v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add "); } LIT.with(|lit| lit.clone()) };
            let mut v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v158: Rc<str> = match &v136 {
                US1::US1_StateChangedSinceRead => {
                    let mut v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                    v155.clone()
                }
                US1::US1_StateWriteFailed(v156) => {
                    let mut v156: Rc<str> = v156.clone();
                    v156.clone()
                }
                _ => unreachable!(),
            };
            let mut v159: Rc<str> = Rc::<str>::from(format!("{}{}", v153, v1));
            let mut v160: Rc<str> = Rc::<str>::from(format!("{}{}", v159, v154));
            let mut v161: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v158));
            let mut v162: Rc<str> = Rc::<str>::from(format!("{}{}", v161, v22));
            let mut v163: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v162 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v163
        }
    }
}
fn view_add_run_1() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: bool = !(v3).is_empty() && (v3).bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == 45u8);
    if v4 {
        let mut v5: i32 = 3i32;
        let mut v6: Rc<str> = usize::try_from(v5).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v7: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) } == v6.clone();
        let mut v15: US3 = if v7 {
            let mut v8: US4 = US4::US4_ViewList;
            US3::US3_VerbRead(v8.clone())
        } else {
            let mut v10: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) } == v6.clone();
            if v10 {
                let mut v11: US4 = US4::US4_ViewNext;
                US3::US3_VerbRead(v11.clone())
            } else {
                US3::US3_VerbRefused
            }
        };
        match &v15 {
            US3::US3_VerbRead(v24) => {
                let mut v24: US4 = v24.clone();
                let mut v25: bool = std::env::args().skip(6i32 as usize).all(|argument| !argument.is_empty() && !argument.contains(char::is_whitespace));
                if v25 {
                    let mut v29: Rc<str> = match &v24 {
                        US4::US4_ViewList => {
                            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                            v26.clone()
                        }
                        US4::US4_ViewNext => {
                            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
                            v27.clone()
                        }
                    };
                    let mut v30: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(6i32 as usize).collect::<Vec<_>>().join(" ").as_str());
                    view_stored_2(v1.clone(), v3.clone(), v29.clone(), v30.clone())
                } else {
                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: each stored flag must be one word without spaces"); } LIT.with(|lit| lit.clone()) };
                    let mut v33: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v32 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v33
                }
            }
            US3::US3_VerbRefused => {
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view verbs are "); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list|next"); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view verbs are list|next"); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view verbs are list|next"); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view verbs are list|next"); } LIT.with(|lit| lit.clone()) };
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view verbs are list|next"); } LIT.with(|lit| lit.clone()) };
                let mut v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v23
            }
        }
    } else {
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view names are lowercase letters, digits and hyphens"); } LIT.with(|lit| lit.clone()) };
        let mut v38: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v37 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v38
    }
}
fn field_end_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US5) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US5::US5_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let mut v10: i32 = v1.wrapping_add(1i32);
                let mut v25: US5 = match &v3 {
                    US5::US5_ScanEscaped => {
                        US5::US5_ScanQuoted
                    }
                    US5::US5_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US5::US5_ScanQuoted
                        } else {
                            US5::US5_ScanPlain
                        }
                    }
                    US5::US5_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US5::US5_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US5::US5_ScanPlain
                            } else {
                                US5::US5_ScanQuoted
                            }
                        }
                    }
                };
                (v0, v1, v2, v3) = (v0.clone(), v10, v2, v25.clone());
                continue;
            }
        }
    }
}
fn skip_spaces_10(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US5 = US5::US5_ScanPlain;
            let mut v6: i32 = field_end_9(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_10(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn unescaped_from_12(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v6, &*v7, &*v7, &*v7].concat());
            return v8.clone();
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v10: bool = v9 == 92i32;
            if v10 {
                let mut v11: i32 = v1.wrapping_add(1i32);
                let mut v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v15: bool = v14 == 110i32;
                let mut v24: Rc<str> = if v15 {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let mut v19: bool = v14 == 116i32;
                        if v19 {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21.clone()
                        }
                    }
                };
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v13, &*v24, &*v25, &*v25].concat());
                (v0, v1, v2, v3, v4) = (v0.clone(), v12, v12, v3, v26.clone());
                continue;
            } else {
                let mut v28: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v28, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn unquoted_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_12(v3.clone(), v4, v5, v6, v7.clone())
}
fn view_at_7(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let mut v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = 0i32;
    let mut v5: i32 = field_start_8(v0.clone(), v2, v3, v4);
    let mut v6: US5 = US5::US5_ScanPlain;
    let mut v7: i32 = field_end_9(v0.clone(), v5, v3, v6.clone());
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_11(v8.clone());
    let mut v10: i32 = 1i32;
    let mut v11: i32 = field_start_8(v0.clone(), v2, v3, v10);
    let mut v12: US5 = US5::US5_ScanPlain;
    let mut v13: i32 = field_end_9(v0.clone(), v11, v3, v12.clone());
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let mut v15: Rc<str> = unquoted_11(v14.clone());
    let mut v16: i32 = 2i32;
    let mut v17: i32 = field_start_8(v0.clone(), v2, v3, v16);
    let mut v18: US5 = US5::US5_ScanPlain;
    let mut v19: i32 = field_end_9(v0.clone(), v17, v3, v18.clone());
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let mut v21: Rc<str> = unquoted_11(v20.clone());
    (v9.clone(), v15.clone(), v21.clone())
}
fn view_run_6() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile view <root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
        let mut v6: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v5 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v6
    } else {
        let (mut v7, mut v8): (bool, Rc<str>) = views_loaded_3(v1.clone());
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View ("); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v11, &v12));
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = std::rc::Rc::<str>::from(v13.replace(&*v10, &v14));
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = std::rc::Rc::<str>::from(v15.replace(&*v16, &v17));
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = std::rc::Rc::<str>::from(v18.replace(&*v19, &v20));
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = std::rc::Rc::<str>::from(v21.replace(&*v22, &v23));
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v24, &*v10, &*v25, &*v25].concat());
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v26));
        let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v27));
        let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v25));
        let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v25));
        let mut v32: i32 = 0i32;
        let mut v33: i32 = declared_from_4(v8.clone(), v31.clone(), v32);
        let mut v34: bool = v33 < 0i32;
        if v34 {
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view: no view named "); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
            let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v37));
            let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v25));
            let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v25));
            let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v25));
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; agile views <root> lists them"); } LIT.with(|lit| lit.clone()) };
            let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v3));
            let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v36));
            let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v41));
            let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v42));
            let mut v47: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v46 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v47
        } else {
            let (mut v48, mut v49, mut v50): (Rc<str>, Rc<str>, Rc<str>) = view_at_7(v8.clone(), v33);
            let mut v51: i32 = { let extra = std::env::args().skip(5i32 as usize).collect::<Vec<_>>(); match std::env::current_exe().and_then(|exe| std::process::Command::new(exe).arg("agile").arg(&*v49).arg(&*v1).args((v50).split_whitespace()).args(&extra).status()) { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("eoie error: agile view could not start: {}", error); 2 } } };
            v51
        }
    }
}
fn views_listed_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View ("); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View (\""); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View (\""); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View (\""); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View (\""); } LIT.with(|lit| lit.clone()) };
        let mut v10: i32 = declared_from_4(v0.clone(), v9.clone(), v1);
        let mut v11: bool = v10 < 0i32;
        if v11 {
            return v2;
        } else {
            let (mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, Rc<str>) = view_at_7(v0.clone(), v10);
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v15));
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v13));
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v15));
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v14));
            let mut v20: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let mut v21: i32 = v10.wrapping_add(1i32);
            let mut v22: i32 = v2.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v21, v22);
            continue;
        }
    }
}
fn views_run_13() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let (mut v2, mut v3): (bool, Rc<str>) = views_loaded_3(v1.clone());
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = views_listed_14(v3.clone(), v4, v5);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile views ok views="); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v6).to_string().as_str());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v10));
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v11));
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v11));
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v9));
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v15));
    let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v11));
    let mut v20: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v20
}
fn views_entry_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("views"); } LIT.with(|lit| lit.clone()) } == v1.clone();
    let mut v14: US6 = if v2 {
        let mut v3: US7 = US7::US7_ViewsCommand;
        US6::US6_CommandRead(v3.clone())
    } else {
        let mut v5: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view"); } LIT.with(|lit| lit.clone()) } == v1.clone();
        if v5 {
            let mut v6: US7 = US7::US7_ViewCommand;
            US6::US6_CommandRead(v6.clone())
        } else {
            let mut v8: bool = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view-add"); } LIT.with(|lit| lit.clone()) } == v1.clone();
            if v8 {
                let mut v9: US7 = US7::US7_ViewAddCommand;
                US6::US6_CommandRead(v9.clone())
            } else {
                US6::US6_CommandUnknown
            }
        }
    };
    match &v14 {
        US6::US6_CommandRead(v15) => {
            let mut v15: US7 = v15.clone();
            match &v15 {
                US7::US7_ViewAddCommand => {
                    view_add_run_1()
                }
                US7::US7_ViewCommand => {
                    view_run_6()
                }
                US7::US7_ViewsCommand => {
                    views_run_13()
                }
            }
        }
        US6::US6_CommandUnknown => {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile views: expected views, view or view-add"); } LIT.with(|lit| lit.clone()) };
            let mut v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v23
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        views_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_views_run() -> i32 {
    closure0()()
}
