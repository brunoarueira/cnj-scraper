use std::env;
use std::fs;

mod cnj_extractor;

use cnj_extractor::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("CNJs found {:?}", extract(&contents));
}

#[cfg(test)]
mod tests;
