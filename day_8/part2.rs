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

    let mut image: Vec<u32> = vec![];

    // For each pixel, loop through layers until we find non-transparent value
    for pixel in 0..(width*height) {
        for layer in 0..num_layers {
            let pixel_val =  image_list[(layer * width * height) + pixel];
            if pixel_val != 2 {
                image.push(pixel_val);
                break
            }
        }
    }

    // Print the resulting image
    for j in 0..height {
        for i in 0..width {
           let pixel = image[(j * width) + i];
           if pixel == 1 {
               print!("#");
           } else {
               print!(" ");
           }
        }
        println!();
    }

}
