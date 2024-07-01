use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;

use moelyrics::generator::{Options, to_html};
use moelyrics::parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "URL")]
    url: Option<String>,

    #[arg(short, long, value_name = "FILE PATH", help = "Output .html file path")]
    output: Option<PathBuf>,

    #[arg(long, help = "Display Romaji below lyric lines")]
    romaji: bool,

    #[arg(long, help = "Display Chinese Translation below lyric lines")]
    translation: bool,

    #[arg(long, help = "Display Hiragana above lyric lines")]
    hiragana: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let output = cli.output.expect("Invalid output file path");
    let resp = reqwest::get(cli.url.expect("Invalid url")).await?.text().await?;
    let parsed = parser::to_lyric_lines(resp.as_str());
    let html = to_html(Options {
        lyric_lines: parsed,
        show_romaji: cli.romaji,
        show_translation: cli.translation,
        show_hiragana_tips: cli.hiragana,
    });
    let mut file = File::create(output).unwrap();
    file.write(html.as_bytes()).unwrap();
    Ok(())
}


