use std::fs::File;
use valve_map::from_reader;

fn main() {
    let mut file = File::open("examples/basic.map").unwrap();
    let map = from_reader(&mut file).unwrap();
    println!("{:#?}", map);
}
