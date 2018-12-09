pub fn repeat<I>(func: &mut FnMut() -> I, n: usize) -> Vec<I> {
    (0..n).into_iter().map(|_| func()).collect()
}
