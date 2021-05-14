#![allow(unused_mut, unused_variables, unused_must_use)]

use crate::{
    _extract::{SevenZip, SevenZipAddOptions, SevenZipOptions},
    _load, _resolve, _utils,
    lang_zhCN::formatter as cnText,
};
use anyhow::Result as AnyResult;
use clap::ArgMatches;
use cnText::start_dl_pkg;
use colorful::Colorful;
use fs::remove_dir_all;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use libept::base::v1;
use read_input::prelude::*;
use snailquote::escape;
use std::path::Path;
use std::{collections::HashMap, env::var};
use std::{collections::HashSet, path::PathBuf};
use std::{io::Write, time::Duration};
use subprocess::Exec;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task;
use tokio::{fs, time::interval};
use v1::Package;

use crate::types;

pub async fn command<'args>(args: &ArgMatches<'args>) -> AnyResult<()> {
    let mut is_force = false;
    let mut is_debug = false;

    let userpf = var("userprofile")?;
    let userpf = Path::new(&userpf);

    println!(
        /*"  {}\t Goto SubCommand `add`"*/ "  {}\t {}",
        "Bootstrap".green().bold(),
        cnText::gotosc("add")
    );

    if args.occurrences_of("force") >= 1 {
        is_force = true;
        println!("  {}\t Detected `--force` flag!", "Warning".yellow().bold());
    }
    if args.occurrences_of("debug") >= 1 {
        is_debug = true;
        println!(
            "  {}\t Detected `--debug` flag!",
            "Debugger".yellow().bold()
        );
    }

    if is_debug {
        println!(
            "  {}\t ArgsMatches = \n{:#?}",
            "Debugger".yellow().bold(),
            args
        );
    }
    if !is_force {
        check_or_init_env(is_debug).await?;
    }

    if !userpf
        .join(".bept")
        .join("indexes")
        .join("indexes.toml")
        .exists()
    {
        println!(
            /*"  {}\t Reading Indexes"*/ "  {}\t {}",
            "Package".green().bold(),
            cnText::reading_indexes()
        );
        crate::update::command(args).await?;
    }

    let indexes = fs::read(userpf.join(".bept").join("indexes").join("indexes.toml")).await?;
    let indexes = String::from_utf8(indexes)?;
    let indexes = toml::from_str::<types::IndexesOutput>(&indexes)?;

    let get_arg = {
        if let Some(v) = args.values_of("NAME") {
            v.collect::<HashSet<&str>>()
        } else {
            println!(
                /*"  {}\t Searching..."*/ "  {}\t {}",
                "Package".green().bold(),
                cnText::sea_searching()
            );
            return Ok(());
        }
    };

    if is_debug {
        println!("{:#?}", get_arg);
    }

    let get_resl = crate::search::search_packages(&get_arg, &indexes).await?;
    let mut will_install = vec![];
    for i in &get_resl {
        if i.0.len() == 1 {
            println!(
                /*"  {}\t Selected Package {:?}"*/ "  {}\t {}",
                "Indexes".green().bold(),
                cnText::selected_pkg(
                    &(i.0[0].1.to_underline_format().unwrap_or(
                        /* "UNKNOWN_UNKNOWN_UNKNOWN_UNKNOWN".to_string() */
                        cnText::sea_unknown_id()
                    ))
                )
            );
            will_install.push(i.0[0].1.to_owned());
        } else if i.0.len() > 1 {
            let (mut nums, mut items) = (vec![], vec![]);
            println!("");
            println!(
                /*"  {}\t Keyword {:?} has more than one package has been matched:\n"*/
                "  {}\t {}\n",
                "Indexes".green().bold(),
                cnText::sea_more_kw(&i.1)
            );
            for u in 0..i.0.len() {
                println!(
                    "    \t{}\t{:?}",
                    format!("[{}]", u).dark_gray().bold(),
                    i.0[u].1.to_underline_format().unwrap_or(
                        /* "UNKNOWN_UNKNOWN_UNKNOWN_UNKNOWN".to_string() */
                        cnText::sea_unknown_id()
                    )
                );
                nums.push(i.0[u].0);
                items.push(i.0[u].1.to_owned());
            }
            let input = input::<usize>()
                .msg(format!(
                    /*"\n  {}\t Please Choose One > ",*/
                    "\n  {}\t {}",
                    "Indexes".cyan().bold(),
                    cnText::choose_one()
                ))
                .inside_err(
                    0..i.0.len(),
                    format!(
                        "  {}\t That does not look like a number from 0 to {}. Please try again\n",
                        "Indexes".red().bold(),
                        i.0.len() - 1
                    ),
                )
                .err(format!(
                    "  {}\t That value does not pass. Please try again",
                    "Indexes".red().bold()
                ))
                .try_get()?;
            if is_debug {
                println!("input = {:#?}", items[input]);
            }
            println!(
                /* "  {}\t Selected Package: {:?}" */ "  {}\t {}",
                "Indexes".green().bold(),
                cnText::selected_pkg(
                    &(items[input].to_underline_format().unwrap_or(
                        /*"UNKNOWN_UNKNOWN_UNKNOWN_UNKNOWN".to_string()*/
                        cnText::sea_unknown_id()
                    ))
                )
            );
            will_install.push(items[input].clone());
        }
    }

    fs::remove_dir_all(userpf.join(".bept").join("temp")).await?;
    fs::create_dir_all(userpf.join(".bept").join("temp")).await?;

    println!(
        "  {}\t {}",
        "Installer".green().bold(),
        cnText::start_dl_pkg()
    );

    let mut m: MultiProgress = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template(
            format!(
                "  {}\t {}",
                "Downloading".cyan().bold(),
                r#"[{bar:55.}] ({bytes_per_sec:green}, {eta}) {msg}"#
            )
            .as_str(),
        )
        .progress_chars("=> ");

    // for i in &will_install {
    //     let temp_p = userpf
    //         .join(".bept")
    //         .join("temp")
    //         .join(format!("{}.7z", i.to_underline_format().unwrap()));
    //     let pbb = m.add(ProgressBar::new(0));

    //     pbb.set_style(sty.clone());

    //     dl_spawn(pbb.clone(), i.clone(), temp_p).await?;
    //     m.join()?;
    //     let mut ii = interval(Duration::from_millis(1000));
    //     ii.tick().await;
    //     ii.tick().await;
    //     ii.tick().await;
    //     ii.tick().await;
    // }

    let mut dl_threads: Vec<tokio::task::JoinHandle<AnyResult<()>>> = will_install
        .iter()
        .map(|i| {
            let temp_p = userpf
                .join(".bept")
                .join("temp")
                .join(format!("{}.7z", i.to_underline_format().unwrap()));
            let pbb = m.add(ProgressBar::new(0));

            pbb.set_style(sty.clone());

            tokio::task::spawn(dl_spawn(pbb.clone(), i.clone(), temp_p))
        })
        .collect();

    m.join()?;

    for t in &mut dl_threads {
        let _ = t.await??;
    }

    println!("  {}\t {}", "Installer".green().bold(), cnText::dl_pkg_ok());

    let save_to = {
        if let Some(p) = args.value_of("save-to") {
            Some(std::env::current_dir()?.join(p))
        } else {
            None
        }
    };

    let no_save = {
        if args.occurrences_of("no-save") > 0 {
            true
        } else {
            false
        }
    };

    let no_execute = {
        if args.occurrences_of("no-execute") > 0 {
            true
        } else {
            false
        }
    };

    if let Some(p) = &save_to {
        for i in &will_install {
            if let Err(_) = fs::copy(
                userpf
                    .join(".bept")
                    .join("temp")
                    .join(format!("{}.7z", i.to_underline_format()?)),
                p.join(format!("{}.7z", i.to_underline_format()?)),
            )
            .await
            {
                println!("  {}\t {}", "Installer".red().bold(), "自定义复制失败");
                break;
            }
        }
    }

    if no_execute {
        return Ok(());
    }

    let mut ready_copy = false;
    let mut cp_path = PathBuf::new();
    if let Some(p) = _resolve::resolve_fir_path().await.last() {
        cp_path = p.to_owned();
        if let Ok(_) = fs::write(cp_path.join(".bept.resource"), []).await {
            println!(
                "  {}\t 使用在 {:?} 的资源目录",
                "Installer".green().bold(),
                cp_path
            );
            ready_copy = !ready_copy;
        }
    }

    if !no_save && ready_copy {
        for i in &will_install {
            fs::copy(
                userpf
                    .join(".bept")
                    .join("temp")
                    .join(format!("{}.7z", i.to_underline_format()?)),
                cp_path.join(format!("{}.7z", i.to_underline_format()?)),
            )
            .await?;
            println!(
                "  {}\t 已保存 {:?} 到资源目录",
                "Installer".green().bold(),
                i.to_underline_format()?
            )
        }
    } else if cp_path == PathBuf::new() && !ready_copy {
        println!(
            "  {}\t {}",
            "Installer".red().bold(),
            "未找到 `Resource` 文件夹"
        );
    } else {
        println!(
            "  {}\t {}",
            "Installer".red().bold(),
            "`Resource` 文件夹不可写或已开启 `--no-save`"
        );
    }

    _load::load_from(&userpf.to_path_buf(), &userpf.join(".bept").join("temp")).await?;

    Ok(())
}

