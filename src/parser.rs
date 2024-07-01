use std::collections::HashMap;
use clap::builder::Str;
use ego_tree::NodeRef;
use regex::Regex;
use scraper::{Html, Node, Selector};

#[derive(Debug)]
pub struct LyricLine {
    pub lines: Vec<LyricWord>,
    pub translated: String,
}

#[derive(Debug)]
pub struct LyricWord {
    pub word_type: LyricWordType,
    pub word: String,
    pub hiragana: Option<String>,
    pub romaji: String
}

#[derive(Debug)]
pub enum LyricWordType {
    Hiragana,
    Kanji,
}

fn remove_span_tags(html: &str) -> String {
    let re_open_span = Regex::new(r"<span[^>]*>").unwrap();
    let re_close_span = Regex::new(r"</span>").unwrap();
    let html_no_open_spans = re_open_span.replace_all(html, "");
    let html_no_spans = re_close_span.replace_all(&html_no_open_spans, "");
    html_no_spans.into_owned()
}



fn process_element(element: &NodeRef<Node>, word: &mut LyricWord) {
    for original_lyrics_line_words in element.children() {
        if let Some(word_type) = &original_lyrics_line_words.value().as_element() {
            let word_type = &word_type.name.local;
            let words: Vec<_> = original_lyrics_line_words.children().collect();
            if word_type.starts_with("rb") {
                word.word = words[0].value().as_text().unwrap().to_string();
            } else if word_type.starts_with("rt") {
                let hiragana: Vec<char> = words[0].value().as_text().unwrap().chars().collect();
                let hiragana = hiragana[1..hiragana.len()-1].iter().collect();
                word.romaji = hiragana_to_romaji(&hiragana);
                word.hiragana = Some(hiragana);
            }
        }
    }
}

