use glium::backend::Facade;
use glium::{self, DepthTest, DrawParameters, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use Camera;
use util::ToArr;
use base::math::*;
use base::prop::Plant;
use std::rc::Rc;
use super::PlantRenderer;

/// Graphical representation of a 'base::Plant'
pub struct PlantView {
    branches: Vec<VertexBuffer<Vertex>>,
    renderer: Rc<PlantRenderer>,
    pos: Point3f,
}

impl PlantView {
    pub fn from_plant<F: Facade>(pos: Point3f,
                                 plant: &Plant,
                                 renderer: Rc<PlantRenderer>,
                                 facade: &F)
                                 -> Self {
        // FIXME handle other plant types
        let mut verts = 0;
        let branches = match *plant {
            Plant::Tree { ref branches } => {
                branches.iter()
                    .map(|branch| {
                        let mut vertices = Vec::new();
                        for cp in branch.points.iter() {
                            verts += 1;
                            vertices.push(Vertex {
                                position: cp.point.to_arr(),
                                color: branch.color.to_arr(),
                            });
                        }

                        VertexBuffer::new(facade, &vertices).unwrap()
                    })
                    .collect()
            }
        };

        debug!("{} verts -> {:?}", verts, pos);

        PlantView {
            branches: branches,
            renderer: renderer,
            pos: pos,
        }
    }

    pub fn draw<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        let uniforms = uniform! {
            offset: self.pos.to_arr(),
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: camera.view_matrix().to_arr(),
        };

        let params = DrawParameters {
            depth: glium::Depth {
                write: true,
                test: DepthTest::IfLess,
                ..Default::default()
            },
            ..Default::default()
        };

        for vbuf in &self.branches {
            surface.draw(vbuf,
                      &NoIndices(PrimitiveType::LineStrip),
                      self.renderer.program(),
                      &uniforms,
                      &params)
                .unwrap();
        }
    }
}

/// Vertex type used to render plants/trees.
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);
