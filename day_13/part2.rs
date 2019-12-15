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

    list[0] = 2;
    return list;
}

fn main() {
    let mut program: Vec<i128> = get_command_list();
    let mut program_index = 0;
    let mut relative_base = 0;
    let mut score = 0;

    let mut game: Vec<Vec<i128>> = vec![vec![0; 35]; 25];

    let mut ball_x = 0;
    let mut my_x = 0;
    let mut count = 0;
    let mut control = 0;

    // To visualise the game - a delay is required
    let print_on: bool = false;
    let ten_millis = std::time::Duration::from_millis(20);

    loop {
        let (x,y,z,o) = run_program(program.clone(), program_index, relative_base, control);
        program = x.clone();
        program_index = y;
        relative_base = z;
        if program.len() == 0 {
            break
        }

        // Save output to score/game
        if o[0] == -1 {
            score = o[2];
        } else {
            game[o[1] as usize][o[0] as usize] = o[2];
        }

        // Save x position of ball and self
        if o[2] == 4 {
            ball_x = o[0];
        } else if o[2] == 3 {
            my_x = o[0];
        }

        // Adjust controls to hit ball
        if my_x < ball_x {
            control = 1;
        } else if my_x > ball_x {
            control = -1;
        } else {
            control = 0;
        }

        // Print the game 
        if print_on && (count > 878) && (count % 2 == 1) {
            std::thread::sleep(ten_millis);
            print_game(game.clone(), score, control);
        }
        count+= 1;
    }

    println!("Final score is {}", score);
}

fn print_game(game: Vec<Vec<i128>>, score: i128, control: i128) {
    for line in game {
        for item in line {
            if item == 0 {
                print!(" ");
            } else if item == 1 {
                print!("|");
            } else if item == 2{
                print!("#");
            } else if item == 3 {
                print!("-");
            } else if item == 4 {
                print!("o");
            }

        }
        println!();
    }
    println!("Score: {}, Moving: {}", score, control);
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
