use crate::{canvas::Canvas, matrix4::Matrix4, ray::Ray, tuple::Tuple, world::World};

#[derive(Clone, Copy)]
pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub field_of_view: f64,
    pub transform: Matrix4,
}

impl Camera {
    pub fn new(hsize: i32, vsize: i32, field_of_view: f64) -> Self {
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4::identity(),
        }
    }

    fn half_extents(self) -> (f64, f64) {
        let half_view = (self.field_of_view / 2.).tan();
        let aspect = self.hsize as f64 / self.vsize as f64;

        let (half_width, half_height) = if aspect > 1. {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        (half_width, half_height)
    }

    fn pixel_size(self) -> f64 {
        let (half_width, _) = self.half_extents();

        2. * half_width / self.hsize as f64
    }

    pub fn ray_for_pixel(self, px: i32, py: i32) -> Ray {
        let x_offset = (px as f64 + 0.5) * self.pixel_size();
        let y_offset = (py as f64 + 0.5) * self.pixel_size();

        let (half_width, half_height) = self.half_extents();
        let world_x = half_width - x_offset;
        let world_y = half_height - y_offset;

        let inverse_transform = self.transform.inverse().unwrap();
        let pixel = inverse_transform * Tuple::point(world_x, world_y, -1.);
        let origin = inverse_transform * Tuple::point(0., 0., 0.);

        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize as usize, self.vsize as usize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);

                canvas.write_pixel(x, y, color);
            }
        }

        canvas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Color, misc::approx_equal, transformations::view_transform, world::World};
    use std::f64::consts::PI;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.;
        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert!(approx_equal(c.field_of_view, PI / 2.));
        assert_eq!(c.transform, Matrix4::identity());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.);

        assert!(approx_equal(c.pixel_size(), 0.01));
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.);

        assert!(approx_equal(c.pixel_size(), 0.01));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0., 0., 0.));
        assert_eq!(r.direction, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Tuple::point(0., 0., 0.));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.);
        c.transform = Matrix4::rotation_y(PI / 4.) * Matrix4::translation(0., -2., 5.);

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0., 2., -5.));
        assert_eq!(
            r.direction,
            Tuple::vector(2_f64.sqrt() / 2., 0., -2_f64.sqrt() / 2.)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();

        let mut c = Camera::new(11, 11, PI / 2.);

        let from = Tuple::point(0., 0., -5.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::vector(0., 1., 0.);
        c.transform = view_transform(from, to, up);

        let image = c.render(&w);

        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
