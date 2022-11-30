#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]

enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles);
    }
    return (Age::New, miles);
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    let roof_type: &str = if roof { "Hard top" } else { "Covertible" };
    let new_or_used: &str = if miles == 0 { "new" } else { "used" };

    println!(
        "Prepare a {} car: {:?}, {}, {}, {} miles\n",
        new_or_used, motor, color, roof_type, miles
    );
    Car {
        color,
        motor,
        roof,
        age: car_quality(miles),
    }
}

fn main() {
    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
