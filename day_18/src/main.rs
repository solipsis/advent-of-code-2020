fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sum: isize = 0;

    for line in input.lines() {
        let v = calc(line.to_string());
        sum += v;
        println!("line_done: {}", v);
    }

    println!("Part1: {}", sum);
}

fn calc(eq: String) -> isize {
    let mut eq: String = eq.clone();
    let mut val: isize = -1;
    // repeatedly find rightmost lparen
    // calc until r paren
    // replace with vals, remove parens, repeat.
    loop {
        let lparen = eq.rfind('(');
        match lparen {
            Some(idx) => {
                let sub = &eq[idx..];
                let rparen = sub.find(')').unwrap(); // assume exists?
                val = solve(sub[1..rparen].to_string());
                eq.replace_range(idx..(idx+rparen+1), &val.to_string());
            }
            None => return solve(eq),
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
    println!("res: {}", acc);
    return acc;
}
