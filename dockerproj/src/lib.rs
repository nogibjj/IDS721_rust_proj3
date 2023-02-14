/*A library that returns back random names */

use rand::Rng;
pub const NAME: [&str; 3] = [
    "John",
    "Anna",
    "Oreo",
];
pub fn random_name() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..NAME.len());
    NAME[random_index]
}