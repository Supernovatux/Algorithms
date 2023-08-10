pub mod bisection;
pub mod exhaustive_search;
pub mod fibbonacci_search;
pub mod interval_halving;

type Solver = fn(fn(f64) -> f64, f64, f64, f64) -> f64;

pub struct Algorithms {
    pub name: &'static str,
    pub func: Solver,
}
impl Algorithms {}
// Array of all Algorithms
pub const ALGORITHMS: [Algorithms; 5] = [
    Algorithms {
        name: "Exhaustive Search",
        func: exhaustive_search::exhaustive_search,
    },
    Algorithms {
        name: "Interval Halving",
        func: interval_halving::interval_halving,
    },
    Algorithms {
        name: "Bisection",
        func: bisection::bisection_search,
    },
    Algorithms {
        name: "Golder ratio Search",
        func: fibbonacci_search::golder_search,
    },
    Algorithms {
        name: "Fibonacci Search",
        func: fibbonacci_search::fibbonacci_search,
    },
];
