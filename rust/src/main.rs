mod day16;

fn main() {
    let mut args = std::env::args();
    _ = args.next();
    let day = args
        .next()
        .expect("First argument needs to be the day.")
        .parse::<i64>()
        .unwrap();
    let part = args
        .next()
        .expect("Second argument needs to be the part.")
        .parse::<i64>()
        .unwrap();
    let path = args
        .next()
        .expect("Third argument needs to be the path.");

    let contents = std::fs::read_to_string(path).expect("Should be able to read the file.");


    if day == 16 && part == 1 {
        println!("{}", day16::task1(&contents));
    } else if day == 16 && part == 2 {
        println!("{}", day16::task2(&contents));
    }
}
