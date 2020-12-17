// --- Day 17: Conway Cubes ---
// As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source aboard one of their super-secret imaging satellites.
//
// The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.
//
// The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional coordinate (x,y,z), there exists a single cube which is either active or inactive.
//
// In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to this is a small flat region of cubes (your puzzle input); the cubes in this region start in the specified active (#) or inactive (.) state.
//
// The energy source then proceeds to boot up by executing six cycles.
//
// Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.
//
// During a cycle, all cubes simultaneously change their state according to the following rules:
//
// If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
// If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
// The engineers responsible for this experimental energy source would like you to simulate the pocket dimension and determine what the configuration of cubes should be at the end of the six-cycle boot process.
//
// For example, consider the following initial state:
//
// .#.
// ..#
// ###
// Even though the pocket dimension is 3-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the 3-dimensional space.)
//
// Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view follows the active cells in each cycle):
//
// Before any cycles:
//
// z=0
// .#.
// ..#
// ###
//
//
// After 1 cycle:
//
// z=-1
// #..
// ..#
// .#.
//
// z=0
// #.#
// .##
// .#.
//
// z=1
// #..
// ..#
// .#.
//
//
// After 2 cycles:
//
// z=-2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-1
// ..#..
// .#..#
// ....#
// .#...
// .....
//
// z=0
// ##...
// ##...
// #....
// ....#
// .###.
//
// z=1
// ..#..
// .#..#
// ....#
// .#...
// .....
//
// z=2
// .....
// .....
// ..#..
// .....
// .....
//
//
// After 3 cycles:
//
// z=-2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
//
// z=-1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...
//
// z=0
// ...#...
// .......
// #......
// .......
// .....##
// .##.#..
// ...#...
//
// z=1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...
//
// z=2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
// After the full six-cycle boot process completes, 112 cubes are left in the active state.
//
// Starting with your given initial configuration, simulate six cycles. How many cubes are left in
// the active state after the sixth cycle?

fn parse_map_line(line: &str) -> Vec<bool> {
    line.chars()
        .map(|char| match char {
            '.' => false,
            '#' => true,
            _ => panic!("Unexpected char {}", char),
        })
        .collect()
}

pub fn part1(lines: &[String]) -> usize {
    let rows: Vec<Vec<bool>> = lines.iter().map(|line| parse_map_line(line)).collect();

    let cycles = 6;

    // Note: The map will grow by 2 (+1 & -1) in each dimension for each cycle.

    let mut pocket_dimension = vec![ // Z
        vec![ // Y
            vec![ // X
                false;
                rows[0].len() + 2*cycles
            ];
            rows.len() + 2*cycles];
        1 + 2*cycles
    ];

    for (y, row) in rows.iter().enumerate() {
        for (x, cube) in row.iter().enumerate() {
            pocket_dimension[cycles][y + cycles][x + cycles] = *cube
        }
    }

    for _ in 1..=cycles {
        let previous_pocket_dimension = pocket_dimension.clone();

        // For each cube in the pocket dimension
        for (z, plane) in previous_pocket_dimension.iter().enumerate() {
            for (y, row) in plane.iter().enumerate() {
                'main: for (x, cube) in row.iter().enumerate() {
                    let mut active_neightbours = 0;

                    // Check its neighbours
                    let min_z = if z > 0 { z - 1 } else { z };
                    let min_y = if y > 0 { y - 1 } else { y };
                    let min_x = if x > 0 { x - 1 } else { x };
                    let max_z = if z < previous_pocket_dimension.len() - 1 {
                        z + 1
                    } else {
                        z
                    };
                    let max_y = if y < plane.len() - 1 { y + 1 } else { y };
                    let max_x = if x < row.len() - 1 { x + 1 } else { x };

                    #[allow(clippy::needless_range_loop)]
                    for target_z in min_z..=max_z {
                        for target_y in min_y..=max_y {
                            for target_x in min_x..=max_x {
                                if target_x == x && target_y == y && target_z == z {
                                    continue;
                                };
                                let neighbour_cube =
                                    previous_pocket_dimension[target_z][target_y][target_x];
                                if neighbour_cube {
                                    active_neightbours += 1;

                                    // If a cube is active and exactly 2 or 3 of its neighbors are
                                    // also active, the cube remains active.
                                    // Otherwise, the cube becomes inactive.
                                    if *cube && active_neightbours > 3 {
                                        pocket_dimension[z][y][x] = false;
                                        continue 'main;
                                    }

                                    // If a cube is inactive but exactly 3 of its neighbors are
                                    // active, the cube becomes active.
                                    // Otherwise, the cube remains inactive.
                                    if !*cube && active_neightbours > 3 {
                                        continue 'main;
                                    }
                                }
                            }
                        }
                    }

                    // If a cube is active and exactly 2 or 3 of its neighbors are
                    // also active, the cube remains active.
                    // Otherwise, the cube becomes inactive.
                    if *cube && !(active_neightbours == 2 || active_neightbours == 3) {
                        pocket_dimension[z][y][x] = false
                    }
                    // If a cube is inactive but exactly 3 of its neighbors are
                    // active, the cube becomes active.
                    // Otherwise, the cube remains inactive.
                    else if !*cube && active_neightbours == 3 {
                        pocket_dimension[z][y][x] = true
                    }
                }
            }
        }

        // println!("State at {}", i);
        // for plane in pocket_dimension.clone() {
        //     println!("State at {:?}", plane);
        // }
    }

    pocket_dimension
        .iter()
        .map(|plane| {
            plane
                .iter()
                .map(|row| row.iter().filter(|cube| **cube).count())
                .sum::<usize>()
        })
        .sum::<usize>()
}

