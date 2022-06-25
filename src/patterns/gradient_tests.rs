#[cfg(test)]
mod tests {
    use crate::patterns::gradient::*;

    #[test]
    fn gradient_pattern_linear_interpolation() {
        let g = Gradient::new(Color::white(), Color::black());
        assert_eq!(g.pattern_at(&Point::zero()), Color::white());
        assert_eq!(
            g.pattern_at(&Point {
                x: 0.25,
                y: 0.0,
                z: 0.0,
            }),
            Color {
                red: 0.75,
                green: 0.75,
                blue: 0.75,
            }
        );
        assert_eq!(
            g.pattern_at(&Point {
                x: 0.5,
                y: 0.0,
                z: 0.0,
            }),
            Color {
                red: 0.5,
                green: 0.5,
                blue: 0.5,
            }
        );
        assert_eq!(
            g.pattern_at(&Point {
                x: 0.75,
                y: 0.0,
                z: 0.0,
            }),
            Color {
                red: 0.25,
                green: 0.25,
                blue: 0.25,
            }
        );
    }
}
