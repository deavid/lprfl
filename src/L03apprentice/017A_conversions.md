# Converting types with explicit conversions
As you can imagine, it is pretty common having a mix of types. With so many data 
types it is easy to end up having an operation that needs to mix them in some
way, and Rust does not allow that.

We will avoid the majority of those problems by using as few types as 
possible, just i64 and f64. But it will happen at some point.

For example, consider a simplistic example:

```rust
   let apples: i64 = 2;
   let apple_weight: f64 = 0.35;
   let total_weight: f64 = apples * apple_weight;
```

In this case we could define `apples` to be a float, and it will work,
but this could appear differently where this is not so simple, 
and we need "apples" to be an integer regardless.

To make this work we can simply convert "apples" variable in the spot, 
right where we need it:

```rust
   let apples: i64 = 2;
   let apple_weight: f64 = 0.35;
   let total_weight: f64 = apples as f64 * apple_weight;
```

By adding `as f64` we're making what is called an *"explicit conversion"*. 
Rust will convert the integer into a floating point on the spot 
and will use that on the calculation of total_weight.

The variable `apples` is still an integer after that line. If we keep using 
it as a float we would need to keep converting every time. 
So if it is needed in all places as a float, 
maybe it is better to just make it a float from the start.

Explicit conversions have their limitations though. 
It works only between some types that are easy for Rust to convert. 
You can't use it to convert to or from strings 
(there are other conversion types for that), or between string types.

Doing the reverse, converting a float into an integer, 
will drop any decimal places. So `2.99999 as i64` is just `2`. 
It **will not round the number** to 3. 
There are other ways for rounding numbers.

The reason why explicit conversion works for so few cases and its behavior:
These conversions are usually supported by your CPU and are really fast. 
Rounding numbers or converting an integer into a string will take several 
iterations in your CPU to complete. Those are also very fast, 
just not as fast as the explicit conversion. Rust is trying to signal you where 
your program might be consuming more resources, which is helpful, 
but we don't need to be obsessed with speed. 
Rust is very fast, and so is your computer (even if old).
