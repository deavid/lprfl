# Data types

We have reached a point where I cannot continue to explain much more without 
giving you a fair bit of theory. Sorry about this, but it's needed.

## What are data types?

First: what is data? Data is just the technical term for value. 
The number 5 is a value and is data, and so is 2.41 or the string "Basement".

The concept of "data types" just refers to the different kinds of values. 
You see, 3+5 is 8, but "Base"+"ment" is "Basement". 
The sign plus (+) might work differently depending on if it is text or a number.

Some operators like dividing (/) make sense on numbers (8/2=4) but 
don't make any sense on texts ("Base"/"ment" = ????).

So the primordial thing that puts data types apart is what can be done with the 
values, what makes sense and what doesn't; 
and also what the operations actually mean.

Because, you know, computers might look like really smart, but they're actually
pretty stupid. If you do, "3"+"5" is just "35" and not "8" 
(because of the quotes, it is treated like text, so it gets concatenated when 
using the plus sign). They take everything literally, too literally. 
And we need to tell them what to do, step by step to a level that feels like 
training a monkey how to make a chocolate brownie.

There are tons of data types, you can construct your own, and you can also use
other people's types. But in general, to start, we have three basic types:
* Numbers
   * Integers (which can be positive or negative, but don't have a decimal point)
   * Floating point numbers (or floats) that actually have decimal places.
* Texts, generally called "strings" in programming 
  (at some point later I might explain where the name "string" comes from, it's a bit funny)
* True/False types, called "boolean", used to store the result of comparisons such as "apples > 1".

You might be inclined to always use floating point numbers for everything, as 
they can do much more than regular integers. If they're better, there's no point
in using the puny integers, right?

Well, no. Later on I will point out the different problems that floating points
have. For now, just write 2.0 instead of 2 for every number, and you're done.
