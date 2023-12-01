use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').collect();

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}

fn part1(_lines : &Vec<&str>) -> i32 {
    0
}

fn part2(_lines : &Vec<&str>) -> i32 {
    0
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let lines : Vec<&str> = "
".split("\n").collect();
        let answer : i32 = part1(&lines);
        assert_eq!(answer, 0);
    }

    #[test]
    fn part2_test() {
        let lines : Vec<&str> = "
".split("\n").collect();
        let answer : i32 = part2(&lines);
        assert_eq!(answer, 0);
    }

}
