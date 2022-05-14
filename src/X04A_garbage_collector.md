# What is a Garbage Collector?

It's something that Rust doesn't have. Have a good day!

Garbage Collector is about memory allocation. In C & C++ memory needs to be manually requested and freed. Because this is very dangerous, most other languages automatically allocate and free.

Allocating is "easy", as the first use is allocating. But automatic free is hard, because you need to ensure that no parts of the program can access that variable anymore.

So a Garbage Collector is a system that can track when variables go out of scope (unreachable) in order to free the memory. There are lots of types of GCs (each language might use a different approach), but they tend to have performance penalties, memory over-consumption and may temporarily halt all threads of the program to do the free.
* In Python, the GC prevents Python from running several threads of python code in parallel. So most python code, even if it uses threads, effectively is single-threaded because of the GC.
* In Java, the GC tends to free memory very late, causing bloat, a lot of memory consumption.
* In Go, the GC causes micro-pauses. All threads suddenly stop for a few milliseconds to clean up.
* In Rust, there's no GC. Instead the automatic free is computed at compile time (by static analysis). So no bloat, no performance penalty.
* C & C++, there's no GC, you do it manually, and a mistake causes memory corruption and segfaults.

Rust is kind of a compiler assisted C++. Automatic like Python, but if the compiler cannot prove your program is correct (even if the program in fact IS correct), it will reject it and won't compile. 

So the drawback from Rust approach is the added difficulty of learning Ownership & Borrowing (which I talk about in my blog post)
....
