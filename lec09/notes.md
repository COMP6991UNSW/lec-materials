# Week 5 Lecture Notes

 - Polymorphism, why?
 - Generic type parameters (parametric polymorphism)
 - Monomorphisation
 - Traits (ad-hoc polymorphism)
 - Static vs. Dynamic Dispatch

## 1. Making Functions Generic

 - Look at smallest_* functions, understand why they're not great
 - Compare to (older) Go, C, Python and Rust in how they deal with the problem of functions for many types.
 - Write a `smallest` function in Rust
 - Compare C++ and Rust's approach to types
 - Understand some syntactical Rust specifics
    - Use the "turbofish" syntax, and where it commonly occurs.
    - Use the `where` syntax, as well as the more concise `<T: Type>` syntax
    - Apply multiple bounds to a single type
 - Briefly discuss Hindley Milner type systems
 
## 2. Discuss Caveats of "smallness"
 
 - Have an existential crisis over what "small" is
 - Briefly discuss Ordering, Equality, and how Rust approaches them

## 3. Sequences

 - Understand approaches to generalise to any number of types
 - Explore the possible edge cases with "any number of types" and approaches to fix this.
 - Implement our own return type to deal with edge cases of comparison.
 - Discuss types as constraints, and types as arguments
 - How to make a function operating on a sequence more generic
 - Syntax in Rust
    - Briefly discuss APIT, RPIT
    
## 4. Monomorphisation and Dynamic Dispatch

 - Understand what Monomorphisation is.
 - Understand what Dynamic Dispatch is.
 - Discuss tradeoffs between the two
 
 
## 3. Interesting Traits

 - PartialEq / Eq
 - PartialOrd / Ord
 - Default
 - Copy, Clone
 - Debug, Display
 - ToString
 - Add / AddAssign
   - Associated Type vs. Parameters
   - Default Types
 - BitAdd / BitAddAssign
 - From, Into, TryFrom, TryInto
 - FromStr, AsRef (AsMut), Borrow (BorrowMut), ToOwned
 - Error
 - Iterator, FromIterator, IntoIterator
 - Discuss how derive macros work
 
 
## 4. Building Our Own Iterator
 - How to implement a Fibonacci Iterator
 - How to implement an iterator for arrays

## 5. Trait Objects
 - Traits must be in scope
 - Understand how traits work
 - Trait Object Rules
 - Self: Sized
 - Constrast Traits with OOP

## Extra Content

 - The Orphan Rule
 - Variadic Functions and Rust
 - Using Types at Runtime
 - Specialisation
