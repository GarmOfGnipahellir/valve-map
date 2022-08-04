use valve_map::map_from_str;

fn main() {
    let input = include_str!("basic.map");
    let map = map_from_str(input).unwrap();
    println!("{:#?}", map);
}
