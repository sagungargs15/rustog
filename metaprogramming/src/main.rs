// 1. Simple Declarative Macro
macro_rules! say_hello {
    // Match zero arguments
    () => {
        println!("Hello!");
    };
    // Match one argument
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    // Match multiple arguments
    ($($name:expr),*) => {
        $(
            println!("Hello, {}!", $name);
        )*
    };
}

// 2. Create a simple struct builder macro
macro_rules! make_struct {
    ($struct_name:ident { $($field_name:ident : $field_type:ty),* }) => {
        #[derive(Debug)]
        struct $struct_name {
            $($field_name: $field_type),*
        }

        impl $struct_name {
            fn new($($field_name: $field_type),*) -> Self {
                $struct_name {
                    $($field_name),*
                }
            }
        }
    };
}

fn main() {
    // Using the say_hello macro
    say_hello!();                    // Prints: Hello!
    say_hello!("Alice");            // Prints: Hello, Alice!
    say_hello!("Bob", "Carol");     // Prints: Hello, Bob! Hello, Carol!

    // Using the make_struct macro
    make_struct!(Person {
        name: String,
        age: u32
    });

    let person = Person::new(String::from("Alice"), 30);
    println!("{:?}", person);  // Prints: Person { name: "Alice", age: 30 }
}