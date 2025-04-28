use geom_primitive::Vector2d;

use std::cell::RefCell;

fn main() {
    
    let ref_cell = RefCell::new(String::from("Hello, world!"));
    let r = ref_cell.borrow();
    println!("{}", r);
    let mut w = ref_cell.borrow_mut();

}
