# Looping' around

I'm sure you feel that variables don't do that much. But that's because the programs we can write up to now are too linear and simple. We need to step up the game with... loops!

Loops are ways of repeating the same piece of code several times without need of copy pasting. For example, imagine we want to make a simple program that counts from 1 to 100:

```rust
fn main() {
   println!("Number {}", 1);
   println!("Number {}", 2);
   println!("Number {}", 3);
   // ...
   println!("Number {}", 98);
   println!("Number {}", 99);
   println!("Number {}", 100);
 }
```

As you can imagine, this gets tedious very easily. Copying, pasting and changing all the numbers manually is cumbersome.

Presenting... the for loop!

```rust
fn main() {
   for number in 1..=100 {
       println!("Number {}", number);
   }
 }
```

This program does exactly the same thing in just three lines! Amazing, isn't it? Now variables are actually being useful.

The syntax for this is as follows[^note]:
[^note] As usual, I'm lying and it's not the real syntax. For loops are way more powerful than this; we'll get to that later.

for variable_name in first_number..=last_number

We are asking the program to have a variable that counts from 1 to 100. The current count number will be stored in the variable "number" (which we can name it as we like).

The "..=" in between the numbers is used to define a range. The equals on the right means that it includes the right number. There also exists ".." which does not include the last number (i.e. 0..100 counts from 0 to 99).

We can also put a loop inside a loop, so we can count in two directions. This could be useful to describe all positions in a chess board:

```rust
fn main() {
   for row in 1..=8 {
       for column in 1..=8 {
           println!("Row {}, Column {}", row, column);
       }
   }
 }
```

And of course, there's no limit. You could put three or ten loops one inside another. The limit is your imagination here!

In this code, it will first pick a row, then go over all the columns. When all 8 columns are done it will proceed with the next row.