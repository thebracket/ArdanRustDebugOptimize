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
* Dynamic Results
* Anyhow
* Propagating Errors with `?`
* Ignoring Errors
* Performance Impact of Error-Handling

### 2.1.4 First Hour Wrap-Up

* TODO
* Take a short break (12 minutes)

## 2.2 Second Hour

### 2.2.1 Introduction

* In this hour, we will talk about two of the most common sources of errors in what looks like valid code:
    * Type Conversion
    * Units of Measurement

### 2.2.2 Type Conversion

* Type Conversions
    * `as` is easy, but can be wrong
    * `into()` for easy conversions
    * `try_into()` for conversions that might fail
    * The magic `From` conversion

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
