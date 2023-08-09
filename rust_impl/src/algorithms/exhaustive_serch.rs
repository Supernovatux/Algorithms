use crate::utils::linspace;

pub fn exhaustive_search(func: fn(f64) -> f64, a: f64, b: f64, err: f64) -> f64 {
    // Lets do 1000 devs per iter and find maximum
    let mut current_err = f64::MAX;
    let mut x = a;
    let mut y = b;
    while current_err < err {
        current_err = (x - y).abs() / 1000f64;
        let mut max_x = x;
        for i in linspace(x, y, 1000) {
            if func(i) > func(max_x) {
                max_x = i;
            }
        }
        x = max_x - current_err;
        y = x + current_err;
    }
    return x;
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
