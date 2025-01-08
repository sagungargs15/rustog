
// Define a basic trait
trait Animal {
    // Abstract method (must be implemented)
    fn make_sound(&self) -> String;
    
    // Method with default implementation
    fn introduce(&self) -> String {
        format!("I make this sound: {}", self.make_sound())
    }
}

// Simple struct for a Dog
struct Dog {
    name: String,
}

// Simple struct for a Cat
struct Cat {
    name: String,
}

// Implement the Animal trait for Dog
impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof!")
    }
    
    // We can override the default implementation
    fn introduce(&self) -> String {
        format!("I'm {} and I say: {}", self.name, self.make_sound())
    }
}

// Implement Animal for Cat
impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("Meow!")
    }
    // Using default introduce() implementation
}


fn main() {
    // Using the Animal trait
    let dog = Dog { name: String::from("Buddy") };
    let cat = Cat { name: String::from("Whiskers") };
    
    println!("Dog: {}", dog.introduce());
    println!("Cat: {}", cat.introduce());

} 

// 1. More advanced trait features (associated types, supertraits)?
// How to use trait objects with dynamic dispatch?
// How to implement traits for external types?
// 4. How to use trait bounds in generics?