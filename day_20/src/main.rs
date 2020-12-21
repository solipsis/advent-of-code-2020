use std::collections::HashMap;
use std::collections::HashSet;
use std::str;
use transpose;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    edges: Vec<u16>, // treat each edge as 10 bits. Was probably an unneeded optimization
    data: Vec<u8>,
    orientation: u8,
}

#[derive(Debug, Clone)]
struct Orientation {
    up: u16,
    left: u16,
    down: u16,
    right: u16,
}

const PIECE_SIZE: usize = 10;
const ROW_SIZE: usize = 12;
const COL_SIZE: usize = 12;

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();

    // for a given edge, which pieces contain it?
    let mut connected: HashMap<u16, HashSet<usize>> = HashMap::new();
    let mut tiles: HashMap<usize, Tile> = HashMap::new();

    for tile_str in input {
        let tile = parse_tile(tile_str.to_string());
        for edge in &tile.edges {
            let inverse = edge_inverse(*edge);
            if !connected.contains_key(edge) {
                connected.insert(*edge, HashSet::new());
            }
            if !connected.contains_key(&inverse) {
                connected.insert(inverse, HashSet::new());
            }
            connected.get_mut(edge).unwrap().insert(tile.id);
            connected.get_mut(&inverse).unwrap().insert(tile.id);
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
}

fn attach_tile(
    id: usize,
    grid: &mut Vec<(usize, Orientation)>,
    used: &mut HashSet<usize>,
    tiles: &HashMap<usize, Tile>,
    connected: &HashMap<u16, HashSet<usize>>,
) {
    used.insert(id); // TODO: Does rust have a defer equivalent?
    let mut edges: Vec<u16> = tiles.get(&id).unwrap().edges.clone();

    // try flipped both directions (flip at end of loop)
    for _flip in 0..2 {
        // try each orientation
        for _orient in 0..4 {
            edges.rotate_right(1);

            let orientation = Orientation {
                up: edges[0],
                left: edges[1],
                down: edges[2],
                right: edges[3],
            };

            // validate that we can put this piece here
            let row = grid.len() / ROW_SIZE;
            let col = grid.len() % COL_SIZE;

            // check if left side matches inverse of piece left of this
            if col >= 1 {
                let prev_edges = &grid.last().unwrap();
                if edge_inverse(orientation.left) != prev_edges.1.right {
                    continue;
                }
            }
            // check if top side matches piece above this
            if row >= 1 {
                let above_edges = &grid.get(grid.len() - ROW_SIZE).unwrap();
                if edge_inverse(orientation.up) != above_edges.1.down {
                    continue;
                }
            }

            grid.push((id, orientation.clone()));

            // We are done if all pieces have been placed
            if grid.len() == ROW_SIZE * COL_SIZE {
                println!(
                    "Part_1: {}",
                    grid[0].0
                        * grid[COL_SIZE - 1].0
                        * grid[(ROW_SIZE - 1) * COL_SIZE].0
                        * grid[(ROW_SIZE * COL_SIZE) - 1].0
                );
                // panic!("hooray");
                part_2(grid, tiles);
            }

            // check all pieces that could possible connect to this one
            // different if next piece starts a new row
            let candidates;
            if col == COL_SIZE - 1 {
                let row_starter = grid[row * ROW_SIZE].1.clone();
                candidates = connected.get(&edge_inverse(row_starter.down));
            } else {
                candidates = connected.get(&edge_inverse(orientation.right));
            }

            match candidates {
                None => (),
                Some(connections) => {
                    for next_id in connections {
                        // don't reuse pieces
                        if used.contains(&next_id) {
                            continue;
                        }
                        attach_tile(*next_id, grid, used, tiles, connected);
                    }
                }
            }
            grid.pop();
        }

        // flip the piece by reversing all edges and swapping left+right
        for edge in edges.iter_mut() {
            *edge = edge_inverse(*edge);
        }
        let tmp = edges[1]; //left
        edges[1] = edges[3];
        edges[3] = tmp;
    }

    used.remove(&id);
}

fn part_2(grid: &mut Vec<(usize, Orientation)>, tiles: &HashMap<usize, Tile>) {
    // mega image 8x8 * 12 * 12
    // for each tile in our unstripped image
    //     figure out the orientation
    //         print appropriately to mega image
    let mut complete: Vec<u8> = vec!['_' as u8; 8 * 8 * (COL_SIZE * ROW_SIZE)];
    for (idx, piece) in grid.iter().enumerate() {
        let tile = tiles.get(&piece.0).unwrap();
        let mut data = tile.data.clone();
        // keep rotating until it matches our intended format
        'outer: for _flip in 0..2 {
            for _rotate in 0..4 {
                data = rotate_piece(data);
                if run_to_num(data[..PIECE_SIZE].to_vec()) == piece.1.up {
                    // strip frame from image
                    let mut stripped = Vec::new();
                    for (idx, b) in data.iter().enumerate() {
                        let sub_col = idx % PIECE_SIZE;
                        let sub_row = idx / PIECE_SIZE;
                        if sub_row == 0 || sub_row == 9 || sub_col == 0 || sub_col == 9 {
                            continue;
                        }
                        stripped.push(*b);
                    }
                    data = stripped;
                    break 'outer;
                }
            }

            // flip the piece by reversing all rows
            for row in data.chunks_mut(10) {
                row.reverse();
            }
        }

        // calc piece row / column
        // multiply by sub_rows + sub_cols
        // add to mega image
        //
        let row = idx / ROW_SIZE;
        let col = idx % COL_SIZE;
        let sub_row_size = ROW_SIZE * 8;
        let base = (row * ROW_SIZE * 64) + (col * 8);
        for (idx, b) in data.iter().enumerate() {
            let sub_col = idx % 8;
            let sub_row = idx / 8;

            let pixel_idx = base + sub_col + (sub_row * sub_row_size);
            complete[pixel_idx] = *b;
        }
    }

    // look in all orientations for sea monsters
    for _flip in 0..2 {
        for _rot in 0..4 {
            // search for monstros
            //                   #
            // #    ##    ##    ###
            //  #  #  #  #  #  #
            let monster_offsets: Vec<(usize, isize)> = vec![
                (18, -1),
                (0, 0),
                (5, 0),
                (6, 0),
                (11, 0),
                (12, 0),
                (17, 0),
                (18, 0),
                (19, 0),
                (1, 1),
                (4, 1),
                (7, 1),
                (10, 1),
                (13, 1),
                (16, 1),
            ];
            let monster_length = 20;
            let mut monsters_found = false;
            let pixel_row_size = ROW_SIZE * 8;
            'search: for idx in 0..complete.len() {
                //'outer: for (idx, pixel) in complete.iter_mut().enumerate() {
                let row = idx / pixel_row_size;
                let col = idx % pixel_row_size;

                // ignore if not enough room for monster
                if row == 0
                    || col > (pixel_row_size + 1) - monster_length
                    || row >= (complete.len() / pixel_row_size) - 1
                {
                    continue;
                }

                for offset in &monster_offsets {
                    //println!("{}", (pixel_row_size as isize));
                    //println!("{}", (10000 + (pixel_row_size as isize*offset.1)) as usize);
                    let adjusted =
                        ((idx + offset.0) as isize + (pixel_row_size as isize * offset.1)) as usize;
                    //let adjusted = (idx + offset.0) + (pixel_row_size*offset.1);
                    if complete[adjusted] != '#' as u8 {
                        continue 'search;
                    }
                }
                monsters_found = true;

                // mark it on image
                for offset in &monster_offsets {
                    let adjusted =
                        ((idx + offset.0) as isize + (pixel_row_size as isize * offset.1)) as usize;
                    // let adjusted = (idx + offset.0) + (pixel_row_size*offset.1);
                    complete[adjusted] = 'O' as u8;
                }
            }

            // we are done
            if monsters_found {
                for line in complete.chunks(ROW_SIZE * 8) {
                    println!("{}", str::from_utf8(line).unwrap());
                }
                let mut sum = 0;
                for u in complete {
                    if u == '#' as u8 {
                        sum += 1;
                    }
                }
                println!("Part2: {}", sum);
                panic!("done"); // too lazy properly clean this up
            }

            // rotate image
            complete = rotate_image(complete);
        }

        // flip image
        for line in complete.chunks_mut(ROW_SIZE * 8) {
            line.reverse();
        }
    }
}

