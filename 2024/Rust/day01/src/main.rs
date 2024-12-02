use std::fs;

fn main() {
    let lines : Vec<String> = process_file("input.txt");

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}

fn process_file(filename : &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    let lines : Vec<String> = contents.split('\n').map(str::to_string).collect();
    return lines;
}


fn part1(lines : &Vec<String>) -> i32 {
    let mut one : Vec<i32> = Vec::new();
    let mut two : Vec<i32> = Vec::new();

    for line in lines {
        let parts : Vec<&str> = line.split("   ").collect();   

        if parts[0] != "" {
            one.push(parts[0].parse().unwrap());
            two.push(parts[1].parse().unwrap());
        }
    }


    one.sort();
    two.sort();

    let mut total : i32 = 0;
    for i in 0..one.len() {
        total += (one[i] - two[i]).abs();
     }
    total
}

fn part2(_lines : &Vec<String>) -> i32 {
    0
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let lines = process_file("example.txt");
        let answer : i32 = part1(&lines);
        assert_eq!(answer, 11);
    }

    #[test]
    fn part2_test() {
        let lines = process_file("example.txt");
        let answer : i32 = part2(&lines);
        assert_eq!(answer, 0);
    }

}
