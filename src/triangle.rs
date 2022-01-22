use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::BuildHasherDefault,
};

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
    kind: TriangleKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TriangleKind {
    Flat,
    Smooth { n1: Tuple, n2: Tuple, n3: Tuple },
}

impl Triangle {
    pub(crate) fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Self {
        Self {
            p1,
            p2,
            p3,
            kind: TriangleKind::Flat,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn smooth(p1: Tuple, p2: Tuple, p3: Tuple, n1: Tuple, n2: Tuple, n3: Tuple) -> Self {
        Self {
            p1,
            p2,
            p3,
            kind: TriangleKind::Smooth { n1, n2, n3 },
        }
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

    pub(crate) fn local_normal_at(&self, uvt: &UVT) -> Tuple {
        let UVT { u, v, .. } = uvt;

        match self.kind {
            TriangleKind::Flat => self.normal(),
            TriangleKind::Smooth { n1, n2, n3 } => {
                (n2 * *u + n3 * *v + n1 * (1. - *u - *v)).normalize()
            }
        }
    }

    pub(crate) fn local_intersect(&self, local_ray: Ray) -> Vec<UVT> {
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
        vec![UVT { u, v, t }]
    }

    pub(crate) fn bounding_box(&self) -> BoundingBox {
        BoundingBox::from_points(&[self.p1, self.p2, self.p3])
    }

    pub(crate) fn from_obj_file(file_contents: &str) -> std::io::Result<WavefrontObj> {
        let mut vertices = vec![];
        let mut normals = vec![];

        let mut current_group = "default";

        let map_hasher = BuildHasherDefault::<DefaultHasher>::default();
        let mut groups: HashMap<String, Vec<Triangle>, _> = HashMap::with_hasher(map_hasher);

        for line in file_contents.lines() {
            if let Some((node_type, rest)) = line.split_once(" ") {
                match node_type {
                    "v" => {
                        let mut rest = rest.split_ascii_whitespace();
                        let x = rest.next().unwrap().parse::<f64>().unwrap();
                        let y = rest.next().unwrap().parse::<f64>().unwrap();
                        let z = rest.next().unwrap().parse::<f64>().unwrap();

                        vertices.push(Tuple::point(x, y, z));
                    }
                    "vn" => {
                        let mut rest = rest.split_ascii_whitespace();
                        let x = rest.next().unwrap().parse::<f64>().unwrap();
                        let y = rest.next().unwrap().parse::<f64>().unwrap();
                        let z = rest.next().unwrap().parse::<f64>().unwrap();

                        normals.push(Tuple::vector(x, y, z));
                    }
                    "f" => {
                        // "1//3 2//4 3//5"
                        let rest = rest.split_ascii_whitespace();
                        // ["1//3", "2//4", "3//5"]
                        let mut indices = rest.map(|attr| {
                            let mut it = attr.split('/').map(|i| i.parse::<usize>().ok());

                            let vertex = it.next().unwrap().unwrap() - 1;
                            let texture = it.next().flatten().map(|t| t - 1);
                            let normal = it.next().flatten().map(|t| t - 1);

                            (vertex, texture, normal)
                        });

                        let (start_index, _, normal1) = indices.next().unwrap();
                        for window in indices.collect::<Vec<_>>().windows(2) {
                            if let [(index2, _, normal2), (index3, _, normal3)] = window {
                                let entry = groups.entry(current_group.to_owned());
                                let triangle = match (normal1, normal2, normal3) {
                                    (Some(n1), Some(n2), Some(n3)) => Triangle::smooth(
                                        vertices[start_index],
                                        vertices[*index2],
                                        vertices[*index3],
                                        normals[n1],
                                        normals[*n2],
                                        normals[*n3],
                                    ),
                                    _ => Triangle::new(
                                        vertices[start_index],
                                        vertices[*index2],
                                        vertices[*index3],
                                    ),
                                };

                                entry.or_insert(vec![]).push(triangle);
                            }
                        }
                    }
                    "g" => {
                        current_group = rest;
                    }
                    _ => {}
                }
            }
        }

        Ok(WavefrontObj {
            groups,
            #[cfg(test)]
            vertices,
            #[cfg(test)]
            normals,
        })
    }
}

pub(crate) struct WavefrontObj {
    groups: HashMap<String, Vec<Triangle>, BuildHasherDefault<DefaultHasher>>,
    #[cfg(test)]
    vertices: Vec<Tuple>,
    #[cfg(test)]
    normals: Vec<Tuple>,
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

#[derive(Clone, Copy)]
pub(crate) struct UVT {
    pub(crate) t: f64,
    pub(crate) u: f64,
    pub(crate) v: f64,
}

#[cfg(test)]
mod tests {
    use crate::{
        intersection::{Intersection, TorUVT},
        misc::approx_equal,
        shape::{ShapeOrGroup, SimpleObject},
    };

    use super::*;

    impl Triangle {
        fn normals(&self) -> (Tuple, Tuple, Tuple) {
            match self.kind {
                TriangleKind::Flat => (self.normal(), self.normal(), self.normal()),
                TriangleKind::Smooth { n1, n2, n3 } => (n1, n2, n3),
            }
        }
    }

    fn test_smooth_tri() -> Triangle {
        let p1 = Tuple::point(0., 1., 0.);
        let p2 = Tuple::point(-1., 0., 0.);
        let p3 = Tuple::point(1., 0., 0.);
        let n1 = Tuple::vector(0., 1., 0.);
        let n2 = Tuple::vector(-1., 0., 0.);
        let n3 = Tuple::vector(1., 0., 0.);

        Triangle::smooth(p1, p2, p3, n1, n2, n3)
    }

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
        let uvt1 = UVT {
            t: 0.,
            u: 0.5,
            v: 0.25,
        };
        let uvt2 = UVT {
            t: 0.,
            u: 0.75,
            v: 0.25,
        };
        let uvt3 = UVT {
            t: 0.,
            u: 0.25,
            v: 0.5,
        };
        let n1 = t.local_normal_at(&uvt1);
        let n2 = t.local_normal_at(&uvt2);
        let n3 = t.local_normal_at(&uvt3);

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
        assert!(approx_equal(xs[0].t, 2.));
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
    fn converting_an_obj_file_to_a_group() {
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

        let g = obj.to_group();

        let group_objects = if let ShapeOrGroup::Group(group) = g.shape {
            group
        } else {
            panic!("Didn't get a group back from obj file!")
        };

        // The order of the triangles in this test is a bit arbitrary
        // because of iteration order in a HashMap
        assert_eq!(
            group_objects[1],
            Object::group(vec![Object::new(Shape::Triangle(t2))])
        );
        assert_eq!(
            group_objects[0],
            Object::group(vec![Object::new(Shape::Triangle(t1))])
        );
    }

    #[test]
    fn a_smooth_triangle_uses_uv_to_interpolate_the_normal() {
        let i = UVT {
            u: 0.45,
            v: 0.25,
            t: 1.,
        };
        let tri = test_smooth_tri();
        let n = tri.local_normal_at(&i);

        assert_eq!(n, Tuple::vector(-0.5547, 0.83205, 0.));
    }

    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let uvt = UVT {
            t: 1.,
            u: 0.45,
            v: 0.25,
        };
        let r = Ray::new(Tuple::point(-0.2, 0.3, -2.), Tuple::vector(0., 0., 1.));
        let tri = test_smooth_tri();
        let i = Intersection::new(
            &TorUVT::UVT { uvt },
            SimpleObject::new(Shape::Triangle(tri)),
        );
        let comps = i.prepare_computations(r, &[i]);

        assert_eq!(comps.normal_vector, Tuple::vector(-0.5547, 0.83205, 0.));
    }

    #[test]
    fn vertex_normal_records() {
        let file_contents = r#"
vn 0 0 1
vn 0.707 0 -0.707
vn 1 2 3
"#;

        let obj = Triangle::from_obj_file(file_contents).unwrap();
        assert_eq!(obj.normals[1 - 1], Tuple::vector(0., 0., 1.));
        assert_eq!(obj.normals[2 - 1], Tuple::vector(0.707, 0., -0.707));
        assert_eq!(obj.normals[3 - 1], Tuple::vector(1., 2., 3.));
    }

    #[test]
    fn faces_with_normals() {
        let file_contents = r#"
v 0 1 0
v -1 0 0
v 1 0 0
vn -1 0 0
vn 1 0 0
vn 0 1 0
f 1//3 2//1 3//2
f 1/1/3 2/102/1 3/14/2
"#;

        let obj = Triangle::from_obj_file(file_contents).unwrap();
        let g = &obj.groups["default"];
        let t1 = g[0];
        let t2 = g[1];

        assert_eq!(t1.p1, obj.vertices[1 - 1]);
        assert_eq!(t1.p2, obj.vertices[2 - 1]);
        assert_eq!(t1.p3, obj.vertices[3 - 1]);
        assert_eq!(t1.normals().0, obj.normals[3 - 1]);
        assert_eq!(t1.normals().1, obj.normals[1 - 1]);
        assert_eq!(t1.normals().2, obj.normals[2 - 1]);
        assert_eq!(t2, t1);
    }
}
