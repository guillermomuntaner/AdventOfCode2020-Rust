// --- Day 20: Jurassic Jigsaw ---
// The high-speed train leaves the forest and quickly carries you south. You can even see a desert
// in the distance! Since you have some spare time, you might as well see if there was anything
// interesting in the image the Mythical Information Bureau satellite captured.
//
// After decoding the satellite messages, you discover that the data actually contains many small
// images created by the satellite's camera array. The camera array consists of many cameras; rather
// than produce a single square image, they produce many smaller square image tiles that need to be
// reassembled back into a single image.
//
// Each camera in the camera array returns a single monochrome image tile with a random unique ID
// number. The tiles (your puzzle input) arrived in a random order.
//
// Worse yet, the camera array appears to be malfunctioning: each image tile has been rotated and
// flipped to a random orientation. Your first task is to reassemble the original image by orienting
// the tiles so they fit together.
//
// To show how the tiles should be reassembled, each tile's image data includes a border that should
// line up exactly with its adjacent tiles. All tiles have this border, and the border lines up
// exactly when the tiles are both oriented correctly. Tiles at the edge of the image also have this
// border, but the outermost edges won't line up with any other tiles.
//
// For example, suppose you have the following nine tiles:
//
// Tile 2311:
// ..##.#..#.
// ##..#.....
// #...##..#.
// ####.#...#
// ##.##.###.
// ##...#.###
// .#.#.#..##
// ..#....#..
// ###...#.#.
// ..###..###
//
// Tile 1951:
// #.##...##.
// #.####...#
// .....#..##
// #...######
// .##.#....#
// .###.#####
// ###.##.##.
// .###....#.
// ..#.#..#.#
// #...##.#..
//
// Tile 1171:
// ####...##.
// #..##.#..#
// ##.#..#.#.
// .###.####.
// ..###.####
// .##....##.
// .#...####.
// #.##.####.
// ####..#...
// .....##...
//
// Tile 1427:
// ###.##.#..
// .#..#.##..
// .#.##.#..#
// #.#.#.##.#
// ....#...##
// ...##..##.
// ...#.#####
// .#.####.#.
// ..#..###.#
// ..##.#..#.
//
// Tile 1489:
// ##.#.#....
// ..##...#..
// .##..##...
// ..#...#...
// #####...#.
// #..#.#.#.#
// ...#.#.#..
// ##.#...##.
// ..##.##.##
// ###.##.#..
//
// Tile 2473:
// #....####.
// #..#.##...
// #.##..#...
// ######.#.#
// .#...#.#.#
// .#########
// .###.#..#.
// ########.#
// ##...##.#.
// ..###.#.#.
//
// Tile 2971:
// ..#.#....#
// #...###...
// #.#.###...
// ##.##..#..
// .#####..##
// .#..####.#
// #..#.#..#.
// ..####.###
// ..#.#.###.
// ...#.#.#.#
//
// Tile 2729:
// ...#.#.#.#
// ####.#....
// ..#.#.....
// ....#..#.#
// .##..##.#.
// .#.####...
// ####.#.#..
// ##.####...
// ##..#.##..
// #.##...##.
//
// Tile 3079:
// #.#.#####.
// .#..######
// ..#.......
// ######....
// ####.#..#.
// .#...#.##.
// #.#####.##
// ..#.###...
// ..#.......
// ..#.###...
// By rotating, flipping, and rearranging them, you can find a square arrangement that causes all
// adjacent borders to line up:
//
// #...##.#.. ..###..### #.#.#####.
// ..#.#..#.# ###...#.#. .#..######
// .###....#. ..#....#.. ..#.......
// ###.##.##. .#.#.#..## ######....
// .###.##### ##...#.### ####.#..#.
// .##.#....# ##.##.###. .#...#.##.
// #...###### ####.#...# #.#####.##
// .....#..## #...##..#. ..#.###...
// #.####...# ##..#..... ..#.......
// #.##...##. ..##.#..#. ..#.###...
//
// #.##...##. ..##.#..#. ..#.###...
// ##..#.##.. ..#..###.# ##.##....#
// ##.####... .#.####.#. ..#.###..#
// ####.#.#.. ...#.##### ###.#..###
// .#.####... ...##..##. .######.##
// .##..##.#. ....#...## #.#.#.#...
// ....#..#.# #.#.#.##.# #.###.###.
// ..#.#..... .#.##.#..# #.###.##..
// ####.#.... .#..#.##.. .######...
// ...#.#.#.# ###.##.#.. .##...####
//
// ...#.#.#.# ###.##.#.. .##...####
// ..#.#.###. ..##.##.## #..#.##..#
// ..####.### ##.#...##. .#.#..#.##
// #..#.#..#. ...#.#.#.. .####.###.
// .#..####.# #..#.#.#.# ####.###..
// .#####..## #####...#. .##....##.
// ##.##..#.. ..#...#... .####...#.
// #.#.###... .##..##... .####.##.#
// #...###... ..##...#.. ...#..####
// ..#.#....# ##.#.#.... ...##.....
// For reference, the IDs of the above tiles are:
//
// 1951    2311    3079
// 2729    1427    2473
// 2971    1489    1171
// To check that you've assembled the image correctly, multiply the IDs of the four corner tiles
// together. If you do this with the assembled tiles from the example above, you get
// 1951 * 3079 * 2971 * 1171 = 20899048083289.
//
// Assemble the tiles into an image. What do you get if you multiply together the IDs of the four
// corner tiles?

