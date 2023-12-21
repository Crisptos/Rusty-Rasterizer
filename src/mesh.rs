// Represents a coordinate in 3D world space
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Triangle {
    points: [Vector3; 3],
}

pub struct Mesh {
    tris: Vec<Triangle>,
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

        cube.tris.push(Triangle {   // South Face
            points: ([
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ]),
        });                         // South Face

        cube.tris.push(Triangle {   // East Face
            points: ([
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
            ]),
        });                         // East Face

        cube.tris.push(Triangle {   // North Face
            points: ([
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            ]),
        });                         // North Face

        cube.tris.push(Triangle {   // West Face
            points: ([
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            ]),
        });                         // West Face

        cube.tris.push(Triangle {   // Top Face
            points: ([
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
            ]),
        });                         // Top Face

        cube.tris.push(Triangle {   // Bottom Face
            points: ([
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            ]),
        });
        cube.tris.push(Triangle {
            points: ([
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ]),
        });                         // Bottom Face

        cube
    }
}
