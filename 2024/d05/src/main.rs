use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_file_to_str("input.txt");
    println!("{}", printing(&data, false));
    println!("{}", printing(&data, true));
}

fn read_file_to_str(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn printing(input: &str, fix: bool) -> usize {
    let mut rules: Vec<(usize, usize)> = vec![];
    let mut updates: Vec<Vec<usize>> = vec![];
    let mut is_rule = true;
    for line in input.split('\n') {
        if is_rule {
            if line.is_empty() {
                is_rule = false;
                continue;
            }
            let split = line.find('|').unwrap();
            let rl = &line[0..split];
            let rr = &line[split + 1..];
            rules.push((rl.parse().unwrap(), rr.parse().unwrap()))
        } else {
            if line.is_empty() {
                continue;
            }
            updates.push(line.split(',').map(|s| s.parse().unwrap()).collect());
        }
    }

    updates
        .iter()
        .filter(|u| {
            let valid = rules.iter().all(|(l, r)| {
                let li = u.iter().position(|li| li == l);
                let ri = u.iter().position(|ri| ri == r);
                match (li, ri) {
                    (Some(lii), Some(rii)) => lii < rii,
                    _ => true,
                }
            });
            if fix {
                return !valid;
            } else {
                return valid;
            }
        })
        .map(|u| {
            let mut tu = u.clone();
            if fix {
                // we need to sort the vec
                tu.sort_by(|a, b| sort_s(a.clone(), b.clone(), rules.clone()));
            }
            // take middle
            let middle = tu.len() / 2;
            tu[middle]
        })
        .fold(0, |acc, x| acc + x)
}

fn sort_s(a: usize, b: usize, rules: Vec<(usize, usize)>) -> std::cmp::Ordering {
    for (l, r) in rules {
        if (a == l) && b == r {
            return std::cmp::Ordering::Less;
        }
        if (a == r) && b == l {
            return std::cmp::Ordering::Greater;
        }
    }
    return std::cmp::Ordering::Equal;
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_DATA: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_printing() {
        assert_eq!(printing(TEST_DATA, false), 143);
    }

    #[test]
    fn test_printing_fix() {
        assert_eq!(printing(TEST_DATA, true), 123);
    }
}
