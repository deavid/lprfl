# Enums!

We just saw structs, where we could have a type that contains many things at 
a single time. Structs are one of the basic blocks of programming. Enums are
probably their other half.

Consider the following, what if instead of storing many things at a time, we
wanted to store one of several options?

For example, a type that could be either an integer or a string. But not both.

The usefulness of this, as usual, is hard to see initially. But examples will
follow soon!

Imagine a datatype called `When` that could take any of the following:

* "Tomorrow": As in, the meeting will be tomorrow.
* 12 (hours): Even will happen in 12 hours.

In a struct, the problem is that we must have two fields and one must remain
empty:

```rust
struct When {
    name String,
    hours i64,
}
```

However, Rust doesn't allow them to be empty. So we might be compelled to use
zero as empty or the empty name as not set. But this isn't a good practice.
Using special values with special meanings is a bad idea in programming. We had
a long history with special values, and it usually ends in bugs.

Here's where an Enum helps:

```rust
enum When {
    name(String),
    hours(i64),
}
```

This can only take one of the values. Either is name or is hours.

We use them like this:

```rust
let tomorrow = When::name("tomorrow".to_string());
let hours_3 = When::hours(3);
```

> C++ has something similar to this, called union. But it's not safe to use. 
> Rust does have union as well and trust me, you don't want to use unions unless
> you really, really… really know what you're doing.

They also are useful to define names, for example a list of operation modes:

```rust
enum OperationMode {
    Read,
    ReadWrite,
    Append,
    Create,
    SelfDestruct,
}
```

> Don't confuse Rust enums with C++/Java enums. In these languages enums are 
> useful only to associate a number to a name. Rust enums are way more powerful.

You might say: "Well, nice, but I don't see myself using this ever. How is this
a critical thing to know for a beginner?"

And the answer is that Rust itself is plagued with two very popular enums. 
You can't avoid it. You'll have enums in your code, want it or not.

These enums are `Option<T>` and `Result<T>`. I'll discuss them now.

## Option 

Option is basically:

```rust
enum Option {
    Some(value),
    None,
}
```

And it is used when a value can be empty. But truly empty. Missing. Nil. Gone.

Because, you see, this is the empty string:

```rust
let text = "".to_string();
```

But it's not missing. It's an empty string. The value is not empty; it contains
an empty string. Weird, hah!

You want an empty value?

```rust
let text: Option<String> = None;
```

This one is empty.

And we can use this to have optional parameters in a function or a struct:

```rust
struct User {
    username String,
    real_name Option<String>,
}
```

## Result 

The other enum Result, is similar to this:

```rust
enum Result {
    Ok(value),
    Err(error),
}
```

This is used to have fallible operations. If something fails, it will return an
`Err(error)`. The `error` inside contains details on what failed. If it works,
returns `Ok(value)` where `value` contains the data we needed.

For example, consider a function to divide:

```rust
fn divide(a: f64, b: f64) -> f64 {
    return a / b;
}
```

However, divide by zero is an error. To be able to communicate this error we can
do:

```rust
fn divide(a: f64, b: f64) -> Result<f64> {
    if b == 0 {
        return Err(DivideByZeroError);
    }
    return Ok(a / b);
}
```

# Unwrap

As commented, Rust libraries are full of `Option` and `Result`. So it's very
easy to find something that returns these things.

For example, if we want to parse a string into a number:

```rust
let num: i64 = "1235".parse();
```

This doesn't work because `parse()` will return a `Result` and not an `i64`:

```rust
let num: Result<i64> = "1235".parse();
```

But this is inconvenient. If we know that the result is going to be Ok, we
can use unwrap:

```rust
let result_num: Result<i64> = "1235".parse();
let num: i64 = result_num.unwrap();
```

Or, in one line:

```rust
let num: i64 = "1235".parse().unwrap();
```

The problem with `unwrap()` is that **will make the program crash** if the result
is an error. So be careful when using this in places that are not guaranteed to
do as expected.

If you are okay with making the program crash at that point, consider using
`expect()` instead, which does the same but allows you to provide a message:

```rust
let num: i64 = "1235".parse().expect("failed to parse credit card number");
```

However, if making the program crash is not a good idea, you have to handle the
error:

```rust
let result_num: Result<i64> = "1235".parse();
match result_num {
    Ok(num) => {
        // .. handle here the parsed number ..
    },
    Err(e) => {
        // .. handle the error ..
    }
}
```

Rust forces you to choose what to do with the errors. There's no "default action"
but instead, compiler errors and warnings until you decide to `unwrap` or handle
it. If you don't decide, you'll get complaints.

Oh, and this works with option too!

```rust
// TODO: divide was returning a result - find another example!
let result_num: Option<f64> = divide(3.0, 1.2);
match result_num {
    Some(num) => {
        // .. handle here the parsed number ..
    },
    None => {
        // .. 
    }
}
```
