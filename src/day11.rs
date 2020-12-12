// --- Day 11: Seating System ---
// Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes
// directly to the tropical island where you can finally start your vacation. As you reach the
// waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!
//
// By modeling the process people use to choose (or abandon) their seat in the waiting area, you're
// pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your
// puzzle input).
//
// The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or
// an occupied seat (#). For example, the initial seat layout might look like this:
//
// L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL
// Now, you just need to model the people who will be arriving shortly. Fortunately, people are
// entirely predictable and always follow a simple set of rules. All decisions are based on the
// number of occupied seats adjacent to a given seat (one of the eight positions immediately up,
// down, left, right, or diagonal from the seat). The following rules are applied to every seat
// simultaneously:
//
// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat
// becomes empty.
// Otherwise, the seat's state does not change.
// Floor (.) never changes; seats don't move, and nobody sits on the floor.
//
// After one round of these rules, every seat in the example layout becomes occupied:
//
// #.##.##.##
// #######.##
// #.#.#..#..
// ####.##.##
// #.##.##.##
// #.#####.##
// ..#.#.....
// ##########
// #.######.#
// #.#####.##
// After a second round, the seats with four or more occupied adjacent seats become empty again:
//
// #.LL.L#.##
// #LLLLLL.L#
// L.L.L..L..
// #LLL.LL.L#
// #.LL.LL.LL
// #.LLLL#.##
// ..L.L.....
// #LLLLLLLL#
// #.LLLLLL.L
// #.#LLLL.##
// This process continues for three more rounds:
//
// #.##.L#.##
// #L###LL.L#
// L.#.#..#..
// #L##.##.L#
// #.##.LL.LL
// #.###L#.##
// ..#.#.....
// #L######L#
// #.LL###L.L
// #.#L###.##
// #.#L.L#.##
// #LLL#LL.L#
// L.L.L..#..
// #LLL.##.L#
// #.LL.LL.LL
// #.LL#L#.##
// ..L.L.....
// #L#LLLL#L#
// #.LLLLLL.L
// #.#L#L#.##
// #.#L.L#.##
// #LLL#LL.L#
// L.#.L..#..
// #L##.##.L#
// #.#L.LL.LL
// #.#L#L#.##
// ..L.L.....
// #L#L##L#L#
// #.LLLLLL.L
// #.#L#L#.##
// At this point, something interesting happens: the chaos stabilizes and further applications of
// these rules cause no seats to change state! Once people stop moving around, you count 37 occupied
// seats.
//
// Simulate your seating area by applying the seating rules repeatedly until no seats change state.
// How many seats end up occupied?

use array2d::Array2D;

/// None = flor, false = empty seat, true = occupied seat.
type Seat = Option<bool>;

fn parse_map_charline(char: &char) -> Seat {
    match char {
        '.' => None,
        '#' => Some(true),
        'L' => Some(false),
        _ => panic!("Unexpected char {}", char),
    }
}

fn parse_map(lines: &[String]) -> Array2D<Seat> {
    let rows: Vec<Vec<Seat>> = lines
        .iter()
        .map(|line| line.chars().map(|char| parse_map_charline(&char)).collect())
        .collect();
    Array2D::<Seat>::from_rows(&rows)
}

