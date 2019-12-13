use std::fs::File;
use std::io::{BufRead, BufReader};
#[macro_use] extern crate text_io;

fn main() {
    let f = BufReader::new(File::open("src/input.txt").unwrap());

    // map each line to a number
    let mut progs: Vec<Vec<i32>> = f.lines()
        .map(|line| line.unwrap().trim().split(",")
            .flat_map(|num| num.parse()).collect()
        )
        .collect();

    // let mut progiter = &mut progs[0].iter();
    let mut i = 0;
    // for i in 0..progs[0].len() {
    while i < progs[0].len() {
        
        // for intcode in progs[0].iter() {
        let op = progs[0][i];
        let opcode =  op % 100;
        let mut params = vec![false, false, false];
        let mut param_num = op /100;
        let mut xi = 0;
        while param_num > 0 {
            let param = param_num % 10;
            if param == 1 {params[xi] = true}
            xi +=1;
            param_num /= 10;
        }
        // print!("{} ", i);
        // println!("intcode:{:?}, op:{:?}, p1:{:?}, p2:{:?}, p3:{:?}",op, opcode, params[0], params[1], params[2]);
       
        match opcode {
            1 => {
                // progs[0][param3] = progs[0][param1]+progs[0][pram2]
                i+=1;
                let param1 = progs[0][i];
                i+=1;
                let param2 = progs[0][i];
                i+=1;
                let param3 = progs[0][i];
                let a = if params[0] {param1} else {progs[0][param1 as usize]};
                let b = if params[1] {param2} else {progs[0][param2 as usize]};
                progs[0][param3 as usize] = a+b;
            },
            2 => {
                // progs[0][param3] = progs[0][param1]*progs[0][pram2]
                i+=1;
                let param1 = progs[0][i];
                i+=1;
                let param2 = progs[0][i];
                i+=1;
                let param3 = progs[0][i];
                let a = if params[0] {param1} else {progs[0][param1 as usize]};
                let b = if params[1] {param2} else {progs[0][param2 as usize]};
                // println!("intcode:{:?}, op:{:?}, p1:{:?}, p2:{:?}, p3:{:?}, a:{:?}, b:{:?}",op, opcode, param1, param2, param3, a, b);
                progs[0][param3 as usize] = a*b;
            },
            3 => {
                // println!("Input Required: ");
                // let immediate: i32 = read!();
                i+=1;
                let param1 = progs[0][i];
                // progs[0][param1 as usize] = immediate;
                progs[0][param1 as usize] = 1;
            },
            4 => {
                // print!("{} ", i);
                // println!("intcode:{:?}, op:{:?}, p1:{:?}, p2:{:?}, p3:{:?}",op, opcode, params[0], params[1], params[2]);
                i+=1;
                let param1 = progs[0][i];
                let toprint = if params[0] {param1} else {progs[0][param1 as usize]};
                // println!("{} ", param1);
                println!("program output: {}", toprint);
            },

            99 => {
                break;
            },
            _ => panic!("encountered unknown opcode {}", op),
        }
        i+=1;
    }

}