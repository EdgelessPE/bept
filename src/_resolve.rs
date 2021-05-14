#![allow(unused_mut, unused_variables, unused_must_use, unused_imports)]
use anyhow::Result as AnyResult;
use std::path::{Path, PathBuf};
use sysinfo::{Disk, DiskExt, RefreshKind, System, SystemExt};

pub async fn resolve_el_path() -> Vec<PathBuf> {
    System::new_all()
        .get_disks()
        .iter()
        .filter_map(|x| {
            if x.get_mount_point()
                .join("Edgeless")
                .join("Resource")
                .exists()
            {
                Some(
                    x.get_mount_point()
                        .join("Edgeless")
                        .join("Resource")
                        .to_path_buf(),
                )
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>()
}

pub async fn resolve_fir_path() -> Vec<PathBuf> {
    System::new_all()
        .get_disks()
        .iter()
        .filter_map(|x| {
            if x.get_mount_point().join("FirPE").join("Resource").exists() {
                Some(
                    x.get_mount_point()
                        .join("FirPE")
                        .join("Resource")
                        .to_path_buf(),
                )
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>()
}

pub async fn resolve_custmon_path(pf: &PathBuf) -> Vec<PathBuf> {
    System::new_all()
        .get_disks()
        .iter()
        .filter_map(|x| {
            if x.get_mount_point().join(pf).exists() {
                Some(x.get_mount_point().join(pf).to_path_buf())
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>()
}
