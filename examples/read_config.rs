use std::process::exit;

use image_rpg::Config;

fn main() {
    let cfg = Config::from_file("resources/config.conf").unwrap_or_else(|e| {
        eprintln!("error:{:?}", e);
        exit(1);
    });

    println!("{:#?}", cfg);
}
