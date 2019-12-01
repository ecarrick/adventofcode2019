use std::fs::File;
use std::io::{BufRead, BufReader};


// recursive
fn calculate_fuel(mass: i32) -> i32{
    if mass < 6 {
        return 0;
    }
    else {
        let fuel = (mass/3)-2;
        return fuel+calculate_fuel(fuel);
    }
}

fn main() {
    let f = BufReader::new(File::open("src/input.txt").unwrap());

    let masses: Vec<i32> = f.lines()
        .flat_map(|line| line.unwrap().trim().parse()).collect();

    let fuel: (i32) = masses.into_iter().map(|x| calculate_fuel(x)).sum();

    println!("{:?}", fuel);
}