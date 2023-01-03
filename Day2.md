# Day Two: Avoiding Bugs

## 2.1 First Hour

### 2.1.1 Introduction

* In today's classes we are going to discuss:
    * Rust Philosophy: fail fast, crashing is always better than passing bad data.
    * Handling Panics
    * Rust Error Handling and Propagation
    * Type Conversions
    * Unit Conversions and avoiding Unit Bugs with Strong Types
    * Range Constraints
    * Fearless Concurrency
    * The Bugs from which Rust can Save You...
    * ... and the times you still have to worry about them.
* Let's start this hour by talking about failing fast, and error handling.

### 2.1.2 Fail Fast

* It's usually better to fail hard---even crash---than to continue with bad data.
* Bad data is awful: you can spend huge amounts of time cleaning up databases and output files after accidentally storing the wrong result.
* Panic rather than accept questionable results
* Panicking when you encounter bad data is easy
    * Open the `panic` project. REPL: https://replit.com/@HerbertWolverso/Panic#src/main.rs
        * We're pretending to do some long calculations, expecting the value of 4.
        * When we don't find 4, we call the `panic!` macro to terminate execution with an error message.
    * Open the `unwrap` project. REPL: https://replit.com/@HerbertWolverso/Unwrap#src/main.rs
        * We have `x`, which is an `Option`: it either contains data or it doesn't. This is the closest Rust will let you get to a `null` pointer in safe code.
        * When we call `unwrap` on `x`, we are accessing its inner value. In this case, there isn't one: so the program panics. The error message isn't very helpful.
        * Let's replace `unwrap` with `expect("no value for x present")` and run again - now we have a helpful error.
        * We can replace `unwrap` with `unwrap_or(3)` to assign a default value. *Only do this if your default will always make sense.*
* Panic in threads
    * Open the `panic_thread` project. REPL: https://replit.com/@HerbertWolverso/PanicThread#src/main.rs
    * We've wrapped our panicking program in threads, and wait for them all to finish at the end.
    * When we run it, anywhere from 1 to 2 panic messages will appear.
    * The program still terminates.
* Panic Handlers
    * Open the `panic_handler` project. REPL: (TODO: Crashed)
    * We've added `panic::set_hook` - when a panic occurs, this code is executed.
    * Now each thread finishes, the failures list where they went wrong.
* The difference from exceptions
    * Rust error handling isn't exceptions like you find in other languages.
    * `throw`ing an exception is expensive: the stack has to be unwound, moving through each possible catcher.
    * If you `throw` on the hot path, performance is often quite poor.
    * Rust `panic!` jumps to a single exit-point, with a stack trace. This is still slower than just checking for errors.
    * Let's move onto how to handle errors gracefully.

### 2.1.3 Rust Error Handling

* The Result Type - no exceptions!
    * Rust has two ways to express failure:
        * `Option` is the equivalent of `null`: it either contains a value or it doesn't.
        * `Result` contains either a value, or a specific error describing what happens.
    * These differ from exceptions in that they are returned from a function like a regular variable.
    * These differ from checked-exceptions (such as Java) in that ignoring them generates a warning, not a compiler error.
    * Let's look at these in action. Open the project `result_option`.
        * `maybe_success` returns an `Option`. It has either `Some(..)` or `None`. This is good for specifying whether a result exists at all, but gives no clue as to why a failure occurred.
        * `success_or_error` uses a custom type to represent an error, in this case `NotEqualToFour`.
        * It's more verbose, but you can see exactly what went wrong.
* Handling Different types of Error
    * Rust errors are very specific.
    * It's quite possible to wind up needing to handle multiple types of error.
    * Open `multi_error`. REPL: TODO
        * We have two functions returning different types of error.
        * The calling function then returns any error it receives.
        * This requires a LOT of boilerplate:
            * Each error has to implement the `Error` type.
            * Each error has to implement `Display` to print out the error.
            * Errors themselves are wrapped in `dyn Box` - a smart pointer featuring dynamic dispatch.
            * Performance is good, but maintaining a lot of errors can become cumbersome.
        * Let's reduce the boilerplate a bit
    * Open `multi_error_anyhow`. REPL: TODO:
        * We've added a dependency on `anyhow`.
        * `anyhow` provides convenience wrappers to simplify multi-error handling.
        * We've removed the custom error types.
        * The output remains the same.
        * When you care that an error occurred more than you care about a specific type of error, `anyhow` is essential.
