# Enums!

We just saw structs, where we could have a type that contains many things at 
a single time. Structs are one of the basic blocks of programming. Enums are
probably their other half.

Consider the following, what if instead of storing many things at a time, we
wanted to store one of several options?

For example, a type that could be either an integer or a string. But not both.

The usefulness of this, as usual, is hard to see initially. But examples will
follow soon!

## A form with options

Something that is easy to relate are real-world forms. Imagine you were trying
to get the bank to lend money to you, and they lend you a form.

In the form, the following appears:

> What is your current status?
>
> - (_) Are you working?
>   - Name of the company [_______]
>   - Field / Type of company [______]
>   - Type of contract?
>     (_) Permanent full time
>     (_) Permanent part-time
>     (_) Contractor
> - (_) Are you studying?
>   - Field of study [______]
>   - Years in school [__]
> - (_) Other, not studying or working.
>   - Please specify [________]

Or this:

> Marital Status:
> - (_) Single
> - (_) Married
> - (_) Other

These "single choice" options can be represented as Enums.

For example:

```rust
enum MaritalStatus {
    Single,
    Married,
    Other,
}
```

Then in code, we can choose one of the three options:

```rust
let status1 = MaritalStatus::Single;
let status2 = MaritalStatus::Married;
let status3 = MaritalStatus::Other;
```

Of course, you can put as many options as you want!

But, for "Other" we don't really know what happened here, so we want to have
the user to specify what they meant by "Other":

> Marital Status:
> - (_) Single
> - (_) Married
> - (X) Other  [_________________]

In Rust, we do:

```rust
enum MaritalStatus {
    Single,
    Married,
    Other(String),
}
```

Now, if we specify "Other" we need to put text with it:

```rust
let status3 = MaritalStatus::Other("divorced".to_string());
```

You can also have names and multiple values inside too:

```rust
enum MaritalStatus {
    Single,
    Married,
    Other{ status: String, observations: String},
}
```

```rust
let status3 = MaritalStatus::Other {
    status: "divorced".to_string(),
    observations: "in 2004".to_string(),
};
```

More complex enums are possible, as you can compose them with structs:

```rust
enum ContractType {
    PermFullTime,
    PermPartTime,
    Contractor,
}

struct Work {
    company_name: String,
    field: String,
    contract_type: ContractType,
}

struct Study {
    field: String,
    years: i64,
}

enum CurrentStatus {
    Work,
    Study,
    Other { specify: String },
}
```

This reflects the earlier form:

> What is your current status?
>
> - (_) Are you working?
>   - Name of the company [_______]
>   - Field / Type of company [______]
>   - Type of contract?
>     (_) Permanent full time
>     (_) Permanent part-time
>     (_) Contractor
> - (_) Are you studying?
>   - Field of study [______]
>   - Years in school [__]
> - (_) Other, not studying or working.
>   - Please specify [________]

## Enums in other languages

In C++, Java and most other languages, enums are much simpler and can't do
most of what Rust can.

In fact, they're just a fancy way of creating constant values associated to
numbers.

Imagine we're writing a library to open files, and we have a file mode:

```rust
enum FileMode {
    Read,       // -> 0
    Write,      // -> 1
    ReadWrite,  // -> 2
    Create,     // -> 3
    Append,     // -> 4
}
```

These are very similar to creating constants associated to numbers:

```rust
const READ: u16      = 0;
const WRITE: u16     = 1;
const READWRITE: u16 = 2;
const CREATE: u16    = 3;
const APPEND: u16    = 4;
```

Having an enum makes the creation simpler, and groups everything together nicely.

It starts counting at zero, but we can assign a particular number if we want. It
will continue counting from there:

```rust
enum FileMode {
    Read = 12,  // -> 12
    Write,      // -> 13
    ReadWrite,  // -> 14
    Create,     // -> 15
    Append,     // -> 16
}
```

If we have two numbers or more assigned, it works too:

```rust
enum FileMode {
    Read = 12,  // -> 12
    Write,      // -> 13
    ReadWrite,  // -> 14
    Create = 20,// -> 20
    Append,     // -> 21
}
```

In Rust, you can extract the actual number by casting to integer:

```rust 
dbg!(FileMode::ReadWrite as u16);
```

