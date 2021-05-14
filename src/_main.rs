 
use colorful::Colorful;
use indicatif::{ProgressBar, ProgressStyle};
use libept::base;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("");
    println!(
        "  {}\t Better-Ept, version = {}",
        "Bootstrap".green().bold(),
        "0.1.2-beta"
    );
    println!("  {}\t Got Packages Url", "Installer".green().bold());
    let package =
        base::v1::Package::from_underline_format("Atom_1.48.0.0_Oxygen_办公编辑".to_string())?;
    let url =
        package.get_download_url("https://pineapple.edgeless.top/file/1/插件包/".to_string())?;
    // println!("{}", url.to_string());
    println!("  {}\t Request Server", "Installer".green().bold());
    let mut dl = package
        .get_it("https://pineapple.edgeless.top/file/1/插件包/".to_string())
        .await?;
    let length = dl.content_length().unwrap_or(0);
    let mut complete: u64 = 0;

    let mut file = File::create("./out.7z")?;

    let pb_text = format!(
        "  {}\t{}",
        "Downloading".cyan().bold(),
        r#" [ {bar:40.} ] {bytes}/{total_bytes}  ({bytes_per_sec:green}, {eta}) {msg}"#
    );
    let pb = ProgressBar::new(length);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&pb_text)
            .progress_chars("=> "),
    );

    pb.set_message(format!("{}: {}", "Current", package.name).as_str());

    pb.println(format!(
        "  {}\t Download Packages",
        "Installer".green().bold()
    ));

    while let Some(chunk) = dl.chunk().await? {
        complete += chunk.len() as u64;
        /*println!(
            "{} {} {:.2}/{:.2} MiB",
            "Downloading".cyan().bold(),
            (complete as f64) / 1024.00 / 1024.00,
            (length as f64) / 1024.00 / 1024.00
        );*/
        pb.set_position(complete);
        file.write(chunk.as_ref())?;
    }
    file.flush()?;
    pb.finish_and_clear();
    println!("  {}\t Ok", "Installer".green().bold());
    println!("  {}\t Executing Scripts...", "Installer".green().bold());
    println!("  {}\t\t Ok, exit code = 0", "Exit".green().bold());

    // https://pineapple.edgeless.top/file/1/%E6%8F%92%E4%BB%B6%E5%8C%85/%E5%8A%9E%E5%85%AC%E7%BC%96%E8%BE%91/Atom_1.48.0.0_Oxygen.7z
    Ok(())
}