async fn dl_spawn<'s>(pb: ProgressBar, i: Package, tpath: PathBuf) -> AnyResult<()> {
    // interval(Duration::from_micros(8000)).tick().await;
    let mut resp = i
        .get_it("https://pineapple.edgeless.top/file/1/插件包/".to_string())
        .await
        .unwrap();
    let length = resp.content_length().unwrap_or(0);
    pb.set_length(length);
    let mut complete: u64 = 0;
    let mut file = File::create(tpath).await?;
    pb.set_message(&format!(" \t {}", i.to_underline_format()?.bold()));
    while let Some(chunk) = resp.chunk().await? {
        complete += chunk.len() as u64;
        pb.set_position(complete);
        file.write(&chunk).await?;
    }
    pb.finish_and_clear();
    Ok(())
}

pub async fn check_or_init_env(is_debug: bool) -> AnyResult<types::BaseConfig> {
    let userpf = var("userprofile")?;
    let userpf = Path::new(&userpf);
    let mut basecfg: types::BaseConfig;
    let default_cfg = format!(
        r#"[bept.version]
version = "1.0.0"
use_ept = false

[bept.files]
root = {0}
indexes = {1}
temp = {2}
"#,
        escape(&userpf.join(".bept").to_string_lossy()),
        escape(&userpf.join(".bept").join("indexes").to_string_lossy()),
        escape(&userpf.join(".bept").join("temp").to_string_lossy())
    );

    if is_debug {
        println!(
            "  {}\t UserProfile = {:?}",
            "Debugger".yellow().bold(),
            userpf
        );
        println!(
            "  {}\t BeptFolder = {:#?}",
            "Debugger".yellow().bold(),
            userpf.join(".bept"),
        );
        println!(
            "  {}\t Default Config = \n{}",
            "Debugger".yellow().bold(),
            default_cfg
        );
    }

    if !userpf.join(".bept").exists() {
        println!(
            /*"  {}\t Not found `.bept` folder, creating...",*/
            "  {}\t {}",
            "Checker".yellow().bold(),
            cnText::not_found_folder_and_creating(".bept")
        );
        fs::create_dir_all(userpf.join(".bept")).await?;
    } else {
        println!(
            /*"  {}\t Found `.bept` folder, Ok.",*/ "  {}\t {}",
            "Checker".green().bold(),
            cnText::found_folder(".bept")
        );
    }

    if !userpf.join(".bept").join("base.toml").exists() {
        println!(
            /*"  {}\t Not found `.bept/base.toml` file, creating...",*/
            "  {}\t {}",
            "Checker".yellow().bold(),
            cnText::not_found_file_and_creating(".bept/base.toml")
        );
        println!(
            /*"  {}\t Writing Default Config...",*/ "  {}\t {}",
            "Checker".green().bold(),
            cnText::write_default()
        );
        basecfg = types::BaseConfig::from_str(&default_cfg)?;
        fs::write(userpf.join(".bept").join("base.toml"), &default_cfg).await?;
    } else {
        println!(
            /* "  {}\t Found `.bept/base.toml` file, Ok.", */
            "  {}\t {}",
            "Checker".green().bold(),
            cnText::found_file(".bept/base.toml")
        );
        let cstr = String::from_utf8(fs::read(userpf.join(".bept").join("base.toml")).await?)?;
        basecfg = types::BaseConfig::from_str(&cstr)?;
    }

    if !userpf.join(".bept").join("temp").exists() {
        println!(
            /*"  {}\t Not found `.bept/temp` folder, creating...",*/
            "  {}\t {}",
            "Checker".yellow().bold(),
            cnText::not_found_folder_and_creating(".bept\temp")
        );
        fs::create_dir_all(userpf.join(".bept").join("temp")).await?;
    } else {
        println!(
            /* "  {}\t Found `.bept/temp` folder, Ok.", */
            "  {}\t {}",
            "Checker".green().bold(),
            cnText::found_folder(".bept/temp")
        );
    }

    if !userpf.join(".bept").join("indexes").exists() {
        println!(
            /* "  {}\t Not found `.bept/indexes` folder, creating...", */
            "  {}\t {}",
            "Checker".yellow().bold(),
            cnText::not_found_folder_and_creating(".bept/indexes")
        );
        fs::create_dir_all(userpf.join(".bept").join("indexes")).await?;
    } else {
        println!(
            /* "  {}\t Found `.bept/indexes` folder, Ok.", */
            "  {}\t {}",
            "Checker".green().bold(),
            cnText::found_folder(".bept/indexes")
        );
    }

    Ok(basecfg)
}
