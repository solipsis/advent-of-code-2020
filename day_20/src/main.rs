use std::collections::HashMap;
use std::collections::HashSet;
use std::str;
use transpose;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    edges: Vec<u16>,
    data: Vec<u8>,
    orientation: u8,
}

struct Orientation {
    up: u16,
    left: u16,
    down: u16,
    right: u16,
}

const PIECE_SIZE: usize = 10;
const ROW_SIZE: usize = 3;
const COL_SIZE: usize = 3;

fn main() {
    let test = 1;
    let reversed = reverse_bits(test);
    //println!("{:#018b}", test);
    //println!("{:#018b}", reversed);

    let input: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();

    // for a given edge, which pieces contain it?
    let mut connected: HashMap<u16, HashSet<usize>> = HashMap::new();
    let mut tiles: HashMap<usize, Tile> = HashMap::new();

    //let mut tiles: Vec<Tile> = Vec::new();
    for tile_str in input {
        let tile = parse_tile(tile_str.to_string());
        for edge in &tile.edges {
            if !connected.contains_key(edge) {
                connected.insert(*edge, HashSet::new());
            }
            let links = connected.get_mut(edge).unwrap();
            links.insert(tile.id);
        }
        tiles.insert(tile.id, tile);
    }

    let mut tile_keys: Vec<usize> = Vec::new();
    for key in tiles.keys() {
        tile_keys.push(key.clone());
    }

    // try a starting tile
    for id in &tile_keys {
        let mut used: HashSet<usize> = HashSet::new();
        let mut grid: Vec<(usize, Orientation)> = Vec::new();
        attach_tile(*id, &mut grid, &mut used, &mut tiles, &connected);
    }

    println!("Fail");

    // for each tile
    //     for each orientatin
    //         for each tile that can connect to right side
    //             attach tile
    //             if row > 0, also check that it matches with tile above (mod operation with edge)
    //             if no more tiles to place, we are done
    //
}

fn attach_tile(
    id: usize,
    grid: &mut Vec<(usize, Orientation)>,
    used: &mut HashSet<usize>,
    tiles: &HashMap<usize, Tile>,
    connected: &HashMap<u16, HashSet<usize>>,
) {
    used.insert(id);
    //let mut tile = &tiles.get_mut(&id).unwrap();

    let edges: Vec<u16> = tiles.get(&id).unwrap().edges.clone();

    // try each orientation
    for orient in 0..4 {
        edges.rotate_right(1);

        let orientation = Orientation {
            up: edges[0],
            left: edges[1],
            down: edges[2],
            right: edges[3],
        };

        let left_side = edges.get(((orient + 1) % 4) as usize).unwrap();
        let right_side = edges.get(((orient + 3) % 4) as usize).unwrap();
        let top_side = edges.get(orient as usize).unwrap();

        // validate that we can put this piece here
        let row = grid.len() / ROW_SIZE;
        let col = grid.len() % COL_SIZE;

        // check if left side matches inverse of piece left of this
        if row > 1 {
            let left_tuple = &grid.last().unwrap();
            let prev = &tiles.get(&left_tuple.0).unwrap();
            let prev_right = prev.edges.get(((&left_tuple.1 + 3) % 4) as usize).unwrap();
            if edge_inverse(*left_side) != *prev_right {
                continue;
            }
        }
        // check if top side matches piece above this
        if col > 1 {
            let above_tuple = &grid.last().unwrap();
            let prev = &tiles.get(&above_tuple.0).unwrap();
            let prev_bottom = prev.edges.get(((&above_tuple.1 + 2) % 4) as usize).unwrap();
            if edge_inverse(*top_side) != *prev_bottom {
                continue;
            }
        }

        grid.push((id, orient.clone()));
        println!("Grid: {:?}", grid);

        if grid.len() == 9 {
            panic!("hooray");
        }

        // check all pieces that have the inverse edge of the current right side
        match connected.get(&edge_inverse(*right_side)) {
            None => continue,
            Some(connections) => {
                for next_id in connections {
                    attach_tile(*next_id, grid, used, tiles, connected);
                }
            }
        }
        grid.pop();
    }

    used.remove(&id);
}

fn parse_tile(tile_str: String) -> Tile {
    let mut lines: Vec<&str> = tile_str.split("\n").collect();
    let id: usize = lines[0].split(" ").collect::<Vec<&str>>()[1]
        .strip_suffix(":")
        .unwrap()
        .parse()
        .unwrap();
    // println!("\n-----------------------------------------\nid: {}", id);

    //let mut data: Vec<Vec<char>> = Vec::new();
    lines = lines[1..].to_vec();
    let mut data: Vec<u8> = lines.join("").bytes().collect();

    // parse edges counter-clocwise TOP->LEFT->BOTTOM->RIGHT
    let top: u16 = run_to_num(data[..PIECE_SIZE].to_vec());
    data = rotate_piece(data);
    let left: u16 = run_to_num(data[..PIECE_SIZE].to_vec());
    data = rotate_piece(data);
    let bottom: u16 = run_to_num(data[..PIECE_SIZE].to_vec());
    data = rotate_piece(data);
    let right: u16 = run_to_num(data[..PIECE_SIZE].to_vec());

    // back to start
    data = rotate_piece(data);

    return Tile {
        id,
        data,
        edges: vec![top, left, bottom, right],
        orientation: 0,
    };
}

fn rotate_piece(data: Vec<u8>) -> Vec<u8> {
    // rotate matrix by transposing the data, then reversing the rows
    let mut transposed: Vec<u8> = vec![0; PIECE_SIZE * PIECE_SIZE];
    transpose::transpose(&data, &mut transposed, PIECE_SIZE, PIECE_SIZE);
    for x in 0..PIECE_SIZE {
        transposed[x * PIECE_SIZE..(PIECE_SIZE + x * PIECE_SIZE)].reverse();
    }

    /*
    for row in transposed.chunks(PIECE_SIZE) {
        let s = str::from_utf8(row).unwrap();
        println!("{:?}", s);
    }
    */
    return transposed;
}

fn run_to_num(run: Vec<u8>) -> u16 {
    //println!("Run: {}", run);
    let mut num: u16 = 0;
    for (idx, b) in run.iter().enumerate() {
        if *b == '#' as u8 {
            num = num | (1 << ((PIECE_SIZE - 1) - idx));
        }
    }
    println!("Num: {:#012b}", num);
    println!("inv: {:#012b}", edge_inverse(num));
    return num;
}

fn edge_inverse(edge: u16) -> u16 {
    return reverse_bits(edge) >> 6;
}

fn reverse_bits(x: u16) -> u16 {
    let mut rev = x;
    rev = ((rev & 0xaaaa) >> 1) | ((rev & 0x5555) << 1);
    rev = ((rev & 0xcccc) >> 2) | ((rev & 0x3333) << 2);
    rev = ((rev & 0xf0f0) >> 4) | ((rev & 0x0f0f) << 4);
    rev = ((rev & 0xff00) >> 8) | ((rev & 0x00ff) << 8);
    rev
}
