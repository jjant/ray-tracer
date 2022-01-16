use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

use crate::{
    misc::EPSILON,
    ray::Ray,
    shape::{BoundingBox, Object, Shape},
    tuple::Tuple,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    p1: Tuple,
    p2: Tuple,
    p3: Tuple,
}

impl Triangle {
    pub(crate) fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Self {
        Self { p1, p2, p3 }
    }

    fn edge1(&self) -> Tuple {
        self.p2 - self.p1
    }

    fn edge2(&self) -> Tuple {
        self.p3 - self.p1
    }

    fn normal(&self) -> Tuple {
        self.edge2().cross(self.edge1()).normalize()
    }

    pub(crate) fn local_normal_at(&self, _local_point: Tuple) -> Tuple {
        self.normal()
    }

    pub(crate) fn local_intersect(&self, local_ray: Ray) -> Vec<f64> {
        let dir_cross_edge2 = local_ray.direction.cross(self.edge2());
        let det = self.edge1().dot(dir_cross_edge2);

        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / det;
        let p1_to_origin = local_ray.origin - self.p1;
        let u = f * p1_to_origin.dot(dir_cross_edge2);
        if u < 0. || u > 1. {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross(self.edge1());
        let v = f * local_ray.direction.dot(origin_cross_e1);
        if v < 0. || (u + v) > 1. {
            return vec![];
        }

        let t = f * self.edge2().dot(origin_cross_e1);
        vec![t]
    }

    pub(crate) fn bounding_box(&self) -> BoundingBox {
        BoundingBox::from_points(&[self.p1, self.p2, self.p3])
    }

    pub(crate) fn from_obj_file(file_contents: &str) -> std::io::Result<WavefrontObj> {
        let mut vertices = vec![];

        let mut current_group = "default";
        let mut groups: HashMap<String, Vec<Triangle>> = HashMap::new();

        for line in file_contents.lines() {
            if let Some((node_type, rest)) = line.split_once(" ") {
                if node_type == "v" {
                    let mut rest = rest.split(" ");
                    let x = rest.next().unwrap().parse::<f64>().unwrap();
                    let y = rest.next().unwrap().parse::<f64>().unwrap();
                    let z = rest.next().unwrap().parse::<f64>().unwrap();

                    vertices.push(Tuple::point(x, y, z));
                } else if node_type == "f" {
                    let rest = rest.split(" ");
                    let mut indices = rest.map(|i| i.parse::<usize>().unwrap() - 1);

                    let start_index = indices.next().unwrap();
                    for window in indices.collect::<Vec<_>>().windows(2) {
                        if let [index2, index3] = window {
                            let entry = groups.entry(current_group.to_owned());

                            entry.or_insert(vec![]).push(Triangle::new(
                                vertices[start_index],
                                vertices[*index2],
                                vertices[*index3],
                            ));
                        }
                    }
                } else if node_type == "g" {
                    current_group = rest;
                } else {
                    // Skip line
                }
            }
        }

        Ok(WavefrontObj { vertices, groups })
    }
}

pub(crate) struct WavefrontObj {
    vertices: Vec<Tuple>,
    groups: HashMap<String, Vec<Triangle>>,
}

impl WavefrontObj {
    pub(crate) fn to_group(self) -> Object {
        Object::group(
            self.groups
                .into_iter()
                .map(|(_, triangles)| {
                    let triangles = triangles
                        .into_iter()
                        .map(|triangle| Object::new(Shape::Triangle(triangle)))
                        .collect();

                    Object::group(triangles)
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::misc::approx_equal;

    use super::*;

    #[test]
    fn constructing_a_triangle() {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);
        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.edge1(), Tuple::vector(-1., -1., 0.));
        assert_eq!(t.edge2(), Tuple::vector(1., -1., 0.));
        assert_eq!(t.normal(), Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let n1 = t.local_normal_at(Tuple::point(0., 0.5, 0.));
        let n2 = t.local_normal_at(Tuple::point(-0.5, 0.75, 0.));
        let n3 = t.local_normal_at(Tuple::point(0.5, 0.25, 0.));

        assert_eq!(n1, t.normal());
        assert_eq!(n2, t.normal());
        assert_eq!(n3, t.normal());
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let r = Ray::new(Tuple::point(0., -1., -2.), Tuple::vector(0., 1., 0.));
        let xs = t.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1p3_edge() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let r = Ray::new(Tuple::point(1., 1., -2.), Tuple::vector(0., 0., 1.));
        let xs = t.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p1p2_edge() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let r = Ray::new(Tuple::point(-1., 1., -2.), Tuple::vector(0., 0., 1.));
        let xs = t.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_misses_the_p2p3_edge() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let r = Ray::new(Tuple::point(0., -1., -2.), Tuple::vector(0., 0., 1.));
        let xs = t.local_intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = Triangle::new(
            Tuple::point(0., 1., 0.),
            Tuple::point(-1., 0., 0.),
            Tuple::point(1., 0., 0.),
        );
        let r = Ray::new(Tuple::point(0., 0.5, -2.), Tuple::vector(0., 0., 1.));
        let xs = t.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert!(approx_equal(xs[0], 2.));
    }

    #[test]
    fn parse_vertices() {
        let file_contents = r#"
v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0"#;

        let obj = Triangle::from_obj_file(file_contents).unwrap();

        assert_eq!(obj.vertices[1 - 1], Tuple::point(-1., 1., 0.));
        assert_eq!(obj.vertices[2 - 1], Tuple::point(-1., 0.5, 0.));
        assert_eq!(obj.vertices[3 - 1], Tuple::point(1., 0., 0.));
        assert_eq!(obj.vertices[4 - 1], Tuple::point(1., 1., 0.));
    }

    #[test]
    fn parsing_triangle_faces() {
        let file_contents = r#"
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
f 1 2 3
f 1 3 4
"#;
        let obj = Triangle::from_obj_file(file_contents).unwrap();
        let t1 = obj.groups["default"][0];
        let t2 = obj.groups["default"][1];

        assert_eq!(t1.p1, obj.vertices[1 - 1]);
        assert_eq!(t1.p2, obj.vertices[2 - 1]);
        assert_eq!(t1.p3, obj.vertices[3 - 1]);
        assert_eq!(t2.p1, obj.vertices[1 - 1]);
        assert_eq!(t2.p2, obj.vertices[3 - 1]);
        assert_eq!(t2.p3, obj.vertices[4 - 1]);
    }

    #[test]
    fn triangulating_polygons() {
        let file_contents = r#"
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0
f 1 2 3 4 5
"#;
        let obj = Triangle::from_obj_file(file_contents).unwrap();

        let t1 = obj.groups["default"][0];
        let t2 = obj.groups["default"][1];
        let t3 = obj.groups["default"][2];

        assert_eq!(t1.p1, obj.vertices[1 - 1]);
        assert_eq!(t1.p2, obj.vertices[2 - 1]);
        assert_eq!(t1.p3, obj.vertices[3 - 1]);
        assert_eq!(t2.p1, obj.vertices[1 - 1]);
        assert_eq!(t2.p2, obj.vertices[3 - 1]);
        assert_eq!(t2.p3, obj.vertices[4 - 1]);
        assert_eq!(t3.p1, obj.vertices[1 - 1]);
        assert_eq!(t3.p2, obj.vertices[4 - 1]);
        assert_eq!(t3.p3, obj.vertices[5 - 1]);
    }

    #[test]
    fn triangles_in_groups() {
        let file_contents = r#"
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
"#;
        let obj = Triangle::from_obj_file(file_contents).unwrap();

        let t1 = obj.groups["FirstGroup"][0];
        let t2 = obj.groups["SecondGroup"][0];

        assert_eq!(t1.p1, obj.vertices[1 - 1]);
        assert_eq!(t1.p2, obj.vertices[2 - 1]);
        assert_eq!(t1.p3, obj.vertices[3 - 1]);
        assert_eq!(t2.p1, obj.vertices[1 - 1]);
        assert_eq!(t2.p2, obj.vertices[3 - 1]);
        assert_eq!(t2.p3, obj.vertices[4 - 1]);
    }

    #[test]
    fn converting_an_OBJ_file_to_a_group() {
        let file_contents = r#"
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
"#;
        let obj = Triangle::from_obj_file(file_contents).unwrap();
        let g = obj.to_group();

        assert!(true);
        // TODO:
        // Then g includes "FirstGroup" from parser
        // And g includes "SecondGroup" from parser
    }
}
