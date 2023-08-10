use num::signum;

use crate::utils::differentiate;

pub fn bisection_search(f: fn(f64) -> f64, a: f64, b: f64, err: f64) -> f64 {
    let mut a = a;
    let mut b = b;
    let mut c = (a + b) / 2.0;
    let df = |x: f64| differentiate(f, x, err / 100.0);
    while (b - a) / 2.0 > err {
        if df(c) == 0.0 {
            break;
        } else if signum(df(a)) == signum(df(c)) {
            a = c;
        } else {
            b = c;
        }
        c = (a + b) / 2.0;
    }
    c
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::functions::TEST_FUNCTIONS;
    #[test]
    fn tester() {
        for fns in TEST_FUNCTIONS {
            let res = bisection_search(fns.f, fns.a, fns.b, 1e-7);
            println!("Result: {}", res);
            //assert!(abs(res - fns.sol) <= 1e-6);
        }
    }
}
