use std::collections::HashMap;

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
            Some(idx) => {
                line.chars().nth(idx+1).unwrap()
            }
            None => 'X'
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
                let arr1: Vec<usize> = line[colon+1..idx]
                    .trim()
                    .split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect();
                sub.push(arr1);
                let arr2: Vec<usize> = line[idx+1..]
                    .trim()
                    .split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect();
                sub.push(arr2);
            },
            None => {
                let arr: Vec<usize> = line[colon+1..]
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
    let possibilities = enumerate(&rules, 0);
  //  println!("possibilities: {:?}", possibilities);

    let mut sum = 0;
    for message in segments[1].lines() {
        if possibilities.contains(&message.to_string()) {
            sum += 1;
        }
    }
    println!("sum: {}", sum);
}

fn enumerate(rules: &HashMap<usize,Rule>, id: usize) -> Vec<String> {
    println!("enumerate({})", id);
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
                    new_sub_rule_strs.push(next);
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
