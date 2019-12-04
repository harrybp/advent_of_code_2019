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

    list[1] = 12;
    list[2] = 2;

    let mut index: i32 = 0;
    let mut opcode: i32 = list[index as usize];
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
    println!("{}", list[0]);
}
