fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sum: isize = 0;
    let mut sum_2: isize = 0;

    for line in input.lines() {
        sum += calc(line.to_string());

        let terms: Vec<&str> = line.split(" ").collect();
        let replaced: Vec<String> = terms.clone().iter().map(|x| x.to_string()).collect();
        sum_2 += calc_part_2(replaced.join(" "));
    }

    println!("Part1: {}", sum);
    println!("Part2: {}", sum_2);
}

fn calc(eq: String) -> isize {
    println!("original: {}", eq);
    let mut eq: String = eq.clone();
    // repeatedly find rightmost lparen
    // calc until r paren
    // replace with vals, remove parens, repeat.
    loop {
        let lparen = eq.rfind('(');
        match lparen {
            Some(idx) => {
                let sub = &eq[idx..];
                let rparen = sub.find(')').unwrap(); // assume exists?
                let val = solve(sub[1..rparen].to_string());
                eq.replace_range(idx..(idx+rparen+1), &val.to_string());
            }
            None => return solve(eq),
        }
    }


}

fn calc_part_2(eq: String) -> isize {
    println!("original: {}", eq);
    let mut eq: String = eq.clone();
    // repeatedly find rightmost lparen
    // calc until r paren
    // replace with vals, remove parens, repeat.
    loop {
        let lparen = eq.rfind('(');
        match lparen {
            Some(idx) => {
                let sub = &eq[idx..];
                let rparen = sub.find(')').unwrap(); // assume exists?
                let val = solve_part_2(sub[1..rparen].to_string());
                eq.replace_range(idx..(idx+rparen+1), &val.to_string());
            }
            None => return solve_part_2(eq),
        }
    }
}


fn solve(eq: String) -> isize {
    println!("sub: {}", eq);
    let terms: Vec<&str> = eq.split(" ").collect();
    let mut acc: isize = terms[0].parse().unwrap();

    let mut operator: char = '_';
    for term in terms[1..].iter() {
        match term {
            &"*" => operator = '*',
            &"+" => operator = '+',
            _ => {
                let v: isize = term.parse().unwrap();
                match operator {
                    '*' => acc *= v,
                    '+' => acc += v,
                    _ => panic!("invalid operator"),
                }
            }
        }
    }
    return acc;
}

fn solve_part_2(eq: String) -> isize {
    println!("sub: {}", eq);
    let mut terms: Vec<String> = eq.split(" ").map(|x| x.to_string()).collect();

    // resolve addition first
    let mut substituted: Vec<String> = terms.clone();
    loop {
        if !terms.contains(&"+".to_string()) {
            break;
        }
        for (idx, term) in terms.iter().enumerate() {
            if term.contains("+") {
                let v: isize = terms[idx-1].parse::<isize>().unwrap() + terms[idx+1].parse::<isize>().unwrap();
                substituted[idx] = v.to_string();
                substituted.remove(idx-1);
                substituted.remove(idx); // value shifted 1 left from previous op
                break;
            }
        }
        terms = substituted.clone();
    }

    // only multiplication is left
    let mut acc: isize = terms[0].parse().unwrap();
    for term in terms[1..].iter() {
        if term.contains("*") {
            continue;
        }
        let v: isize = term.parse().unwrap();
        acc *= v;
    }
    return acc;
}
