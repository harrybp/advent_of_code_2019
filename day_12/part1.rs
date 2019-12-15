fn main() {
    struct Moon {
        id: i32,
        position: Vec<i32>,
        velocity: Vec<i32>
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
    let steps = 1000;


    for _step in 0..steps {

        // Update velocities
        for i in 0..moons.len() {
            for j in i..(moons.len()) {
                if moons[i].id == moons[j].id {
                    continue
                }
                for axis in 0..3 {
                    if moons[i].position[axis] > moons[j].position[axis] {
                        moons[i].velocity[axis] -= 1;
                        moons[j].velocity[axis] += 1;
                    } else if moons[i].position[axis] < moons[j].position[axis] {
                        moons[i].velocity[axis] += 1;
                        moons[j].velocity[axis] -= 1;
                    }
                }
            }
        }

        // Update positions
        for i in 0..moons.len() {
            for axis in 0..3 {
                moons[i].position[axis] += moons[i].velocity[axis];
            }
        }
    }

    // Work out energy in system
    let mut energy = 0;
    for moon in moons {
        let potential = moon.position[0].abs() + moon.position[1].abs() + moon.position[2].abs();
        let kinetic = moon.velocity[0].abs() + moon.velocity[1].abs() + moon.velocity[2].abs();
        let total = potential * kinetic;
        energy += total;
    }
    println!("Energy: {}", energy);



}

