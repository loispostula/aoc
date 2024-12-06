use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let data = read_file_to_str("input.txt");
    println!("{:?}", predict(&data));
}

fn read_file_to_str(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn next(&self) -> Self {
        match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
            Direction::Up => write!(f, "^"),
        }
    }
}
fn clear_terminal() {
    // ANSI escape code to clear the terminal and move the cursor to the top-left
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}
fn printit(
    pos: (isize, isize),
    direction: &Direction,
    visited: &HashMap<(isize, isize), bool>,
    obstruction: &HashMap<(isize, isize), ()>,
    m: &Vec<Vec<bool>>,
) {
    clear_terminal();
    for (y, l) in m.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c {
                print!("#");
            } else {
                if (y as isize, x as isize) == pos {
                    print!("{direction}");
                } else if let Some(_) = visited.get(&(y as isize, x as isize)) {
                    print!("X");
                } else if let Some(()) = obstruction.get(&(y as isize, x as isize)) {
                    print!("0");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
    io::stdout().flush().unwrap();
}

fn is_cycle(
    pos: (isize, isize),
    direction: &Direction,
    visited: &HashMap<(isize, isize), bool>,
    m: &Vec<Vec<bool>>,
) -> bool {
    let height = m.len() as isize;
    let width = m[0].len() as isize;
    let initial_x = pos.1.clone();
    let initial_y = pos.0.clone();
    let mut x = pos.1.clone();
    let mut y = pos.0.clone();
    let mut d = direction.clone();
    let mut v: HashMap<(isize, isize), bool> = HashMap::new();
    println!("-");
    while x >= 0 && x < width && y >= 0 && y < height {
        printit((y, x), &d, &v, &HashMap::default(), m);
        sleep(Duration::from_millis(50));
        // if changes > 5 {
        // println!("{changes}"); //
        // return false;
        // }
        let is_obstacle = m[y as usize][x as usize];
        if is_obstacle {
            // println!("{changes}"); //
            // we'll need to go back
            match d {
                Direction::Up => {
                    y += 1;
                }
                Direction::Down => {
                    y -= 1;
                }
                Direction::Left => {
                    x += 1;
                }
                Direction::Right => {
                    x -= 1;
                }
            }

            d = d.clone().next();
        } else {
            v.insert((y, x), true);
            // println!("."); //
            match d {
                Direction::Up => {
                    y -= 1;
                }
                Direction::Down => {
                    y += 1;
                }
                Direction::Left => {
                    x -= 1;
                }
                Direction::Right => {
                    x += 1;
                }
            }
            if x == initial_x && y == initial_y {
                return true;
            }
        }
    }

    return x == initial_x && y == initial_y;
}

fn _predict(
    m: &Vec<Vec<bool>>,
    start_pos: (isize, isize),
    pos: (isize, isize),
    mut direction: Direction,
    visited: &mut HashMap<(isize, isize), bool>,
    height: isize,
    width: isize,
    obstruction: &mut HashMap<(isize, isize), ()>,
    mut just_turned: bool,
    mut init: bool,
) {
    let (mut y, mut x) = pos;
    if x < 0 || x >= width || y < 0 || y >= height {
        // We've move out of the board
        return;
    }

    // We need to get our current position
    let is_obstacle = m[y as usize][x as usize];
    // println!(
    //     "{:?} {} {} {} => {}",
    //     direction, pos.1, pos.0, is_obstacle, visited
    // );
    if is_obstacle {
        // we'll need to go back
        match direction {
            Direction::Up => {
                y += 1;
            }
            Direction::Down => {
                y -= 1;
            }
            Direction::Left => {
                x += 1;
            }
            Direction::Right => {
                x -= 1;
            }
        }
        direction = direction.next();
        just_turned = true;
        init = false;
    } else {
        if !visited.contains_key(&(y, x)) {
            visited.insert((y, x), init);
        }
        // let's put a fake obstacle in front and test cycle
        let cand_obs = match direction {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Right => (y, x + 1),
            Direction::Left => (y, x - 1),
        };
        if cand_obs.1 >= 0
            && cand_obs.1 < width
            && cand_obs.0 >= 0
            && cand_obs.0 < height
            && cand_obs != start_pos
            && !m[cand_obs.0 as usize][cand_obs.1 as usize]
        {
            if !obstruction.contains_key(&cand_obs) {
                let mut cm: Vec<Vec<bool>> = vec![];
                for (i, l) in m.iter().enumerate() {
                    let mut ll = vec![];
                    for (j, o) in l.iter().enumerate() {
                        if i == cand_obs.0 as usize && j == cand_obs.1 as usize {
                            ll.push(true);
                        } else {
                            ll.push(*o);
                        }
                    }
                    cm.push(ll);
                }
                // println!("{} {} {}", pos.1, pos.0, direction);
                // printit(pos, &direction, &visited, &obstruction, &cm);
                if is_cycle(pos, &direction, visited, &cm) {
                    obstruction.insert(cand_obs, ());
                    // printit(pos, &direction, &visited, &obstruction, &cm);
                }
            }
        }
        match direction {
            Direction::Up => {
                y -= 1;
            }
            Direction::Down => {
                y += 1;
            }
            Direction::Left => {
                x -= 1;
            }
            Direction::Right => {
                x += 1;
            }
        }
    }
    return _predict(
        m,
        start_pos,
        (y, x),
        direction,
        visited,
        height,
        width,
        obstruction,
        just_turned,
        init,
    );
}

pub fn predict(input: &str) -> (usize, usize) {
    let (m, pos, direction, width, height, mut visited) = parse_map(input);
    // let mut visited: HashMap<(isize, isize), bool> = HashMap::new();
    let mut obstruction: HashMap<(isize, isize), ()> = HashMap::new();
    printit(pos, &direction, &visited, &obstruction, &m);
    _predict(
        &m,
        pos,
        pos,
        direction,
        &mut visited,
        height,
        width,
        &mut obstruction,
        false,
        true,
    );
    (visited.len(), obstruction.len())
}

fn parse_map(
    input: &str,
) -> (
    Vec<Vec<bool>>,
    (isize, isize),
    Direction,
    isize,
    isize,
    HashMap<(isize, isize), bool>,
) {
    let mut m: Vec<Vec<bool>> = vec![];
    let mut pos: (isize, isize) = (0, 0);
    let mut direction: Direction = Direction::Up;
    let mut width: isize = 0;
    let mut height: isize = 0;
    let mut visited: HashMap<(isize, isize), bool> = HashMap::new();
    for (i, line) in input.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut l = vec![];
        for (j, c) in line.chars().enumerate() {
            if c == '#' || c == 'O' {
                l.push(true);
            } else {
                l.push(false);
                match c {
                    'v' => {
                        direction = Direction::Down;
                        pos = (i as isize, j as isize);
                        visited.insert((i as isize, j as isize), false);
                    }
                    '>' => {
                        direction = Direction::Right;
                        pos = (i as isize, j as isize);
                        visited.insert((i as isize, j as isize), false);
                    }
                    '<' => {
                        direction = Direction::Left;
                        pos = (i as isize, j as isize);
                        visited.insert((i as isize, j as isize), false);
                    }
                    '^' => {
                        direction = Direction::Up;
                        pos = (i as isize, j as isize);
                        visited.insert((i as isize, j as isize), false);
                    }
                    '|' | '-' | '+' => {
                        visited.insert((i as isize, j as isize), false);
                    }
                    _ => {}
                }
            }
        }
        if width == 0 {
            width = l.len() as isize;
        }
        height += 1;
        m.push(l);
    }
    (m, pos, direction, width, height, visited)
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_DATA: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_predict() {
        let (visited, obstruction) = predict(TEST_DATA);
        assert_eq!(visited, 41);
        assert_eq!(obstruction, 6);
        // assert!(false);
    }

    #[test]
    fn test_is_cycle() {
        let input = r#".#...
.>--#
.|#|.
.|#|.
#|-|.
...#."#;
        let (m, pos, direction, _, _, visited) = parse_map(input);
        assert_eq!(is_cycle(pos, &direction, &visited, &m), true);
    }
    #[test]
    fn test_is_cycle_as_well() {
        let input = r#"..#.....
.---.|#.
..^..|..
.#|--|..
.....#.."#;
        let (m, pos, direction, _, _, visited) = parse_map(input);
        assert_eq!(is_cycle(pos, &direction, &visited, &m), true);
    }
    #[test]
    fn test_is_cycle_bis() {
        let input = r#"..#.....
.----|#.
..^..|..
.#|--|..
.....#.."#;
        let (m, pos, direction, _, _, visited) = parse_map(input);
        assert_eq!(is_cycle(pos, &direction, &visited, &m), true);
    }
    #[test]
    fn test_is_cycle_ter() {
        let input = r#"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-+-+-+.
..|...|.#.
#O^---+...
......#..."#;
        let (m, pos, direction, _, _, visited) = parse_map(input);
        let obstruction: HashMap<(isize, isize), ()> = HashMap::new();
        printit(pos, &direction, &visited, &obstruction, &m);
        assert_eq!(is_cycle(pos, &direction, &visited, &m), true);
    }
    #[test]
    fn test_is_cycle_quad() {
        let input = r#"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-|-+-+.
.+---->O#.
#+----+...
......#..."#;
        let (m, pos, direction, _, _, visited) = parse_map(input);
        let obstruction: HashMap<(isize, isize), ()> = HashMap::new();
        printit(pos, &direction, &visited, &obstruction, &m);
        assert_eq!(is_cycle(pos, &direction, &visited, &m), true);
    }
    #[test]
    fn test_is_cycle_five() {
        let input = r#"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-|-+-+.
..|...|.#.
#O<---+...
......#..."#;
        let (m, pos, direction, _, _, visited) = parse_map(input);
        let obstruction: HashMap<(isize, isize), ()> = HashMap::new();
        printit(pos, &direction, &visited, &obstruction, &m);
        assert_eq!(is_cycle(pos, &direction, &visited, &m), true);
    }
    #[test]
    fn test_is_cycle_siz() {
        let input = r#"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-|-+-+.
.+----++#.
#+----+<..
......#O.."#;
        let (m, pos, direction, _, _, visited) = parse_map(input);
        let obstruction: HashMap<(isize, isize), ()> = HashMap::new();
        printit(pos, &direction, &visited, &obstruction, &m);
        assert_eq!(is_cycle(pos, &direction, &visited, &m), true);
    }
    #[test]
    fn test_is_cycle_sev() {
        let input = r#"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-|-+-+.
......|.#.
#..O^-+...
......#..."#;
        let (m, pos, direction, _, _, visited) = parse_map(input);
        let obstruction: HashMap<(isize, isize), ()> = HashMap::new();
        printit(pos, &direction, &visited, &obstruction, &m);
        assert_eq!(is_cycle(pos, &direction, &visited, &m), true);
    }
    #[test]
    fn test_is_cycle_fail() {
        let input = r#".#.
#>#
#|.
#|.
#|.
#..
#.."#;
        let (m, pos, direction, _, _, visited) = parse_map(input);
        let obstruction: HashMap<(isize, isize), ()> = HashMap::new();
        printit(pos, &direction, &visited, &obstruction, &m);
        assert_eq!(is_cycle(pos, &direction, &visited, &m), false);
    }
}
