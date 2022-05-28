# Simple explanation on &str vs String

We will use these soon, so they need a bit of an introduction. 
They both store a sequence of characters.

It all boils down to what I commented early on. Rust (and the computer itself) 
needs to know how big a variable is. 
But a string can change in size very abruptly.

Because of this, Rust cannot know the size of most strings until the program is 
actually executed. This poses a problem, because without a rigid size in memory
we cannot have variables that contain text. At most, we can have a variable 
that stores a single character, but this isn't very useful.

So what do we do? Well, instead of having the text itself in a variable 
we store where the string is and its length. This is the type `&str`.

While this is the fastest way for the computer to do this, and convenient for 
some parts of the program, it's certainly very hard to manipulate. It's nearly 
impossible to change the contents of a `&str` variable.

It's because of that inconvenience that String exists. The String type 
encapsulates and automates string changes.

So, in general, when a text never changes, `&str` is just fine. But if we want 
to store the user's input, String might be better.

In Rust these two are very easy to convert from one to another:

```rust
 fn main() {
   let s1 = "This is a &str";
   let s1a = s1.to_owned(); // This converts &str to String.
   let s2 = String::from("This is a String");
   let s2a = s2.as_str(); // This converts String to &str.
 }
```

We'll go on more details later on, but for now we will basically use whatever 
type Rust is happier with. Depending on what we're doing we will probably need 
one or the other and convert accordingly.

Note that there are lots of functions to create a `String` from a `&str`:

```rust
let s1 = "text".to_owned();
let s2 = "text".to_string();
let s3 = String::from("text");
```

They all do the same. In programming, we try to avoid having two things for
the same, but in this case, Rust has to have three variants, because they can
be used in different contexts.

* `x.to_owned()` means to create an owned version of the variable; that is, to
  remove the `&` character (borrow), which we will be learning soon.
* `x.to_string()` means to convert something into a string. This is equivalent
  to `format!("{}", x)`.
* `String::from(x)` means to create a string from something else. It can only be
  used with &str, String or other types that actually represent text or at least
  a character.

Is any of those faster? Maybe. Probably String::from(). But I bet it cannot be
measured in 99.9% of real Rust programs. The difference is too small to actually
matter.
