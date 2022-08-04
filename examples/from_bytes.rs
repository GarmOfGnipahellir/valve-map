use valve_map::from_bytes;

fn main() {
    let input = include_bytes!("basic.map");
    let map = from_bytes(input).unwrap();
    println!("{:#?}", map);
}
