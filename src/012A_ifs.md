# What If...?

... it is raining outside? You should get an umbrella, right?

This is basically a conditional: If it's raining, get an umbrella. 
We have these in Rust and use the keyword "if". 
The name may not sound original at all, but helps to read the program 
as if it were English.

For example:

```rust
fn main() {
   let apples = 6;
   if apples > 1 {
       println!("You have many apples!");
   }
 }
```

This program will print `You have many apples!` only if the 
`apples` variable is bigger than one.

Of course, it only changes if you manually go and change the variable value. 
Don't go that fast! We'll see something useful soon.

The syntax is:
```rust
if condition { 
    // ... what to do if the condition is true ... 
} else { 
    // ... what to do if it's not ... 
}
```

Condition can be anything that is either true or false. Some examples:
* `apples == 1` → if "apples" is exactly 1.
* `apples != 1` → if it's not 1. Any other value except one will do.
* `apples >= 1` → if it's greater or equal to 1.
* `apples < 1` → if it's less than 1.
* `apples <= 1` → if it's less than or equal to 1.

Notice how I told you the `if` has an else part, but I did not 
write it on the above program. That's because it's optional. 
If we only care about the part it's true, the `else` is not needed.

However, we cannot do the reverse. You cannot have an `else` without an `if`. 
If we need to target when the condition is false, we need to reverse the 
condition, so it returns true when we need to.

Let's see an example with an `else`:

```rust
fn main() {
   let apples = 1;
   if apples > 1 {
       println!("You have many apples!");
   } else {
       println!("Please go to the supermarket.");
   }
 }
```

Ok, enough of this. I guess it's too simple, and we need to spice it up with...
