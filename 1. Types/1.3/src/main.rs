fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// Prefer Option and Result transforms over explicit match expressions

    // Sometimes match statements are unnecessary. For example if only the value is relevant and
    // the absence of the value can be ignored:
    struct File {
        title: Option<String>,
    }

    let file = File { title: Some(String::from("report.pdf")) };
    match &file.title {
        Some(t) => println!("fields is {t}"),
        None => {},
    }

    // Use an if let expression in this case:
    if let Some(t) = &file.title {
        println!("fields is {t}");
    }

    // In the case of a Result<T, E> a match expressions can be used to handle errors:
    let result = std::fs::File::open("/etc/passwd");
    let f = match result {
        Ok(f) => f,
        Err(_e) => panic!("Failed to open /etc/passwd!"),
    };
    // Assume `f` is a valid `std::fs::File` from here onward.

    // Both Option and Result provide a pair of methods that extract their inner value and panic!
    // unwrap and expect.
    let f = std::fs::File::open("/etc/passwd").unwrap();
    // expect is the same but with a custom error message:
    let f = std::fs::File::open("/etc/passwd").expect("Could not open /etc/passwd");

    // The key ingredient for reducing boilerplate code in Rust's question mark operator. This
    // piece of syntactic sugar takes care of matching the Err arm, transforming the error type
    // if necessary and building the return Err(...) expression, all in a single character:
    pub fn find_user(username: &str) -> Result<UserId, std::io::Error> {
        let f = std::fs::File::open("/etc/passwd")?;
        // ...
    }
    // There generally no cost to these apparent method invocations. They are all generic functions
    // marked as #[inline] so the generated code will typically compile to machine code that's
    // identical to the manual version.
    //
    // This means that you should always prefer Option and Result
    // transforms over explicit match expressions.

    // If a function accumulates errors from a variety of different libraries, use map_err:
    pub fn find_user(username: &str) -> Result<UserId, String> {
        let f = std::fs::File::open("/etc/passwd")
            .map_err(|e| format!("Failed to open password file: {:?}", e))?;
        // ...
    }

    // Use .as_ref() as needed when transformations involve references:
    struct InputData {
        payload: Option<Vec<u8>>,
    }

    impl InputData {
        pub fn encrypted(&self) -> Vec<u8> {
            encrypt(self.payload.as_ref().unwrap_or(&vec![]))
        }
    }

    Ok(())
}
