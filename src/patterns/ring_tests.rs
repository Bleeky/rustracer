#[cfg(test)]
mod tests {
    use crate::patterns::ring::*;

    #[test]
    fn create_stripe_pattern() {
        let s = Ring::new(
            Pattern::SolidColor(SolidColor::new(Color::white())),
            Pattern::SolidColor(SolidColor::new(Color::black())),
        );
        assert_eq!(s.pattern_at(&Point::zero()), Color::white());
        assert_eq!(
            s.pattern_at(&Point {
                x: 1.0,
                y: 0.0,
                z: 0.0
            }),
            Color::black()
        );
        assert_eq!(
            s.pattern_at(&Point {
                x: 0.0,
                y: 0.0,
                z: 1.0
            }),
            Color::black()
        );
        assert_eq!(
            s.pattern_at(&Point {
                x: 0.708,
                y: 0.0,
                z: 0.708
            }),
            Color::black()
        );
    }
}
