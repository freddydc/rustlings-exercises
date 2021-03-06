// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` for hints :)

use std::f32;

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

    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
