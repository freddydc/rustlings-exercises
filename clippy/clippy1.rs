// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)

fn main() {
    let x = 1.2331_f64;
    let y = 1.2332_f64;
    let error_margin = f64::EPSILON;
    // y != x
    if (y - x).abs() > error_margin {
        println!("Success!");
    }
    // x == y
    if (x - 1.2331_f64).abs() < error_margin {
        println!("Pass!");
    }
}
