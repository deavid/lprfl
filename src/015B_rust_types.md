# Rust data types

Rust data types are however a bit more nuanced. You see, these values we want to store have to be actually recorded somewhere in your physical computer. And that will be your computer memory (RAM, or DDR if you prefer). The number 4 might not look like much, but Rust has to ask for memory from your computer in order to save the value somewhere, else it would be lost, forgotten and programs won't run.

So the question that Rust faces is: how much memory is needed for this variable? A variable can change its value later on, for example that x=4, later on it can be x=99999999. It needs to grab enough memory in order to ensure that all values you might want to put there actually do fit.

This is why integers are split into different sizes:
* i8
* i16
* i32
* i64

Each of those is a valid Rust type for an integer. And yeah, the "i" is short for integer. The number that it follows is the number of bits that Rust will get for them.

Is 8 bits a lot? Let's see. A computer might have 4 gigabytes of RAM. Giga just means billion. So that's 4,000,000,000 bytes. And a byte is just 8 bits.

So this means that i8 only uses 1 byte of memory. And if a computer has 4,000,000,000 bytes, I can surely assume it can store a whole lot of those and still have plenty of space left. So yeah, it's very tiny.

But this affects the range of numbers we can store. It is fine for storing -4, 12 or 81. But 355 is too much for an i8 and the program will fail. If we need to store bigger numbers, we need to use types that use more space.

For example, i16 can store up to 32,000.

But you know what? This is too complicated. Why bother? Use i64 and that's it. Forget about the others for now and just use i64 for everything. Your computer has so much memory that it doesn't care if a number uses 1 byte or 8.

An i64 can store any integer number up to roughly 9,000,000,000,000,000,000. Either positive or negative. Unless you want to start counting atoms, I think we have more than enough to work with. So again, let's use i64 for all our integer numbers and forget about this conversation.

Rust also has unsigned numbers, which work exactly the same as the regular integers but they can't store negative numbers, Instead of "i" they go with "u" and work exactly the same:
* u8
* u16
* u32
* u64

Why limit ourselves to only positive? Well, they can hold double the numbers inside. But... why? wasn't 9,00.. or whatever enough for us...?

Exactly. It is enough. So again, we won't be using these. Use i64.

There are two extra integer types in Rust:
* usize
* isize

If your computer is a 64bit one, then these are equivalent to u64 and i64. But these have their specific use. We will come back later to the "usize" as it is how computers locate things in memory. Certain things such as locating an item in a list/collection of numbers use "usize".

I'm dumping this to you now to give an approximate picture of the data types that Rust has in the hopes that you'll begin to recognise these later on. I'll explain this again later on with more detail, so no need to study and memorize these things now.

Floating points, very similarly have two variants for sizes:
* f32
* f64

"f" for float, and 32 or 64 for the amount of memory that they use. And guess what, we'll use f64 when we need decimal places and forget about f32.

Boolean values (bool) only have one type:
* bool

There isn't much secret on this type, it is quite like an integer that holds 0 for false and 1 for true. That is all.

Strings (texts) also come in two variants, but, surprise! it's not for size!
* str
* String

The difference is a bit too nuanced to explain right now, so let's just say they're almost the same thing. We will use String for our variables, as it is simpler to use, but in some cases, Rust wants a "str" instead. They can be converted one to another, so it's not a big deal.

As you might have noticed, they don't have any specific size. So, how big can the text be inside?

Short answer: As long as you want.

They're dynamic. Rust will request more memory when needed. So as long as your computer has enough memory to store the text, it will work[^note]. 
[^note] Oh crap, I'm sorry, I'm lying way too much... I swear I will fix this mess later on. For now, bear with me, this is the explanation you need

And those are all types we will be using for a long, long time here. So, to recap:
* Integer numbers (without decimal places): i64
* Floating point numbers (with decimal places): f64
* Strings (text): String (with the first S in uppercase)
* Booleans (true/false): bool
* Locating values inside a list: usize (we'll see this later on)
