#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_NoMoreNotebooks,
    UH0_SpiralTestNotebook(Rc<str>, Rc<str>, Rc<UH0>),
}
fn run_all_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: Rc<UH0>) -> i32 {
    loop {
        match &*v6 {
            UH0::UH0_NoMoreNotebooks => {
                return 0i32;
            }
            UH0::UH0_SpiralTestNotebook(v7, v8, v9) => {
                let mut v7: Rc<str> = v7.clone();
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v11: bool = v2 == v10 ;
                let mut v15: bool = if v11 {
                    true
                } else {
                    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v2, &*v3, &*v10, &*v10].concat());
                    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v7, &*v3, &*v10, &*v10].concat());
                    let mut v14: bool = v12.contains(&*v13);
                    v14
                };
                let mut v17: bool = if v15 {
                    let mut v16: bool = std::path::Path::new(&*v0).join(&*v8).is_file();
                    v16
                } else {
                    false
                };
                let mut v21: i32 = if v17 {
                    if v4 {
                        let mut v18: i32 = { std::io::Write::write_all(&mut std::io::stdout(),format!("spiral test blocks {}
",&*v8).as_bytes()).unwrap(); 0i32 };
                        v18
                    } else {
                        let mut v19: i32 = { let root=std::path::Path::new(&*v0); let target=std::path::Path::new(&*v1); let kino=std::env::var_os("EOIE_SPIRAL_CELLS").map(std::path::PathBuf::from).or_else(|| std::env::var_os("EOIE_SPIRAL_BUNDLE").map(|bundle| std::path::PathBuf::from(bundle).join("..").join("..").join("cells"))).unwrap_or_else(|| root.join("..").join("spiral").join("apps").join("cells")); let source=root.join(&*v8); let output=target.join("spiral-tests").join(source.file_name().unwrap()).with_extension("spi.ipynb"); let _=std::fs::create_dir_all(target.join("spiral-tests")); let eoie=target.join("debug").join(format!("eoie{}",std::env::consts::EXE_SUFFIX)); let mut command=std::process::Command::new("pwsh"); command.current_dir(root).arg("-NoProfile").arg("-File").arg(kino.join("spi").join("run_notebook.ps1")).arg("--path").arg(&source).arg("--output-path").arg(&output).arg("--no-html").env("EOIE_BIN_UNDER_TEST",&eoie).env("SPIRAL_CELLS_WARM","0").env("SPIRAL_CELLS_MEMO","0").env("SPIRAL_CELLS_COMPILER","process"); match eoie_process::run_bounded(&mut command,v5) { Ok(status) => status.code().unwrap_or(1), Err(error) => { let _=std::io::Write::write_all(&mut std::io::stderr(),error.as_bytes()); 2 } } };
                        v19
                    }
                } else {
                    0i32
                };
                let mut v22: bool = v21 == 0i32;
                if v22 {
                    (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5, v9.clone());
                    continue;
                } else {
                    return v21;
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, u64) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, u64) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64| -> i32 {
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
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-cli"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_cli_model/argument_file.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-jobs"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_jobs/model.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-repo-state"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_repo_state/state.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-table-query"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_table_query/query.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-coverage-prune"); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_coverage_prune/lcov.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_bundle_retention/receipt_read.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie-read-query"); } LIT.with(|lit| lit.clone()) };
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src/eoie_read_query/model.spi"); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMoreNotebooks); } CASE.with(|case| case.clone()) };
        let mut v30: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v27.clone(), v28.clone(), v29.clone()));
        let mut v31: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v13.clone(), v26.clone(), v30.clone()));
        let mut v32: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v24.clone(), v25.clone(), v31.clone()));
        let mut v33: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v22.clone(), v23.clone(), v32.clone()));
        let mut v34: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v20.clone(), v21.clone(), v33.clone()));
        let mut v35: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v18.clone(), v19.clone(), v34.clone()));
        let mut v36: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v16.clone(), v17.clone(), v35.clone()));
        let mut v37: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v13.clone(), v15.clone(), v36.clone()));
        let mut v38: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v13.clone(), v14.clone(), v37.clone()));
        let mut v39: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v11.clone(), v12.clone(), v38.clone()));
        let mut v40: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v8.clone(), v10.clone(), v39.clone()));
        let mut v41: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v8.clone(), v9.clone(), v40.clone()));
        let mut v42: Rc<UH0> = Rc::new(UH0::UH0_SpiralTestNotebook(v6.clone(), v7.clone(), v41.clone()));
        run_all_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5, v42.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_dev_spiral_tests_run(v0: &str, v1: &str, v2: &str, v3: &str, v4: bool, v5: u64) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, v5)
}
