enum Shape {
    Rectangle(f64,f64),
    Circle(f64),
}
fn main() {
    
   
    let rect = Shape::Rectangle(19.0, 22.0);
    calculate_area(rect);

    let circle = Shape::Circle(35.0);
    println!("{}", calculate_area(circle));

}


fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(a,b ) => a * b 
    };
    return area;
}