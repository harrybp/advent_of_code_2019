use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut asteroids: Vec<(i32,i32)> = vec!();

    // Read map into vector of coordinates
    let mut y = 0;
    for line in reader.lines() {
        let mut x = 0;
        for item in line.unwrap().chars() {
            if item == '#' {
                asteroids.push((x,y));
            }
            x += 1;
        }
        y += 1;
    }
    
    // Loop over each pair of asteroids and check for line of sight
    let mut highest_can_see = 0;
    let mut highest_asteroid = asteroids[0];
    for source in 0..asteroids.len() {
        let mut asteroids_can_see = 0;
        for target in 0..asteroids.len() {
            let result = can_see(asteroids.clone(), source, target);
            if result & (target != source) {
                asteroids_can_see += 1;
            }
        }
        if asteroids_can_see > highest_can_see {
            highest_can_see = asteroids_can_see;
            highest_asteroid = asteroids[source];
        }
    }
    println!("Best asteroid is ({},{}), it can see {} others", highest_asteroid.0, highest_asteroid.1, highest_can_see);
}

fn can_see(asteroids: Vec<(i32,i32)>, source: usize, target: usize) -> bool {
    // Loop over asteroids to check if they are in the way
    let source_asteroid = asteroids[source];
    let target_asteroid = asteroids[target];
    for asteroid in asteroids {
        if (asteroid == source_asteroid) | (asteroid == target_asteroid) {
            continue
        }
        if on_same_line(source_asteroid, asteroid, target_asteroid) {
            return false;
        }
    }
    return true;
}

fn on_same_line(start: (i32, i32), middle: (i32, i32), end: (i32, i32)) -> bool {
    // Check that middle is actually between the start and end
    if !((((start.0 >= middle.0) & (middle.0 >= end.0)) | ((start.0 <= middle.0) & (middle.0 <= end.0))) & 
         (((start.1 >= middle.1) & (middle.1 >= end.1)) | ((start.1 <= middle.1) & (middle.1 <= end.1)))) {
        return false
    }
    // Colinear equation, checks three points are on the same line
    return ((start.1 - middle.1) * (start.0 - end.0)) == ((start.1 - end.1) * (start.0 - middle.0));
}
