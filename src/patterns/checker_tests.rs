#[cfg(test)]
mod tests {
    use crate::patterns::checker::*;

    #[test]
    fn checker_repeat_in_x() {
        let checker = Checker::new(Color::white(), Color::black());
        assert_eq!(checker.pattern_at(&Point::zero()), Color::white());
        assert_eq!(
            checker.pattern_at(&Point {
                x: 0.99,
                y: 0.0,
                z: 0.0,
            }),
            Color::white()
        );
        assert_eq!(
            checker.pattern_at(&Point {
                x: 1.01,
                y: 0.0,
                z: 0.0,
            }),
            Color::black()
        );
    }
    #[test]
    fn checker_repeat_in_y() {
        let checker = Checker::new(Color::white(), Color::black());
        assert_eq!(checker.pattern_at(&Point::zero()), Color::white());
        assert_eq!(
            checker.pattern_at(&Point {
                x: 0.0,
                y: 0.99,
                z: 0.0,
            }),
            Color::white()
        );
        assert_eq!(
            checker.pattern_at(&Point {
                x: 0.0,
                y: 1.01,
                z: 0.0,
            }),
            Color::black()
        );
    }
    #[test]
    fn checker_repeat_in_z() {
        let checker = Checker::new(Color::white(), Color::black());
        assert_eq!(checker.pattern_at(&Point::zero()), Color::white());
        assert_eq!(
            checker.pattern_at(&Point {
                x: 0.0,
                y: 0.0,
                z: 0.99,
            }),
            Color::white()
        );
        assert_eq!(
            checker.pattern_at(&Point {
                x: 0.0,
                y: 0.0,
                z: 1.01,
            }),
            Color::black()
        );
    }
}
