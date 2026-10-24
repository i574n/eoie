#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn agile_init_entry_0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v3));
    let mut v5: bool = std::path::Path::new(&*v4).exists();
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi already exists"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
        let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v6));
        let mut v12: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v11 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v12
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v13));
        let mut v15: bool = std::path::Path::new(&*v14).exists();
        if v15 {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj already exists"); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v1));
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
            let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v16));
            let mut v22: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v21 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v22
        } else {
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/prompt.spi"); } LIT.with(|lit| lit.clone()) };
            let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v23));
            let mut v25: bool = std::path::Path::new(&*v24).exists();
            if v25 {
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/prompt.spi already exists"); } LIT.with(|lit| lit.clone()) };
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v1));
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
                let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v26));
                let mut v32: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v31 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v32
            } else {
                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
                let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v33));
                let mut v35: bool = std::fs::create_dir_all(&*v34).is_ok();
                if v35 {
                    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v13));
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    agile\n    prompt\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v38: bool = std::fs::write(&*v36, &*v37).is_ok();
                    if v38 {
                        let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v3));
                        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\nunion agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v49: bool = std::fs::write(&*v39, &*v48).is_ok();
                        if v49 {
                            let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v23));
                            let mut v51: u64 = v2.wrapping_sub(5100000u64);
                            let mut v52: Rc<str> = std::rc::Rc::<str>::from((v51).to_string());
                            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl current () : prompt_lease = PromptLease (\"init\", \"scaffolded by agile init; agile begin opens a lease\", 4500000u64, 600000u64, "); } LIT.with(|lit| lit.clone()) };
                            let mut v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v52));
                            let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) };
                            let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v55));
                            let mut v57: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
                            let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, LeaseActive)\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
                            let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union lease_status = | LeaseActive :: lease_status | LeaseClosed :: lease_status\nunion prompt_lease = | PromptLease :: string * string * u64 * u64 * u64 * u64 * lease_status -> prompt_lease\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v60));
                            let mut v63: bool = std::fs::write(&*v50, &*v62).is_ok();
                            if v63 {
                                let mut v64: u64 = v37.len() as u64;
                                let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_kind =\n    | Epic :: task_kind\n    | Feature :: task_kind\n    | Story :: task_kind\n    | TaskKind :: task_kind\n\nunion priority =\n    | P0 :: priority | P1 :: priority | P2 :: priority | P3 :: priority | P4 :: priority | P5 :: priority\n\nunion difficulty =\n    | D0 :: difficulty | D1 :: difficulty | D2 :: difficulty | D3 :: difficulty | D4 :: difficulty\n    | D5 :: difficulty | D6 :: difficulty | D7 :: difficulty | D8 :: difficulty\n\nunion task_status =\n    | Planned :: task_status | Active :: task_status | Blocked :: task_status | Paused :: task_status | Done :: task_status\n\nunion agile_item =\n    | Task :: string * string * task_kind * priority * difficulty * u32 * task_status * string * string * string * string -> agile_item\n\ninl main () : i32 = 0i32\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v69: u64 = v68.len() as u64;
                                let mut v70: Rc<str> = std::rc::Rc::<str>::from((v51).to_string());
                                let mut v71: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v70));
                                let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v55));
                                let mut v73: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                                let mut v74: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v73));
                                let mut v75: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v59));
                                let mut v76: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v75));
                                let mut v77: u64 = v76.len() as u64;
                                let mut v78: u64 = v64 + v69 + v77;
                                let mut v79: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
                                let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile init ok root="); } LIT.with(|lit| lit.clone()) };
                                let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v1));
                                let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files=3 bytes="); } LIT.with(|lit| lit.clone()) };
                                let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                                let mut v84: Rc<str> = std::rc::Rc::<str>::from((v78).to_string());
                                let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v84));
                                let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                let mut v87: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v86));
                                let mut v88: Rc<str> = std::rc::Rc::<str>::from((v79).to_string());
                                let mut v89: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v88));
                                let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" next=eoie agile begin <root> <title>"); } LIT.with(|lit| lit.clone()) };
                                let mut v91: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v90));
                                let mut v92: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v91.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                v92
                            } else {
                                let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/prompt.spi"); } LIT.with(|lit| lit.clone()) };
                                let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                                let mut v95: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v1));
                                let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let mut v97: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v96));
                                let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v93));
                                let mut v99: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v98 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v99
                            }
                        } else {
                            let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                            let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                            let mut v103: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v1));
                            let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let mut v105: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v104));
                            let mut v106: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v101));
                            let mut v107: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v106 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v107
                        }
                    } else {
                        let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
                        let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                        let mut v111: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v1));
                        let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let mut v113: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v112));
                        let mut v114: Rc<str> = Rc::<str>::from(format!("{}{}", v113, v109));
                        let mut v115: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v114 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v115
                    }
                } else {
                    let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not create the state directory"); } LIT.with(|lit| lit.clone()) };
                    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile init "); } LIT.with(|lit| lit.clone()) };
                    let mut v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v1));
                    let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v121: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v120));
                    let mut v122: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v117));
                    let mut v123: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v122 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v123
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_init_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_init_run() -> i32 {
    closure0()()
}
