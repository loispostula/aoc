use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_file_to_str("input.txt");
    println!("{}", find_xmas(&data));
    println!("{}", find_x_mas(&data));
}

fn read_file_to_str(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn ind(s: &str, ii: usize, c: char) -> bool {
    if let Some(cc) = s.chars().nth(ii) {
        return cc == c;
    }
    return false;
}

pub fn find_x_mas(input: &str) -> usize {
    let mut result = 0;
    let splitted: Vec<&str> = input.split("\n").collect();
    for i in 0..(splitted.len() - 2) {
        let line = splitted[i];
        for j in 0..(line.len() - 2) {
            for (l, r, b, t) in vec![
                ('M', 'S', 'M', 'S'),
                ('M', 'M', 'S', 'S'),
                ('S', 'S', 'M', 'M'),
                ('S', 'M', 'S', 'M'),
            ] {
                if ind(line, j, l) && ind(line, j + 2, r) {
                    if ind(splitted[i + 1], j + 1, 'A') {
                        if ind(splitted[i + 2], j, b) && ind(splitted[i + 2], j + 2, t) {
                            result += 1;
                        }
                    }
                }
            }
        }
    }

    result
}

pub fn find_xmas(input: &str) -> isize {
    let mut matrix: HashMap<isize, HashMap<isize, char>> = HashMap::new();
    let mut max_heigth: isize = 0;
    let mut max_width: isize = 0;
    for (i, line) in input.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut l: HashMap<isize, char> = HashMap::new();
        for (j, c) in line.chars().enumerate() {
            l.insert(j as isize, c);
        }
        max_width = line.len() as isize;
        matrix.insert(i as isize, l);
        max_heigth += 1;
    }
    let mut result = 0;
    println!("GOT a {}x{} matrix", max_heigth, max_width);

    for i in 0..max_heigth {
        for j in 0..max_width {
            let c = matrix.get(&i).unwrap().get(&j).unwrap().clone();
            if c != 'X' {
                continue;
            }

            for (row_jump, col_jump) in Vec::<(isize, isize)>::from([
                (0, 1),   // Right
                (0, -1),  // Left
                (1, 0),   // Down
                (1, 1),   // Down-Right
                (1, -1),  // Down-Left
                (-1, -1), // Up-Left
                (-1, 0),  // Up
                (-1, 1),  // Up-Right
            ]) {
                let mut new_row = i.clone();
                let mut new_col = j.clone();
                let mut add = true;
                for should_char in vec!['M', 'A', 'S'] {
                    new_col += col_jump;
                    new_row += row_jump;
                    if let Some(ll) = matrix.get(&new_row) {
                        if let Some(cc) = ll.get(&new_col) {
                            if *cc != should_char {
                                add = false;
                            }
                        } else {
                            add = false;
                        }
                    } else {
                        add = false;
                    }
                }

                if add {
                    result += 1;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_DATA: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_finding_xmas() {
        assert_eq!(find_xmas(TEST_DATA), 18);
    }
    #[test]
    fn test_finding_x_mas() {
        assert_eq!(find_x_mas(TEST_DATA), 9);
    }
}
