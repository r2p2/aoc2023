use std::collections::HashSet;

// attention, hides newline chars
struct Map<'a> {
    data: &'a str,
    idx_nl: usize,
}
impl<'a> Map<'a> {
    fn from(data: &'a str) -> Map<'a> {
        Map {
            data,
            idx_nl: data.find('\n').unwrap(),
        }
    }

    fn width(&self) -> usize {
        self.idx_nl
    }
    fn height(&self) -> usize {
        self.data.len() / (self.idx_nl + 1)
    }
    fn get(&self, x: usize, y: usize) -> char {
        self.data.bytes().nth(y * (self.width() + 1) + x).unwrap() as char // FIXME(RPE): nth is slow
    }
}

struct Part {
    c: char,
    x: usize,
    y: usize,
}
impl Part {
    fn borders(&self) -> Vec<(usize, usize)> {
        let mut cells: Vec<(usize, usize)> = Vec::new();
        if self.x > 0 && self.y > 0 {
            cells.push((self.x - 1, self.y - 1));
        }
        if self.y > 0 {
            cells.push((self.x, self.y - 1));
            cells.push((self.x + 1, self.y - 1));
        }
        if self.x > 0 {
            cells.push((self.x - 1, self.y));
        }
        cells.push((self.x + 1, self.y));
        if self.x > 0 {
            cells.push((self.x - 1, self.y + 1));
        }
        cells.push((self.x, self.y + 1));
        cells.push((self.x + 1, self.y + 1));
        cells
    }
}

struct Amount {
    s: String,
    x: usize,
    y: usize,
}
impl Amount {
    fn is_on(&self, x: usize, y: usize) -> bool {
        if self.y != y {
            return false;
        }
        if self.x > x {
            return false;
        }

        (x - self.x) < self.s.len()
    }
}

fn is_part(c: char) -> bool {
    if c >= '0' && c <= '9' {
        return false;
    }

    if c == '.' {
        return false;
    }

    return true;
}

fn detect_all_amounts(map: &Map) -> Vec<Amount> {
    let mut amounts: Vec<Amount> = Vec::new();

    for y in 0..map.height() {
        let mut x = 0;
        while x < map.width() {
            let c = map.get(x, y);

            if !(c >= '0' && c <= '9') {
                x += 1;
                continue;
            }
            let mut amount = Amount {
                s: String::from(c),
                x,
                y,
            };

            x += 1;
            while x < map.width() {
                let c = map.get(x, y);
                if !(c >= '0' && c <= '9') {
                    break;
                }
                amount.s.push(c);
                x += 1;
            }
            amounts.push(amount)
        }
    }
    amounts
}

fn detect_all_parts(map: &Map) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();

    for y in 0..map.height() {
        for x in 0..map.width() {
            let c = map.get(x, y);

            if is_part(c) {
                parts.push(Part { c, x, y });
            }
        }
    }
    parts
}

pub fn task1(input: &str) -> u64 {
    let map = Map::from(input);
    let parts = detect_all_parts(&map);
    let amounts = detect_all_amounts(&map);
    let mut connections: Vec<HashSet<usize>> = parts.iter().map(|_| HashSet::new()).collect();

    parts.iter().enumerate().for_each(|(i, part)| {
        part.borders().iter().for_each(|(x, y)| {
            amounts.iter().enumerate().for_each(|(j, a)| {
                if a.is_on(*x, *y) {
                    connections[i].insert(j);
                }
            })
        })
    });

    connections
        .iter()
        .enumerate()
        .map(|(_, h)| {
            h.iter()
                .map(|s| amounts.get(*s).unwrap().s.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .sum()
}

pub fn task2(input: &str) -> u64 {
    let map = Map::from(input);
    let parts = detect_all_parts(&map);
    let amounts = detect_all_amounts(&map);
    let mut connections: Vec<HashSet<usize>> = parts.iter().map(|_| HashSet::new()).collect();

    parts.iter().enumerate().for_each(|(i, part)| {
        part.borders().iter().for_each(|(x, y)| {
            amounts.iter().enumerate().for_each(|(j, a)| {
                if a.is_on(*x, *y) {
                    connections[i].insert(j);
                }
            })
        })
    });

    connections
        .iter()
        .enumerate()
        .filter(|(i, _)| parts[*i].c == '*')
        .filter(|(_, h)| h.len() == 2)
        .map(|(_, h)| {
            h.iter()
                .map(|s| amounts.get(*s).unwrap().s.parse::<u64>().unwrap())
                .product::<u64>()
        })
        .sum()
}

#[test]
fn map_test() {
    let data = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
    let map = Map::from(data);
    assert_eq!(map.width(), 10);
    assert_eq!(map.height(), 10);
    assert_eq!(map.get(3, 1), '*');
}

#[test]
fn task1_test() {
    let data = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
    assert_eq!(task1(data), 4361);
}

#[test]
fn task2_test() {
    let data = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
    assert_eq!(task2(data), 467835);
}

