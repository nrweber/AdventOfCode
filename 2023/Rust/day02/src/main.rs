use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').collect();

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}

fn part1(lines : &Vec<&str>) -> i32 {
    let reduced_lines = lines.iter().map(|l| l.replace(":", "").replace(",", "").replace(";", ""));

    let mut total = 0;

    for x in reduced_lines { 
        if x == "" {
            continue;
        }

        let split : Vec<_> = x.split(' ').collect();

        let mut good_game = true;

        for i in (2..split.len()).step_by(2) {
            let num = split[i].parse::<i32>().unwrap();
            let color = split[i+1];
            if (color == "blue" && num > 14) || (color == "green" && num > 13) || (color == "red" && num > 12) {
                good_game = false;
            }
        }

        if good_game {
            total += split[1].parse::<i32>().unwrap();
        }

    }

    total
}

fn part2(lines : &Vec<&str>) -> i32 {
    let reduced_lines = lines.iter().map(|l| l.replace(":", "").replace(",", "").replace(";", ""));

    let mut total = 0;

    for x in reduced_lines { 
        if x == "" {
            continue;
        }

        let split : Vec<_> = x.split(' ').collect();

        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        for i in (2..split.len()).step_by(2) {
            let num = split[i].parse::<i32>().unwrap();
            let color = split[i+1];
            if color == "blue" && num > blue {
                blue = num;
            }
            else if color == "green" && num > green {
                green = num;
            }
            else if color == "red" && num > red {
                red = num;
            }
        }

        total += blue * green * red;

    }

    total
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let lines : Vec<&str> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".split("\n").collect();
        let answer : i32 = part1(&lines);
        assert_eq!(answer, 8);
    }

    #[test]
    fn part2_test() {
        let lines : Vec<&str> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".split("\n").collect();
        let answer : i32 = part2(&lines);
        assert_eq!(answer, 2286);
    }

}
