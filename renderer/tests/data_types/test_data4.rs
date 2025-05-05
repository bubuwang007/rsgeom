use renderer::data_types::F32_4;

#[test]
fn test_init() {
    let a = F32_4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 3.0);
    assert_eq!(a.w, 4.0);
}


#[test]
fn test_index() {
    let a = F32_4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(a[0], 1.0);
    assert_eq!(a[1], 2.0);
    assert_eq!(a[2], 3.0);
    assert_eq!(a[3], 4.0);

    let mut b = a;
    b[0] = 5.0;
    b[1] = 6.0;
    b[2] = 7.0;
    b[3] = 8.0;
    assert_eq!(b[0], 5.0);
    assert_eq!(b[1], 6.0);
    assert_eq!(b[2], 7.0);
    assert_eq!(b[3], 8.0);
}

#[test]
fn test_to_string() {
    let a = F32_4::new(1.0, 2.5, 3.0, 4.0);
    assert_eq!(a.to_string(), "f32(1, 2.5, 3, 4)");
}
