pub fn linspace(start: f64, stop: f64, num: usize) -> Vec<f64> {
    let mut v = Vec::new();
    let step = (stop - start) / (num - 1) as f64;
    for i in 0..num {
        v.push(start + i as f64 * step);
    }
    v
}
pub const DELTA: f64 = 1e-14;
pub fn differentiate(f: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}
pub mod functions;
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
