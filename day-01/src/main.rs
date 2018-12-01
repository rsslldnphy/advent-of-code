use std::fs::File;
use std::io::Read;

static INPUT_FILE: &'static str = "input.txt";

fn parse_frequency(freq: &str) -> i32 {
    let mut chars = freq.chars();
    chars.next();
    let delta: i32 = chars.as_str().to_string().parse().unwrap();
    return if freq.to_string().starts_with("+") { delta } else { -1 * delta };
}

fn main() {
    let mut f = File::open(INPUT_FILE).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let total = contents.split('\n').map(&parse_frequency).fold(0, |acc, x| acc + x);
    
    println!("{:?}", total);
}
