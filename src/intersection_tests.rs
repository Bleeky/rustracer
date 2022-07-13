#[cfg(test)]
mod tests {
    use std::vec;

    use crate::intersection::*;
    use crate::material::*;
    use crate::matrix::*;
    use crate::objects::plane::Plane;
    use crate::objects::sphere::*;
    use crate::patterns::checker::*;
    use crate::patterns::solid_color::*;
    use crate::patterns::Pattern;
    use crate::world;

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
        let precomputed = prepare_computations(&i, &ray, &vec![i.clone()]);
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
        let precomputed = prepare_computations(&i, &ray, &vec![i.clone()]);
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
        let precomputed = prepare_computations(&i, &ray, &vec![i.clone()]);
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
        let comps = prepare_computations(&i, &ray, &vec![i.clone()]);
        let c = shade_hit(&world, &comps, 0);
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
        let comps = prepare_computations(&i, &ray, &vec![i.clone()]);
        let c = shade_hit(&world, &comps, 0);
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
        let sphere2 = Object::Sphere(Sphere::new(&Material::default()))
            .set_transform(Matrix44::translation(0.0, 0.0, 10.0));
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
        let comps = prepare_computations(&i, &r, &vec![i.clone()]);
        let c = shade_hit(&world, &comps, 0);
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
        let s = Object::Sphere(Sphere::new(&Material::default()))
            .set_transform(Matrix44::translation(0.0, 0.0, 1.0));

