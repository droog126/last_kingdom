use rand::{thread_rng, Rng};

pub fn random_arr2(x: u32, y: u32) -> impl Iterator<Item = [f32; 2]> + Clone {
    std::iter::repeat_with(move || {
        let mut rng = thread_rng();
        let randx = rng.gen::<f32>() * x as f32 - x as f32 / 2.0;
        let randy = rng.gen::<f32>() * y as f32 - y as f32 / 2.0;
        [randx, randy]
    })
}

pub fn random_arr4(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> impl Iterator<Item = [i32; 4]> + Clone {
    std::iter::repeat_with(move || {
        let mut rng = thread_rng();
        let randx = rng.gen_range(-x / 2..x / 2);
        let randy = rng.gen_range(-y / 2..y / 2);
        let randWidth = rng.gen_range(0..width);
        let randHeight = rng.gen_range(0..height);
        [randx, randy, randWidth, randHeight]
    })
}
