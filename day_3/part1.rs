use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // HashSet for storing visited positions, vec for storing intersects
    let mut locations_hit: HashSet<(i32,i32)> = HashSet::new();
    let mut intersections: Vec<(i32,i32)> = Vec::new();

    for line in reader.lines() {
        //Split line into vector of commands
        let line_str: String = line.unwrap();
        let split = line_str.split(",");

        let mut pos_x: i32 = 0;
        let mut pos_y: i32 = 0;

        //Hashset for positions visited for this wire only
        let mut locations_hit_now: HashSet<(i32,i32)> = HashSet::new();

        for item in split {
            //Split command into direction and amount
            let direction = &item[..1];
            let amount_str = &item[1..];
            let amount: i32 = amount_str.parse().unwrap();

            //Loop over all positions and add to hashset
            for i in 0..amount {
                if direction == "U" {
                    pos_y -= 1;
                } else if direction == "D" {
                    pos_y += 1;
                } else if direction == "L" {
                    pos_x -= 1;
                } else if direction == "R" {
                    pos_x += 1;
                }

                //If position is new for this wire then add to global hashset
                let local_clash = locations_hit_now.insert((pos_x,pos_y));
                if local_clash {
                    //If position is not new for global hashset then its a collision
                    let collision = locations_hit.insert((pos_x, pos_y));
                    if !collision {
                        intersections.push((pos_x, pos_y));
                    }
                }
            }
        }
    }

    //Loop over collisions and find lowest manhattan distance
    let mut lowest_distance: i32 = std::i32::MAX;
    for intersect in intersections {
        let distance: i32 = intersect.0.abs() + intersect.1.abs();
        println!("{} {}: {}", intersect.0, intersect.1, distance);
        if distance < lowest_distance {
            lowest_distance = distance;
        }
    }
    println!("Lowest distance is {}", lowest_distance);
}
