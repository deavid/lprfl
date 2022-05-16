# Borrowing

This is most probably the most difficult concept in Rust. Borrowing is very 
specific to Rust only, and it's the main reason so many people abandon learning 
Rust. And also it is why so many people say Rust is difficult.

Learning this is what sets apart developers from Rust Developers.

Are you ready?

- When you see things like `x`, `y`, `f64`, etc. It means that you own the 
  variable, meaning that you can do whatever: read, write and free their memory.
- If you see `&x`, `&y`, `&f64`, etc. Means that you don't own it, it was lent
  to you temporarily. You can only read their contents, but most of the time you
  can also copy the contents elsewhere too.
- Finally, if you see `&mut x`, `&mut y`, `&mut f64`, etc. Is as before, but
  you can also write to them. With these you can do mostly everything except
  free their memory.

That's it. `&` is read-only access, `&mut` is read and write access, and if 
nothing appears, it's full access.

Wait, you thought this was going to be more difficult? I wonder why.

Ok, jokes aside, now comes the difficult part.

Rust will ensure the correctness of our programs in a way that feels "too much",
it will analyze and prove correctness to a level that no other language that I
know does.

This makes the Rust compiler very picky. There's lots of stuff that it doesn't
like. And it's hard to make the compiler happy.

Maybe you experienced this already. Or maybe I did a good job up to now keeping
you away from this. Now it's time to start introducing (very gently) to 
Ownership and Borrowing in Rust.

When we store a value into a variable, it uses some amount of memory 
of the computer. It seems kinda obvious that 100 numbers will use more memory
than 5 numbers.

And memory needs to be freed. Because if we keep asking for memory and never
returning it, our computer might run out of memory at some point.

Therefore, Rust must allocate (ask for memory) when we put a new variable, and
it must free the memory when it's no longer needed.

```rust
fn some_function() {
    let x: i64 = 12345; // <-- memory gets allocated here.
    println!("using the variable: {}", x);
} // <-- memory gets freed here, when the function ends.

fn main() {
    some_function();
}
```

Now, this is the basic of what Ownership means: when we create something, we 
own it, and we're responsible for freeing it when we're done with it.

Rust does this for us in a very natural way. So much that we could ignore
this up to this point.

The problem with this approach is that, when we pass a variable to a function,
the function now owns that variable. And this means that the function will
free this variable when it ends, we could not use it anymore.

Sounds weird, but it will seem clear here:

```rust
fn print(x: i64) { // <-- x ownership is captured when calling the function.
    println!("Value is: {}", x);
} // <-- memory gets freed here, when the function ends.

fn some_function() {
    let x: i64 = 12345; // <-- memory gets allocated here.
    print(x); // <-- we call the function, but 'x' is now owned by 'print'.
    println!("using the variable: {}", x); // <-- 'x' no longer exists!!
}

fn main() {
    some_function();
}
```

But ---I hear you asking---, if I run the above code it works.

Yeah, I know. But this one does not:

```rust
fn print(x: String) { // <-- x ownership is captured when calling the function.
    println!("Value is: {}", x);
} // <-- memory gets freed here, when the function ends.

fn some_function() {
    let x: String = "text".to_string(); // <-- memory gets allocated here.
    print(x); // <-- we call the function, but 'x' is now owned by 'print'.
    println!("using the variable: {}", x); // <-- 'x' no longer exists!!
}

fn main() {
    some_function();
}
```

Some data types are so simple that Rust can just copy them around. But text
in the other hand, is a bit more complicated and that trick no longer works[^1].

Now, to make this work is pretty easy. Because `print()` is only reading, we
only need read-only access. We can use the ampersand to borrow temporarily
the variable:

```rust
fn print(x: &String) { // <-- we only get a "reference" to 'x'. We don't own it.
    println!("Value is: {}", x);
} 

fn some_function() {
    let x: String = "text".to_string(); // <-- memory gets allocated here.
    print(&x); // <-- now we only lent 'x'. We keep ownership.
    println!("using the variable: {}", x); // <-- 'x' does exist here and works.
} // <-- 'x' memory gets freed here.

fn main() {
    some_function();
}
```

As you can see, I only added two `&` in the code, and we don't have the 
problem anymore.

Now we also have `&mut`, and this one is fascinating. It allows us to
do this:

```rust
fn add_five(x: &mut i64) { 
    x += 5
} 

fn some_function() {
    let x: i64 = 100; 
    println!("x = {}", x); 
    add_five(&mut x); 
    println!("x = {}", x); 
    add_five(&mut x); 
    println!("x = {}", x); 
}

fn main() {
    some_function();
}
```

Because `add_five` receives a mutable reference, when we change it, we will see
the changes appear on `some_function`.

I think you can see this is super useful, and our functions got now superpowers!

Let's step this up a little.

Turns out you can also store references like `&x` or `&mut x`. And for what 
reason do we want to do this? Well… it's too early to explain. Let's just say
that it is possible, and you don't want to do this.

```rust
let mut x = 5;
let y = &mut x;
*y = 3;
println!("x = {}", x);
```

Also in structs:

```rust
struct Thingy {
    x: &mut i64,
    y: &f64,
}
```

But this must be written as:
```rust
struct Thingy<'a> {
    x: &'a mut i64,
    y: &'a f64,
}
```

The point is: it is possible. And I need you to know that this can be done to 
understand the next explanation.

But at your current level, if you see yourself coding something like that, 
simply back off and find a way to avoid storing references `&var` `&mut var`. 
They're very painful to work with.

