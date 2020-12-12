use std::collections::HashSet;

const PREAMBLE_SIZE: usize = 5;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let vals: Vec<usize> = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{:?}", vals);


   // populate_targets(5, &vals, &mut targets);

    for idx in PREAMBLE_SIZE..vals.len() {
        let mut valid = false;
        let mut targets = HashSet::new();
        populate_targets(idx, &vals, &mut targets);

        for search_idx in idx-PREAMBLE_SIZE..idx {
            if targets.contains(&vals[search_idx]) && vals[search_idx]*2 != vals[idx] {
                valid = true;
                break
            }
        }
        if !valid {
            println!("Done: {}", vals[idx]);
            return
        }
    }

}

fn populate_targets(idx: usize, input: &Vec<usize>, targets: &mut HashSet<usize>) {
    let target = &input[idx];
    let input = &input[idx-PREAMBLE_SIZE..idx+1];
    println!("{:?}", input);
    println!("input IDX: {}", target);
    
    for i in 0..PREAMBLE_SIZE {
        targets.insert(target.wrapping_sub(input[i]));
    }

    println!("{:?}", targets);
}
