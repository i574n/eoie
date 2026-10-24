#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_NoFailures,
    UH0_FailedThen(Rc<str>, i32, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_NoMoreNotebooks,
    UH1_SpiralTestNotebook(Rc<str>, Rc<str>, Rc<UH1>),
}
fn announced_1(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (&*v0).lines().count() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: Rc<str> = std::rc::Rc::<str>::from((&*v0).lines().nth(v1 as usize).unwrap_or(""));
            let mut v5: i32 = { std::io::Write::write_all(&mut std::io::stdout(),format!("spiral test blocks {}
",&*v4).as_bytes()).unwrap(); 0i32 };
            let mut v6: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v6);
            continue;
        } else {
            return 0i32;
        }
    }
}
fn first_code_3(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_FailedThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let mut v4: Rc<UH0> = v4.clone();
                (v0, v1) = (v4.clone(), v3);
                continue;
            }
            UH0::UH0_NoFailures => {
                return v1;
            }
        }
    }
}
fn failure_list_4(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_FailedThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = if v5 {
                    v2.clone()
                } else {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
                    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
                    v8.clone()
                };
                (v0, v1) = (v4.clone(), v9.clone());
                continue;
            }
            UH0::UH0_NoFailures => {
                return v1.clone();
            }
        }
    }
}
fn failure_count_5(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_FailedThen(v2, v3, v4) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_NoFailures => {
                return v1;
            }
        }
    }
}
fn reported_2(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    match &*v0 {
        UH0::UH0_NoFailures => {
            let mut v2: bool = v1 == 0i32;
            if v2 {
                ()
            } else {
                eprintln!("spiral notebooks: every notebook reported passing, but the batch exited {}", v1);
                ()
            };
            v1
        }
        _ => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = failure_list_4(v0.clone(), v3.clone());
            let mut v5: i32 = 0i32;
            let mut v6: i32 = failure_count_5(v0.clone(), v5);
            eprintln!("spiral notebooks failed ({}): {}", v6, &*v4);
            let mut v7: i32 = 1i32;
            first_code_3(v0.clone(), v7)
        }
    }
}
fn selected_notebooks_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH1>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match &*v3 {
            UH1::UH1_NoMoreNotebooks => {
                return v4.clone();
            }
            UH1::UH1_SpiralTestNotebook(v5, v6, v7) => {
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let mut v8: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v14: bool = if v8 {
                    true
                } else {
                    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v5));
                    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v2));
                    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
                    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v2));
                    let mut v13: bool = v12.contains(&*v10);
                    v13
                };
                let mut v16: bool = if v14 {
                    let mut v15: bool = std::path::Path::new(&*v0).join(&*v6).is_file();
                    v15
                } else {
                    false
                };
                let mut v22: Rc<str> = if v16 {
                    let mut v17: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    if v17 {
                        v6.clone()
                    } else {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v18));
                        let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v6));
                        v20.clone()
                    }
                } else {
                    v4.clone()
                };
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v7.clone(), v22.clone());
                continue;
            }
        }
    }
}
fn run_batch_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> (i32, std::rc::Rc<str>) {
    let mut v4: (i32, std::rc::Rc<str>) = { let root=std::path::Path::new(&*v0); let target=std::path::Path::new(&*v1); let cells_root=std::env::var_os("EOIE_SPIRAL_CELLS").map(std::path::PathBuf::from).or_else(|| std::env::var_os("EOIE_SPIRAL_BUNDLE").map(|bundle| std::path::PathBuf::from(bundle).join("..").join("..").join("cells"))).unwrap_or_else(|| root.join("..").join("spiral").join("apps").join("cells")); let relatives=(&*v2).lines().map(str::to_owned).collect::<Vec<_>>(); let reports_dir=target.join("spiral-tests"); let _=std::fs::create_dir_all(&reports_dir); let eoie=target.join("debug").join(format!("eoie{}",std::env::consts::EXE_SUFFIX)); let port=20000+std::process::id()%10000; let mut command=std::process::Command::new("pwsh"); command.current_dir(root).arg("-NoProfile").arg("-File").arg(cells_root.join("spi").join("run_notebook.ps1")); for relative in &relatives { command.arg("--path").arg(root.join(relative)); } command.arg("--output-dir").arg(&reports_dir).arg("--no-html").arg("--gate").env("EOIE_BIN_UNDER_TEST",&eoie).env("SPIRAL_CELLS_WARM","0").env("SPIRAL_CELLS_MEMO","0").env_remove("SPIRAL_CELLS_COMPILER").env("SPIRAL_CELLS_COMPILER_PORT",port.to_string()); let bounded=(v3).saturating_mul(relatives.len().max(1) as u64); let stdout_handle=std::io::stdout(); let stderr_handle=std::io::stderr(); let streamed=eoie_process_streaming::run_bounded_streaming(&mut command,bounded,None,|kind,bytes| match kind { eoie_process_streaming::ProcessStreamKind::Stdout => { let mut out=stdout_handle.lock(); std::io::Write::write_all(&mut out,bytes).and_then(|_| std::io::Write::flush(&mut out)).map_err(|error| error.to_string()) } eoie_process_streaming::ProcessStreamKind::Stderr => { let mut err=stderr_handle.lock(); std::io::Write::write_all(&mut err,bytes).and_then(|_| std::io::Write::flush(&mut err)).map_err(|error| error.to_string()) } }); match streamed { Ok(capture) => { let code=if matches!(capture.termination,eoie_process_streaming::ProcessTermination::TimedOut) { 124 } else { capture.status.code().unwrap_or(1) }; let quote=char::from(34u8); let failed_key=format!("{quote}failed{quote}:"); let file_key=format!("{quote}file{quote}:{quote}"); let text=String::from_utf8_lossy(&capture.stdout).replace(char::from(92u8),"/").to_lowercase(); let reports=text.lines().filter_map(|line| line.strip_prefix("spiral-tests file ")).map(|json| { let failed=json.split(failed_key.as_str()).nth(1).map(|rest| rest.chars().take_while(|c| c.is_ascii_digit()).collect::<String>()).unwrap_or_default(); let file=json.split(file_key.as_str()).nth(1).and_then(|rest| rest.split(quote).next()).unwrap_or("").to_owned(); (file,failed) }).collect::<Vec<_>>(); let failing=relatives.iter().filter(|relative| { let wanted=relative.replace(char::from(92u8),"/").to_lowercase(); match reports.iter().find(|(file,_)| file.ends_with(&wanted)) { Some((_,failed)) => failed.as_str() != "0", None => true } }).cloned().collect::<Vec<_>>(); (code,std::rc::Rc::<str>::from(failing.join("
"))) } Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); (2,std::rc::Rc::<str>::from(relatives.join("
"))) } } };
    v4.clone()
}
fn failures_of_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: i32 = (&*v0).lines().count() as i32;
        let mut v5: bool = v1 < v4;
        if v5 {
            let mut v6: i32 = v1.wrapping_add(1i32);
            let mut v7: Rc<str> = std::rc::Rc::<str>::from((&*v0).lines().nth(v1 as usize).unwrap_or(""));
            let mut v8: Rc<UH0> = Rc::new(UH0::UH0_FailedThen(v7.clone(), v2, v3.clone()));
            (v0, v1, v2, v3) = (v0.clone(), v6, v2, v8.clone());
            continue;
        } else {
            return v3.clone();
        }
    }
}
fn run_registered_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64) -> i32 {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-planning-rollup"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_planning_rollup/planning_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-scan"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/todo_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-archive-tar-runtime"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_archive_tar_runtime/format.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-bundle-retention"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_bundle_retention/series.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_bundle_retention/receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-job-host"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_job_host/host_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-bundle-manifest"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_bundle_manifest/bundle_manifest.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-verbs"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_verbs/verbs.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-machine-crowding"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_machine_crowding/crowding.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-repo-affected"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_repo_affected/graph.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("command-spec-domain"); } LIT.with(|lit| lit.clone()) };
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/command_spec_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("canonical-plan-ir-domain"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/canonical_plan_ir_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-views"); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_views/views_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-edit"); } LIT.with(|lit| lit.clone()) };
    let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_edit/edit_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-dev-spiral-tests"); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_dev_spiral_tests/failures.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-handoff-render"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_handoff_render/render.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("authority-state-domain"); } LIT.with(|lit| lit.clone()) };
    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/authority_state_domain/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-note-model"); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_note_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-machine-watch"); } LIT.with(|lit| lit.clone()) };
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_machine_watch/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-window-claim"); } LIT.with(|lit| lit.clone()) };
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_tree_lease/lease.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-adoption"); } LIT.with(|lit| lit.clone()) };
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_adoption/classify.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-process-table"); } LIT.with(|lit| lit.clone()) };
    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_process_table/ps_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-cli"); } LIT.with(|lit| lit.clone()) };
    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_cli_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_cli_model/argument_file.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-jobs"); } LIT.with(|lit| lit.clone()) };
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_jobs/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-repo-state"); } LIT.with(|lit| lit.clone()) };
    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_repo_state/state.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-table-query"); } LIT.with(|lit| lit.clone()) };
    let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_query/query.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-lcov"); } LIT.with(|lit| lit.clone()) };
    let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_lcov/lcov.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_bundle_retention/receipt_read.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-read-query"); } LIT.with(|lit| lit.clone()) };
    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_read_query/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_receipt/receipt.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-program"); } LIT.with(|lit| lit.clone()) };
    let mut v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_program/program.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-read"); } LIT.with(|lit| lit.clone()) };
    let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_read/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_process_table/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-facts"); } LIT.with(|lit| lit.clone()) };
    let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_facts/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-command-line"); } LIT.with(|lit| lit.clone()) };
    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_command_line/launch.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-process"); } LIT.with(|lit| lit.clone()) };
    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_process/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-tables"); } LIT.with(|lit| lit.clone()) };
    let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_tables/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-write"); } LIT.with(|lit| lit.clone()) };
    let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_write/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-git"); } LIT.with(|lit| lit.clone()) };
    let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_git/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-jobs"); } LIT.with(|lit| lit.clone()) };
    let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_jobs/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-do-lab"); } LIT.with(|lit| lit.clone()) };
    let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_do_lab/grammar.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/workspace_primitives/toolchain.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-contracts"); } LIT.with(|lit| lit.clone()) };
    let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_notes/notes_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_contracts/receipts_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-agile-contracts"); } LIT.with(|lit| lit.clone()) };
    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_agile_contracts/agile_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-archive-contracts"); } LIT.with(|lit| lit.clone()) };
    let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_archive_contracts/archive_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-release-contracts"); } LIT.with(|lit| lit.clone()) };
    let mut v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_release_contracts/upgrade_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_cli/windows_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-coverage-prune"); } LIT.with(|lit| lit.clone()) };
    let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/coverage_prune_model/model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_coverage_prune/coverage_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_coverage_prune/evidence_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_release_contracts/hygiene_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-proxy-search"); } LIT.with(|lit| lit.clone()) };
    let mut v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_proxy_search/hash_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_jobs/stop_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-query"); } LIT.with(|lit| lit.clone()) };
    let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_query/query.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-task-next"); } LIT.with(|lit| lit.clone()) };
    let mut v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_next/ranking.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_task_next/next_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_model/roots.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-roots"); } LIT.with(|lit| lit.clone()) };
    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_roots/roots_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_query/stats.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_query/table_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-todo-record"); } LIT.with(|lit| lit.clone()) };
    let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_record/ledger.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_todo_record/record_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-standards"); } LIT.with(|lit| lit.clone()) };
    let mut v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_standards/rules.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-style-census"); } LIT.with(|lit| lit.clone()) };
    let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_style_census/probes.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_style_census/census_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-window-run"); } LIT.with(|lit| lit.clone()) };
    let mut v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_window_run/run_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-table-test"); } LIT.with(|lit| lit.clone()) };
    let mut v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_test/table_test_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-resume"); } LIT.with(|lit| lit.clone()) };
    let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_resume/resume_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-memory"); } LIT.with(|lit| lit.clone()) };
    let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_memory/memory_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-guide"); } LIT.with(|lit| lit.clone()) };
    let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_guide/guide_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-table-logs"); } LIT.with(|lit| lit.clone()) };
    let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_logs/logs_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_logs/logs_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-experiments"); } LIT.with(|lit| lit.clone()) };
    let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_experiment_stats/experiment_stats.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_experiments/experiment_plan.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_experiments/experiment_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_experiments/experiments_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-machine-windows"); } LIT.with(|lit| lit.clone()) };
    let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_machine_windows/window_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_machine_windows/windows_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_standards/standards_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile-memory-links"); } LIT.with(|lit| lit.clone()) };
    let mut v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_memory_links/links_model.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/agile_memory_links/links_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_read_query/grep_cli.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v149: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoMoreNotebooks); } CASE.with(|case| case.clone()) };
    let mut v150: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v62.clone(), v148.clone(), v149.clone()));
    let mut v151: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v145.clone(), v147.clone(), v150.clone()));
    let mut v152: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v145.clone(), v146.clone(), v151.clone()));
    let mut v153: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v118.clone(), v144.clone(), v152.clone()));
    let mut v154: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v141.clone(), v143.clone(), v153.clone()));
    let mut v155: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v141.clone(), v142.clone(), v154.clone()));
    let mut v156: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v136.clone(), v140.clone(), v155.clone()));
    let mut v157: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v136.clone(), v139.clone(), v156.clone()));
    let mut v158: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v136.clone(), v138.clone(), v157.clone()));
    let mut v159: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v136.clone(), v137.clone(), v158.clone()));
    let mut v160: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v133.clone(), v135.clone(), v159.clone()));
    let mut v161: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v133.clone(), v134.clone(), v160.clone()));
    let mut v162: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v131.clone(), v132.clone(), v161.clone()));
    let mut v163: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v129.clone(), v130.clone(), v162.clone()));
    let mut v164: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v127.clone(), v128.clone(), v163.clone()));
    let mut v165: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v125.clone(), v126.clone(), v164.clone()));
    let mut v166: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v123.clone(), v124.clone(), v165.clone()));
    let mut v167: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v120.clone(), v122.clone(), v166.clone()));
    let mut v168: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v120.clone(), v121.clone(), v167.clone()));
    let mut v169: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v118.clone(), v119.clone(), v168.clone()));
    let mut v170: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v115.clone(), v117.clone(), v169.clone()));
    let mut v171: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v115.clone(), v116.clone(), v170.clone()));
    let mut v172: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v57.clone(), v114.clone(), v171.clone()));
    let mut v173: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v57.clone(), v113.clone(), v172.clone()));
    let mut v174: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v111.clone(), v112.clone(), v173.clone()));
    let mut v175: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v8.clone(), v110.clone(), v174.clone()));
    let mut v176: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v107.clone(), v109.clone(), v175.clone()));
    let mut v177: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v107.clone(), v108.clone(), v176.clone()));
    let mut v178: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v105.clone(), v106.clone(), v177.clone()));
    let mut v179: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v53.clone(), v104.clone(), v178.clone()));
    let mut v180: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v102.clone(), v103.clone(), v179.clone()));
    let mut v181: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v94.clone(), v101.clone(), v180.clone()));
    let mut v182: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v97.clone(), v100.clone(), v181.clone()));
    let mut v183: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v97.clone(), v99.clone(), v182.clone()));
    let mut v184: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v97.clone(), v98.clone(), v183.clone()));
    let mut v185: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v50.clone(), v96.clone(), v184.clone()));
    let mut v186: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v94.clone(), v95.clone(), v185.clone()));
    let mut v187: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v92.clone(), v93.clone(), v186.clone()));
    let mut v188: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v90.clone(), v91.clone(), v187.clone()));
    let mut v189: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v87.clone(), v89.clone(), v188.clone()));
    let mut v190: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v87.clone(), v88.clone(), v189.clone()));
    let mut v191: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v50.clone(), v86.clone(), v190.clone()));
    let mut v192: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v84.clone(), v85.clone(), v191.clone()));
    let mut v193: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v82.clone(), v83.clone(), v192.clone()));
    let mut v194: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v80.clone(), v81.clone(), v193.clone()));
    let mut v195: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v78.clone(), v79.clone(), v194.clone()));
    let mut v196: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v76.clone(), v77.clone(), v195.clone()));
    let mut v197: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v74.clone(), v75.clone(), v196.clone()));
    let mut v198: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v72.clone(), v73.clone(), v197.clone()));
    let mut v199: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v70.clone(), v71.clone(), v198.clone()));
    let mut v200: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v48.clone(), v69.clone(), v199.clone()));
    let mut v201: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v67.clone(), v68.clone(), v200.clone()));
    let mut v202: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v65.clone(), v66.clone(), v201.clone()));
    let mut v203: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v62.clone(), v64.clone(), v202.clone()));
    let mut v204: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v62.clone(), v63.clone(), v203.clone()));
    let mut v205: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v13.clone(), v61.clone(), v204.clone()));
    let mut v206: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v59.clone(), v60.clone(), v205.clone()));
    let mut v207: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v57.clone(), v58.clone(), v206.clone()));
    let mut v208: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v55.clone(), v56.clone(), v207.clone()));
    let mut v209: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v53.clone(), v54.clone(), v208.clone()));
    let mut v210: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v50.clone(), v52.clone(), v209.clone()));
    let mut v211: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v50.clone(), v51.clone(), v210.clone()));
    let mut v212: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v48.clone(), v49.clone(), v211.clone()));
    let mut v213: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v46.clone(), v47.clone(), v212.clone()));
    let mut v214: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v44.clone(), v45.clone(), v213.clone()));
    let mut v215: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v42.clone(), v43.clone(), v214.clone()));
    let mut v216: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v40.clone(), v41.clone(), v215.clone()));
    let mut v217: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v38.clone(), v39.clone(), v216.clone()));
    let mut v218: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v36.clone(), v37.clone(), v217.clone()));
    let mut v219: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v34.clone(), v35.clone(), v218.clone()));
    let mut v220: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v32.clone(), v33.clone(), v219.clone()));
    let mut v221: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v30.clone(), v31.clone(), v220.clone()));
    let mut v222: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v28.clone(), v29.clone(), v221.clone()));
    let mut v223: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v26.clone(), v27.clone(), v222.clone()));
    let mut v224: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v24.clone(), v25.clone(), v223.clone()));
    let mut v225: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v22.clone(), v23.clone(), v224.clone()));
    let mut v226: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v20.clone(), v21.clone(), v225.clone()));
    let mut v227: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v18.clone(), v19.clone(), v226.clone()));
    let mut v228: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v16.clone(), v17.clone(), v227.clone()));
    let mut v229: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v13.clone(), v15.clone(), v228.clone()));
    let mut v230: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v13.clone(), v14.clone(), v229.clone()));
    let mut v231: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v11.clone(), v12.clone(), v230.clone()));
    let mut v232: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v8.clone(), v10.clone(), v231.clone()));
    let mut v233: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v8.clone(), v9.clone(), v232.clone()));
    let mut v234: Rc<UH1> = Rc::new(UH1::UH1_SpiralTestNotebook(v6.clone(), v7.clone(), v233.clone()));
    let mut v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v236: Rc<str> = selected_notebooks_6(v0.clone(), v2.clone(), v3.clone(), v234.clone(), v235.clone());
    if v4 {
        let mut v237: i32 = 0i32;
        announced_1(v236.clone(), v237)
    } else {
        let mut v239: bool = v236.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v239 {
            0i32
        } else {
            let mut v240: (i32, std::rc::Rc<str>) = run_batch_7(v0.clone(), v1.clone(), v236.clone(), v5);
            let mut v241: i32 = v240.clone().0;
            let mut v242: Rc<str> = v240.1.clone();
            let mut v243: i32 = 0i32;
            let mut v244: bool = v241 == 0i32;
            let mut v245: i32 = if v244 {
                1i32
            } else {
                v241
            };
            let mut v246: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFailures); } CASE.with(|case| case.clone()) };
            let mut v247: Rc<UH0> = failures_of_8(v242.clone(), v243, v245, v246.clone());
            reported_2(v247.clone(), v241)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64| -> i32 {
        run_registered_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_dev_spiral_tests_run(v0: &str, v1: &str, v2: &str, v3: &str, v4: bool, v5: u64) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, v5)
}
