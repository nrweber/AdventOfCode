use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').collect();

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}

#[derive(Debug)]
struct Location {
    left : String,
    right : String
}

impl Location {
    fn new(left: String, right : String) -> Location {
        return Location { left, right };
    }
}

fn part1(lines : &Vec<&str>) -> i32 {
    let instructions : Vec<char> = lines[0].chars().collect();

    let mut locs : HashMap<String, Location> = HashMap::<String, Location>::new();

    for i in 2..lines.len() {
        if lines[i] == "" {
            continue;
        }

        //println!("{}", lines[i]);
        let line = lines[i].replace(",", "").replace("(", "").replace(")", "");
        let split : Vec<&str> = line.split(" ").collect();

        locs.insert(split[0].to_string(), Location::new(split[2].to_string(), split[3].to_string() ));

        //println!("{:?}", split);
    }

    //println!("{:?}", locs);

    let mut jumps : i32 = 0;
    let mut current_location : &String = &("AAA".to_string());
    while current_location != "ZZZ" {
        //println!("start: '{}'", current_location);
        //println!("{:?}", locs);
        //
        let instruction_key = instructions[(jumps as usize) % instructions.len()];
        if instruction_key == 'R' {
            current_location = &locs[current_location].right;
        }
        else {
            current_location = &locs[current_location].left;
        }
        //println!("to: '{}'", current_location);
        //println!("-------");

        jumps += 1;
    }
    jumps
}

fn part2(lines : &Vec<&str>) -> i32 {
    let instructions : Vec<char> = lines[0].chars().collect();

    let mut current_locations : Vec<String> = Vec::new();

    let mut locs : HashMap<String, Location> = HashMap::<String, Location>::new();

    for i in 2..lines.len() {
        if lines[i] == "" {
            continue;
        }

        //println!("{}", lines[i]);
        let line = lines[i].replace(",", "").replace("(", "").replace(")", "");
        let split : Vec<&str> = line.split(" ").collect();

        locs.insert(split[0].to_string(), Location::new(split[2].to_string(), split[3].to_string() ));

        //println!("{:?}", split);
        
        if split[0].ends_with("A") {
            current_locations.push(split[0].to_string())
        }
    }

    //println!("{:?}", locs);

    let mut jumps : i32 = 0;
    let mut all_done = false;

    while all_done == false {
        all_done = true;
        for i in 0..current_locations.len() {
            let mut current_location : &String = &(current_locations[i]);
            let instruction_key = instructions[(jumps as usize) % instructions.len()];
            if instruction_key == 'R' {
                current_location = &locs[current_location].right;
            }
            else {
                current_location = &locs[current_location].left;
            }

            current_locations[i] = current_location.clone();

            if current_locations[i].ends_with("Z") == false {
                all_done = false;
            }

        }

        //println!("{:?}", current_locations);
        jumps += 1;

        if jumps % 100000000 == 0{
            println!("{}", jumps);
        }
    }
    jumps
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let lines : Vec<&str> = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".split("\n").collect();
        let answer : i32 = part1(&lines);
        assert_eq!(answer, 2);

        let lines : Vec<&str> = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)".split("\n").collect();
        let answer : i32 = part1(&lines);
        assert_eq!(answer, 6);
    }

    #[test]
    fn part2_test() {
        let lines : Vec<&str> = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)".split("\n").collect();
        let answer : i32 = part2(&lines);
        assert_eq!(answer, 6);
    }

}
