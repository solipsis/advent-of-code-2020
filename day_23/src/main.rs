use std::collections::VecDeque;
use std::cmp;

const MAX_CUP: u32 = 20;
const MIN_CUP: u32 = 1;
fn main() {
    //let input = vec![3,8,9,1,2,5,4,6,7];
    let input = vec![3,8,9,1,2,5,4,6,7,10,11,12,13,14,15,16,17,18,19,20];
    //let input = vec![3,1,5,6,7,9,8,2,4,10,11,12,13,14,15,16,17,18,19,20];
    //let input = vec![1,2,3,4,5,6,7,8,9,10];
    //let input = vec![0; 1000000];
    let mut circle: VecDeque<u32> = VecDeque::with_capacity(MAX_CUP as usize);
    /*
    circle.push_back(3);
    circle.push_back(1);
    circle.push_back(5);
    circle.push_back(6);
    circle.push_back(7);
    circle.push_back(9);
    circle.push_back(8);
    circle.push_back(2);
    circle.push_back(4);
    for x in 9..MAX_CUP {
        circle.push_back(x as u32 + 1); 
    }
    */


    for i in input {
        circle.push_back(i);
    }
    println!("{:?}", circle);
    println!("start");

    //circle.drain(1..3);
    let mut current_cup;
    let mut current_cup_idx = 0;
    for x in 0..10 {
       // if x % 100000 == 0 {
       //     println!("checkpoint: {}", x);
       // }
        current_cup = circle[current_cup_idx];
        println!("-- Move {} -----", x+1);

        println!("circle: {:?} || {}", circle, current_cup);
        let target_idx = cmp::min(current_cup_idx+3, circle.len()-1);
        let mut pickup = circle.drain(current_cup_idx+1..target_idx+1).collect::<VecDeque<_>>();
        // if cups wrap around when picking up
        while pickup.len() < 3 {
            pickup.push_back(circle.pop_front().unwrap());
        }
        println!("pickup: {:?}", pickup);

        let mut destination_cup = if current_cup - 1 >= MIN_CUP {current_cup-1} else {MAX_CUP};
        while pickup.contains(&destination_cup) {
            destination_cup -= 1;
            if destination_cup < MIN_CUP {
                destination_cup = MAX_CUP;
            }
        }

        let mut destination_cup_idx = 0;
        for (idx, cup) in circle.iter().enumerate() {
            if *cup == destination_cup {
                destination_cup_idx = idx+1; // insert clockwise of target element
                break;
            }
        }
        println!("destination: {}, idx: {}", destination_cup, destination_cup_idx);

        // insert in reverse order because of shifting
        while !pickup.is_empty() {
            circle.insert(destination_cup_idx, pickup.pop_back().unwrap());
        }
        for (idx, cup) in circle.iter().enumerate() {
            if *cup == current_cup {
                current_cup_idx = idx+1;
                break;
            }
        }
        if current_cup_idx == circle.len() {
            current_cup_idx = 0;
        }

    }
    println!("{:?}", circle);
    /*
    println!("safety: {} {}", circle[0], circle[1]);
    for (idx, cup) in circle.iter().enumerate() {
        if *cup == 1 {
            println!("done: {}, {}", circle[idx+1], circle[idx+2]);
        }
    }
    */




}
