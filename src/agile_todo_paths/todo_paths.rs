#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_CFamily,
    US1_HashFamily,
    US1_DashFamily,
    US1_HaskellFamily,
    US1_LeanFamily,
    US1_LispFamily,
    US1_ErlangFamily,
    US1_MarkupFamily,
    US1_MlFamily,
    US1_PascalFamily,
    US1_BatchFamily,
}
#[derive(Clone)]
enum US0 {
    US0_ScanPath(US1),
    US0_SkipNestedRepository,
    US0_SkipVendorDirectory,
    US0_SkipUnknownSyntax,
    US0_SkipGeneratedOutput,
    US0_SkipLinkedEntry,
}
#[derive(Clone)]
enum US2 {
    US2_KnownSyntax(US1),
    US2_UnknownSyntax,
}
fn vendor_segment_from_2(mut v0: Rc<str>, mut v1: i32) -> bool {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v3: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        if v4 {
            return false;
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules|vendor|third_party|target|dist|_build|.zig-cache|zig-out|.lake|__pycache__|.venv|venv|.gradle|.next|bin|obj|deps|.git|.cache|elm-stuff|bower_components|.dart_tool|.mypy_cache|.pytest_cache|.tox"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v3 as usize).unwrap_or(""));
            let mut v7: bool = v5.split("|").any(|item| item == &*v6);
            if v7 {
                return true;
            } else {
                let mut v8: i32 = v3.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v8);
                continue;
            }
        }
    }
}
fn last_offset_of_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = v0.len() as i32;
        let mut v5: bool = v4 <= v2;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v0.as_bytes().get(v2 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v7: bool = v6 == v1;
            if v7 {
                let mut v8: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1, v8, v2);
                continue;
            } else {
                let mut v10: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1, v10, v3);
                continue;
            }
        }
    }
}
fn syntax_of_path_3(mut v0: Rc<str>) -> US2 {
    let mut v1: i32 = 47i32;
    let mut v2: i32 = 0i32;
    let mut v3: i32 = -1i32;
    let mut v4: i32 = last_offset_of_4(v0.clone(), v1, v2, v3);
    let mut v5: i32 = v4.wrapping_add(1i32);
    let mut v6: i32 = v0.len() as i32;
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v6 as usize).unwrap_or(""));
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
    let mut v9: i32 = 46i32;
    let mut v10: i32 = 0i32;
    let mut v11: i32 = -1i32;
    let mut v12: i32 = last_offset_of_4(v8.clone(), v9, v10, v11);
    let mut v13: bool = v12 < 0i32;
    let mut v18: Rc<str> = if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v14.clone()
    } else {
        let mut v15: i32 = v12.wrapping_add(1i32);
        let mut v16: i32 = v8.len() as i32;
        let mut v17: Rc<str> = std::rc::Rc::<str>::from(v8.get(v15 as usize..v16 as usize).unwrap_or(""));
        v17.clone()
    };
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spi|spir|fs|fsx|fsi|rs|c|h|cc|cpp|cxx|hpp|hh|cs|js|mjs|cjs|ts|tsx|jsx|go|java|kt|kts|swift|zig|gleam|scala|dart|wgsl|glsl|hlsl|proto|jsonc|css|scss|less|php|sv|v"); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v21: bool = v18 == v20 ;
    let mut v22: bool = v21 == false;
    let mut v23: bool = v19.split("|").any(|item| item == &*v18);
    let mut v24: bool = v22 && v23;
    if v24 {
        let mut v25: US1 = US1::US1_CFamily;
        US2::US2_KnownSyntax(v25.clone())
    } else {
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps1|psm1|psd1|py|pyi|sh|bash|zsh|fish|rb|toml|yml|yaml|ex|exs|r|pl|pm|cmake|nix|conf|cfg|mk|tf|jl|nu|ini|gitignore|gitattributes|editorconfig|dockerignore"); } LIT.with(|lit| lit.clone()) };
        let mut v28: bool = v18 == v20 ;
        let mut v29: bool = v28 == false;
        let mut v30: bool = v27.split("|").any(|item| item == &*v18);
        let mut v31: bool = v29 && v30;
        if v31 {
            let mut v32: US1 = US1::US1_HashFamily;
            US2::US2_KnownSyntax(v32.clone())
        } else {
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("makefile|dockerfile|justfile|gemfile|rakefile|vagrantfile"); } LIT.with(|lit| lit.clone()) };
            let mut v35: bool = v34.split("|").any(|item| item == &*v8);
            if v35 {
                let mut v36: US1 = US1::US1_HashFamily;
                US2::US2_KnownSyntax(v36.clone())
            } else {
                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lua|sql|ada|adb|ads|vhd|vhdl"); } LIT.with(|lit| lit.clone()) };
                let mut v39: bool = v18 == v20 ;
                let mut v40: bool = v39 == false;
                let mut v41: bool = v38.split("|").any(|item| item == &*v18);
                let mut v42: bool = v40 && v41;
                if v42 {
                    let mut v43: US1 = US1::US1_DashFamily;
                    US2::US2_KnownSyntax(v43.clone())
                } else {
                    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hs|lhs|elm|purs|agda|idr"); } LIT.with(|lit| lit.clone()) };
                    let mut v46: bool = v18 == v20 ;
                    let mut v47: bool = v46 == false;
                    let mut v48: bool = v45.split("|").any(|item| item == &*v18);
                    let mut v49: bool = v47 && v48;
                    if v49 {
                        let mut v50: US1 = US1::US1_HaskellFamily;
                        US2::US2_KnownSyntax(v50.clone())
                    } else {
                        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lean"); } LIT.with(|lit| lit.clone()) };
                        let mut v53: bool = v18 == v52 ;
                        if v53 {
                            let mut v54: US1 = US1::US1_LeanFamily;
                            US2::US2_KnownSyntax(v54.clone())
                        } else {
                            let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("clj|cljs|cljc|edn|lisp|lsp|el|scm|ss|rkt|asm|nasm"); } LIT.with(|lit| lit.clone()) };
                            let mut v57: bool = v18 == v20 ;
                            let mut v58: bool = v57 == false;
                            let mut v59: bool = v56.split("|").any(|item| item == &*v18);
                            let mut v60: bool = v58 && v59;
                            if v60 {
                                let mut v61: US1 = US1::US1_LispFamily;
                                US2::US2_KnownSyntax(v61.clone())
                            } else {
                                let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("erl|hrl|tex|sty|bib"); } LIT.with(|lit| lit.clone()) };
                                let mut v64: bool = v18 == v20 ;
                                let mut v65: bool = v64 == false;
                                let mut v66: bool = v63.split("|").any(|item| item == &*v18);
                                let mut v67: bool = v65 && v66;
                                if v67 {
                                    let mut v68: US1 = US1::US1_ErlangFamily;
                                    US2::US2_KnownSyntax(v68.clone())
                                } else {
                                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("md|markdown|livemd|html|htm|xhtml|xml|svg|xaml|vue|svelte|csproj|fsproj|props|targets|plist"); } LIT.with(|lit| lit.clone()) };
                                    let mut v71: bool = v18 == v20 ;
                                    let mut v72: bool = v71 == false;
                                    let mut v73: bool = v70.split("|").any(|item| item == &*v18);
                                    let mut v74: bool = v72 && v73;
                                    if v74 {
                                        let mut v75: US1 = US1::US1_MarkupFamily;
                                        US2::US2_KnownSyntax(v75.clone())
                                    } else {
                                        let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ml|mli|sml"); } LIT.with(|lit| lit.clone()) };
                                        let mut v78: bool = v18 == v20 ;
                                        let mut v79: bool = v78 == false;
                                        let mut v80: bool = v77.split("|").any(|item| item == &*v18);
                                        let mut v81: bool = v79 && v80;
                                        if v81 {
                                            let mut v82: US1 = US1::US1_MlFamily;
                                            US2::US2_KnownSyntax(v82.clone())
                                        } else {
                                            let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pas|pp|dpr|dpk|lpr|inc"); } LIT.with(|lit| lit.clone()) };
                                            let mut v85: bool = v18 == v20 ;
                                            let mut v86: bool = v85 == false;
                                            let mut v87: bool = v84.split("|").any(|item| item == &*v18);
                                            let mut v88: bool = v86 && v87;
                                            if v88 {
                                                let mut v89: US1 = US1::US1_PascalFamily;
                                                US2::US2_KnownSyntax(v89.clone())
                                            } else {
                                                let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bat|cmd"); } LIT.with(|lit| lit.clone()) };
                                                let mut v92: bool = v18 == v20 ;
                                                let mut v93: bool = v92 == false;
                                                let mut v94: bool = v91.split("|").any(|item| item == &*v18);
                                                let mut v95: bool = v93 && v94;
                                                if v95 {
                                                    let mut v96: US1 = US1::US1_BatchFamily;
                                                    US2::US2_KnownSyntax(v96.clone())
                                                } else {
                                                    US2::US2_UnknownSyntax
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn path_verdict_1(mut v0: Rc<str>, mut v1: bool) -> US0 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v0.ends_with(&*v2);
    if v3 {
        US0::US0_SkipNestedRepository
    } else {
        let mut v5: i32 = 0i32;
        let mut v6: bool = vendor_segment_from_2(v0.clone(), v5);
        if v6 {
            US0::US0_SkipVendorDirectory
        } else {
            let mut v8: US2 = syntax_of_path_3(v0.clone());
            match &v8 {
                US2::US2_KnownSyntax(v10) => {
                    let mut v10: US1 = v10.clone();
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spi|spir"); } LIT.with(|lit| lit.clone()) };
                    let mut v12: i32 = 47i32;
                    let mut v13: i32 = 0i32;
                    let mut v14: i32 = -1i32;
                    let mut v15: i32 = last_offset_of_4(v0.clone(), v12, v13, v14);
                    let mut v16: i32 = v15.wrapping_add(1i32);
                    let mut v17: i32 = v0.len() as i32;
                    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v17 as usize).unwrap_or(""));
                    let mut v19: Rc<str> = std::rc::Rc::<str>::from(v18.to_ascii_lowercase());
                    let mut v20: i32 = 46i32;
                    let mut v21: i32 = 0i32;
                    let mut v22: i32 = -1i32;
                    let mut v23: i32 = last_offset_of_4(v19.clone(), v20, v21, v22);
                    let mut v24: bool = v23 < 0i32;
                    let mut v29: Rc<str> = if v24 {
                        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v25.clone()
                    } else {
                        let mut v26: i32 = v23.wrapping_add(1i32);
                        let mut v27: i32 = v19.len() as i32;
                        let mut v28: Rc<str> = std::rc::Rc::<str>::from(v19.get(v26 as usize..v27 as usize).unwrap_or(""));
                        v28.clone()
                    };
                    let mut v30: bool = v11.split("|").any(|item| item == &*v29);
                    let mut v31: bool = v30 == false;
                    let mut v32: bool = v1 && v31;
                    if v32 {
                        US0::US0_SkipGeneratedOutput
                    } else {
                        US0::US0_ScanPath(v10.clone())
                    }
                }
                US2::US2_UnknownSyntax => {
                    US0::US0_SkipUnknownSyntax
                }
            }
        }
    }
}
fn todo_path_verdict_0(mut v0: Rc<str>, mut v1: bool) -> i32 {
    let mut v2: US0 = path_verdict_1(v0.clone(), v1);
    match &v2 {
        US0::US0_ScanPath(v3) => {
            let mut v3: US1 = v3.clone();
            let mut v14: i32 = match &v3 {
                US1::US1_BatchFamily => {
                    11i32
                }
                US1::US1_CFamily => {
                    1i32
                }
                US1::US1_DashFamily => {
                    3i32
                }
                US1::US1_ErlangFamily => {
                    7i32
                }
                US1::US1_HashFamily => {
                    2i32
                }
                US1::US1_HaskellFamily => {
                    4i32
                }
                US1::US1_LeanFamily => {
                    5i32
                }
                US1::US1_LispFamily => {
                    6i32
                }
                US1::US1_MarkupFamily => {
                    8i32
                }
                US1::US1_MlFamily => {
                    9i32
                }
                US1::US1_PascalFamily => {
                    10i32
                }
            };
            let mut v15: i32 = 100i32 + v14;
            v15
        }
        US0::US0_SkipGeneratedOutput => {
            4i32
        }
        US0::US0_SkipLinkedEntry => {
            5i32
        }
        US0::US0_SkipNestedRepository => {
            1i32
        }
        US0::US0_SkipUnknownSyntax => {
            3i32
        }
        US0::US0_SkipVendorDirectory => {
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, bool) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, bool) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: bool| -> i32 {
        todo_path_verdict_0(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_todo_path_verdict(v0: &str, v1: bool) -> i32 {
    closure0()(Rc::<str>::from(v0), v1)
}
