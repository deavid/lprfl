# Closures


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