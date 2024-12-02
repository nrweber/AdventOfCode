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
    println!("{:?}", lines);
    0
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
        assert_eq!(answer, 1);
    }

    #[test]
    fn part2_test() {
        let lines = process_file("example.txt");
        let answer : i32 = part2(&lines);
        assert_eq!(answer, 0);
    }

}
