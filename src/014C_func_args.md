# Function arguments


But what if we wanted a configurable number of steps? We can do that!

Functions can accept data when they're called, like this:

```rust
  let pi = calculate_pi(100);
```

But for this to work we need to change the function signature:

```rust
  fn calculate_pi(steps: i64) -> f64 {
```

We have to specify between the parenthesis what is the variable name that will record the input value, as well as the type. Since steps won't have any decimal places, we use "i64" instead of "f64".

Therefore, a more complete description on the syntax for a function is:
        fn function_name( input_var_name: type ) -> return_type { ... }

Where the arguments can be omitted and the return type can be omitted as well if we don't use them.

It is possible as well to have many input values, just separating them by comma:
        fn function_name( input1: type1, input2: type2, input2: type2, ... ) { ... }

Each input variable needs to have its own type associated. We'll see more examples on this later on.

For now, let's see the full program changed:

```rust
fn calculate_pi(steps: i64) -> f64 {
   // https://en.wikipedia.org/wiki/Leibniz_formula_for_%CF%80
   // pi/4 = 1 - 1/3 + 1/5 - 1/7 + 1/9 - ...
   let mut pi_4: f64 = 1.0;
   let mut divisor = 3.0;
   for _ in 0..steps {
       pi_4 -= 1.0 / divisor;
       divisor += 2.0;
       pi_4 += 1.0 / divisor;
       divisor += 2.0;
   }
   return pi_4 * 4.0;
 }
 fn main() {
   for n in 1..10 {
       let steps = i64::pow(10, n);
       let pi = calculate_pi(steps);
       println!("pi: {} ({} steps)", pi, steps);
   }
 }
```

This program now has a loop in main() that will do 10 different 
calculations of PI at different precision levels.

Here's the output:

```
 $ cargo run --release
   Compiling learnrust v0.1.0 (/home/deavid/git/rust/learnrust)
    Finished release [optimized] target(s) in 1.00s
     Running `target/release/learnrust`
 pi: 3.189184782277596  (10 steps)
 pi: 3.1465677471829556 (100 steps)
 pi: 3.1420924036835256 (1000 steps)
 pi: 3.1416426510898874 (10000 steps)
 pi: 3.141597653564762  (100000 steps)
 pi: 3.1415931535894743 (1000000 steps)
 pi: 3.1415927035898146 (10000000 steps)
 pi: 3.1415926585894076 (100000000 steps)
 pi: 3.1415926540880768 (1000000000 steps)
```

It's important to run this with `--release` or it will take a bit too long to compute.

Also, it's always mesmerizing to see how more and more digits 
are getting *"stuck in"* as the number of steps grows.

I think this variation should be self-explanatory with maybe 
the exception to this line:

```rust
       let steps = i64::pow(10, n);
```

This computes \\(steps = 10^n\\), that is, the power of 10 raised to the "n". 
For this we use the function pow that exists inside the `i64` type. 
The double colon operator `a::b` is used to access the functions inside other 
libraries (or any content in fact).

Note that this function pow accepts two numbers as input and returns a number.