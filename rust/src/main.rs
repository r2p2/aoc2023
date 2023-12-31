mod day02;
mod day03;
mod day05;
mod day06;
mod day16;
mod day19;
mod day20;
mod day22;
mod day25;

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
    let path = args.next().expect("Third argument needs to be the path.");

    let contents = std::fs::read_to_string(path).expect("Should be able to read the file.");

    if day == 2 && part == 1 {
        println!("{}", day02::task1(&contents));
    } else if day == 2 && part == 2 {
        println!("{}", day02::task2(&contents));
    } else if day == 3 && part == 1 {
        println!("{}", day03::task1(&contents));
    } else if day == 3 && part == 2 {
        println!("{}", day03::task2(&contents));
    } else if day == 5 && part == 1 {
        println!("{}", day05::task1(&contents));
    } else if day == 5 && part == 2 {
        println!("{}", day05::task2(&contents));
    } else if day == 6 && part == 1 {
        println!("{}", day06::task1(&contents));
    } else if day == 6 && part == 2 {
        println!("{}", day06::task2(&contents));
    } else if day == 16 && part == 1 {
        println!("{}", day16::task1(&contents));
    } else if day == 16 && part == 2 {
        println!("{}", day16::task2(&contents));
    } else if day == 19 && part == 1 {
        println!("{}", day19::task1(&contents));
    } else if day == 19 && part == 2 {
        println!("{}", day19::task2(&contents));
    } else if day == 20 && part == 1 {
        println!("{}", day20::task1(&contents));
    } else if day == 20 && part == 2 {
        println!("{}", day20::task2(&contents));
    } else if day == 22 && part == 1 {
        println!("{}", day22::task1(&contents));
    } else if day == 22 && part == 2 {
        println!("{}", day22::task2(&contents));
    } else if day == 25 && part == 1 {
        println!("{}", day25::task1(&contents));
    }
}
