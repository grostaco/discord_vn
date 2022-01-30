use rusttype::PositionedGlyph;

pub fn glyphs_width(glyphs: &Vec<PositionedGlyph>) -> u32 {
    let min_x = glyphs
        .first()
        .map(|g| g.pixel_bounding_box().unwrap().min.x)
        .unwrap();
    let max_x = glyphs
        .last()
        .map(|g| g.pixel_bounding_box().unwrap().max.x)
        .unwrap();
    (max_x - min_x) as u32
}
