use crate::color::Color;
use crate::light::Light;
use crate::misc::approx_equal;
use crate::pattern::Pattern;
use crate::shape::Object;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pattern: Option<Pattern>,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
            reflective: 0.,
            pattern: None,
            transparency: 0.,
            refractive_index: 1.,
        }
    }

    pub fn with_pattern(pattern: Pattern) -> Self {
        Self {
            pattern: Some(pattern),
            ..Self::new()
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && approx_equal(self.ambient, other.ambient)
            && approx_equal(self.diffuse, other.diffuse)
            && approx_equal(self.specular, other.specular)
            && approx_equal(self.shininess, other.shininess)
    }
}

pub fn lighting(
    material: Material,
    object: Object,
    light: Light,
    point: Tuple,
    eye_vector: Tuple,
    normal_vector: Tuple,
    in_shadow: bool,
) -> Color {
    let color = if let Some(pattern) = material.pattern {
        pattern.pattern_at_object(object, point)
    } else {
        material.color
    };

    // combine the surface color with the light's color/intensity
    let effective_color = color * light.intensity;
    // find the direction to the light source
    let light_vector = (light.position - point).normalize();
    // compute the ambient contribution
    let ambient = effective_color * material.ambient;

    // light_dot_normal represents the cosine of the angle between the
    // light vector and the normal vector. A negative number means the
    // light is on the other side of the surface.
    let light_dot_normal = light_vector.dot(normal_vector);

    let (diffuse, specular) = if light_dot_normal < 0. {
        (Color::black(), Color::black())
    } else {
        // compute the diffuse contribution
        let diffuse = effective_color * material.diffuse * light_dot_normal;
        // reflect_dot_eye represents the cosine of the angle between the
        // reflection vector and the eye vector. A negative number means the
        // light reflects away from the eye.
        let reflect_vector = (-light_vector).reflect(normal_vector);
        let reflect_dot_eye = reflect_vector.dot(eye_vector);
        if reflect_dot_eye <= 0. {
            let specular = Color::black();
            (diffuse, specular)
        } else {
            // compute the specular contribution
            let factor = reflect_dot_eye.powf(material.shininess);
            let specular = light.intensity * material.specular * factor;
            (diffuse, specular)
        }
    };

    if in_shadow {
        ambient
    } else {
        ambient + diffuse + specular
    }
}
#[cfg(test)]
mod tests {
    use crate::light::Light;
    use crate::misc::approx_equal;
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn the_default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::new(1., 1., 1.));
        assert!(approx_equal(m.ambient, 0.1));
        assert!(approx_equal(m.diffuse, 0.9));
        assert!(approx_equal(m.specular, 0.9));
        assert!(approx_equal(m.shininess, 200.));
        assert_eq!(m.reflective, 0.);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let o = Object::sphere();
        let position = Tuple::point(0., 0., 0.);
        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = lighting(m, o, light, position, eye_vector, normal_vector, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = Material::new();
        let o = Object::sphere();
        let position = Tuple::point(0., 0., 0.);
        let eye_vector = Tuple::vector(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = lighting(m, o, light, position, eye_vector, normal_vector, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::new();
        let o = Object::sphere();
        let position = Tuple::point(0., 0., 0.);
        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));
        let result = lighting(m, o, light, position, eye_vector, normal_vector, false);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let o = Object::sphere();
        let position = Tuple::point(0., 0., 0.);
        let eye_vector = Tuple::vector(0., -2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));
        let result = lighting(m, o, light, position, eye_vector, normal_vector, false);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let o = Object::sphere();
        let position = Tuple::point(0., 0., 0.);
        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 0., 10.), Color::new(1., 1., 1.));
        let result = lighting(m, o, light, position, eye_vector, normal_vector, false);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::new();
        let o = Object::sphere();
        let eye_vector = Tuple::vector(0., 0., -1.);
        let position = Tuple::point(0., 0., 0.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = Light::point_light(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let in_shadow = true;
        let result = lighting(m, o, light, position, eye_vector, normal_vector, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn transparency_and_refractive_index_for_the_default_material() {
        let m = Material::new();
        assert!(approx_equal(m.transparency, 0.0));
        assert!(approx_equal(m.refractive_index, 1.0));
    }
}
