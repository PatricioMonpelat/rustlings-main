// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)


fn main() {
    let num: i32 = 3;
    let answer = square(num);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}