use array2d::Array2D;
use std::collections::HashSet;

type Image = Array2D<bool>;

#[derive(PartialEq, Debug, Clone)]
struct Tile {
    uid: u32,
    image: Image,
}

#[derive(PartialEq, Debug, Clone)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

fn rotate(image: &Image) -> Image {
    let num_rows = image.num_columns();
    let num_columns = image.num_rows();
    let mut rotated = Array2D::filled_with(false, num_rows, num_columns);
    for x in 0..num_columns {
        for y in 0..num_rows {
            rotated[(y, x)] = image[(num_columns - 1 - x, y)];
        }
    }
    rotated
}

fn flip_around_vertical_axis(image: &Image) -> Image {
    let num_rows = image.num_rows();
    let num_columns = image.num_columns();
    let mut rotated = Array2D::filled_with(false, num_rows, num_columns);
    for x in 0..num_columns {
        for y in 0..num_rows {
            rotated[(y, x)] = image[(y, num_columns - 1 - x)]
        }
    }
    rotated
}

fn permutations(image: &Image) -> Vec<Image> {
    let rot_0 = image.clone();
    let rot_90 = rotate(&rot_0);
    let rot_180 = rotate(&rot_90);
    let rot_270 = rotate(&rot_180);
    let flipped_rot_0 = flip_around_vertical_axis(&rot_0);
    let flipped_rot_90 = rotate(&flipped_rot_0);
    let flipped_rot_180 = rotate(&flipped_rot_90);
    let flipped_rot_270 = rotate(&flipped_rot_180);
    vec![
        rot_0,
        rot_90,
        rot_180,
        rot_270,
        flipped_rot_0,
        flipped_rot_90,
        flipped_rot_180,
        flipped_rot_270,
    ]
}

fn do_match(a: &Image, edge: &Edge, b: &Image) -> bool {
    match edge {
        Edge::Top => a.as_rows().first().unwrap() == b.as_rows().last().unwrap(),
        Edge::Bottom => a.as_rows().last().unwrap() == b.as_rows().first().unwrap(),
        Edge::Left => a.as_columns().first().unwrap() == b.as_columns().last().unwrap(),
        Edge::Right => a.as_columns().last().unwrap() == b.as_columns().first().unwrap(),
    }
}

fn find_match_for_edge(fixed: &Tile, matching: &Tile, edge: &Edge) -> Option<Image> {
    for permutation in &permutations(&matching.image) {
        if do_match(&fixed.image, &edge, &permutation) {
            return Some(permutation.clone());
        }
    }
    None
}

fn find_match(fixed: &Tile, matching: &Tile) -> Option<(Edge, Image)> {
    let permutations = permutations(&matching.image);
    for edge in vec![Edge::Top, Edge::Right, Edge::Bottom, Edge::Left] {
        for permutation in &permutations {
            if do_match(&fixed.image, &edge, &permutation) {
                return Some((edge, permutation.clone()));
            }
        }
    }
    None
}

/// The uid with format "Tile 2503:"
fn parse_id(line: &str) -> u32 {
    line.chars()
        .skip(5)
        .filter_map(|char| char.to_digit(10))
        .fold(0, |acc, digit| acc * 10 + digit)
}

fn parse_map_line(line: &str) -> Vec<bool> {
    line.chars()
        .map(|char| match char {
            '.' => false,
            '#' => true,
            _ => panic!("Unexpected char {}", char),
        })
        .collect()
}

