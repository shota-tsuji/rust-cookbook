use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let randon_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", randon_string);
    println!("{}", randon_string.len());
}
