use renderer::data_types::F32_2;

#[test]
fn test_init() {
    let a = F32_2::new(1.0, 2.0);
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
}

#[test]
fn test_index() {
    let a = F32_2::new(1.0, 2.0);
    assert_eq!(a[0], 1.0);
    assert_eq!(a[1], 2.0);

    let mut b = a;
    b[0] = 3.0;
    b[1] = 4.0;
    assert_eq!(b[0], 3.0);
    assert_eq!(b[1], 4.0);
}

#[test]
fn test_to_string() {
    let a = F32_2::new(1.0, 2.5);
    assert_eq!(a.to_string(), "f32(1, 2.5)");
}
