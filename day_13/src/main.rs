fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = input.trim().split("\n");
    let earliest: i32 = input.next().unwrap().parse().unwrap();
    let schedule: Vec<i32> = input.next().unwrap()
        .split(",")
        .map(|x| match x.parse() {
            Ok(x) => x,
            Err(_) => -1,
        })
        .collect();

    let mut min = 99999;
    let mut min_id = -1;
    for bus in &schedule {
        if *bus == -1 {
            continue;
        }

        let div = earliest / *bus;
        let diff = ((div+1) * bus) - earliest;
        if diff < min {
            min = diff;
            min_id = *bus;
        }

    }

    println!("{:?}", schedule);
    println!("Part1: {}", min * min_id);

    println!("Chinese remainder theorem params:");
    let mut idx = 0;
    for bus in &schedule {
        if *bus == -1 {
            idx += 1;
            continue
        }
        let m = ((10 * *bus) - idx) % *bus;
        println!("{} mod {}", m, bus);
        idx += 1;
    }
        
}
