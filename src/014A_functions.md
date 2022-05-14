# Introducing Functions

It might seem difficult to picture that sometimes the same 
piece of code needs to repeat in different places of the code, 
and it's not a loop what I'm talking about.

The example programs I can add in this book have to be small, 
so we can go line by line. In reality, good useful programs have thousands of lines. 
It's hard to make something really useful in twenty lines 
with only the basic operations we learned.

It might seem daunting to think about writing several thousand lines, but it is 
easier than it seems. We always start small, we keep adding pieces and after a 
few weeks it's easy to have those thousand lines.

One of the applications I wrote, zzping, has nearly 7000 lines of code in it. 
And it has been only a hobby without much investment from my side. 
Programs get big very easily.

Having all those instructions inside the `fn main() {...}` is very hard to follow. 
It's similar to organizing stuff in your room or in the house. 
If all items of your house were in a single big box, trying to find anything 
there is nearly impossible, so we all use different drawers, stands and boxes 
to sort the stuff, so we can locate it later.

In the same fashion, we split big programs into different files, so all related 
instructions that work towards a similar goal are near each other, and each file
has its own tools there.

We will see later on how to split into different files 
(in Rust, those are called modules), but now I want to explain how to sort 
stuff out inside a single file.

Imagine we're doing some sort of program that tells the user 
interesting stuff, and it has a menu:

```rust
fn main() {
   println!("Welcome to the Trivia program!");
   println!("------------------------------");
   println!("");
   println!("Please choose an option:");
   println!("");
   println!("    1. Tell a funny story");
   println!("    2. Show the multiplication table for a number");
   println!("    3. Show the dividing table for a number");
   println!("    4. Tell the future for a zodiac sign");
   println!("    5. Browse the cooking recipes");
   println!("    6. Exit the program");
   println!("");
   println!(" Your option:");
 }
```

As you can imagine, each of these options almost consists of its own program. 
Trying to code everything in here is going to be really confusing:

```rust
fn main() {
   println!("Welcome to the Trivia program!");
   println!("------------------------------");
   println!("");
   println!("Please choose an option:");
   println!("");
   println!("    1. Tell a funny story");
   println!("    2. Show the multiplication table for a number");
   println!("    3. Show the division table for a number");
   println!("    4. Tell the future for a zodiac sign");
   println!("    5. Browse the cooking recipes");
   println!("    6. Exit the program");
   println!("");
   println!(" Your option:");
   let option = 1; // TODO: Read the user input option and store it here.
   if option == 1 {
       println!("Here's a joke...")
       // TODO: Add jokes and a joke selector
   }
   if option == 2 {
       // TODO: Ask the user which number
       // ... compute and show the multiplication table
   }
   if option == 3 {
       // TODO: Ask the user which number
       // ... compute and show the division table
   }
   if option == 4 {
       // TODO: Add a database of zodiacs with their predictions
       // ... ask the user and show the matching one
   }
   if option == 5 {
       // TODO: Add a database of recipes
       // ... ask the user and show the matching one
   }
   if option == 6 {
       // TODO: Exit the program here
   }
   // TODO: Loop again to the beginning if option 6 wasn't chosen.
}
```

It would be nice if we could break the program into subprograms that do specific
things, so we can call those when we need them, right?

That concept is exactly what a function is.

As usual, I'll go back to short and stupid programs, but keep in mind the above
example where they're useful.

```rust
fn welcome() {
   println!("Welcome adventurer!");
   println!("There are lots of treasures hidden in this place.");
   println!("Oh, and there's also a princess trapped in a castle.");  
   println!("");
   println!("You know what to do.");
}

fn main() {
   welcome();
   println!("End of program");
}
```

Here we can see a "welcome" function that prints six lines of text. 
What the computer does is:
* It always starts from the main function.
* Reads the `welcome()` call to the function, so it jumps to the top `fn welcome()`
* Executes the five `println!`
* The function ends on the `}`, so it goes back where it was on main.
* Reads the next line and prints "End of program".
* Reaches the end of main (the `}`) and the program ends here.

A function has two sides, the declaration and the call. The function declaration states what is the function name and what it does:

```rust
fn your_function_name() { 
  // ... your code here for what this function does ... 
}
```

And then you can call it as many times you want, using:

```rust
your_function_name();
```

And the key thing here is "call it as many times you want". The function's 
purpose **is to be reused several times**, so you don't have to repeat your 
code several times.

The declaration part doesn't need to be above or below the main(), it can be 
anywhere as long as it is not inside the main. 

Placing it before or after the main is fine, the order doesn't matter.
