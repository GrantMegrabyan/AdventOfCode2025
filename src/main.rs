mod day01;

fn main() {
    // Read the first argument
    let args: Vec<String> = std::env::args().collect();
    let arg = &args[1];

    let day = &arg[..5];
    let part = &arg[5..];
    let input = get_input(day, part);

    run(day, part, &input);
}

fn get_input(day: &str, part: &str) -> String {
    std::fs::read_to_string(format!("inputs/{}{}.txt", day, part)).unwrap()
}

fn run(day: &str, part: &str, input: &str) {
    match (day, part) {
        ("day01", "part1") => {
            let result = day01::part1(input).unwrap();
            println!("Result: {}", result);
        }
        // ("day01", "part2") => day01::part2(input),
        _ => panic!("Invalid day or part"),
    }
}