* Propagating Errors with `?`
    * Matching on errors can become unwieldy. Rust includes the `?` operator to help.
    * Open `error_propagation`. REPL: TODO:
        * We've replaced all of the `match` and `unwrap` with `?`
        * Question mark automatically unwraps, or propagates the error to the caller if an error occurs.
        * You can only use `?` in functions that return a `Result` type.
        * You can make `main` return a `Result` and use the propagation all the way up.
* Ignoring Errors
    * Sometimes, you don't care if an error occurred. Particularly in server loops, sometimes you just want to try to keep going.
    * If you call an error-returning function without handling the error, Rust gives you a warning.
    * If you *really* don't want to handle the error, `let _ = my_erroneous_function()` will suppress the warning.

### 2.1.4 First Hour Wrap-Up

* In this section, we've talked about:
    * The virtues of failing fast.
    * Panics and handling panics.
    * Rust's error propagation model.
* Take a short break (12 minutes)

## 2.2 Second Hour

### 2.2.1 Introduction

* In this hour, we will talk about two of the most common sources of errors in what looks like valid code:
    * Type Conversion
    * Units of Measurement
* Because they look like valid code, failure can easily trip you up.

### 2.2.2 Type Conversion

* Type Conversions
    * `as` is easy, but can be wrong
        * Open the project `as_is`. REPL: TODO
        * Notice that we are setting `n` to the maximum possible `u64` - and then casting it to a `u32`.
        * We're also setting `n` to a negative number, and casting it to a `u32`.
        * Neither operation makes sense: but `cargo clippy` doesn't give you any warnings, and `cargo run` completes with incorrect results.
        * **This is dangerous**: `as` is very convenient, but you won't receive any warning that it is causing incorrect results.
    * `into()` for easy conversions
        * For always-safe conversions, you can use `into()`. 
        * Open the `as_into` project. REPL: TODO
            * The same conversions fail to compile: the conversions are not guaranteed to be safe.
            * You can always upcast an equivalently signed type. You can't always go back.
            * `into` is very convenient for safe conversion.
            * Change to safe conversions, and it works.
    * `try_into()` for conversions that might fail
        * For conversion between signed and unsigned, or larger types to smaller types---they *might* work.
        * Open `as_try_into`. REPL: TODO.
            * We are now using `try_into` instead of `into`.
            * `try_into` returns a `Result`---so you can treat it like any other result.
                * You can unwrap it
                * Map it
                * Use a default
                * Match it
                * Propagate with `?`
            * Avoid safety problems and use `try_into` for conversions that might fail.
    * The magic `From` conversion
        * The verbosity of type conversions with `into()` everywhere can get messy.
        * You can use Rust's traits system to help.
        * Open `into_traits` project. REPL: TODO.
            * We've created two functions that use the generic type `TryInto<type>`.
            * You can pass any numeric value of a possibly convertible type, and failure will occur at runtime.
            * You've moved the cognitive burden away from your users, and onto the library developer.

### 2.2.3 Unit Conversions

* Unit Conversions
    * Isn't it confusing when a geometry type can take radians or degrees - or doesn't specify distance units?
    * Why type aliases don't help
    * "New Types" - types within a type, with strong type guarantees.
        * Introduce the concept of `MyType(AnotherType)`
        * Create Radians and Degrees example.
        * Expand `into` types to allow for conversion.
        * The `X: Into<Y>` trait requirement for functions.
        * Let's test automatic conversions
    * Expanded Units
        * Credit: https://www.ferrisellis.com/content/rust-implementing-units-for-types/
        * Floating point accuracy problems
        * Create a base `Length` type and `LengthUnit` trait, with conversion to nanometers and an output function.
        * Creating units of length by implementing `LengthUnit`.
        * Implementing conversion from numeric types.
        * Testing with some geometry functions.

### 2.2.4 Second Hour Wrap-Up

* Wrap-Up
    * TODO
    * Take a short break (12 minutes)

## 2.3 Third Hour

### 2.3.1 Introduction

* Introduction

### 2.3.2 NewTypes with Range Constraints

* "New Types" with range constraints

### 2.3.3 Fearless Concurrency

* Fearless Concurrency
    * C++ Example of data race
    * Rust example---it won't compile
    * Rust example with atomics

#### 2.3.4 Third Hour Wrap-Up

* Wrap-Up
    * Take a short break (12 minutes)

## 2.4 Fourth Hour

### 2.4.1 Introduction

* Introduction

### 2.4.2 Buffer Overruns

* Buffer Overruns

### 2.4.3 Checking for Overflow

* Checking for Overflow

### 2.4.4 Use after Move and Lifetimes

* Use after Move and Lifetimes

### 2.4.5 Day Two Wrap-Up

* Wrap-Up
    * See You Tomorrow
