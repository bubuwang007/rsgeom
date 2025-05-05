use renderer::data_types::{F32_2, F32_3};

#[test]
fn test_init() {
    let a = F32_3::new(1.0, 2.0, 3.0);
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 3.0);

    let b = F32_3::from_value(4.0);
    assert_eq!(b.x, 4.0);
    assert_eq!(b.y, 4.0);
    assert_eq!(b.z, 4.0);

    let f2 = F32_2::new(5.0, 6.0);
    let c = F32_3::from_data2(&f2, 7.0);
    assert_eq!(c.x, 5.0);
    assert_eq!(c.y, 6.0);
    assert_eq!(c.z, 7.0);
}

#[test]
fn test_index() {
    let a = F32_3::new(1.0, 2.0, 3.0);
    assert_eq!(a[0], 1.0);
    assert_eq!(a[1], 2.0);
    assert_eq!(a[2], 3.0);

    let mut b = a;
    b[0] = 4.0;
    b[1] = 5.0;
    b[2] = 6.0;
    assert_eq!(b[0], 4.0);
    assert_eq!(b[1], 5.0);
    assert_eq!(b[2], 6.0);
}

#[test]
fn test_to_string() {
    let a = F32_3::new(1.0, 2.5, 3.5);
    assert_eq!(a.to_string(), "f32(1, 2.5, 3.5)");
}
