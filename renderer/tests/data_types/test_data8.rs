use renderer::data_types::F32_8;

#[test]
fn test_init() {
    let a = F32_8::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
    assert_eq!(a.a, 1.0);
    assert_eq!(a.b, 2.0);
    assert_eq!(a.c, 3.0);
    assert_eq!(a.d, 4.0);
    assert_eq!(a.e, 5.0);
    assert_eq!(a.f, 6.0);
    assert_eq!(a.g, 7.0);
    assert_eq!(a.h, 8.0);
}

#[test]
fn test_index() {
    let a = F32_8::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
    assert_eq!(a[0], 1.0);
    assert_eq!(a[1], 2.0);
    assert_eq!(a[2], 3.0);
    assert_eq!(a[3], 4.0);
    assert_eq!(a[4], 5.0);
    assert_eq!(a[5], 6.0);
    assert_eq!(a[6], 7.0);
    assert_eq!(a[7], 8.0);

    let mut b = a;
    b[0] = 9.0;
    b[1] = 10.0;
    b[2] = 11.0;
    b[3] = 12.0;
    b[4] = 13.0;
    b[5] = 14.0;
    b[6] = 15.0;
    b[7] = 16.0;
    assert_eq!(b[0], 9.0);
    assert_eq!(b[1], 10.0);
    assert_eq!(b[2], 11.0);
    assert_eq!(b[3], 12.0);
    assert_eq!(b[4], 13.0);
    assert_eq!(b[5], 14.0);
    assert_eq!(b[6], 15.0);
    assert_eq!(b[7], 16.0);
}

#[test]
fn test_to_string() {
    let a = F32_8::new(1.0, 2.5, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
    assert_eq!(a.to_string(), "f32(1, 2.5, 3, 4, 5, 6, 7, 8)");
}