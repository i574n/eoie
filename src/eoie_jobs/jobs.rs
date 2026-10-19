#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_JobPending,
    US0_JobRunning,
    US0_JobFinished(i32),
    US0_JobTimedOut,
    US0_JobSpawnFailed(Rc<str>),
    US0_JobLost,
}
#[derive(Clone)]
enum US1 {
    US1_PlainStart,
    US1_HeavyStart,
}
#[derive(Clone)]
enum US2 {
    US2_TreeStopped,
    US2_SurvivorsLeft(u64),
}
#[derive(Clone)]
enum US3 {
    US3_JobStart,
    US3_JobSupervise,
    US3_JobStatus,
    US3_JobWait,
    US3_JobList,
    US3_JobTail,
    US3_JobUnknown,
}
fn decimal_u64_2(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = v0.trim().parse::<u64>().unwrap_or(0);
    v1
}
fn jobs_root_3() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_state_of_7(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US0 {
    let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v4 != true;
    if v5 {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v7: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
        if v7 {
            let mut v8: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
            US0::US0_JobFinished(v8)
        } else {
            let mut v10: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            if v10 {
                US0::US0_JobTimedOut
            } else {
                let mut v12: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                US0::US0_JobSpawnFailed(v12.clone())
            }
        }
    } else {
        let mut v16: bool = v1 > 0u64;
        if v16 {
            let mut v17: u64 = v1.wrapping_add(15000u64);
            let mut v18: bool = v17 < v3;
            if v18 {
                US0::US0_JobLost
            } else {
                US0::US0_JobRunning
            }
        } else {
            let mut v22: u64 = v2.wrapping_add(15000u64);
            let mut v23: bool = v22 < v3;
            if v23 {
                US0::US0_JobLost
            } else {
                US0::US0_JobPending
            }
        }
    }
}
fn joined_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn job_directory_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_3();
    joined_10(v1.clone(), v0.clone())
}
fn job_file_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_11(v0.clone());
    joined_10(v2.clone(), v1.clone())
}
fn text_or_empty_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn now_ms_13() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_8(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_9(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_12(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_9(v0.clone(), v4.clone());
    let mut v6: Rc<str> = text_or_empty_12(v5.clone());
    let mut v7: u64 = decimal_u64_2(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_9(v0.clone(), v8.clone());
    let mut v10: Rc<str> = text_or_empty_12(v9.clone());
    let mut v11: u64 = decimal_u64_2(v10.clone());
    let mut v12: u64 = now_ms_13();
    (v3.clone(), v7, v11, v12)
}
fn job_state_now_6(mut v0: Rc<str>) -> US0 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_8(v0.clone());
    job_state_of_7(v1.clone(), v2, v3, v4)
}
fn elapsed_of_14(mut v0: Rc<str>) -> u64 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_8(v0.clone());
    let mut v5: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: u64 = if v5 {
        v4
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.split(char::from(9u8)).nth(1).unwrap_or("0"));
        decimal_u64_2(v6.clone())
    };
    let mut v9: bool = v8 > v3;
    if v9 {
        let mut v10: u64 = v8.wrapping_sub(v3);
        v10
    } else {
        0u64
    }
}
fn decimal_text_15(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn file_bytes_16(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn job_summary_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US0 = job_state_now_6(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok id="); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v0, &*v4, &*v1, &*v5].concat());
    let mut v20: Rc<str> = match &v2 {
        US0::US0_JobFinished(v9) => {
            let mut v9: i32 = *v9;
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_JobLost => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US0::US0_JobPending => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US0::US0_JobRunning => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US0::US0_JobSpawnFailed(v12) => {
            let mut v12: Rc<str> = v12.clone();
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US0::US0_JobTimedOut => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
    };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit="); } LIT.with(|lit| lit.clone()) };
    let mut v29: i32 = match &v2 {
        US0::US0_JobFinished(v22) => {
            let mut v22: i32 = *v22;
            v22
        }
        US0::US0_JobLost => {
            125i32
        }
        US0::US0_JobPending => {
            3i32
        }
        US0::US0_JobRunning => {
            3i32
        }
        US0::US0_JobSpawnFailed(v23) => {
            let mut v23: Rc<str> = v23.clone();
            125i32
        }
        US0::US0_JobTimedOut => {
            124i32
        }
    };
    let mut v30: Rc<str> = std::rc::Rc::<str>::from((v29).to_string());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let mut v32: u64 = elapsed_of_14(v1.clone());
    let mut v33: Rc<str> = decimal_text_15(v32);
    let mut v34: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v21, &*v30, &*v31, &*v33].concat());
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stdout_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = job_file_9(v1.clone(), v36.clone());
    let mut v38: u64 = file_bytes_16(v37.clone());
    let mut v39: Rc<str> = decimal_text_15(v38);
    let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr_bytes="); } LIT.with(|lit| lit.clone()) };
    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
    let mut v42: Rc<str> = job_file_9(v1.clone(), v41.clone());
    let mut v43: u64 = file_bytes_16(v42.clone());
    let mut v44: Rc<str> = decimal_text_15(v43);
    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v39, &*v40, &*v44, &*v45].concat());
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v48: Rc<str> = job_file_9(v1.clone(), v47.clone());
    let mut v49: Rc<str> = text_or_empty_12(v48.clone());
    let mut v50: Rc<str> = std::rc::Rc::<str>::from(v49.split(char::from(9u8)).nth(3).unwrap_or("").trim());
    let mut v51: bool = v50.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = if v51 {
        v45.clone()
    } else {
        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" detail=\""); } LIT.with(|lit| lit.clone()) };
        let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v52, &*v50, &*v53, &*v45, &*v45].concat());
        v54.clone()
    };
    let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let mut v58: Rc<str> = job_file_9(v1.clone(), v57.clone());
    let mut v59: Rc<str> = text_or_empty_12(v58.clone());
    let mut v60: Rc<str> = std::rc::Rc::<str>::from([&*v56, &*v59, &*v45, &*v45, &*v45].concat());
    let mut v61: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v34, &*v46, &*v55, &*v60].concat());
    v61.clone()
}
fn stdout_line_17(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn listed_4(mut v0: Rc<str>) -> () {
    loop {
        let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v1 != true;
        if v2 {
            let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = job_summary_5(v5.clone(), v3.clone());
            stdout_line_17(v6.clone());
            v0 = v4.clone();
            continue;
        }
        return ();
    }
}
fn job_listed_1() -> i32 {
    let mut v0: i32 = std::env::args().count() as i32;
    let mut v1: bool = v0 > 3i32;
    let mut v5: u64 = if v1 {
        let mut v2: i32 = 3i32;
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
        decimal_u64_2(v3.clone())
    } else {
        20u64
    };
    let mut v6: Rc<str> = jobs_root_3();
    let mut v7: Rc<str> = { let mut names = std::fs::read_dir(&*v6).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v5 as usize); std::rc::Rc::<str>::from(names.join("
")) };
    listed_4(v7.clone());
    0i32
}
fn stderr_line_20(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn rejected_19(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_20(v3.clone());
    2i32
}
fn heavy_gate_script_22() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_HEAVY_GATE").map(std::path::PathBuf::from).filter(|path| path.is_file()).or_else(|| std::env::current_dir().ok().and_then(|cwd| cwd.ancestors().map(|base| base.join("spiral").join("apps").join("compiler").join("tmp").join("scripts").join("heavy.ps1")).find(|candidate| candidate.is_file()))).map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
    v0.clone()
}
fn text_written_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn job_argv_21(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    match &v0 {
        US1::US1_HeavyStart => {
            let mut v4: Rc<str> = heavy_gate_script_22();
            let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v5 {
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v6.clone()
            } else {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv.inner"); } LIT.with(|lit| lit.clone()) };
                let mut v8: Rc<str> = job_file_9(v1.clone(), v7.clone());
                let mut v9: Rc<str> = text_written_23(v8.clone(), v3.clone());
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = job_file_9(v1.clone(), v10.clone());
                let mut v12: Rc<str> = job_file_9(v1.clone(), v7.clone());
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$argv = @(Get-Content -LiteralPath '"); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("')\n. '"); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\n"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v12, &*v14, &*v4, &*v15].concat());
                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Invoke-Heavy -Label '"); } LIT.with(|lit| lit.clone()) };
                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (eoie job "); } LIT.with(|lit| lit.clone()) };
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")' { & $argv[0] @($argv | Select-Object -Skip 1) }\n"); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v18, &*v1, &*v19, &*v20].concat());
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit $LASTEXITCODE\n"); } LIT.with(|lit| lit.clone()) };
                let mut v23: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v17, &*v21, &*v22, &*v20].concat());
                let mut v24: Rc<str> = text_written_23(v11.clone(), v23.clone());
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh\n-NoProfile\n-File\n"); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v11, &*v20, &*v20, &*v20].concat());
                v26.clone()
            }
        }
        US1::US1_PlainStart => {
            v3.clone()
        }
    }
}
fn supervisor_executable_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let copy = || -> std::io::Result<std::path::PathBuf> { let source = std::env::current_exe()?; let metadata = std::fs::metadata(&source)?; let stamp = metadata.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|elapsed| elapsed.as_secs()).unwrap_or(0); let folder = std::path::Path::new(&*v0).join("supervisor"); std::fs::create_dir_all(&folder)?; let target = folder.join(format!("eoie-{}-{}{}", metadata.len(), stamp, std::env::consts::EXE_SUFFIX)); if !target.is_file() { let temporary = target.with_extension("tmp"); std::fs::copy(&source, &temporary)?; std::fs::rename(&temporary, &target)?; } Ok(target) }; std::rc::Rc::<str>::from(copy().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default()) };
    v1.clone()
}
fn job_started_18() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy"); } LIT.with(|lit| lit.clone()) };
    let mut v5: US1 = if v2 {
        US1::US1_HeavyStart
    } else {
        US1::US1_PlainStart
    };
    let mut v7: i32 = match &v5 {
        US1::US1_HeavyStart => {
            1i32
        }
        US1::US1_PlainStart => {
            0i32
        }
    };
    let mut v8: i32 = 3i32.wrapping_add(v7);
    let mut v9: i32 = std::env::args().count() as i32;
    let mut v10: i32 = v8.wrapping_add(3i32);
    let mut v11: bool = v9 < v10;
    if v11 {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start [--heavy] <label> <timeout-ms> <program> [args...] | job status <id> | job wait <id> <max-ms> | job list [count] | job tail <id> [lines]"); } LIT.with(|lit| lit.clone()) };
        rejected_19(v12.clone())
    } else {
        let mut v14: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v8 as usize).unwrap_or_default());
        let mut v15: i32 = v8.wrapping_add(1i32);
        let mut v16: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v15 as usize).unwrap_or_default());
        let mut v17: u64 = decimal_u64_2(v16.clone());
        let mut v18: bool = v17 == 0u64;
        if v18 {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout-ms must be a positive integer"); } LIT.with(|lit| lit.clone()) };
            rejected_19(v19.clone())
        } else {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
            let mut v22: u64 = now_ms_13();
            let mut v23: Rc<str> = decimal_text_15(v22);
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = std::rc::Rc::<str>::from(v14.chars().map(|c| if c.is_ascii_alphanumeric() || c == char::from(45u8) { c } else { char::from(95u8) }).take(40).collect::<String>());
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v23, &*v24, &*v25, &*v26].concat());
            let mut v28: Rc<str> = job_directory_11(v27.clone());
            let mut v29: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v28).err().map(|error| format!("create {}: {error}", &*v28)).unwrap_or_default());
            let mut v30: bool = v29.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v31: bool = v30 != true;
            if v31 {
                rejected_19(v29.clone())
            } else {
                let mut v33: i32 = v8.wrapping_add(2i32);
                let mut v34: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(v33 as usize).collect::<Vec<_>>().join("
"));
                let mut v35: Rc<str> = job_argv_21(v5.clone(), v27.clone(), v14.clone(), v34.clone());
                let mut v36: bool = v35.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v36 {
                    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy needs spiral's scripts/heavy.ps1: set EOIE_HEAVY_GATE or run from inside a tree that has a spiral checkout"); } LIT.with(|lit| lit.clone()) };
                    rejected_19(v37.clone())
                } else {
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
                    let mut v40: Rc<str> = job_file_9(v27.clone(), v39.clone());
                    let mut v41: Rc<str> = text_written_23(v40.clone(), v14.clone());
                    let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
                    let mut v43: Rc<str> = job_file_9(v27.clone(), v42.clone());
                    let mut v44: Rc<str> = decimal_text_15(v17);
                    let mut v45: Rc<str> = text_written_23(v43.clone(), v44.clone());
                    let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
                    let mut v47: Rc<str> = job_file_9(v27.clone(), v46.clone());
                    let mut v48: Rc<str> = std::rc::Rc::<str>::from(std::env::current_dir().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
                    let mut v49: Rc<str> = text_written_23(v47.clone(), v48.clone());
                    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
                    let mut v51: Rc<str> = job_file_9(v27.clone(), v50.clone());
                    let mut v52: Rc<str> = text_written_23(v51.clone(), v35.clone());
                    let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
                    let mut v54: Rc<str> = job_file_9(v27.clone(), v53.clone());
                    let mut v55: u64 = now_ms_13();
                    let mut v56: Rc<str> = decimal_text_15(v55);
                    let mut v57: Rc<str> = text_written_23(v54.clone(), v56.clone());
                    let mut v58: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v45, &*v49, &*v52, &*v57].concat());
                    let mut v59: bool = v58.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v60: bool = v59 != true;
                    if v60 {
                        rejected_19(v58.clone())
                    } else {
                        let mut v62: Rc<str> = jobs_root_3();
                        let mut v63: Rc<str> = supervisor_executable_24(v62.clone());
                        let mut v64: Rc<str> = { let launch = |flags: u32| { let program = if (&*v63).is_empty() { std::env::current_exe()? } else { std::path::PathBuf::from(&*v63) }; let mut command = std::process::Command::new(program); command.arg("job").arg("supervise").arg(&*v27).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()); #[cfg(windows)] { use std::os::windows::process::CommandExt; command.creation_flags(flags); } #[cfg(unix)] { use std::os::unix::process::CommandExt; command.process_group(0); let _ = flags; } command.spawn() }; match launch(0x0100_0208).or_else(|_| launch(0x0000_0208)) { Ok(child) => std::rc::Rc::<str>::from(child.id().to_string()), Err(error) => std::rc::Rc::<str>::from(format!("error: {error}")) } };
                        let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error:"); } LIT.with(|lit| lit.clone()) };
                        let mut v66: bool = v64.starts_with(&*v65);
                        if v66 {
                            rejected_19(v64.clone())
                        } else {
                            let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job start ok id="); } LIT.with(|lit| lit.clone()) };
                            let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" supervisor_pid="); } LIT.with(|lit| lit.clone()) };
                            let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" timeout_ms="); } LIT.with(|lit| lit.clone()) };
                            let mut v71: Rc<str> = decimal_text_15(v17);
                            let mut v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" gate="); } LIT.with(|lit| lit.clone()) };
                            let mut v76: Rc<str> = match &v5 {
                                US1::US1_HeavyStart => {
                                    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
                                    v74.clone()
                                }
                                US1::US1_PlainStart => {
                                    let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
                                    v73.clone()
                                }
                            };
                            let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
                            let mut v78: Rc<str> = std::rc::Rc::<str>::from([&*v77, &*v14, &*v26, &*v26, &*v26].concat());
                            let mut v79: Rc<str> = std::rc::Rc::<str>::from([&*v70, &*v71, &*v72, &*v76, &*v78].concat());
                            let mut v80: Rc<str> = std::rc::Rc::<str>::from([&*v68, &*v27, &*v69, &*v64, &*v79].concat());
                            stdout_line_17(v80.clone());
                            0i32
                        }
                    }
                }
            }
        }
    }
}
fn job_status_25() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_11(v1.clone());
        let mut v5: bool = std::path::Path::new(&*v4).is_dir();
        v5
    } else {
        false
    };
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = job_summary_5(v7.clone(), v1.clone());
        stdout_line_17(v8.clone());
        0i32
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v1, &*v10, &*v10, &*v10].concat());
        rejected_19(v11.clone())
    }
}
fn child_container_28() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn container() -> u64 { static CONTAINER: std::sync::OnceLock<u64> = std::sync::OnceLock::new(); *CONTAINER.get_or_init(|| { unsafe extern "system" { fn CreateJobObjectW(attributes: *mut std::ffi::c_void, name: *const u16) -> *mut std::ffi::c_void; fn SetInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32) -> i32; } unsafe { let job = CreateJobObjectW(std::ptr::null_mut(), std::ptr::null()); if !job.is_null() { let mut limits = [0u64; 18]; limits[2] = 0x800; SetInformationJobObject(job, 9, limits.as_mut_ptr().cast(), 144); } job as u64 } }) } #[cfg(not(windows))] fn container() -> u64 { 0 } container() };
    v0
}
fn supervised_27(mut v0: std::rc::Rc<std::cell::RefCell<std::process::Child>>, mut v1: Rc<str>, mut v2: u64, mut v3: u64) -> i32 {
    loop {
        let mut v4: i64 = match v0.clone().borrow_mut().try_wait() { Ok(Some(status)) => status.code().map(|code| code as i64).unwrap_or(-2), Ok(None) => -1, Err(_) => -2 };
        let mut v5: bool = v4 == -1i64;
        let mut v6: bool = v5 != true;
        if v6 {
            let mut v7: i32 = v4 as i32;
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = job_file_9(v1.clone(), v8.clone());
            let mut v10: u64 = now_ms_13();
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = std::rc::Rc::<str>::from((v7).to_string());
            let mut v13: Rc<str> = std::rc::Rc::<str>::from((v10).to_string());
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v14, &*v11, &*v16, &*v11].concat());
            let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v14, &*v13, &*v14, &*v17].concat());
            let mut v19: Rc<str> = text_written_23(v9.clone(), v18.clone());
            return v7;
        } else {
            let mut v20: u64 = now_ms_13();
            let mut v21: bool = v20 > v2;
            if v21 {
                let mut v22: u64 = child_container_28();
                let mut v23: u64 = { let mut child = v0.borrow_mut(); let root = child.id(); #[cfg(windows)] let survivors = { unsafe extern "system" { fn TerminateJobObject(job: *mut std::ffi::c_void, code: u32) -> i32; fn QueryInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32, returned: *mut u32) -> i32; } let _ = root; let job = v22 as *mut std::ffi::c_void; if !job.is_null() { unsafe { TerminateJobObject(job, 124); } } let _ = child.kill(); let _ = child.wait(); let active = || -> u64 { let mut accounting = [0u64; 6]; if job.is_null() || unsafe { QueryInformationJobObject(job, 1, accounting.as_mut_ptr().cast(), 48, std::ptr::null_mut()) } == 0 { 0 } else { accounting[5] & 0xffff_ffff } }; let mut waited = 0u64; while active() > 0 && waited < 3000 { std::thread::sleep(std::time::Duration::from_millis(100)); waited += 100; } active() }; #[cfg(unix)] let survivors = { let _ = v22; let _ = std::process::Command::new("kill").arg("-KILL").arg("--").arg(format!("-{root}")).status(); let _ = child.kill(); let _ = child.wait(); std::thread::sleep(std::time::Duration::from_millis(300)); std::fs::read_dir("/proc").map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| std::fs::read_to_string(entry.path().join("stat")).ok().and_then(|stat| stat.rsplit_once(char::from(41u8)).map(|(_, rest)| rest.split_whitespace().nth(2) == Some(root.to_string().as_str()))).unwrap_or(false)).count() as u64).unwrap_or(0) }; survivors };
                let mut v24: bool = v23 == 0u64;
                let mut v27: US2 = if v24 {
                    US2::US2_TreeStopped
                } else {
                    US2::US2_SurvivorsLeft(v23)
                };
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = job_file_9(v1.clone(), v28.clone());
                let mut v30: u64 = now_ms_13();
                let mut v39: Rc<str> = match &v27 {
                    US2::US2_SurvivorsLeft(v32) => {
                        let mut v32: u64 = *v32;
                        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wall-clock timeout; "); } LIT.with(|lit| lit.clone()) };
                        let mut v34: Rc<str> = std::rc::Rc::<str>::from((v32).to_string());
                        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes left alive"); } LIT.with(|lit| lit.clone()) };
                        let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let mut v37: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v36, &*v36].concat());
                        v37.clone()
                    }
                    US2::US2_TreeStopped => {
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wall-clock timeout; process tree stopped"); } LIT.with(|lit| lit.clone()) };
                        v31.clone()
                    }
                };
                let mut v40: Rc<str> = std::rc::Rc::<str>::from((124i32).to_string());
                let mut v41: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
                let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
                let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v42, &*v39, &*v44, &*v45].concat());
                let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v42, &*v41, &*v42, &*v46].concat());
                let mut v48: Rc<str> = text_written_23(v29.clone(), v47.clone());
                return 124i32;
            } else {
                let mut v49: u64 = now_ms_13();
                let mut v50: u64 = v3.wrapping_add(1000u64);
                let mut v51: bool = v49 > v50;
                let mut v56: u64 = if v51 {
                    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
                    let mut v53: Rc<str> = job_file_9(v1.clone(), v52.clone());
                    let mut v54: Rc<str> = decimal_text_15(v49);
                    let mut v55: Rc<str> = text_written_23(v53.clone(), v54.clone());
                    v49
                } else {
                    v3
                };
                let mut v57: u64 = 200u64;
                std::thread::sleep(std::time::Duration::from_millis(v57));
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2, v56);
                continue;
            }
        }
    }
}
fn job_supervised_26() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_9(v1.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_12(v3.clone());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = job_file_9(v1.clone(), v5.clone());
    let mut v7: Rc<str> = text_or_empty_12(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_9(v1.clone(), v8.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = job_file_9(v1.clone(), v10.clone());
    let mut v12: u64 = child_container_28();
    let mut v13: Result<std::rc::Rc<std::cell::RefCell<std::process::Child>>, std::rc::Rc<str>> = { let mut parts = v4.lines(); let program = parts.next().unwrap_or_default().to_owned(); let arguments = parts.map(str::to_owned).collect::<Vec<_>>(); let outcome = (|| -> std::io::Result<std::process::Child> { let stdout = std::fs::File::create(&*v9)?; let stderr = std::fs::File::create(&*v11)?; let mut command = std::process::Command::new(&program); command.args(&arguments).current_dir(&*v7).stdin(std::process::Stdio::null()).stdout(stdout).stderr(stderr); #[cfg(unix)] { use std::os::unix::process::CommandExt; command.process_group(0); } let child = command.spawn()?; #[cfg(windows)] { unsafe extern "system" { fn AssignProcessToJobObject(job: *mut std::ffi::c_void, process: *mut std::ffi::c_void) -> i32; } use std::os::windows::io::AsRawHandle; if v12 != 0 { unsafe { AssignProcessToJobObject(v12 as *mut std::ffi::c_void, child.as_raw_handle()); } } } #[cfg(not(windows))] { let _ = v12; } Ok(child) })(); outcome.map(|child| std::rc::Rc::new(std::cell::RefCell::new(child))).map_err(|error| std::rc::Rc::<str>::from(format!("spawn {program}: {error}"))) };
    let mut v14: bool = v13.is_ok();
    if v14 {
        let mut v15: std::rc::Rc<std::cell::RefCell<std::process::Child>> = v13.clone().unwrap();
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = job_file_9(v1.clone(), v16.clone());
        let mut v18: u64 = v15.borrow().id() as u64;
        let mut v19: Rc<str> = decimal_text_15(v18);
        let mut v20: Rc<str> = text_written_23(v17.clone(), v19.clone());
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = job_file_9(v1.clone(), v21.clone());
        let mut v23: u64 = now_ms_13();
        let mut v24: Rc<str> = decimal_text_15(v23);
        let mut v25: Rc<str> = text_written_23(v22.clone(), v24.clone());
        let mut v26: u64 = now_ms_13();
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = job_file_9(v1.clone(), v27.clone());
        let mut v29: Rc<str> = text_or_empty_12(v28.clone());
        let mut v30: u64 = decimal_u64_2(v29.clone());
        let mut v31: u64 = v26.wrapping_add(v30);
        let mut v32: u64 = now_ms_13();
        supervised_27(v15.clone(), v1.clone(), v31, v32)
    } else {
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let mut v35: Rc<str> = job_file_9(v1.clone(), v34.clone());
        let mut v36: u64 = now_ms_13();
        let mut v37: Rc<str> = v13.clone().err().unwrap_or_default();
        let mut v38: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
        let mut v39: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v44: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v40, &*v37, &*v42, &*v43].concat());
        let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v38, &*v40, &*v39, &*v40, &*v44].concat());
        let mut v46: Rc<str> = text_written_23(v35.clone(), v45.clone());
        125i32
    }
}
fn last_lines_30(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = { let text = std::fs::read_to_string(&*v0).unwrap_or_default(); let lines = text.lines().collect::<Vec<_>>(); let from = lines.len().saturating_sub(v1 as usize); std::rc::Rc::<str>::from(lines[from..].join("
")) };
    v2.clone()
}
fn job_tailed_29() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_11(v1.clone());
        let mut v5: bool = std::path::Path::new(&*v4).is_dir();
        v5
    } else {
        false
    };
    if v6 {
        let mut v7: i32 = std::env::args().count() as i32;
        let mut v8: bool = v7 > 4i32;
        let mut v12: u64 = if v8 {
            let mut v9: i32 = 4i32;
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v9 as usize).unwrap_or_default());
            decimal_u64_2(v10.clone())
        } else {
            20u64
        };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = job_summary_5(v13.clone(), v1.clone());
        stdout_line_17(v14.clone());
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--- stdout (last "); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = decimal_text_15(v12);
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines)\n"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = job_file_9(v1.clone(), v18.clone());
        let mut v20: Rc<str> = last_lines_30(v19.clone(), v12);
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v17, &*v20, &*v21].concat());
        stdout_line_17(v22.clone());
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--- stderr (last "); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = decimal_text_15(v12);
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = job_file_9(v1.clone(), v25.clone());
        let mut v27: Rc<str> = last_lines_30(v26.clone(), v12);
        let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v17, &*v27, &*v21].concat());
        stdout_line_17(v28.clone());
        0i32
    } else {
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v1, &*v30, &*v30, &*v30].concat());
        rejected_19(v31.clone())
    }
}
fn waited_32(mut v0: Rc<str>, mut v1: u64) -> US0 {
    loop {
        let mut v2: US0 = job_state_now_6(v0.clone());
        let mut v4: bool = match &v2 {
            US0::US0_JobPending => {
                false
            }
            US0::US0_JobRunning => {
                false
            }
            _ => {
                true
            }
        };
        let mut v7: bool = if v4 {
            true
        } else {
            let mut v5: u64 = now_ms_13();
            let mut v6: bool = v5 > v1;
            v6
        };
        if v7 {
            return v2.clone();
        } else {
            let mut v8: u64 = 250u64;
            std::thread::sleep(std::time::Duration::from_millis(v8));
            (v0, v1) = (v0.clone(), v1);
            continue;
        }
    }
}
fn job_waited_31() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = jobs_root_3();
    let mut v3: Rc<str> = supervisor_executable_24(v2.clone());
    let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v4 != true;
    let mut v8: bool = if v5 {
        let mut v6: bool = std::env::var_os("EOIE_JOB_RELAUNCHED").is_some();
        let mut v7: bool = v6 == false;
        v7
    } else {
        false
    };
    let mut v11: bool = if v8 {
        let mut v9: bool = std::env::current_exe().map(|path| path.to_string_lossy() == &*v3).unwrap_or(false);
        let mut v10: bool = v9 == false;
        v10
    } else {
        false
    };
    if v11 {
        let mut v12: i32 = match std::process::Command::new(&*v3).args(std::env::args().skip(1)).env("EOIE_JOB_RELAUNCHED", "1").status() { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("eoie error: job relaunch: {error}"); 2 } };
        v12
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
        let mut v14: bool = v1.starts_with(&*v13);
        let mut v17: bool = if v14 {
            let mut v15: Rc<str> = job_directory_11(v1.clone());
            let mut v16: bool = std::path::Path::new(&*v15).is_dir();
            v16
        } else {
            false
        };
        if v17 {
            let mut v18: u64 = now_ms_13();
            let mut v19: i32 = 4i32;
            let mut v20: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v19 as usize).unwrap_or_default());
            let mut v21: u64 = decimal_u64_2(v20.clone());
            let mut v22: u64 = v18.wrapping_add(v21);
            let mut v23: US0 = waited_32(v1.clone(), v22);
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = job_summary_5(v24.clone(), v1.clone());
            stdout_line_17(v25.clone());
            match &v23 {
                US0::US0_JobFinished(v26) => {
                    let mut v26: i32 = *v26;
                    v26
                }
                US0::US0_JobLost => {
                    125i32
                }
                US0::US0_JobPending => {
                    3i32
                }
                US0::US0_JobRunning => {
                    3i32
                }
                US0::US0_JobSpawnFailed(v27) => {
                    let mut v27: Rc<str> = v27.clone();
                    125i32
                }
                US0::US0_JobTimedOut => {
                    124i32
                }
            }
        } else {
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: unknown job id "); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v36: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v1, &*v35, &*v35, &*v35].concat());
            rejected_19(v36.clone())
        }
    }
}
fn job_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
    let mut v20: US3 = if v2 {
        US3::US3_JobStart
    } else {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervise"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            US3::US3_JobSupervise
        } else {
            let mut v6: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
            if v6 {
                US3::US3_JobStatus
            } else {
                let mut v8: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                if v8 {
                    US3::US3_JobWait
                } else {
                    let mut v10: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                    if v10 {
                        US3::US3_JobList
                    } else {
                        let mut v12: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
                        if v12 {
                            US3::US3_JobTail
                        } else {
                            US3::US3_JobUnknown
                        }
                    }
                }
            }
        }
    };
    match &v20 {
        US3::US3_JobList => {
            job_listed_1()
        }
        US3::US3_JobStart => {
            job_started_18()
        }
        US3::US3_JobStatus => {
            job_status_25()
        }
        US3::US3_JobSupervise => {
            job_supervised_26()
        }
        US3::US3_JobTail => {
            job_tailed_29()
        }
        US3::US3_JobUnknown => {
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start [--heavy] <label> <timeout-ms> <program> [args...] | job status <id> | job wait <id> <max-ms> | job list [count] | job tail <id> [lines]"); } LIT.with(|lit| lit.clone()) };
            rejected_19(v27.clone())
        }
        US3::US3_JobWait => {
            job_waited_31()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_run() -> i32 {
    closure0()()
}
