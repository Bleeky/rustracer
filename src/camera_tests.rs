#[cfg(test)]
mod tests {
    use crate::camera::*;
    use crate::color::*;
    use crate::intersection::*;
    use crate::vector3::*;
    use crate::world::*;

    #[test]
    fn test_pixel_size_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f64::consts::PI / 2.0);
        assert_eq!(c.pixel_size, 0.009999999999999998);
    }

    #[test]
    fn test_pixel_size_vertical_canvas() {
        let c = Camera::new(125, 200, std::f64::consts::PI / 2.0);
        assert_eq!(c.pixel_size, 0.009999999999999998);
    }

    #[test]
    fn test_ray_through_canvas_center() {
        let c = Camera::new(201, 101, std::f64::consts::PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(
            r.origin,
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        );
        assert_eq!(
            r.direction,
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        );
    }

    #[test]
    fn test_ray_camera_transformed() {
        let mut c = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
        c.transform = Matrix44::translation(0.0, -2.0, 5.0).rotate_y(std::f64::consts::FRAC_PI_4);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(
            r.origin,
            Point {
                x: 0.0,
                y: 2.0,
                z: -5.0,
            }
        );
        assert_eq!(
            r.direction,
            Vector3 {
                x: (2.0_f64.sqrt() / 2.0),
                y: 0.0,
                z: -(2.0_f64.sqrt() / 2.0),
            }
        );
    }

    #[test]
    fn test_ray_camera_through_corner() {
        let c = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(
            r.origin,
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        );
        assert_eq!(
            r.direction,
            Vector3 {
                x: 0.6651864261194508,
                y: 0.3325932130597254,
                z: -0.6685123582500481,
            }
        );
    }

    #[test]
    fn test_render_pixel() {
        let world = World::default();
        let mut camera = Camera::new(11, 11, std::f64::consts::FRAC_PI_2);
        camera.transform = view_transform(
            Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );
        let ray = camera.ray_for_pixel(5, 5);
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
}