fn iterate_until_stable(
    map: &Array2D<Seat>,
    search_visible: bool,
    threshold: i32,
) -> Array2D<Seat> {
    #[rustfmt::skip]
    let directions: Vec<(i32, i32)> = vec![
        (-1, 1), (0, 1), (1, 1),
        (-1, 0),         (1, 0),
        (-1,-1), (0,-1), (1,-1),
    ];

    let mut seat_map = map.clone();

    let valid_x = 0..seat_map.num_columns() as i32;
    let valid_y = 0..seat_map.num_rows() as i32;

    loop {
        let mut changed = false;

        let initial_seat_map = seat_map.clone();

        for x in 0..initial_seat_map.num_columns() {
            'outer: for y in 0..initial_seat_map.num_rows() {
                let occupied = if let Some(occ) = initial_seat_map[(y, x)] {
                    occ
                } else {
                    continue;
                };

                let mut occupied_seats = 0;

                for direction in &directions {
                    let mut distance = 1;
                    loop {
                        let target_x = x as i32 + (direction.0 * distance);
                        if !valid_x.contains(&target_x) {
                            break;
                        }

                        let target_y = y as i32 + (direction.1 * distance);
                        if !valid_y.contains(&target_y) {
                            break;
                        }

                        let target_index = (target_y as usize, target_x as usize);

                        let target_seat = initial_seat_map[target_index];

                        if let Some(visible_seat_occupied) = target_seat {
                            if visible_seat_occupied {
                                occupied_seats += 1;

                                if occupied && occupied_seats >= threshold {
                                    seat_map[(y, x)] = Some(false);
                                    changed = true;
                                    continue 'outer;
                                }
                            }
                            break;
                        }

                        if search_visible {
                            distance += 1
                        } else {
                            break;
                        }
                    }
                }

                if !occupied && occupied_seats == 0 {
                    seat_map[(y, x)] = Some(true);
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    seat_map
}

pub fn part1(lines: &[String]) -> usize {
    let map = parse_map(lines);
    let stabilized_map = iterate_until_stable(&map, false, 4);
    stabilized_map
        .elements_column_major_iter()
        .filter(|seat| seat.unwrap_or(false))
        .count()
}

// --- Part Two ---
// As soon as people start to arrive, you realize your mistake. People don't just care about
// adjacent seats - they care about the first seat they can see in each of those eight directions!
//
// Now, instead of considering just the eight immediately adjacent seats, consider the first seat in
// each of those eight directions. For example, the empty seat below would see eight occupied seats:
//
// .......#.
// ...#.....
// .#.......
// .........
// ..#L....#
// ....#....
// .........
// #........
// ...#.....
// The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied
// ones:
//
// .............
// .L.L.#.#.#.#.
// .............
// The empty seat below would see no occupied seats:
//
// .##.##.
// #.#.#.#
// ##...##
// ...L...
// ##...##
// #.#.#.#
// .##.##.
// Also, people seem to be more tolerant than you expected: it now takes five or more visible
// occupied seats for an occupied seat to become empty (rather than four or more from the previous
// rules). The other rules still apply: empty seats that see no occupied seats become occupied,
// seats matching no rule don't change, and floor never changes.
//
// Given the same starting layout as above, these new rules cause the seating area to shift around
// as follows:
//
// L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL
// #.##.##.##
// #######.##
// #.#.#..#..
// ####.##.##
// #.##.##.##
// #.#####.##
// ..#.#.....
// ##########
// #.######.#
// #.#####.##
// #.LL.LL.L#
// #LLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLL#
// #.LLLLLL.L
// #.LLLLL.L#
// #.L#.##.L#
// #L#####.LL
// L.#.#..#..
// ##L#.##.##
// #.##.#L.##
// #.#####.#L
// ..#.#.....
// LLL####LL#
// #.L#####.L
// #.L####.L#
// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##LL.LL.L#
// L.LL.LL.L#
// #.LLLLL.LL
// ..L.L.....
// LLLLLLLLL#
// #.LLLLL#.L
// #.L#LL#.L#
// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##L#.#L.L#
// L.L#.#L.L#
// #.L####.LL
// ..#.#.....
// LLL###LLL#
// #.LLLLL#.L
// #.L#LL#.L#
// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##L#.#L.L#
// L.L#.LL.L#
// #.LLLL#.LL
// ..#.L.....
// LLL###LLL#
// #.LLLLL#.L
// #.L#LL#.L#
// Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once
// this occurs, you count 26 occupied seats.
//
// Given the new visibility method and the rule change for occupied seats becoming empty, once
// equilibrium is reached, how many seats end up occupied?

pub fn part2(lines: &[String]) -> usize {
    let map = parse_map(lines);
    let stabilized_map = iterate_until_stable(&map, true, 5);
    stabilized_map
        .elements_column_major_iter()
        .filter(|seat| seat.unwrap_or(false))
        .count()
}
