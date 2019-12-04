use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    //Read line in and remove newline
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let _ = reader.read_line(&mut line);
    line.truncate(line.len() - 1);

    //Split into vector of ints
    let split = line.split(",");
    let mut list: Vec<i32> = Vec::new();
    for item in split {
       list.push(item.parse::<i32>().unwrap());
    }

    let target: i32 = 19690720;
    for i in 0..99 {
        for j in 0..99 {
            let mut list_copy = list.clone();
            list_copy[1] = i;
            list_copy[2] = j;
            let result: i32 = run_program(list_copy);
            if result == target {
                println!("{} & {}: {}", i, j, (100 * i) + j);
                break
            }
        }
    }
}

fn run_program(mut list: Vec<i32>) -> i32 {
    let mut index: i32 = 0;
    let mut opcode: i32;
    loop {
        opcode = list[index as usize];
        if opcode == 99 {
            break
        }

        //Get operands
        let src_a = list[(index + 1) as usize];
        let src_b = list[(index + 2) as usize];
        let dest = list[(index + 3) as usize];

        //Do operation
        if opcode == 1 {
            list[dest as usize] = list[src_a as usize] + list[src_b as usize];
        } else if opcode == 2 {
            list[dest as usize] = list[src_a as usize] * list[src_b as usize];
        }

        index += 4;
    }
    return list[0]
}
