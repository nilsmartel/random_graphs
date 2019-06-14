pub struct Line {
    start: Point<i32>,
    end: Point<i32>,
    current: usize,
    steps: usize,
}

struct LineItem {
    pos: Point,
    progress: f64,
}

impl Iterator for Line {
    type Item = LineItem;

    fn next(&mut self) -> Option<LineItem> {
        if self.current == self.steps {
            return None;
        }
    }
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        let path = start - end;

        let steps = if path.x().abs() > path.y().abs() {
            path.x().abs() as usize
        } else {
            path.y().abs() as usize
        };

        Line {
            start,
            end,
            current: 0,
            steps,
        }
    }

    #[inline]
    pub fn progress(&self) -> f64 {
        self.current as f64 / self.steps as f64
    }
}

#[derive(Copy, Clone)]
pub struct Point<T>(T, T)
where
    T: Copy
        + Clone
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>;

impl<T> Point<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Point<T> {
        Point(x, y)
    }

    #[inline]
    pub fn x(self) -> T {
        self.0
    }

    #[inline]
    pub fn y(self) -> T {
        self.1
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}
