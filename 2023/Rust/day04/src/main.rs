use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').filter(|l| l.len() != 0).collect();

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}

fn part1(lines : &Vec<&str>) -> i32 {
    let mut total = 0;
    for line in lines {
        if line == &"" {
            continue;
        }

        let firstsplit : Vec<&str> = line.split(':').collect();
        let secondsplit : Vec<&str> = firstsplit[1].split('|').collect();
        let game_str = secondsplit[0].trim().replace("  ", " ");
        let game : Vec<&str> = game_str.split(' ').collect();
        let numbers_str = secondsplit[1].trim().replace("  ", " ");
        let numbers : Vec<&str> = numbers_str.split(' ').collect();
        //println!("'{:?}'", game);
        //println!("'{:?}'", numbers);

        let mut count = 0;
        for num in &numbers {
            for g in &game {
                if g == num {
                    count += 1;
                    break;
                }
            }
        }

        if count != 0 {
            //println!("'{}'", count);
            //println!("'{}'", i32::pow(2,count-1));
            total += i32::pow(2,count-1);
        }
    }
    total
}

fn part2(lines : &Vec<&str>) -> i32 {

    let mut counts : Vec<i32> = vec![1; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        if line == &"" {
            continue;
        }

        let firstsplit : Vec<&str> = line.split(':').collect();
        let secondsplit : Vec<&str> = firstsplit[1].split('|').collect();
        let game_str = secondsplit[0].trim().replace("  ", " ");
        let game : Vec<&str> = game_str.split(' ').collect();
        let numbers_str = secondsplit[1].trim().replace("  ", " ");
        let numbers : Vec<&str> = numbers_str.split(' ').collect();
        //println!("'{:?}'", game);
        //println!("'{:?}'", numbers);

        let mut count = 0;
        for num in &numbers {
            for g in &game {
                if g == num {
                    count += 1;
                    break;
                }
            }
        }

        let end = i + count + 1;

        for j in i+1..end {
            counts[j] += counts[i];
        }
    }
    counts.iter().sum()
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let lines : Vec<&str> = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".split("\n").collect();
        let answer : i32 = part1(&lines);
        assert_eq!(answer, 13);
    }

    #[test]
    fn part2_test() {
        let lines : Vec<&str> = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".split("\n").collect();
        let answer : i32 = part2(&lines);
        assert_eq!(answer, 30);
    }

}
