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

    let output = run_program(list);
    println!("{}", output);
}

fn get_parameter_modes(mut opcode: i32) -> (i32, i32, i32, i32) {
    //Get parameter modes
    let mut param_1_mode = 0;
    let mut param_2_mode = 0;
    let mut param_3_mode = 0;
    if opcode > 4 {
        let opcode_string: String = opcode.to_string();
        let mut count = 0;
        for i in (0..opcode_string.len()).rev() {
            let opcode_position: i32 = (opcode_string.as_bytes()[i] as char).to_digit(10).unwrap() as i32;
            if count == 4 {
                param_3_mode = opcode_position;
            } else if count == 3 {
                param_2_mode = opcode_position;
            } else if count == 2 {
                param_1_mode = opcode_position;
            } else if count == 0 {
                opcode = opcode_position;
            }
            count += 1;
        }
    }
    return (opcode, param_1_mode, param_2_mode, param_3_mode);
}

fn operation_1(list: &mut Vec<i32>, param_1: i32, param_2: i32, param_3: i32) -> i32{
    list[param_3 as usize] = param_1 + param_2;
    return 4;
}

fn operation_2(list: &mut Vec<i32>, param_1: i32, param_2: i32, param_3: i32) -> i32{
    list[param_3 as usize] = param_1 * param_2;
    return 4;
}


fn run_program(mut list: Vec<i32>) -> i32 {
    let mut index: i32 = 0;
    let mut opcode: i32;
    let mut increment: i32 = 4;
    let input = 0;
    let mut output = 0;
    loop {
        opcode = list[index as usize];
        let (opcode, param_1_mode, param_2_mode, param_3_mode) = get_parameter_modes(opcode);
        if opcode == 9 {
            break
        }

        //Get parameter 1
        let param_1 = if param_1_mode == 0 {
            list[list[(index + 1) as usize] as usize]
        } else {
            list[(index + 1) as usize]
        };
        
        //Input / output operations
        if opcode == 3 {
            list[param_1 as usize] = input;
            index += 2;
            continue
        } else if opcode == 4 {
            output = param_1;
            index += 2;
            continue
        }

        // Get parameter 2 & 3
        let param_2 = if param_2_mode == 0 {
            list[list[(index + 2) as usize] as usize]
        } else {
            list[(index + 2) as usize]
        };
        let param_3 = if (param_3_mode == 0) & false {
            list[list[(index + 3) as usize] as usize]
        } else {
            list[(index + 3) as usize]
        };

        //Integer processing operations
        if opcode == 1 {
            increment = operation_1(&mut list, param_1, param_2, param_3);
        } else if opcode == 2 {
            increment = operation_2(&mut list, param_1, param_2, param_3);
        }

        index += increment;
    }
    return output
}


