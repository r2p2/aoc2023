#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}
impl Race {
    fn new(time: u64, record: u64) -> Race {
        Race { time, record }
    }

    fn solutions(&self) -> usize {
        (1..self.time)
            .into_iter()
            .map(|tp| (tp * self.time) - (tp * tp))
            .filter(|d| *d > self.record)
            .count()
    }
}

fn parse(input: &str) -> Vec<Race> {
    let mut line_it = input.lines();
    let mut time_it = line_it.next().unwrap().split_whitespace();
    let mut record_it = line_it.next().unwrap().split_whitespace();

    time_it.next();
    record_it.next();

    let mut races = Vec::new();

    while let Some(time) = time_it.next() {
        let record = record_it.next().unwrap();
        races.push(Race::new(time.parse().unwrap(), record.parse().unwrap()));
    }

    races
}

fn parse2(orig_input: &str) -> Vec<Race> {
    let mut input = orig_input.to_string();
    input.retain(|c| c.is_ascii_digit() || c == '\n');
    let mut line_it = input.lines();

    vec![Race::new(
        line_it.next().unwrap().parse().unwrap(),
        line_it.next().unwrap().parse().unwrap(),
    )]
}

pub fn task1(input: &str) -> usize {
    let races = parse(input);

    races.iter().map(|r| r.solutions()).product()
}

pub fn task2(input: &str) -> usize {
    let races = parse2(input);

    races.iter().map(|r| r.solutions()).product()
}

#[test]
fn race_test() {
    let race = Race::new(7, 9);
    assert_eq!(race.solutions(), 4);
}

#[test]
fn task1_test() {
    let data = r#"Time:      7  15   30
Distance:  9  40  200
"#;
    assert_eq!(task1(data), 288);
}

#[test]
fn task2_test() {
    let data = r#"Time:      7  15   30
Distance:  9  40  200
"#;
    assert_eq!(task2(data), 71503);
}
