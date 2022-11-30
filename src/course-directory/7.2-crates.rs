use regex::Regex;

fn main() {
    let regx = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", regx.is_match("2014-01-01"));
}
