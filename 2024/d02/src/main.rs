use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_file_to_str("input.txt");
    let reports = parse_input(&data);
    println!(
        "Not Dampened: {}",
        how_many_levels_safe(reports.clone(), false)
    );
    println!("Dampened: {}", how_many_levels_safe(reports, true));
}

fn read_file_to_str(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn generate_dampened_levels(input: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut result = Vec::new();

    for i in 0..input.len() {
        // Clone the input vector
        let mut temp = input.clone();
        // Remove the element at index `i`
        temp.remove(i);
        // Add the resulting vector to the result
        result.push(temp);
    }

    result
}

pub fn is_level_safe(level: &Vec<isize>, dampened: bool) -> bool {
    if dampened {
        let sub_levels = generate_dampened_levels(level);
        return sub_levels.iter().any(|l| is_level_safe(l, false));
    } else {
        let mut ascending = false;
        let mut initialized = false;
        for w in level.windows(2) {
            if !initialized {
                initialized = true;
                ascending = w[0] < w[1];
            }
            if ascending && w[0] >= w[1] {
                return false;
            }
            if !ascending && w[0] < w[1] {
                return false;
            }
            let a = (w[0] - w[1]).abs();
            if a == 0 || a > 3 {
                // Not safe
                return false;
            }
        }
        true
    }
}

pub fn how_many_levels_safe(reports: Vec<Vec<isize>>, dampened: bool) -> usize {
    reports
        .iter()
        .filter(|x| is_level_safe(*x, dampened))
        .count()
}

pub fn parse_input(input: &str) -> Vec<Vec<isize>> {
    let mut reports: Vec<Vec<isize>> = vec![];
    input.split("\n").for_each(|l| {
        if l.is_empty() {
            return;
        }
        let levels: Vec<isize> = l.split(" ").map(|s| s.parse::<isize>().unwrap()).collect();
        reports.push(levels);
    });
    reports
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_DATA: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn level_safe() {
        assert_eq!(is_level_safe(&vec![7, 6, 4, 2, 1], false), true);
        assert_eq!(is_level_safe(&vec![1, 2, 7, 8, 9], false), false);
        assert_eq!(is_level_safe(&vec![9, 7, 6, 2, 1], false), false);
        assert_eq!(is_level_safe(&vec![1, 3, 2, 4, 5], false), false);
        assert_eq!(is_level_safe(&vec![8, 6, 4, 4, 1], false), false);
        assert_eq!(is_level_safe(&vec![1, 3, 6, 7, 9], false), true);
    }

    #[test]
    fn level_safe_dampened() {
        assert_eq!(is_level_safe(&vec![7, 6, 4, 2, 1], true), true);
        assert_eq!(is_level_safe(&vec![1, 2, 7, 8, 9], true), false);
        assert_eq!(is_level_safe(&vec![9, 7, 6, 2, 1], true), false);
        assert_eq!(is_level_safe(&vec![1, 3, 2, 4, 5], true), true);
        assert_eq!(is_level_safe(&vec![8, 6, 4, 4, 1], true), true);
        assert_eq!(is_level_safe(&vec![1, 3, 6, 7, 9], true), true);
    }

    #[test]
    fn test_count_safe() {
        let reports = parse_input(TEST_DATA);
        assert_eq!(how_many_levels_safe(reports, false), 2);
    }

    #[test]
    fn test_count_safe_dampened() {
        let reports = parse_input(TEST_DATA);
        assert_eq!(how_many_levels_safe(reports, true), 4);
    }

    #[test]
    fn inputs() {
        let reports = parse_input(TEST_DATA);
        assert_eq!(reports.len(), 6);
        assert_eq!(reports[0], vec![7, 6, 4, 2, 1]);
        assert_eq!(reports[1], vec![1, 2, 7, 8, 9]);
        assert_eq!(reports[2], vec![9, 7, 6, 2, 1]);
        assert_eq!(reports[3], vec![1, 3, 2, 4, 5]);
        assert_eq!(reports[4], vec![8, 6, 4, 4, 1]);
        assert_eq!(reports[5], vec![1, 3, 6, 7, 9]);
    }
}
