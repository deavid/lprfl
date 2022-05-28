# A bit more detailed explanation on functions

A function has always two parts, the declaration, and the call.

The declaration is this part with the `fn` keyword:

```rust
fn one() -> i64 {
    return 1;
}
```

Where the call is this code:

```rust
add_one();
```

You can have functions declared that are not called (you'll get a warning), but
you cannot have a function called that is not declared.

Also, we can call the same function as many times we want. It is the whole point
of functions to be able to be called thousands of times.

To execute/call a function, we write the name and follow it by parenthesis:

```rust
add_one();
```

If we forget the parenthesis, the function will not be called:

```rust
add_one;  // WRONG!
```

The return value of a function can be used as a regular value:

```rust
let two = 1 + one();
let three = 1 + one() + 1;
```

Rust will execute the code in the function, get the return value and "replace"
the function call with that value:

```rust
let two = 1 + (1);
let three = 1 + (1) + 1;
```

Function names are actually variables. We won't be using this "feature" in this
book, but it's good that you know this is possible:

```rust
let one_fn = one;

let two = one_fn() + 1;
```

These are called function pointers. This is why it's important you don't forget
the parenthesis, as the error might get you confused. Rust will understand that
you want to store the function code somewhere[^1].

------
[^1]: Actually they are function pointers, they point to the memory address 
where the code sits. But as usual, this goes outside what I want to explain.

## They're black boxes

We need to understand functions as a black box. Something goes in, something
goes out. We don't care that much on what happens inside when we call it.

You can imagine them as a microchip, that has pins for getting data in, and
pins that give data out:

```
              ┌——————————————————┐
 number_1  -->│                  │
              │                  │
              │  add_numbers()   │---> result_number
              │                  │
 number_2  -->│                  │
              └——————————————————┘
              
```

This equates to this function:

```rust
fn add_numbers(number_1:i64, number_2:i64) -> i64 {
//     │          │             │              ↳ Return value type
//     │          │             ↳ Second argument name and type
//     │          ↳ First argument name and type
//     ↳ Function name
```

Remember: You can have functions that don't return a value. A function that
accepts one, two, or many arguments. Or both.

This function doesn't need any arguments, and doesn't return anything:

```rust
fn help() {
```

This one accepts one argument but doesn't return anything:

```rust
fn manual(page: i64) {
```

This one doesn't have arguments, yet has a return value:

```rust
fn calculate_pi() -> f64 {
```




## Omitting "return"

In Rust, most of the time we don't need to add `return` to return a value.
If the last statement of the function lacks a semicolon, implies return.

For example:

```rust
fn add_numbers(a: i64, b: i64) -> i64 {
    let c = a + b;
    return c;
}
```

Can be simplified as:

```rust
fn add_numbers(a: i64, b: i64) -> i64 {
    let c = a + b;
    c
}
```

Which in turn, can be simplified again:

```rust
fn add_numbers(a: i64, b: i64) -> i64 {
    a + b
}
```

Nice!

## Types as matching shapes

Remember those kids toys where you have to put the rectangle on the rectangular
hole, and the circle in the circular hole?

Data types are somewhat like this.

Imagine that the types were shapes:
* Integers are squares: ■
* floating points are circles: ◉
* Strings are parallelograms: ▰

Now, when you make a function, the arguments are like "sockets" you must connect:

```
              ┌——————————————————┐
 number_1 ○-->│                  │
              │                  │
              │  add_numbers()   │◉---> result_number
              │                  │
 number_2 ○-->│                  │
              └——————————————————┘             
```

A function that accepts several types would look like this:

```
   ┌————————┐
--▢|        |
   |        |
--◎|        |
   |        |
--▱|        |
   └————————┘             
```

You need to provide the right shapes to that box to get it working.

Or in specific terms, you need to provide the exact types the function is 
asking for to be able to call it.

You cannot call a function that accepts a `f64` with an `i64`.

In the same sense, if your code is expecting an `i64` as a result, the function
must return that:

```rust
fn calc_pi() -> f64 {
    // code here...
    return pi;
}

fn main() {
    let pi:i64 = calc_pi(); // Error! calc_pi returns f64 but the variable is i64.
}
```
