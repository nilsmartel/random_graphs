mod graph;
mod render;
use render::coordinates::Point;

fn main() {
    let width = 512usize;
    let height = 256usize;

    let mut buffer = vec![[255, 255, 255, 255]; width * height];

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

    for (p1, p2) in g.edges.iter() {
        let p1: Point<i32> = Point::new(p1.0, p1.1).into();
        let p2: Point<i32> = Point::new(p2.0, p2.1).into();
        for pxl in render::coordinates::LineIterator::new(p1, p2) {
            if pxl.pos.x() >= 0
                && pxl.pos.x() < width as i32
                && pxl.pos.y() >= 0
                && pxl.pos.y() < width as i32
            {
                buffer[pxl.pos.x() as usize + pxl.pos.y() as usize * width] = [0, 0, 0, 255]
            }
        }
    }

    let mut frame_buffer = mini_gl_fb::gotta_go_fast("Random Graph", width as f64, height as f64);

    frame_buffer.update_buffer(&buffer);
}
