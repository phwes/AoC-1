const RED_CUBES: i32 = 12;
const GREEN_CUBES: i32 = 13;
const BLUE_CUBES: i32 = 14;

struct Cube {
    color: String, // Enums in rust are too dumb?
    amount: i32,
}

fn parseCubes(line: &str) -> Vec<Cube> {
    let mut string = line;
    let words = string.split(" ").collect::<Vec<&str>>();

    let mut cubes = Vec::new();
    for (i, word) in words.iter().enumerate().step_by(2) {
        let amount = word.parse::<i32>().unwrap();
        let color = words[i + 1].to_string();
        println!("{} {}", amount, color);
        let cube = Cube { color, amount };

        cubes.push(cube);
    }

    return cubes;
}

fn canPlayWithCubes(cubes: Vec<Cube>) -> bool {
    let mut canPlay = true;
    for cube in cubes {
        match cube.color.trim() {
            "red" => {
                if cube.amount > RED_CUBES {
                    canPlay = false;
                }
            }
            "green" => {
                if cube.amount > GREEN_CUBES {
                    canPlay = false;
                }
            }
            "blue" => {
                if cube.amount > BLUE_CUBES {
                    canPlay = false;
                }
            }
            _ => !panic!("Unknown color: {}", cube.color),
        }
    }
    canPlay
}

fn main() {
    let sum: usize = std::fs::read_to_string("res/input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let string = line.split(":").collect::<Vec<&str>>()[1]
                .replace(",", "")
                .replace(";", "");
            let game_id = i + 1;

            println!("{}: {}", game_id, string);

            let cubes = parseCubes(string.trim());
            let can_play = canPlayWithCubes(cubes);
            if can_play {
                return game_id;
            }

            return 0;
        })
        .sum(); // TODO: replace map + sum with reduce

    println!("sum: {}", sum);
    ()
}
