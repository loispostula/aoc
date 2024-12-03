use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_file_to_str("input.txt");
    println!("{}", multiply(&data))
}

fn read_file_to_str(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn multiply(input: &str) -> isize {
    let mut result = 0;
    let mut add = true;
    let re: Regex = Regex::new(r"(mul\(([0-9]*),([0-9]*)\)|don't|do)").unwrap();
    for cap in re.captures_iter(input) {
        let token = cap.get(1).unwrap().as_str();
        if token.starts_with("mul(") {
            if add {
                if let Ok(left) = cap.get(2).unwrap().as_str().parse::<isize>() {
                    if let Ok(right) = cap.get(3).unwrap().as_str().parse::<isize>() {
                        result += left * right
                    }
                }
            }
        } else if token == "do" {
            add = true;
        } else if token == "don't" {
            add = false
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_DATA: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(TEST_DATA), 48);
    }
}
