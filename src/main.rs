mod graph;
mod matrix;
mod render;
use matrix::Matrix;
use render::coordinates::Point;

fn main() {
    let width = 512usize;
    let height = 256usize;
    let dim = Point::new(width as i32, height as i32);;

    let mut buffer: Vec<[u8; 4]> = vec![[255, 255, 255, 255]; width * height];

    let count: usize = std::env::args()
        .nth(1)
        .and_then(|a| {
            let x: usize = a
                .parse::<usize>()
                .expect("Argument has to be a positive number");
            Some(x)
        })
        .unwrap_or(512);
    let g = graph::Graph::new_random(count);

    let mx = Matrix::new((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)) * 64.0;
    for (p1, p2) in g.edges.iter().map(|(a, b)| (&mx * *a, &mx * *b)) {
        println!("{:?} to {:?}", p1, p2);
        let p1: Point<i32> = Point::new(p1.0, p1.1).into();
        let p2: Point<i32> = Point::new(p2.0, p2.1).into();
        for pxl in render::coordinates::LineIterator::new(p1, p2) {
            if pxl.pos.in_bounds(Point::new(0, 0), dim) {
                buffer[pxl.pos.x() as usize + pxl.pos.y() as usize * width] = [0, 0, 0, 255]
            }
        }
    }

    let mut frame_buffer = mini_gl_fb::gotta_go_fast("Random Graph", width as f64, height as f64);

    frame_buffer.update_buffer(&buffer);

    frame_buffer.persist();
}
