# A tiny bit on strings

Storing and manipulating texts in a program is a bit harder to explain than numbers. 
This is because a computer does not work with text, but with numbers.

Surely you have heard that in a computer everything
are ones and zeros (the binary system). 
These can be combined to create big numbers, and with some tricks the CPU can 
also work natively with floating point in order to have decimal places.

But text is not a number, which means that Rust needs a bit of work behind the 
scenes to make these work.

As a basic introduction, a text is nothing more than a sequence of letters. 
The text `ABC` might be a string, but it also is a sequence of
three letters: `A`, `B` and `C`.

Each letter is a character. If we assign each possible character a number, we can in fact store text using numbers:
* `A` → 65
* `B` → 66
* `C` → 67
* ...

This is totally arbitrary, but there's a standard called *"ASCII"* that defines
a table of conversions from letters to numbers. If all programs use the same 
table[^1], then we can save those numbers into a file, and get the correct texts
back when another program reads the same file. 

So a string is just a list of numbers that it is conveniently interpreted as text.

Instead of the ASCII table Rust by default uses UTF8 for strings. I won't go 
into much detail here, but the world has different languages with 
different characters. Think about Chinese, Japanese, and Russian. The ASCII 
table is not good for those. UTF8 is still compatible with ASCII, 
but it allows for a character to span multiple number entries (bytes), so we 
can have in the same format any kind of writing language.

To keep things simple, just assume it is ASCII for now. We'll cover UTF later on.

Why are texts called "strings"?
https://stackoverflow.com/questions/880195/the-history-behind-the-definition-of-a-string
(To be completed later)

[^1]: Actually they don't, and that's why we have so many problems with tildes,
foreign characters, and the reason why UTF exists.