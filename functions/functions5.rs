// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    // 1. Add a `return` ahead of `num * num;`
    // 2. remove `;`, make it to be `num * num`
    num * num
}
