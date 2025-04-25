use geom_primitive::Point2d;

fn main() {
    let a = "hello".to_string();
    let mut b = &a;  
    let c = &a;        

    println!("a: {}, b: {}, c:{}", a, b, c);
}
