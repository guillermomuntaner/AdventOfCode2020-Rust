// --- Day 3: Toboggan Trajectory ---
// With the toboggan login problems resolved, you set off toward the airport. While travel by
// toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is
// covered in trees. You'll need to see which angles will take you near the fewest trees.
//
// Due to the local geology, trees in this area only grow on exact integer coordinates in a grid.
// You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For
// example:
//
// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#
// These aren't the only trees, though; due to something you read about once involving arboreal
// genetics and biome stability, the same pattern repeats to the right many times:
//
// ..##.........##.........##.........##.........##.........##.......  --->
// #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
// .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
// ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
// .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
// ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
// .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
// .#........#.#........#.#........#.#........#.#........#.#........#
// #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
// #...##....##...##....##...##....##...##....##...##....##...##....#
// .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
// You start on the open square (.) in the top-left corner and need to reach the bottom (below the
// bottom-most row on your map).
//
// The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers
// rational numbers); start by counting all the trees you would encounter for the slope right 3,
// down 1:
//
// From your starting position at the top-left, check the position that is right 3 and down 1. Then,
// check the position that is right 3 and down 1 from there, and so on until you go past the bottom
// of the map.
//
// The locations you'd check in the above example are marked here with O where there was an open
// square and X where there was a tree:
//
// ..##.........##.........##.........##.........##.........##.......  --->
// #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
// .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
// ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
// .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
// ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
// .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
// .#........#.#........X.#........#.#........#.#........#.#........#
// #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
// #...##....##...##....##...#X....##...##....##...##....##...##....#
// .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
// In this example, traversing the map using this slope would cause you to encounter 7 trees.
//
// Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many
// trees would you encounter?

use array2d::Array2D;

fn parse_map_line(line: &str) -> Vec<bool> {
    line.chars().map(|char| match char {
        '.' => false,
        '#' => true,
        _ => panic!("Unexpected char {}", char)
    }).collect()
}

pub fn calculate_cut_trees_for_toboggan(lines: &Vec<String>) -> i32 {
    let rows: Vec<Vec<bool>> = lines.iter().map(|line| parse_map_line(line)).collect();
    let tree_map = Array2D::from_rows(&rows);
    let width = tree_map.num_columns();
    let height = tree_map.num_rows();
    let mut x = 0;
    let mut y = 0;
    let mut count_trees = 0;
    while y < height {
        if tree_map[(y,x)] {
            count_trees += 1
        }
        y += 1;
        x = (x + 3) % width;
    }
    return count_trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] pub fn test_remainder_operator() {
        assert_eq!(2 % 5, 2);
        assert_eq!(7 % 5, 2);
    }

    #[test] pub fn test_parse_map_line() {
        let line = parse_map_line(&"..##.......");
        assert_eq!(line, vec![false, false, true, true, false, false, false, false, false, false, false]);
    }

}