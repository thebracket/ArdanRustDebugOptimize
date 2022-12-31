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
    * In the third hour:
        * We'll go over some of the more common bugs encountered by newcomers to the Rust world.
        * Floating point numbers.
        * Unicode.
        * Heap vs. Stack Storage.
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
    * We're going to talk about common bugs and how to avoid them.
    * We've already seen the importance of unit tests, Rust has a lot more to offer in this regard.
    * We're going to take this hour to tour some of the more common bugs encountered by Rust programmers.
* Floating Point Precision
    * Many higher-level languages shield you from the intricacies of IEEE 754 floating point numbers. Rust doesn't.
    * Rust offers really high performance floating point code. It's tempting to use it everywhere. Be careful.
    * Floating point numbers are inherently *approximations*. NEVER use them for money!
    * Open https://replit.com/@HerbertWolverso/FloatingPointAddition#src/main.rs
    * 0.1 + 0.2 really should equal 0.3
    * Add `println!("{}", 0.1 + 0.2)`. `0.30000000000000004` probably wasn't what you meant.
    * Rust has your back.
        * Open the `floating_imprecision` project.
        * Enable strict checks with `#[warn(clippy::pedantic)]`.
        * Clippy now warns that the floating point comparison will fail, and provides an explanatory link: https://rust-lang.github.io/rust-clippy/master/index.html#float_cmp
        * Add EPSILON check.
    * So what do we do about this?
        * As Clippy recommended, use EPSILON. 
            * The `float_cmp` crate can make this easier.
        * For money, don't use floats. Instead:
            * Use an integer, multiplied by 100. Faster, accurate. Just be careful to pick a rounding strategy that your accountants like.
            * Use one of the decimal crates: `rs-decimal` or `bigdecimal` are popular. `bigdecimal` is used by Diesel for database integrations.
                * See https://replit.com/@HerbertWolverso/Decimal#src/main.rs
                * This is similar to the `Decimal` type in C#
        * Consider the precision you *need*. Are you ok with approximate answers? It's a problem if you are counting money, not so bad if you are estimating the distance to Mars---but really bad if you are trying to insert a probe into Mars orbit.
* Unicode
    * If you are coming from C or similar low-level languages, you probably expect `char` to be 8 bits.
    * `u8` is 8 bits (as is `i8`). `char` is variable: it contains exactly one `UTF-8` character.
    * Go to https://replit.com/@HerbertWolverso/StringLength#src/main.rs
        * Strings are really a *vector* of `u8` entries. `String::len()` returns the number of *bytes* required to store the string.
        * Characters are as large as they need to be for a unicode character. You have to use `chars().count()` to count the actual characters.
        * Even then, there are some phrases that can be encoded in more than one way---so you can't always trust the count!
    * This makes multi-lingual programmer easier---and can lead to some very subtle bugs when transmitting data.
* Arrays just love being on the stack
    * Not every language makes you worry about the difference between stack and heap. Rust is a systems language, you have to care.
    * See https://replit.com/@HerbertWolverso/StackArrays#src/main.rs
        * This crashes because `my_array` is entirely on the stack: and even though you have tons of memory, your stack is constrained to 64kb by default on Linux.
    * So what can we do about it?
        * Let's try allocating a `Box` and onto the stack. That didn't work either!
        * Use a vector: `vec!` instead of an array, vectors are automatically heap allocated.
        * If you really want a non-resizable container without the overhead, you can call `into_boxed_slice()`.
* Wrap-Up 3rd Hour
    * We've looked at some of the more common bug types: Floating point, unicode and memory allocation.
    * Take a 12-minute break, we'll be back to finish up the day.

## Fourth Hour

* Introduction
    * We'll wrap up the first day by talking about some of the ways in which the Rust toolchain can help you avoid bugs, and keep your team on course.
* Documentation
    * A lot of bugs are found during *integration*: your library code works, your application code works, but when the two come together---things just aren't what you expect.
    * Usually, this is because the *intent* of the library developer doesn't match the *expectation* of the library consumer.
    * You can't be 100% sure that you'll always solve this: but documentation gets you 90% of the way there.
    * Fortunately, Rust has great documentation tools built in.
        * Open `geometry_undocumented` project.
        * It's a very simple library, but pretty complete: you have an exported public function and a unit test. The unit test even takes into account floating-point precision by using a formatted string to limit the output.
        * You can probably guess what this does without documentation, but we'll use it for an example.
        * Let's find the missing documentation. Add `#![warn(missing_docs)]`. The exclamation mark makes the directive apply to the whole project.
        * Now open `geometry_documented`:
            * `//!` at the top defines *crate-level* documentation. This describes the purpose of your library.
            * Documentation uses Markdown, just like Github.
            * Function documentation should include:
                * A description of what the function does.
                * All arguments, and what they are for.
                * What is returned.
            * You can now mouse-over the function and see documentation in your IDE.
            * Now run `cargo doc --no-deps --open`
                * If you forget `--no-deps`, Rust will document *everything* the crate uses as a dependency. This can take a while.
                * Your browser opens with a searchable HTML documentation system.
                * It's a great idea to include this in your Continuous Integration system to ensure everyone stays up-to-date.
                * If you publish a crate, the Rust crate's system automatically publishes your documentation to `docs.rs`.
        * Searchable documentation is great for details---but many people learn better from examples. Fortunately, Rust includes an `examples` system as well.
            * `geometry_documented` includes an additional folder, next to `src`: `examples`
            * `examples/area.rs` contains a simple example of using our library function.
            * `cargo run --example` lists available examples.
            * `cargo run --example area` runs the example.
* Unit Testing your Documentation
    * A major problem with in-source documentation is that it can grow stale. What if you change a function and forget to update the comments? There are volumes of text talking about this problem online.
    * Open the `geometry_documented_full` project.
        * The `area_of_a_circle` function now includes a test in the documentation itself.
        * Run `cargo test` and the documentation examples are *also* tested.
        * Now let's comply with a Kansas school board, in the 1990s when they defined PI as equal to 3.
* Code Coverage
    * https://blog.rng0.io/how-to-do-code-coverage-in-rust
* Wrap-Up

Debugging
Avoiding Bugs
Optimization
Distributed Applications
