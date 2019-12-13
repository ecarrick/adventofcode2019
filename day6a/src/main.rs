use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Planet {id:String}

#[derive(Debug)]
struct Orbit {
    parent: Planet,
    child: Planet
}

impl Orbit {
    fn new(o: String) -> Orbit {
        let mut piter = o.split(")");
        return Orbit{parent:Planet{id:piter.next().unwrap().to_string()}, child:Planet{id:piter.next().unwrap().to_string()}}
    }
}

fn main() {
    // run the test cases given in the problem
    // change to 'input_modified.txt' to get answer
    let f = BufReader::new(File::open("src/input_test.txt").unwrap());

    // map each line as a separate program as a vector of numbers
    let mut orbits: Vec<Orbit> = f.lines()
        .map(|line| Orbit::new(line.unwrap().trim().to_string()) )
        .collect();

    let first: Vec<Orbit> = orbits.into_iter().filter(|o| o.parent.id=="H").collect();
    println!("{:?}", first);

    // println!("{:?}", orbits)

}