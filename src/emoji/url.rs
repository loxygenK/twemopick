pub enum ImageType {
    Raster,
    Svg
}

pub fn get_image_url(codepoint: &str, image_type: &ImageType) -> String {
    match image_type {
        ImageType::Svg => format!("https://twemoji.maxcdn.com/v/latest/svg/{}.svg", codepoint),
        ImageType::Raster => format!("https://twemoji.maxcdn.com/v/latest/72x72/{}.png", codepoint),
    }
}

pub fn get_emoji_dataurl(codepoint: &str, image_type: &ImageType) -> String {
    let image_bytes = reqwest::blocking::get(&get_image_url(codepoint, image_type))
        .expect("Receiving emoji image data")
        .error_for_status()
        .expect("Checking acceptance of the request")
        .bytes()
        .expect("Converting the response into byte");

    let data_url = format!(
        "data:image/{};base64,{}",
        match image_type {
            ImageType::Raster => "png",
            ImageType::Svg => "svg+xml"
        },
        base64::encode(image_bytes)
    );

    data_url
}

