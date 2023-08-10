use cached::proc_macro::cached;

#[cached]
pub fn fibbonacci(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibbonacci(n - 1) + fibbonacci(n - 2)
    }
}

pub fn invfibbonacci(a: f64, b: f64, err: f64) -> usize {
    let mut n = 0;
    while (fibbonacci(n) as f64) < ((b - a) / err) {
        n += 1;
    }
    n
}
