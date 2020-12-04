use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {

    let f = File::open("input.txt").expect("unable to open input");
    let f = BufReader::new(f);

    let mut fields_map = HashMap::new();
    let mut valid_count = 0;

    for line in f.lines() {

        let line = line.unwrap().clone();
        if line.trim().len() == 0 {

            println!("fields: {}", fields_map.keys().len());
            println!("valid: {}", validate(&fields_map));
            if validate(&fields_map) {
                valid_count += 1;
            }
            fields_map = HashMap::new();
            continue;
        }

        // split each line by spaces. Add all key to set
        // when empty line, check if preceding set has all needed values
        let fields: Vec<_> = line.trim().split(' ').collect();
        for (_i, &field) in fields.iter().enumerate() {

            println!("field: {}", field);
            fields_map.insert(field[0..3].to_string(), true);
        }

    }

    println!("Result: {}", valid_count);

}

fn validate(m: &HashMap<String, bool>) -> bool {
    if m.get("byr").is_none() {
        return false;
    }
    if m.get("iyr").is_none() {
        return false;
    }
    if m.get("eyr").is_none() {
        return false;
    }
    if m.get("hgt").is_none() {
        return false;
    }
    if m.get("hcl").is_none() {
        return false;
    }
    if m.get("ecl").is_none() {
        return false;
    }
    if m.get("pid").is_none() {
        return false;
    }

    return true;
}
