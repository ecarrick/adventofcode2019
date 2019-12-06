use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // run the test cases given in the problem
    // change to 'input_modified.txt' to get answer
    let f = BufReader::new(File::open("src/input_test.txt").unwrap());

    // map each line as a separate program as a vector of numbers
    let mut progs: Vec<Vec<usize>> = f.lines()
        .map(|line| line.unwrap().trim().split(",")
            .flat_map(|num| num.parse()).collect()
        )
        .collect();


    // println!("{:?}", progs[0]);

    for p in 0..progs.len() {

        println!("{:?}", progs[p]);
        for i in (0..progs[p].len()).step_by(4) {
            if progs[p][i] == 1 {
                // print!("add");
                let offset = progs[p][i+3];
                // print!("{} ", progs[p][i+1]);
                // println!("{} ", progs[p][i+2]);
                progs[p][offset] = progs[p][progs[p][i+1]]+progs[p][progs[p][i+2]];
            }
            else if progs[p][i] == 2{
                // print!("mult");
                let offset = progs[p][i+3];
                progs[p][offset] =  progs[p][progs[p][i+1]]*progs[p][progs[p][i+2]];
            }
            else if progs[p][i] == 99 {
                println!("{:?}", progs[p]);
                break;
            }
            else {
                panic!("encountered unknown opcode {}", progs[p][i]);
            }
            println!("{} {} {} {}",progs[p][i], progs[p][i+1],progs[p][i+2],progs[p][i+3])
        }
        println!("--------------------------------------------------")

    }
}