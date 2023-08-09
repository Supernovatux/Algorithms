pub fn exhaustive_search(func: fn(f64) -> f64, a: f64, b: f64, err: f64) -> f64 {
    // recursively do till the error is less than the given error
    let mut max_y = f64::MIN;
    let mut max_x = f64::MIN;
    let step = (b - a) / 1000.0;
    let mut x = a;
    while x <= b {
        let y = func(x);
        if y > max_y {
            max_y = y;
            max_x = x;
        }
        x += step;
    }
    if step < err {
        max_x
    } else {
        exhaustive_search(func, max_x - step, max_x + step, err)
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
            let res = exhaustive_search(fns.f, fns.a, fns.b, 1e-7);
            println!("Result: {}", res);
            //assert!(abs(res - fns.sol) <= 1e-7);
        }
    }
}
