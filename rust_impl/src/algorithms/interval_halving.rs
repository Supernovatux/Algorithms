fn interval_halving(f: fn(f64) -> f64, a: f64, b: f64, err: f64) -> f64 {
    let mut a = a;
    let mut b = b;
    while (b - a) / 2.0 > err {
        let xm = (a + b) / 2.0;
        let l = b - a;
        let x1 = a + l / 4.0;
        let x2 = b - l / 4.0;
        let f1 = f(x1);
        let f2 = f(x2);
        let fm = f(xm);
        if (f1 > fm) && (f2 > fm) {
            return max(
                interval_halving(f, a, x1, err),
                interval_halving(f, x2, b, err),
            );
        } else if f1 > fm {
            b = xm;
        } else if f2 > fm {
            a = xm;
        } else {
            a = x1;
            b = x2;
        }
    }
    return (a + b) / 2.0;
}

fn max(err_1: f64, err_2: f64) -> f64 {
    if err_1 > err_2 {
        return err_1;
    } else {
        return err_2;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::functions::TEST_FUNCTIONS;
    use num::abs;
    #[test]
    fn tester() {
        for fns in TEST_FUNCTIONS {
            let res = interval_halving(fns.f, fns.a, fns.b, 1e-7);
            println!("Result: {}", res);
            assert!(abs(res - fns.sol) <= 1e-6);
        }
    }
    #[test]
    fn stress() {
        for _ in 1..1000 {
            tester();
        }
    }
}
