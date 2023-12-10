const STARTING_CHAR: char = 'J';
const STARTING_DIR: Direction = Right;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use std::collections::HashSet;

use Direction::*;

impl Direction {
    fn next_coords(&self, cur: (usize, usize)) -> (usize, usize) {
        match &self {
            Up => (cur.0 - 1, cur.1),
            Down => (cur.0 + 1, cur.1),
            Left => (cur.0, cur.1 - 1),
            Right => (cur.0, cur.1 + 1),
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    pt1(input);
    pt2(input);
}
fn pt2(input: &str) {
    let map: Vec<Vec<char>> = input
        .split('\n')
        .map(|l| l.trim().chars().collect())
        .collect();

    let mut starting_location = (0, 0);
    'outer: for (line_no, line) in map.iter().enumerate() {
        for (c_no, c) in line.iter().enumerate() {
            if c == &'S' {
                starting_location = (line_no, c_no);
                break 'outer;
            }
        }
    }
    if starting_location == (0, 0) {
        panic!("starting_location not found");
    }

    let mut loopy: HashSet<(usize, usize)> = HashSet::new();

    //since this is a contiguous loop, you just count the whole loop
    let mut cur_loc = starting_location;
    let mut prev_char_direction = STARTING_DIR;

    let mut start = true;
    loop {
        let mut cur_char = map[cur_loc.0][cur_loc.1];
        if !start && cur_char == 'S' {
            break;
        }
        if start {
            cur_char = STARTING_CHAR;
            start = false;
        }
        loopy.insert((cur_loc.0, cur_loc.1));

        prev_char_direction = match (cur_char, prev_char_direction) {
            ('|', Down) => Down,
            ('|', Up) => Up,
            ('-', Right) => Right,
            ('-', Left) => Left,
            ('L', Left) => Up,
            ('L', Down) => Right,
            ('7', Right) => Down,
            ('7', Up) => Left,
            ('F', Up) => Right,
            ('F', Left) => Down,
            ('J', Right) => Up,
            ('J', Down) => Left,
            _ => panic!("error"),
        };

        cur_loc = prev_char_direction.next_coords(cur_loc);
    }

    //now we just TV it
    let line_length = map.len();
    let row_len = map[0].len();

    let mut inside_tile_count = 0;

    for i in 0..line_length {
        let mut lines_passed_through = 0;
        let mut left_matcher: Option<char> = None;

        for j in 0..row_len {
            match loopy.get(&(i, j)) {
                Some(v) => {
                    let mut cur_char = map[v.0][v.1];
                    if cur_char == 'S' {
                        cur_char = STARTING_CHAR;
                    }

                    print!("{}", cur_char);
                    if cur_char == '-' {
                        continue;
                    }

                    if cur_char == 'F' || cur_char == 'L' {
                        left_matcher = Some(cur_char);
                        continue;
                    }

                    if let Some(ref aleft_matcher) = left_matcher {
                        if aleft_matcher == &'F' && cur_char == '7' {
                            continue;
                        }
                        if aleft_matcher == &'L' && cur_char == 'J' {
                            continue;
                        }
                    }
                    lines_passed_through += 1;
                }
                None => {
                    if lines_passed_through % 2 == 1 {
                        print!("I");
                        inside_tile_count += 1;
                    } else {
                        print!("O");
                    }
                }
            }
        }
        println!();
    }
    println!("inside tile count = {}", inside_tile_count);
}

fn pt1(input: &str) {
    let map: Vec<Vec<char>> = input
        .split('\n')
        .map(|l| l.trim().chars().collect())
        .collect();

    let mut starting_location = (0, 0);
    'outer: for (line_no, line) in map.iter().enumerate() {
        for (c_no, c) in line.iter().enumerate() {
            if c == &'S' {
                starting_location = (line_no, c_no);
                break 'outer;
            }
        }
    }
    if starting_location == (0, 0) {
        panic!("starting_location not found");
    }

    //since this is a contiguous loop, you just count the whole loop
    let mut count = 0;
    let mut cur_loc = starting_location;
    let mut prev_char_direction = STARTING_DIR;

    loop {
        let mut cur_char = map[cur_loc.0][cur_loc.1];
        if count != 0 && cur_char == 'S' {
            break;
        }
        if count == 0 {
            cur_char = STARTING_CHAR;
        }

        prev_char_direction = match (cur_char, prev_char_direction) {
            ('|', Down) => Down,
            ('|', Up) => Up,
            ('-', Right) => Right,
            ('-', Left) => Left,
            ('L', Left) => Up,
            ('L', Down) => Right,
            ('7', Right) => Down,
            ('7', Up) => Left,
            ('F', Up) => Right,
            ('F', Left) => Down,
            ('J', Right) => Up,
            ('J', Down) => Left,
            _ => panic!("error"),
        };

        cur_loc = prev_char_direction.next_coords(cur_loc);
        count += 1;
    }

    println!("count = {}", count / 2);
}
