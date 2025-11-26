// Here is a demonstration of how to tie methods to new type (struct)
// This is analogous to classes in Python (in some ways)

#[derive(Debug)]
struct CarRace {
    name: String,
    laps: Vec<i32>,
}

impl CarRace {
    // No receiver, a static method.
    // This is like the class constructor.
    fn new(name: &str, vec: Vec<i32>) -> Self {
        Self { name: String::from(name), laps: vec }
    }

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
    let mut race = CarRace::new("Monaco Grand Prix", Vec::new());
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);
}
