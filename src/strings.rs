use regex::Regex;
use unicode_normalization::UnicodeNormalization;

pub fn slugify(input: &str) -> String {
    let normalized = input.nfkd().collect::<String>();
    let ascii = normalized
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() || c.is_ascii_whitespace() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect::<String>();

    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(&ascii, "-").to_string()
}

pub fn sanitize_name(name: &str) -> String {
    slugify(name).replace("-", "_")
}
