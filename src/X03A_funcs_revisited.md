# A bit more detailed explanation on functions

(…) (roughly explain a type, why the parentheses produce the call of the 
function, what happens if you forget the parentheses when calling, etc.)

## Omitting "return"

In Rust, most of the time we don't need to add `return` to return a value.
If the last statement of the function lacks a semicolon, implies return.

For example:

```rust
fn add_numbers(a: i64, b: i64) {
    let c = a + b;
    return c;
}
```

Can be simplified as:

```rust
fn add_numbers(a: i64, b: i64) {
    let c = a + b;
    c
}
```

Which in turn, can be simplified again:

```rust
fn add_numbers(a: i64, b: i64) {
    a + b
}
```

Nice!