        let i = Intersection {
            object: s,
            distance: 5.0,
        };
        let comps = prepare_computations(&i, &ray, &vec![i.clone()]);
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
        let color = color_at(&world, &ray, 0);
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
        let comps = prepare_computations(&intersection, &ray, &vec![intersection.clone()]);
        let color = shade_hit(&world, &comps, 0);
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
        let color = color_at(&world, &ray, 0);
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
        let color = color_at(&world, &ray, 0);
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
        let color = color_at(&world, &ray, 0);
        assert_eq!(color, inner.material().color);
    }

    #[test]
    fn test_precomputing_reflection_vector() {
        let p = Object::Plane(Plane::new(Material::default()));
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 1.0,
                z: -1.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: -2.0_f64.sqrt() / 2.0,
                z: 2.0_f64.sqrt() / 2.0,
            },
        };
        let intersection = Intersection {
            object: p.clone(),
            distance: 2.0_f64.sqrt(),
        };
        let comps = prepare_computations(&intersection, &ray, &vec![intersection.clone()]);
        assert_eq!(
            comps.reflectv,
            Vector3 {
                x: 0.0,
                y: 2.0_f64.sqrt() / 2.0,
                z: 2.0_f64.sqrt() / 2.0
            }
        )
    }

    #[test]
    fn test_reflected_color_on_non_reflective_material() {
        let mut world = World::default();
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
        world.objects[1].set_material(Material {
            ambient: 1.0,
            ..Material::default()
        });
        let intersection = Intersection {
            object: world.objects[1].clone(),
            distance: 1.0,
        };
        let comps = prepare_computations(&intersection, &ray, &vec![intersection.clone()]);
        let color = reflected_color(&world, &comps, 0);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn test_reflected_color_on_reflective_material() {
        let mut world = World::default();
        let plane = Object::Plane(Plane::new(Material {
            reflective: 0.5,
            ..Material::default()
        }))
        .set_transform(Matrix44::translation(0.0, -1.0, 0.0));
        world.objects.push(plane.clone());
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -3.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: -2.0_f64.sqrt() / 2.0,
                z: 2.0_f64.sqrt() / 2.0,
            },
        };
        let intersection = Intersection {
            object: plane.clone(),
            distance: 2.0_f64.sqrt(),
        };
        let comps = prepare_computations(&intersection, &ray, &vec![intersection.clone()]);
        let color = reflected_color(&world, &comps, 5);
        assert_eq!(
            color,
            Color {
                red: 0.1903306,
                green: 0.23791324,
                blue: 0.14274795
            }
        );
    }

    #[test]
    fn test_shade_hit_on_reflective_material() {
        let mut world = World::default();
        let plane = Object::Plane(Plane::new(Material {
            reflective: 0.5,
            ..Material::default()
        }))
        .set_transform(Matrix44::translation(0.0, -1.0, 0.0));
        world.objects.push(plane.clone());
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -3.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: -2.0_f64.sqrt() / 2.0,
                z: 2.0_f64.sqrt() / 2.0,
            },
        };
        let intersection = Intersection {
            object: plane.clone(),
            distance: 2.0_f64.sqrt(),
        };
        let comps = prepare_computations(&intersection, &ray, &vec![intersection.clone()]);
        let color = shade_hit(&world, &comps, 5);
        assert_eq!(
            color,
            Color {
                red: 0.8767561,
                green: 0.9243387,
                blue: 0.8291734
            }
        );
    }

    #[test]
    fn test_color_at_with_mutually_reflective() {
        let light = Light::PointLight(PointLight {
            position: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            color: Color::white(),
        });
        let lower_plane = Object::Plane(Plane::new(Material {
            reflective: 1.0,
            ..Material::default()
        }))
        .set_transform(Matrix44::translation(0.0, -1.0, 0.0));
        let higher_plane = Object::Plane(Plane::new(Material {
            reflective: 1.0,
            ..Material::default()
        }))
        .set_transform(Matrix44::translation(0.0, 1.0, 0.0));
        let world = World::new(vec![lower_plane.clone(), higher_plane.clone()], vec![light]);
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let _color_at = color_at(&world, &ray, 5);
    }

    #[test]
    fn test_recursive_maximum_depth() {
        let mut world = World::default();
        let plane = Object::Plane(Plane::new(Material {
            reflective: 0.5,
            ..Material::default()
        }))
        .set_transform(Matrix44::translation(0.0, -1.0, 0.0));
        world.objects.push(plane.clone());
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -3.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: -2.0_f64.sqrt() / 2.0,
                z: 2.0_f64.sqrt() / 2.0,
            },
        };
        let intersection = Intersection {
            object: plane.clone(),
            distance: 2.0_f64.sqrt(),
        };
        let comps = prepare_computations(&intersection, &ray, &vec![intersection.clone()]);
        let color = reflected_color(&world, &comps, 0);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn test_n1_n2_multi_intersection() {
        let a = Object::Sphere(Sphere::glass(1.5)).set_transform(Matrix44::scaling(2.0, 2.0, 2.0));
        let b = Object::Sphere(Sphere::glass(2.0))
            .set_transform(Matrix44::translation(0.0, 0.0, -0.25));
        let c =
            Object::Sphere(Sphere::glass(2.5)).set_transform(Matrix44::translation(0.0, 0.0, 0.25));
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -4.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let intersections = vec![
            Intersection {
                object: a.clone(),
                distance: 2.0,
            },
            Intersection {
                object: b.clone(),
                distance: 2.75,
            },
            Intersection {
                object: c.clone(),
                distance: 3.25,
            },
            Intersection {
                object: b.clone(),
                distance: 4.75,
            },
            Intersection {
                object: c.clone(),
                distance: 5.25,
            },
            Intersection {
                object: a.clone(),
                distance: 6.0,
            },
        ];
        let mut index = 0;
        for inter in intersections.to_vec() {
            let comps = prepare_computations(&inter, &ray, &intersections.to_vec());
            match index {
                0 => {
                    assert_eq!(comps.n1, 1.0);
                    assert_eq!(comps.n2, 1.5);
                }
                1 => {
                    assert_eq!(comps.n1, 1.5);
                    assert_eq!(comps.n2, 2.0);
                }
                2 => {
                    assert_eq!(comps.n1, 2.0);
                    assert_eq!(comps.n2, 2.5);
                }
                3 => {
                    assert_eq!(comps.n1, 2.5);
                    assert_eq!(comps.n2, 2.5);
                }
                4 => {
                    assert_eq!(comps.n1, 2.5);
                    assert_eq!(comps.n2, 1.5);
                }
                5 => {
                    assert_eq!(comps.n1, 1.5);
                    assert_eq!(comps.n2, 1.0);
                }
                _ => {}
            }
            index += 1;
        }
    }

    #[test]
    fn test_under_point_refraction() {
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
        let sphere =
            Object::Sphere(Sphere::glass(1.52)).set_transform(Matrix44::translation(0.0, 0.0, 1.0));
        let i = Intersection {
            object: sphere.clone(),
            distance: 5.0,
        };
        let comps = prepare_computations(&i, &ray, &vec![i.clone()]);
        assert!(comps.under_point.z > std::f64::EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn test_refracted_color_on_opaque() {
        let w = World::default();
        let s = &w.objects[0];
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
        let intersections = vec![
            Intersection {
                object: s.clone(),
                distance: 4.0,
            },
            Intersection {
                object: s.clone(),
                distance: 6.0,
            },
        ];
        let comps = prepare_computations(&intersections[0], &ray, &intersections);
        let c = refracted_color(&w, &comps, 5);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn test_refracted_color_max_depth() {
        let mut w = World::default();
        w.objects[0].set_material(Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Material::default()
        });
        let s = &w.objects[0];
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
        let intersections = vec![
            Intersection {
                object: s.clone(),
                distance: 4.0,
            },
            Intersection {
                object: s.clone(),
                distance: 6.0,
            },
        ];
        let comps = prepare_computations(&intersections[0], &ray, &intersections);
        let c = refracted_color(&w, &comps, 0);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn test_total_internal_reflection() {
        let mut w = World::default();
        w.objects[0].set_material(Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Material::default()
        });
        let s = &w.objects[0];
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 2.0_f64.sqrt() / 2.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let intersections = vec![
            Intersection {
                object: s.clone(),
                distance: -2.0_f64.sqrt() / 2.0,
            },
            Intersection {
                object: s.clone(),
                distance: 2.0_f64.sqrt() / 2.0,
            },
        ];
        let comps = prepare_computations(&intersections[1], &ray, &intersections);
        let c = refracted_color(&w, &comps, 5);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn test_refracted_color_on_ray() {
        let mut w = World::default();
        w.objects[0].set_material(Material {
            ambient: 1.0,
            pattern: Some(Pattern::TestPattern),
            ..Material::default()
        });
        w.objects[1].set_material(Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Material::default()
        });
        let a = &w.objects[0];
        let b = &w.objects[1];
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.1,
            },
            direction: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let intersections = vec![
            Intersection {
                object: a.clone(),
                distance: -0.9899,
            },
            Intersection {
                object: b.clone(),
                distance: -0.4899,
            },
            Intersection {
                object: b.clone(),
                distance: 0.4899,
            },
            Intersection {
                object: a.clone(),
                distance: 0.9899,
            },
        ];
        let comps = prepare_computations(&intersections[2], &ray, &intersections);
        let c = refracted_color(&w, &comps, 5);
        assert_eq!(
            c,
            Color {
                red: 0.0,
                green: 0.9988847,
                blue: 0.047216423,
            }
        );
    }

    #[test]
    fn test_shade_hit_on_transparent_material() {
        let mut w = World::default();
        let floor = Object::Plane(Plane::new(Material {
            transparency: 0.5,
            refractive_index: 1.5,
            ..Material::default()
        }))
        .set_transform(Matrix44::translation(0.0, -1.0, 0.0));
        w.objects.push(floor.clone());
        let ball = Object::Sphere(Sphere::new(&Material {
            ambient: 0.5,
            color: Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
            },
            ..Material::default()
        }))
        .set_transform(Matrix44::translation(0.0, -3.5, -0.5));
        w.objects.push(ball);
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -3.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: -2.0_f64.sqrt() / 2.0,
                z: 2.0_f64.sqrt() / 2.0,
            },
        };
        let intersections = vec![Intersection {
            object: floor.clone(),
            distance: 2.0_f64.sqrt(),
        }];
        let comps = prepare_computations(&intersections[0], &ray, &intersections);
        let c = shade_hit(&w, &comps, 5);
        assert_eq!(
            c,
            Color {
                red: 0.93642545,
                green: 0.68642545,
                blue: 0.68642545
            }
        );
    }

    #[test]
    fn test_reflectance_total_internal_reflection() {
        let sphere = Object::Sphere(Sphere::glass(1.52));
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 2.0_f64.sqrt() / 2.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let intersections = vec![
            Intersection {
                object: sphere.clone(),
                distance: -2.0_f64.sqrt() / 2.0,
            },
            Intersection {
                object: sphere.clone(),
                distance: 2.0_f64.sqrt() / 2.0,
            },
        ];
        let comps = prepare_computations(&intersections[1], &ray, &intersections);
        let reflectance = schlick(&comps);
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn test_reflectance_90_degrees_on_surface() {
        let sphere = Object::Sphere(Sphere::glass(1.52));
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let intersections = vec![
            Intersection {
                object: sphere.clone(),
                distance: -1.0,
            },
            Intersection {
                object: sphere.clone(),
                distance: 1.0,
            },
        ];
        let comps = prepare_computations(&intersections[1], &ray, &intersections);
        let reflectance = schlick(&comps);
        assert_eq!(reflectance, 0.04257998988032341);
    }

    #[test]
    fn test_reflectance_n2_greater_n1() {
        let sphere = Object::Sphere(Sphere::glass(1.52));
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.99,
                z: -2.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let intersections = vec![Intersection {
            object: sphere.clone(),
            distance: 1.8589,
        }];
        let comps = prepare_computations(&intersections[0], &ray, &intersections);
        let reflectance = schlick(&comps);
        assert_eq!(reflectance, 0.4901048406804614);
    }

    #[test]
    fn test_reflectance_used_in_shade_hit() {
        let mut w = World::default();
        let floor = Object::Plane(Plane::new(Material {
            reflective: 0.5,
            transparency: 0.5,
            refractive_index: 1.5,
            ..Material::default()
        }))
        .set_transform(Matrix44::translation(0.0, -1.0, 0.0));
        w.objects.push(floor.clone());
        let ball = Object::Sphere(Sphere::new(&Material {
            ambient: 0.5,
            color: Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
            },
            ..Material::default()
        }))
        .set_transform(Matrix44::translation(0.0, -3.5, -0.5));
        w.objects.push(ball);
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -3.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: -2.0_f64.sqrt() / 2.0,
                z: 2.0_f64.sqrt() / 2.0,
            },
        };
        let intersections = vec![Intersection {
            object: floor.clone(),
            distance: 2.0_f64.sqrt(),
        }];
        let comps = prepare_computations(&intersections[0], &ray, &intersections);
        let c = shade_hit(&w, &comps, 5);
        assert_eq!(
            c,
            Color {
                red: 0.9339152,
                green: 0.69643426,
                blue: 0.69243073
            }
        );
    }
}
