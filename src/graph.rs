use rand::Rng;
use std::collections::HashSet;

pub struct Graph<'a> {
    verts: Vec<Vertex>,
    edges: HashSet<Edge>,
}

impl<'a> Graph<'a> {
    pub fn new_random(count: usize) -> Self {
        let mut rng = rand::thread_rng();

        let v = (0..count)
            .map(|_| Vertex(rng.gen(), rng.gen(), rng.gen()))
            .collect::<Vec<Vector>>();
    }
}

struct Vertex(f32, f32, f32);
struct Edge<'a>(&'a Vector, &'a Vector);
