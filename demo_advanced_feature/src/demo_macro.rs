// REF: https://doc.rust-lang.org/book/ch19-06-macros.html

// The term macro refers to a family of features in Rust:
// declarative macros with macro_rules!
// and three kinds of procedural macros:
// - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// - Attribute-like macros that define custom attributes usable on any item
// - Function-like macros that look like function calls but operate on the tokens specified as their argument

// TODO: review https://doc.rust-lang.org/book/ch19-06-macros.html#the-difference-between-macros-and-functions

mod demo_declarative_macro {
    // The #[macro_export] annotation indicates that this macro should be made available
    // whenever the crate in which the macro is defined is brought into scope.
    // Without this annotation, the macro can’t be brought into scope.
    #[macro_export]
    macro_rules! simplified_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
}

mod demo_procedural_macro {
    // TODO: review https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes
}
