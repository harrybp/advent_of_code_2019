use std::fs::File;
use std::io::{BufRead, BufReader};


// Return the program as a list of ints
fn get_command_list() -> Vec<i128> {
    //Read line in and remove newline
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let _ = reader.read_line(&mut line);
    line.truncate(line.len() - 1);

    //Split into vector of ints
    let split = line.split(",");
    let mut list: Vec<i128> = Vec::new();
    for item in split {
       list.push(item.parse::<i128>().unwrap());
    }
    let extra_memory = 500;
    for _i in 0..extra_memory {
        list.push(0);
    }
    return list;
}

fn main() {
    let mut program: Vec<i128> = get_command_list();
    let mut program_index = 0;
    let mut relative_base = 0;
    let mut count_blocks = 0;

    loop {
        let (x,y,z,o) = run_program(program.clone(), program_index, relative_base, 0);
        program = x.clone();
        program_index = y;
        relative_base = z;
        if program.len() == 0 {
            break
        }
        if o[2] == 2 {
            count_blocks += 1;
        }
    }
    println!("There were {} blocks", count_blocks);
}

fn run_program(mut list: Vec<i128>, mut index: i128, mut relative_base: i128, input: i128) -> (Vec<i128>, i128, i128, Vec<i128>) {
    let mut opcode: i128;
    let mut output: Vec<i128> = vec![];

    loop {
        //println!("P");
        opcode = list[index as usize];
        if opcode == 99 {
            list = vec![];
            break
        }
        let (opcode, param_1_mode, param_2_mode, param_3_mode) = get_parameter_modes(opcode);

        // Get parameter 1
        let param_1 = if (param_1_mode == 0) & (opcode != 3) {
            list[list[(index + 1) as usize] as usize]
        } else if (param_1_mode == 2) & (opcode != 3) {
            list[(list[(index + 1) as usize] + relative_base) as usize]
        } else if param_1_mode == 2 {
            list[(index + 1) as usize] + relative_base
        } else {
            list[(index + 1) as usize]
        };

        // --------------------------------------------------------------------
        // [3,4] : Input / output operations
        if opcode == 3 { //input
            list[param_1 as usize] = input;
            index += 2;
            continue
        } else if opcode == 4 {//output
            //println!("S");
            index += 2;
            output.push(param_1);
            if output.len() < 3 {
                continue
            } else {
                break
            }
        } else if opcode == 9 {
            relative_base += param_1;
            index += 2;
            continue
        }
        // --------------------------------------------------------------------
        // Get parameter 2
        let param_2 = if param_2_mode == 0 {
            list[list[(index + 2) as usize] as usize]
        } else if param_2_mode == 2 {
            list[(list[(index + 2) as usize] + relative_base) as usize]
        } else {
            list[(index + 2) as usize]
        };


        // --------------------------------------------------------------------
        // [5,6] : Jump if true/false
        if opcode == 5 {//jump if true
            if param_1 != 0 {
                index = param_2;
            } else {
                index += 3;
            }
            continue
        } else if opcode == 6 {//jump if false
           if param_1 == 0 {
                index = param_2;
           } else {
               index += 3;
           }
            continue
        }
        // --------------------------------------------------------------------
        // Get parameter 3
        let param_3 = if (param_3_mode == 0) & false {
            list[list[(index + 3) as usize] as usize]
        } else if param_3_mode == 2 {
            list[(index + 3) as usize] + relative_base
        } else {
            list[(index + 3) as usize]
        };

        // --------------------------------------------------------------------
        // [7,8] : Conditionals
        if opcode == 7 { //if a < b
           if param_1 < param_2 {
               list[param_3 as usize] = 1;
           } else {
                list[param_3 as usize] = 0;
           }
        }
        if opcode == 8 { //if a == b
            if param_1 == param_2 {
                list[param_3 as usize] = 1;
            } else {
                list[param_3 as usize] = 0;
            }
        }
        // --------------------------------------------------------------------

        // --------------------------------------------------------------------
        // [1,2] : Integer processing operations
        if opcode == 1 { //add
            list[param_3 as usize] = param_1 + param_2;
        } else if opcode == 2 { //multiply
            list[param_3 as usize] = param_1 * param_2;
        }
        index += 4;
        // --------------------------------------------------------------------
    }
    return (list, index, relative_base, output);
}

// Process a raw opcode to extract opcode plus the three parameter modes
fn get_parameter_modes(mut opcode: i128) -> (i128, i128, i128, i128) {
    //Get parameter modes
    let mut param_1_mode = 0;
    let mut param_2_mode = 0;
    let mut param_3_mode = 0;
    if opcode > 4 {
        let opcode_string: String = opcode.to_string();
        let mut count = 0;
        for i in (0..opcode_string.len()).rev() {
            let opcode_position: i128 = (opcode_string.as_bytes()[i] as char).to_digit(10).unwrap() as i128;
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
