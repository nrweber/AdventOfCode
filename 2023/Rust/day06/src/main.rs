use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').collect();

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}

fn distance_for_time(hold_time : u64, race_length : u64) -> u64 {
    (race_length-hold_time)*hold_time
}

fn part1(lines : &Vec<&str>) -> u64 {
    let line1 : Vec<&str> = lines[0].split_whitespace().collect();
    let race_lengths : Vec<u64> = line1[1..].iter().map(|x| x.parse::<u64>().unwrap()).collect();
    //println!("{:?}", race_lengths);
    let line2 : Vec<&str> = lines[1].split_whitespace().collect();
    let distances : Vec<u64> = line2[1..].iter().map(|x| x.parse::<u64>().unwrap()).collect();
    //println!("{:?}", distances);

    let mut result = 1;
    for i in 0..race_lengths.len() {
        let race_length = race_lengths[i];
        let distance = distances[i];

        let mut count = 0;
        for press_length in 1..race_length {
            if distance_for_time(press_length, race_length) > distance {
                count += 1;
            }
        }
        //println!("{}", count);
        result *= count;
    }


    result
}

fn part2(lines : &Vec<&str>) -> u64 {
    let line1 : String = lines[0].chars().filter(|c| !c.is_whitespace()).collect();
    let line1_split : Vec<&str> = line1.split(':').collect();
    let race_length = line1_split[1].parse::<u64>().unwrap();

    let line2 : String = lines[1].chars().filter(|c| !c.is_whitespace()).collect();
    let line2_split : Vec<&str> = line2.split(':').collect();
    let distance = line2_split[1].parse::<u64>().unwrap();


    let mut count = 0;
    for press_length in 1..race_length {
        if distance_for_time(press_length, race_length) > distance {
            count += 1;
        }
    }
    count
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let lines : Vec<&str> = "Time:      7  15   30
Distance:  9  40  200".split("\n").collect();
        let answer : u64 = part1(&lines);
        assert_eq!(answer, 288);
    }

    #[test]
    fn part2_test() {
        let lines : Vec<&str> = "Time:      7  15   30
Distance:  9  40  200".split("\n").collect();
        let answer : u64 = part2(&lines);
        assert_eq!(answer, 71503);
    }

}
