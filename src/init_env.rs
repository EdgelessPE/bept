#![allow(unused_variables, unused_assignments)]
use crate::lang_zhCN::formatter as cnText;
use anyhow::Result;
use clap::ArgMatches;
use colorful::Colorful;

pub async fn command<'args>(args: &ArgMatches<'args>) -> Result<()> {
    let mut is_force = false;
    let mut is_debug = false;

    println!(
        /*"  {}\t Goto SubCommand `init-env`"*/
        "  {}\t {}",
        "Bootstrap".green().bold(),
        cnText::gotosc("init-env")
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

    crate::add::check_or_init_env(is_debug).await?;
    crate::update::command(args).await?;

    Ok(())
}
