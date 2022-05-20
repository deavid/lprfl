# Floating point gotchas

Let's begin saying that, in math:

\\[ 1.1 + 1.1 + 1.1 = 3.3 \\]

That is obvious.

But in computers:

```rust
let a = 1.1;
let b = 3.3;

let c = a + a + a;

println!("b = {}", b);
println!("c = {}", c);
println!("b == c? -> {}", b == c);
println!("b - c = {}", b - c);

```

`b` is not equal to `c`. The difference is `-0.0000000000000004440892098500626`.

Of course, this is wrong. The difference should have been zero. But this is not
how computers work.

Surprised? We thought computers were stupid glorified calculators, but they 
can't even get a simple sum right.

This is because how decimals work in a computer when using floating points. 
They use base 2 (ones and zeros) to represent the numbers, and when we have a
number that has a decimal point, this becomes a problem.

> If you're not interested on the explanation, just remember that you cannot
> expect any two floats to be equal. You have to check if they are 
> "close enough". Also, before converting them to integer, you first have to
> round them. You might think you have 295.0 but in reality it might be 
> 294.999999999996, and if you just cut the decimals instead of rounding you
> get 294 instead of 295. It feels stupid, because it is.

We need to understand that not all numbers can be written down.

Yes, let that sink in. There are numbers you can't write.

For example \\(\pi\\), we can write `3.1415` but doesn't matter for how long we
try, it will never be the real \\(\pi\\). Because it's irrational.

But on the real numbers it also happens. Even in the rational ones (fractional).

A common example is \\(10 / 3\\):

\\[\frac{10}{3}=3.333333333…\\]

It doesn't matter how long you keep writing, you cannot write it down.

Why?

Well, our system is decimal, base 10. And \\(10 = 2 × 5\\).

This means we can only represent fractions that divide by multiples of 2 and 5.

So, for example the following fractions are representable:

\\[\\frac{1}{25}\\]

\\[\\frac{6}{2560}\\]

But \\(1/3\\) is not, because 3 is a prime, and our decimal system doesn't 
include 3 as a factor.

Now, for a computer that is using a binary system the only factor is 2.

This means that when a number is the result of dividing by 5, we can probably
represent, but the computer cannot.

The number 1.1 is in reality:

\\[\\frac{11}{2 × 5}\\]

The computer can represent \\(11/2=5.5\\) but not \\(11/10=1.1\\).

That will lead to infinite decimals, but a `f64` will cut at some point. And
there you have your imprecision. While your written number comes with full
precision, the one written by the computer can't get that level of precision
regardless of having 20 or 500 decimal places.

Do you think it's lame?

Try writing down:

\\[\\frac{100}{29}\\]
\\[\\frac{100}{53}\\]
\\[\\frac{100}{97}\\]

Were you able to do it? No?

Lame!

Well, that's how your computer feels. Please be respectful. 
They only have base 2 to work with.

## Is this a Rust problem?

No, it is a hardware problem. All languages suffer from this. Some of them are
better than others at hiding this fact.

There exist "Decimal" implementations in some languages including Rust that
use base 10 and get rid of this problem entirely.

But those are much slower, but used in financial applications to prevent errors.

## Why this wasn't addressed?

When they created floating points, computers were very slow and memory was very
limited. Adding support for base 10 decimal numbers would have been near 
impossible and most of the programs of the era wouldn't have enough resources to
run.

Also, floats were invented for scientific purposes, like sending people to the
moon. In this case, high precision was better than having base 10. Because even
if they can't fully represent some numbers, overall they give better precision
to numbers that arise from physics calculations.

So they went with that, and we're stuck with them forever. Nowadays, the 
majority of the applications would benefit from base 10, and there is more than
enough resources to run them in this manner. Yet we keep doing this in software.

I guess that everyone got used to this, and no one cares anymore.

## Conclusion

Comparing if two floats are equal: bad.

Comparing if two floats are not equal: very bad.

Converting to integer without rounding: super-mega bad.

### What to do instead?

**Comparing if two floats are equal:**

Instead, check if the difference between them is very small:

```rust
let a = 1.1;
let b = 1.1;

if (a-b).abs() < 0.00000001 {
    // equal
}
```

**Comparing if two floats are not equal:**

Instead, check if the difference between them is bigger than a very small number:

```rust
let a = 1.1;
let b = 1.1;

if (a-b).abs() > 0.00000001 {
    // not equal
}
```

**Converting to integer without rounding:**

Please round first:
```rust
let a:f32 = 294.999999999;
let b:i64 = a.round() as i64;
```

## Other problems

### Not A Number?

See 0/0.

### Infinity?

See 1/0 and -1/0.

### Positive and negative zeros?

See \\(1 × 0\\) and \\(-1 × 0\\)

### Not sortable

In Rust, if you have data in a table that are floats, and you want to sort this
data, you can't.

Because of `NaN`, floats cannot be sorted. But they can be partially sorted.

### Not infinitely big

### Loss of precision at very big numbers