fn parse_tile(tile_str: String) -> Tile {
    let mut lines: Vec<&str> = tile_str.split("\n").collect();
    let id: usize = lines[0].split(" ").collect::<Vec<&str>>()[1]
        .strip_suffix(":")
        .unwrap()
        .parse()
        .unwrap();

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
    transposed
}

fn rotate_image(data: Vec<u8>) -> Vec<u8> {
    let mut transposed: Vec<u8> = Vec::with_capacity(data.len());
    transposed.resize(data.len(), 0);
    transpose::transpose(&data, &mut transposed, ROW_SIZE * 8, COL_SIZE * 8);

    for line in transposed.chunks_mut(ROW_SIZE * 8) {
        line.reverse();
    }
    transposed
}

fn run_to_num(run: Vec<u8>) -> u16 {
    let mut num: u16 = 0;
    for (idx, b) in run.iter().enumerate() {
        if *b == '#' as u8 {
            num = num | (1 << ((PIECE_SIZE - 1) - idx));
        }
    }
    num
}

fn edge_inverse(edge: u16) -> u16 {
    reverse_bits(edge) >> 6
}

fn reverse_bits(x: u16) -> u16 {
    let mut rev = x;
    rev = ((rev & 0xaaaa) >> 1) | ((rev & 0x5555) << 1);
    rev = ((rev & 0xcccc) >> 2) | ((rev & 0x3333) << 2);
    rev = ((rev & 0xf0f0) >> 4) | ((rev & 0x0f0f) << 4);
    rev = ((rev & 0xff00) >> 8) | ((rev & 0x00ff) << 8);
    rev
}
