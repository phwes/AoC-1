const FILE_PATH: &str = "res/test.txt";

fn main() {
    std::fs::read_to_string(FILE_PATH).unwrap();
}
