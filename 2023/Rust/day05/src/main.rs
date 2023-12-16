use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').collect();

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}

fn part1(lines : &Vec<&str>) -> i64 {
    let seeds_one : Vec<&str> = lines[0].split(' ').collect();
    let mut seeds : Vec<i64> = seeds_one[1..].iter().map(|s| s.parse::<i64>().unwrap()).collect();

    //println!("{:?}", seeds);

    let mut nextList : Vec<i64> = Vec::new();


    let mut i : usize = 3;
    while i < lines.len() {
        let line = lines[i];
        if line == "" {
            //end of section
            i += 1;

            while seeds.len() > 0 {
                nextList.push(seeds[0]);
                seeds.remove(0);
            }
            seeds = nextList;
            nextList = Vec::new();

            //println!("{:?}", seeds);
            //println!("end of section");
        }
        else
        {
            let line_split : Vec<&str> = line.split(' ').collect();
            let range_nums : Vec<i64> = line_split.iter().map(|s| s.parse::<i64>().unwrap()).collect();
            //println!("{:?}", range_nums);

            let d_start = range_nums[0];
            let i_start = range_nums[1];
            let num = range_nums[2];

            let mut j : usize = 0;
            while j < seeds.len() {
                if seeds[j] >= i_start && seeds[j] < i_start+num {
                    //println!("{} -> {}", seeds[j], d_start+(seeds[j]-i_start));
                    nextList.push(d_start+(seeds[j]-i_start));
                    seeds.remove(j);
                }
                else {
                    j += 1;
                }
            }

            //println!("end line");
        }
        i += 1;
    }

    *seeds.iter().min().unwrap()
}

fn part2(lines : &Vec<&str>) -> i64 {
    let mut loops : u64 = 0;
    let seeds_one : Vec<&str> = lines[0].split(' ').collect();
    let input_seed_ranges : Vec<i64> = seeds_one[1..].iter().map(|s| s.parse::<i64>().unwrap()).collect();


    for num in 0..i64::MAX  {
        if num != 0 && num % 100000 == 0 {
            println!("{}", num);
        }
        let mut seed = num;

        let mut section_done = false;
        let mut i : usize = lines.len()-1;
        while i >= 3 {
            let line = lines[i];
            if line.contains(":") || line == "" {
                //end of section
                i -= 1;
                section_done = false;
                //println!("{:?}", seeds);
                //println!("end of section");
            }
            else if section_done == false {
                let line_split : Vec<&str> = line.split(' ').collect();
                let range_nums : Vec<i64> = line_split.iter().map(|s| {
                                                                 //println!("--{}", s);
                                                                  s.parse::<i64>().unwrap()
                }).collect();
                //println!("{:?}", range_nums);

                let d_start = range_nums[0];
                let i_start = range_nums[1];
                let num = range_nums[2];

                if seed >= d_start && seed < d_start+num {
                    //println!("{} -> {}", seeds[j], d_start+(seeds[j]-i_start));
                    seed = i_start+(seed-d_start);
                    section_done = true;
                }

                //println!("end line");
            }
            i -= 1;
        }

        let mut w : usize = 0;
        while w < input_seed_ranges.len() {
            if seed >= input_seed_ranges[w] && seed < (input_seed_ranges[w]+input_seed_ranges[w+1]) {
                return num;
            }
            w += 2;
        }
    }

    0
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_test() {
        let lines : Vec<&str> = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".split("\n").collect();
        let answer : i64 = part1(&lines);
        assert_eq!(answer, 35);
    }

    #[test]
    fn part2_test() {
        let lines : Vec<&str> = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".split("\n").collect();
        let answer : i64 = part2(&lines);
        assert_eq!(answer, 46);
    }
}
