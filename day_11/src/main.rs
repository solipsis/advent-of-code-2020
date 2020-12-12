
fn main() {
    let mut input: Vec<Vec<char>> = std::fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let mut prev = count_seats(&input);
    loop {
       // println!("--------------");

        let cpy = clone_grid(&input);
        let mut row: usize = 0;
        while row < input.len() {
            let mut col: usize = 0;
            while col < input[0].len() {
           //     input[row][col] = update(&cpy, row, col);
                input[row][col] = update_part_2(&cpy, row, col);
                col += 1;
            }
            row += 1;
        }
        
        if count_seats(&input) == prev {
            println!("done: {}", prev);
            return;
        }
        prev = count_seats(&input);
       // print_grid(&input);
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!("{:?}", line);
    }
}

fn clone_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for row in grid {
        result.push(row.clone());
    }
    return result;
}

fn count_seats(grid: &Vec<Vec<char>>) -> i32 {
    let mut occupied = 0;
    for row in grid {
        for col in row {
            if *col == '#' {
                occupied += 1;
            }
        }
    }
    return occupied;
}

fn update(grid: &Vec<Vec<char>>, row: usize, col: usize) -> char {

    if grid[row][col] == '.' { return '.' }

    let points: Vec<(i8, i8)> = vec![(-1,-1),(-1,0),(-1,1), (0,-1),(0,1), (1,-1),(1,0),(1,1)];
    let mut adjacent_occupied = 0;

    let max_row = grid.len() as i8;
    let max_col = grid[0].len() as i8;

    for (r, c) in points {
        let r = row as i8 + r;
        let c = col as i8 + c;
        if r >= 0 && r < max_row && c >= 0 && c < max_col {
            if grid[r as usize][c as usize] == '#' {
                adjacent_occupied += 1;
            }
        }
    }
    if grid[row][col] == 'L' && adjacent_occupied == 0 {
        return '#';
    } else if grid[row][col] == '#' && adjacent_occupied >= 4 {
        return 'L';
    } else {
        return grid[row][col];
    }
}

fn update_part_2(grid: &Vec<Vec<char>>, row: usize, col: usize) -> char {

    if grid[row][col] == '.' { return '.' }

    let points: Vec<(i8, i8)> = vec![(-1,-1),(-1,0),(-1,1), (0,-1),(0,1), (1,-1),(1,0),(1,1)];
    let mut adjacent_occupied = 0;

    let max_row = grid.len() as i8;
    let max_col = grid[0].len() as i8;

    for (r, c) in points {
        let mut lr = row as i8;
        let mut lc = col as i8;

        // keep adding r/c until valid or out of bound
        loop {
            lr = lr + r;
            lc = lc + c;

            if !(lr >= 0 && lr < max_row && lc >= 0 && lc < max_col) {
                break;
            }
            match grid[lr as usize][lc as usize] {
                '.' => { continue; }
                '#' => { 
                    adjacent_occupied += 1;
                    break;
                }
                'L' => { break; }
                _ => ()
            }
        }
    }

    if grid[row][col] == 'L' && adjacent_occupied == 0 {
        return '#';
    } else if grid[row][col] == '#' && adjacent_occupied >= 5 {
        return 'L';
    } else {
        return grid[row][col];
    }
}