fn hiragana_to_romaji(hiragana: &String) -> String{
    let hiragana_list = ["あ", "い", "う", "え", "お", "ア", "イ", "ウ", "エ", "オ", "ぁ", "ぃ", "ぅ", "ぇ", "ぉ", "ァ", "ィ", "ゥ", "ェ", "ォ", "ん", "ン", "ゃ", "ゅ", "ょ", "ゎ", "ャ", "ュ", "ョ", "ヮ", "ヵ", "ヶ", "か", "き", "く", "け", "こ", "カ", "キ", "ク", "ケ", "コ", "さ", "し", "す", "せ", "そ", "サ", "シ", "ス", "セ", "ソ", "た", "ち", "つ", "て", "と", "タ", "チ", "ツ", "テ", "ト", "な", "に", "ぬ", "ね", "の", "ナ", "ニ", "ヌ", "ネ", "ノ", "は", "ひ", "ふ", "へ", "ほ", "ハ", "ヒ", "フ", "ヘ", "ホ", "ま", "み", "む", "め", "も", "マ", "ミ", "ム", "メ", "モ", "や", "ゆ", "よ", "ヤ", "ユ", "ヨ", "ら", "り", "る", "れ", "ろ", "ラ", "リ", "ル", "レ", "ロ", "わ", "ゐ", "ゑ", "を", "ワ", "ヰ", "ヱ", "ヲ", "が", "ぎ", "ぐ", "げ", "ご", "ガ", "ギ", "グ", "ゲ", "ゴ", "ざ", "じ", "ず", "ぜ", "ぞ", "ザ", "ジ", "ズ", "ゼ", "ゾ", "だ", "ぢ", "づ", "で", "ど", "ダ", "ヂ", "ヅ", "デ", "ド", "ば", "び", "ぶ", "べ", "ぼ", "バ", "ビ", "ブ", "ベ", "ボ", "ぱ", "ぴ", "ぷ", "ぺ", "ぽ", "パ", "ピ", "プ", "ペ", "ポ", "いぇ", "イェ", "きゃ", "きゅ", "きょ", "しゃ", "しゅ", "しょ", "ちゃ", "ちゅ", "ちょ", "にゃ", "にゅ", "にょ", "ひゃ", "ひゅ", "ひょ", "みゃ", "みゅ", "みょ", "りゃ", "りゅ", "りょ", "キャ", "キュ", "キョ", "シャ", "シュ", "ショ", "チャ", "チュ", "チョ", "ニャ", "ニュ", "ニョ", "ヒャ", "ヒュ", "ヒョ", "ミャ", "ミュ", "ミョ", "リャ", "リュ", "リョ", "ぎゃ", "ぎゅ", "ぎょ", "じゃ", "じゅ", "じょ", "ぢゃ", "ぢゅ", "ぢょ", "びゃ", "びゅ", "びょ", "ぴゃ", "ぴゅ", "ぴょ", "くゎ", "ぐゎ", "ギャ", "ギュ", "ギョ", "ジャ", "ジュ", "ジョ", "ヂャ", "ヂュ", "ヂョ", "ビャ", "ビュ", "ビョ", "ピャ", "ピュ", "ピョ", "クヮ", "グヮ", "きぇ", "くぃ", "くぇ", "くぉ", "ぐぃ", "ぐぇ", "ぐぉ", "キェ", "クィ", "クェ", "クォ", "グィ", "グェ", "グォ", "しぇ", "じぇ", "すぃ", "ずぃ", "ちぇ", "つぁ", "つぃ", "つぇ", "つぉ", "にぇ", "ひぇ", "ふぁ", "ふぃ", "ふぇ", "ふぉ", "シェ", "ジェ", "スィ", "ズィ", "チェ", "ツァ", "ツィ", "ツェ", "ツォ", "ニェ", "ヒェ", "ファ", "フィ", "フェ", "フォ", "ふゅ", "ふょ", "フュ", "フョ"];
    let romaji_list = ["a ", "i ", "u ", "e ", "o ", "a ", "i ", "u ", "e ", "o ", "a ", "i ", "u ", "e ", "o ", "a ", "i ", "u ", "e ", "o ", "n ", "n ", "ya ", "yu ", "yo ", "wa ", "ya ", "yu ", "yo ", "wa ", "ka ", "ke ", "ka ", "ki ", "ku ", "ke ", "ko ", "ka ", "ki ", "ku ", "ke ", "ko ", "sa ", "si ", "su ", "se ", "so ", "sa ", "si ", "su ", "se ", "so ", "ta ", "ti ", "tu ", "te ", "to ", "ta ", "ti ", "tu ", "te ", "to ", "na ", "ni ", "nu ", "ne ", "no ", "na ", "ni ", "nu ", "ne ", "no ", "ha ", "hi ", "hu ", "he ", "ho ", "ha ", "hi ", "hu ", "he ", "ho ", "ma ", "mi ", "mu ", "me ", "mo ", "ma ", "mi ", "mu ", "me ", "mo ", "ya ", "yu ", "yo ", "ya ", "yu ", "yo ", "ra ", "ri ", "ru ", "re ", "ro ", "ra ", "ri ", "ru ", "re ", "ro ", "wa ", "wi ", "we ", "wo ", "wa ", "wi ", "we ", "wo ", "ga ", "gi ", "gu ", "ge ", "go ", "ga ", "gi ", "gu ", "ge ", "go ", "za ", "zi ", "zu ", "ze ", "zo ", "za ", "zi ", "zu ", "ze ", "zo ", "da ", "di ", "du ", "de ", "do ", "da ", "di ", "du ", "de ", "do ", "ba ", "bi ", "bu ", "be ", "bo ", "ba ", "bi ", "bu ", "be ", "bo ", "pa ", "pi ", "pu ", "pe ", "po ", "pa ", "pi ", "pu ", "pe ", "po ", "ye ", "ye ", "kya ", "kyu ", "kyo ", "sya ", "syu ", "syo ", "tya ", "tyu ", "tyo ", "nya ", "nyu ", "nyo ", "hya ", "hyu ", "hyo ", "mya ", "myu ", "myo ", "rya ", "ryu ", "ryo ", "kya ", "kyu ", "kyo ", "sya ", "syu ", "syo ", "tya ", "tyu ", "tyo ", "nya ", "nyu ", "nyo ", "hya ", "hyu ", "hyo ", "mya ", "myu ", "myo ", "rya ", "ryu ", "ryo ", "gya ", "gyu ", "gyo ", "zya ", "zyu ", "zyo ", "dya ", "dyu ", "dyo ", "bya ", "byu ", "byo ", "pya ", "pyu ", "pyo ", "kwa ", "gwa ", "gya ", "gyu ", "gyo ", "zya ", "zyu ", "zyo ", "dya ", "dyu ", "dyo ", "bya ", "byu ", "byo ", "pya ", "pyu ", "pyo ", "kwa ", "gwa ", "kye ", "kwi ", "kwe ", "kwo ", "gwi ", "gwe ", "gwo ", "kya ", "kwi ", "kwe ", "kwo ", "gwi ", "gwe ", "gwo ", "sye ", "zye ", "swi ", "zwi ", "tye ", "twa ", "twi ", "twe ", "two ", "nye ", "hye ", "hwa ", "hwi ", "hwe ", "hwo ", "sye ", "zye ", "swi ", "zwi ", "tye ", "twa ", "twi ", "twe ", "two ", "nye ", "hye ", "hwa ", "hwi ", "hwe ", "hwo ", "hwyu ", "hwyo ", "hwyu ", "hwyo "];
    let mapping: HashMap<_, _> = hiragana_list.iter().zip(romaji_list.iter()).collect();
    let mut result = hiragana.clone();
    for hiragana_word in hiragana_list.iter().rev() {
        if hiragana.contains(hiragana_word) {
            result = result.replace(hiragana_word, mapping.get(hiragana_word).unwrap());
        }
    }
    result.chars()
        .filter(|&c| c.is_ascii_alphabetic() || c == ' ')
        .collect::<String>()
        .trim_end()
        .to_string()
}


