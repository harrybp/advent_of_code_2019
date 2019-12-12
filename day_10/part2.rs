use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut asteroids: Vec<(i32,i32)> = vec![];
    let station = (17,23);
    // Read map into vector of coordinates
    let mut y = 0;
    for line in reader.lines() {
        let mut x = 0;
        for item in line.unwrap().chars() {
            if item == '#' {
                if (x,y) != station {
                    asteroids.push((x,y));
                }
            }
            x += 1;
        }
        y += 1;
    }

    let mut visible_asteroids = get_visible_asteroids(asteroids.clone(), station);
    visible_asteroids.sort_by_key(|k| get_angle(station, *k));
    println!("The 200'th asteroid is ({},{})", visible_asteroids[199].0, visible_asteroids[199].1);
}

fn get_visible_asteroids(asteroids: Vec<(i32,i32)>, source: (i32,i32)) -> Vec<(i32,i32)> {
    let mut visible_asteroids: Vec<(i32,i32)> = vec![];
    for asteroid in asteroids.clone() {
        if can_see(&asteroids, source, asteroid) {
            visible_asteroids.push(asteroid);
        }
    }
    return visible_asteroids
}

fn get_angle(source: (i32,i32), target: (i32,i32)) -> i32 {
    // Get angle between vector (source -> target) and y axis
    let y2 = (source.1 - target.1) as f64;
    let x2 = (source.0 - target.0) as f64;
    let x1 = 0.0 as f64;
    let y1 = 1.0 as f64;
    let result_1 = y2.atan2(x2) - y1.atan2(x1);
    let result_2 = result_1 * (180.0 / std::f64::consts::PI);
    let result_3 = (result_2  + 360.0) % 360.0;
    return (result_3*1000.0) as i32;
}


fn can_see(asteroids: &Vec<(i32,i32)>, source_asteroid: (i32,i32), target_asteroid: (i32,i32)) -> bool {
    // Loop over asteroids to check if they are in the way
    for asteroid in asteroids.clone() {
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
