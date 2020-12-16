use std::collections::HashMap;

pub fn play(numbers: &[u64], target_position: usize) -> u64 {

    let mut number = *numbers.last().unwrap();
    let mut prior_position = numbers.len()-1;
    let mut timestamps: HashMap<u64, usize> = HashMap::new();

    for position in 0..(numbers.len()-1) {
        timestamps.insert(numbers[position], position);
    }

    for position in numbers.len()..target_position {
        let last_number = number;
        let last_position = prior_position;
        number = match timestamps.get(&last_number) {
            None => 0,
            Some(prior_position) => (last_position - prior_position) as u64,
        };
        timestamps.insert(last_number, last_position);
        prior_position = position;
    }

    number
}

pub fn part1() -> u64 {
    play(&vec![6,4,12,1,20,0,16], 2020)
}

// --- Part Two ---
// Impressed, the Elves issue you a challenge: determine the 30000000th number spoken. For example, given the same starting numbers as above:
//
// Given 0,3,6, the 30000000th number spoken is 175594.
// Given 1,3,2, the 30000000th number spoken is 2578.
// Given 2,1,3, the 30000000th number spoken is 3544142.
// Given 1,2,3, the 30000000th number spoken is 261214.
// Given 2,3,1, the 30000000th number spoken is 6895259.
// Given 3,2,1, the 30000000th number spoken is 18.
// Given 3,1,2, the 30000000th number spoken is 362.
// Given your starting numbers, what will be the 30000000th number spoken?

pub fn part2() -> u64 {
    play(&vec![6,4,12,1,20,0,16], 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_game() {
        assert_eq!(play(&vec![0,3,6], 4), 0);
        assert_eq!(play(&vec![0,3,6], 5), 3);
        assert_eq!(play(&vec![0,3,6], 6), 3);
        assert_eq!(play(&vec![0,3,6], 7), 1);
        assert_eq!(play(&vec![0,3,6], 8), 0);
        assert_eq!(play(&vec![0,3,6], 9), 4);
        assert_eq!(play(&vec![0,3,6], 10), 0);

        // Given the starting numbers 1,3,2, the 2020th number spoken is 1.
        assert_eq!(play(&vec![1,3,2], 2020), 1);
        // Given the starting numbers 2,1,3, the 2020th number spoken is 10.
        assert_eq!(play(&vec![2,1,3], 2020), 10);
        // Given the starting numbers 1,2,3, the 2020th number spoken is 27.
        assert_eq!(play(&vec![1,2,3], 2020), 27);
        // Given the starting numbers 2,3,1, the 2020th number spoken is 78.
        assert_eq!(play(&vec![2,3,1], 2020), 78);
        // Given the starting numbers 3,2,1, the 2020th number spoken is 438.
        assert_eq!(play(&vec![3,2,1], 2020), 438);
        // Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
        assert_eq!(play(&vec![3,1,2], 2020), 1836);
    }
}