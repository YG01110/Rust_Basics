Create the project with Cargo. Run this in your terminal:

cargo new hello_world     // This creates a src/main.rs and Cargo.toml file which tells the package manager cargo that it is a Rust project.
cd hello_world


What crago.toml looks like ?

[package]
name = "hello_world"
version = "0.1.0"
edition = "2024"

[dependencies]

Tips while compiling and running:

cargo check
/*
Runs the Rust compiler's syntax and type checker on your code without generating an executable binary. 
Because it skips the final code-generation step, it's much faster than building. Use it constantly while editing to catch errors early.
*/

cargo build
/*
Reads Cargo.toml, resolves and downloads any dependencies (recording exact versions in Cargo.lock), and invokes rustc to compile your code into an executable stored at target/debug/<project_name>. 
This is a debug build— it compiles fast and includes extra debugging info, but runs slower.
*/

cargo build --release
/*
Does the same thing as cargo build, but tells rustc to apply heavy compiler optimizations. 
It takes longer to compile, and saves the much faster, production-ready binary in target/release/<project_name>.
*/

cargo run
/*
A convenience shortcut that runs cargo build (if any of your code changed since the last build) and immediately executes the resulting binary in target/debug/ in one step. 
(You can also run cargo run --release to build and run the optimized version.)
*/

cargo build --profile <name>
/*
If you ever define a custom build profile in Cargo.tomlIt goes into target/<name>/
*/

What is the use of Cargo.lock file and when is it created first?

Its entire job is to guarantee reproducible builds—making sure your code compiles with the exact same library versions on every machine, even months later.
It is created first when you compile the code because at that time it takes a snapshot of the versions used.The next time whoever compiles it uses the same versions.



Cargo.toml = "Here is the acceptable range of versions my code can work with."
Cargo.lock = "Here is the exact version we actually downloaded and compiled.".The next time you build we use these versions only but they should still obey the cargo.toml ranges.

Remember Cargo.toml has higher priority here.



