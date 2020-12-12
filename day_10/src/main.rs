fn main() {
    let mut input: Vec<u64> = std::fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    input.sort();

    let mut prev = 0;
    let mut one_count = 0;
    let mut three_count = 0;
    for i in &input {
        if *i == prev+1 {
            one_count += 1;
        }
        if *i == prev+3 {
            three_count += 1;
        }
        prev = *i;
    }
    three_count += 1;

    println!("Part1: {}", one_count*three_count);
    part2(&input);
}

fn part2(input: &Vec<u64>) {
    let memo_size = (*input.iter().max().unwrap() + 3) as usize;
    let mut idx = 0;
    let mut memo: Vec<u64> = vec![0; memo_size];

    // initialize first dp value
    memo[2] = 1;

    while idx < input.len() {
        let v: usize = input[idx] as usize; // val 1 should be in memo[3]
        memo[v+2] = memo[v+1] + memo[v] + memo[v-1];
        idx += 1;
    }

    println!("Part2: {}", memo.iter().max().unwrap());
}

