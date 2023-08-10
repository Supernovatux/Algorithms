use std::f64::consts::PI;

pub fn downparabola(x: f64) -> f64 {
    -x * x + 1.0
}
pub fn cosx(x: f64) -> f64 {
    //-1,1
    x.cos()
}
pub fn exp_sqr(x: f64) -> f64 {
    //-2,2
    (-x * x).exp()
}
pub fn sinc(x: f64) -> f64 {
    //0.5,1.4
    //0.819675
    (3.0 * PI * x).sin() / x
}
pub fn xthroot(x: f64) -> f64 {
    //0,10
    //e
    x.powf(1.0 / x)
}
pub fn mxthpower(x: f64) -> f64 {
    //0,10
    //1/e
    x.powf(-x)
}
pub struct TestFunction {
    pub f: fn(f64) -> f64,
    pub a: f64,
    pub b: f64,
    pub sol: f64,
    pub name: &'static str,
}
impl TestFunction {
    pub fn solve(&self, x: f64) -> f64 {
        (self.f)(x)
    }
}
// A function vec to iterate over
pub const TEST_FUNCTIONS: [TestFunction; 6] = [
    TestFunction {
        f: downparabola,
        a: -1.0,
        b: 1.0,
        sol: 0.0,
        name: "downparabola",
    },
    TestFunction {
        f: cosx,
        a: -1.0,
        b: 1.0,
        sol: 0.0,
        name: "cosx",
    },
    TestFunction {
        f: exp_sqr,
        a: -2.0,
        b: 2.0,
        sol: 0.0,
        name: "exp_sqr",
    },
    TestFunction {
        f: sinc,
        a: 0.5,
        b: 1.4,
        sol: 0.819675,
        name: "sinc",
    },
    TestFunction {
        f: xthroot,
        a: 0.0,
        b: 10.0,
        sol: std::f64::consts::E,
        name: "xthroot",
    },
    TestFunction {
        f: mxthpower,
        a: 0.0,
        b: 10.0,
        sol: 1.0 / std::f64::consts::E,
        name: "mxthpower",
    },
];
