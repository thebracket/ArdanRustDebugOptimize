# Ultimate Rust: Debugging & Optimizing Rust

## Overview

This class is designed to run over 5 days, with each day containing four
1-hour blocks (with a break at the top of each hour).

The first section of the class will cover debugging techniques for Rust, and using Rust to avoid
creating bugs in the first place. The second section focuses on benchmarking, and optimization---both
from a high-level perspective and small optimizations that can make a difference. The class will finish
with an overview of fitting Rust into distributed architectures and debugging/optimizing your overall
pipeline.

**Day 1**                  | **Day 2**                      | **Day 3**                     | **Day 4**                                | **Day 5**
---------------------------|--------------------------------|-------------------------------|------------------------------------------|-------------------------
**[Debugging](./Day1.md)** | **[Avoiding Bugs](./Day2.md)** | **[Optimization](./Day3.md)** | **[High-level Optimization](./Day4.md)** | **[Distributed Applications](./Day5.md)**
[Introduction](./Day1.md#introduction) | [Introduction](./Day2.md#211-introduction) | [Introduction](./Day3.md#311-day-3-introduction)  | [Introduction](./Day4.md#411-introduction) | [Introduction](./Day5.md#511-introduction)
[Debug a Simple Program](./Day1.md#debugging-example) | [Fail Fast](./Day2.md#212-fail-fast)  | [Cargo Profiles](./Day3.md#312-cargo-profiles) | [CPU or I/O Bound?](./Day4.md#412-cpu-or-io-bound) | [Tracing with Tokio](./Day5.md#512-tracing-with-tokio)
[Print Debugging](./Day1.md#122-printing-debug-information) | [Error Handling](./Day2.md#213-rust-error-handling) | [LTO and PGO](./Day3.md#313-profile-guided-optimization-pgo) | [CPU Bound with Rayon](./Day4.md#413-rayon-for-cpu-bound-tasks) | [Logging Strategies](./Day5.md#522-logging-strategies-for-distributed-applications)
[Logging](./Day1.md#123-logging) | [Type Conversions](./Day2.md#222-type-conversion) | [Benchmarking](./Day3.md#322-benchmarking) | [I/O Bound & Async](./Day4.md#422-io-bound-async) | [Finding Bottlenecks](./Day5.md#532-understanding-your-bottleneck)
[Floating Point](./Day1.md#132-floating-point-precision) | [Unit Conversion/New Types](./Day2.md#223-unit-conversions) | [Using a Profiler](./Day3.md#332-using-a-profiler)  | [Blocking Functions](./Day4.md#423-async-blocking-functions)  | [Where Rust can Help](./Day5.md#533-where-rust-can-help)
[Unicode](./Day1.md#133-unicode) | [Range Constraints](./Day2.md#232-newtypes-with-range-constraints) | [Micro-Optimizations](./Day3.md#333-micro-optimizations)  | [Managing the Future](./Day4.md#424-managing-callbacks) | [Rust is Most Useful...](./Day5.md#534-whats-rust-best-at)
[Arrays and the Stack](./Day1.md#134-arrays-and-the-stack) | [Fearless Concurrency](./Day2.md#233-fearless-concurrency) | [Memory Allocation](./Day3.md#342-memory-allocation) | [IO *and* CPU bound](./Day4.md#432-io-and-cpu-bound-problems) | [Rust is Least Useful...](./Day5.md#535-whats-rust-worst-at)
[Documentation](./Day1.md#142-documentation) | [Buffer Overruns](./Day2.md#242-buffer-overruns) | [Iterators](./Day3.md#343-fixed-size-iterators) | [Wrong Algorithm?](./Day4.md#433-is-the-algorithm-the-problem) | [Gradually Introducing Rust](./Day5.md#542-introducing-rust-to-your-architecture)
[Documentation Testing](./Day1.md#143-documentation--testing) | [Overflow](./Day2.md#243-checking-for-overflow) | [Slicing & Concurrency](./Day3.md#344-slicing-and-concurrency) | [Speed vs. Accuracy](./Day4.md#442-speed-vs-accuracy---its-always-a-trade-off) | [Microservices](./Day5.md#543-microservices)
[Code Coverage](./Day1.md#144-code-coverage) | [Use-after-Move/Lifetimes](./Day2.md#244-use-after-move-and-lifetimes) | [Wrap-Up](./Day3.md#345-day-3-wrap-up) | [Feature Flags](./Day4.md#443-letting-the-user-decide-with-feature-flags) | [FFI and Other Languages](./Day5.md#544-ffi-and-other-languages)
[Wrap-Up](./Day1.md#145-day-one-wrap-up) | [Wrap-Up](./Day2.md#245-day-two-wrap-up) | | [Wrap-Up](./Day4.md#444-day-four-wrap-up) | [Final Wrap-Up](./Day5.md#545-class-wrap-up)

## Requirements/Prerequisites

To participate in the workshop/code-along portions of this class, you will need:

* Rust installed via `rustup.rs`
    * Verify that `cargo` runs.
    * Verify that you can run "hello world".
* Visual Studio Code installed
    * Rust Analyzer Extension installed
    * C++ Extension installed (from Microsoft) OR `CodeLLDB` extension

> If you prefer to use a different development environment, you may do so---but some (small) parts of the class will refer to Visual Studio Code features. You may have to find equivalent functionality for your environment.
