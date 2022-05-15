// ANCHOR: all
fn main() {
    // ANCHOR: waldo
    let name = "Waldo";
    println!("Hello, {}!", name);
    // ANCHOR_END: waldo

    // ANCHOR: math
    let a = 3;
    let b = 7;
    let c = a * b;
    println!("The result of {} * {} is {}", a, b, c);
    // ANCHOR_END: math

    // ANCHOR: mut
    let mut x = 4;
    let a = 3;
    println!("{}", x);
    x = 6;
    println!("{}", x);
    x = 1 + a;
    println!("{}", x);
    // ANCHOR_END: mut

    // ANCHOR: sum
    let x = 4;
    let a = 3;
    println!("{} + {} = {}", x, a, x + a);
    // ANCHOR_END: sum
}
// ANCHOR_END: all
