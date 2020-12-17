use std::collections::HashMap;
use std::collections::HashSet;

// what do I want
//
// tuple of tuples what could go wrong?
// map of String -> ((usize, usize),(usize, usize))
//

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

fn main() {
    let mut global_valid: HashSet<usize> = HashSet::new();

    // name to disjoint set of ranges
    let mut constraints: HashMap<String, (Range, Range)> = HashMap::new();

    let input: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();

    let constraint_input: Vec<String> = input[0].lines().map(|x| x.to_string()).collect();

    for line in constraint_input {
        let separator = line.find(":").unwrap();
        let key = &line[..separator].trim();
        //println!("key: {}", key);

        let ranges: Vec<&str> = line[separator + 1..].trim().split(" ").collect();

        let r1: Vec<&str> = ranges[0].split("-").collect();
        let r1: Range = Range {
            start: r1[0].parse().unwrap(),
            end: r1[1].parse().unwrap(),
        };
        // println!("r1: {:?}", r1);

        let r2: Vec<&str> = ranges[2].split("-").collect();
        let r2: Range = Range {
            start: r2[0].parse().unwrap(),
            end: r2[1].parse().unwrap(),
        };
        for i in r1.start..(r1.end + 1) {
            global_valid.insert(i as usize);
        }
        for i in r2.start..(r2.end + 1) {
            global_valid.insert(i as usize);
        }

        constraints.insert(key.to_string(), (r1, r2));
        //println!("r2: {:?}", r2);
    }

    let mut valid_tickets: Vec<Vec<usize>> = Vec::new();

    let mut sum: usize = 0;
    for line in input[2].trim().lines() {
        if line.contains("ticket") {
            continue;
        }

        let vals: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();

        let mut valid = true;
        for v in &vals {
            if !global_valid.contains(&v) {
                valid = false;
                sum += v;
            }
        }
        if valid {
            valid_tickets.push(vals.clone())
        }
    }
    println!("Part1: {}", sum);

    // for each column -> set of ranges it could belong to
    // each column starts with all possible, and marks entries as impossible
    // if a column only has 1 possible, mark for all others as impossible

    //let mut columns: Vec<&mut HashSet<String>> = vec![&mut HashSet::new(); constraints.len()];
    let mut fields: HashMap<String, HashSet<usize>> = HashMap::new();
    for key in constraints.keys() {
        let mut set: HashSet<usize> = HashSet::new();
        for i in 0..20 {
            set.insert(i);
        }
        fields.insert(key.to_string(), set);
    }

    for ticket in valid_tickets {
        for (idx, val) in ticket.iter().enumerate() {
            for key in constraints.keys() {
                let blah: &(Range, Range) = constraints.get(key).unwrap();
                //println!("{}", is_in_range(blah, &val));
                if !is_in_range(blah, &val) {
                    remove_possibility(&mut fields, key.to_string(), idx);
                }
            }
        }
    }

    let mut all_solved = false;
    while !all_solved {
        all_solved = true;
        // if only 1 possibility. Keep track of key + col
        // remove that col from everywhere not that key
        let mut to_remove: Vec<(String, usize)> = Vec::new();
        for key in constraints.keys() {
            let set = fields.get(key).unwrap();
            if set.len() == 1 {
                let mut only_remaining: usize = 99;
                // how did i get myself in this situation :D
                for only in fields.get(key).unwrap() {
                    only_remaining = *only;
                }
                to_remove.push((key.to_string(), only_remaining));
            } else {
                all_solved = false;
            }
        }

        for (rem_key, col) in &to_remove {
            for key in constraints.keys() {
                if key != rem_key {
                    remove_possibility(&mut fields, key.to_string(), *col);
                }
            }
        }
    }

    // grab actual ticket
    let mut ticket: Vec<usize> = Vec::new();
    for line in input[1].trim().lines() {
        if line.contains("ticket") {
            continue;
        }
        ticket = line.split(",")
            .map(|x| x.parse().unwrap())
            .collect();
    }

    let mut product: usize = 1;
    for key in fields.keys() {
        if key.contains("departure") {
            let mut only: usize = 99;
            for v in fields.get(key).unwrap() {
                only = *v;
            }
            product *= ticket[only];
            //println!("dep: {:?}", fields.get(key).unwrap());
        }
    }

    println!("{:#?}", fields);
    println!("Part2: {}", product);
}

// mark specific field as not possible for a certain column
fn remove_possibility(fields: &mut HashMap<String, HashSet<usize>>, field: String, column: usize) {
    let mut existing = fields.get(&field).unwrap().clone();
    existing.remove(&column);
    fields.insert(field, existing);
}

fn is_in_range(constraint: &(Range, Range), val: &usize) -> bool {
    if val >= &constraint.0.start && val <= &constraint.0.end {
        return true;
    }
    if val >= &constraint.1.start && val <= &constraint.1.end {
        return true;
    }
    return false;
}
