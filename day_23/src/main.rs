use std::collections::VecDeque;
use std::cmp;

const MAX_CUP: usize = 9;
const MIN_CUP: usize = 1;
fn main() {
    //let input = vec![3,8,9,1,2,5,4,6,7];
    let input = vec![3,1,5,6,7,9,8,2,4];

    let mut next = vec![0; 1+MAX_CUP as usize]; // 1 indexed 


    for (idx, v) in input.iter().enumerate() {
        if idx == input.len()-1 {
            next[*v as usize] = input[0];
        } else {
            next[*v as usize] = input[idx+1];
        }
    }
    //println!("{:?}", next);

    let mut current_cup = input[0];
    let mut take: Vec<usize> = vec![0; 3];
    for _x in 0..100 {
       // println!("-- Move {} -----", x+1);
      //  println!("{:?}", vec![1,2,3,4,5,6,7,8,9]);
      //  println!("{:?}", next[1..].to_vec());
      //  print(&next);
        
        // grab 3 to the right of current
        take[0] = next[current_cup];
        take[1] = next[take[0]];
        take[2] = next[take[1]];

        // re-attach current across gap
        next[current_cup] = next[take[2]];

        // calculate destination
        let mut destination = current_cup - 1;
        if destination < MIN_CUP { destination = MAX_CUP; }
        while take.contains(&destination) {
            destination -= 1;
            if destination < MIN_CUP { destination = MAX_CUP; }
        }
        // track cup currently after destination
        let tmp = next[destination];

        next[destination] = take[0];
        next[take[2]] = tmp;


     //   println!("pickup: {:?}", take);
     //   println!("destination: {:?}", destination);

     //   println!("-----------------");

        current_cup = next[current_cup];
    }
    print(&next);
}

fn print(next: &Vec<usize>) {
    let mut idx = 1;
    let mut out = Vec::new();
    for _x in 0..next.len()-1 {
        out.push(idx);
        idx = next[idx];
    }
    println!("{:?}", out);
    
}

