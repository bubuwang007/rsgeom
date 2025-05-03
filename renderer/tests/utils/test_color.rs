use renderer::utils::color::*;


#[test]
fn test_float_to_u8() {
    assert_eq!(float_to_u8(0.0), 0);
    assert_eq!(float_to_u8(0.5), 127);
    assert_eq!(float_to_u8(1.0), 255);
    assert_eq!(float_to_u8(-0.5), 0);
    assert_eq!(float_to_u8(1.5), 255);
}

#[test]
fn test_u8_to_float() {
    assert_eq!(u8_to_float(0), 0.0);
    assert_eq!(u8_to_float(127), 0.49803922);
    assert_eq!(u8_to_float(255), 1.0);
    assert_eq!(u8_to_float(128), 0.5019608);
    assert_eq!(u8_to_float(255), 1.0);
}

