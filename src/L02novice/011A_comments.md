# Adding some comments

The programs are sometimes hard to understand, and it would be nice to leave
some notes for the people reading it, so they can understand it too. 
And I bet you that you'll forget what a program does after 3 months, 
even if you wrote it yourself. It happens to me too. 
So it's good to have some notes on the program, so we can understand it later.

They can also be used to denote that some work is yet missing 
(we call these `TODO`) so it serves as a reminder for later on. 
But we all know that we will never get to do them. 
That's how it works in reality, trust me.

```rust
fn main() {
   // Compute all the cells in a Chess Board:
   for row in 1..=8 {
       for column in 1..=8 {
           // TODO: For now just display the numbers, we'll fix this ""later"".
           println!("Row {}, Column {}", row, column);
       }
   }
 }
```

Most comments use the double slash `//`. 
When Rust sees this, it ignores any text on the right side of it, 
so we can use this to add our thoughts on the program.

However, if you need to comment out a lot of lines, 
adding `//` to every line can be daunting. 
VS Code has shortcuts for this (and you can customize them) so it's easier.

<!-- Shortcut for VSCode for comments - default and how to customize -->

But Rust also has comment blocks. 
A comment block starts with `/*` and ends with `*/`. 
Rust will ignore everything in the middle, even if there are multiple lines.


```rust
/*fn main() {
   for row in 1..=8 {
       for column in 1..=8 {
           println!("Row {}, Column {}", row, column);
       }
   }
 }*/
```

Voilà. Now we no longer have any program. From Rust's perspective, the file is empty.

But these comment types are a bit trickier. They cannot be nested. If you try to add a block comment on something that already contains a comment block inside, Rust will get confused.

Because of this, I prefer to avoid these and stick with the simple and reliable double slash (//).