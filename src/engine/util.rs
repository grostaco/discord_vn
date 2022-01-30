pub fn strip_whitespace(mut s: String) -> String {
    s.retain(|c| !c.is_whitespace());
    s
}
