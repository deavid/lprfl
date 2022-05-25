#![allow(dead_code)]

#[derive(Debug)]
enum Numbers {
    // Zero,           // 0
    One = 1,        // 1
    Two,            // 2
    Three,          // 3
    Four,           // 4
    FourtyTwo = 42, // -> 42
    FourtyThree,
}

#[derive(Debug)]
enum CppMode {
    Cruise = 1,
    FiringGuns,
    Collecting,
}

#[derive(Debug)]
struct Ship {
    // position: (u8, u8, u8, u8),
    // position2: [u8; 4],
    lifes: u32,
    current_mode: CppMode,
    current_mode2: i64,
}

struct FourNum {
    n1: u8,
    n2: u8,
    n3: u8,
    n4: u8,
}

enum RustMode {
    Cruise(f32),
    FiringGuns,
    Collecting(String),
}

const CRUISE: i64 = 1;
const FIRING: i64 = 2;
const COLLECTING: i64 = 3;

const TCRUISE: (&str, i64) = ("Cruise", 1);
const TFIRING: (&str, i64) = ("Firing", 2);
const TCOLLECTING: (&str, i64) = ("Collecting", 3);

fn main() {
    // Mode::Cruise = 100; <--- this is impossible!
    let mut mode2 = CppMode::Cruise;
    mode2 = CppMode::FiringGuns;
    mode2 = CppMode::Collecting;

    let mut ship_mode = TCRUISE;
    ship_mode = TFIRING;
    ship_mode = TCOLLECTING;

    let mut ship = Ship {
        lifes: 10,
        current_mode: CppMode::Cruise,
        current_mode2: CRUISE,
    };
    ship.current_mode = CppMode::Collecting;
    ship.current_mode = CppMode::FiringGuns;

    if ship.current_mode2 == CRUISE {}
    if ship.current_mode2 == FIRING {
        // ...
    }
    if ship.current_mode2 == COLLECTING {
        // ...
    }

    dbg!(ship);

    // println!("One: {:?}", Numbers::One as u32);
    // println!("Two: {:?}", Numbers::Two as u32);
    // println!("FourtyTwo: {:?}", Numbers::FourtyTwo as u32);
    // println!("FourtyThree: {:?}", Numbers::FourtyThree as u32);
}

union MyType {
    i: i64,       // -> number 1
    f: f32,       // -> number 2
    t: [char; 8], // -> number 3
}

struct MyTypeSafe {
    field: u8,     // this is the field we use in "value"
    value: MyType, // here we write the field promised in "field"
}

impl MyTypeSafe {
    pub fn write_i(&mut self, i: i64) {
        self.field = FIELD_I;
        self.value.i = i;
    }
    pub fn write_f(&mut self, f: f32) {
        self.field = FIELD_F;
        self.value.f = f;
    }
    pub fn write_t(&mut self, t: [char; 8]) {
        self.field = FIELD_T;
        self.value.t = t;
    }
}
const FIELD_I: u8 = 1; // these are just labels - the number is unimportant.
const FIELD_F: u8 = 2;
const FIELD_T: u8 = 3;

fn test() {
    let s = MyTypeSafe {
        field: FIELD_F,            // we want to write here a float.
        value: MyType { f: 10.0 }, // <- so we write here the 'F' field as promised.
    };

    // (... some stuff happens in between, and we don't know what "s" contains anymore ...)

    // if we try to read s.value.f it might not work because s.value might not be a float anymore...
    // so how do we know how to read s.value now?

    // easy! check the field! we promised that every time we would write into s.value, we would
    // also update s.field to match the actual field used in s.value

    if s.field == FIELD_F {
        // dbg!(s.value.f);
    } else {
        panic!("Unexpected field found!")
    }
}

enum AsteroidType {
    Aluminum,
    Plastic,
    Cardboard,
}

struct Speed {
    x: f32,
    y: f32,
}

impl Speed {
    fn velocity(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// enum ShipMode {
//     Cruise(Speed),
//     Firing { rate: u32, speed: Speed },
//     Collecting(AsteroidType),
// }

struct Ship2 {
    // ... blabla
    mode: ShipMode,
}

/*
    Select your ShipMode!

    ( ) Cruise:
            Speed:  x [___] - y [___]
    ( ) Firing
            - rate [___]
            - speed:
                Speed:  x [___] - y [___]

    (X) Collecting:
        ( ) Aluminum
        ( ) Plastic
        ( ) Cardboard

*/

fn test2() {
    let cruise = ShipMode::Cruise(Speed { x: 1.0, y: 0.0 });
    let speed = Speed { x: 1.0, y: 0.0 };
    let cruise = ShipMode::Cruise(speed);
    let speed = Speed { x: 1.0, y: 0.0 };
    let firing = ShipMode::Firing { rate: 31, speed };
    let firing = ShipMode::Firing {
        rate: 31,
        speed: Speed { x: 1.0, y: 0.0 },
    };
    let collec = ShipMode::Collecting(AsteroidType::Aluminum);
}
enum ShipMode {
    Cruise(Speed),
    Firing { rate: u32, speed: Speed },
    Collecting(AsteroidType),
}
