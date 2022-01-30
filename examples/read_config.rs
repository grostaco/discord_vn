use image_rpg::Config;

fn main() {
    let cfg = Config::from_file("resources/characters.conf").unwrap();

    println!("{:?}", cfg);
}
