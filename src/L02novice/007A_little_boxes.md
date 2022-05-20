# Variables are like little boxes that store things inside

Printing some texts gets boring very fast. A computer does calculations 
for us, that's what makes them useful. 

It is possible to do simple calculations and print them, but this 
doesn't have much mystery either:

```rust
 println!("{}", (2 * (1 + 5) + 3 / 7) / 2);
```

This will print 6, because it works with integers (whole numbers). 
Just like a regular calculator but without decimal points.

To get a decimal value just use all numbers as decimals, even if it's `2.0`:

```rust
println!("{}", (2.0 * (1.0 + 5.0) + 3.0 / 7.0) / 2.0);
```

This prints `6.214285714285714`, just like a calculator with probably more digits.

You'll notice that Rust will error out if you mix numbers without decimals with 
numbers that do have decimals. We'll go into more detail on this soon. 
For now, just remember that if you want decimal points, they need to be in all numbers.

Again, this gets boring very easily. We need to spice it up with… variables!

If you did math school before, you probably remember equations. For example:

\\[ 1+x = 2x+5 \\]    

These have an *unknown* that is \\(x\\) that must be resolved for. In this case it would be \\(x=-4\\).

> If you hate math and equations, do not worry. 
> This is just to anchor into something you know. In coding we don't do equations. 
> The computer is the one doing math, not us.

So do we agree that \\(x\\) is "something" whose value is \\(-4\\), right?

Ok, hold on to that idea. That is the same for variables. What is not the same is:
* We don't have equations. We have instructions. Instructions do things like 
  storing something or printing in the terminal.
* We don't have *unknowns*, and \\(x\\) in this example is an *unknown*. 
  In programming, we have variables. An *unknown* is something that we don't know (yet), while 
  a **variable** is something that **always has a value, and we know it**.

In Rust, we would do instead:

```rust,no_run,noplayground
x = -4;
```

Here `x` is the variable. It can be any name: `a`, `j` or even a word `animals`. 
Heck, even several words together: `number_of_legs_in_a_dog`. 
(If you're a cat person, feel free to set `number_of_legs_in_a_cat` instead)

```rust,no_run,noplayground
number_of_legs_in_a_cat = 4;
```

I did it for you. You can thank me later.

The equals part is an operation, it means "to store". 
It actually saves the value on the right (number 4) into the name of the left.

> Variables should be named in `snake_case`, meaning they should be all 
> lower case, contain only English characters, and it should start by letter.
> Spaces are not valid[^1].

So now we do have a name `x` or `number_of_legs_in_a_cat` whose value is 4.

The semicolon marks the end of the instruction. This serves to tell Rust that 
this line is something that needs to be executed, and to avoid confusion with 
the next line. If we forget the semicolon, it will think that two lines are 
in fact one and will get confused.

If we translate this line into English it will say:
>  Please store the value four into the variable named 
> `number_of_legs_in_a_cat`, end of instruction.

<!-- TODO: Like a telegram where it said "stop" -->

And from this point, the computer would remember that this name equals to 4, 
so we could print it later:

```rust
println!("{}", number_of_legs_in_a_cat);
```

This is actually the same as doing:

```rust
println!("4");
```

So, if this is the case, why do we complicate this so much?

Well, variables will help us do much more complex programs, as they 
can keep track of what was the user input or other data that 
we are managing inside. 
It will make sense soon, so bear with me for now.

----

[^1]: Actually it's more flexible than that, Rust also allows some emojis; 
but for simplicity, let's use only English alphabet.
