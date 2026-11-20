pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let i = (h * 6.0).floor() as i32;
    let f = h * 6.0 - i as f32;

    let p = (v * (1.0 - s) * 255.0) as u8;
    let q = (v * (1.0 - s * f) * 255.0) as u8;
    let t = (v * (1.0 - s * (1.0 - f)) * 255.0) as u8;
    let v_u8 = (v * 255.0) as u8;

    match i % 6 {
        0 => (v_u8, t, p),
        1 => (q, v_u8, p),
        2 => (p, v_u8, t),
        3 => (p, q, v_u8),
        4 => (t, p, v_u8),
        _ => (v_u8, p, q),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv_red() {
        // h=0, s=1, v=1 → pure red
        assert_eq!(hsv_to_rgb(0.0, 1.0, 1.0), (255, 0, 0));
    }

    #[test]
    fn test_hsv_green() {
        // h=1/3, s=1, v=1 → pure green
        assert_eq!(hsv_to_rgb(1.0 / 3.0, 1.0, 1.0), (0, 255, 0));
    }

    #[test]
    fn test_hsv_blue() {
        // h=2/3, s=1, v=1 → pure blue
        assert_eq!(hsv_to_rgb(2.0 / 3.0, 1.0, 1.0), (0, 0, 255));
    }

    #[test]
    fn test_hsv_white_when_saturation_zero() {
        // s=0 → grayscale, v=1 → white
        assert_eq!(hsv_to_rgb(0.5, 0.0, 1.0), (255, 255, 255));
        assert_eq!(hsv_to_rgb(0.0, 0.0, 1.0), (255, 255, 255));
    }

    #[test]
    fn test_hsv_black_when_value_zero() {
        // v=0 → black regardless of hue/saturation
        assert_eq!(hsv_to_rgb(0.0, 1.0, 0.0), (0, 0, 0));
        assert_eq!(hsv_to_rgb(0.5, 1.0, 0.0), (0, 0, 0));
    }

    #[test]
    fn test_hsv_wraps_at_one() {
        // h=1.0 should wrap to same as h=0.0
        assert_eq!(hsv_to_rgb(1.0, 1.0, 1.0), hsv_to_rgb(0.0, 1.0, 1.0));
    }

    #[test]
    fn test_hsv_half_brightness() {
        // v=0.5 → colors at half intensity
        assert_eq!(hsv_to_rgb(0.0, 1.0, 0.5), (127, 0, 0));
    }
}