Now, this means that there may exist several pieces of code that have access
to the same variable at the same time via different methods.

And this is a problem.

If one part of a program is using a variable and another part, inadvertently is
changing the variable… well, it can end very badly.

For this reason, Rust limits how ownership and references can coexist, and how 
many we can have.

* Ownership (`var`): There can be only one owner at a time. When we call a function, we 
  transfer the ownership of the variables unless they are references `&x` `&mut x`.
* Mutable reference (`&mut var`): There must be at most one `&mut var` at any time.
  And while it exists, the original variable cannot be read or written (more or less).
* Shared reference (`&var`): There can be many, but they cannot coexist with `&mut var`.

In other words, there must be always one owner, and one writer. Readers can be 
many, as long as there aren't anyone writing.

The ownership system in Rust can be related with having a car:

* `var`: You own a car, therefore you can choose to dispose of it at any time.
* `&mut var`: You send the car to a mechanic, they can do changes to your car, 
  but you cannot use it meanwhile. And you cannot dispose of it while is on 
  the mechanic.
* `&var`: You let others see your parked car, but not touch it. Many can see 
  your car at the same time, but meanwhile you cannot use it or send it to the
  mechanic.

Which roughly translates into:

```rust
struct Car {
    horses: i64,
    air_conditioner: bool,
}

fn buy_cheap_car() -> Car {
    return Car{horses: 60, air_conditioner: false};
}

impl Car {
    // "self" here transfers ownership. And this function will free the variable.
    fn dispose(self) {
        println!("Bye car! {} horses, A/C:{}", self.horses, self.air_conditioner);
    }
    // "&mut self" gives exclusive write access.
    fn upgrade_ac(&mut self) {
        self.air_conditioner = true;
    }
    fn upgrade_engine(&mut self) {
        self.horses += 10;
    }
    // "&self" gives shared read access.
    fn admire(&self) {
        println!(
            "Woo, nice car with {} horses and {} A/C", 
            self.horses, 
            self.air_conditioner
            );
    }
}

fn main() {
    let mut mycar = buy_cheap_car();
    let admirer1 = &mycar;
    admirer1.admire();
    
    let mechanic = &mut mycar;
    mechanic.upgrade_ac();

    let admirer1 = &mycar;
    let admirer2 = &mycar;
    // mechanic can't be used here.
    admirer1.admire();
    admirer2.admire();

    let mechanic = &mut mycar;
    mechanic.upgrade_engine();

    let admirer1 = &mycar;
    let admirer2 = &mycar;
    admirer1.admire();
    admirer2.admire();

    mycar.dispose();
    // Car no longer exists, this won't work:
    // let admirer1 = &mycar;
    // admirer1.admire();
}
```

I know that at this point this will feel confusing. It's a lot to unpack.

And I guess you have questions like:
* Why should I use this?
* Seems complicated.
* Why not use `&mut` all the time? Or just the regular `var`.
* Is this to put restrictions on the code?

No, no… forget about all those thoughts. 

It's not something we want to use, it's something that we will need to use.
Meaning, that the usage will be apparent soon enough. Don't overthink it.

We're trying to solve a problem here. The problem is sharing variables across
many pieces of code.

Before, we were doing lots of things like:

```rust
let p: Point2D;
// (...)
p = p.move(distance_vector);
```

But this is inconvenient. And we would like to just write:

```rust
let p: Point2D;
// (...)
p.move(distance_vector);
```

And that should update the point. 

For that, we want to use `&mut self` instead. So, we had:
```rust
impl Point2D {
    fn move(mut p: Self, v: Vector2D) { 
        p.x += v.dx;
        p.y += v.dy;
        return p;
    }
}
```

With what we learned, we can do:

```rust
impl Point2D {
    fn move(&mut self, v: Vector2D) { 
        self.x += v.dx;
        self.y += v.dy;
    }
}
```

And now that updates the point. Nice.

Still one problem. If we do:

```rust
let mut p1 = Point2D{x: 10.0, y: -5.0};
let mut p2 = Point2D{x: 5.0, y: 15.0};
let distance_vector = Vector2D{dx: 50.0, dy:0.0};
// (...)
p1.move(distance_vector);
p2.move(distance_vector);
```

The second move, for `p2` won't work. The problem is that `move` is taking
ownership of `distance_vector`, so we lose the variable.

But we don't need ownership just for reading the contents, do we?

So let's update that:


```rust
struct Point2D {
    x: f64,
    y: f64,
}

struct Vector2D {
    dx: f64,
    dy: f64,
}

impl Point2D {
    // Added ampersand         v---- here
    fn translate(&mut self, v: &Vector2D) { 
        self.x += v.dx;
        self.y += v.dy;
    }
}

fn main() {
    let mut p1 = Point2D{x: 10.0, y: -5.0};
    let mut p2 = Point2D{x: 5.0, y: 15.0};
    let distance_vector = Vector2D{dx: 50.0, dy:0.0};
    // Added ampersand
    //           v---- here
    p1.translate(&distance_vector);
    p2.translate(&distance_vector);
}
```

With just that single `&` character, now it works.

I'll leave this here. Take it as an introduction into borrowing and ownership. 
Later on we will revisit this topic a bit more in-depth. So if it's not fully
clear, don't worry!

Also, we're reaching a point where you can already read (and probably write) 
what could be production-ready Rust code. This is (almost) what real code looks
like. You're getting close to be able to start your own applications!

-------
[^1]: If you ever asked yourself why all code samples and exercises I wrote 
almost always had number variables, this is the reason. They're easier to work 
with and avoids these problems.




