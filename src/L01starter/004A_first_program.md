# Your first program

Before doing anything, it is important to have a folder where you put all your programs. 
So go ahead and create a folder with the name you like (i.e., "programs")[^1].

A few tips though:

* Prefer to use lowercase only for the folder, no spaces. 
  Use an underscore if you need to separate words, but I would 
  recommend writing something short.

* Place the folder somewhere that has a short, easy path. Don't place it on your 
  Desktop. On Windows, "C:\" might be better as the home folder contains a path with spaces.

Now open a terminal and go to this folder. 
Using `cd ..` and `cd your_folder_name` should do the trick. 
If you followed my tips, this should be an easy task.

Once you're set, run the following command:

```console
$ cargo new learnrust
     Created binary (application) `learnrust` package
```

This will create a folder called `learnrust`, you should be able to find it with 
your file explorer. 
Inside, there are a few folders and files. 
This is how an empty Rust program looks.
<!-- Screenshot of the file manager? -->

It already includes a program in src called `main.rs` (we'll check out the contents later). 
And it can be executed. 

Do `cd learnrust` on the console to get into the folder, then run:

```console
$ cargo run
```

The output will be similar to this:

```console
$ cargo run
Compiling learnrust v0.1.0 (/home/deavid/git/rust/learnrust)
Finished dev [unoptimized + debuginfo] target(s) in 0.34s
  Running `target/debug/learnrust`
Hello, world!
```

The program has been built and executed. The program output is "Hello, world!".

Congrats! You just wrote your first program. (more or less, hah)

If you execute "cargo run" again, then:

```console
    $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/learnrust`
    Hello, world!
```

Notice how something is different. 
At first, it compiled the program, then executed it. 
On the second time, it noticed that the program was unchanged, so 
it was run without compiling it again.

This is covered in `Chapter 1.2 - Hello World!` of the holy book.

## What is this cargo command?

Surely you noticed that we did "cargo run" and not "rust run". 
Cargo is like the swiss-army knife of Rust, it will simplify all our processes 
during coding, and removes a lot of stuff that we don't need to learn.

Main things that it does:

* Instead of compiling the program with `rustc`, it's just `cargo build`. 
  `rustc` will need flags, and it's a bit tricky to do right, cargo makes this super simple.
* Cargo built the program in `./target/debug/` or `./target/release`. 
  We could run these directly, but we would need to remember to build them first. 
  `cargo run` does this for us.
* We might need to download libraries for additional stuff 
  (this is common in all programming languages). Instead of doing this manually, 
  we can declare in `Cargo.toml` which ones do we want, and when 
  we do `cargo build` it will download anything missing automatically.

There's much more than this, but for now this is what we will be using. 
The bottom line is that we will always use `cargo` and forget about the other commands. 
That's fewer things to remember.

As a side note, if you installed with `rustup.rs` as I recommended, there's 
also a `rustup` command. This is used to update rust and cargo themselves. 
If you need to update, just run `rustup update` and everything will be done 
automatically. 
It's a good idea to do this once a month or so, but if you don't do it is also fine.

[^1]: Personally I use [Git](https://git-scm.com/) for everything, and I have 
the folders structured as `/home/deavid/git/rust/project_name`.
