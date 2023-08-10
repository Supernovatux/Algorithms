pub fn fibbonacci(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibbonacci(n - 1) + fibbonacci(n - 2)
    }
}
