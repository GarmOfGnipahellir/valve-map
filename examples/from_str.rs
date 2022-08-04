use valve_map::from_str;

fn main() {
    let input = include_str!("basic.map");
    let map = from_str(input).unwrap();
    println!("{:#?}", map);
}