pub fn to_lyric_lines(html: &str) -> Vec<LyricLine> {
    let test_lyrics: String = remove_span_tags(html);

    let fragment = Html::parse_fragment(test_lyrics.as_str());
    let selector = Selector::parse(r#"div[class="Lyrics-line"]"#).unwrap();
    let original_selector = Selector::parse(r#"div[class="Lyrics-original"]"#).unwrap();
    let translated_selector = Selector::parse(r#"div[class="Lyrics-translated"]"#).unwrap();

    let lyrics_lines = fragment.select(&selector);
    let mut parsed_lyric_lines: Vec<LyricLine> = Vec::new();
    for lyrics_line in lyrics_lines {
        let mut parsed_lyric_line: LyricLine = LyricLine {
            lines: vec![],
            translated: String::new(),
        };

        let translated_word_node = lyrics_line.select(&translated_selector).next().unwrap();
        if translated_word_node.has_children() {
            let translated_word_node = translated_word_node.children().next().unwrap();
            if translated_word_node.value().is_text() {
                parsed_lyric_line.translated = translated_word_node.value().as_text().unwrap().to_string();
            }
        } else {
            continue;
        }

        let original_lyrics_line = lyrics_line.select(&original_selector).next().unwrap();
        for original_lyrics_line_nodes in original_lyrics_line.children() {
            if original_lyrics_line_nodes.value().is_text() {
                let hiragana_word = original_lyrics_line_nodes.value().as_text().unwrap();
                let word = LyricWord {
                    word_type: LyricWordType::Hiragana,
                    word: hiragana_word.to_string(),
                    hiragana: None,
                    romaji: hiragana_to_romaji(&hiragana_word.to_string())
                };
                parsed_lyric_line.lines.push(word);
            } else if original_lyrics_line_nodes.value().is_element() {
                let mut word = LyricWord {
                    word_type: LyricWordType::Kanji,
                    word: String::new(),
                    hiragana: Some(String::new()),
                    romaji: String::new()
                };
                process_element(&original_lyrics_line_nodes, &mut word);
                parsed_lyric_line.lines.push(word);
            }
        }

        parsed_lyric_lines.push(parsed_lyric_line);
    }

    parsed_lyric_lines
}