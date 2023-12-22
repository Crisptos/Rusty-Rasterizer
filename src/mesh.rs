extern crate nalgebra_glm as glm;
#[derive(Clone)]
pub struct Triangle {
    pub points: [glm::Vec4; 3],
}

pub struct Mesh {
    pub tris: Vec<Triangle>,
}

impl Triangle {
    pub fn new(points: [glm::Vec4; 3]) -> Self {
        Triangle { points: (points) }
    }
}

impl Mesh {
    // Create an empty mesh
    pub fn new() -> Self {
        let tris = Vec::new();
        Self { tris: (tris) }
    }

    // Create a cube mesh
    pub fn new_cube() -> Self {
        let mut cube: Self = Mesh::new();

        cube.tris.push(Triangle {
            // South Face
            points: ([
                glm::Vec4::new(0.0, 0.0, 0., 1.0),
                glm::Vec4::new(0.0, 1.0, 0.0, 1.0),
                glm::Vec4::new(1.0, 1.0, 0.0, 1.0),
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                glm::Vec4::new(0.0, 0.0, 0.0, 1.0),
                glm::Vec4::new(1.0, 1.0, 0.0, 1.0),
                glm::Vec4::new(1.0, 0.0, 0.0, 1.0),
            ]),
        });

        cube.tris.push(Triangle {
            // East Face
            points: ([
                glm::Vec4::new(1.0, 0.0, 0.0, 1.0),
                glm::Vec4::new(1.0, 1.0, 0.0, 1.0),
                glm::Vec4::new(1.0, 1.0, 1.0, 1.0),
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                glm::Vec4::new(1.0, 0.0, 0.0, 1.0),
                glm::Vec4::new(1.0, 1.0, 1.0, 1.0),
                glm::Vec4::new(1.0, 0.0, 1.0, 1.0),
            ]),
        });

        cube.tris.push(Triangle {
            // North Face
            points: ([
                glm::Vec4::new(1.0, 0.0, 1.0, 1.0),
                glm::Vec4::new(1.0, 1.0, 1.0, 1.0),
                glm::Vec4::new(0.0, 1.0, 1.0, 1.0),
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                glm::Vec4::new(1.0, 0.0, 1.0, 1.0),
                glm::Vec4::new(0.0, 1.0, 1.0, 1.0),
                glm::Vec4::new(0.0, 0.0, 0.0, 1.0),
            ]),
        });

        cube.tris.push(Triangle {
            // West Face
            points: ([
                glm::Vec4::new(0.0, 0.0, 1.0, 1.0),
                glm::Vec4::new(0.0, 1.0, 1.0, 1.0),
                glm::Vec4::new(0.0, 1.0, 0.0, 1.0),
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                glm::Vec4::new(0.0, 0.0, 1.0, 1.0),
                glm::Vec4::new(0.0, 1.0, 0.0, 1.0),
                glm::Vec4::new(0.0, 0.0, 0.0, 1.0),
            ]),
        });

        cube.tris.push(Triangle {
            // Top Face
            points: ([
                glm::Vec4::new(0.0, 1.0, 0.0, 1.0),
                glm::Vec4::new(0.0, 1.0, 1.0, 1.0),
                glm::Vec4::new(1.0, 1.0, 1.0, 1.0),
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                glm::Vec4::new(0.0, 1.0, 0.0, 1.0),
                glm::Vec4::new(1.0, 1.0, 1.0, 1.0),
                glm::Vec4::new(1.0, 1.0, 0.0, 1.0),
            ]),
        });

        cube.tris.push(Triangle {
            // Bottom Face
            points: ([
                glm::Vec4::new(1.0, 0.0, 1.0, 1.0),
                glm::Vec4::new(0.0, 0.0, 1.0, 1.0),
                glm::Vec4::new(0.0, 0.0, 0.0, 1.0),
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                glm::Vec4::new(1.0, 0.0, 1.0, 1.0),
                glm::Vec4::new(0.0, 0.0, 0.0, 1.0),
                glm::Vec4::new(1.0, 0.0, 0.0, 1.0),
            ]),
        });

        cube
    }

    pub fn translate(&mut self, n: f32){
        for tri in &self.tris{
            for mut v in tri.points{
                println!("Old: {}", v.z);
                v.z += n;
                println!("New: {}", v.z);
            }
        }
    }
}
