use rand::Rng;

pub struct Graph {
    edges: Vec<(Vertex, Vertex)>,
}

impl Graph {
    pub fn new_random(count: usize) -> Self {
        let mut rng = rand::thread_rng();

        let verts = (0..count)
            .map(|_| Vertex(rng.gen(), rng.gen(), rng.gen()))
            .collect::<Vec<Vertex>>();

        let edges = (0..count)
            .map(move |i| (verts[i], verts[rng.gen_range(0, verts.len())]))
            .collect::<Vec<_>>();

        Graph { edges }
    }
}

#[derive(Copy, Clone, Debug)]
struct Vertex(f32, f32, f32);
