use rand::{thread_rng, Rng};

pub fn random_xy(width: u32, height: u32) -> impl Iterator<Item = [f32; 2]> + Clone {
    let mut rng = thread_rng();
    std::iter::repeat_with(move || {
        let randx = rng.gen::<f32>() * width as f32 - width as f32 / 2.0;
        let randy = rng.gen::<f32>() * height as f32 - height as f32 / 2.0;
        [randx, randy]
    })
}
