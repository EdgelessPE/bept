#![allow(unused_mut, unused_variables, unused_must_use)]
use crate::lang_zhCN::formatter as cnText;
use anyhow::Result as AnyResult;
use clap::ArgMatches;
use colorful::Colorful;
use libept::base::v1;
use std::collections::HashSet;
use std::env::var;
use std::path::Path;
use tokio::fs;
use v1::Package;

use crate::types;

pub async fn command<'args>(args: &ArgMatches<'args>) -> AnyResult<()> {
    let mut is_force = false;
    let mut is_debug = false;

    let userpf = var("userprofile")?;
    let userpf = Path::new(&userpf);

    println!(
        /* "  {}\t Goto SubCommand `search`"*/ "  {}\t {}",
        "Bootstrap".green().bold(),
        cnText::gotosc("search")
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
    if !is_force {
        crate::add::check_or_init_env(is_debug).await?;
    }

    println!(
        /*"  {}\t Reading Indexes"*/ "  {}\t {}",
        "Package".green().bold(),
        cnText::reading_indexes()
    );
    if !userpf
        .join(".bept")
        .join("indexes")
        .join("indexes.toml")
        .exists()
    {
        println!(
            /*"  {}\t Updating Indexes"*/ "  {}\t {}",
            "Package".green().bold(),
            cnText::updating_indexes()
        );
        crate::update::command(args).await?;
    }

    let indexes = fs::read(userpf.join(".bept").join("indexes").join("indexes.toml")).await?;
    let indexes = String::from_utf8(indexes)?;
    let indexes = toml::from_str::<types::IndexesOutput>(&indexes)?;

    let exps = {
        if let Some(v) = args.values_of("NAME") {
            v.collect::<HashSet<&str>>()
        } else {
            println!(
                /*"  {}\t Searching..."*/ "  {}\t {}",
                "Package".green().bold(),
                cnText::sea_searching()
            );
            return Ok(());
        }
    };

    if is_debug {
        println!(
            "  {}\t Search Args: {:#?}",
            "Debugger".yellow().bold(),
            &exps
        );
        println!(
            "  {}\t Indexes: {:#?}",
            "Debugger".yellow().bold(),
            &indexes
        );
    }

    println!(
        /*"  {}\t Searching..."*/ "  {}\t {}",
        "Package".green().bold(),
        cnText::sea_searching()
    );
    let resl = search_packages(&exps, &*indexes).await?;

    println!(
        /* "  {}\t Search Result: "*/ "  {}\t {}",
        "Package".green().bold(),
        cnText::sea_result()
    );
    println!("");
    for i in resl {
        println!("{}", i);
    }
    Ok(())
}

#[derive(Debug)]
pub enum SearchKeywords {
    Number(usize),
    Regex(String),
    Name(String),
}

#[derive(Debug)]
pub struct SearchResult(pub Vec<(usize, Package)>, pub SearchKeywords);

impl std::fmt::Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        /*"    {} - {}:",
        "Keyword".cyan().bold(),
        format!("{:?}", &self.1).dark_gray().bold() */
        writeln!(f, "    {}", cnText::sea_found_kw(&self.1))?;
        for u in &self.0 {
            writeln!(
                f,
                "    \t{}\t{:?}",
                format!("[{}]", u.0).dark_gray().bold(),
                u.1.to_underline_format().unwrap_or(
                    /* "UNKNOWN_UNKNOWN_UNKNOWN_UNKNOWN".to_string() */
                    cnText::sea_unknown_id()
                )
            )?;
        }
        writeln!(f, "")
    }
}

pub async fn search_packages(
    exps: &HashSet<&str>,
    indexes: &Vec<Package>,
) -> AnyResult<Vec<SearchResult>> {
    let mut get_resl: Vec<SearchResult> = vec![];
    for i in exps {
        if !i.is_empty() {
            if let Ok(n) = i.parse::<usize>() {
                if n > 0 {
                    if let Some(v) = indexes.get(n - 1) {
                        get_resl.push(SearchResult(
                            vec![(n, v.clone())],
                            SearchKeywords::Number(n),
                        ));
                    } else {
                        println!(
                            /*"  {}\t Invalid Index ID, {:?}"*/ "  {}\t {}",
                            "Indexes".yellow().bold(),
                            cnText::sea_invalid_id(i)
                        );
                    }
                } else {
                    println!(
                        /*"  {}\t Invalid Index ID, {:?}"*/ "  {}\t {}",
                        "Indexes".yellow().bold(),
                        cnText::sea_invalid_id(i)
                    );
                }
                continue;
            } else if i.len() > 2 && &i[..2] == "$!" {
                if let Ok(_) = Package::from_underline_format(i.to_string()) {
                    let mut cur_resl = vec![];
                    let mut nn: usize = 0;
                    for cc in &*indexes {
                        nn += 1;
                        if cc.to_underline_format()? == i[2..].to_string() {
                            cur_resl.push((nn, cc.clone()));
                        }
                    }
                    get_resl.push(SearchResult(
                        cur_resl,
                        SearchKeywords::Name(i[2..].to_string()),
                    ));
                    continue;
                } else {
                    println!(
                        /*"  {}\t Invalid Index ID, {:?}"*/ "  {}\t {}",
                        "Indexes".yellow().bold(),
                        cnText::sea_invalid_name(i)
                    );
                }
            } else if let Ok(exp) = regex::Regex::new(&i) {
                let mut cur_resl = vec![];
                let mut nn: usize = 0;
                for i in &*indexes {
                    nn += 1;
                    if exp.is_match(&(i.to_underline_format()?)) {
                        cur_resl.push((nn, i.clone()))
                    }
                }
                get_resl.push(SearchResult(cur_resl, SearchKeywords::Regex(i.to_string())));

                continue;
            } else {
                println!(
                    /*"  {}\t Invalid Index ID, {:?}"*/ "  {}\t {}",
                    "Indexes".yellow().bold(),
                    cnText::sea_invalid_arg(i)
                );
            }
        }
    }
    Ok(get_resl)
}
