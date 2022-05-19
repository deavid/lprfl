#[derive(Debug, Clone)]
enum LengthUnit {
    Meter,
    Kilometer,
    Yard,
    Furlong,
}

impl LengthUnit {
    fn meters(&self) -> f64 {
        match self {
            LengthUnit::Meter => 1.0,
            LengthUnit::Kilometer => 1000.0,
            LengthUnit::Yard => 0.9144,
            LengthUnit::Furlong => 201.168,
        }
    }
}

impl std::fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit = match self {
            LengthUnit::Meter => "m",
            LengthUnit::Kilometer => "km",
            LengthUnit::Yard => "yd",
            // https://en.wikipedia.org/wiki/Furlong
            LengthUnit::Furlong => "furlong",
        };
        write!(f, "{}", unit)
    }
}

#[derive(Debug, Clone)]
struct Distance {
    length: f64,
    units: LengthUnit,
}

impl Distance {
    fn into_unit(self, units: LengthUnit) -> Self {
        self.as_unit(units)
    }
    fn as_unit(&self, units: LengthUnit) -> Self {
        let mut length = self.length;
        length *= self.units.meters();
        length /= units.meters();
        Self { length, units }
    }
}

impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}{}", self.length, self.units)
    }
}

enum CarCommand {
    DriveInReverse(Distance), // 1
    DriveForward(Distance),   // 2
    Honk,                     // 3
    TurnLeft,                 // 4
    TurnRight,                // 5
}

// enum Option is ... either Some(i64) or None ... Some || None.
#[derive(Debug)]
enum Option {
    Some(i64),
    None,
}

fn _main() {
    let opt = Option::Some(123);
    dbg!(opt);
    let opt = Option::None;
    dbg!(opt);
}

// struct Option is ... some & none ... i64 & bool
// struct Option {
//     some: i64,
//     none: bool,
// }

// fn test() {
//     let o = Option {
//         some: 12333,
//         none: false,
//     };
//     let o = Option {
//         some: 0,
//         none: true,
//     };
// }

// enum Result {
//     Ok(i64),
//     Error(String),
// }

fn main() {
    let dist32km = Distance {
        length: 32.0,
        units: LengthUnit::Kilometer,
    };
    let command = CarCommand::DriveForward(dist32km);
    process_command(&command);
}

fn process_command(command: &CarCommand) {
    match command {
        CarCommand::DriveInReverse(dist) => println!("mooooorv: {}", dist), // todo! is like panic!
        CarCommand::DriveForward(dist) => println!("vrooooom: {}", dist.as_unit(LengthUnit::Yard)),
        CarCommand::Honk => println!("HONK HONK"),
        CarCommand::TurnLeft => println!("turn left"),
        CarCommand::TurnRight => println!("turn right"),
    }
}
