# Incrementing and decrementing

There's a lot of stuff we can do with variables, but a very common thing is 
to use them to count, so an instruction that just says 
"add one to X" is quite handy:

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

This program will output `4`, `5`, `7` and `10`.

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

This one returns `10`, `9`, `8` and `7`.

These operations are just a shorthand of this:


```rust
   x = x + 1;
   x = x - 1;
```

Which just means: read "x", add one, and write the result into "x" again; 
overwriting the previous content.

A reminder: these are instructions, not math equations. 
The equals sign stores on the left the result of evaluating the right side.

Most operations you can think of that take the same form have a shortened 
operator as well. 
For example doubling a number is just `x *= 2`, which means `x = x * 2`.

## The modulo or remainder operator

Something that is little known is that in programming we have an operator
to get the remainder of a division.

For example, if we do `12 / 10 = 1`, but the remainder of that division is 2.

To get the remainder we use the modulo operator `%`:

```rust
let v = 35;
let b = 10;
let remainder = v % b;
println!("{} % {} = {}", v, b, remainder);
```

This operator is useful when we want a value that wraps around a particular
number. For example when counting time, seconds goes to 60, then back to zero.

So if we do:
```rust
let time_sec = 100322;
let seconds = time_sec % 60;
```

It will make the `seconds` variable to be on the range 0…60, excluding 60.

This, with the use of divisions, it can make for a very easy code to transform
a lot of seconds into hours, minutes, and seconds. Here's the recipe.

```rust
let time_sec = 100322;
let seconds = (time_sec) % 60;
let minutes = (time_sec / 60) % 60;
let hours =   (time_sec / 60 / 60) % 24;
let days =    (time_sec / 24 / 60 / 60) % 30;
let months =  (time_sec / 30 / 24 / 60 / 60) % 12;
let years =   (time_sec / 12 / 30 / 24 / 60 / 60);
```

> NOTE: The operator actually computes the modulo, not the remainder, which
> is almost the same, but not identical. For negative values it behaves 
> differently.