# What are those other data types for?

So we talked earlier about Rust data types and we're just using a few. Why do they exist? Let's put them together here:

| Type | Min |     Max      | | Type |      Min      |      Max     |
|------|-----|--------------|-|------|---------------|--------------|
|  u8  |  0  |          255 | |  i8  |          -128 |          127 |
|  u16 |  0  |        65535 | |  i16 |        -32768 |        32767 |
|  u32 |  0  |   4294967296 | |  i32 |   -2147483648 |   2147483647 |
|  u64 |  0  | 1.84 * 10^19 | |  i64 | -9.22 * 10^18 | 9.22 * 10^18 |
	

Examples:
* u8 is generally used to represent characters in the ASCII table, and also bytes in a file in disk.
* u16 is used for storing UTF16 characters, which are 2 byte long.
* u32 can be used to store colors of pixels of an image (Red, Green, Blue and Alpha)

While a single number might be small in comparison to the amount of memory a computer has, some programs are in fact "number crunching programs", where millions or even billions of numbers need to be processed very fast. When you have so much data, compacting it on the smallest possible representation will save memory and/or disk space.

That is why, for what we're doing in this guide we don't care that much and use i64 for everything. All in all these are toy programs. Our toy programs don't do much and the difference in memory used is roughly zero.

In real life, things are different. Say you want to read a JPG photo from your phone camera and do some process on it, like enhancing the photo. A photo can easily be 16 million pixels, and if for each pixel we use a i64 for each color (Red, Green and Blue), we would be wasting a lot of memory.

A program that plays chess by itself has to consider billions of chess possibilities, in this case it is also important to keep each representation as compact as possible.

On the other hand, there are also the two float types, f32 and f64. The difference between them is not the maximum or minimum number like integers, but the amount of significant digits they can carry. f32 can record 6 digits correctly (and a few extra ones) while f64 can record more than 12.
<!-- in fact, f64 can represent bigger numbers than f32, but not that this does matter here... -->

Floating point numbers are used specially on scientific problems. Depending on the problem at hand an f64 might be required, while in others f32 might be better as it has enough precision but it is smaller, allowing for much complex simulations.

Graphics cards use a lot of floating point numbers to draw scenes for a game. If the precision is not high enough it might lead to jitter: https://www.youtube.com/watch?v=jsLiDQyyBXk

I hope this helps understanding why these types exist, and why we aren't using them for now.