use std::collections::HashSet;

const PREAMBLE_SIZE: usize = 25;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let vals: Vec<usize> = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut result: usize = 0;

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
            result = vals[idx];
            break;
        }
    }

    part2(&vals, result);
}

fn populate_targets(idx: usize, input: &Vec<usize>, targets: &mut HashSet<usize>) {
    let target = &input[idx];
    let input = &input[idx-PREAMBLE_SIZE..idx+1];

    for i in 0..PREAMBLE_SIZE {
        targets.insert(target.wrapping_sub(input[i]));
    }
}

fn part2(input: &Vec<usize>, target: usize) {
    let mut tail_idx: usize = 0;
    let mut head_idx: usize = 0;
    let mut sum: usize = 0;

    loop {
        if sum < target {
            sum += input[head_idx];
            head_idx += 1;
        } else if sum > target {
            sum -= input[tail_idx];
            tail_idx += 1;
        }

        if sum == target {
            let range = &input[tail_idx..head_idx-1];
            let min = range.iter().min().unwrap();
            let max = range.iter().max().unwrap();

            println!("Part2: {}", min + max);
            return
        }
    }
}
