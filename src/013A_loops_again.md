# For loops again!

Yay! Wait, I thought you were excited about this.

Let's do something useful. This program will graph the function "y = x * x / 20 - 9" in the terminal:

```rust
fn main() {
   for y in -10..10 {
       for x in -30..30 {
           let value = x * x / 20 - 9;
           if value >= y {
               print!(" ");
           } else {
               print!("#");
           }
       }
       println!();
   }
 }
```

As you can see, it only uses "for" and "if". It might look complicated, but with a bit of work you should be able to follow it.

For the record, this is the output it produces:

```
 $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/learnrust`
                                                            
                                               
                      #########              
                    #############            
                   ###############           
                  #################          
                 ###################         
                #####################        
               #######################       
              #########################      
             ###########################     
            #############################    
            #############################    
           ###############################   
          #################################  
          #################################  
         ################################### 
         ################################### 
        #####################################
        #####################################
```

You can change the formula in "let value = x * x / 20 - 9" and it will graph whatever math function you like.

I know, there's a lot to unpack here. I'll go step by step. But first, for the avid readers, yes the function is mirrored upside down. This is because the first line that is drawn first is in math the bottom one, and computers draw top to bottom while in math the Y axis goes from bottom to top.

It can be fixed, but to keep things simple I prefer to keep this bug in.

Let's go first on the inner code of the loops:

```rust
           let value = x * x / 20 - 9;
           if value >= y {
               print!(" ");
           } else {
               print!("#");
           }
```

This computes x² by doing "x * x", divides by 20 and then subtracts 9. Rust has a pow() function to do x² instead of x * x, but to keep it simple I avoided it. Anyway, this gets stored into the "value" variable and the "if" compares against "y".

So, if it's less than "y" prints a hash "#", if not, it prints a space " " otherwise.

This is meant to fill the shape. Given the math function:

        y = x² / 20 - 9

It computes which "squares" are below the math function and uses the "#" character to fill the shape.

Notice how I used "print!" instead of "println!". The lack of the "ln" (which stands for line) makes that print not open a new line, so it prints to the right, like a typewriter.

So all that is left is to loop across all "x" and "y" squares:


```rust
   for y in -10..10 {
       for x in -30..30 {
           // ... 
       }
       println!();
   }
```

So using here a nested loop we iterate through negative and positive values for both x and y.

As "y" is usually the vertical axis, and the terminal prints first left-to-right and then to-to-bottom, we need to first print an entire row; That's why the "for y" appears first and the "for x" appears next.

After the "for x" is done, we need to move to the next line, so an empty "println!()" will move the cursor to the next line.
