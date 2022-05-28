#![allow(dead_code)]

fn main_no_loop() {
    // ANCHOR: no_loop
    let n1 = 0;
    let n2 = 1;
    println!("{} {}", n1, n2);

    let n3 = n1 + n2;

    println!("{}", n3);

    let n1 = n2;
    let n2 = n3;

    let n3 = n1 + n2;

    println!("{}", n3);
    // ANCHOR_END: no_loop
}

fn main_loop1() {
    // ANCHOR: loop1
    let mut n1 = 0;
    let mut n2 = 1;
    println!("{} {}", n1, n2);
    for _ in 0..10 {
        let n3 = n1 + n2;

        println!("{}", n3);

        n1 = n2;
        n2 = n3;
    }
    // ANCHOR_END: loop1
}

fn main_loop2() {
    // ANCHOR: loop2
    let mut n1 = 0;
    let mut n2 = 1;
    print!("{},{},", n1, n2);
    for _ in 0..10 {
        for _ in 0..10 {
            let n3 = n1 + n2;

            print!("{},", n3);

            n1 = n2;
            n2 = n3;
        }
        println!()
    }
    // ANCHOR_END: loop2
}

fn main() {
    // ANCHOR: loop2_float
    let mut n1 = 0.0;
    let mut n2 = 1.0;
    print!("{},{},", n1, n2);
    for _ in 0..10 {
        for _ in 0..10 {
            let n3 = n1 + n2;

            print!("{},", n3);

            n1 = n2;
            n2 = n3;
        }
        println!()
    }
    // ANCHOR_END: loop2_float
}
