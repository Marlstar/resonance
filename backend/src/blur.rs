use image::RgbaImage;

pub fn blur(ytid: &str, sigma: f32) -> RgbaImage {
    // TODO: error handling
    return image::open(crate::dirs().song_thumbnail(ytid)).unwrap()
        //.blur(15.0);
        .fast_blur(sigma)
        .to_rgba8();
}
