use std::{fs::File, io::Read};

fn load_source_file(path: &str) -> String {
    let mut file: File = std::fs::File::open(path).unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn main() {
    let contents:String = load_source_file("./test/example.carb");
    print!("{}", contents);
}