// --- Part Two ---
// For some reason, your simulated results don't match what the experimental energy source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.
//
// The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional coordinate (x,y,z,w), there exists a single cube (really, a hypercube) which is still either active or inactive.
//
// Each cube only ever considers its neighbors: any of the 80 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.
//
// The initial state of the pocket dimension still consists of a small flat region of cubes. Furthermore, the same rules for cycle updating still apply: during each cycle, consider the number of active neighbors of each cube.
//
// For example, consider the same initial state as in the example above. Even though the pocket dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)
//
// Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z and w coordinate:
//
// Before any cycles:
//
// z=0, w=0
// .#.
// ..#
// ###
//
//
// After 1 cycle:
//
// z=-1, w=-1
// #..
// ..#
// .#.
//
// z=0, w=-1
// #..
// ..#
// .#.
//
// z=1, w=-1
// #..
// ..#
// .#.
//
// z=-1, w=0
// #..
// ..#
// .#.
//
// z=0, w=0
// #.#
// .##
// .#.
//
// z=1, w=0
// #..
// ..#
// .#.
//
// z=-1, w=1
// #..
// ..#
// .#.
//
// z=0, w=1
// #..
// ..#
// .#.
//
// z=1, w=1
// #..
// ..#
// .#.
//
//
// After 2 cycles:
//
// z=-2, w=-2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-1, w=-2
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=-2
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=1, w=-2
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=-2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-2, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=-1, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=1, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=-2, w=0
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=-1, w=0
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=0
// .....
// .....
// .....
// .....
// .....
//
// z=1, w=0
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=0
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=-2, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=-1, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=1, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=-2, w=2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-1, w=2
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=2
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=1, w=2
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=2
// .....
// .....
// ..#..
// .....
// .....
// After the full six-cycle boot process completes, 848 cubes are left in the active state.
//
// Starting with your given initial configuration, simulate six cycles in a 4-dimensional space. How many cubes are left in the active state after the sixth cycle?

