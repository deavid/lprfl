use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    let sides = 5.0;
    let distance = 200.0;
    let angle = 360.0 / sides;

    turtle.forward(distance);
    turtle.right(angle);
    turtle.forward(distance);
    turtle.right(angle);
    turtle.forward(distance);
    turtle.right(angle);
    turtle.forward(distance);
    turtle.right(angle);
    turtle.forward(distance);
    turtle.right(angle);
    turtle.forward(distance);
    turtle.right(angle);
    turtle.forward(distance);
    turtle.right(angle);
    turtle.forward(distance);
    turtle.right(angle);
    turtle.forward(distance);
}
