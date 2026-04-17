pub const SUN_SVG: &[u8] = include_bytes!("../data/icons/sun.svg");
pub const MOON_SVG: &[u8] = include_bytes!("../data/icons/moon.svg");

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_symbolic_ready(svg: &[u8], name: &str) {
        let s = std::str::from_utf8(svg).unwrap_or_else(|_| panic!("{name}: invalid utf-8"));
        assert!(
            s.contains("currentColor"),
            "{name}: expected `currentColor` to drive symbolic recolor"
        );
        let has_hex_color = s.match_indices('#').any(|(i, _)| {
            s[i + 1..]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_hexdigit())
        });
        assert!(!has_hex_color, "{name}: hex color literal would block recolor");
        assert!(!s.contains("rgb("), "{name}: `rgb(` literal would block recolor");
        assert!(!s.contains("hsl("), "{name}: `hsl(` literal would block recolor");
    }

    #[test]
    fn sun_is_symbolic_ready() {
        assert_symbolic_ready(SUN_SVG, "sun.svg");
    }

    #[test]
    fn moon_is_symbolic_ready() {
        assert_symbolic_ready(MOON_SVG, "moon.svg");
    }
}
