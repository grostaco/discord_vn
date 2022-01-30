use image_rpg::Script;

fn main() {
    let cfg = Script::from_file("resources/script.txt").unwrap();

    println!("{:#?}", cfg);
}