pub fn part2(lines: &[String]) -> usize {
    let rows: Vec<Vec<bool>> = lines.iter().map(|line| parse_map_line(line)).collect();

    let cycles = 6;

    // Note: The map will grow by 2 (+1 & -1) in each dimension for each cycle.

    let mut pocket_dimension = vec![ // W
        vec![ // Z
              vec![ // Y
                    vec![ // X
                          false;
                          rows[0].len() + 2*cycles
                    ];
                    rows.len() + 2*cycles];
              1 + 2*cycles
        ]; 1 + 2*cycles
    ];

    for (y, row) in rows.iter().enumerate() {
        for (x, cube) in row.iter().enumerate() {
            pocket_dimension[cycles][cycles][y + cycles][x + cycles] = *cube
        }
    }

    for _ in 1..=cycles {
        let previous_pocket_dimension = pocket_dimension.clone();

        // For each cube in the pocket dimension
        for (w, hyperplane) in previous_pocket_dimension.iter().enumerate() {
            for (z, plane) in hyperplane.iter().enumerate() {
                for (y, row) in plane.iter().enumerate() {
                    'main: for (x, cube) in row.iter().enumerate() {
                        let mut active_neightbours = 0;

                        // Check its neighbours
                        let min_w = if w > 0 { w - 1 } else { w };
                        let min_z = if z > 0 { z - 1 } else { z };
                        let min_y = if y > 0 { y - 1 } else { y };
                        let min_x = if x > 0 { x - 1 } else { x };
                        let max_w = if w < previous_pocket_dimension.len() - 1 {
                            w + 1
                        } else {
                            w
                        };
                        let max_z = if z < hyperplane.len() - 1 { z + 1 } else { z };
                        let max_y = if y < plane.len() - 1 { y + 1 } else { y };
                        let max_x = if x < row.len() - 1 { x + 1 } else { x };

                        #[allow(clippy::needless_range_loop)]
                        for target_w in min_w..=max_w {
                            for target_z in min_z..=max_z {
                                for target_y in min_y..=max_y {
                                    for target_x in min_x..=max_x {
                                        if target_x == x
                                            && target_y == y
                                            && target_z == z
                                            && target_w == w
                                        {
                                            continue;
                                        };
                                        let neighbour_cube = previous_pocket_dimension[target_w]
                                            [target_z][target_y][target_x];
                                        if neighbour_cube {
                                            active_neightbours += 1;

                                            // If a cube is active and exactly 2 or 3 of its neighbors are
                                            // also active, the cube remains active.
                                            // Otherwise, the cube becomes inactive.
                                            if *cube && active_neightbours > 3 {
                                                pocket_dimension[w][z][y][x] = false;
                                                continue 'main;
                                            }

                                            // If a cube is inactive but exactly 3 of its neighbors are
                                            // active, the cube becomes active.
                                            // Otherwise, the cube remains inactive.
                                            if !*cube && active_neightbours > 3 {
                                                continue 'main;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // If a cube is active and exactly 2 or 3 of its neighbors are
                        // also active, the cube remains active.
                        // Otherwise, the cube becomes inactive.
                        if *cube && !(active_neightbours == 2 || active_neightbours == 3) {
                            pocket_dimension[w][z][y][x] = false
                        }
                        // If a cube is inactive but exactly 3 of its neighbors are
                        // active, the cube becomes active.
                        // Otherwise, the cube remains inactive.
                        else if !*cube && active_neightbours == 3 {
                            pocket_dimension[w][z][y][x] = true
                        }
                    }
                }
            }
        }

        // println!("State at {}", i);
        // for plane in pocket_dimension.clone() {
        //     println!("State at {:?}", plane);
        // }
    }

    pocket_dimension
        .iter()
        .map(|hyperplane| {
            hyperplane
                .iter()
                .map(|plane| {
                    plane
                        .iter()
                        .map(|row| row.iter().filter(|cube| **cube).count())
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part1() {
        let input_text = ".#.\n..#\n###";
        let input: Vec<String> = input_text.lines().map(|line| line.to_string()).collect();
        assert_eq!(part1(&input), 112);
    }

    #[test]
    pub fn test_part2() {
        let input_text = ".#.\n..#\n###";
        let input: Vec<String> = input_text.lines().map(|line| line.to_string()).collect();
        assert_eq!(part2(&input), 848);
    }
}
