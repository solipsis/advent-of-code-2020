use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {

    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .trim()
        .split("\n\n")
        .map(|x| x.to_owned())
        .collect();
    
    let p1: VecDeque<u8> = input[0].split("\n").skip(1).map(|x| x.parse().unwrap()).collect();
    let p2: VecDeque<u8> = input[1].split("\n").skip(1).map(|x| x.parse().unwrap()).collect();

    part_1(p1.clone(), p2.clone());
    recursive_game(p1, p2, 1);
}

fn recursive_game(mut p1: VecDeque<u8>, mut p2: VecDeque<u8>,  game: usize) -> bool {
    
    // track all seen states in this particular game
    let mut states_p1: HashSet<VecDeque<u8>> = HashSet::new();
    let mut states_p2: HashSet<VecDeque<u8>> = HashSet::new();
    let mut sub_game = game;

    // optimization pointed out by user "curious_sapi3n"
    // If player 1 has the largest card and it is larger than the size
    // of the decks, preventing a further subgame, then player 1 is guaranteed to
    // to win that subgame because player 2 can never claim that card
    if game > 1 {
        let max_p1 = p1.iter().max().unwrap();
        let max_p2 = p2.iter().max().unwrap();
        if max_p1 > max_p2 && *max_p1 as usize > p1.len() {
            return true;
        }
    }

    while p1.len() > 0 && p2.len() > 0 {

        if states_p1.contains(&p1) || states_p2.contains(&p2) {
            return true; // player one wins due to anti-loop rule
        }
        states_p1.insert(p1.clone());
        states_p2.insert(p2.clone());

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        // recursive combat
        if p1.len() >= c1 as usize && p2.len() >= c2 as usize {
            // create new decks that are copies
            let mut nd1: VecDeque<u8> = p1.clone();
            nd1.truncate(c1 as usize);
            let mut nd2: VecDeque<u8> = p2.clone();
            nd2.truncate(c2 as usize);
            // p1 wins if true
            sub_game += 1;
            if recursive_game(nd1, nd2, sub_game) {
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
        println!("Part 2: {}", score(&p1, &p2));
    }

    return p1_wins;
}

fn score(p1: &VecDeque<u8>, p2: &VecDeque<u8>) -> usize {
    let mut score: usize = 0;
    let mut multiplier: usize = 1;
    if !p1.is_empty() {
        for v in p1.iter().rev() {
            score += *v as usize * multiplier;
            multiplier += 1;
        }
    } else {
        for v in p2.iter().rev() {
            score += *v as usize * multiplier;
            multiplier += 1;
        }
    }
    score
}

fn part_1(mut p1: VecDeque<u8>, mut p2: VecDeque<u8>) {
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

    println!("Part 1: {}", score(&p1, &p2));
}
