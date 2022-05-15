// ANCHOR: all
fn main() {
    // ANCHOR: header
    println!();
    println!("#############################################################");
    println!("#                                                           #");
    println!("#                This is a PRINT program                    #");
    println!("#                                                           #");
    println!("#############################################################");
    println!();
    println!("Summary: This program demonstrates different");
    println!("         ways of printing text");
    println!();
    // ANCHOR_END: header
    // ANCHOR: sum
    println!("The sum of 2 + 3 is {}. Isn't that great?", 2 + 3);
    // ANCHOR_END: sum
    // ANCHOR: mult
    println!("And {} * {} is {}. Fantastic.", 4, 3, 12);
    // ANCHOR_END: mult
    // ANCHOR: newline
    println!();
    // ANCHOR_END: newline
    // ANCHOR: rounding
    println!("5.0 / 3.0 = {}", 5.0 / 3.0);
    println!("5.0 / 3.0 = {:.3}", 5.0 / 3.0);
    println!("5.0 / 3.0 = {:.2}", 5.0 / 3.0);
    println!("5.0 / 3.0 = {:.0}", 5.0 / 3.0);
    // ANCHOR_END: rounding
    println!();
    println!("10 / 2 = {:04}", 5);
    println!("5.0 / 3.0 = {:07.2}", 5.0 / 3.0);
    println!();
}
// ANCHOR_END: all
