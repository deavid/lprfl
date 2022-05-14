# Incrementing and decrementing

There's a lot of stuff we can do with variables, but a very common thing is to use them to count, so an instruction that just says "add one to X" is quite handy:

```rust
 fn main() {
   let mut x = 4;
   println!("{}", x);
   x += 1;
   println!("{}", x);
   x += 2;
   println!("{}", x);
   x += 3;
   println!("{}", x);
  }
```

This program will output 4,5,7 and 10.

The other way around, subtracting, is also possible:

```rust
 fn main() {
   let mut x = 10;
   println!("{}", x);
   x -= 1;
   println!("{}", x);
   x -= 1;
   println!("{}", x);
   x -= 1;
   println!("{}", x);
  }
```

This one returns 10, 9, 8 and 7.

These operations are just a shorthand of this:


```rust
   x = x + 1;
   x = x - 1;
```

Which just means: read "x", add one, and write the result into "x" again; overwriting the previous content.

A reminder: these are instructions, not math equations. The equals sign stores on the left the result of evaluating the right side.

Most operations you can think of that take the same form have a shortened operator as well. For example doubling a number is just "x *= 2", which means "x = x * 2".
