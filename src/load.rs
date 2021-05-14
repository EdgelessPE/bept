use crate::*;
use _extract::*;
use add::check_or_init_env;
use anyhow::Result as AnyResult;
use clap::ArgMatches;
use colorful::Colorful;
use std::collections::HashSet;
use std::env::var;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;

pub async fn command<'s>(args: &ArgMatches<'s>) -> AnyResult<()> {
    let mut is_force = false;
    let mut is_debug = false;

    let userpf = var("userprofile")?;
    let userpf = Path::new(&userpf);

    println!(
        /*"  {}\t Goto SubCommand `add`"*/ "  {}\t {}",
        "Bootstrap".green().bold(),
        cnText::gotosc("load")
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

    let mut elpf = _utils::get_pf_edgeless()?;
    if let Ok(p) = fs::read_link(&elpf).await {
        println!(
            "  {}\t {} {:?}",
            "Installer".green().bold(),
            "检测到目录链接, Edgeless 目录已被重定向为",
            elpf
        );
        elpf = p;
    }
    if elpf.join("plugin_release").exists() {
        fs::create_dir_all(elpf.join("plugin_release")).await?;
    }
    println!(
        "  {}\t 使用 `{:?}` 作为 Edgeless 目录",
        "Installer".green().bold(),
        elpf
    );

    let get_arg = {
        if let Some(v) = args.values_of("DIRS") {
            v.collect::<HashSet<&str>>()
        } else {
            println!(
                /*"  {}\t Searching..."*/ "  {}\t {}",
                "Package".green().bold(),
                "什么都不加载?"
            );
            return Ok(());
        }
    };

    let szip = SevenZip::new(SevenZipOptions {
        path: _utils::get_exe_path()?
            .join("lib7z.dll")
            .to_str()
            .unwrap_or("")
            .to_string(),
        switches: vec![],
    });

    for i in get_arg {
        let pp = PathBuf::from(i);
        let pe = pp.extension().unwrap_or(OsStr::new(""));
        if pp.exists() && (pe == "7z" || pe == "7zf" || pe == "7zn") {
            println!(
                "  {}\t {} {:?}",
                "Installer".green().bold(),
                "正在加载包",
                pp
            );
            _load::load_7z(pp, elpf.clone(), szip.clone(), false).await?;
        } else {
            println!("  {}\t {} {:?}", "Installer".red().bold(), "无效的路径", pp);
        }
    }

    Ok(())
}
