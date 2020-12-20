use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
struct Rule {
    sub: Vec<Vec<usize>>,
    primitive: char,
}

fn main() {
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let segments: Vec<&str> = input.trim().split("\n\n").collect();

    // Parse Rules
    for line in segments[0].lines() {
        // grab id
        let colon = line.find(":").unwrap();
        let id: usize = line[..colon].parse().unwrap();

        let primitive: char = match line.find("\"") {
            Some(idx) => line.chars().nth(idx + 1).unwrap(),
            None => 'X',
        };
        if primitive != 'X' {
            let rule = Rule {
                sub: Vec::new(),
                primitive,
            };
            rules.insert(id, rule);
            continue;
        }

        // sublist
        let mut sub = Vec::new();
        match line.find("|") {
            Some(idx) => {
                let arr1: Vec<usize> = line[colon + 1..idx]
                    .trim()
                    .split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect();
                sub.push(arr1);
                let arr2: Vec<usize> = line[idx + 1..]
                    .trim()
                    .split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect();
                sub.push(arr2);
            }
            None => {
                let arr: Vec<usize> = line[colon + 1..]
                    .trim()
                    .split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect();
                sub.push(arr);
            }
        };
        let rule = Rule {
            sub,
            primitive: 'X',
        };
        rules.insert(id, rule);
    }

    let mut memo: HashMap<usize, Vec<String>> = HashMap::new(); // memoize overlapping recursion
    let mut sum = 0;
    let possibilities: HashSet<String> = HashSet::from_iter(enumerate(&rules, 0, &mut memo).iter().cloned());
    for message in segments[1].lines() {
        if possibilities.contains(&message.to_string()) {
            sum += 1;
        }
    }
    println!("Part1: {}", sum);
    println!("len: {}", possibilities.len());

    // 42 42 31 is minimum string
    // rules 8 + 11 made up entirely of combinations of sets 42 and 31
    let mut set_42 = enumerate(&rules, 42, &mut memo);
    let mut set_31 = enumerate(&rules, 31, &mut memo);
    set_42.sort();
    set_31.sort();

    let mut sum_p2 = 0;
    for message in segments[1].lines() {
        let mut substr: String = message.to_string().clone();
        let mut prev_len: usize = 0;

        'outer: while prev_len != substr.len() {
            prev_len = substr.len();
            // strip one "42" prefix if available else break to next message
            match strip_prefix(&substr.to_string(), &set_42) {
                Some(sub) => substr = sub,
                None => break,
            };

            // strip n "42" prefixes and n "31" prefixes
            'inner: for n in 1..10 {
                let mut rule_11_substr = substr.clone();
                // 42
                for _i in 0..n {
                    match strip_prefix(&rule_11_substr.to_string(), &set_42) {
                        Some(sub) => rule_11_substr = sub,
                        None => continue 'inner,
                    }
                }
                // 31
                for _i in 0..n {
                    match strip_prefix(&rule_11_substr.to_string(), &set_31) {
                        Some(sub) => rule_11_substr = sub,
                        None => continue 'inner,
                    }
                }
                // success if we consumed the entire string
                if rule_11_substr.len() == 0 {
                    sum_p2 += 1;
                    break 'outer;
                }
            }
        }
    }
    println!("part_2: {}", sum_p2);
}

fn strip_prefix(message: &String, set: &Vec<String>) -> Option<String> {
    for prefix in set {
        if message.starts_with(prefix) {
            let trimmed = message.clone().strip_prefix(prefix).unwrap().to_string();
            return Some(trimmed);
        }
    }
    None
}

// TODO: should probably memoize this whole thing
// create all possible rules starting from a given rule
fn enumerate(rules: &HashMap<usize, Rule>, id: usize, memo: &mut HashMap<usize, Vec<String>>) -> Vec<String> {

    if let Some(cached) = memo.get(&id) {
        return cached.clone();
    }

    // zip together each set of rules with each possible sub-rule
    let mut strs: Vec<String> = Vec::new();
    let rule = &rules[&id];

    if rule.primitive != 'X' {
        strs.push(rule.primitive.to_string());
    }

    for rule_list in &rule.sub {
        let mut sub_rule_strs: Vec<String> = vec!["".to_string(); 1];
        for sub_rule in rule_list {
            let mut new_sub_rule_strs: Vec<String> = Vec::new();
            let sub_strs = enumerate(rules, *sub_rule, memo);
            for existing in &sub_rule_strs {
                for sub_str in &sub_strs {
                    let next = existing.to_owned() + &sub_str.to_owned();
                    new_sub_rule_strs.push(next);
                }
            }
            sub_rule_strs = new_sub_rule_strs.clone();
        }

        for blah in &sub_rule_strs {
            strs.push(blah.clone());
        }
    }
    // memoize and return
    memo.insert(id, strs.clone());
    return strs;
}
