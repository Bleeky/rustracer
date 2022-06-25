#[cfg(test)]
mod tests {
    use crate::material::Material;
    use crate::objects::sphere::*;
    use crate::patterns::stripe::*;
    use crate::patterns::*;

    #[test]
    fn create_stripe_pattern() {
        let s = Stripe::new(Color::white(), Color::black());
        assert_eq!(s.color_a, Color::white());
        assert_eq!(s.color_b, Color::black());
    }
    #[test]
    fn create_stripe_pattern_constant_y() {
        let s = Stripe::new(Color::white(), Color::black());
        assert_eq!(s.pattern_at(&Point::zero()), Color::white());
        assert_eq!(
            s.pattern_at(&Point {
                x: 0.0,
                y: 1.0,
                z: 0.0
            }),
            Color::white()
        );
        assert_eq!(
            s.pattern_at(&Point {
                x: 0.0,
                y: 2.0,
                z: 0.0
            }),
            Color::white()
        );
    }
    #[test]
    fn create_stripe_pattern_constant_z() {
        let s = Stripe::new(Color::white(), Color::black());
        assert_eq!(s.pattern_at(&Point::zero()), Color::white());
        assert_eq!(
            s.pattern_at(&Point {
                x: 0.0,
                y: 0.0,
                z: 1.0
            }),
            Color::white()
        );
        assert_eq!(
            s.pattern_at(&Point {
                x: 0.0,
                y: 0.0,
                z: 2.0
            }),
            Color::white()
        );
    }
    #[test]
    fn create_stripe_pattern_alters_x() {
        let s = Stripe::new(Color::white(), Color::black());
        assert_eq!(s.pattern_at(&Point::zero()), Color::white());
        assert_eq!(
            s.pattern_at(&Point {
                x: 0.9,
                y: 0.0,
                z: 0.0
            }),
            Color::white()
        );
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
                x: -0.1,
                y: 0.0,
                z: 0.0
            }),
            Color::black()
        );
        assert_eq!(
            s.pattern_at(&Point {
                x: -1.0,
                y: 0.0,
                z: 0.0
            }),
            Color::black()
        );
        assert_eq!(
            s.pattern_at(&Point {
                x: -1.1,
                y: 0.0,
                z: 0.0
            }),
            Color::white()
        );
    }

    #[test]
    fn test_stripe_with_object_transformation() {
        let mut s = Object::Sphere(Sphere::new(Material::default()));
        s.set_transform(Matrix44::scaling(2.0, 2.0, 2.0));
        let p = Pattern::Stripe(Stripe::new(Color::white(), Color::black()));
        let c = p.pattern_at_object(
            &Point {
                x: 1.5,
                y: 0.0,
                z: 0.0,
            },
            &s,
        );
        assert_eq!(c, Color::white());
    }
    #[test]
    fn test_stripe_with_pattern_transformation() {
        let s = Object::Sphere(Sphere::new(Material::default()));
        let mut p = Pattern::Stripe(Stripe::new(Color::white(), Color::black()));
        p.set_transform(Matrix44::scaling(2.0, 2.0, 2.0));
        let c = p.pattern_at_object(
            &Point {
                x: 1.5,
                y: 0.0,
                z: 0.0,
            },
            &s,
        );
        assert_eq!(c, Color::white());
    }
    #[test]
    fn test_stripe_with_pattern_and_object_transformation() {
        let mut s = Object::Sphere(Sphere::new(Material::default()));
        s.set_transform(Matrix44::scaling(2.0, 2.0, 2.0));
        let mut p = Pattern::Stripe(Stripe::new(Color::white(), Color::black()));
        p.set_transform(Matrix44::translation(0.5, 0.0, 0.0));
        let c = p.pattern_at_object(
            &Point {
                x: 2.5,
                y: 0.0,
                z: 0.0,
            },
            &s,
        );
        assert_eq!(c, Color::white());
    }
}
