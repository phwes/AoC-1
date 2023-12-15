fn replaceToDigits(string: &str) -> String {
    let mut result = String::new();

    result = string.replace("zero", "zero0zero");
    result = result.replace("one", "one1one");
    result = result.replace("two", "two2two");
    result = result.replace("three", "three3three");
    result = result.replace("four", "four4four");
    result = result.replace("five", "five5five");
    result = result.replace("six", "six6six");
    result = result.replace("seven", "seven7seven");
    result = result.replace("eight", "eight8eigth");
    result = result.replace("nine", "nine9nine");

    result
}
fn main() {
    let sum = std::fs::read_to_string("res/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut string = line.to_string();
            string = replaceToDigits(&string);

            let first_digit = string.chars().find(|c| c.is_digit(10)).unwrap();
            let last_digit = string.chars().rev().find(|c| c.is_digit(10)).unwrap();

            let number = first_digit.to_string() + last_digit.to_string().as_str();
            println!("number: {}", number);
            println!("string: {}", string);

            number.parse::<i32>().unwrap()
        })
        .sum::<i32>();

    println!("sum: {}", sum);
    ()
}
