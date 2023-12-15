fn main() {
    let sum = std::fs::read_to_string("res/input.txt")
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
            let last_digit = line.chars().rev().find(|c| c.is_digit(10)).unwrap();

            let number = first_digit.to_string() + &last_digit.to_string();
            let number = number.parse::<i32>().unwrap();

            acc + number
        });
    print!("{}", sum);
    ()
}
