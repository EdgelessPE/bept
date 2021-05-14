#![allow(
    unused_mut,
    unused_variables,
    unused_must_use,
    unused_imports,
    unused_assignments
)]
use crate::{add::check_or_init_env, types};
use anyhow::Result as AnyResult;
use clap::ArgMatches;
use colorful::Colorful;
use crypto::sha3::Sha3;
use crypto::util::rust_crypto_util_fixed_time_eq_asm;
use libept::base::v1;
use reqwest::Request;
use std::env::var;
use std::path::Path;
use tokio::fs;
static DEFAULT_CFG: &str = r#"[[list]]
url = "http://s.edgeless.top/?token=index"
edgeless_compat = true

[default]
index = 0
"#;

pub async fn command<'args>(args: &ArgMatches<'args>) -> AnyResult<()> {
    let mut is_force = false;
    let mut is_debug = false;

    println!(
        "  {}\t Goto SubCommand `update`",
        "Bootstrap".green().bold()
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

    let mut regcfg: types::RegistryConfig = toml::from_str(DEFAULT_CFG)?;
    let userpf = var("userprofile")?;
    let userpf = Path::new(&userpf);
    let regfile = userpf.join(".bept").join("registry.toml");

    if is_debug {
        println!(
            "  {}\t UserProfile = {:?}",
            "Debugger".yellow().bold(),
            userpf
        );
        println!(
            "  {}\t Registry config path = {}",
            "Debugger".yellow().bold(),
            regfile.to_string_lossy()
        );
    }

    let basecfg = check_or_init_env(is_debug).await?;

    if !regfile.exists() {
        println!(
            "  {}\t Not found `.bept/registry.toml` file, creating...",
            "Checker".yellow().bold()
        );
        println!("  {}\t Writing Default Config...", "Checker".green().bold());
        fs::write(regfile, DEFAULT_CFG).await?;
    } else {
        println!(
            "  {}\t Found `.bept/registry.toml` file, Reading.",
            "Checker".green().bold()
        );
        let rf = String::from_utf8(fs::read(regfile).await?)?;
        regcfg = toml::from_str(&rf)?;
    }

    if is_debug {
        println!(
            "  {}\t Current Registry config = \n{:#?}",
            "Debugger".yellow().bold(),
            regcfg
        );
    }

    let mut default_i;

    if let Some(v) = regcfg.list.get(regcfg.default.index) {
        default_i = v;
    } else {
        if let Some(v) = regcfg.list.get(0) {
            default_i = v;
        } else {
            return Err(types::BaseError::UnknownRegistry).map_err(anyhow::Error::new);
        }
    }

    let t = reqwest::get(&default_i.url)
        .await?
        .text_with_charset("gb18030")
        .await?;

    let mut list = regex::Regex::new(r#"\r\n|\n"#)?
        .split(t.trim())
        .collect::<Vec<&str>>();

    let mut ll = vec![];

    for i in &mut list {
        ll.push(v1::Package::from_underline_format(i.trim().to_string())?);
    }

    drop(list);

    if is_debug {
        println!(
            "  {}\t PackageList = \n{:#?}",
            "Debugger".yellow().bold(),
            &ll
        );
    }

    println!("  {}\t Writing Indexes", "Indexes".green().bold(),);

    fs::write(
        userpf.join(".bept").join("indexes").join("indexes.toml"),
        toml::to_string(&ll.into() as &types::IndexesOutput)?,
    )
    .await?;

    Ok(())
}
