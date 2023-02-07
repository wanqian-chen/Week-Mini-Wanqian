/* create a coin function which takes one parameter of the probability of heads,
and generate a random number between 0 and 1. If the number is less than the probability
of heads, return 1, otherwise return 0. */

use rand::Rng;
pub fn coin(probability: f64) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    if random_number < probability {
        return 1;
    } else {
        return 0;
    }
}
