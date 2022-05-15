# Struct-specific functions, really?

Really. Remember this function?

```rust
fn move_point(mut point: Point2D, distance: Vector2D) -> Point2D {
    point.x += distance.dx;
    point.y += distance.dy;
    return point;
}
```

Turns out that it acts on points, and it only makes sense on points. So we
can make this function be part of the struct itself.

The idea is that, instead of writing:

```rust
p1 = move_point(p1, d1);
```

We could write:

```rust
p1 = p1.move_point(d1);
```

Which looks way nicer. And because "point" is implied, we can rename it:

```rust
p1 = p1.move(d1);
```

And it's way more natural now.

So, how we make this happen?

Implementing the struct. There's the keyword `impl` and we just do:

```rust
impl Point2D {
    // Stuff here..
}
```

And inside, we put the function:

```rust
impl Point2D {
    fn move_point(mut point: Point2D, distance: Vector2D) -> Point2D {
        point.x += distance.dx;
        point.y += distance.dy;
        return point;
    }
}
```

The first parameter, we call it `Self` to make the `p.move_point` magic happen:

```rust
impl Point2D {
    fn move_point(point: Self, distance: Vector2D) -> Point2D {
        point.x += distance.dx;
        point.y += distance.dy;
        return point;
    }
}
```

Now rename the function to `move`, because "point" is implied, and voilà:
```rust
impl Point2D {
    fn move(point: Self, distance: Vector2D) -> Point2D {
        point.x += distance.dx;
        point.y += distance.dy;
        return point;
    }
}
```

This is all we need to get the fancy stuff in. And the cool part is that these
functions don't pollute our other code. They're encapsulated in the type.

If you want to move a point, or do something to a point, you look into the 
functions that are implemented for the point. Which makes sense, right?

Basically we look, and ask: what do points do? And the list of functions tells
us the capabilities of this new type.

Also, if we want we can also call this function explicitly:

```rust
p = p.move(d1);

p = Point2D::move(p, d1); // Exactly the same as the previous line.
```

Why would we want to do this? Well, right now we don't. It's longer, uglier, and
harder to read. When possible, we will use the shorter form.

But this is the key: when possible. Sometimes is not possible, or very 
inconvenient. That's when this form will come in handy.

## A bit of keywords 

When a struct has a variable inside, it is called a **member**.

```rust
struct Point {
    x: f64,  // <-- this is a member of Point
}
```

When a function exists inside the implementation of a struct, it is called a
**method**:

```rust
impl Point {
    fn move(mut p: Self, d: f64) {  // <-- this is a method of Point
        p.x += d;
        return p;
    }
}
```

Now you know. Be sure to use these terms as much as possible to avoid those weird
looks from senior devs that seem that want to kill someone. "Have you heard? They
just called a method a 'function'… A function! How silly is that?"

Also be sure to ask for a pay rise because of this too.[^1]

-------

[^1]: For those that haven't caught on the joke, almost all developers are very
snob towards naming things. And they act like these have nothing to do, 
completely different. Furthermore, we keep crafting new names for the same 
thing, up to the point where the same concept has like 20 names. But it feels so
special! I wonder what would happen when they ran out of English words to use[^2].

[^2]: Wait, I do know. See [TMTLA](https://acronyms.thefreedictionary.com/Too+Many+Three+Letter+Acronyms).

