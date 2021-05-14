use crate::*;
use _extract::*;
use add::check_or_init_env;
use anyhow::Result as AnyResult;
use clap::ArgMatches;
use colorful::Colorful;
use std::env::var;
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

    // if !is_force {
    //     check_or_init_env(is_debug).await?;
    // }

    let mut elpf = _utils::get_pf_edgeless()?;
    if let Ok(p) = fs::read_link(&elpf).await {
        elpf = p;
    }

    let mut ready_copy = false;
    let mut cp_path = PathBuf::new();

    let szip = SevenZip::new(SevenZipOptions {
        path: _utils::get_exe_path()?
            .join("lib7z.dll")
            .to_str()
            .unwrap_or("")
            .to_string(),
        switches: vec![],
    });

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

    if ready_copy {
        if cp_path.join("..").join("Nes_Inport.7z").exists() {
            for i in _utils::get_dir_entry(&cp_path.join("..")).await? {
                if i.file_name() == "Nes_Inport.7z" {
                    _load::load_7z(i.path(), elpf.clone(), szip.clone(), true).await?;
                    break;
                }
            }
        }

        _load::load_from(&userpf.to_path_buf(), &cp_path).await?;
    }

    Ok(())
}
