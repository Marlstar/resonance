pub fn blur(ytid: &str) {
    // TODO: error handling
    let raw = image::open(crate::dirs().song_thumbnail(ytid)).unwrap()
        //.blur(15.0);
        .fast_blur(200.0);

    let _ = raw.save(crate::dirs().song_thumbnail_blurred(ytid));
}
