# Declaring variables

Before we can store values into a variable, we need to declare it. Declaring a 
variable means to tell Rust to create it, we will be explaining to it that this 
name is something that we will be using later.
<!-- TODO: I don't really like this explanation but can't think of anything else -->

Some programming languages don't require declaring, and just storing a value 
for the first time will do the trick. 
This is the case for Python, but not for Rust.

Anyway, it's not a big deal. Declaring a variable is very easy, we just have to use `let`:

```rust
   let x;
```

This comes from the wording in math of *"let x be a number..."*, so we use 
the keyword let to announce new variable names.

> A keyword is a reserved name by the programming language. 
> So this means that you can't have a variable named `let`, because `let let;` 
> would be confusing for Rust to understand your program.

A very simple program that makes use of variables could be:

```rust
   fn main() {
      let x;
      x = 4;
      println!("{}", x);
   }
```

You already know what it does. Prints `4`. That's all.

A variable can change their value at any time. 
For this we need to use `let mut x` instead of `let x`, so Rust knows that we 
want to mutate the value inside this variable later on (more on this later).

An example:

```rust
   fn main() {
      let mut x;
      x = 4;
      println!("{}", x);
      x = 6;
      println!("{}", x);
      x = 1;
      println!("{}", x);
   }
```

That will print `4`, then `6`, then `1`. 
It's just that the variable has changed the value it 
contained over time, as the program runs. 
Then the `println!()` just reads the value at that point in time and prints 
it to the terminal.

An important thing here, we have to use `let mut x` instead of just `let x` 
to tell Rust that this variable is "mutable", this means that we can change 
the contents later on.

We can declare one variable per line, each one with its own let:

```rust
let x;
let y;
let z;
```

But we can't declare these in a single let instruction. 
Other programming languages such as C++ allow separating them with commas, but not in Rust. 
We need one line for each one.

Most of the time we declare a variable we actually want to give them a value, 
because after all a value must always have a value at all times. 
So we can save a few lines and do it in one shot:
<!-- TODO: Why can we declare without assigning if it has to have a value anyway? -->


```rust
let x = 4;
let y = 3;
let z = 2;
```

So now this does two things at the same time, it declares, and it stores a value. 
To be clear, this is just shorter and nicer on our eyes. 
To the computer, it is exactly the same as if we declared first, 
and then we used another 3 lines to store the value. 
The program will be identical and will run equally fast.

Bottom line: do you prefer to see it on three lines all together or in six lines? 
Which one is easier to understand and read for you? 
Whatever is your response, that should be what you should write.

We don't write the programs for computers, we write them for humans to understand. 
If you think that a particular way is easier to read and understand, go with it.

As you'll start to notice by now, there are several ways to write a program 
(in fact, they're infinite). 
This might feel annoying. Worse even, there's no "right way". 
There are subjectively better and worse ways, but it's always up to humans 
to define what looks and feels better and come up with reasons for it.

Don't be bothered about this, don't think much about this. It is fine. 
Just write what it feels better to you personally, and you'll be grand. 
Over time, you'll learn more about how to make the code more readable, 
but that comes with experience. For now, let's focus on learning this.
