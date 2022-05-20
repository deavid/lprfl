# Appendix E. Rust compared to other languages

* Rust has structs and impl.
    * C++, Java, Python, JavaScript have classes.
    * C doesn't have classes, but it has structs.
        * However, no function methods can be placed.

* Rust has generics
    * C++ and Java have it too.
    * C and Python do not.
        * Python doesn't need it.
    * Go is adding them.

* Rust has traits
    * Java and Go have interfaces.

## Features unique to Rust

* The fact that by default variables are not mutable. In other languages you
  have to specify "const" to get the opposite effect.

* Move by default instead of Copy.

* Ownership and Borrowing.

* No Garbage Collector and no manual memory management (At the same time)
  * C/C++ have no GC, but manual memory allocation.
  * Almost every other language has a GC.
