#[cfg(test)]
mod tests {
    use crate::intersection::*;
    use crate::material::*;
    use crate::matrix::*;
    use crate::objects::sphere::*;

    #[test]
    fn test_smallest_intersection() {
        let i1 = Intersection {
            object: Object::Sphere(Sphere::new(&&Material::default())),
            distance: 1.0,
        };
        let i2 = Intersection {
            object: Object::Sphere(Sphere::new(&&Material::default())),
            distance: 2.0,
        };
        assert_eq!(hit(vec![i1.clone(), i2]).unwrap(), i1);
    }

    #[test]
    fn test_smallest_intersection_2() {
        let i1 = Intersection {
            object: Object::Sphere(Sphere::new(&&Material::default())),
            distance: -1.0,
        };
        let i2 = Intersection {
            object: Object::Sphere(Sphere::new(&&Material::default())),
            distance: 2.0,
        };
        assert_eq!(hit(vec![i1, i2.clone()]).unwrap(), i2);
    }

    #[test]
    fn test_smallest_intersection_3() {
        let i1 = Intersection {
            object: Object::Sphere(Sphere::new(&Material::default())),
            distance: -2.0,
        };
        let i2 = Intersection {
            object: Object::Sphere(Sphere::new(&Material::default())),
            distance: -1.0,
        };
        assert!(hit(vec![i1, i2]) == None);
    }

    #[test]
    fn test_smallest_intersection_4() {
        let i1 = Intersection {
            object: Object::Sphere(Sphere::new(&Material::default())),
            distance: 5.0,
        };
        let i2 = Intersection {
            object: Object::Sphere(Sphere::new(&Material::default())),
            distance: 7.0,
        };
        let i3 = Intersection {
            object: Object::Sphere(Sphere::new(&Material::default())),
            distance: -3.0,
        };
        let i4 = Intersection {
            object: Object::Sphere(Sphere::new(&Material::default())),
            distance: 2.0,
        };
        assert_eq!(hit(vec![i1, i2, i3, i4.clone()]).unwrap(), i4);
    }

