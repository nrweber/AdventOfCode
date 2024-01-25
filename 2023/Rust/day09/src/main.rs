use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').collect();

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}


fn steps_between(nums : &Vec<i32>) -> Vec<i32> {
    let mut steps : Vec<i32> = Vec::new();

    for i in 1..nums.len() {
        steps.push(nums[i] - nums[i-1]);
    }

    steps
}

fn next_number(nums : &Vec<i32>) -> i32 {
   
    let steps = steps_between(nums);

    //for i in 0..steps.len() {
    //    print!("{},", steps[i]);
    //}
    //println!("");

    let mut all_zero = true;
    for n in &steps {
        if n != &0 {
            all_zero = false;
            break;
        }
    }

    if all_zero {
        return nums[nums.len()-1];
    }

    let next = nums[nums.len()-1] + next_number(&steps);
    //println!("next: {}", next);
    return next;
}

fn prev_number(nums : &Vec<i32>) -> i32 {
   
    let steps = steps_between(nums);

    //for i in 0..steps.len() {
    //    print!("{},", steps[i]);
    //}
    //println!("");

    let mut all_zero = true;
    for n in &steps {
        if n != &0 {
            all_zero = false;
            break;
        }
    }

    if all_zero {
        return nums[nums.len()-1];
    }

    let next = nums[0] - prev_number(&steps);
    //println!("next: {}", next);
    return next;
}

fn part1(lines : &Vec<&str>) -> i32 {

    let mut sum : i32 = 0;

    for line in lines {
        //println!("--------");
        if line == &"" {
            continue;
        }

        let nums : Vec<i32> = line.split(' ').map(|x| -> i32 { x.parse().unwrap() }).collect();

        let next_number = next_number(&nums);
        //println!("{}", next_number);

        sum += next_number;
    }
    sum
}

fn part2(lines : &Vec<&str>) -> i32 {
    let mut sum : i32 = 0;

    for line in lines {
        //println!("--------");
        if line == &"" {
            continue;
        }

        let nums : Vec<i32> = line.split(' ').map(|x| -> i32 { x.parse().unwrap() }).collect();

        let prev_number = prev_number(&nums);
        //println!("{}", prev_number);

        sum += prev_number;
    }
    sum
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let lines : Vec<&str> = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".split("\n").collect();
        let answer : i32 = part1(&lines);
        assert_eq!(answer, 114);
    }

    #[test]
    fn part2_test() {
        let lines : Vec<&str> = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".split("\n").collect();
        let answer : i32 = part2(&lines);
        assert_eq!(answer, 2);
    }

}
