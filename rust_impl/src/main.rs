use color_print::{cprint, cprintln};
use rust_impl::{algorithms::ALGORITHMS, utils::functions::TEST_FUNCTIONS};

fn main() {
    cprintln!("<s><u><g!>Sample of all algorithms and functions to find maxima</></></>");
    cprintln!("---------------------------------------------------");
    for algs in ALGORITHMS {
        cprintln!("<s><g>Algorithm: {}</></>", algs.name);
        for fns in TEST_FUNCTIONS {
            let res = (algs.func)(fns.f, fns.a, fns.b, 1e-7);
            // Display error and result
            cprintln!("<m>Function: {}</m>", fns.name);
            cprint!("<g>Result: {:+.20}     </g>", res);
            cprintln!("<r>Error: {:+.20}</r>", fns.sol - res);
        }
        println!("---------------------------------------------------");
    }
}
