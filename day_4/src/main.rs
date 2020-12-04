use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {


    println!("byr_valid: {}", validate_byr(&String::from("2021")));
    println!("hgt: {}", validate_hgt(&String::from("60in")));
    println!("hgt: {}", validate_hgt(&String::from("190cm")));
    println!("hgt: {}", validate_hgt(&String::from("190in")));
    println!("hgt: {}", validate_hgt(&String::from("190")));

    println!("------------------------------------");
    println!("hcl: {}", validate_hcl(&String::from("#123abc")));
    println!("hcl: {}", validate_hcl(&String::from("#123abz")));
    println!("hcl: {}", validate_hcl(&String::from("123abc")));

    part2();

    return




}

fn part1() {

    let f = File::open("input.txt").expect("unable to open input");
    let f = BufReader::new(f);

    let mut fields_map = HashMap::new();
    let mut valid_count = 0;

    for line in f.lines() {

        let line = line.unwrap().clone();
        if line.trim().len() == 0 {

    //        println!("fields: {}", fields_map.keys().len());
    //        println!("valid: {}", validate(&fields_map));
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

     //       println!("field: {}", field);
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

fn validate2(m: &HashMap<String, String>) -> bool {
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

    if !validate_byr(m.get("byr").unwrap()) { 
        println!("fail byr: {}", m.get("byr").unwrap());
        return false;
    }
    if !validate_iyr(m.get("iyr").unwrap()) {
        println!("fail iyr");
        return false;
    }
    if !validate_eyr(m.get("eyr").unwrap()) {
        println!("fail eyr");
        return false;
    }
    if !validate_hgt(m.get("hgt").unwrap()) { 
        println!("fail hgt");
        return false;
    }
    if !validate_hcl(m.get("hcl").unwrap()) {
        println!("fail hcl");
        return false;
    }
    if !validate_ecl(m.get("ecl").unwrap()) { 
        println!("fail ecl");
        return false;
    }
    if !validate_pid(m.get("pid").unwrap()) {
        println!("fail pid");
        return false;
    }

    return true;
}

fn validate_byr(byr: &String) -> bool {
    if byr.len() != 4 {
        return false;
    }
    let i: i32 = match byr.parse() {
        Ok(i) => i,
        Err(_) => return false,
    };
    return i >= 1920 && i <= 2002;
}

fn validate_iyr(iyr: &String) -> bool {
    if iyr.len() != 4 {
        return false;
    }
    let i: i32 = match iyr.parse() {
        Ok(i) => i,
        Err(_) => return false,
    };
    return i >= 2010 && i <= 2020;
}

fn validate_eyr(eyr: &String) -> bool {
    if eyr.len() != 4 {
        return false;
    }
    let i: i32 = match eyr.parse() {
        Ok(i) => i,
        Err(_) => return false,
    };
    return i >= 2020 && i <= 2030;
}

fn validate_hgt(hgt: &String) -> bool {
    if String::as_str(&hgt).ends_with("in") {
        let i: i32 = match hgt[..hgt.len()-2].parse() {
                Ok(i) => i,
                Err(_) => return false,
        };
        return i >= 59 && i <= 76;
    } else if String::as_str(&hgt).ends_with("cm") {
        let i: i32 = match hgt[..hgt.len()-2].parse() {
                    Ok(i) => i,
                    Err(_) => return false,
            };
            return i >= 150 && i <= 193;
    }
    return false
}

fn validate_hcl(hcl: &String) -> bool {
    if !hcl.starts_with("#") {
        return false;
    }
    let chars = hcl[1..].chars();
    for (_, b) in chars.enumerate() {
        if !b.is_ascii_hexdigit() {
            return false;
        }
    }

    return true;
}

fn validate_ecl(ecl: &String) -> bool {
    match ecl.as_str() {
        "amb"|"blu"|"brn"|"gry"|"grn"|"hzl"|"oth" => return true,
        _ => return false,
    }
}

fn validate_pid(pid: &String) -> bool {
    if pid.len() != 9 { return false; }

    for c in pid.chars() {
        if !c.is_digit(10) {
            return false;
        }
    }
    return true;
}


fn part2() {
    let f = File::open("input.txt").expect("unable to open input");
    let f = BufReader::new(f);

    let mut fields_map = HashMap::new();
    let mut valid_count = 0;

    struct Passport {
        byr: i32,
        iyr: i32,
        eyr: i32,
        hgt: String,
        hcl: String,
        pid: String,
    }

    for line in f.lines() {

        let line = line.unwrap().clone();
        if line.trim().len() == 0 {

            //println!("fields: {}", fields_map.keys().len());
            //println!("valid: {}", validate(&fields_map));
            if validate2(&fields_map) {
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
            fields_map.insert(field[0..3].to_string(), field[4..].to_string());
        }

    }

    println!("Result: {}", valid_count);
}
