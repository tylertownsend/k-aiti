use regex::Regex;

pub fn parse_error_message(output: &str) -> Option<String> {
    let error_regex = Regex::new(r"Error:.*").unwrap();
    let mut last_error: Option<String> = None;

    for line in output.lines() {
        if error_regex.is_match(line) {
            last_error = Some(line.to_string());
        }
    }

    last_error
}