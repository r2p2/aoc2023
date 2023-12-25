#[derive(Debug, PartialEq, Clone, Copy)]
struct Interval {
    start: i64,
    len: i64,
}
impl Interval {
    fn new(start: i64, len: i64) -> Interval {
        Interval { start, len }
    }
}

#[derive(Debug)]
struct Range {
    src: i64,
    dst: i64,
    len: i64,
}
impl Range {
    fn map(&self, value: i64) -> Option<i64> {
        if value < self.src || value > self.src_last() {
            None
        } else {
            Some(value + self.offset())
        }
    }
    fn offset(&self) -> i64 {
        self.dst - self.src
    }
    fn src_last(&self) -> i64 {
        self.src + self.len - 1
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}
type Maps = Vec<Map>;
impl Map {
    fn new() -> Map {
        Map { ranges: Vec::new() }
    }
    fn sort_src(&mut self) {
        self.ranges.sort_by(|a, b| a.src.cmp(&b.src))
    }
    fn map(&self, value: i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|r| r.map(value))
            .unwrap_or_else(|| value)
    }

    fn map_interval(&self, interval: Interval) -> Vec<Interval> {
        let mut input = interval;
        let mut output: Vec<Interval> = Vec::new();

        for r in self.ranges.iter() {
            if input.len <= 0 {
                return output;
            }
            let left = Interval::new(input.start, std::cmp::min(r.src - input.start, input.len));
            if left.len > 0 {
                output.push(left);
                input.start += left.len;
                input.len -= left.len;
            }

            if input.len <= 0 {
                return output;
            }

            if r.src >= input.start {
                let to_map = Interval::new(r.src + r.offset(), std::cmp::min(r.len, input.len));
                if to_map.len > 0 {
                    output.push(to_map);
                    input.start += to_map.len;
                    input.len -= to_map.len;
                }
            } else if r.src < input.start && r.src_last() >= input.start {
                let to_map = Interval::new(
                    input.start + r.offset(),
                    std::cmp::min(r.src_last() - input.start + 1, input.len),
                );
                if to_map.len > 0 {
                    output.push(to_map);
                    input.start += to_map.len;
                    input.len -= to_map.len;
                }
            }
        }

        if input.len > 0 {
            output.push(input)
        }
        return output;
    }
}

fn parse(input: &str) -> (Vec<i64>, Maps) {
    let mut line_it = input.lines();
    let seeds: Vec<i64> = line_it
        .next()
        .unwrap()
        .split_whitespace()
        .filter(|s| *s != "seeds:")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut maps = Maps::new();

    for line in line_it {
        if line.len() == 0 {
            continue;
        }

        if line.find("map:").is_some() {
            maps.push(Map::new());
            continue;
        }

        let mut range_it = line.split_whitespace();
        maps.last_mut().unwrap().ranges.push(Range {
            dst: range_it.next().unwrap().parse::<i64>().unwrap(),
            src: range_it.next().unwrap().parse::<i64>().unwrap(),
            len: range_it.next().unwrap().parse::<i64>().unwrap(),
        })
    }

    maps.iter_mut().for_each(|m| m.sort_src());

    (seeds, maps)
}

pub fn task1(input: &str) -> i64 {
    let (seeds, maps) = parse(input);

    seeds
        .iter()
        .map(|seed| maps.iter().fold(seed.clone(), |s, m| m.map(s)))
        .min()
        .unwrap()
}

pub fn task2(input: &str) -> i64 {
    let (seeds_ranges, maps) = parse(input);
    let mut seeds: Vec<Interval> = Vec::new();
    seeds_ranges.as_slice().chunks(2).for_each(|s| {
        seeds.push(Interval::new(s[0], s[1]));
    });

    maps.iter()
        .fold(seeds.clone(), |seeds, m| {
            seeds.iter().map(|s| m.map_interval(*s)).flatten().collect()
        })
        .iter()
        .map(|i| i.start)
        .min()
        .unwrap()
}

#[test]
fn range_test() {
    let range = Range {
        src: 50,
        dst: 98,
        len: 2,
    };
    assert_eq!(range.map(49), None);
    assert_eq!(range.map(50), Some(98));
    assert_eq!(range.map(51), Some(99));
    assert_eq!(range.map(52), None);
}

#[test]
fn task1_test() {
    let data = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;
    assert_eq!(task1(data), 35)
}

#[test]
fn map_test() {
    let mut map = Map::new();
    map.ranges.push(Range {
        src: 6,
        dst: 0,
        len: 6,
    });
    map.sort_src();

    assert_eq!(
        map.map_interval(Interval::new(1, 5)),
        vec![Interval::new(1, 5)]
    );

    assert_eq!(
        map.map_interval(Interval::new(6, 3)),
        vec![Interval::new(0, 3)]
    );

    assert_eq!(
        map.map_interval(Interval::new(13, 3)),
        vec![Interval::new(13, 3)]
    );

    assert_eq!(
        map.map_interval(Interval::new(5, 3)),
        vec![Interval::new(5, 1), Interval::new(0, 2)]
    );

    assert_eq!(
        map.map_interval(Interval::new(5, 9)),
        vec![
            Interval::new(5, 1),
            Interval::new(0, 6),
            Interval::new(12, 2)
        ]
    );
}

#[test]
fn map_test2() {
    let mut map = Map::new();
    map.ranges.push(Range {
        src: 45,
        dst: 81,
        len: 19,
    });
    map.ranges.push(Range {
        src: 64,
        dst: 68,
        len: 13,
    });
    map.ranges.push(Range {
        src: 77,
        dst: 45,
        len: 23,
    });
    map.sort_src();

    assert_eq!(
        map.map_interval(Interval::new(79, 14)),
        vec![Interval::new(47, 14)]
    );
}

#[test]
fn task2_test() {
    let data = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;
    assert_eq!(task2(data), 46)
}
