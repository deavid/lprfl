# Returning values from functions

It is also possible to use a function to calculate a certain value. For example, we can calculate the number PI:

```rust
 fn calculate_pi() -> f64 {
   // https://en.wikipedia.org/wiki/Leibniz_formula_for_%CF%80
   // pi/4 = 1 - 1/3 + 1/5 - 1/7 + 1/9 - ...
   let mut pi_4 = 1.0;
   let mut divisor = 3.0;
   let steps = 20_000_000;
   for _ in 0..steps {
       pi_4 -= 1.0 / divisor;
       divisor += 2.0;
       pi_4 += 1.0 / divisor;
       divisor += 2.0;
   }
   return pi_4 * 4.0;
 }
 fn main() {
   let pi = calculate_pi();
   println!("pi: {}", pi);
 }
```

Again, this program might be a bit too much, but the important thing is in the first line:

```rust
 fn calculate_pi() -> f64 {
```

The arrow symbol (->) indicates that this function returns a value. "f64" is which kind of value we want to return, which for this case is a floating point number (one with decimal places). If we wanted to return an integer we would have used "i64".

The last line of the function specifies the actual value to be returned:

```rust
   return pi_4 * 4.0;
```

Since this fraction series calculates PI divided by four, we need to multiply it by four to get PI, and the return keyword is what signals Rust that it should exit the function at that point (it marks the end of it) and return the value on the right.

It is worth noting that while the "return" keyword is in most other programming languages as well, Rust specifically has another way of writing this:

```rust
   pi_4 * 4.0
```

Just removing the return keyword and also removing the semicolon signals Rust that this is the output of that block. It behaves similar but not identical to return. For simplicity sake we'll keep using return for now which could be simpler to read and understand. However, this form is actually preferred in Rust code. We will learn it in depth later on.

The next thing we should focus on is how this function is called:

```rust
  let pi = calculate_pi();
```

Notice how it is used like it was a value. Rust will call the function, get the output and replace here the function by the output value. Then it will be stored in the variable "pi".

Everything else in this program is something that we already saw before. Let me go over a few things that might seem new:

```rust
  let mut pi_4 = 1.0;
   let mut divisor = 3.0;
   let steps = 20_000_000;
   for _ in 0..steps {
```

We are using "let mut" here to be able to change the values of pi_4 and divisor inside the function. Without that, Rust will not let us change the variable contents.

Another thing that might seem strange is the number 20_000_000. This is just 20 million, but the underscores are placed to make it easier to read. They have no meaning to Rust.

In the for loop, you'll notice that it says "0..steps"; so this basically makes the for count up to "steps". There's nothing special with this.

Finally, the for has as a variable an underscore "_". This is because this variable is not used. We only need N steps, but we use "divisor" to keep track of the position. Rust will emit warnings for unused variables. To avoid this, we use an underscore to signal Rust we don't need this value.

The result of the program is:
        pi: 3.1415926785904635

It's quite close to the real thing. Of course, if you increment the number of steps it will get closer, but the program will take a bit to run. For those cases we can make our Rust program faster by adding the "--release" flag to cargo run:

```
 $ cargo run --release
   Compiling learnrust v0.1.0 (/home/deavid/git/rust/learnrust)
    Finished release [optimized] target(s) in 1.00s
     Running `target/release/learnrust`
  pi: 3.1415926785904635
```