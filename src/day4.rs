// --- Day 4: Passport Processing ---
// You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of
// your passport. While these documents are extremely similar, North Pole Credentials aren't issued
// by a country and therefore aren't actually valid documentation for travel in most of the world.
//
// It seems like you're not the only one having problems, though; a very long line has formed for
// the automatic passport scanners, and the delay could upset your travel itinerary.
//
// Due to some questionable network security, you realize you might be able to solve both of these
// problems at the same time.
//
// The automatic passport scanners are slow because they're having trouble detecting which passports
// have all required fields. The expected fields are as follows:
//
// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)
// Passport data is validated in batch files (your puzzle input). Each passport is represented as a
// sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank
// lines.
//
// Here is an example batch file containing four passports:
//
// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm
//
// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
// hcl:#cfa07d byr:1929
//
// hcl:#ae17e1 iyr:2013
// eyr:2024
// ecl:brn pid:760753108 byr:1931
// hgt:179cm
//
// hcl:#cfa07d eyr:2025 pid:166559648
// iyr:2011 ecl:brn hgt:59in
// The first passport is valid - all eight fields are present. The second passport is invalid - it
// is missing hgt (the Height field).
//
// The third passport is interesting; the only missing field is cid, so it looks like data from
// North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system
// temporarily ignore missing cid fields. Treat this "passport" as valid.
//
// The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any
// other field is not, so this passport is invalid.
//
// According to the above rules, your improved system would report 2 valid passports.
//
// Count the number of valid passports - those that have all required fields. Treat cid as optional.
// In your batch file, how many passports are valid?

pub fn count_passwords_with_all_fields(lines: &[String]) -> i32 {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;

    let mut count = 0;
    let mut iter = lines.iter();
    loop {
        let line = iter.next();
        match line {
            Some(x) if !x.is_empty() => {
                if x.contains("byr") {
                    byr = true
                }
                if x.contains("iyr") {
                    iyr = true
                }
                if x.contains("eyr") {
                    eyr = true
                }
                if x.contains("hgt") {
                    hgt = true
                }
                if x.contains("hcl") {
                    hcl = true
                }
                if x.contains("ecl") {
                    ecl = true
                }
                if x.contains("pid") {
                    pid = true
                }
            }
            _ => {
                if byr && iyr && eyr && hgt && hcl && ecl && pid {
                    count += 1;
                }
                if line.is_none() {
                    break;
                } else {
                    byr = false;
                    iyr = false;
                    eyr = false;
                    hgt = false;
                    hcl = false;
                    ecl = false;
                    pid = false;
                }
            }
        }
    }
    count
}

// --- Part Two ---
// The line is moving more quickly now, but you overhear airport security talking about how
// passports with invalid data are getting through. Better add some data validation, quick!
//
// You can continue to ignore the cid field, but each other field has strict rules about what values
// are valid for automatic validation:
//
// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.
// Your job is to count the passports where all required fields are both present and valid according
// to the above rules. Here are some example values:
//
// byr valid:   2002
// byr invalid: 2003
//
// hgt valid:   60in
// hgt valid:   190cm
// hgt invalid: 190in
// hgt invalid: 190
//
// hcl valid:   #123abc
// hcl invalid: #123abz
// hcl invalid: 123abc
//
// ecl valid:   brn
// ecl invalid: wat
//
// pid valid:   000000001
// pid invalid: 0123456789
// Here are some invalid passports:
//
// eyr:1972 cid:100
// hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
//
// iyr:2019
// hcl:#602927 eyr:1967 hgt:170cm
// ecl:grn pid:012533040 byr:1946
//
// hcl:dab227 iyr:2012
// ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
//
// hgt:59cm ecl:zzz
// eyr:2038 hcl:74454a iyr:2023
// pid:3556412378 byr:2007
// Here are some valid passports:
//
// pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
// hcl:#623a2f
//
// eyr:2029 ecl:blu cid:129 byr:1989
// iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
//
// hcl:#888785
// hgt:164cm byr:2001 iyr:2015 cid:88
// pid:545766238 ecl:hzl
// eyr:2022
//
// iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
// Count the number of valid passports - those that have all required fields and valid values.
// Continue to treat cid as optional. In your batch file, how many passports are valid?
use lazy_static::lazy_static;
use regex::Regex;

/// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn has_valid_byr(line: &str) -> bool {
    lazy_static! {
        static ref BYR_REGEX: Regex = Regex::new(r"byr:(19[2-9]\d|200[012])\b").unwrap();
    }
    BYR_REGEX.is_match(line)
}

/// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn has_valid_iyr(line: &str) -> bool {
    lazy_static! {
        static ref IYR_REGEX: Regex = Regex::new(r"iyr:(201\d|2020)\b").unwrap();
    }
    IYR_REGEX.is_match(line)
}

/// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn has_valid_eyr(line: &str) -> bool {
    lazy_static! {
        static ref EYR_REGEX: Regex = Regex::new(r"eyr:(202\d|2030)\b").unwrap();
    }
    EYR_REGEX.is_match(line)
}

/// hgt (Height) - a number followed by either cm or in:
/// If cm, the number must be at least 150 and at most 193.
/// If in, the number must be at least 59 and at most 76.
fn has_valid_hgt(line: &str) -> bool {
    lazy_static! {
        static ref HGT_REGEX: Regex =
            Regex::new(r"hgt:((?:1[5-8]\d|19[0-3])cm|(?:59|6\d|7[0-6])in)\b").unwrap();
    }
    HGT_REGEX.is_match(line)
}

/// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn has_valid_hcl(line: &str) -> bool {
    lazy_static! {
        static ref HCL_REGEX: Regex = Regex::new(r"hcl:#[a-f0-9]{6}\b").unwrap();
    }
    HCL_REGEX.is_match(line)
}

/// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn has_valid_ecl(line: &str) -> bool {
    lazy_static! {
        static ref ECL_REGEX: Regex = Regex::new(r"ecl:(?:amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    }
    ECL_REGEX.is_match(line)
}

/// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn has_valid_pid(line: &str) -> bool {
    lazy_static! {
        static ref PID_REGEX: Regex = Regex::new(r"pid:\d{9}\b").unwrap();
    }
    PID_REGEX.is_match(line)
}

pub fn count_valid_passwords(lines: &[String]) -> i32 {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;

    let mut count = 0;
    let mut iter = lines.iter();
    loop {
        let maybe_line = iter.next();
        match maybe_line {
            Some(line) if !line.is_empty() => {
                if has_valid_byr(line) {
                    byr = true
                }
                if has_valid_iyr(line) {
                    iyr = true
                }
                if has_valid_eyr(line) {
                    eyr = true
                }
                if has_valid_hgt(line) {
                    hgt = true
                }
                if has_valid_hcl(line) {
                    hcl = true
                }
                if has_valid_ecl(line) {
                    ecl = true
                }
                if has_valid_pid(line) {
                    pid = true
                }
            }
            _ => {
                if byr && iyr && eyr && hgt && hcl && ecl && pid {
                    count += 1;
                }
                if maybe_line.is_none() {
                    break;
                } else {
                    byr = false;
                    iyr = false;
                    eyr = false;
                    hgt = false;
                    hcl = false;
                    ecl = false;
                    pid = false;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_byr_regex() {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        assert_eq!(has_valid_byr(&"byr:1919"), false);
        assert_eq!(has_valid_byr(&"asdr:1920"), false);
        assert_eq!(has_valid_byr(&"byr:1920"), true);
        assert_eq!(has_valid_byr(&"byr:1987"), true);
        assert_eq!(has_valid_byr(&"byr:2002"), true);
        assert_eq!(has_valid_byr(&"byr:2003"), false);
        assert_eq!(has_valid_byr(&"byr:191"), false);
    }

    #[test]
    pub fn test_iyr_regex() {
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        assert_eq!(has_valid_iyr(&"iyr:1987"), false);
        assert_eq!(has_valid_iyr(&"asdr:2010"), false);
        assert_eq!(has_valid_iyr(&"iyr:2010"), true);
        assert_eq!(has_valid_iyr(&"iyr:2015"), true);
        assert_eq!(has_valid_iyr(&"iyr:2020"), true);
        assert_eq!(has_valid_iyr(&"iyr:2021"), false);
        assert_eq!(has_valid_iyr(&"iyr:191"), false);
    }

    #[test]
    pub fn test_hgt_regex() {
        // hgt (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        assert_eq!(has_valid_hgt(&"asdr:150cm"), false);
        assert_eq!(has_valid_hgt(&"hgt:150ccm"), false);
        assert_eq!(has_valid_hgt(&"hgt:149cm"), false);
        assert_eq!(has_valid_hgt(&"hgt:150cm"), true);
        assert_eq!(has_valid_hgt(&"hgt:177cm"), true);
        assert_eq!(has_valid_hgt(&"hgt:193cm"), true);
        assert_eq!(has_valid_hgt(&"hgt:194cm"), false);
        // If in, the number must be at least 59 and at most 76.
        assert_eq!(has_valid_hgt(&"asdr:150in"), false);
        assert_eq!(has_valid_hgt(&"hgt:150cin"), false);
        assert_eq!(has_valid_hgt(&"hgt:58in"), false);
        assert_eq!(has_valid_hgt(&"hgt:59in"), true);
        assert_eq!(has_valid_hgt(&"hgt:63in"), true);
        assert_eq!(has_valid_hgt(&"hgt:76in"), true);
        assert_eq!(has_valid_hgt(&"hgt:77in"), false);
    }

    #[test]
    pub fn test_ecl_regex() {
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        assert_eq!(has_valid_ecl(&"ecl:ads1"), false);
        assert_eq!(has_valid_ecl(&"asdr:amb"), false);
        assert_eq!(has_valid_ecl(&"ecl:amb"), true);
        assert_eq!(has_valid_ecl(&"ecl:blu"), true);
        assert_eq!(has_valid_ecl(&"ecl:brn"), true);
        assert_eq!(has_valid_ecl(&"ecl:gry"), true);
        assert_eq!(has_valid_ecl(&"ecl:grn"), true);
        assert_eq!(has_valid_ecl(&"ecl:hzl"), true);
        assert_eq!(has_valid_ecl(&"ecl:oth"), true);
    }

    #[test]
    pub fn test_pid_regex() {
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        assert_eq!(has_valid_pid(&"pid:ads1"), false);
        assert_eq!(has_valid_pid(&"asdr:amb"), false);
        assert_eq!(has_valid_pid(&"pid:zzzzzzzzz"), false);
        assert_eq!(has_valid_pid(&"pid:900000001"), true);
        assert_eq!(has_valid_pid(&"pid:100000001"), true);
        assert_eq!(has_valid_pid(&"pid:900000001"), true);
        assert_eq!(has_valid_pid(&"pid:000000001"), true);
        assert_eq!(has_valid_pid(&"pid:000000001"), true);
        assert_eq!(has_valid_pid(&"pid:000000001"), true);
        assert_eq!(has_valid_pid(&"pid:0000000001"), false);
    }
}
