const FILE_PATH: &str = "res/input.txt";

fn get_start_of_number(line: &String, char_j: usize) -> usize {
    let mut current_char_position = char_j;
    let mut current_char: char;

    loop {
        current_char = line.chars().nth(current_char_position).unwrap();
        if !current_char.is_digit(10) {
            current_char_position += 1;
            break;
        }
        if current_char_position == 0 {
            break;
        }
        current_char_position -= 1;
    }

    current_char_position
}

fn left_number(line: &String, char_j: usize) -> i32 {
    let first_char = line.chars().nth(char_j - 1).unwrap();
    if !first_char.is_digit(10) {
        return 0;
    }

    let mut number = String::new();
    let first_char_position = get_start_of_number(line, char_j - 1);
    for char in line.chars().skip(first_char_position) {
        if char.is_digit(10) {
            number.push(char);
        } else {
            break;
        }
    }

    number.chars().collect::<String>().parse::<i32>().unwrap()
}

fn right_number(line: &String, char_j: usize) -> i32 {
    let first_char = line.chars().nth(char_j + 1).unwrap();
    if !first_char.is_digit(10) {
        return 0;
    }

    let mut number = String::new();
    for char in line.chars().skip(char_j + 1) {
        if char.is_digit(10) {
            number.push(char);
        } else {
            break;
        }
    }
    number.parse::<i32>().unwrap()
}

fn middle_number(line: &String, char_j: usize) -> i32 {
    let middle_char = line.chars().nth(char_j).unwrap();
    if !middle_char.is_digit(10) {
        return 0;
    }

    let first_char_position = get_start_of_number(line, char_j);

    let mut number = String::new();
    for char in line.chars().skip(first_char_position) {
        if char.is_digit(10) {
            number.push(char);
        } else {
            break;
        }
    }

    number.parse::<i32>().unwrap()
}

fn gear_ratio(matrix: &Vec<String>, line_i: usize, char_j: usize) -> i32 {
    let line_length = matrix[0].len();
    let mut n_numbers = 0;
    let mut ratio = 1;

    if char_j != 0 {
        let left_n = left_number(&matrix[line_i], char_j);
        if left_n != 0 {
            ratio *= left_n;
            n_numbers += 1;
        }
    }

    if char_j != line_length - 1 {
        let right_n = right_number(&matrix[line_i], char_j);
        if right_n != 0 {
            ratio *= right_n;
            n_numbers += 1;
        }
    }

    if line_i != 0 {
        let top_middle_number = middle_number(&matrix[line_i - 1], char_j);
        if top_middle_number != 0 {
            ratio *= top_middle_number;
            n_numbers += 1;
        } else {
            let top_left_number = left_number(&matrix[line_i - 1], char_j);
            let top_right_number = right_number(&matrix[line_i - 1], char_j);
            if top_left_number != 0 {
                ratio *= top_left_number;
                n_numbers += 1;
            }
            if top_right_number != 0 {
                ratio *= top_right_number;
                n_numbers += 1;
            }
        }
    }

    if line_i != line_length - 1 {
        let bottom_middle_number = middle_number(&matrix[line_i + 1], char_j);
        if bottom_middle_number != 0 {
            ratio *= bottom_middle_number;
            n_numbers += 1;
        } else {
            let bottom_left_number = left_number(&matrix[line_i + 1], char_j);
            let bottom_right_number = right_number(&matrix[line_i + 1], char_j);
            if bottom_left_number != 0 {
                ratio *= bottom_left_number;
                n_numbers += 1;
            }
            if bottom_right_number != 0 {
                ratio *= bottom_right_number;
                n_numbers += 1;
            }
        }
    }

    if n_numbers == 2 {
        return ratio;
    }

    return 0;
}

fn main() {
    let matrix = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let mut sum = 0;
    for (line_i, line) in matrix.iter().enumerate() {
        for (char_j, char) in line.chars().enumerate() {
            if char == '*' {
                sum += gear_ratio(&matrix, line_i, char_j);
            }
        }
    }
    println!("sum: {}", sum)
}
