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

### 2.1.2 Fail Fast

* Panic rather than accept questionable results
* Different ways to panic
* Panic in threads
* Panic Handlers
* The difference from exceptions

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
