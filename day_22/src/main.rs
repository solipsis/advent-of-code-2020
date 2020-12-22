use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {

    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .trim()
        .split("\n\n")
        .map(|x| x.to_owned())
        .collect();
    
    let p1: VecDeque<usize> = input[0].split("\n").skip(1).map(|x| x.parse().unwrap()).collect();
    let p2: VecDeque<usize> = input[1].split("\n").skip(1).map(|x| x.parse().unwrap()).collect();
    let mut memo: HashMap<String, bool> = HashMap::new();

    part_1(p1.clone(), p2.clone());

    recursive_game(p1, p2, &mut memo, 1);

    /*
    let mut score = 0;
    let mut multiplier = 1;
    while p1.len() > 0 {
        score += p1.pop_back().unwrap() * multiplier;
        multiplier += 1;
    }
    while p2.len() > 0 {
        score += p2.pop_back().unwrap() * multiplier;
        multiplier += 1;
    }
    
    println!("Part 1: {}", score);
    */
    /*
    while p1.len() > 0 && p2.len() > 0 {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        // recursive combat
        if p1.len() >= c1 && p2.len() >= c2 {
            // create new decks that are copies
            let mut nd1: VecDeque<usize> = p1.clone();
            while nd1.len() != c1 {
                nd1.pop_back();
            }
            let mut nd2: VecDeque<usize> = p2.clone();
            while nd2.len() != c2 {
                nd2.pop_back();
            }
            recursive_game(p1.clone(), p2.clone(), &mut memo);
        } else { // normal combat
            if c1 > c2 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }

    }
    */
    
}

fn recursive_game(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>, memo: &mut HashMap<String,bool>,  mut game: usize) -> bool {
//    println!("start game: {}", game);
    
    // track all seen states in this particular game
    let mut states: HashSet<String> = HashSet::new();
    let mut sub_game = game;

    // hash the state to see if we have seen this substate before
    /*
    let hash = blake3::hash(to_id(&p1, &p2).as_bytes()).to_hex().to_string();
    if let Some(prev) = memo.get(&hash) {
       // println!("hooray-cache?: {}", memo.len());
        println!("cached game {} done: {}, cache-size: {}", game, *prev, memo.len());
        return *prev;
    }
    */

    //println!("{}", hash);

    while p1.len() > 0 && p2.len() > 0 {
        let hash = blake3::hash(to_id(&p1, &p2).as_bytes()).to_hex().to_string();
        if states.contains(&hash) {
            return true; // player one wins due to anti-loop rule
        }
        states.insert(hash.clone());

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        // recursive combat
        if p1.len() >= c1 && p2.len() >= c2 {
            // create new decks that are copies
            let mut nd1: VecDeque<usize> = p1.clone();
            while nd1.len() != c1 {
                nd1.pop_back();
            }
            let mut nd2: VecDeque<usize> = p2.clone();
            while nd2.len() != c2 {
                nd2.pop_back();
            }
            // p1 wins if true
            sub_game += 1;
            if recursive_game(nd1.clone(), nd2.clone(), memo, sub_game) {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        } else { // normal combat
            if c1 > c2 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }
    }
    let p1_wins = if p1.len() > p2.len() {true} else {false};

    if game == 1 {
        let mut score = 0;
        let mut multiplier = 1;
        while p1.len() > 0 {
            score += p1.pop_back().unwrap() * multiplier;
            multiplier += 1;
        }
        while p2.len() > 0 {
            score += p2.pop_back().unwrap() * multiplier;
            multiplier += 1;
        }
        println!("Part 2: {}", score);
        panic!("done");
    }
    

    // memoize
    //memo.insert(hash, p1_wins);
    return p1_wins;
}

fn to_id(p1: &VecDeque<usize>, p2: &VecDeque<usize>) -> String {
    let mut elements: Vec<String> = vec![String::from("p1")];
    for el in p1 {
        elements.push(el.to_string());
    }
    elements.push(String::from("p2"));
    for el in p2 {
        elements.push(el.to_string());
    }
    return elements.join(",");
}

fn part_1(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) {
    while p1.len() > 0 && p2.len() > 0 {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    let mut score = 0;
    let mut multiplier = 1;
    while p1.len() > 0 {
        score += p1.pop_back().unwrap() * multiplier;
        multiplier += 1;
    }
    while p2.len() > 0 {
        score += p2.pop_back().unwrap() * multiplier;
        multiplier += 1;
    }
    
    println!("Part 1: {}", score);
}


