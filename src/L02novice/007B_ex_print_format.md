# Exercise: Printing and Formatting

So we saw that `println!()` does print lines in the console. Let's practice a 
bit and see other fun ways of using `println!()`.

First, we'll create another program which we'll call `print.rs`.

Go ahead and create a new file in `learnrust/src/bin/print.rs`.

Add the contents:
```rust
fn main() {}
```

Now open `Cargo.toml` and add:

```toml
[[bin]]
name = "print"
```

Your `Cargo.toml` should now look like this:

```toml
[package]
name = "learnrust"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at ...

[[bin]]
name = "sample1"

[[bin]]
name = "print"

[dependencies]
```

We can now run this program with `cargo run --bin print`:

```console
$ cargo run --bin print
   Compiling learnrust v0.1.0 (/home/deavid/git/rust/lprfl/learnrust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/print`
```

As you can see, aside of compiling the program, this does nothing. Pretty 
obvious when you see the code, right? It's almost empty.

## Printing banners of text

Say we want to have some banner when the program starts. That would require 
multiple lines and will describe what is this program.

While it's possible to have multiple lines in a single `println!()`, I don't 
recommend this. It's hard to read and hard to write.

Instead, we'll just use one `println!()` per line:

```rust
{{#include ../learnrust/src/bin/print.rs:header}}
```

> HINT: On the piece of code above there should appear a "play" button if your
> browser supports JavaScript. If you click it, you can see the output of this
> program. The play button appears in most of the pieces of code in this book,
> so you don't need to try every single program out.

## Placeholders

Now, printing text is nice. But soon we will want to replace a part of the text
with something we can replace later on.

For example, consider this piece of code:

```rust
{{#include ../learnrust/src/bin/print.rs:sum}}
```

The placeholder is `{}`. These two characters tell Rust that "here we want to
put a value". Then after the text, we add a comma and write down the value we 
want. It can be a number, text, or like in this case, an operation.

Rut basically will do the following steps:

1. It will compute the sum we wrote:
```rust
    println!("The sum of 2 + 3 is {}. Isn't that great?", 5);
```
2. Replaces the placeholder with the value:
```rust
    println!("The sum of 2 + 3 is 5. Isn't that great?");
```
3. Prints the text into the console:
```console
The sum of 2 + 3 is 5. Isn't that great?
```

We can use more than one placeholder, just add more values at the end separated
by commas:

```rust
{{#include ../learnrust/src/bin/print.rs:mult}}
```

This prints: `And 4 * 3 is 12. Fantastic.`

The values must appear in the same order as the placeholders. The first value
corresponds to the first `{}` from the left, the second value to the second `{}`.
You get the idea.

If it's empty, without text or values, it will just print a new line. This can 
be useful to separate parts of the output:

```rust
{{#include ../learnrust/src/bin/print.rs:newline}}
```

But be careful! We cannot put values without a placeholder. This **DOES NOT WORK**:
```rust
println!(5 * 10);
```

> HINT: Try to click the play button for the above code. It should show you the
> error that Rust gives for this particular code.

Instead, if we just want a value, we need to add a placeholder for it:

```rust
println!("{}", 5 * 10);
```

And of course, the number of placeholders must be equal to the number of values
we added. If too many placeholders appear, it will error out:

```rust
println!("{} this placeholder does not have a value->{}", 5 * 10);
```

And in the reverse, if we have too many values, it is also an error:

```rust
println!("{}", 5, 10, 5 * 10);
```

## Understanding Rust errors

Again, please read the errors carefully. They explain a lot of what is wrong.

For example, on the above code we had:

```console
error: 2 positional arguments in format string, but there is 1 argument
 --> src/main.rs:4:11
  |
4 | println!("{} this placeholder does not have a value->{}", 5 * 10);
  |           ^^                                         ^^   ------

error: could not compile `playground` due to previous error
```

**ALWAYS** read the errors from top to bottom! Start from the first line and 
keep reading line by line, like a book. I know this sounds obvious, but in the 
console, we tend to read just the last line:

```
error: could not compile `playground` due to previous error
```

In fact, most people just reads:
```
error: could not compile
```

And forgets about anything else. I swear. Some people just reads `error` and 
gets puzzled. Don't be like them. Read the errors from top to bottom.

Let's break down the error in small parts to understand what it means:

```
(...) 2 positional arguments in format string (...)
```

By "positional arguments", Rust means that we wrote two placeholders. And 
"format string" is the text to print `"{} this placeholder does not have a value->{}"`.

```
(...), but there is 1 argument
```

And the "argument" is the value on the right of the text, in our case `, 5 * 10`.

So, let's read again:
```
error: 2 positional arguments in format string, but there is 1 argument
```

Therefore, what Rust means here is:

> You wrote two placeholders `{}` in the text, but you only provided one 
> value on the right: `, 5 * 10`

And if we keep reading (we must keep reading!), we have a nice help of what's
happening:
```console
 --> src/main.rs:4:11
  |
4 | println!("{} this placeholder does not have a value->{}", 5 * 10);
  |           ^^                                         ^^   ------
```

Notice the first line: `--> src/main.rs:4:11`. This tells us which file has the
problem (in my case I executed this in the browser, so the file does not match
with what we have in our project). The `:4:11` means **line 4**, character 11. 
Rust is telling us exactly (down to which character) has the problem. This is 
super useful!!

In the next lines, see how it is showing the line of the program that is 
affected. And the characters bellow (`^^` and `-----`) are underlining the parts
that are contributing to this error.

> **Error messages are super helpful!** Keep reading them[^1]. They will explain to
> you how to fix your program.

[^1]: Each time that you reach out for help for an error and turns out that you
didn't read the error message carefully, put one coin into your 
"Haven't read the error message" jar. Use the contents to invite your helper
friends to a drink from time to time.

## Formatting numbers

Now that I've been a pain in the neck for long enough about errors, let's do
something interesting again. Sorry about that, but you'll thank me later.

Let's say we want to print some decimal numbers. But sometimes, they're ugly:

```rust
println!("5.0 / 3.0 = {}", 5.0 / 3.0);
```

This prints `5.0 / 3.0 = 1.6666666666666667`[^2], and it might be difficult
to read. Wouldn't it be nice if we could round to two decimals?

Let's see a few samples:

```rust
{{#include ../learnrust/src/bin/print.rs:rounding}}
```

Click on the play button and see how they are rounded to different decimal 
places. Also, notice that Rust rounded the number, so 1.6 becomes 2.

> You can read the full documentation on formatting in here: <https://doc.rust-lang.org/std/fmt/>

We can also do leading zeros:
```rust
println!("10 / 2 = {:04}", 5);
```

And we can mix both together, but be careful because the leading zeros count
all digits and dot. So you need more "leading zeros" to cover for the decimal
places:
```rust
println!("5.0 / 3.0 = {:07.2}", 5.0 / 3.0);
```

[^2]: Note that, depending on your computer and Rust version you might see more or
less decimals; Also the last digits might change. This is normal. Don't
think much about it. The explanation is an advanced concept that I hope to get
at later stages of this book. For now, let's move on.

## Done!

That's it for now! Feel free to play around and try yourself.

Here's the program completed:

```rust
{{#include ../learnrust/src/bin/print.rs:all}}
```

Remember to write save it in `learnrust/src/bin/print.rs` and edit `Cargo.toml`
as I explained on top. 

Then run it with: `cargo run --bin print`

```console
$ cargo run --bin print
   Compiling learnrust v0.1.0 (/home/deavid/git/rust/lprfl/learnrust)
    Finished dev [unoptimized + debuginfo] target(s) in 1.11s
     Running `target/debug/print`

#############################################################
#                                                           #
#                This is a PRINT program                    #
#                                                           #
#############################################################

Summary: This program demonstrates different
         ways of printing text

The sum of 2 + 3 is 5. Isn't that great?
And 4 * 3 is 12. Fantastic.

5.0 / 3.0 = 1.6666666666666667
5.0 / 3.0 = 1.667
5.0 / 3.0 = 1.67
5.0 / 3.0 = 2

10 / 2 = 0005
5.0 / 3.0 = 0001.67
```
