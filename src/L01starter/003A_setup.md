# Setting up the computer

So you reached this point! I guess we're doing it. Let's stop the chatter and bring in the real stuff. Let's install Rust.

## Installing Rust

Get your browser and go to <https://rustup.rs/>. Follow the instructions there.
* For Windows, there's a `rust-init.exe` that you download and will install everything.
* For Linux (my case), there's a single command line that you copy and paste into the console.
<!--- TODO: And this is a security risk that some people don't want to take. Add an alternative option. -->

(By the way, if you want to be a developer, you also need to make the terminal your friend)

That will be all we need to do. Congrats! You installed Rust in your machine!

> `Chapter 1.1 - Installation` of the Holy Book contains the installation instructions.

## Choosing an editor for Rust

I'm very opinionated here. Get Visual Studio Code by Microsoft. 
(not to be confused with MS Visual Studio)

People call it VS Code or just "Code". 
It supports nearly all languages, the source-code is open, and works really, really well.

> By default, the binaries provided by Microsoft are proprietary and include telemetry.
> 
> If you wish to avoid this, you can get [VSCodium](https://vscodium.com/), a free/open fork that is built without any Microsoft customizations.

From those that code with Rust, they mainly use VS Code or Vim. 
And I'm not going to recommend Vim to anyone. 
It is an excellent program, but geared towards very senior people. 
So VS Code it is.

I have more than 15 years coding, and I do use VS Code. It's great. 
On the other hand, I almost never use Vim: 
it requires a lot of investment that I don't want to commit to.

Now, go to the extensions panel and search for "rust-analyzer" and install it. 
This is all you need to get the best Rust experience.

![VSCode Extensions](./img/vscode_extensions.png)

> It is really important to have Rust properly installed at this point, or rust-analyzer will fail.
