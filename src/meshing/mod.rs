mod utils;

use crate::{Brush, Face};
use anyhow::{anyhow, Result};
use glam::{Mat3, Vec2, Vec3};

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

    pub fn from_brush(brush: &Brush) -> Result<Self> {
        let planes = brush
            .faces
            .iter()
            .map(|f| Plane::from_triangle(f.triangle))
            .collect::<Vec<_>>();
        let mut polys = planes
            .iter()
            .map(|p| Poly {
                normal: p.normal,
                verts: Vec::new(),
            })
            .collect::<Vec<_>>();

        for (i, p1) in planes.iter().enumerate() {
            for (j, p2) in planes.iter().enumerate() {
                'inner: for (k, p3) in planes.iter().enumerate() {
                    if let Some(point) = plane_point_intersect([p1, p2, p3]) {
                        // all verts must lie in a plane
                        for p in &planes {
                            let dist = p.normal.dot(point - p.origin);
                            if dist > 0.0 {
                                continue 'inner;
                            }
                        }

                        polys[i].add_vert(point, &brush.faces[i]);
                        polys[j].add_vert(point, &brush.faces[j]);
                        polys[k].add_vert(point, &brush.faces[k]);
                    }
                }
            }
        }

        let meshes = polys
            .iter()
            .map(|p| p.triangulate())
            .collect::<Result<Vec<_>>>()?;

        Ok(Self::merge(meshes))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Poly {
    pub normal: Vec3,
    pub verts: Vec<Vert>,
}

impl Poly {
    pub fn add_vert(&mut self, position: Vec3, face: &Face) {
        let Face {
            axis_u,
            axis_v,
            offset,
            scale,
            ..
        } = face;
        let scale = Vec2::from_slice(scale);
        let axis_u = Vec3::from_slice(axis_u) / scale.x;
        let axis_v = Vec3::from_slice(axis_v) / scale.y;
        let offset = Vec2::from_slice(offset);

        let uv = (Vec2::new(
            position.x * axis_u.x + position.y * axis_u.y + position.z * axis_u.z,
            position.x * axis_v.x + position.y * axis_v.y + position.z * axis_v.z,
        ) + offset)
            / 64.0;

        self.verts.push(Vert {
            position,
            normal: self.normal,
            uv,
        });
    }

    pub fn ordered_verts(&self) -> Result<Vec<Vert>> {
        if self.verts.len() < 3 {
            return Err(anyhow!("not enough points"));
        }

        let center = self
            .verts
            .iter()
            .fold(Vec3::ZERO, |acc, p| acc + p.position)
            / self.verts.len() as f32;

        let mut ordered = self.verts.clone();
        for n in 0..ordered.len() - 2 {
            let a = (ordered[n].position - center).normalize();
            let p = self.normal.cross(a);

            let mut smallest_angle = -1.0;
            let mut smallest = usize::MAX;

            for m in n + 1..ordered.len() {
                let b = (ordered[m].position - center).normalize();
                if p.dot(b) > 0.0 {
                    let angle = a.dot(b);
                    if angle > smallest_angle {
                        smallest_angle = angle;
                        smallest = m;
                    }
                }
            }

            ordered.swap(n + 1, smallest);
        }

        Ok(ordered)
    }

    pub fn triangulate(&self) -> Result<Mesh> {
        let verts = self.ordered_verts()?;

        let positions = verts.iter().map(|v| v.position.to_array()).collect();
        let normals = verts.iter().map(|v| v.normal.to_array()).collect();
        let uvs = verts.iter().map(|v| v.uv.to_array()).collect();
        let indices = (2..verts.len())
            .flat_map(|i| [0, (i - 1) as u32, i as u32])
            .collect();

        Ok(Mesh {
            positions,
            normals,
            uvs,
            indices,
            ..Default::default()
        })
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
