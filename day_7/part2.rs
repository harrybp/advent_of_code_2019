use std::fs::File;
use std::io::{BufRead, BufReader};

// Return the program as a list of ints
fn get_command_list() -> Vec<i32> {
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
    return list;
}


fn main() {
    // Try every unique 5 digit combination of phases, save the highest output
    let mut highest_signal = 0;
    for amp_a_param in 5..10 {
        for amp_b_param in 5..10 {
            for amp_c_param in 5..10 {
                for amp_d_param in 5..10 {
                    for amp_e_param in 5..10 {
                        let slice = vec![amp_a_param, amp_b_param, amp_c_param, amp_d_param, amp_e_param];

                        // Make sure all elements are unique
                        if (1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1])) {
                            continue
                        }
                        let result = try_combination(slice);
                        if result > highest_signal {
                            highest_signal = result;
                        }
                    }
                }
            }
        }
    }
    println!("{}", highest_signal);
}

fn try_combination(amp_phases: Vec<i32>) -> i32 {

    // Set up initial program states
    let mut result = 0;
    let mut program_states: Vec<Vec<i32>> = vec![get_command_list(), get_command_list(), get_command_list(), get_command_list(), get_command_list()];
    let mut program_indexes: Vec<i32> = vec![0,0,0,0,0];

    // Feed each program its phase
    for i in 0..5 {
        let (x, y, _z) = run_program(get_command_list(), program_indexes[i], amp_phases[i], true);
        program_states[i] = x;
        program_indexes[i] = y;
    }

    // Run each program in round robin, moving to next one when it produces an output
    let mut last_e_output = 0;
    let mut index = 0;
    loop {
        let (x, y, z) = run_program(program_states[index].clone(), program_indexes[index], result, false);
        program_states[index] = x;
        program_indexes[index] = y;
        result = z;
        if index == 4 {
            last_e_output = z;
        }
        if program_states[index].len() == 0 {
            break
        }
        index = (index + 1) % 5;
    }
    return last_e_output;
}

// Process a raw opcode to extract opcode plus the three parameter modes
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

// Run the program returning (program code, current index, output)
// Use same args as input to restart from this point,
// Return after output opcode (or input opcode if setting_phase is true)
// If a 99 opcode occurs we set program code to empty list and return 
fn run_program(mut list: Vec<i32>, mut index: i32, input: i32, setting_phase: bool) -> (Vec<i32>, i32, i32) {
    let mut opcode: i32;
    let mut output = 0;

    loop {
        opcode = list[index as usize];
        let (opcode, param_1_mode, param_2_mode, param_3_mode) = get_parameter_modes(opcode);
        if opcode == 9 {
            list = vec![];
            break
        }

        // Get parameter 1
        let param_1 = if (param_1_mode == 0) & (opcode != 3) {
            list[list[(index + 1) as usize] as usize]
        } else {
            list[(index + 1) as usize]
        };

        // --------------------------------------------------------------------
        // [3,4] : Input / output operations
        if opcode == 3 { //input
            list[param_1 as usize] = input;
            index += 2;
            if setting_phase {
                break
            }
            continue
        } else if opcode == 4 {//output
            output = param_1;
            index += 2;
            break
        }
        // --------------------------------------------------------------------

        // Get parameter 2
        let param_2 = if param_2_mode == 0 {
            list[list[(index + 2) as usize] as usize]
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
    return (list, index, output);
}


