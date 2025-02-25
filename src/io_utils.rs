use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use log::info;
use regex::Regex;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;

pub fn to_io_err_with_context(context: String) -> impl Fn(io::Error) -> io::Error {
    move |err: io::Error| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("{}: {}", context, err),
        )
    }
}


pub fn simple_patterns_to_regexps(patterns: &[String]) -> Result<Vec<Regex>, Box<dyn std::error::Error>> {
    let mut regexps = Vec::new();
    for pattern in patterns {
        regexps.push(simple_pattern_to_regex(pattern)?);
    }
    Ok(regexps)
}

fn simple_pattern_to_regex(pattern: &str) -> Result<Regex, Box<dyn std::error::Error>> {
    let regex = format!(".*{}", pattern.replace(".", "\\.").replace("*", ".*"));
    info!("simple_pattern_to_regex: '{}' -> '{}'", pattern, regex);
    Ok(Regex::new(&regex)?)
}

async fn is_text_file(file_path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    let mut file = TokioFile::open(file_path).await?;
    let mut buffer = [0; 1024];
    let size = file.read(&mut buffer).await?;
    Ok(is_text(&buffer[..size]))
}

fn is_text(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return true;
    }

    // If any byte is 0, it's likely binary.
    if bytes.iter().any(|&b| b == 0) {
        return false;
    }

    // If the bytes form valid UTF-8, assume it's text.
    if std::str::from_utf8(bytes).is_ok() {
        return true;
    }

    // Otherwise, use a heuristic: count control characters that are not typical text whitespace.
    let allowed_controls = [b'\n', b'\r', b'\t'];
    let total = bytes.len();
    let non_text_count = bytes.iter()
        .filter(|&&b| b < 0x20 && !allowed_controls.contains(&b))
        .count();

    // If more than 30% of the characters are unusual control characters, treat as binary.
    (non_text_count as f64 / total as f64) < 0.3
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_pattern_to_regex_test_simple_pattern() {
        let result = simple_pattern_to_regex("*.txt").unwrap();
        assert_eq!(".*.*\\.txt", result.as_str());
    }

    #[test]
    fn simple_pattern_to_regex_test_path_pattern() {
        let result = simple_pattern_to_regex("test/*/*.rs").unwrap();
        assert_eq!(".*test/.*/.*\\.rs", result.as_str());
    }

    #[test]
    fn simple_pattern_to_regex_test_empty_string() {
        let result = simple_pattern_to_regex("").unwrap();
        assert_eq!(".*", result.as_str());
    }

    #[test]
    fn is_text_test_empty() {
        assert!(is_text(&[]));
    }

    #[test]
    fn is_text_test_sample_string() {
        assert!(is_text("Hello, world!".as_bytes()));
    }

    #[test]
    fn is_text_test_binary_bytes() {
        assert!(!is_text(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    fn is_text_test(bytes: &[u8], expected: bool) {
        assert_eq!(is_text(bytes), expected);
    }
}