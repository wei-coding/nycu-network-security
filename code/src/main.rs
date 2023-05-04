use regex::Regex;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let s = "(?:){4294967295}";
    let re = Regex::new(s).unwrap();
    println!("Compile Finished! Compile takes {:?} seconds", now.elapsed());
    println!("match result: {}", re.is_match(" "));
}
