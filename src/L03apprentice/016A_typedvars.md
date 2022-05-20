# Back to declaring variables... with type

Now that we have a rough idea on types, we can do more things with the "let" keyword.

For example, we can tell Rust to use an i64 for our number:

```rust
   let x: i64 = 2;
```

That will force the selection of this type. But wait... 
if we didn't specify a type, and Rust needs one to know how much memory to use... 
how did Rust make the previous examples work?

As a reminder, we did:

```rust
   fn main() {
      let x;
      x = 4;
      println!("{}", x);
   }
```

And we did not specify a type here. 
In fact, this could be i8, i16, i32, i64 or even u8, u16, etc. 
All these will work.

Which one did select Rust and why?

Did we care which one to use when writing this? No. And neither does Rust.

Rust will do something similar to what I'm telling you to do. `i64` is a 
perfectly fine type for most uses, so just go with that. Rust is probably doing 
the same thing here: It seems you want a number; does i64 work?
Ok, so that one then; that looks good[^1].

Rust does a lot of guesswork to select the type, and most of the time we don't 
need to care about this. In a few particular times it might be confused with too 
many options that are too different, and might complain. In such cases, we need
to help it by defining the type. Aside from that we'll avoid setting the types
and let Rust do its magic.

[^1]: Note here that Rust might guess differently and prefer an i32 over an i64 
"because it's smaller, and it still works". As we didn't care about which type, 
it's pointless to try to know what Rust exactly does. 
If we want to be sure it's an i64, we just write it down.
