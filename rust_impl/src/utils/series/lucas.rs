pub fn lucas(n: usize) -> usize {
    if n == 0 {
        2
    } else if n == 1 {
        1
    } else {
        lucas(n - 1) + lucas(n - 2)
    }
}
