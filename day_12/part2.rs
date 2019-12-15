use std::collections::HashSet;

fn main() {
    struct Moon {
        id: i128,
        position: Vec<i128>,
        velocity: Vec<i128>
    }
    let moon_0 = Moon {
        id: 0,
        position: vec![-8, -18, 6],
        velocity: vec![0, 0, 0]
    };

    let moon_1 = Moon {
        id: 1,
        position: vec![-11, -14, 4],
        velocity: vec![0, 0, 0]
    };

    let moon_2 = Moon {
        id: 2,
        position: vec![8, -3, -10],
        velocity: vec![0, 0, 0]
    };

    let moon_3 = Moon {
        id: 3,
        position: vec![-2, -16, 1],
        velocity: vec![0, 0, 0]
    };

    let mut moons: Vec<Moon> = vec![moon_0, moon_1, moon_2, moon_3];

    for axis in 0..3 {
        let mut count = 0;
        let mut states: HashSet<String> = HashSet::new();
        loop {

            // Update velocities
            for i in 0..moons.len() {
                for j in i..(moons.len()) {
                    if moons[i].id == moons[j].id {
                        continue
                }
                    if moons[i].position[axis] > moons[j].position[axis] {
                        moons[i].velocity[axis] -= 1;
                        moons[j].velocity[axis] += 1;
                    } else if moons[i].position[axis] < moons[j].position[axis] {
                        moons[i].velocity[axis] += 1;
                        moons[j].velocity[axis] -= 1;
                    }
                }
            }

            // Update positions
            for i in 0..moons.len() {
                moons[i].position[axis] += moons[i].velocity[axis];
            }

            let string_repr = format!("{}{}{}{}{}{} {}{}{}{}{}{} {}{}{}{}{}{} {}{}{}{}{}{}",
                    moons[0].position[0], moons[0].position[1], moons[0].position[2],
                    moons[0].velocity[0], moons[0].velocity[1], moons[0].velocity[2],
                    moons[1].position[0], moons[1].position[1], moons[1].position[2],
                    moons[1].velocity[0], moons[1].velocity[1], moons[1].velocity[2],
                    moons[2].position[0], moons[2].position[1], moons[2].position[2],
                    moons[2].velocity[0], moons[2].velocity[1], moons[2].velocity[2],
                    moons[3].position[0], moons[3].position[1], moons[3].position[2],
                    moons[3].velocity[0], moons[3].velocity[1], moons[3].velocity[2]);

            if !states.insert(string_repr) {
                break
            }
            count += 1;
        }

        println!("Steps for axis {}: {}", axis, count);
    }
    println!("Use google to find lowest common multiple of these three for the answer...");
}


