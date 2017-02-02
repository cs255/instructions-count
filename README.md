Instructions for Writing an Instruction Counting Pass for Rust's MIR

In order to write an MIR transformation pass easily you need three things:

1. A Rust nightly compiler

2. Cargo: the Rust build system

3. The source code for the Rust compiler.

Thankfully there is an easy way to get all three: rustup.

Rustup is the system most Rust hackers use to manage their Rust environment. It's very easy to install:

1. SSH into the cycle machines

2. Copy paste the following into your command line, it will run the install script and set your default rust compiler to nightly

$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly

3. Follow the instructions to add rustc and cargo to your PATH

4. Ensure your rust compiler is on the correct version by making sure the output of the following command contains the word "nightly":

$ rustc -V

5. Make sure the same is true of the next command

$ cargo -V

6. Type the following command to download the rust source code into "~/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/" or something similar

$ rustup component add rust-src

Once you have done the above, you are ready to start writing your first MIR pass. 

This git repository is a rust project that contains everything you need to start writing your own compiler pass.
You need to finish the compiler pass in the "src/lib.rs" file so that it insert dynamic instruction counting tooling into the compiled "src/main.rs" program.
You can use "cargo build" in order to compile the project, and "cargo run" to run it. Your goal is to make the X static variable equal to the number of dynamically executed MIR statements, including Storage, Descriminant, and NOOPs.

Some sample code is in lib.rs to get your started.

If you have any questions, please email jbisnett@u.rochester.edu
