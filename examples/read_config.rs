use image_rpg::Config;

fn main() {
    let cfg = Config::from_file("resources/config.conf").unwrap();

    println!("{:#?}", cfg);
}
