use fprof::utils::{create_abs_path, read_to_string};
use regex::Regex;

const RE_TIME: &str =
    r"^([[[:word:]]+\s*]+)\s*.{1}+\s*Time \(s\).*elapsed = ([[:digit:]]+):([[:digit:]]+):([[:digit:]]+).*$";

fn is_match(input: &str) -> bool {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(RE_TIME).unwrap();
    }
    RE.is_match(input)
}

fn test(input: &str) {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(RE_TIME).unwrap();
    }
    let caps = RE.captures(input).unwrap();
    let step = caps.get(1).map_or("", |m| m.as_str());
    let hour = caps.get(2).map_or(0, |m| m.as_str().parse::<u64>().unwrap());
    let min = caps.get(3).map_or(0, |m| m.as_str().parse::<u64>().unwrap());
    let sec = caps.get(4).map_or(0, |m| m.as_str().parse::<u64>().unwrap());
    let total = hour*3600 + min*60 + sec;
    println!("{},{}", total, step);
}

fn main() {
    let file = create_abs_path("examples/f1.log");
    let contents = read_to_string(file);
    for line in contents.lines() {
        if is_match(line) {
            // println!("{}", line);
            test(line);
        }
    }
}
