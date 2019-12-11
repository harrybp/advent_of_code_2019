use std::fs::File;
use std::io::{BufRead, BufReader};

struct Node {
    children: Vec<Node>,
    data: String
}

impl Node {
    pub fn new(my_data: String) -> Node {
        Node {
            children: vec!(),
            data: my_data
        }
    }

    // Sums up the number of transitive orbits
    pub fn orbit_count(&mut self, depth: i32) -> i32 {
        let mut size: i32 = depth;
        for child in self.children.iter_mut() {
            size += child.orbit_count(depth+1);
        }
        return size;
    }

    // Returns True if a node with given data is in the tree
    pub fn find(&mut self, data: &String) -> bool {
        if &self.data == data {
            return true;
        } else {
            let mut found: bool = false;
            for node in self.children.iter_mut() {
                if node.find(data) {
                    found = true;
                }
            }
            return found;
        }
    }

    // Inserts child node into node with given data
    pub fn find_and_insert(&mut self, data: &String, child:  Node) -> bool {
        if &self.data == data {
            self.add_child(child);
            return true
        } else {
            for node in self.children.iter_mut() {
                if node.find(data) {
                    node.find_and_insert(data, child);
                    return true
                }
            }
        }
        return false
    }

    // Returns string representation of the tree
    pub fn to_str(&self, depth: usize) -> String {
        let mut string_rep: String = format!("{}:", self.data);
        for node in self.children.iter() {
            string_rep = format!("{0}\n{1:width$}{2}", string_rep, " ", node.to_str(depth + 1), width=(depth+1)*2);
        }
        return string_rep
    }

    // Add a node as a child to this node
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn get_orbit_list(&mut self, data: &String) -> Vec<String> {
        if self.find(data) {
            let mut orbit_list: Vec<String> = vec![self.data.clone()];
            for child in self.children.iter_mut() {
                if child.find(data) {
                    orbit_list.append(&mut child.get_orbit_list(data));
                    return orbit_list
                }
            }
        } 
        return vec![];
    }

    pub fn print_orbit_list(&mut self, data: &String) {
        let orbit_list: Vec<String> = self.get_orbit_list(data);
        for planet in orbit_list {
            print!("{} -> ", planet);
        }
        println!("{}", data);
    }

}


fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut orbits: Vec<(String,String)> = vec!();

    for line in reader.lines() {
        //Get orbiter and orbited planets
        let line_str: String = line.unwrap();
        let split = line_str.split(")");
        let vec: Vec<&str> = split.collect();
        let orbit_ed: String = vec[0].to_string();
        let orbit_er: String = vec[1].to_string();
        orbits.push((orbit_ed, orbit_er));
    }

    //Build tree
    let mut root: Node = Node::new("COM".to_string());

    while orbits.len() > 0 {
        let (orbit_ed, orbit_er): (String, String) = orbits.remove(0);
        if !root.find(&orbit_ed) {
            orbits.push((orbit_ed, orbit_er));
        } else {
            let new_planet: Node = Node::new(orbit_er);
            root.find_and_insert(&orbit_ed, new_planet);
        }
    }

    // Get the paths from root to YOU and SAN
    let orbit_list_you: Vec<String> = root.get_orbit_list(&"YOU".to_string());
    let orbit_list_san: Vec<String> = root.get_orbit_list(&"SAN".to_string());

    // Find where the paths diverge
    let mut index = 0;
    loop {
        if orbit_list_you[index] != orbit_list_san[index] {
            break
        }
        index += 1;
    }

    // Answer is how far from divergence point to YOU and SAN
    let answer = (orbit_list_you.len() - index) + (orbit_list_san.len() - index);
    println!("{}", answer);
    
}
