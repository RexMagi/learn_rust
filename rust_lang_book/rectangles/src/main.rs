fn main() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}