pub fn part1(lines: &[String]) -> u64 {
    let tiles: Vec<Tile> = lines
        .split(|line| line == "")
        .map(|tile_info| {
            let mut iter = tile_info.iter();
            // The uid with format "Tile 2503:"
            let id = parse_id(iter.next().unwrap());
            // The piece data itself
            let rows: Vec<_> = iter.map(|line| parse_map_line(line)).collect();
            Tile {
                uid: id,
                image: Array2D::from_rows(&rows),
            }
        })
        .collect();

    let mut edge_tiles: Vec<Tile> = Vec::new();

    for tile in &tiles {
        let mut matched_borders = 0;
        for matching_tile in &tiles.clone() {
            if tile.uid == matching_tile.uid {
                continue;
            }
            let found_match = find_match(&tile, &matching_tile);
            if found_match.is_some() {
                matched_borders += 1;
            }
        }
        match matched_borders {
            2 => edge_tiles.push(tile.clone()),
            3 => {}
            4 => {}
            _ => panic!("Unexpected tile with just 1 matched border"),
        }
    }

    //let mut puzzle: Vec<Vec<u32>> = vec![vec![0; side]; side];
    //
    //puzzle[0][0] = tiles[0].0;
    //
    //for y in 0..side {
    //    for x in 0..side {
    //
    //    }
    //}

    assert_eq!(edge_tiles.len(), 4);

    edge_tiles.iter().fold(1, |acc, tile| acc * tile.uid as u64)
}

// --- Part Two ---
// Now, you're ready to check the image for sea monsters.
//
// The borders of each tile are not part of the actual image; start by removing them.
//
// In the example above, the tiles become:
//
// .#.#..#. ##...#.# #..#####
// ###....# .#....#. .#......
// ##.##.## #.#.#..# #####...
// ###.#### #...#.## ###.#..#
// ##.#.... #.##.### #...#.##
// ...##### ###.#... .#####.#
// ....#..# ...##..# .#.###..
// .####... #..#.... .#......
//
// #..#.##. .#..###. #.##....
// #.####.. #.####.# .#.###..
// ###.#.#. ..#.#### ##.#..##
// #.####.. ..##..## ######.#
// ##..##.# ...#...# .#.#.#..
// ...#..#. .#.#.##. .###.###
// .#.#.... #.##.#.. .###.##.
// ###.#... #..#.##. ######..
//
// .#.#.### .##.##.# ..#.##..
// .####.## #.#...## #.#..#.#
// ..#.#..# ..#.#.#. ####.###
// #..####. ..#.#.#. ###.###.
// #####..# ####...# ##....##
// #.##..#. .#...#.. ####...#
// .#.###.. ##..##.. ####.##.
// ...###.. .##...#. ..#..###
// Remove the gaps to form the actual image:
//
// .#.#..#.##...#.##..#####
// ###....#.#....#..#......
// ##.##.###.#.#..######...
// ###.#####...#.#####.#..#
// ##.#....#.##.####...#.##
// ...########.#....#####.#
// ....#..#...##..#.#.###..
// .####...#..#.....#......
// #..#.##..#..###.#.##....
// #.####..#.####.#.#.###..
// ###.#.#...#.######.#..##
// #.####....##..########.#
// ##..##.#...#...#.#.#.#..
// ...#..#..#.#.##..###.###
// .#.#....#.##.#...###.##.
// ###.#...#..#.##.######..
// .#.#.###.##.##.#..#.##..
// .####.###.#...###.#..#.#
// ..#.#..#..#.#.#.####.###
// #..####...#.#.#.###.###.
// #####..#####...###....##
// #.##..#..#...#..####...#
// .#.###..##..##..####.##.
// ...###...##...#...#..###
// Now, you're ready to search for sea monsters! Because your image is monochrome, a sea monster
// will look like this:
//
//                   #
// #    ##    ##    ###
//  #  #  #  #  #  #
//
// When looking for this pattern in the image, the spaces can be anything; only the # need to match.
// Also, you might need to rotate or flip your image before it's oriented correctly to find sea
// monsters. In the above image, after flipping and rotating it to the appropriate orientation,
// there are two sea monsters (marked with O):
//
// .####...#####..#...###..
// #####..#..#.#.####..#.#.
// .#.#...#.###...#.##.O#..
// #.O.##.OO#.#.OO.##.OOO##
// ..#O.#O#.O##O..O.#O##.##
// ...#.#..##.##...#..#..##
// #.##.#..#.#..#..##.#.#..
// .###.##.....#...###.#...
// #.####.#.#....##.#..#.#.
// ##...#..#....#..#...####
// ..#.##...###..#.#####..#
// ....#.##.#.#####....#...
// ..##.##.###.....#.##..#.
// #...#...###..####....##.
// .#.##...#.##.#.#.###...#
// #.###.#..####...##..#...
// #.###...#.##...#.##O###.
// .O##.#OO.###OO##..OOO##.
// ..O#.O..O..O.#O##O##.###
// #.#..##.########..#..##.
// #.#####..#.#...##..#....
// #....##..#.#########..##
// #...#.....#..##...###.##
// #..###....##.#...##.##.#
// Determine how rough the waters are in the sea monsters' habitat by counting the number of # that
// are not part of a sea monster. In the above example, the habitat's water roughness is 273.
//
// How many # are not part of a sea monster?

