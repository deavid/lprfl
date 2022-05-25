use std::collections::HashMap;

struct PhoneNumber {
    pub name: String,
    pub phone: i64,
}

#[allow(dead_code, clippy::vec_init_then_push)]
fn test() {
    let mut telephone_numbers = vec![];
    telephone_numbers.push(PhoneNumber {
        name: "me1".to_string(),
        phone: 12341,
    });
    telephone_numbers.push(PhoneNumber {
        name: "me2".to_string(),
        phone: 12342,
    });
    telephone_numbers.push(PhoneNumber {
        name: "me3".to_string(),
        phone: 12343,
    });
    telephone_numbers.push(PhoneNumber {
        name: "me4".to_string(),
        phone: 12344,
    });

    // what is the phone number of "me4"?
    let look_for = "me4".to_string(); // <-- the one that we want to find!

    for pn in telephone_numbers.iter() {
        // .iter -> iterate through object/list
        if pn.name == look_for {
            dbg!(pn.phone);
        }
    }

    let mut n = 0;
    let tn_size = telephone_numbers.len(); // Len returns the count of items in the list.
                                           // tn_size = 4 ... for this particular code.
    loop {
        if n >= tn_size {
            break;
        }
        let pn = &telephone_numbers[n];

        if pn.name == look_for {
            dbg!(pn.phone);
        }

        n += 1;
    }
}

fn hmaps() {
    //                  Key -> Value
    let mut db: HashMap<String, (u8, u8, u8, u8)> = HashMap::new();
    // Imagine... that we had 10 billion domains here.
    db.insert("example.com".to_string(), (127, 0, 0, 1));
    db.insert("wiki.com".to_string(), (33, 33, 33, 33));
    db.insert("reddit.com".to_string(), (12, 1, 1, 12));

    // let kevin: Option<&i64> = db.get("kevin");
    // dbg!(kevin);
    // // Some(&335122333)

    // when we access, we no longer loop though 10 billion to get the value.
    // there is a "loop" inside to do this, but it doesn't do 10 billion iterations.
    // it does approx log(n) iterations, or 34 iterations.
    let excom: (u8, u8, u8, u8) = db["example.com"];
    dbg!(excom);
    // -> (127, 0, 0, 1)
}

fn main() {
    hmaps();
}
