use geom_primitive::XY;

fn main() {
    let mut a = 0.0;
    for i in 0..1000000000 {
        let p = XY::from_coordinates(i as f64, i as f64 + 1.0);
        a += p.x + p.y;
    }
    println!("a: {}", a);
}
