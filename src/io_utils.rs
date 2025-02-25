use std::error::Error;
use std::io;
use log::info;
use regex::Regex;


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
}