use renderer::data_types::{float2::Float2, float3::Float3};

#[test]
fn test_init() {
    let a = Float3::new(1.0, 2.0, 3.0);
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 3.0);

    let b = Float3::from_value(4.0);
    assert_eq!(b.x, 4.0);
    assert_eq!(b.y, 4.0);
    assert_eq!(b.z, 4.0);

    let f2 = Float2::new(5.0, 6.0);
    let c = Float3::from_float2(&f2, 7.0);
    assert_eq!(c.x, 5.0);
    assert_eq!(c.y, 6.0);
    assert_eq!(c.z, 7.0);
}