pub fn part2(lines: &[String]) -> usize {
    let mut tiles: Vec<Tile> = lines
        .split(|line| line == "")
        .map(|tile_info| {
            let mut iter = tile_info.iter();
            // The uid with format "Tile 2503:"
            let id = parse_id(iter.next().unwrap());
            // The piece data itself
            let rows: Vec<_> = iter.map(|line| parse_map_line(line)).collect();
            Tile {
                uid: id,
                image: Array2D::from_rows(&rows),
            }
        })
        .collect();

    let side = (tiles.len() as f64).sqrt() as usize;
    let mut puzzle: Vec<Vec<Option<Tile>>> = vec![vec![None; side]; side];

    puzzle[0][0] = Some(tiles.remove(0));

    // 1st piece place on top-left corner.

    // From there start matching towards left side until no more pieces are found.
    let mut end_x_pos = side;
    'left_to_right_filling: for x in 0..side {
        if puzzle[0][x].is_none() {
            let tile = puzzle[0][x - 1].clone().unwrap();
            for (pos, matching_tile) in tiles.iter().enumerate() {
                if let Some(matching_permutation) =
                    find_match_for_edge(&tile, matching_tile, &Edge::Right)
                {
                    puzzle[0][x] = Some(Tile {
                        uid: matching_tile.uid,
                        image: matching_permutation,
                    });
                    tiles.remove(pos);
                    continue 'left_to_right_filling;
                }
            }
            // No tile on the right found, stop searching in this direction.
            end_x_pos = x;
            break 'left_to_right_filling;
        }
    }

    // No tile was found, means there is nothing towards the right side.
    // Shift the whole puzzle towards the right end and search from the left.
    let width = puzzle[0].len();
    let snapshot = puzzle.clone();
    for x in 0..width {
        puzzle[0][(x + width - end_x_pos) % width] = snapshot[0][x].clone();
    }

    // Now search in the other direction
    'right_to_left_filling: for x in (0..side).rev() {
        if puzzle[0][x].is_none() {
            let tile = puzzle[0][x + 1].clone().unwrap();
            for (pos, matching_tile) in tiles.iter().enumerate() {
                if let Some(matching_permutation) =
                    find_match_for_edge(&tile, matching_tile, &Edge::Left)
                {
                    puzzle[0][x] = Some(Tile {
                        uid: matching_tile.uid,
                        image: matching_permutation,
                    });
                    tiles.remove(pos);
                    continue 'right_to_left_filling;
                }
            }
            panic!("Couldn't fill all tiles after the bi directional scan");
        }
    }

    // From there start matching towards bottom side until no more pieces are found.
    let mut end_y_pos = side;
    'top_to_bottom_filling: for y in 0..side {
        if puzzle[y][0].is_none() {
            let tile = puzzle[y - 1][0].clone().unwrap();
            for (pos, matching_tile) in tiles.iter().enumerate() {
                if let Some(matching_permutation) =
                    find_match_for_edge(&tile, matching_tile, &Edge::Bottom)
                {
                    puzzle[y][0] = Some(Tile {
                        uid: matching_tile.uid,
                        image: matching_permutation,
                    });
                    tiles.remove(pos);
                    // Fill the whole row
                    for x in 1..side {
                        let tile = puzzle[y][x - 1].clone().unwrap();
                        for (pos, matching_tile) in tiles.iter().enumerate() {
                            if let Some(matching_permutation) =
                                find_match_for_edge(&tile, matching_tile, &Edge::Right)
                            {
                                puzzle[y][x] = Some(Tile {
                                    uid: matching_tile.uid,
                                    image: matching_permutation,
                                });
                                tiles.remove(pos);
                                break;
                            }
                        }
                    }
                    continue 'top_to_bottom_filling;
                }
            }
            // No tile on the bottom found, stop searching in this direction.
            end_y_pos = y;
            break 'top_to_bottom_filling;
        }
    }

    // No tile was found, means there is nothing towards the right side.
    // Shift the whole puzzle towards the right end and search from the left.
    let height = puzzle.len();
    let snapshot = puzzle.clone();
    for y in 0..height {
        for x in 0..width {
            puzzle[(y + height - end_y_pos) % height][x] = snapshot[y][x].clone();
        }
    }

    // Now search in the other direction
    'bottom_to_top_filling: for y in (0..side).rev() {
        if puzzle[y][0].is_none() {
            let tile = puzzle[y + 1][0].clone().unwrap();
            for (pos, matching_tile) in tiles.iter().enumerate() {
                if let Some(matching_permutation) =
                    find_match_for_edge(&tile, matching_tile, &Edge::Top)
                {
                    puzzle[y][0] = Some(Tile {
                        uid: matching_tile.uid,
                        image: matching_permutation,
                    });
                    tiles.remove(pos);
                    // Fill the whole row
                    for x in 1..side {
                        let tile = puzzle[y][x - 1].clone().unwrap();
                        for (pos, matching_tile) in tiles.iter().enumerate() {
                            if let Some(matching_permutation) =
                                find_match_for_edge(&tile, matching_tile, &Edge::Right)
                            {
                                puzzle[y][x] = Some(Tile {
                                    uid: matching_tile.uid,
                                    image: matching_permutation,
                                });
                                tiles.remove(pos);
                                break;
                            }
                        }
                    }
                    continue 'bottom_to_top_filling;
                }
            }
            panic!("Couldn't fill all tiles after the bi directional scan");
        }
    }

    // Stitch the puzzle into a single image
    // Discard the edges of each tile before combining them. Each tile becomes 2 columns & 2 rows smaller.
    let sample_tile = &puzzle
        .first()
        .unwrap()
        .first()
        .unwrap()
        .as_ref()
        .unwrap()
        .image;
    let cut_tile_side = sample_tile.num_columns() - 2;
    let combined_side = side * cut_tile_side;
    let mut image: Image = Array2D::filled_with(false, combined_side, combined_side);
    // For every tile
    for y in 0..side {
        for x in 0..side {
            // Copy every point, ignoring the border
            let tile_image = &puzzle[y][x].as_ref().unwrap().image;
            for tile_y in 1..tile_image.num_rows() - 1 {
                for tile_x in 1..tile_image.num_columns() - 1 {
                    image[(
                        y * cut_tile_side + tile_y - 1,
                        x * cut_tile_side + tile_x - 1,
                    )] = tile_image[(tile_y, tile_x)]
                }
            }
        }
    }

    // Now search for the sea monsters. They look like this:
    //                   #
    // #    ##    ##    ###
    //  #  #  #  #  #  #
    let sea_monster_image: Image = Array2D::from_rows(&[
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false,
        ],
        vec![
            true, false, false, false, false, true, true, false, false, false, false, true, true,
            false, false, false, false, true, true, true,
        ],
        vec![
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            true, false, false, true, false, false, false,
        ],
    ]);

    let mut sea_monster_coordinates: HashSet<(usize, usize)> = HashSet::new();

    for sea_monster_permutation in permutations(&sea_monster_image) {
        // Place the sea monster image in all possible positions over the image
        for y in 0..=image.num_rows() - sea_monster_permutation.num_rows() {
            'monster_scan: for x in 0..=image.num_columns() - sea_monster_permutation.num_columns()
            {
                // If all true values of the sea monster are also true in the image, its a match
                for monster_y in 0..sea_monster_permutation.num_rows() {
                    for monster_x in 0..sea_monster_permutation.num_columns() {
                        if !sea_monster_permutation[(monster_y, monster_x)] {
                            continue;
                        };
                        // If there is monster & image doesn't have it, its not a match.
                        // Continue searching in next position.
                        if !image[(y + monster_y, x + monster_x)] {
                            continue 'monster_scan;
                        }
                    }
                }
                // Reached this point, its a match!. Store all monster points in the list.
                for monster_y in 0..sea_monster_permutation.num_rows() {
                    for monster_x in 0..sea_monster_permutation.num_columns() {
                        if sea_monster_permutation[(monster_y, monster_x)] {
                            sea_monster_coordinates.insert((y + monster_y, x + monster_x));
                        }
                    }
                }
            }
        }
    }

    let total_hashes = image
        .elements_column_major_iter()
        .filter(|point| **point)
        .count();
    let monster_hashes = sea_monster_coordinates.len();
    let water_roughness = total_hashes - monster_hashes;

    println!(
        "total_hashes: {}, monster_hashes: {}, water_roughness: {}",
        total_hashes, monster_hashes, water_roughness
    );
    water_roughness
}
