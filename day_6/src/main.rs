
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    //part1();
    part2();
}

fn part1() {

    let mut file = File::open("input.txt").expect("unable to open input");
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}",  why),
        Ok(_) => (),
    }

    let mut sum = 0;
    let input: Vec<String> = s.split("\n\n").map(|x| x.replace("\n", " ")).collect();
    for i in input {
        let mut uniq = HashSet::new();
        let entries: Vec<&str> = i.split(" ").collect();
        for e in entries {
            for c in e.chars() {
                uniq.insert(c.to_string());
            }
        }
        sum += uniq.len();
        println!("in: {}", i);
        println!("map: {:?}", uniq);
    }
    println!("sum: {}", sum);
}

// XXX: Solution requires trailing newline at end of input
fn part2() {

    let mut file = File::open("input.txt").expect("unable to open input");
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}",  why),
        Ok(_) => (),
    }

    let mut sum = 0;
    let input: Vec<String> = s.split("\n\n").map(|x| x.replace("\n", " ")).collect();
    for i in input {

        let mut uniq: HashMap<String, i32> = HashMap::new();
        let entries: Vec<&str> = i.split(" ").collect();


        let mut localSum = 0;
        // for each person
        for e in &entries {


            // each answer a person chose
            for c in e.chars() {
                let cur = match &uniq.get(&c.to_string()) {
                    Some(v) => v,
                    None => &0 as &i32
                };
                &uniq.insert(c.to_string(), cur+1);
            }
        }
        
        for (_q, &count) in &uniq {
            println!("count: {}, entrieslen: {}", count, entries.len());
            if count as usize == entries.len() {
                localSum += 1;
                sum += 1;
            }
        }

        println!("in: {}", i);
        println!("map: {:?}", uniq);
        println!("local: {}", localSum);
    }
    println!("sum: {}", sum);
}
