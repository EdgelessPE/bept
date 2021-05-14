mod _extract;
mod _load;
mod _resolve;
mod _utils;
mod _verify_firpe;
mod add;
mod init;
mod init_env;
mod lang_zhCN;
mod load;
mod search;
mod test;
mod types;
mod update;
use anyhow::Result as AnyResult;
use clap::{App, Arg, SubCommand};
use colorful::Colorful;
use lang_zhCN::formatter as cnText;
use std::process::exit;

#[tokio::main]
async fn main() -> AnyResult<()> {
    println!("");
    println!(
        /* "  {}\t Better Edgeless Package Tool, version = {}, FirPE" */
        "  {}\t {}",
        "Bootstrap".green().bold(),
        cnText::startup("0.23.1-FirPE")
    );
    crate::_verify_firpe::resolve_panic().await?;
    let matches = App::new("Better-Ept")
        .version("0.21.0-FirPE")
        .author("Oxygen")
        .about("Better Edgeless Package Tool")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add Packages")
                .version("1.0.0")
                .arg(
                    Arg::with_name("no-execute")
                        .long("--no-execute")
                        .short("N")
                        .help("Only Save, No Execute"),
                )
                .arg(
                    Arg::with_name("save-to")
                        .long("--save-to")
                        .short("O")
                        .value_name("path")
                        .help("Save To..."),
                )
                .arg(
                    Arg::with_name("no-save")
                        .long("--no-save")
                        .short("S")
                        .help("Only Load Execute, No Save"),
                )
                .arg(Arg::with_name("force").long("--force").help("Force Mode"))
                .arg(
                    Arg::with_name("debug")
                        .long("--debug")
                        .help("Show Debug Output"),
                )
                .arg(
                    Arg::with_name("NAME")
                        .multiple(true)
                        .takes_value(true)
                        .help("Package Name"),
                ),
        )
        // .subcommand(
        //     SubCommand::with_name("init-env")
        //         .arg(
        //             Arg::with_name("force")
        //                 .long("--force")
        //                 .help("Force Install Package"),
        //         )
        //         .arg(
        //             Arg::with_name("debug")
        //                 .long("--debug")
        //                 .help("Show Debug Output"),
        //         ),
        // )
        .subcommand(
            SubCommand::with_name("update")
                .arg(Arg::with_name("force").long("--force").help("Force"))
                .arg(
                    Arg::with_name("debug")
                        .long("--debug")
                        .help("Show Debug Output"),
                ),
        )
        // .subcommand(
        //     SubCommand::with_name("test")
        //         .arg(Arg::with_name("force").long("--force").help("Force"))
        //         .arg(
        //             Arg::with_name("debug")
        //                 .long("--debug")
        //                 .help("Show Debug Output"),
        //         ),
        // )
        .subcommand(
            SubCommand::with_name("search")
                .arg(
                    Arg::with_name("NAME")
                        .multiple(true)
                        .takes_value(true)
                        .help("Package Name"),
                )
                .arg(Arg::with_name("force").long("--force").help("Force"))
                .arg(
                    Arg::with_name("debug")
                        .long("--debug")
                        .help("Show Debug Output"),
                ),
        )
        .subcommand(
            SubCommand::with_name("load")
                .arg(Arg::with_name("force").long("--force").help("Force"))
                .arg(
                    Arg::with_name("debug")
                        .long("--debug")
                        .help("Show Debug Output"),
                )
                .arg(
                    Arg::with_name("DIRS")
                        .multiple(true)
                        .takes_value(true)
                        .help("Package Dir"),
                ),
        ).subcommand(
            SubCommand::with_name("init")
                .arg(Arg::with_name("force").long("--force").help("Force"))
                .arg(
                    Arg::with_name("debug")
                        .long("--debug")
                        .help("Show Debug Output"),
                )
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(args)) => {
            add::command(args).await?;
        }
        // ("init-env", Some(args)) => {
        //     init_env::command(args).await?;
        // }
        ("update", Some(args)) => {
            update::command(args).await?;
        }
        ("search", Some(args)) => {
            search::command(args).await?;
        }
        ("init", Some(args)) => {
            init::command(args).await?;
        }
        ("load", Some(args)) => {
            load::command(args).await?;
        }
        // ("test", Some(args)) => {
        //     test::command(args).await?;
        // }
        _ => {
            println!("  {}\t No SubCommand", "Bootstrap".green().bold());
            println!("\n{}\n", matches.usage());
            exit(9009);
        }
    }

    println!(
        "  {}\t\t {}\n",
        "Exit".green().bold(),
        cnText::exit_with_code(0)
    );
    Ok(())
}
