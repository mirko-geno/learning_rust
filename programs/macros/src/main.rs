/*
Macros are chunks of code that are written in other place and can
be called in other place like functions, but they have some differences.

    * Instead of calling a pointer to a function, the chunk of code defined
in the macro is placed in the code during compilation. This means, a macro
doesn't necessarily take ownership of a variable.

    * Macros can take a variable amount of parameters, where functions can
only take a defined amount of them.

    * Macros must be defined or bring into scope before calling them in a file,
as opposed to functions which you can define anywhere and call anywhere.


In some way, they work similar to a match, they compare values to patterns and
decide what to do with them, wether it's transformed, expanded, etc. This
might involve generation new code, duplicating sections, injecting boilerplate
or other compile-time logic.

'macro_rules!' is used to declare a new macro, the #[macro_export] annotation
indicates that this macro is available for use in the crate.
*/


#[macro_export]
macro_rules! custom_vec {
    ( $( $x:expr ), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                println!("Added {} to vector", $x);
            )*
            temp_vec
        }
    };
}


#[macro_export]
macro_rules! multi_println {
    ( $($ex: expr), *) => {
        $(
            println!("expression received is {}", $ex);
        )*   
    };
}


fn main() {
    let vec: Vec<u32> = custom_vec!(2, 3, 5);
    println!("vec: {:?}", vec);

    multi_println!(2, 5.3, String::from("Miles Morales"));
}
