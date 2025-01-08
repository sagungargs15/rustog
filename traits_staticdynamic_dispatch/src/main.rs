trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog { name: String }
struct Cat { name: String }

impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof!")
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("Meow!")
    }
}

fn main() {
    // STATIC DISPATCH (Monomorphization)
    fn static_make_noise<T: Animal>(animal: &T) {
        println!("Static dispatch: {}", animal.make_sound());
    }
    
    // DYNAMIC DISPATCH (Virtual Table)
    fn dynamic_make_noise(animal: &dyn Animal) {
        println!("Dynamic dispatch: {}", animal.make_sound());
    }
    
    let dog = Dog { name: String::from("Buddy") };
    let cat = Cat { name: String::from("Whiskers") };
    
    // Static dispatch examples
    static_make_noise(&dog);  // Compiler creates Dog-specific version
    static_make_noise(&cat);  // Compiler creates Cat-specific version
    
    // Dynamic dispatch examples
    dynamic_make_noise(&dog); // Uses vtable lookup at runtime
    dynamic_make_noise(&cat); // Uses vtable lookup at runtime
    
    // Static dispatch with collections (must be same type)
    let static_animals: Vec<Dog> = vec![
        Dog { name: String::from("Rover") },
        Dog { name: String::from("Rex") }
    ];
    
    // Dynamic dispatch with collections (can mix types)
    let dynamic_animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: String::from("Buddy") }),
        Box::new(Cat { name: String::from("Whiskers") })
    ];
}