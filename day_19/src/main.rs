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
    let mut set_42 = enumerate(&rules, 42);
    let mut set_31 = enumerate(&rules, 31);
    //let set_11 = enumerate(&rules, 11);

    set_42.sort();
    set_31.sort();

    println!("set_42: {:?}", set_42); 
    println!("set_31: {:?}", set_31); 
    let mut sum_p2 = 0;

    for message in segments[1].lines() {
        let mut substr: String = message.to_string().clone();
        let mut prev_len: usize = 0;

        'outer: while prev_len != substr.len() {
            println!("outer----------------------------------");
            println!("substr: {}", &substr);
            prev_len = substr.len();
            // strip one "42" prefix if available else break to next message
            match strip_prefix(&substr.to_string(), &set_42) {
                Some(sub) => substr = sub,
                None => break,
            };
            println!("stripped 8->42: {}", &substr);

            // strip n "42" prefixes and n "31" prefixes
            'inner: for n in 1..10 {
                println!("N: {}", n);
                let mut rule_11_substr = substr.clone();
                // 42
                for _i in 0..n {
                    match strip_prefix(&rule_11_substr.to_string(), &set_42) {
                        Some(sub) => rule_11_substr = sub,
                        None => continue 'inner,
                    }
                }
                println!("striped 11->42: {}", &rule_11_substr);
                // 31
                for _i in 0..n {
                    match strip_prefix(&rule_11_substr.to_string(), &set_31) {
                        Some(sub) => rule_11_substr = sub,
                        None => continue 'inner,
                    }
                }
                println!("stripped 11->31: {}", &rule_11_substr);

                if rule_11_substr.len() == 0 {
                    sum_p2 += 1;
                    println!("matched: {}", message);
                    results.insert(message.to_string());
                    break 'outer;
                }
            }
        }
    }
    println!("part_2: {}", sum_p2);
}

fn strip_prefix(message: &String, set: &Vec<String>) -> Option<String> {
    //println!("strip_prefix: {}, set: {:?}", message, set);
    for prefix in set {
        if message.starts_with(prefix) {
            let trimmed = message.clone().strip_prefix(prefix).unwrap().to_string();
            return Some(trimmed);
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
