# It's like cooking recipes, seriously

You may not be into cooking, and that's fine. 
But probably you know what a cooking recipe is: 
nothing more than a set of steps (instructions) that if followed 
produce the desired result: delicious food.

A program works in the same way. 
It has a set of instructions that should be followed step by step to produce 
the desired result. 

There are stupid simple cooking recipes, for example to prepare frozen pizza:
* Preheat the oven to 180ºC for 10 minutes
* Remove the pizza from the box and remove the film
* Put the pizza into the oven, on top of a tray.
* Wait 15 minutes.
* Turn off the oven, remove the pizza and serve.

And there are simple computer programs as well:
* Print one line on the console that says `Hello world!`
* end the program.

But a common problem in cooking recipes is that they prepare a specific 
amount of food. If you want more or less, you have to tweak *"the program"* to 
roughly make more or less food to meet your requirements.

In programs, we have inputs (or arguments), where we can add a value and the 
program will take it into account for the calculation. In recipes, this is like 
having a number of "people to serve" and having some formula to scale up the 
ingredients to get the right amount of food.

We also have conditions, which work like those recipe 
steps that say "cook until brown".

There are loops, which allow us to say "do this 10 times".

There are functions, which in recipes appears when a meal is very complicated 
and for a particular step says "to do the base of the cake, refer to this other recipe". 
It avoids repeating ourselves every time that a set of steps we can reuse across recipes.

Of course programs can do things really complicated that are quite far from recipes, 
but if you're starting to learn, this comparison will serve you to get a 
better grasp on how this all works.

## Program Arguments

When you execute a program in the console, it can accept arguments:

```
$ myprogram "argument1" "argument2" "argument3"
```

They also have "return values" which you can check in Bash[^1]:
```
$ echo "hello"
$ echo $? 
0
```

Zero indicates success. Any other number is an error.

## Input and Output

A program in the console has by default three ways to communicate:

* Standard Input  (stdin): This is what the user types in the console.
* Standard Output (stdout): This is where the program writes in the console.
* Standard Error  (stderr): Here the program sends error messages, usually they
appear in the console too.

