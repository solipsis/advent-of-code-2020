use std::collections::VecDeque;

fn main() {

    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .trim()
        .split("\n\n")
        .map(|x| x.to_owned())
        .collect();
    
    let mut p1: VecDeque<usize> = input[0].split("\n").skip(1).map(|x| x.parse().unwrap()).collect();
    let mut p2: VecDeque<usize> = input[1].split("\n").skip(1).map(|x| x.parse().unwrap()).collect();
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

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
    
    println!("score: {}", score);
}
