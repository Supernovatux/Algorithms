pub fn linspace(start: f64, stop: f64, num: usize) -> Vec<f64> {
    let mut v = Vec::new();
    let step = (stop - start) / (num - 1) as f64;
    for i in 0..num {
        v.push(start + i as f64 * step);
    }
    return v;
}
pub const DELTA: f64 = 1e-14;
pub fn differentiate(f: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    return (f(x + h) - f(x - h)) / (2.0 * h);
}
pub mod functions {
    use std::f64::consts::PI;

    pub fn downparabola(x: f64) -> f64 {
        return -x * x + 1.0;
    }
    pub fn cosx(x: f64) -> f64 {
        //-1,1
        return x.cos();
    }
    pub fn exp_sqr(x: f64) -> f64 {
        //-2,2
        return (-x * x).exp();
    }
    pub fn sinc(x: f64) -> f64 {
        //0.5,1.4
        //0.819675
        return (3.0 * PI * x).sin() / x;
    }
    pub fn xthroot(x: f64) -> f64 {
        //0,10
        //e
        return x.powf(1.0 / x);
    }
    pub fn mxthpower(x: f64) -> f64 {
        //0,10
        //1/e
        return x.powf(-x);
    }
    pub struct TestFunction {
        pub f: fn(f64) -> f64,
        pub a: f64,
        pub b: f64,
        pub sol: f64,
    }
    impl TestFunction {
        pub fn new(f: fn(f64) -> f64, a: f64, b: f64, sol: f64) -> TestFunction {
            TestFunction { f, a, b, sol }
        }
        pub fn solve(&self, x: f64) -> f64 {
            return (self.f)(x);
        }
    }
    // A function vec to iterate over
    pub const TEST_FUNCTIONS: [TestFunction; 6] = [
        TestFunction {
            f: downparabola,
            a: -1.0,
            b: 1.0,
            sol: 0.0,
        },
        TestFunction {
            f: cosx,
            a: -1.0,
            b: 1.0,
            sol: 0.0,
        },
        TestFunction {
            f: exp_sqr,
            a: -2.0,
            b: 2.0,
            sol: 0.0,
        },
        TestFunction {
            f: sinc,
            a: 0.5,
            b: 1.4,
            sol: 0.819675,
        },
        TestFunction {
            f: xthroot,
            a: 0.0,
            b: 10.0,
            sol: std::f64::consts::E,
        },
        TestFunction {
            f: mxthpower,
            a: 0.0,
            b: 10.0,
            sol: 1.0 / std::f64::consts::E,
        },
    ];
}
// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linspace() {
        let v = linspace(0.0, 1.0, 11);
        assert_eq!(v.len(), 11);
        assert_eq!(v[0], 0.0);
        assert_eq!(v[10], 1.0);
    }
    #[test]
    fn test_differentiate() {
        let f = functions::downparabola;
        let x = 0.0;
        let h = 1e-6;
        let df = differentiate(f, x, h);
        println!("{}", df);
    }
}
