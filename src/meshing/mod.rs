mod poly;
mod utils;

use self::poly::{Poly, ToPolys};
use crate::{Brush, Entity, Face};
use anyhow::{anyhow, Result};
use glam::{Mat3, Vec2, Vec3};
use std::collections::HashMap;

pub trait ToMesh {
    fn to_mesh(&self) -> Result<Mesh>;
}

impl ToMesh for Brush {
    fn to_mesh(&self) -> Result<Mesh> {
        Mesh::from_brush(self)
    }
}

impl ToMesh for Entity {
    fn to_mesh(&self) -> Result<Mesh> {
        Mesh::from_entity(self)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Mesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn merge(meshes: Vec<Mesh>) -> Self {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        for mut mesh in meshes {
            let offset = positions.len() as u32;
            positions.append(&mut mesh.positions);
            normals.append(&mut mesh.normals);
            uvs.append(&mut mesh.uvs);
            indices.append(&mut mesh.indices.iter().map(|i| *i + offset).collect());
        }

        Self {
            positions,
            normals,
            uvs,
            indices,
        }
    }

    fn from_polys(polys: &Vec<Poly>) -> Result<Self> {
        let meshes = polys
            .iter()
            .map(|p| p.triangulate())
            .collect::<Result<Vec<_>>>()?;

        Ok(Self::merge(meshes))
    }

    pub fn from_brush(brush: &Brush) -> Result<Self> {
        let polys = brush.to_polys();

        // let mut map: HashMap<String, Vec<Poly>> = HashMap::new();
        // for poly in polys {
        //     let key = poly.texture.clone();
        //     if let Some(val) = map.get_mut(&key) {
        //         val.push(poly);
        //     } else {
        //         map.insert(key, vec![poly]);
        //     }
        // }

        let meshes = polys
            .iter()
            .map(|p| p.triangulate())
            .collect::<Result<Vec<_>>>()?;

        Ok(Self::merge(meshes))
    }

    pub fn from_entity(entity: &Entity) -> Result<Self> {
        if entity.brushes.is_empty() {
            return Err(anyhow!("entity has no brushes"));
        }

        let meshes = entity
            .brushes
            .iter()
            .map(|b| Self::from_brush(b))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self::merge(meshes))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Vert {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

#[derive(Debug, Clone)]
pub(crate) struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(origin: Vec3, normal: Vec3) -> Self {
        Self { origin, normal }
    }

    pub fn from_triangle(points: [[f32; 3]; 3]) -> Self {
        let p1 = Vec3::from_array(points[0]);
        let p2 = Vec3::from_array(points[1]);
        let p3 = Vec3::from_array(points[2]);

        let d1 = (p2 - p1).normalize();
        let d2 = (p3 - p1).normalize();

        Self::new(p1, d2.cross(d1))
    }
}

pub(crate) fn plane_point_intersect(planes: [&Plane; 3]) -> Option<Vec3> {
    let x1 = planes[0].origin;
    let n1 = planes[0].normal;
    let x2 = planes[1].origin;
    let n2 = planes[1].normal;
    let x3 = planes[2].origin;
    let n3 = planes[2].normal;

    let mat = Mat3::from_cols(n1, n2, n3);
    let det = mat.determinant();

    if det == 0.0 {
        return None;
    }

    let v1 = x1.dot(n1) * n2.cross(n3);
    let v2 = x2.dot(n2) * n3.cross(n1);
    let v3 = x3.dot(n3) * n1.cross(n2);

    Some(det.powf(-1.0) * (v1 + v2 + v3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_merge() {
        let mesh1 = Mesh {
            positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
            normals: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
            uvs: vec![[0.0, 0.0], [1.0, 0.0], [2.0, 0.0]],
            indices: vec![0, 1, 2],
        };
        let mesh2 = Mesh {
            positions: vec![[3.0, 0.0, 0.0], [4.0, 0.0, 0.0], [5.0, 0.0, 0.0]],
            normals: vec![[3.0, 0.0, 0.0], [4.0, 0.0, 0.0], [5.0, 0.0, 0.0]],
            uvs: vec![[3.0, 0.0], [4.0, 0.0], [5.0, 0.0]],
            indices: vec![0, 1, 2],
        };
        let merged = Mesh::merge(vec![mesh1, mesh2]);

        assert_eq!(
            merged,
            Mesh {
                positions: vec![
                    [0.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [2.0, 0.0, 0.0],
                    [3.0, 0.0, 0.0],
                    [4.0, 0.0, 0.0],
                    [5.0, 0.0, 0.0]
                ],
                normals: vec![
                    [0.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [2.0, 0.0, 0.0],
                    [3.0, 0.0, 0.0],
                    [4.0, 0.0, 0.0],
                    [5.0, 0.0, 0.0]
                ],
                uvs: vec![
                    [0.0, 0.0],
                    [1.0, 0.0],
                    [2.0, 0.0],
                    [3.0, 0.0],
                    [4.0, 0.0],
                    [5.0, 0.0]
                ],
                indices: vec![0, 1, 2, 3, 4, 5],
            }
        )
    }
}
