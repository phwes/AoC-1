use std::char;

const FILE_PATH: &str = "res/input.txt";

fn char_is_sign(char: char) -> bool {
    if char == '.' || char.is_digit(10) {
        return false;
    }
    return true;
}

fn has_adjacent_sign(matrix: &Vec<String>, i: usize, j: usize) -> bool {
    let mut has_adjacent_sign = false;
    let mut adjacent_signs = 0;

    if i > 0 {
        if j > 0 {
            if char_is_sign(matrix[i - 1].chars().nth(j - 1).unwrap()) {
                adjacent_signs += 1;
            }
        }
        if char_is_sign(matrix[i - 1].chars().nth(j).unwrap()) {
            adjacent_signs += 1;
        }
        if j < matrix[i].len() - 1 {
            if char_is_sign(matrix[i - 1].chars().nth(j + 1).unwrap()) {
                adjacent_signs += 1;
            }
        }
    }

    if j > 0 {
        if char_is_sign(matrix[i].chars().nth(j - 1).unwrap()) {
            adjacent_signs += 1;
        }
    }
    if j < matrix[i].len() - 1 {
        if char_is_sign(matrix[i].chars().nth(j + 1).unwrap()) {
            adjacent_signs += 1;
        }
    }

    if i < matrix.len() - 1 {
        if j > 0 {
            if char_is_sign(matrix[i + 1].chars().nth(j - 1).unwrap()) {
                adjacent_signs += 1;
            }
        }
        if char_is_sign(matrix[i + 1].chars().nth(j).unwrap()) {
            adjacent_signs += 1;
        }
        if j < matrix[i].len() - 1 {
            if char_is_sign(matrix[i + 1].chars().nth(j + 1).unwrap()) {
                adjacent_signs += 1;
            }
        }
    }

    if adjacent_signs > 0 {
        has_adjacent_sign = true;
    }

    return has_adjacent_sign;
}

fn is_last_digit_in_number(matrix: &Vec<String>, i: usize, j: usize) -> bool {
    if matrix[i].len() == j + 1 {
        return true;
    }

    let next_char = matrix[i].chars().nth(j + 1).unwrap();
    if char_is_sign(next_char) || next_char == '.' {
        return true;
    }

    return false;
}

fn main() {
    let matrix = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let mut sum = 0;
    let mut add_number_to_sum = false;
    let mut current_number = String::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let char = matrix[i].chars().nth(j).unwrap();

            if char_is_sign(char) || char == '.' {
                continue;
            }

            if char.is_digit(10) {
                current_number.push(char);
            }

            if has_adjacent_sign(&matrix, i, j) {
                add_number_to_sum = true;
            }

            if is_last_digit_in_number(&matrix, i, j) {
                if add_number_to_sum {
                    sum += current_number.parse::<i32>().unwrap();
                    add_number_to_sum = false;
                }
                current_number = String::new();
            }
        }
    }
    print!("Sum: {}", sum)
}