Another example:
```rust 
enum Numbers {
    One = 1,        // ->  1
    Two,            //     2
    Three,          //     3
    Four,           //     4

    FourtyTwo = 42, // -> 42
    FourtyThree,    //    43
}
```

## Other examples

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

## Understanding Rust Enums internals

> WARN: Technical info ahead! This describes how things work in memory 
> internally. This section is not required to understand, and feel free to skip.
> But for some readers, this might give some insight and understanding on Enums.
> Don't obsess into understanding everything; just a general overview here is fine.

In C++ (and Rust) we have something called "unions". A union is a type where
all contents will be stored in the same place in memory.

```rust
union MyData {
    integer: i64,
    float: f32,
    text: [char; 20],
}
```

These contents will be put one on top of the other, overlapping the same region
in memory.

If we did a struct like that:
```rust
struct MyData {
    integer: i64,
    float: f32,
    text: [char; 20],
}
```

In the memory we will have:

```
  integer float        text
[________][____][____________________]
```

All variables will be packed one after another.

However, if we were only going to use one of those at a time, we would be 
wasting a lot of memory of the computer.

Unions instead put everything in the same place:

```          
[________] integer
[____] float
[____________________] text
```

Or, more accurately:

```          
float integer text
[____]___]___________]
```

There are bytes in memory that will be shared across the float, the integer, 
and the text. Because of this, they use only the memory needed to hold the
largest variable that they can contain.

This makes unions very dangerous as if you write text and then read float you'll
get back basically garbage.

That's why, in Rust, reading unions is unsafe.

But if we knew what field we wrote, then we could actually read it without risk 
of getting back garbage.

Imagine we had constants to specify which field it is:

```rust
const FIELD_FLOAT: u8 = 0;
const FIELD_INTEGER: u8 = 1;
const FIELD_TEXT: u8 = 2;
```

And then we store this along with the union, inside a struct:

```rust
union MyData {
    integer: i64,
    float: f32,
    text: [char; 20],
}

struct MyDataSafe {
    field_written: u8,
    data: MyData,
}
```

Now, as long as we always keep the `field_written` up to date, we know which one
was used, so we can read confidently `data` without risk of getting back 
corrupted values.

We could write an implementation like this to ensure this is the case:

```rust
impl MyDataSafe {
    pub fn write_integer(&mut self, i: i64) {
        self.field_written = FIELD_INTEGER;
        self.data.integer = i;
    }
    pub fn write_float(&mut self, f: f32) {
        self.field_written = FIELD_FLOAT;
        self.data.float = f;
    }
    pub fn write_text(&mut self, t: [char; 20]) {
        self.field_written = FIELD_TEXT;
        self.data.text = t;
    }
}
```

And we could read "safely":

```rust
impl MyDataSafe {
    pub fn read_integer(&self) -> i64 {
        if self.field_written != FIELD_INTEGER {
            panic!("wrong field type");
        }
        unsafe {self.data.integer}
    }
    pub fn write_float(&self) -> f32 {
        if self.field_written != FIELD_FLOAT {
            panic!("wrong field type");
        }
        unsafe {self.data.float}
    }
    pub fn write_text(&self) -> [char; 20] {
        if self.field_written != FIELD_TEXT {
            panic!("wrong field type");
        }
        unsafe {self.data.text}
    }
}
```

Notice two things here. First, when we created the struct:

```rust
struct MyDataSafe {
    field_written: u8,
    data: MyData,
}
```

The layout in memory is:

```          
   float integer text
[_][____]___]___________]
 ^    
 --- field_written
```

Second, those constants:

```rust
const FIELD_FLOAT: u8 = 0;
const FIELD_INTEGER: u8 = 1;
const FIELD_TEXT: u8 = 2;
```

Are actually a regular C++ enum:

```rust
enum Field {
    Float,
    Integer,
    Text,
}
```

And the struct:
```rust
struct MyDataSafe {
    field_written: u8,
    data: MyData,
}
```

Is actually an equivalent of a Rust Enum!

```rust
enum Field {
    Float(f32),
    Integer(i64),
    Text([char; 20]),
}
```

That's what it actually is! Rust enums are kind of "safe C++ style unions".

They use only the memory needed for the biggest value possible of all options,
and they're safe. Plus an extra byte or two to hold which variant are we talking
about.

> In some cases, Rust is "too smart" and it's able to omit the extra byte needed
> to store the variant by using some tricks when compiling. But that is outside
> what I want to cover here.


