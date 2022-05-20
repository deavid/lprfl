# Match!

Have you ever seen code like this?

```rust

let choice = 1;

if choice == 1 {
    guess_game();
} else if choice == 2 {
    convert_units();
} else if choice == 3 {
    test_unwrap();
} else if choice == 4 {
    quit();
}
```

Have you wondered if that could be written in a clearer way?

Then you're in luck! Presenting the `match`:

```rust
let choice = 1;
match choice {
    1 => guess_game(),
    2 => convert_units(),
    3 => test_unwrap(),
    4 => quit(),
    _ => {},
}
```

As you can see, `match` has a beautiful syntax. Concise, yet powerful, that goes
to the point.

What `match` does is exploring different options that a value can take. So, if
we do:

```rust
let choice: i64 = 1;
match choice {
    // ...
}
```

It is going to consider what values `choice` can take. Since it is an `i64`, it
can take billions of different numbers, either positive or negative, and for
each one we can specify what to do in each case.

But it does have a catch, as `match` requires us to consider **all** 
possibilities. That is, we need to cover all possible numbers and specify an
action for it.

To avoid specifying all options one by one, we can have a default by using the
underscore `_ => ...`:


```rust
let choice: i64 = 1;
match choice {
    1 => do_something(),
    _ => panic!("Invalid option!"),
}
```

It can understand ranges too:
```rust
let choice: i64 = 1;
match choice {
    1 => do_something(),
    2..=10 => panic!("This option is not supported yet"),
    _ => panic!("Invalid option!"),
}
```

And it can understand enums too!

```rust
enum Choice {
    GuessGame,
    ConvertUnits,
    TestUnwrap,
    Quit,
    InvalidChoice,
}

let choice = Choice::GuessGame;
match choice {
    Choice::GuessGame => guess_game(),
    Choice::ConvertUnits => convert_units(),
    Choice::TestUnwrap => test_unwrap(),
    Choice::Quit => quit(),
    Choice::InvalidChoice => {},
}
```

And this is useful to handle errors:

```rust
let input_number = "12345"; // provided by user
let res_number:Result<i64> = input_number.parse();

match res_number {
    Ok(number) => println!("your number is {}", number),
    Err(e) => println!("There was an error parsing your number: {:?}", e),
}
```

A bit more advanced usage could be:

```rust
let input_number = "12345"; // provided by user
let number:i64 = match input_number.parse() {
    Ok(number) => number,
    Err(e) => {
        println!("There was an error parsing your number: {:?}", e);
        return;
    }
};

println!("your number is {}", number);
```

