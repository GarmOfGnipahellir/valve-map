use super::{plane_point_intersect, Plane, Vert};
use crate::{Brush, Entity, Face, Mesh};
use anyhow::{anyhow, Result};
use glam::{Vec2, Vec3};

pub(crate) trait ToPolys {
    fn to_polys(&self) -> Vec<Poly>;
}

impl ToPolys for Brush {
    fn to_polys(&self) -> Vec<Poly> {
        let (planes, mut polys) = self
            .faces
            .iter()
            .map(|face| {
                let plane = Plane::from_triangle(face.triangle);
                let poly = Poly {
                    normal: plane.normal,
                    verts: Vec::new(),
                    texture: face.texture_name.to_string(),
                };
                (plane, poly)
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

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

                        polys[i].add_vert(point, &self.faces[i]);
                        polys[j].add_vert(point, &self.faces[j]);
                        polys[k].add_vert(point, &self.faces[k]);
                    }
                }
            }
        }

        polys
    }
}

impl ToPolys for Entity {
    fn to_polys(&self) -> Vec<Poly> {
        self.brushes
            .iter()
            .flat_map(|brush| brush.to_polys())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Poly {
    pub normal: Vec3,
    pub verts: Vec<Vert>,
    pub texture: String,
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
