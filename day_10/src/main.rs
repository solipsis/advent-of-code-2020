fn main() {
    let mut input: Vec<i32> = std::fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    input.sort();
    println!("{:?}", input);

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

    println!("1: {}, 3: {}", one_count, three_count);
    println!("Product: {}", one_count*three_count);
    part2(&input);
}

fn part2(input: &Vec<i32>) {
    let mut idx = input.len()-1;
    let mut acc: u64 = 1;
    while idx > 0 {
        let mut sub_idx = idx-1;
        let mut ways = 0;
        while  input[sub_idx]+3 >= input[idx] {
            ways += 1;
            if sub_idx == 0 {
                break;
            }
            sub_idx -= 1;
        }
        println!("val: {}, ways: {}", input[idx], ways);
        match ways {
            2 => acc *= 2,
            3 => acc *= 4,
            _ => (),
        }
        idx -= ways;
    }
    println!("Acc: {}", acc);

}

