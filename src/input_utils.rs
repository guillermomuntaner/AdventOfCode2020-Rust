use std::str::FromStr;

pub fn read_all<T: FromStr>(file_name: &str) -> Vec<T> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| match x.parse::<T>() {
            Ok(n) => n,
            Err(_) => panic!("Failed to parse"),
        })
        .collect()
}