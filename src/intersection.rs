use crate::plane::*;
use crate::sphere::*;

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub object: &'a Sphere,
    pub distance: f64,
}

pub fn hit<'a>(intersections: &[Intersection<'a>]) -> Option<Intersection<'a>> {
    intersections
        .iter()
        .cloned()
        .filter(|x| x.distance.is_sign_positive())
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::intersection::*;
    use crate::material::*;

    #[test]
    fn test_smallest_intersection() {
        let i1 = Intersection {
            object: &Sphere::new(Material::default()),
            distance: 1.0,
        };
        let i2 = Intersection {
            object: &Sphere::new(Material::default()),
            distance: 2.0,
        };
        assert_eq!(hit(&[i1, i2]).unwrap(), i1);
    }
    #[test]
    fn test_smallest_intersection_2() {
        let i1 = Intersection {
            object: &Sphere::new(Material::default()),
            distance: 5.0,
        };
        let i2 = Intersection {
            object: &Sphere::new(Material::default()),
            distance: 7.0,
        };
        let i3 = Intersection {
            object: &Sphere::new(Material::default()),
            distance: -3.0,
        };
        let i4 = Intersection {
            object: &Sphere::new(Material::default()),
            distance: 2.0,
        };
        assert_eq!(hit(&[i1, i2, i3, i4]).unwrap(), i4);
    }
}
