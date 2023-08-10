use std::f64::consts::E;

use crate::utils::series::fibonacci::{fibbonacci, invfibbonacci};

// Golden ratio method
const INVPHI: f64 = 0.61803398875;
const INVPHI2: f64 = INVPHI * INVPHI;
pub fn golder_search(func: fn(f64) -> f64, a: f64, b: f64, err: f64) -> f64 {
    let mut a = a;
    let mut b = b;
    let h = b - a;
    if h <= err {
        return (a + b) / 2.0;
    }
    let n = ((err / h).log(E) / INVPHI.log(E)).ceil() as usize;
    let mut c = a + INVPHI2 * h;
    let mut d = a + INVPHI * h;
    let mut yc = func(c);
    let mut yd = func(d);
    for _ in 0..n {
        if yc > yd {
            b = d;
            d = c;
            yd = yc;
            c = a + INVPHI2 * (b - a);
            yc = func(c);
        } else {
            a = c;
            c = d;
            yc = yd;
            d = a + INVPHI * (b - a);
            yd = func(d);
        }
    }
    if yc > yd {
        (a + d) / 2.0
    } else {
        (c + b) / 2.0
    }
}
pub fn fibbonacci_search(func: fn(f64) -> f64, a: f64, b: f64, err: f64) -> f64 {
    let mut a = a;
    let mut b = b;
    let mut n = invfibbonacci(a, b, err);
    let mut c = a + (b - a) * (fibbonacci(n - 2) as f64) / (fibbonacci(n) as f64);
    let mut d = a + (b - a) * (fibbonacci(n - 1) as f64) / (fibbonacci(n) as f64);
    let mut yc = func(c);
    let mut yd = func(d);
    for _ in 0..(n - 2) {
        if yc > yd {
            b = d;
            d = c;
            yd = yc;
            n -= 1;
            c = a + (b - a) * (fibbonacci(n - 2) as f64) / (fibbonacci(n) as f64);
            yc = func(c);
        } else {
            a = c;
            c = d;
            yc = yd;
            n -= 1;
            d = a + (b - a) * (fibbonacci(n - 1) as f64) / (fibbonacci(n) as f64);
            yd = func(d);
        }
    }
    if yc > yd {
        (a + d) / 2.0
    } else {
        (c + b) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::functions::TEST_FUNCTIONS;
    #[test]
    fn tester() {
        for fns in TEST_FUNCTIONS {
            let res = fibbonacci_search(fns.f, fns.a, fns.b, 1e-7);
            println!("Result: {}", res);
            //assert!(abs(res - fns.sol) <= 1e-7);
        }
    }
}