    #[test]
    fn test_sphere_intersection_1() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Object::Sphere(Sphere::new(&Material::default()));
        let i = sphere.intersect(&ray);
        assert!(Option::is_some(&i));
        let u = i.unwrap();
        assert_eq!(u[0].distance, 4.0);
        assert_eq!(u[1].distance, 6.0);
    }

    #[test]
    fn test_sphere_intersection_2() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 2.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Object::Sphere(Sphere::new(&Material::default()));
        let i = sphere.intersect(&ray);
        assert_eq!(i, None);
    }

    #[test]
    fn test_sphere_intersection_3() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 1.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Object::Sphere(Sphere::new(&Material::default()));
        let i = sphere.intersect(&ray);
        assert!(Option::is_some(&i));
        let u = i.unwrap();
        assert_eq!(u[0].distance, 5.0);
        assert_eq!(u[1].distance, 5.0);
    }

    #[test]
    fn test_sphere_intersection_4() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Object::Sphere(Sphere::new(&Material::default()));
        let i = sphere.intersect(&ray);
        assert!(Option::is_some(&i));
        let u = i.unwrap();
        assert_eq!(u[0].distance, -1.0);
        assert_eq!(u[1].distance, 1.0);
    }

    #[test]
    fn test_world_intersection() {
        let world = World::default();
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let intersections = intersect_world(&ray, &world);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].distance, 4.0);
        assert_eq!(intersections[1].distance, 4.5);
        assert_eq!(intersections[2].distance, 5.5);
        assert_eq!(intersections[3].distance, 6.0);
    }

    #[test]
    fn test_precomputing_intersection() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Object::Sphere(Sphere::new(&Material::default()));
        let i = Intersection {
            object: sphere,
            distance: 4.0,
        };
        let precomputed = prepare_computations(&i, &ray);
        assert_eq!(precomputed.distance, i.distance);
        assert_eq!(precomputed.object, i.object);
        assert_eq!(
            precomputed.point,
            Point {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        );
        assert_eq!(
            precomputed.eyev,
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        );
        assert_eq!(
            precomputed.normalv,
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        );
    }

    #[test]
    fn test_precomputing_intersection_outside() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Object::Sphere(Sphere::new(&Material::default()));
        let i = Intersection {
            object: sphere,
            distance: 4.0,
        };
        let precomputed = prepare_computations(&i, &ray);
        assert_eq!(precomputed.inside, false);
    }

    #[test]
    fn test_precomputing_intersection_inside() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Object::Sphere(Sphere::new(&Material::default()));
        let i = Intersection {
            object: sphere,
            distance: 1.0,
        };
        let precomputed = prepare_computations(&i, &ray);
        assert_eq!(
            precomputed.point,
            Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }
        );
        assert_eq!(
            precomputed.eyev,
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        );
        assert_eq!(
            precomputed.normalv,
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        );
        assert_eq!(precomputed.inside, true);
    }

    #[test]
    fn test_intersection_shading() {
        let world = World::default();
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let shape = &world.objects[0];
        let i = Intersection {
            object: shape.clone(),
            distance: 4.0,
        };
        let comps = prepare_computations(&i, &ray);
        let c = shade_hit(&world, &comps, &i.object);
        assert_eq!(
            c,
            Color {
                red: 0.3806612,
                green: 0.47582647,
                blue: 0.2854959,
            }
        );
    }

    #[test]
    fn test_intersection_shading_from_inside() {
        let mut world = World::default();
        world.lights = vec![Light::PointLight(PointLight {
            position: Point {
                x: 0.0,
                y: 0.25,
                z: 0.0,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
        })];
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let shape = &world.objects[1];
        let i = Intersection {
            object: shape.clone(),
            distance: 0.5,
        };
        let comps = prepare_computations(&i, &ray);
        let c = shade_hit(&world, &comps, &i.object);
        assert_eq!(
            c,
            Color {
                red: 0.9049845,
                green: 0.9049845,
                blue: 0.9049845,
            }
        );
    }

    #[test]
    fn test_shade_hit_in_shadow() {
        let lights = vec![Light::PointLight(PointLight {
            position: Point {
                x: 0.0,
                y: 0.0,
                z: -10.0,
            },
            color: Color::default(),
        })];
        let sphere1 = Object::Sphere(Sphere::new(&Material::default()));
        let mut sphere2 = Object::Sphere(Sphere::new(&Material::default()));
        sphere2.set_transform(Matrix44::translation(0.0, 0.0, 10.0));
        let world = World {
            objects: vec![sphere1, sphere2],
            lights,
        };
        let r = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let i = Intersection {
            object: Object::Sphere(Sphere::new(&Material::default())),
            distance: 4.0,
        };
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&world, &comps, &i.object);
        assert_eq!(
            c,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        )
    }

    #[test]
    fn test_hit_point_offset() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let mut s = Object::Sphere(Sphere::new(&Material::default()));
        s.set_transform(Matrix44::translation(0.0, 0.0, 1.0));

        let i = Intersection {
            object: s,
            distance: 5.0,
        };
        let comps = prepare_computations(&i, &ray);
        assert!(comps.over_point.z < -std::f64::EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn test_intersection_hit_color() {
        let world = World::default();
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let color = color_at(&world, &ray);
        assert_eq!(
            color,
            Color {
                red: 0.3806612,
                green: 0.47582647,
                blue: 0.2854959,
            }
        );
    }

    #[test]
    fn test_intersection_hit_color_from_outside() {
        let mut world = World::default();
        world.lights = vec![Light::PointLight(PointLight {
            position: Point {
                x: 0.0,
                y: 0.25,
                z: 0.0,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
        })];
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let shape = &world.objects[1];
        let intersection = Intersection {
            object: shape.clone(),
            distance: 0.5,
        };
        let comps = prepare_computations(&intersection, &ray);
        let color = shade_hit(&world, &comps, &intersection.object);
        assert_eq!(
            color,
            Color {
                red: 0.9049845,
                green: 0.9049845,
                blue: 0.9049845,
            }
        );
    }

    #[test]
    fn test_color_ray_miss() {
        let world = World::default();
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let color = color_at(&world, &ray);
        assert_eq!(
            color,
            Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0
            }
        );
    }

    #[test]
    fn test_color_ray_hit() {
        let world = World::default();
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let color = color_at(&world, &ray);
        assert_eq!(
            color,
            Color {
                red: 0.3806612,
                green: 0.47582647,
                blue: 0.2854959,
            }
        );
    }

    #[test]
    fn test_intersection_hit_behind_ray() {
        let mut world = World::default();
        world.objects[0].set_material(Material {
            ambient: 1.0,
            ..Default::default()
        });
        world.objects[1].set_material(Material {
            ambient: 1.0,
            ..Default::default()
        });
        let inner = &world.objects[1];
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.75,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        };
        let color = color_at(&world, &ray);
        assert_eq!(color, inner.material().color);
    }
}
