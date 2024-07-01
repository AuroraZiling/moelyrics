use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use url::Url;

use moelyrics::generator::{Options, to_html};
use moelyrics::html_helper::extract_title;
use moelyrics::parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "URL")]
    url: String,

    #[arg(short, long, value_name = "FILE PATH", help = "Output .html file path")]
    output: PathBuf,

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

    let output = cli.output;
    let parsed_url = Url::parse(cli.url.as_str());
    if parsed_url.is_err() || !parsed_url.unwrap().host_str().unwrap().eq("mzh.moegirl.org.cn"){
        panic!("Invalid url")
    }

    let resp = reqwest::get(cli.url).await?.text().await?;
    let resp_title = extract_title(&resp).unwrap();
    println!("Title: {}", resp_title.split_whitespace().next().unwrap());

    let parsed = parser::to_lyric_lines(resp.as_str());
    if parsed.is_empty() {
        panic!("Empty lyrics")
    }

    let html = to_html(Options {
        lyric_lines: parsed,
        show_romaji: cli.romaji,
        show_translation: cli.translation,
        show_hiragana_tips: cli.hiragana,
    });

    let mut file = File::create(output)?;
    file.write(html.as_bytes())?;
    println!("Generated");
    Ok(())
}


