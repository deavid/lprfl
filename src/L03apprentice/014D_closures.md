# Introduction to Closures

In Rust, it is possible to create a function without using the `fn` keyword.

Why would we want to have a secondary way of creating these?

Well, sometimes we need a function for something that appears just once.

Sometimes these functions are very small.

But still, this doesn't explain why we need a secondary way of creating functions.

The real reason though, is that some functions require functions to be passed 
as arguments.

> *Wait what? You mean to call a function in a function argument?*

Yeah. Nope. That's not it. I mean the function itself, the name, before being called.

Imagine we want a function that counts stuff. But what it counts, is up to the
caller to decide.

For example, given the text:

```
The fox jumped over the lazy dog.
```

We want to count the letter `e`.

```rust
fn count_e(text: &str) -> i64 {
    let mut count = 0;
    for c in text {
        if c == 'e' {
            count += 1;
        }
    }
    count
}
```

Okay, good. But if we want to be able to count any character that we want?

Simple, add another argument!

```rust
fn count(text: &str, what: char) -> i64 {
    let mut count = 0;
    for c in text {
        if c == what {
            count += 1;
        }
    }
    count
}
```

Yeah, we don't see the problem yet. 

But now, imagine we want also to be able to count uppercase values.

What value do we give to this function, so it can compare against many values?

We can pass a function itself!


```rust
fn count(text: &str, what: fn(char) -> bool) -> i64 {
    let mut count = 0;
    for c in text.chars() {
        if what(c) {
            count += 1;
        }
    }
    count
}

fn uppercase(c: char) -> bool {
    if c >= 'A' && c <= 'Z' {
        return true;
    }
    return false;
}

fn vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn main() {
    let text = "The fox Jumped over the Lazy Dog.";

    let c = count(text, uppercase);
    println!("uppercase = {}", c);
    let c = count(text, vowel);
    println!("vowels = {}", c);
}
```

This prints:

```
uppercase = 4
vowels = 9
```

Because we can have such functions (and the Rust library contains a few of those)
we might need to create functions just to be able to call these.

And it can get cumbersome.

For example, Rust has a function ```char::is_alphabetic``` that returns `true`
if the character is a letter.

However, we might want to reverse it:

```rust
fn is_not_alphabetic(c: char) -> bool {
    !c.is_alphabetic()
}

fn main() {
    let is_not_alphabetic2 = |c: char| !c.is_alphabetic();
    
    dbg!(is_not_alphabetic('0'));
    // prints --> is_not_alphabetic('0') = true
    dbg!(is_not_alphabetic2('0'));
    // prints --> is_not_alphabetic2('0') = true

    dbg!(is_not_alphabetic('a'));
    // prints --> is_not_alphabetic('a') = false
    dbg!(is_not_alphabetic2('a'));
    // prints --> is_not_alphabetic2('a') = false

}
```

In the above code we can see already the new way of creating functions.

These are called "Closures" in Rust. In other languages these are called 
"Lambdas".

A closure has the syntax:

```rust
|argument1:type1, argument2:type2| { code; code; code; return value; }
```

You can have a function that accepts no arguments:

```rust
|| { code; code; code; return value; }
```

And as usual, you can omit the `return` by skipping the semicolon:

```rust
|| { code; code; code; value }
```

Finally, if it's only one statement, we can skip the curly brackets altogether.

```rust
|| value
```

For example:

```rust
|| 3.51
```

Or:

```rust
|| println!("Here's a text")
```


These are all functions. But you need to store them into a variable or pass them
into a function in order to use them. This syntax, by itself, does nothing.

```rust
// This makes sense: (gives it a name)
let f1 = || println!("Here's a text");
// This does not:
|| println!("Here's a text");
```

Or a more advanced example could be:

```rust
let c = count(text, |c| matches!(c, 'a'|'e'|'i'|'o'|'u'));
```

We just created a function that returns true for vowels in less than one line.

This is quite powerful, and it's okay to feel overwhelmed on this for now.
I won't be writing code like this, so you don't need to truly understand all 
these for now.

But it's important that we know that these exist.




