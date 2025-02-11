use clap::ValueEnum;
use crate::html_helper::HTML_MODEL;
use crate::parser::LyricLine;
use crate::parser::LyricWordType::{Hiragana, Kanji};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum HiraganaOptions {
    Tips,
    Pure,
    Hidden
}

pub struct Options {
    pub lyric_lines: Vec<LyricLine>,
    pub show_romaji: bool,
    pub show_translation: bool,
    pub show_hiragana: HiraganaOptions
}

pub fn to_html(options: Options) -> String {
    let mut result = String::new();

    for lyric_line in options.lyric_lines {
        let mut line = String::from("    <div>");
        let mut romaji_line = String::new();
        for word in lyric_line.lines {
            match word.word_type {
                Kanji => {
                    match options.show_hiragana {
                        HiraganaOptions::Tips => {
                            line.push_str(format!("<ruby>{}<rt>{}</rt></ruby>", word.word, word.hiragana.unwrap()).as_str())
                        },
                        HiraganaOptions::Pure => {
                            line.push_str(&*word.hiragana.unwrap())
                        },
                        HiraganaOptions::Hidden => {
                            line.push_str(&*word.word)
                        }
                    }
                },
                Hiragana => {
                    line.push_str(&*word.word)
                }
            }
            romaji_line.push_str(word.romaji.as_str());
            romaji_line.push(' ');
        }
        result.push_str(&*line);
        if options.show_romaji {
            result.push_str("<br/>");
            result.push_str(&*romaji_line.trim_end());
        }
        if options.show_translation {
            result.push_str("<br/>");
            result.push_str(&*lyric_line.translated);
        }
        result.push_str("</div>\n    <br/>\n");
    }

    HTML_MODEL.replace("{}", &*result)
}