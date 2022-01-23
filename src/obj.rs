use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::BuildHasherDefault,
};

use crate::{
    math::tuple::Tuple,
    shape::{triangle::Triangle, Object, Shape},
};

pub struct WavefrontObj {
    groups: HashMap<String, Vec<Triangle>, BuildHasherDefault<DefaultHasher>>,
    #[cfg(test)]
    vertices: Vec<Tuple>,
    #[cfg(test)]
    normals: Vec<Tuple>,
}

impl WavefrontObj {
    pub fn to_group(self) -> Object {
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

    pub fn from_file(file_path: &str) -> std::io::Result<Object> {
        let file_contents = std::fs::read_to_string(file_path)?;
        let obj = WavefrontObj::from_file_contents(&file_contents)?;
        Ok(obj.to_group())
    }

    pub fn from_file_contents(file_contents: &str) -> std::io::Result<WavefrontObj> {
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

#[cfg(test)]
mod tests {
    use crate::shape::ShapeOrGroup;

    use super::*;

    #[test]
    fn parse_vertices() {
        let file_contents = r#"
v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0"#;

        let obj = WavefrontObj::from_file_contents(file_contents).unwrap();

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
        let obj = WavefrontObj::from_file_contents(file_contents).unwrap();
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
        let obj = WavefrontObj::from_file_contents(file_contents).unwrap();

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
        let obj = WavefrontObj::from_file_contents(file_contents).unwrap();

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
        let obj = WavefrontObj::from_file_contents(file_contents).unwrap();
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
    fn vertex_normal_records() {
        let file_contents = r#"
vn 0 0 1
vn 0.707 0 -0.707
vn 1 2 3
"#;

        let obj = WavefrontObj::from_file_contents(file_contents).unwrap();
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

        let obj = WavefrontObj::from_file_contents(file_contents).unwrap();
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
