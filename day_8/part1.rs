use std::fs::File;
use std::io::{BufRead, BufReader};

// Return the image as a list of ints
fn get_image_list() -> Vec<u32>{
    //Read line in and remove newline
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let _ = reader.read_line(&mut line);
    line.truncate(line.len() - 1);

    //Split into vector of ints
    let mut list: Vec<u32> = vec![];
    for item in line.chars() {
        list.push(item.to_digit(10).unwrap());
    }
    return list;
}

fn main() {
    let image_list = get_image_list();
    let width =  25;
    let height = 6;
    let num_layers = image_list.len() / (width * height);

    let mut lowest_num_zeros = 9999999;
    let mut lowest_num_zeros_layer = 0;
    let mut num_ones_layers: Vec<u32> = vec![];
    let mut num_twos_layers: Vec<u32> = vec![];

    // Track # of 1's and 2's in each layer and record layer with lowest # of 0's
    for layer in 0..num_layers {
        let mut num_zeros = 0;
        let mut num_ones = 0;
        let mut num_twos = 0;

        for pixel in 0..(width*height) {
            if image_list[(layer * width * height) + pixel] == 0 {
                num_zeros += 1;
            } else if image_list[(layer * width * height) + pixel] == 1 {
                num_ones += 1;
            } else if image_list[(layer * width * height) + pixel] == 2 {
                num_twos += 1;
            }
        }
        if num_zeros < lowest_num_zeros {
            lowest_num_zeros = num_zeros;
            lowest_num_zeros_layer = layer;
        }

        num_ones_layers.push(num_ones);
        num_twos_layers.push(num_twos);
    }

    println!("{} layers of {} x {}", num_layers, width, height);
    println!("Lowest zeros is {} in layer {}, num_ones ({}) * num_twos ({}) = {}", 
            lowest_num_zeros, lowest_num_zeros_layer, 
            num_ones_layers[lowest_num_zeros_layer], num_twos_layers[lowest_num_zeros_layer],
            num_ones_layers[lowest_num_zeros_layer] * num_twos_layers[lowest_num_zeros_layer]);
}
