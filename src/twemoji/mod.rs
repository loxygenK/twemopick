pub enum EmojiOutputType {
    SVG,
    Laster
}

pub fn get_twemoji_codepoint(emoji: &str) -> String {
    // Referred:
    // https://github.com/twitter/twemoji-parser/blob/66059e5da13a0407a49bdbe4b1980e00fcc92b62/src/index.js#L58
    let mut points = Vec::with_capacity(6);
    let mut pending: u32 = 0;

    for char in emoji.encode_utf16().map(|e: u16| u32::from(e)) {
        if pending != 0 {
            points.push(format!(
                "{:02x}",
                0x10000 + ((pending - 0xd800) << 10) + (char - 0xdc00)
            ));
        } else if char > 0xd800 && char <= 0xdbff {
            pending = From::from(char);
        } else {
            points.push(format!("{:02x}", char));
        }
    }
    return points.join("");
}

pub fn generate_remote_url(emoji: &str, output: EmojiOutputType) -> String {
    format!(
        "https://twemoji.maxcdn.com/v/latest/{}/{}.svg",
        match output {
            EmojiOutputType::SVG => "svg",
            EmojiOutputType::Laster => "72x72"
        },
        get_twemoji_codepoint(emoji)
    ).to_string()
}
