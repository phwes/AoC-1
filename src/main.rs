use std::char;

const FILE_PATH: &str = "res/test.txt";

fn char_is_sign(char: char) -> bool {
    if char == '.' || char.is_digit(10) {
        return false;
    }
    return true;
}

fn find_start_at_number(matrix: &Vec<String>, i: usize, j: usize) -> usize {
    println!("char: {}", matrix[i].chars().nth(j).unwrap());
    let mut current_j = j;
    if !matrix[i].chars().nth(current_j).unwrap().is_digit(10) {
        panic!("No number at this position");
    }

    while current_j > 0 {
        let char = matrix[i].chars().nth(current_j - 1).unwrap();
        if char_is_sign(char) {
            current_j += 1;
            break;
        }
        current_j -= 1;
    }

    return current_j;
}

fn get_number_at(matrix: &Vec<String>, i: usize, j: usize) -> i32 {
    let mut number = String::new();
    let mut current_char = matrix[i].chars().nth(j).unwrap();
    let mut current_j = find_start_at_number(matrix, i, j);

    while current_char.is_digit(10) {
        number.push(current_char);

        if current_j == matrix[i].len() - 1 {
            break;
        }

        current_j += 1;
        current_char = matrix[i].chars().nth(current_j).unwrap();
    }

    number.parse::<i32>().unwrap()
}

fn get_adjacent_numbers_sum(matrix: &Vec<String>, i: usize, j: usize) -> i32 {
    let mut adjacent_numbers = 0;
    let mut product = 1;

    if i > 0 {
        let char = matrix[i - 1].chars().nth(j).unwrap();
        if char.is_digit(10) {
            adjacent_numbers += 1;
            product *= get_number_at(matrix, i - 1, j)
        }
    }

    if i < matrix.len() - 1 {
        let char = matrix[i + 1].chars().nth(j).unwrap();
        if char.is_digit(10) {
            adjacent_numbers += 1;
            product *= get_number_at(matrix, i + 1, j)
        }
    }

    if j > 0 {
        let char = matrix[i].chars().nth(j - 1).unwrap();
        if char.is_digit(10) {
            adjacent_numbers += 1;
            product *= get_number_at(matrix, i, j - 1)
        } else if i > 0 {
            let char = matrix[i - 1].chars().nth(j - 1).unwrap();
            if char.is_digit(10) {
                adjacent_numbers += 1;
                product *= get_number_at(matrix, i - 1, j - 1)
            }
        }
    }

    if j < matrix[i].len() - 1 {
        let char = matrix[i].chars().nth(j + 1).unwrap();
        if char.is_digit(10) {
            adjacent_numbers += 1;
            product *= get_number_at(matrix, i, j + 1)
        }
    }
    print!("Adjacent numbers: {}\n", adjacent_numbers);
    if adjacent_numbers < 2 {
        return 0;
    }

    return product; // Calculate adjacent numbers
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

            if char != '*' {
                continue;
            }
            sum += get_adjacent_numbers_sum(&matrix, i, j);
        }
    }
    print!("Sum: {}", sum)
}
