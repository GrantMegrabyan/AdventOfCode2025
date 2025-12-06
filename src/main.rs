mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    // Read the first argument
    let args: Vec<String> = std::env::args().collect();
    let arg = &args[1];

    let day = &arg[..5];
    let part = &arg[5..];
    let input = get_input(day);

    run(day, part, &input);
}

fn get_input(day: &str) -> String {
    std::fs::read_to_string(format!("inputs/{}.txt", day)).unwrap()
}

fn run(day: &str, part: &str, input: &str) {
    match (day, part) {
        ("day01", "part1") => {
            let result = day01::part1(input).unwrap();
            println!("Result: {}", result);
        }
        ("day01", "part2") => {
            let result = day01::part2(input).unwrap();
            println!("Result: {}", result);
        }
        ("day02", "part1") => {
            let result = day02::part1(input).unwrap();
            println!("Result: {}", result);
        }
        ("day02", "part2") => {
            let result = day02::part2(input).unwrap();
            println!("Result: {}", result);
        }
        ("day03", "part1") => {
            let result = day03::part1(input).unwrap();
            println!("Result: {}", result);
        }
        ("day03", "part2") => {
            let result = day03::part2(input).unwrap();
            println!("Result: {}", result);
        }
        ("day04", "part1") => {
            let result = day04::part1(input).unwrap();
            println!("Result: {}", result);
        }
        ("day04", "part2") => {
            let result = day04::part2(input).unwrap();
            println!("Result: {}", result);
        }
        ("day05", "part1") => {
            let result = day05::part1(input).unwrap();
            println!("Result: {}", result);
        }
        _ => panic!("Invalid day or part"),
    }
}
