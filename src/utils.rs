use regex::Regex;

pub fn clean_str(input: &String) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    re.replace(input, "_").to_lowercase()
}