fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort_unstable();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
