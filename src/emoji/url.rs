pub enum ImageType {
    Raster,
    Svg
}

pub fn get_image_url(codepoint: &str, image_type: ImageType) -> String {
    match image_type {
        ImageType::Raster => format!("https://twemoji.maxcdn.com/v/latest/svg/{}.svg", codepoint),
        ImageType::Svg => format!("https://twemoji.maxcdn.com/v/latest/72x72/{}.png", codepoint),
    }
}

