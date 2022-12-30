# Ultimate Rust: Debugging & Optimizing Rust

**Requirements**

* Rust installed via `rustup.rs`
* Visual Studio Code installed
    * Rust Analyzer Extension installed
    * C++ Extension installed (from Microsoft) OR `CodeLLDB` extension

## Day 1: Debugging

### First Hour

* Introduction
    * Today we're going to talk about debugging Rust programs.
    * In the first hour:
        * We'll start with a program that frequently trips up newcomers to Rust: asking a user for their name.
        * We'll walk through two strategies for debugging the program:
            * Using `println!` statements.
            * Using an integrated debugger.
        * Then we'll break the program into functions and use Rust's built-in unit testing to ensure we never have this problem again.
    * In the second hour:
        * We'll expand on `println!` debugging, learning to use pretty printing for data types.
        * We'll implement `Display` to make printing structures easy.
        * We'll look at how to nest structures with `Debug` and `Display` for complex data.
        * We'll replace `println!` with a logging system that supports `syslog` and other logging systems.
* Let's debug a simple program
    * REPL examples [here](https://replit.com/@HerbertWolverso).
    * Using `println` in a REPL (live!)
        * `src/buggy_input` project.
        * https://replit.com/@HerbertWolverso/HelloWhatsYourName-Fail#main.rs
        * Run the program, type `Herbert`. It didn't greet me, even though I carefully matched the requested case!
        * Add a `println!("[{buffer}])");` call.
            * Now you see `[Herbert(new line)]`
        * Add a `println!("[{:?}]", buffer.chars());` call.
            * Now you see `[Chars(['H', 'e', 'r', 'b', 'e', 'r', 't', '\n'])]`
            * Aha - the enter character is in the string!
        * Now try `if buffer.trim() == "Herbert" {` - and it works!
    * Connecting a Debugger
        * Don't forget to install `CodeLLDB` (or `Microsoft C++`)
        * Open Settings (`Ctrl` + `,`)
        * Search for "everywhere"
        * Set "allow breakpoints everywhere" to checked/true.
        * Set a breakpoint on line 9
            * Hover over the left area, red marker appears. Click to set the breakpoint.
            * Open the command palette (`ctrl` + `shift` + `P` or `View -> Command Palette`)
            * Select "Rust-Analyzer: Debug"
            * Select "run buggy-input"
            * Quick tour of the debug window
            * You can see the problem by hovering the mouse over `buffer`
    * Clean up the code to not have this problem again
        * Open `buggy_input_cleaned`
            * We've moved string cleaning into its own function.
            * We've moved reading from stdin into its own function.
                * We can't readily unit test keyboard input, so keep them separated.
            * We've added unit tests to cover the majority of cases
            * Live demo: https://replit.com/@HerbertWolverso/HelloWhatsYourName-Clean#src/main.rs
    * Hour Wrap-Up
        * In this session you've used two strategies for debugging a simple program:
            * `println!` debugging.
            * Using an integrated debugger.
        * You've learned to create unit tests to validate that your sanitizer does what you expect.
        * Take 12 minutes, we'll be back shortly.

## Second Hour

* Printing and Logging
    * Logging with `println!` is useful.
        * You can track progress through a program by printing periodically.
        * `println!` is thread-safe.
        * Sometimes, attaching a debugger is impractical - you can always fall back on logging.
    * Printing structures
        * Open `pretty_printing` project.
        * Notice that we've changed `clean_string` to take a reference.
        * We've added a `User` structure and a pre-populated list of users.
        * We use the `find` iterator function to see if the requested user exists.
            * If it does, we print each structure entry one line at a time.
            * If it doesn't, we bail out.
        * That's not very ergonomic. How can we easily print the user's details?
        * Option 1: Deriving debug
            * Open `pretty_print_debug` project.
            * We've added `#[derive(Debug)]` to `User`
            * Replace the printing with `println!("{:?}", user),`
            * It now outputs `User { name: "Herbert", full_name: "Herbert Wolverson", age: 47 }`
            * You can always use `{:?}` to print a structure that supports debug types.
            * You can use `{:#?}` to use a default formatter that breaks up the output.
            * Notice that Clippy (the linter) is now warning you that you aren't using the other fields.
        * Option 2: Implementing Display
            * Open the `pretty_print_format` project.
            * We've left `#[derive(Debug)]` - it's useful and doesn't add much weight. Release mode will remove it if it is unused.
            * We've imported `use std::fmt::{Display, Formatter};`.
            * We've added `impl Display for User` and a format block.
            * You have to specify the output (`f`), the formatter can be used for writing to files, the screen, other strings, even `panic!` messages. Anything that supports the formatter for output.
            * No more warnings!
            * We can now print the user with just `println!("{user}")`
    * Nesting Structures
        * Open the `pretty_nested` project.
        * Add an `Age` structure containing `birth_year`.
        * Adjust `User` to store an age, and the test record to setup Herbert with the correct year.
        * Notice that it instantly flags as an error: age doesn't support `Display` or `Debug` - Rust detects this, and since the *parent* structure has these properties --- you can't compile until you apply them.
        * Deriving `Debug` fixes the first error.
        * Adding a `Display` implementation fixes the second.
        * Your program runs as before, but is now performing a calculation in the age display.
            * In the real world, you probably care about birth month and day --- but we're keeping it simple.
    * Logging
        * In a real application, you want to log errors rather than just printing.
        * The system is very similar, and all of your `Debug` and `Display` work still applies.
        * Open the `logging` example.
        * In `Cargo.toml`, we've added a dependency on the `log` crate. This is the most commonly used logging system, supporting UNIX `syslog`, `stdout`, `stderr` and configurable logging.
        * We've also imported `env_logger`, which allows you to set logging levels with environment variables.
        * Adding `env_logger::init();` is necessary to read the environment variables and setup logging.
        * We've replaced the `println!` statements with calls to `log::info!` and `log::error!`. Notice that the format code we created still works.
        * Run the program, and there is no output!
        * You need to set an environment variable to to indicate your log level.
            * On *NIX, `RUST_LOG=debug cargo run`
            * On Windows PowerShell, `$env:RUST_LOG='debug' ; cargo run`
        * Now when you run the program, you see nicely colored, timestamped log entries.
        * You can learn about different logging strategies [here](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html)

    * Wrap-Up 2nd Hour
        * You've learned to pretty-print structures, including nested structures.
        * You've learned to use `Display` to customize print output for structures.
        * You've learned to use `log` and `env_logger` for fully customizable logging.
        * Take a 12-minute break and we'll talk about avoiding bugs

## Third Hour

* Introduction

Debugging
Avoiding Bugs
Optimization
Distributed Applications
