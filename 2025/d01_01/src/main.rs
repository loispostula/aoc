use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

fn read_file_to_str(path: &str) -> impl Iterator<Item = std::io::Result<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}

fn compute_password<I>(lines: I) -> std::io::Result<usize>
where
    I: Iterator<Item = std::io::Result<String>>,
{
    let mut start: isize = 50;
    let mut count: usize = 0;
    for line in lines {
        let data = line?;
        if data.is_empty() {
            continue;
        }
        let (direction, raw_amount) = data.split_at(1);
        let mult: isize = match direction.to_lowercase().as_ref() {
            "l" => Ok(-1),
            "r" => Ok(1),
            _ => Err(Error::new(ErrorKind::Other, "bad format")),
        }?;
        let amount = (raw_amount.parse::<isize>().unwrap()) * mult;
        start += amount;
        let div = start.div_euclid(100);
        let rem = start - div * (100);
        start = rem.abs();
        if start == 0 {
            count += 1;
        }
    }

    Ok(count)
}

fn main() -> std::io::Result<()> {
    let lines = read_file_to_str("input.txt");
    let password = compute_password(lines)?;

    println!("Code is {}", password);
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;

    static TEST_DATA: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_password_sample() {
        let lines = BufReader::new(Cursor::new(TEST_DATA)).lines();
        let password = compute_password(lines).unwrap();
        assert_eq!(password, 3);
    }
}
