use crate::_extract::*;
use crate::_utils;
use anyhow::anyhow;
use anyhow::Result as AnyResult;
use colorful::Colorful;
use fs::{read_link, DirEntry};
use std::path::PathBuf;
use subprocess::Exec;
use tokio::fs;
pub async fn load_from(userpf: &PathBuf, eptp: &PathBuf) -> AnyResult<()> {
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

    let _exes = _utils::get_path_execs(&(elpf.clone())).await?;
    for i in &_exes {
        fs::remove_file(i).await?;
    }

    let szip = SevenZip::new(SevenZipOptions {
        path: _utils::get_exe_path()?
            .join("lib7z.dll")
            .to_str()
            .unwrap_or("")
            .to_string(),
        switches: vec![],
    });
    // let eptp = userpf.join(".bept").join("temp");
    for i in _utils::get_dir_entry(&eptp).await? {
        let p = i.path();
        load_7z(p, elpf.clone(), szip.clone(), false).await?;
    }

    println!(
        "  {}\t {}",
        "Installer".green().bold(),
        "安装完成, 正在清理..."
    );
    fs::remove_dir_all(userpf.join(".bept").join("temp")).await?;
    fs::create_dir_all(userpf.join(".bept").join("temp")).await?;
    Ok(())
}

pub async fn load_7z(i: PathBuf, elpf: PathBuf, szip: SevenZip, loadnes: bool) -> AnyResult<()> {
    if let Some(pe) = i.extension() {
        if pe == "7z" && i.is_file() {
            println!(
                "  {}\t {} {:?}\n",
                "Installer".green().bold(),
                "开始解压",
                i
            );
            let mut ii = szip
                .unzip(&i, &elpf, true, SevenZipAddOptions::default())
                .await?;
            let s = ii.wait()?;
            if s.success() {
                println!(
                    "\n  {}\t {} {:?}",
                    "Installer".green().bold(),
                    "解压成功, 退出状态:",
                    s
                );
            } else {
                println!(
                    "\n  {}\t {} {:?}",
                    "Installer".red().bold(),
                    "解压失败, 退出状态:",
                    s
                );
                panic!(s);
            }
            let execs = _utils::get_path_execs(&elpf).await?;
            println!(
                "  {}\t {} {} {}",
                "Installer".green().bold(),
                "开始执行脚本, 共",
                execs.len(),
                "个"
            );
            let mut iii: usize = 0;
            for i in &execs {
                iii += 1;
                println!(
                    "  {}\t {} {:?} ({}/{})",
                    "Installer".green().bold(),
                    "正在执行脚本: ",
                    i,
                    iii,
                    execs.len()
                );
                if i.extension().unwrap() == "cmd" {
                    let sysroot = std::env::var("systemroot")?;
                    let sysroot = PathBuf::from(sysroot);
                    let h = Exec::cmd(sysroot.join("System32").join("cmd.exe"))
                        .args(&["/c", &format!(r#"{}"#, i.to_str().unwrap())])
                        .cwd(&elpf)
                        .capture()?;
                    println!(
                        "  {}\t {} {:?}",
                        "Installer".green().bold(),
                        "脚本已执行, 退出状态:",
                        h.exit_status
                    );
                } else if i.extension().unwrap() == "wcs" {
                    let h = Exec::cmd(_utils::get_exe_path()?.join("libpecmd.dll"))
                        .args(&["load", &format!(r#"{}"#, i.to_str().unwrap())])
                        .cwd(&elpf)
                        .capture()?;
                    println!(
                        "  {}\t {} {:?}",
                        "Installer".green().bold(),
                        "脚本已执行, 退出状态:",
                        h.exit_status
                    );
                } else if i.extension().unwrap() == "ini" && loadnes {
                    let h = Exec::cmd(_utils::get_exe_path()?.join("libpecmd.dll"))
                        .args(&["load", &format!(r#"{}"#, i.to_str().unwrap())])
                        .cwd(&elpf)
                        .capture()?;
                    println!(
                        "  {}\t {} {:?}",
                        "Installer".green().bold(),
                        "脚本已执行, 退出状态:",
                        h.exit_status
                    );
                }

                // fs::rename(i, elpf.join("plugin_release").join(i.file_name().unwrap())).await?;
            }
        }
    }

    Ok(())
}
