use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // FUNDAMENTAL TYPES

    // Putting a larger integer type (i32) into a smaller integer type (i16)
    // Will generate a compile-time error
    let x: i32 = 42;
    let y: i16 = x;

    // error[E0308]: mismatched types
    //     --> src/main.rs:18:18
    //     |
    //     18 |     let y: i16 = x;
    // |            ---   ^ expected `i16`, found `i32`
    // |            |
    //     |            expected due to this
    //     |
    //     help: you can convert an `i32` to an `i16` and panic if the converted value
    // doesn't fit
    //     |
    //     18 |     let y: i16 = x.try_into().unwrap();
    // |                   ++++++++++++++++++++

    // Fix:
    let x: i32 = 42;
    let y: i16 = x.try_into().unwrap();
    // Or:
    let x: i32 = 42;
    let y: i32 = x;

    // Rust doesn't allow some things that appear "safe", such as putting a value
    // from a smaller integer type into a larger integer type
    let x = 42i32; // Integer literal with type suffix
    let y: i64 = x;

    // error[E0308]: mismatched types
    //     --> src/main.rs:36:18
    //     |
    //     36 |     let y: i64 = x;
    // |            ---   ^ expected `i64`, found `i32`
    // |            |
    //     |            expected due to this
    //     |
    //     help: you can convert an `i32` to an `i64`
    // |
    //     36 |     let y: i64 = x.into();
    // |                   +++++++

    // AGGREGATE TYPES
    // 1. Arrays:  Holds multiple instances of a single type
    // 2. Tuples:  Hold instances of multiple heterogeneous types
    // 3. Structs: Also hold instances of heterogeneous types known at compile time but allow
    //             both the overall type and the individual fields to be referred to by name.

    // Struct with two unnamed fields
    struct TextMatch(usize, String);
    let t = TextMatch(4, "str_to_match".to_owned());
    assert_eq!(t.0, 4); // Access by field number

    // ENUMS
    enum HttpResultCode {
        Ok = 200,
        NotFound = 404,
        Unauthorized = 403,
        InternalServerError = 500
    }

    let result_code = HttpResultCode::Unauthorized;
    assert_eq!(result_code as i32, 403);

    // can improve readability:
    // Instead of print_page(/* both_sides= */ true, /* color= */, false)
    // You can do:
    pub enum Sides {
        Both,
        Single
    }

    pub enum Output {
        BlackAndWhite,
        Color
    }

    // then apply to function
    pub fn print_page(sides: Sides, color: Output) -> (Sides, Output) {
        (sides, color)
    }
    print_page(Sides::Both, Output::BlackAndWhite);

    // Can be used in match statement:
    let output = Output::Color;
    let output_str = match output {
        Output::Color => "color",
        Output::BlackAndWhite => "black_and_white",
        _ => "No output"
    };

    // ENUMS WITH FIELDS
    // In Rust each enum variant can have data along with them. It acts as
    // an algebraic data type.

    pub struct Job;
    pub struct CpuId(i32);
    pub enum SchedulerState {
        Insert,
        Pending(std::collections::HashSet<Job>),
        Running(std::collections::HashMap<CpuId, Vec<Job>>),
    }

    // Wrong:
    pub struct RgbColor(i32, i32, i32);
    pub struct DisplayProps {
        pub x: u32,
        pub y: u32,
        pub monochrome: bool,
        pub fg_color: RgbColor,
    }

    // Good:
    pub struct RgbColor(i32, i32, i32);
    pub enum Color {
        Monochrome,
        Foreground(RgbColor),
    }

    pub struct DisplayProps {
        pub x: u32,
        pub y: u32,
        pub color: Color
    }

    // OPTION<T>
    // Is there a value of a particular type (Some(T))? Or not (None)
    pub struct Job {
        pub cpus: Option<Vec<i32>>
    }
    let new_job = Job { cpus: Some(vec![10, 20, 30]) };
    assert!(new_job.cpus.is_some());

    Ok(())
}