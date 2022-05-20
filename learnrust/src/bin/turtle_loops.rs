use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    let sides = 5.0;
    let distance = 200.0;
    let angle = 360.0 / sides;
    for _ in 0..30 {
        turtle.forward(distance);
        turtle.right(angle);
    }
}
