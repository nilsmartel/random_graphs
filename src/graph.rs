use rand::Rng;

pub struct Graph {
    pub edges: Vec<(Vertex, Vertex)>,
}

impl Graph {
    pub fn new_random(count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let offset = Vertex(-0.5, -0.5, -0.5);
        let verts = (0..count)
            .map(|_| Vertex(rng.gen(), rng.gen(), rng.gen()) + offset)
            .collect::<Vec<Vertex>>();

        let edges = (0..count)
            .map(move |i| (verts[i], verts[rng.gen_range(0, verts.len())]))
            .collect::<Vec<_>>();

        Graph { edges }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex(pub f32, pub f32, pub f32);

impl std::ops::Add for Vertex {
    type Output = Self;

    fn add(self, v: Vertex) -> Vertex {
        Vertex(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
}

impl std::ops::Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, s: f32) -> Vertex {
        Vertex(self.0 * s, self.1 * s, self.2 * s)
    }
}
