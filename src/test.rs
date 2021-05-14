// #![feature(async_closure)]
// use crate::_resolve;
// use crate::_utils::get_exe_path;
// use crate::{
//     _extract::{SevenZip, SevenZipAddOptions, SevenZipOptions},
//     _utils,
// };
// use _resolve::resolve_el_path;
// use anyhow::Result as AnyResult;
// use base::v1::Package;
// use clap::ArgMatches;
// use colorful::Colorful;
// use fs::read_dir;
// use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
// use libept::base;
// use std::ffi::{OsStr, OsString};
// use std::io::prelude::*;
// use std::io::BufWriter;
// use std::io::Write;
// use std::os::windows::ffi::{OsStrExt, OsStringExt};
// use std::path::Path;
// use std::path::PathBuf;
// use std::sync::mpsc;
// use std::thread::spawn;
// use std::time::Duration;
// use std::{fs::File, future::Future};
// use tokio::fs;

// pub async fn command<'args>(_args: &ArgMatches<'args>) -> AnyResult<()> {
//     println!("{:?}", resolve_el_path().await);

//     println!("{:?}", get_exe_path()?);

//     // let szip = SevenZip::new(SevenZipOptions::default());
//     // let mut r = szip
//     //     .unzip("./out.7z", "./output2", true, SevenZipAddOptions::default())
//     //     .await?;
//     // let s = r.wait()?;

//     // println!("Ok!, {:?}", s);

//     println!("{:#?}", _utils::get_path_execs("./output2").await?);

//     Ok(())
// }
