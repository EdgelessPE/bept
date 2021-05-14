use anyhow::anyhow;
use anyhow::Result as AnyResult;
use fs::DirEntry;
use std::path::PathBuf;
use std::{env, path::Path};
use tokio::fs;

pub fn get_exe_path() -> AnyResult<PathBuf> {
    Ok(env::current_exe()?
        .parent()
        .ok_or(anyhow!("Not Found exe path"))?
        .to_path_buf())
}

pub async fn get_path_execs<P: AsRef<Path>>(p: P) -> AnyResult<Vec<PathBuf>> {
    let mut execs = vec![];
    let mut dir = fs::read_dir(p).await?;
    while let Some(entry) = dir.next_entry().await? {
        let p = entry.path();
        let pn = p.file_name();
        let pe = p.extension();
        let pt = entry.file_type().await?.is_file();
        if let Some(e) = pe {
            if e == "wcs" && pt {
                execs.push(p.to_path_buf());
            }
            if e == "cmd" && pt {
                execs.push(p.to_path_buf());
            }
            if e == "ini" && pt && pn.is_some() {
                if let Some(v) = pn {
                    if v == "Nes.ini" || v == "nes.ini" {
                        execs.push(p.to_path_buf());
                    }
                }
            }
        }
    }
    Ok(execs)
}

pub async fn get_dir_entry<P: AsRef<Path>>(p: P) -> AnyResult<Vec<DirEntry>> {
    let mut execs = vec![];
    let mut dir = fs::read_dir(p).await?;
    while let Some(entry) = dir.next_entry().await? {
        execs.push(entry);
    }
    Ok(execs)
}

pub fn get_pf_edgeless() -> AnyResult<PathBuf> {
    let pf = PathBuf::from(env::var("ProgramFiles")?);
    if !(pf.join("Edgeless").exists()) {
        return Err(anyhow!("Not Found Edgeless Folder"));
    }
    Ok(pf.join("Edgeless"))
}
