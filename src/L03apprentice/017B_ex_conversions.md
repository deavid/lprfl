# Exercise: explicit conversions

Now, use what you learn about doing `x as f64` to fix the following program:

```rust
let x = 4
let y = 2.0;
let z = x / y;
dbg!(z);
```

We could solve in two directions:

```rust
let x = 4
let y = 2.0;
let z = x as f64 / y;
dbg!(z);
```

Or:

```rust
let x = 4
let y = 2.0;
let z = x / y as i64;
dbg!(z);
```

