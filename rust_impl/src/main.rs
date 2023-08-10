use rust_impl::{
    algorithms::{exhaustive_search::exhaustive_search, ALGORITHMS},
    utils::functions::TEST_FUNCTIONS,
};

fn main() {
    println!("Sample of all functions to find maxima of xsqrt(x)");
    println!("---------------------------------------------------");
    for algs in ALGORITHMS {
        println!("Algorithm: {}", algs.name);
        for fns in TEST_FUNCTIONS {
            let res = (algs.func)(fns.f, fns.a, fns.b, 1e-7);
            // Display error and result
            println!("Function: {}", fns.name);
            print!("Result: {}     ", res);
            println!("Error: {}", fns.sol - res);
        }
        println!("---------------------------------------------------");
    }
}
