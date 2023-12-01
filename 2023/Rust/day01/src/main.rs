use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').collect();

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}

fn part1(lines : &Vec<&str>) -> u32 {
    let mut total : u32 = 0;
    for line in lines {
        for c in line.chars() {
            if c.is_digit(10) {
                total += c.to_digit(10).unwrap() * 10;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                total += c.to_digit(10).unwrap();
                break;
            }
        }
    }

    total
}

fn part2(lines : &Vec<&str>) -> u32 {
    let mut total : u32 = 0;
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                total += c.to_digit(10).unwrap() * 10;
                break;
            }
            else {
                if line[i..].starts_with("one") { total += 10; break; }
                if line[i..].starts_with("two") { total += 20; break; }
                if line[i..].starts_with("three") { total += 30; break; }
                if line[i..].starts_with("four") { total += 40; break; }
                if line[i..].starts_with("five") { total += 50; break; }
                if line[i..].starts_with("six") { total += 60; break; }
                if line[i..].starts_with("seven") { total += 70; break; }
                if line[i..].starts_with("eight") { total += 80; break; }
                if line[i..].starts_with("nine") { total += 90; break; }
            }
        }

        for (i, c) in line.chars().rev().enumerate() {
            let j = line.len()-1-i;
            if c.is_digit(10) {
                total += c.to_digit(10).unwrap();
                break;
            }
            else {
                if line[j..].starts_with("one") { total += 1; break; }
                if line[j..].starts_with("two") { total += 2; break; }
                if line[j..].starts_with("three") { total += 3; break; }
                if line[j..].starts_with("four") { total += 4; break; }
                if line[j..].starts_with("five") { total += 5; break; }
                if line[j..].starts_with("six") { total += 6; break; }
                if line[j..].starts_with("seven") { total += 7; break; }
                if line[j..].starts_with("eight") { total += 8; break; }
                if line[j..].starts_with("nine") { total += 9; break; }
            }
        }
    }

    total
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let lines : Vec<&str> = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".split("\n").collect();
        let answer : u32 = part1(&lines);
        assert_eq!(answer, 142);
    }

    #[test]
    fn part2_test() {
        let lines : Vec<&str> = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".split("\n").collect();
        let answer : u32 = part2(&lines);
        assert_eq!(answer, 281);
    }

}
