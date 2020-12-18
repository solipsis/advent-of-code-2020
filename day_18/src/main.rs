fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sum: isize = 0;
    let mut sum_2: isize = 0;

    for line in input.lines() {
        sum += calc(line.to_string());

        // for part 2, inject parens around each terms before and after each plus sign
        let terms: Vec<&str> = line.split(" ").collect();
        let mut replaced: Vec<String> = terms.clone().iter().map(|x| x.to_string()).collect();
        for (idx, term) in terms.iter().enumerate() {
            //println!("term: {}", term);
            if term.contains("+") {
                replaced[idx-1].insert_str(0, "(");
                replaced[idx+1].push_str(")");
             //   println!("After replace: {} {}", replaced[idx-1], replaced[idx+1])
            }
        }
        //let v2 = calc(replaced.join(" "));
        sum_2 += calc(replaced.join(" "));
    }

    println!("Part1: {}", sum);
    println!("Part2: {}", sum_2);
}

fn calc(eq: String) -> isize {
    //println!("original: {}", eq);
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

fn solve(eq: String) -> isize {
    //println!("sub: {}", eq);
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
    //println!("res: {}", acc);
    return acc;
}
