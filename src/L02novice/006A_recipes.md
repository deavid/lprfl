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

## Understanding what a program is

It might appear as a rhetorical question, but what is really a program?

In some sense, a program is a set of instructions that the computer executes in
order. In our case, it will read a file and execute line by line, from the top
to the bottom.

But programs are more than that.

A program can be compiled or interpreted. Interpreted programs, like Python ones,
are actually reading the file and executing the lines. But compiled programs 
like the ones Rust does, actually create something called a "binary".

Rust will translate all our instructions into another language called "machine 
code", which is the language that our computer does understand. Then it will
write the file with those instructions. This file is what we call the "binary"
program.

Also, programs can interact with the computer in several ways. For example, they
can accept input from the user, or communicate via the console. And they can also
read arguments when they're executed.

As part of our journey to learn programming, it is critical that we understand
arguments and input/output in the console.

### Program Arguments

Let's see arguments first.
When you execute a program in the console, it can accept several arguments.

An argument is basically text that we can provide a program to operate.

For example, we can open a browser from the console, passing the URL to open:

```
$ firefox https://google.com
```

The URL is an argument (or parameter) that we provided to Firefox.

In general, we provide arguments like this:

```
$ myprogram "argument1" "argument2" "argument3"
```

And just for the record, they also have "return values" which you can 
check in Bash[^1]:
```
$ echo "hello"
$ echo $? 
0
```
Zero indicates success. Any other number is an error.

----

[^1]: Bash is for Linux and other Unix-alike operating systems. Windows can do
this too, but in a different way.

### Input and Output

A program in the console has by default three ways to communicate:

* Standard Input  (stdin): This is what the user types in the console.
* Standard Output (stdout): This is where the program writes in the console.
* Standard Error  (stderr): Here the program sends error messages, usually they
appear in the console too.

Very weird names for something tremendously simple:

```
$ my_program
What is your name:      <--- this is stdout
Waldo                   <--- this is stdin (you type this)

Hello, waldo!

What is your age:
white

>> ERROR: white is not a number! Program failure. <--- this is stderr (an error message)
$
```

Hope this serves to explain the basics on how programs have their inputs and 
outputs for the user, as this will be useful later on.