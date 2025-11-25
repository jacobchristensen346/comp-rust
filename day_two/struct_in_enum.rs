// In this script we demonstrate the use of a struct
// within an enum to define multiple enum variants which
// are each associated with one set of pre-determined struct fields.

// The enum gives us flexibility to define the variant we please
// under a common 'category', and then further define the exact
// parameters associated with that variant through the use of structs.

//enum MobType {
//    Drive,
//    Ride,
//}

//enum Fuel {
//    Yes,
//    No,
//}

// Vehicle parameters shared by all vehicles,
// but each vehicle can have its own unique values assigned to each field.
struct Vpar {
    name: String,
    mtype: String,
    fuel: bool,  
}

// Select variant of transportation.
enum Transport {
    Car(Vpar),
    Bike(Vpar),
    Other(Vpar),
}

fn select_vehicle(sel: String, m: String, f: bool) -> Transport {
    let p = Vpar { name: sel.clone(), mtype: m.clone(), fuel: f };
    if sel == String::from("Car") {
        return Transport::Car(p);
    } else if sel == String::from("Bike") {
        return Transport::Bike(p);
    } else {
        return Transport::Other(p);
    }
}

fn main() {
    // Input a vehicle type, mobility type, and whether fuel is needed.
    let veh_type: String = String::from("Bike");
    let mob_type: String = String::from("Ride");
    let fuel_type: bool = false;
    let veh = select_vehicle(veh_type, mob_type, fuel_type);
    //const mtype_car: String = String::from("Drive");
    //const fuel_car: bool = true;
    //const mtype_bike: String = String::from("Ride");
    //const fuel_bike: bool = false;
    //let my_veh = Transport::Car(VPar { mtype: mtype_car, fuel: fuel_car });
    match veh {
        Transport::Car(Vpar { name: i, mtype: j, fuel: k }) => {
            println!("This is your {}... Mobility: {} | Fuel: {}", i, j, k);
            if j == String::from("Drive") && k == true {
                println!("Looks like you are driving a traditional car!");
            }
        }
        Transport::Bike(Vpar { name: i, mtype: j, fuel: k }) => {
            println!("This is your {}... Mobility: {} | Fuel: {}", i, j, k);
            if j == String::from("Ride") && k == false {
                println!("Looks like you are riding a traditional bike!");
            }
        }
        Transport::Other(i) => {
            println!("This is your {}... Mobility: {} | Fuel: {}", i.name, i.mtype, i.fuel)
        }
    }
}
