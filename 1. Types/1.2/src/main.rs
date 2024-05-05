// The mechanisms described in this Item will generally feel familiar,
// as they all have direct analogs in other languages:
//
// Functions: The universal mechanism for associating a chunk of code with a name and a parameter
//          list.
// Methods: Functions that are associated with an instance of a particular data structure.
//          Methods are common in programming languages created after object-orientation
//          arose as a programming paradigm.
// Function pointers: Supported by most languages in the C family, including C++ and Go,
//          as a mechanism that allows an extra level of indirection when invoking other code.
// Closures: Originally most common in the Lisp family of languages but have been
//          retrofitted to many popular programming languages,
//          including C++ (since C++11) and Java (since Java 8).
// Traits: Describe collections of related functionality that all apply to the same underlying item.
//         Traits have rough equivalents in many other languages,
//         including abstract classes in C++ and interfaces in Go and Java.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // FUNCTIONS AND METHODS

    // Return x divided by two
    fn div(x: f64, y: f64) -> f64 {
        if y == 0.0 {
            return f64::NAN;
        }
        // The last expression is the return value
        X / Y
    }

    fn show(x: f64) {
        println!("x = {x}");
    }

    enum Shape {
        Rectangle { width: f64, height: f64 },
        Circle { radius: f64 },
    }

    impl Shape {
        // Method
        pub fn area(&self) -> f64 {
            match self {
                Shape::Rectangle {width, height} => width * height,
                Shape::Circle {radius} => std::f64::consts::PI * radius * radius,
            }
        }
    }

    // A &self parameter indicates that the contents of the data structure may be read from
    // but will not be modified.
    // A &mut self parameter indicates that the method might modify the contents of the data structure.
    // A self parameter indicates that the method consumes the data structure.

    // FUNCTION POINTERS
    // A function pointer is a pointer to some code, with a type that reflects the signature.
    // The bare function pointers are limiting, because the only inputs available to the invoked
    // function are those that are explicitly passed as parameter values.

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    let fun: fn(i32, i32) -> i32 = sum;

    // CLOSURES
    // A closure is a chunk of code that looks like the body of a function definition, but can
    // be built from an expression. It can also capture parts of the surrounding environment.

    let amount = 3;
    let add_num = |y| {
        y + amount
    };
    let z = add_num(23);
    assert_eq!(z, 26);

    // A closure can't be used where a function pointer is expected.
    // Instead, the code that receives the closure has to accept an instance of one of the Fn* traits:
    pub fn modify_all<F>(data: &mut [u32], mut mutator: F)
        where
            F: FnMut(u32) -> u32,
    {
        for value in data {
            *value = mutator(*value);
        }
    }

    // FnOnce: Describes a closure that can be called only once. If some part of the environment is
    //         moved into the closure's context, and the closure's body subsequently moves it
    //         out of the closure's context, then those moves can happen only once—there's
    //         no other copy of the source item to move from—and so the closure can be invoked only once.
    // FnMut:  Describes a closure that can be called repeatedly and that can make changes to its
    //         environment because it mutably borrows from the environment.
    // Fn:     Describes a closure that can be called repeatedly and that only borrows values from
    //         the environment immutably.
    // Or:
    //      FnOnce = Any moved values
    //      FnMut  = Any mutable references to values (&mut T)
    //      Fn     = Only normal references to values (&T)

    // TRAITS
    // A trait defines a set of related functions that some underlying item makes
    // publicly available. The functions are typically methods:

    pub trait Sort {
        fn sort(&mut self);
    }

    // Trait bounds are used to express requirements on the types used in generics:
    pub fn dump_sorted<T>(mut collection: T)
        where
            T: Sort + IntoIterator,
            T::Item: std::fmt::Debug,
    {
        // Next line requires `T: Sort` trait bound.
        collection.sort();
        // Next line requires `T: IntoIterator` trait bound.
        for item in collection {
            // Next line requires `T::Item : Debug` trait bound
            println!("{:?}", item);
        }
    }

    Ok(())
}