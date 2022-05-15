# Not everything are numbers and texts: Structs

So far we have seen the basic building blocks for almost everything. Functions,
conditions (if), loops (for) and some data types. This will get us already 
very far.

But! For today's standards, we need a bit more knowledge. One of the pieces that
we have missing and is still critical are structs.

Do not worry. They're very easy to understand and natural. Structs are basically
composite types. This is, a data type that contains different kinds of data.

For what would we need such a strange thing? Those programmers… are crazy. We
love making up complicated stuff. Hah! Got you! You were thinking this, right?

Anyway, trust me they're super useful. Let's assume we want to track points in
2D space. A point in 2D has two coordinates: \\(x, y\\)

For this, we could write something like:

```rust
let x = 0.0;
let y = 0.0;
```

And we're done. No need for structs.

Okay, but we want 5 points. Now what?

```rust
let point1_x = 0.0;
let point1_y = 0.0;

let point2_x = 0.0;
let point2_y = 0.0;

let point3_x = 0.0;
let point3_y = 0.0;

let point4_x = 0.0;
let point4_y = 0.0;

let point5_x = 0.0;
let point5_y = 0.0;
```

Done! Super easy.

But now let's say we want a function that accepts a point and moves it a certain
amount:

```rust
fn move_point(mut px: f64, mut py:f64, dx:f64, dy:f64) -> (f64, f64) {
    px += dx;
    py += dy;
    return (px, py);
}
```

> Note that this function returns two values. We haven't seen this yet.

And now we want to move a point a few units:

```rust
let (point1_x, point1_y) = move_point(5.0, 0.0, point1_x, point1_y);
```

Still feels good this kind of coding? Doesn't it feel clunky?

Wait! There was a mistake in the above code. We passed `5.0` as the distance
where we should have passed the point. The arguments are in the wrong order!

This, my friend, wouldn't have occurred with clever uses of structs.

Same idea, but with structs:

```rust
struct Point2D {
    x: f64,
    y: f64,
}

struct Vector2D {
    dx: f64,
    dy: f64,
}

fn move_point(mut point: Point2D, distance: Vector2D) -> Point2D {
    point.x += distance.dx;
    point.y += distance.dy;
    return point;
}

fn main() {
    let mut p1 = Point2D{x: 0.0,y: 0.0};
    let p2 = Point2D{x: 3.0,y: 0.0};

    let dist = Vector2D{dx: 5.0, dy: 0.0}
    p1 = move_point(p1, dist);
}
```

Now we're talking. Wait, too much? Ok, let's break it down.

First, we define a new data type called `Point2D`, which is a struct:

```rust
struct Point2D {
    x: f64,
    y: f64,
}
```

It contains two members: `x` and `y`. Both are `f64` (number with decimals).

With this, we can already create our values of type `Point2D`.

So, if we did before:

```rust
let p: f64;
```

We can now do:

```rust
let p: Point2D;
```

Because that's now a user type, and it acts similar to all other types.

To construct it, we simply specify the values of the contents:

```rust
let p2 = Point2D{x: 3.0,y: 0.0};
```

That creates a point `p2` that sits at `3,0`.

To access the contents, just use the dot operator:

```rust
println!("x: {}, y: {}", p2.x, p2.y);
```

To change the value, you can use the same trick with the dot:

```rust
let mut p1 = Point2D{x: 0.0,y: 0.0};

p1.x = 10.0;

println!("x: {}, y: {}", p1.x, p1.y);
```

Or, if you want, you can also replace the point as a whole instead:

```rust
let mut p1 = Point2D{x: 0.0,y: 0.0};
println!("x: {}, y: {}", p1.x, p1.y);

p1 = Point2D{x: 20.0,y: 0.0};

println!("x: {}, y: {}", p1.x, p1.y);
```

I also did the same for something called vector:

```rust
struct Vector2D {
    dx: f64,
    dy: f64,
}
```

A vector can be used to specify distances or relative positions. While point
is absolute. Other than that, it's the same thing. Works exactly the same.

So why? If they're effectively the same thing; both have \\(x, y\\) and use the
same type, why bother on creating another one? Couldn't we use the same Point2D
for everything?

Of course! That would work. But it would work… *too much*.

The issue is that they aren't technically the same thing. A point is like a 
position in GPS (30ºN, 20ºW), while a vector is a 
distance + length (200 km north-east).

If we confuse those two, we can end with:

* A position that basically is on the North Pole. (At the zero position)
* A vector that says: travel 300,000 km south.

Both of which are very wrong.

Remember what happened on the function before where we confused both parts?
Well, not gonna happen now:

```rust
fn move_point(point: Point2D, distance: Vector2D) -> Point2D {
    point.x += distance.dx;
    point.y += distance.dy;
    return point;
}
```

This function accepts first a point, then a vector. If you accidentally reverse
them, Rust is not going to accept the program and error out.

So we have now two types that are almost the same thing but can't be intermixed.

This is **very** useful. 
The amount of errors that get caught this way is tremendous.

Using only one type (point) for everything would at least protect us from mixing
accidentally \\(x\\) and \\(y\\) coordinates.

But using two types, ensures that the code stays error-free (mostly).

Structs not only are useful to enforce type-safety. They also are needed to get
more complex types working.

For example, in an invoicing application you might want to store items.

So we could do:

```rust

struct Item {
    name: String,
    price: f64,
    provider: String,
    observations: String,
    obsolete: bool,
    stock: f64,
    min_stock: f64,
    max_stock: f64,
}
```

And I think this looks very convenient to use, instead of simple variables.
