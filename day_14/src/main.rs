use std::collections::HashMap;

fn recurse(address: &Vec<char>, val: &Vec<char>, memory: &mut HashMap<usize, Vec<char>>) {
    // see if any remaining X's
    // if not write val to mask location
    let mut x_loc: usize = 99;
    for (i, c) in address.iter().enumerate() {
        if *c == 'X' {
            x_loc = i;
            break;
        }
    }
    
    if x_loc == 99 {
        let register: String = address.clone().iter().collect();
        let register: usize = usize::from_str_radix(&register, 2).unwrap();
        *memory.entry(register).or_insert(vec!['z']) = val.clone();
        return;
    }

    // else recurse replacing X with 1 / 0 respectively
    let mut substituted: Vec<char> = address.clone();
    substituted[x_loc] = '0';
    recurse(&substituted, val, memory);
    substituted[x_loc] = '1';
    recurse(&substituted, val, memory);


}

fn part_2(input: &Vec<String>) {

    let mut mask: Vec<char> = vec!['0'; 36];

    let mut memory: HashMap<usize, Vec<char>> = HashMap::new();
    //let mut mem: Vec<char> = vec!['0'; 36];

    for line in input {
        if line.starts_with("mask") {
            let mask_str = line.strip_prefix("mask = ").unwrap();
            for (i, c) in mask_str.chars().enumerate() {
                mask[i] = c;
            }
        } else {
            let rbrace = line.find(']').unwrap();
            let mem_idx_str = &line[4..rbrace];
            let register: usize = mem_idx_str.parse().unwrap();

            let val_idx = line.rfind(' ').unwrap();
            let val: usize = line[val_idx..].trim().parse().unwrap();

            let formatted_value = format!("{:036b}", val); 
            let formatted_address = format!("{:036b}", register);

            let mut address: Vec<char> = formatted_address.clone().chars().collect();
            for (i, m) in mask.iter().enumerate() {
                if *m == '1' {
                    address[i] = '1';
                }
                if *m == 'X' {
                    address[i] = 'X';
                }
            }
            recurse(&address, &formatted_value.clone().chars().collect(), &mut memory);
        }
    }

    let mut sum: usize = 0;
    println!("memsize: {}", memory.len());
    for (_, arr) in memory {
        let val_str: String = arr.into_iter().collect();
        let intval = usize::from_str_radix(&val_str, 2).unwrap();
        sum += intval;
    }

    println!("Part2 sum: {}", sum);
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .trim()
        .lines()
        .map(|x| x.to_string())
        .collect();

    let mut mask: Vec<char> = vec!['0'; 36];

    let mut memory: HashMap<usize, Vec<char>> = HashMap::new();
    //let mut mem: Vec<char> = vec!['0'; 36];

    for line in &input {
        if line.starts_with("mask") {
            let mask_str = line.strip_prefix("mask = ").unwrap();
            for (i, c) in mask_str.chars().enumerate() {
                mask[i] = c;
            }
        //    println!("Mask: {:?}", mask);
        } else {
            let rbrace = line.find(']').unwrap();
            let mem_idx_str = &line[4..rbrace];
            let register: usize = mem_idx_str.parse().unwrap();

            let val_idx = line.rfind(' ').unwrap();
            let val: usize = line[val_idx..].trim().parse().unwrap();

            let formatted = format!("{:036b}", val); 

            let mut result: Vec<char> = formatted.clone().chars().collect();
            for (i, m) in mask.iter().enumerate() {
                if *m == '0' {
                    result[i] = '0';
                } else if *m == '1' {
                    result[i] = '1';
                }
            }

            //memory[&register] = result;
            *memory.entry(register).or_insert(vec!['z']) = result;
        }
    }

    let mut sum: usize = 0;
    println!("memsize: {}", memory.len());
    for (_, arr) in memory {
        let val_str: String = arr.into_iter().collect();
        let intval = usize::from_str_radix(&val_str, 2).unwrap();
        sum += intval;
    }

    println!("Part1: sum: {}", sum);
    part_2(&input);
}
