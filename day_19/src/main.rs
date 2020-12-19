use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Rule {
    sub: Vec<Vec<usize>>,
    primitive: char,
}

fn main() {
    // keep looping
    // rule []string primitive
    //
    // if a rule is composed of known strings, substitute
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let segments: Vec<&str> = input.trim().split("\n\n").collect();

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

        // 285 too high
        // 272

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
    // println!("{:?}", rules);
    //  println!("---------------------------------");
    /*
      let possibilities = enumerate(&rules, 0);
      //println!("possibilities: {:?}", possibilities);

      let mut sum = 0;
      for message in segments[1].lines() {
          if possibilities.contains(&message.to_string()) {
              sum += 1;
          }
      }
      println!("Part1: {}", sum);
    */

    // need to substring at least twice 42 42 31 is minimum string
    // 31 can also appear n times
    let mut results: HashSet<String> = HashSet::new();

    let set_8 = enumerate(&rules, 8);
    let set_42 = enumerate(&rules, 42);
    let set_31 = enumerate(&rules, 31);
    //println!("set_42: {:?}", &set_42);
    //let set_11 = enumerate(&rules, 11);
    //
    let mut sum_p2 = 0;
    for message in segments[1].lines() {
        let mut substr: String = message.to_string().clone();
        let mut prev_len: usize = 0;
        let mut sub_cnt: usize = 0;

        'outer: while prev_len != substr.len() {
            prev_len = substr.len();
            strip_prefix(&substr.to_string(), &set_42);
            for prefix in &set_42 {
                if substr.starts_with(prefix) {
                    sub_cnt += 1;
                    substr = substr.strip_prefix(prefix).unwrap().to_string();
                }
                if sub_cnt >= 2 {
                    let mut tail_cpy = substr.clone();
                    let mut prev_tail_len = 0;
                    let mut tail_cnt = 0;

                    // strip n set 42 then n rule 31

                    // repeatedly strip prefixes from the set of possible outcomes from rule 31
                    // strip no more than sub_cnt - 1 prefixes
                    while prev_tail_len != tail_cpy.len() && tail_cnt < sub_cnt - 1 {
                        prev_tail_len = tail_cpy.len();
                        for tail_prefix in &set_31 {
                            if tail_cpy.starts_with(tail_prefix) {
                                tail_cpy = tail_cpy.strip_prefix(tail_prefix).unwrap().to_string();
                                tail_cnt += 1;
                                if tail_cpy.len() == 0 {
                                    sum_p2 += 1;
                                    println!("matched: {}", message);
                                    results.insert(message.to_string());
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("part_2: {}", sum_p2);
    println!("results: {}", results.len());

    // gather set of "42" rules

    // gather set of "8" rules
    // for each message {
    //     if starts with something from 8 set {
    //         substring and loop
    //     }
    // }

    // gather set of "11" rules
    // gather set of "42" rules?
}

fn strip_prefix(message: &String, set: &Vec<String>) -> Option<String> {
    for prefix in set {
        if message.starts_with(prefix) {
            let trimmed = message.clone().strip_prefix(prefix).unwrap().to_string();
            return Some(trimmed)
        }
    }
    None
}

fn enumerate(rules: &HashMap<usize, Rule>, id: usize) -> Vec<String> {
    //println!("enumerate({})", id);
    let rule = &rules[&id];
    let mut ret: Vec<String> = Vec::new();
    if rule.primitive != 'X' {
        ret.push(rule.primitive.to_string());
        //     println!("primitive: {} {}", id, &rule.primitive);
        return ret;
    }

    // for each of 2 sides of OR
    //     for each rule in that side
    //         for each item currently in sub_possibilities
    //

    //let mut strs: Vec<String> = vec!["".to_string(); 1];
    let mut strs: Vec<String> = Vec::new();

    for rule_list in &rule.sub {
        //let sub_possibilities: Vec<String> = Vec::new();
        let mut sub_rule_strs: Vec<String> = vec!["".to_string(); 1];
        for sub_rule in rule_list {
            let mut new_sub_rule_strs: Vec<String> = Vec::new();
            //        println!("**********************************");
            //        println!("sub_rule: {:?}", sub_rule);
            let sub_strs = enumerate(rules, *sub_rule);
            //println!("sub_strs: {:?}", &sub_strs);
            //println!("existing sub_rule_strs: {:?}", &sub_rule_strs);
            for existing in &sub_rule_strs {
                //            println!("##########################");
                //            println!("existing {:?}", existing);
                for sub_str in &sub_strs {
                    //                println!("----------------------------");
                    //                println!("sub_str: {}", sub_str);
                    let next = existing.to_owned() + &sub_str.to_owned();
                    //                println!("next: {}", &next);
                    if next.len() < 100 {
                        new_sub_rule_strs.push(next);
                    }
                }
            }
            sub_rule_strs = new_sub_rule_strs.clone();
        }

        //    println!("sub_rule_strs: {:?}", &sub_rule_strs);
        for blah in &sub_rule_strs {
            strs.push(blah.clone());
        }
    }

    return strs;
}
