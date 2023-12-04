use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').filter(|l| l.len() != 0).collect();

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}

fn part1(lines : &Vec<&str>) -> u32 {
    let height = lines.len();
    let width = lines[0].len();

    let mut total = 0;

    let mut good_number = false;
    let mut current_number : u32 = 0;

    for y in 0..height {
        let prev_chars : Option<Vec<char>> = match y {
            0 => None,
            _ => Some(lines[y-1].chars().collect())
        };
        let next_chars : Option<Vec<char>> = match y {
            i if i < height-1 => Some(lines[y+1].chars().collect()),
            _ => None
        };


        let current_chars : Vec<char> = lines[y].chars().collect();
        //for (i, c) in lines[y].chars().enumerate() {
        for i in 0..width {
            if i == 0 {
                if good_number {
                    //println!("s {}", current_number);
                    total += current_number;
                }
                good_number = false;
                current_number = 0;
            }


            let c = current_chars[i];

            if c.is_digit(10) == false {
                if good_number {
                    //println!("{}", current_number);
                    total += current_number;
                }
                good_number = false;
                current_number = 0;
            }
            else {
                current_number = (current_number * 10) + c.to_digit(10).unwrap();

                if false == good_number {
                    match prev_chars {
                        None => {},
                        Some(ref pl) => {
                            if i != 0 && pl[i-1].is_digit(10) == false && pl[i-1] != '.' {
                                good_number = true;
                            }
                            if i != 0 && pl[i].is_digit(10) == false && pl[i] != '.' {
                                good_number = true;
                            }
                            if i < width-2 && pl[i+1].is_digit(10) == false && pl[i+1] != '.' {
                                good_number = true;
                            }
                        }
                    };


                    if i != 0 && current_chars[i-1].is_digit(10) == false && current_chars[i-1] != '.' {
                        good_number = true;
                    }
                    if i != width-1 && current_chars[i+1].is_digit(10) == false && current_chars[i+1] != '.' {
                        good_number = true;
                    }

                    match next_chars {
                        None => {},
                        Some(ref nl) => {
                            if i != 0 && nl[i-1].is_digit(10) == false && nl[i-1] != '.' {
                                good_number = true;
                            }
                            if i != 0 && nl[i].is_digit(10) == false && nl[i] != '.' {
                                good_number = true;
                            }
                            if i < width-1 && nl[i+1].is_digit(10) == false && nl[i+1] != '.' {
                                good_number = true;
                            }
                        }
                    };
                }
            }
        }
    }


    total
}

fn part2(lines : &Vec<&str>) -> u32 {
    0
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let lines : Vec<&str> = "467..114.
...*.....
..35..633
......#..
617*.....
.....+.58
..592....
......755
...$.*...
.664.598.".split("\n").collect();
        let answer : u32 = part1(&lines);
        assert_eq!(answer, 4361);
    }

    #[test]
    fn part2_test() {
        let lines : Vec<&str> = "467..114.
...*.....
..35..633
......#..
617*.....
.....+.58
..592....
......755
...$.*...
.664.598.".split("\n").collect();
        let answer : u32 = part2(&lines);
        assert_eq!(answer, 467835);
    }

}
