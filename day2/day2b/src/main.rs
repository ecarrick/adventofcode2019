use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = BufReader::new(File::open("src/input.txt").unwrap());

    // map each line to a number
    let progs: Vec<Vec<usize>> = f.lines()
        .map(|line| line.unwrap().trim().split(",")
            .flat_map(|num| num.parse()).collect()
        )
        .collect();

    'noun: for noun in 0..99 {
        'verb: for verb in 0..99 {
            let mut prog = progs[0].clone();
            prog[1] = noun;
            prog[2] = verb;
            for i in (0..prog.len()).step_by(4) {
                if prog[i] == 1 {
                    // print!("add");
                    let offset = prog[i+3];
                    prog[offset] = prog[prog[i+1]]+prog[prog[i+2]];
                }
                else if prog[i] == 2{
                    // print!("mult");
                    let offset = prog[i+3];
                    prog[offset] =  prog[prog[i+1]]*prog[prog[i+2]];
                }
                else if prog[i] == 99 {
                    // println!("{:?}", prog[0]);
                    break;
                }
                else {
                    panic!("encountered unknown opcode {}", prog[i]);
                }
                // println!("{} {} {} {}",prog[i], prog[i+1],prog[i+2],prog[i+3])
            }
            if prog[0] == 19690720 {
                println!("noun: {} verb: {}", noun, verb);
                break 'noun;
            }
        }
    }

}