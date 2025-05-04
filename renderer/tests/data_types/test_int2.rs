use renderer::data_types::int2::Int2;

#[test]
fn test_init() {
    let a = Int2::new(1, 2);
    assert_eq!(a.x, 1);
    assert_eq!(a.y, 2);
}

#[test]
fn test_index() {
    let a = Int2::new(1, 2);
    assert_eq!(a[0], 1);
    assert_eq!(a[1], 2);

    let mut b = a;
    b[0] = 3;
    b[1] = 4;
    assert_eq!(b[0], 3);
    assert_eq!(b[1], 4);
}

#[test]
fn test_to_string() {
    let a = Int2::new(1, 2);
    assert_eq!(a.to_string(), "Int2(1, 2)");
}