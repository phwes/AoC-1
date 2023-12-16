const RED_CUBES: i32 = 12;
const GREEN_CUBES: i32 = 13;
const BLUE_CUBES: i32 = 14;

struct Cube {
    color: String, // Enums in rust are too dumb?
    amount: i32,
}

struct Bag {
    redCubes: i32,
    greenCubes: i32,
    blueCubes: i32,
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

fn min_bag(cubes: Vec<Cube>) -> Bag {
    let mut bag = Bag {
        redCubes: 0,
        greenCubes: 0,
        blueCubes: 0,
    };

    for cube in cubes {
        match cube.color.trim() {
            "red" => {
                if cube.amount > bag.redCubes {
                    bag.redCubes = cube.amount;
                }
            }
            "green" => {
                if cube.amount > bag.greenCubes {
                    bag.greenCubes = cube.amount;
                }
            }
            "blue" => {
                if cube.amount > bag.blueCubes {
                    bag.blueCubes = cube.amount;
                }
            }
            _ => !panic!("Unknown color: {}", cube.color),
        }
    }
    bag
}

fn main() {
    let sum: i32 = std::fs::read_to_string("res/input.txt")
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
            let smallest_bag = min_bag(cubes);
            let power = smallest_bag.redCubes * smallest_bag.greenCubes * smallest_bag.blueCubes;

            power
        })
        .sum(); // TODO: replace map + sum with reduce

    println!("sum: {}", sum);
    ()
}
