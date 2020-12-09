use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    part2();

}

fn part1() {

    let f = File::open("input.txt").expect("unable to open input");
    let f = BufReader::new(f);

    let mut max = 0;

    for line in f.lines() {

        let blah = line.unwrap();

        let row_str = &blah[..7];
        let col_str = &blah[7..];

        let row = parse_row(&row_str);
        let col = parse_col(&col_str);


        let id = (row * 8) + col;
        println!("id: {}", id);
        if id > max {
            max = id;
        }
    }

    println!("max: {}", max);
}

fn part2() {

    let f = File::open("input.txt").expect("unable to open input");
    let f = BufReader::new(f);

    let mut seats: Vec<isize> = Vec::new();

    for line in f.lines() {

        let blah = line.unwrap();

        let row_str = &blah[..7];
        let col_str = &blah[7..];

        let row = parse_row(&row_str);
        let col = parse_col(&col_str);


        let id = (row * 8) + col;
        println!("id: {}", id);
        seats.push(id);
    }

    seats.sort();
    let mut prev = seats[0];
    for (_, v) in seats.iter().enumerate() {
        if *v != (prev+1) {
            println!("missing between: {} {}", prev, v);
        }
        prev = *v;
    }

}

fn parse_row(s: &str) -> isize {
    let bits = s.replace("B", "1").replace("F", "0");
    return isize::from_str_radix(&bits, 2).unwrap();
}

fn parse_col(s: &str) -> isize {
    let bits = s.replace("R", "1").replace("L", "0");
    return isize::from_str_radix(&bits, 2).unwrap();
}
