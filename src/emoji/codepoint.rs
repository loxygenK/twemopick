pub fn get_emoji_codepoint(emoji: &str) -> String {
    // Referred:
    // https://github.com/twitter/twemoji-parser/blob/66059e5da13a0407a49bdbe4b1980e00fcc92b62/src/index.js#L58
    let mut points = Vec::with_capacity(6);
    let mut pending: u32 = 0;

    for char in emoji.encode_utf16().map(u32::from) {
        if pending != 0 {
            points.push(format!(
                "{:02x}",
                0x10000 + ((pending - 0xd800) << 10) + (char - 0xdc00)
            ));
        } else if char > 0xd800 && char <= 0xdbff {
            pending = char;
        } else {
            points.push(format!("{:02x}", char));
        }
    }

    points.join("")
}

