use std::collections::HashMap;

fn main() {
    let nums: Vec<usize> = std::fs::read_to_string("input.txt").unwrap()
        .trim()
        .split(",")
        .map( |x| x.parse().unwrap())
        .collect();

    let mut state: HashMap<usize,usize> = HashMap::new();
    println!("Nums: {:?}", nums);

    let mut turn: usize = 1;
    for i in 0..(nums.len()-1) {
        state.insert(nums[i], turn);
        turn += 1;
    }
    turn += 1;

    let mut prev: usize = nums[nums.len()-1 as usize];
    let mut say: usize;
    loop {
        if !state.contains_key(&prev) {
            say = 0;
        } else {
        //    println!("{} was last encountered turn {}", say, state[&prev]);
            let prev_use = state[&prev];
            say = (turn-1) - prev_use;
        }
        if turn == 2020 {
            println!("Turn: {}, say: {}", turn, say);
            return
        }
        state.insert(prev, turn-1);
        turn += 1;
        prev = say;
    }

}

