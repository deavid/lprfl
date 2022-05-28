# Exercise: loops

Do you know Fibonacci numbers? These are a sequence in math that go:

```
0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144,...
```

Each number in this list has to be the sum of the previous two numbers.

We start with zero and one and begin adding them together:

```
0 + 1 = 1
1 + 1 = 2
1 + 2 = 3
2 + 3 = 5
3 + 5 = 8
```

But why Fibonacci numbers? Well, they're easy to calculate. Nothing too fancy.

Also, they are closely related to lots of stuff in math. The golden ratio.
The pascal triangle. The rate at which vampires (and bunnies) breed. The size
of an A4. The binomial distribution.

<https://en.wikipedia.org/wiki/Fibonacci_number>

Let's make a program that calculates several thousands of these!

Yeah, exciting. I know.

Again, let's make a new binary `fibonacci.rs` inside `src/bin/` and add
a `fn main() {}`.

Also, in `Cargo.toml`, remember to add:

```toml
[[bin]]
name = "fibonacci"
```

> At some point, I'll stop telling you to do these steps. Don't forget to do 
> this!

## Let's create the main thingy

So we know we want lots of numbers and a loop.

First, let's do it without a loop for 2 or 3 numbers.

```rust
{{#include ../../learnrust/src/bin/fibonacci.rs:no_loop}}
```

We first define `n1` and `n2` as the initial numbers \\(0,1\\). This part is what 
initializes the Fibonacci. 

Then we compute `n3 = n1 + n2`. Each number is the sum of the last two.

In the next step we need to move the numbers to the left, so `n3` becomes `n2`
and `n2` becomes `n1`. And then we can compute `n3` again.

However, the order here matters. If we first move `n3` into `n2`, we will lose
the contents in `n2`.

Instead, we need to move first `n2` to `n1` to avoid losing any data.

## Now onto the loop!

We make the `n1` and `n2` variables mutable and keep everything in a loop:

```rust
{{#include ../../learnrust/src/bin/fibonacci.rs:loop1}}
```

That's it! We have Fibonacci numbers!

It's really fast, right?

> NOTE: We had to use the `mut` keyword to indicate to Rust that we want to be
> able to change the contents of a variable. Otherwise, the compiler 
> would complain.

But it's hard to read. Let's do something about it. What if we print 10 per line?

```rust
{{#include ../../learnrust/src/bin/fibonacci.rs:loop2}}
```

Oh no…

```console
0,1,1,2,3,5,8,13,21,34,55,89,
144,233,377,610,987,1597,2584,4181,6765,10946,
17711,28657,46368,75025,121393,196418,317811,514229,832040,1346269,
2178309,3524578,5702887,9227465,14930352,24157817,39088169,63245986,102334155,165580141,
thread 'main' panicked at 'attempt to add with overflow', src/bin/fibonacci.rs:45:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

What happened here?

Turns out that our numbers are getting too big, and they don't fit.

By default, Rust is using 32bits (4 bytes) for each number, so they can hold
any number up to \\(2^{31} = 2147483648\\).

We could tell Rust to use 64bit instead, but no. We'll be using floating points.

Floating points have less precision than integers, but they can cover numbers
way bigger and way smaller than integers themselves.

All that we need to do is add a decimal place to `n1` and `n2`:

```rust
{{#include ../../learnrust/src/bin/fibonacci.rs:loop2_float}}
```

Yay! That worked!

```
0,1,1,2,3,5,8,13,21,34,55,89,
144,233,377,610,987,1597,2584,4181,6765,10946,
17711,28657,46368,75025,121393,196418,317811,514229,832040,1346269,
2178309,3524578,5702887,9227465,14930352,24157817,39088169,63245986,102334155,165580141,
267914296,433494437,701408733,1134903170,1836311903,2971215073,4807526976,7778742049,12586269025,20365011074,
32951280099,53316291173,86267571272,139583862445,225851433717,365435296162,591286729879,956722026041,1548008755920,2504730781961,
4052739537881,6557470319842,10610209857723,17167680177565,27777890035288,44945570212853,72723460248141,117669030460994,190392490709135,308061521170129,
498454011879264,806515533049393,1304969544928657,2111485077978050,3416454622906707,5527939700884757,8944394323791464,14472334024676220,23416728348467684,37889062373143900,
61305790721611580,99194853094755490,160500643816367070,259695496911122560,420196140727489660,679891637638612200,1100087778366101900,1779979416004714000,2880067194370816000,4660046610375530000,
7540113804746346000,12200160415121877000,19740274219868226000,31940434634990100000,51680708854858330000,83621143489848430000,135301852344706760000,218922995834555200000,354224848179262000000,573147844013817200000,
```

Okay, I know. This is not really helpful. This program might be somewhat helpful
to some mathematician, but not for you and not for me either.

But we have to endure these silly examples for now. At our current level it is 
very hard to find anything to do that it's actually possible with what
I explained so far.

It is important to practice. And these examples will help you get an idea on how
the syntax works.