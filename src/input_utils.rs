use std::str::FromStr;

pub fn read_all(file_name: &str) -> Vec<String> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn read_all_as<T: FromStr>(file_name: &str) -> Vec<T> {
    read_all(file_name)
        .iter()
        .map(|x| match x.parse::<T>() {
            Ok(n) => n,
            Err(_) => panic!("Failed to parse"),
        })
        .collect()
}