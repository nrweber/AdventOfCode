use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines : Vec<&str> = contents.split('\n').collect();

    let p1 = part1(&lines);
    println!("p1: {}", p1);

    let p2 = part2(&lines);
    println!("p2: {}", p2);
}

fn part1(lines : &Vec<&str>) -> i32 {
    let mut hands : Vec<Hand> = Vec::new();

    for line in lines {
        if line == &"" {
            continue;
        }
        let split : Vec<&str> = line.split(" ").collect();
        let h = Hand::new(split[0].to_string(), split[1].parse::<i32>().unwrap());

        hands.push(h);
    }

    hands.sort_by_key(|h| h.key());

    let mut total : i32 = 0;
    for i in 0..(hands.len() as i32) {
        let key : usize = i as usize;
        //println!("{} {}", hands[key].cards, hands[key].bid);
        total += (i+1)*hands[key].bid;
    }

    total
}

fn part2(lines : &Vec<&str>) -> i32 {
    let mut hands : Vec<Hand> = Vec::new();

    for line in lines {
        if line == &"" {
            continue;
        }
        let split : Vec<&str> = line.split(" ").collect();
        let h = Hand::new(split[0].to_string(), split[1].parse::<i32>().unwrap());

        hands.push(h);
    }

    hands.sort_by_key(|h| h.key_with_jokers());

    let mut total : i32 = 0;
    for i in 0..(hands.len() as i32) {
        let key : usize = i as usize;
        //println!("{} {}", hands[key].cards, hands[key].bid);
        total += (i+1)*hands[key].bid;
    }

    total
}

struct Hand {
    cards : String,
    bid : i32
}

impl Hand {

    fn new(cards: String, bid: i32) -> Hand {
        return Hand { cards, bid };
    }

    fn key(&self) -> String {
        return self.hand_type().to_string() + &self.cards.clone().replace("A", "E").replace("K", "D").replace("Q", "C").replace("J", "B").replace("T", "A");
    }
    
    fn key_with_jokers(&self) -> String {
        return self.hand_type_with_joker().to_string() + &self.cards.clone().replace("A", "D").replace("K", "C").replace("Q", "B").replace("T", "A").replace("J", "1");
    }

    fn hand_score(cards : &str) -> i32 {
        let unique_cards : HashSet<char> = HashSet::from_iter(cards.chars());
            
        //Nothing
        if unique_cards.len() == 5 {
            return 0;
        }

        //One Pair
        if unique_cards.len() == 4 {
            return 1;
        }
       
        //Four of a Kind or Full House
        // if first card is 1 or 4 = four of a kind
        // if first card count is 2 or 3 = full house
        //
        if unique_cards.len() == 2 {
            let first = cards.chars().next().unwrap();
            let mut count = 0;
            for c in cards.chars(){
                if c == first {
                    count += 1;
                }
            }
            if count == 1 || count == 4 {
                return 5;
            }
            else {
                return 4;
            }
        }
       
        //Three of a kind or two pair
        // if first or second card count is 2 then two pair
        // if first and second count is 1 or 3 then three of a kind
        if unique_cards.len() == 3 {
            let mut char_iter = cards.chars();
            let first = char_iter.next().unwrap();
            let second = char_iter.next().unwrap();
            let mut first_count = 0;
            let mut second_count = 0;
            for c in cards.chars(){
                if c == first {
                    first_count += 1;
                }
                else if c == second {
                    second_count += 1;
                }
            }
            if first_count == 2 || second_count == 2 {
                return 2;
            }
            else {
                return 3;
            }
        }

        if unique_cards.len() == 1 {
            return 6;
        }

        0
    }
    
    fn hand_type(&self) -> i32 { 
        return Self::hand_score(&self.cards);
    }
    
    fn hand_type_with_joker(&self) -> i32 {
        let mut max_type = 0;
        for c in self.cards.chars() {
            let subed_cards = self.cards.clone().replace("J", &c.to_string());
            let hand_type = Self::hand_score(&subed_cards);
            if hand_type > max_type {
                max_type = hand_type;
            }
        }

        max_type
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hand_type_test_nothing() {
        let h = Hand { cards: "A2345".to_string(), bid: 5 };
        assert_eq!(h.hand_type(), 0);
    }
    
    #[test]
    fn hand_type_test_pair() {
        let target = 1;
        let h1 = Hand { cards: "A2A45".to_string(), bid: 5 };
        assert_eq!(h1.hand_type(), target);
        
        let h2 = Hand { cards: "A2445".to_string(), bid: 5 };
        assert_eq!(h2.hand_type(), target);
    }

    #[test]
    fn hand_type_test_two_pair() {
        let target = 2;
        let h1 = Hand { cards: "A2A44".to_string(), bid: 5 };
        assert_eq!(h1.hand_type(), target);
        
        let h2 = Hand { cards: "AA445".to_string(), bid: 5 };
        assert_eq!(h2.hand_type(), target);
    }

    #[test]
    fn hand_type_test_three_of_a_kind() {
        let target = 3;
        let h1 = Hand { cards: "A2A4A".to_string(), bid: 5 };
        assert_eq!(h1.hand_type(), target);
        
        let h2 = Hand { cards: "12AAA".to_string(), bid: 5 };
        assert_eq!(h2.hand_type(), target);
    }
   
    #[test]
    fn hand_type_test_full_house() {
        let target = 4;
        let h1 = Hand { cards: "A2A2A".to_string(), bid: 5 };
        assert_eq!(h1.hand_type(), target);
        
        let h2 = Hand { cards: "22555".to_string(), bid: 5 };
        assert_eq!(h2.hand_type(), target);
    }
   
    #[test]
    fn hand_type_test_four_of_a_kind() {
        let target = 5;
        let h1 = Hand { cards: "A2AAA".to_string(), bid: 5 };
        assert_eq!(h1.hand_type(), target);
        
        let h2 = Hand { cards: "25555".to_string(), bid: 5 };
        assert_eq!(h2.hand_type(), target);
    }
   
    #[test]
    fn hand_type_test_five_of_a_kind() {
        let target = 6;
        let h1 = Hand { cards: "AAAAA".to_string(), bid: 5 };
        assert_eq!(h1.hand_type(), target);
        
        let h2 = Hand { cards: "55555".to_string(), bid: 5 };
        assert_eq!(h2.hand_type(), target);
    }
   

    #[test]
    fn part1_test() {
        let lines : Vec<&str> = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".split("\n").collect();
        let answer : i32 = part1(&lines);
        assert_eq!(answer, 6440);
    }

    #[test]
    fn part2_test() {
        let lines : Vec<&str> = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".split("\n").collect();
        let answer : i32 = part2(&lines);
        assert_eq!(answer, 5905);
    }

}
