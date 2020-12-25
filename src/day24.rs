// --- Day 24: Lobby Layout ---
// Your raft makes it to the tropical island; it turns out that the small crab was an excellent
// navigator. You make your way to the resort.
//
// As you enter the lobby, you discover a small problem: the floor is being renovated. You can't
// even reach the check-in desk until they've finished installing the new tile floor.
//
// The tiles are all hexagonal; they need to be arranged in a hex grid with a very specific color
// pattern. Not in the mood to wait, you offer to help figure out the pattern.
//
// The tiles are all white on one side and black on the other. They start with the white side facing
// up. The lobby is large enough to fit whatever pattern might need to appear there.
//
// A member of the renovation crew gives you a list of the tiles that need to be flipped over (your
// puzzle input). Each line in the list identifies a single tile that needs to be flipped by giving
// a series of steps starting from a reference tile in the very center of the room. (Every line
// starts from the same reference tile.)
//
// Because the tiles are hexagonal, every tile has six neighbors: east, southeast, southwest, west,
// northwest, and northeast. These directions are given in your list, respectively, as e, se, sw, w,
// nw, and ne. A tile is identified by a series of these directions with no delimiters; for example,
// esenee identifies the tile you land on if you start at the reference tile and then move one tile
// east, one tile southeast, one tile northeast, and one tile east.
//
// Each time a tile is identified, it flips from white to black or from black to white. Tiles might
// be flipped more than once. For example, a line like esew flips a tile immediately adjacent to the
// reference tile, and a line like nwwswee flips the reference tile itself.
//
// Here is a larger example:
//
// sesenwnenenewseeswwswswwnenewsewsw
// neeenesenwnwwswnenewnwwsewnenwseswesw
// seswneswswsenwwnwse
// nwnwneseeswswnenewneswwnewseswneseene
// swweswneswnenwsewnwneneseenw
// eesenwseswswnenwswnwnwsewwnwsene
// sewnenenenesenwsewnenwwwse
// wenwwweseeeweswwwnwwe
// wsweesenenewnwwnwsenewsenwwsesesenwne
// neeswseenwwswnwswswnw
// nenwswwsewswnenenewsenwsenwnesesenew
// enewnwewneswsewnwswenweswnenwsenwsw
// sweneswneswneneenwnewenewwneswswnese
// swwesenesewenwneswnwwneseswwne
// enesenwswwswneneswsenwnewswseenwsese
// wnwnesenesenenwwnenwsewesewsesesew
// nenewswnwewswnenesenwnesewesw
// eneswnwswnwsenenwnwnwwseeswneewsenese
// neswnwewnwnwseenwseesewsenwsweewe
// wseweeenwnesenwwwswnew
// In the above example, 10 tiles are flipped once (to black), and 5 more are flipped twice (to
// black, then back to white). After all of these instructions have been followed, a total of 10
// tiles are black.
//
// Go through the renovation crew's list and determine which tiles they need to flip. After all of
// the instructions have been followed, how many tiles are left with the black side up?

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn parse_instruction(line: &str) -> Vec<Direction> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(e|se|sw|w|nw|ne)").unwrap();
    }
    let mut directions: Vec<Direction> = Vec::new();
    for cap in REGEX.captures_iter(line) {
        directions.push(match cap.get(1).unwrap().as_str() {
            "e" => Direction::East,
            "se" => Direction::SouthEast,
            "sw" => Direction::SouthWest,
            "w" => Direction::West,
            "nw" => Direction::NorthWest,
            "ne" => Direction::NorthEast,
            _ => panic!("Unexpected input"),
        })
    }
    directions
}

/// Axial system
fn to_offset(direction: &Direction) -> (isize, isize) {
    match direction {
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
        Direction::SouthEast => (0, 1),
        Direction::SouthWest => (-1, 1),
        Direction::NorthWest => (0, -1),
        Direction::NorthEast => (1, -1),
    }
}

fn to_coordinates(directions: Vec<Direction>) -> (isize, isize) {
    directions
        .iter()
        .fold((0_isize, 0_isize), |(acc_x, acc_y), direction| {
            let (off_x, off_y) = to_offset(direction);
            (acc_x + off_x, acc_y + off_y)
        })
}

pub fn part1(lines: &[String]) -> usize {
    let instructions: Vec<_> = lines.iter().map(|line| parse_instruction(line)).collect();

    let mut black_tiles: HashSet<(isize, isize)> = HashSet::new();

    for instruction in instructions {
        let coordinates = to_coordinates(instruction);
        if black_tiles.contains(&coordinates) {
            black_tiles.remove(&coordinates);
        } else {
            black_tiles.insert(coordinates);
        }
    }

    black_tiles.len()
}

// --- Part Two ---
// The tile floor in the lobby is meant to be a living art exhibit. Every day, the tiles are all
// flipped according to the following rules:
//
// Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to
// white.
// Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
// Here, tiles immediately adjacent means the six tiles directly touching the tile in question.
//
// The rules are applied simultaneously to every tile; put another way, it is first determined which
// tiles need to be flipped, then they are all flipped at the same time.
//
// In the above example, the number of black tiles that are facing up after the given number of days
// has passed is as follows:
//
// Day 1: 15
// Day 2: 12
// Day 3: 25
// Day 4: 14
// Day 5: 23
// Day 6: 28
// Day 7: 41
// Day 8: 37
// Day 9: 49
// Day 10: 37
//
// Day 20: 132
// Day 30: 259
// Day 40: 406
// Day 50: 566
// Day 60: 788
// Day 70: 1106
// Day 80: 1373
// Day 90: 1844
// Day 100: 2208
// After executing this process a total of 100 times, there would be 2208 black tiles facing up.
//
// How many tiles will be black after 100 days?
pub fn part2(lines: &[String]) -> usize {
    let instructions: Vec<_> = lines.iter().map(|line| parse_instruction(line)).collect();

    let mut black_tiles_map: HashSet<(isize, isize)> = HashSet::new();

    for instruction in instructions {
        let coordinates = to_coordinates(instruction);
        if black_tiles_map.contains(&coordinates) {
            black_tiles_map.remove(&coordinates);
        } else {
            black_tiles_map.insert(coordinates);
        }
    }

    let adjacent = vec![(1, 0), (-1, 0), (0, 1), (-1, 1), (0, -1), (1, -1)];

    for _ in 0..100 {
        let current_black_tiles_map = black_tiles_map.clone();

        let min_x = current_black_tiles_map
            .iter()
            .map(|(x, _)| x)
            .min()
            .unwrap();
        let max_x = current_black_tiles_map
            .iter()
            .map(|(x, _)| x)
            .max()
            .unwrap();
        let min_y = current_black_tiles_map
            .iter()
            .map(|(_, y)| y)
            .min()
            .unwrap();
        let max_y = current_black_tiles_map
            .iter()
            .map(|(_, y)| y)
            .max()
            .unwrap();

        for x in min_x - 1..=max_x + 1 {
            for y in min_y - 1..=max_y + 1 {
                let is_black = current_black_tiles_map.contains(&(x, y));
                let mut adjacent_black_tiles = 0;
                for (acc_x, acc_y) in &adjacent {
                    if current_black_tiles_map.contains(&(x + acc_x, y + acc_y)) {
                        adjacent_black_tiles += 1;
                    }
                }

                if is_black && (adjacent_black_tiles == 0 || adjacent_black_tiles > 2) {
                    black_tiles_map.remove(&(x, y));
                } else if !is_black && adjacent_black_tiles == 2 {
                    black_tiles_map.insert((x, y));
                }
            }
        }
    }

    black_tiles_map.len()
}
