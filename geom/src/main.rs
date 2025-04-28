use geom_primitive::Vector2d;


fn main() {
    let v1 = Vector2d::from_coordinates(1.0, 2.0);
    println!("{}", (2.0 * &v1).to_string());

}
