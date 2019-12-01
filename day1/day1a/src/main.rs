use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = BufReader::new(File::open("src/input.txt").unwrap());

    // map each line to a number
    let masses: Vec<i32> = f.lines()
        .flat_map(|line| line.unwrap().trim().parse()).collect();

    // map all masses from file into algorithm and sum all results
    let fuel: (i32) = masses.into_iter().map(|x| (x/3)-2).sum();


    println!("{:?}", fuel);
}