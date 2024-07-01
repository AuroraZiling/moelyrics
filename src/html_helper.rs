use regex::Regex;

pub fn extract_title(html: &str) -> Option<String> {
    let re = Regex::new(r"(?i)<title>(.*?)</title>").unwrap();
    if let Some(caps) = re.captures(html) {
        return Some(caps[1].to_string());
    }
    None
}

pub const HTML_MODEL: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Moelyrics</title>
</head>
<body>
{}
</body>
</html>"#;