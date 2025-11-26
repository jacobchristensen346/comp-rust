// Here is a demonstration of how to tie methods to new type (struct)
// This is analogous to classes in Python (in some ways)

// Here we have the new type
// which will be given methods using 'impl'.
// The fields in this struct become the instance attributes.
#[derive(Debug)]
struct CarRace {
    name: String,
    laps: Vec<i32>,
}

// The methods that will be applied to our new type.
// Like class methods in Python.
// You can choose to call individual methods
// like this... CarRace::add_lap(&mut race, 20)
// You are explicitely passing the 'receiver' (race),
// otherwise known as the instance/object.
impl CarRace {
    // No receiver, a static method.
    // This is the constructor.
    // The constructor is called using 'new' (see main).
    fn new(name: &str, vec: Vec<i32>) -> Self {
        // The constructor initializes an object
        // with values in the struct's fields
        // acting as instance attributes.
        Self { name: String::from(name), laps: vec }
    }

    // Note: 'self' is a keyword or stand-in
    // for self: Self.
    // Like in Python where you could call 'self'
    // anything you want technically
    // you could do the same here
    // such as x: Self.
    // But using 'self' alone is probably convention
    // and most convenient, just like in Python.

    // Also note that 'Self' is a type alias for
    // the struct connected to the impl block.
    // You can use Self anywhere in the block.

    // Exclusive borrowed read-write access to self.
    // This method borrows the instance and can change it.
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    // This method borrows the instance but can only read it.
    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self (covered later)
    // This method takes the instance, but can only read it.
    // It owns the instance now, so the instance is deallocated
    // once the function completes upon call.
    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    // We create an instance of our new type using 'new'
    // which calls the constructor.
    let mut race = CarRace::new("Monaco Grand Prix", Vec::new());
    // Call individual methods with dot notation.
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);
}
