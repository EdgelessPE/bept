#![allow(dead_code)]
use crate::lang_zhCN::formatter as cnText;
use anyhow::Result as AnyResult;
use colorful::Colorful;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use registry::{Data, Hive, Security};
use std::env;
use std::path::Path;
use tokio::fs;

const IS_CHECK: bool = false;

pub async fn resolve() -> AnyResult<(String, bool)> {
    let reg_trust = Hive::CurrentUser.open(
        r"Software\Microsoft\Windows\CurrentVersion\WinTrust",
        Security::AllAccess,
    )?;
    if let Ok(ss) = reg_trust.value("__PANIC_ERROR") {
        if let Data::U32(vv) = ss {
            if vv == 114514 {
                return Ok((String::new(), true));
            } else if vv == 1919810 {
                return Ok((String::new(), false));
            }
        }
    }
    if let Ok(ss) = reg_trust.value("AllowUnsafe") {
        if let Data::U32(vv) = ss {
            if vv != 255 {
                reg_trust.set_value("__PANIC_ERROR", &Data::U32(1919810))?;
                return Ok((String::new(), false));
            }
        } else {
            reg_trust.set_value("__PANIC_ERROR", &Data::U32(1919810))?;
            return Ok((String::new(), false));
        }
    } else {
        reg_trust.set_value("__PANIC_ERROR", &Data::U32(1919810))?;
        return Ok((String::new(), false));
    }

    let sys_root = env::var("SystemRoot")?;
    let drivers_path = Path::new(&sys_root)
        .join("System32")
        .join("drivers")
        .join("ucert.sys");
    let drivers_repo = Path::new(&sys_root)
        .join("System32")
        .join("DriverStore")
        .join("FileRepository")
        .join("ucert.inf_amd64_223D64613F2E8812")
        .join("ucert.sys");
    if !(drivers_path.exists() && drivers_repo.exists()) {
        reg_trust.set_value("__PANIC_ERROR", &Data::U32(1919810))?;
        return Ok((String::new(), false));
    }

    let drivers_one = fs::read(drivers_path).await?;
    let drivers_two = fs::read(drivers_repo).await?;

    let mut drivers_h = Sha3::sha3_512();
    drivers_h.input(&drivers_one);
    let drivers_one = drivers_h.result_str();
    drivers_h.reset();
    drivers_h.input(&drivers_two);
    let drivers_two = drivers_h.result_str();
    if drivers_one == drivers_two && drivers_one == "14f984ff514d0ad05959933e1c483c81b1ecdecb9faa4298c760d6203bc15739605a86c0dda298e3f9a9b5f0d9309dfa387b1abc64e8151d41a6291ab7515bac".to_string() {
      reg_trust.set_value("__PANIC_ERROR", &Data::U32(114514))?;
      return Ok((drivers_one,true));
    } else {
      reg_trust.set_value("__PANIC_ERROR", &Data::U32(1919810))?;
      return Ok((String::new(), false));
    }
}

pub async fn resolve_panic() -> AnyResult<()> {
    if IS_CHECK {
        let (_, b) = resolve().await?;
        if !b {
            panic!("Unknown!");
        } else {
            println!(
                /*"  {}\t System Check Success"*/ "  {}\t {}",
                "Checker".green().bold(),
                cnText::check_ok()
            );
        }
    } else {
        println!(
            /*"  {}\t Skipping System Check"*/ "  {}\t {}",
            "Checker".yellow().bold(),
            cnText::skip_check()
        );
    }

    Ok(())
}
