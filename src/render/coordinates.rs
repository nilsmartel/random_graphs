pub struct LineIterator {
    start: Point<i32>,
    end: Point<i32>,

    // current step
    current: usize,
    // (max) number of steps to take until the iterator has completed
    steps: usize,

    // Current Position on the line.
    current_position: Point<f64>,
    // Steps to increment current position on line.
    position_increment: Point<f64>,
}

pub struct LineItem {
    pub pos: Point<i32>,
    pub progress: f64,
}

impl Iterator for LineIterator {
    type Item = LineItem;

    fn next(&mut self) -> Option<LineItem> {
        if self.current == self.steps {
            return None;
        }

        let result = LineItem {
            pos: self.current_position.into(),
            progress: self.progress(),
        };
        self.current_position = self.current_position + self.position_increment;
        self.current += 1;

        Some(result)
    }
}

impl LineIterator {
    pub fn new(start: Point<i32>, end: Point<i32>) -> LineIterator {
        let path: Point<i32> = start - end;

        let steps = if path.x().abs() > path.y().abs() {
            path.x().abs() as usize
        } else {
            path.y().abs() as usize
        };

        // TODO ask on users.rust-lang.org how to do this without temporary var
        let path: Point<f64> = path.into();
        let position_increment = path / (steps as f64);

        let current_position: Point<f64> = start.into();

        LineIterator {
            start,
            end,
            current: 0,
            steps,
            current_position,
            position_increment,
        }
    }

    #[inline]
    pub fn progress(&self) -> f64 {
        self.current as f64 / self.steps as f64
    }
}

impl From<Point<i32>> for Point<f64> {
    fn from(p: Point<i32>) -> Self {
        Point(p.x() as f64, p.y() as f64)
    }
}

impl From<Point<i32>> for Point<f32> {
    fn from(p: Point<i32>) -> Self {
        Point(p.x() as f32, p.y() as f32)
    }
}

impl From<Point<f32>> for Point<i32> {
    fn from(p: Point<f32>) -> Self {
        Point(p.x() as i32, p.y() as i32)
    }
}

impl From<Point<f64>> for Point<i32> {
    fn from(p: Point<f64>) -> Self {
        Point(p.x() as i32, p.y() as i32)
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

impl<T> Point<T>
where
    T: Copy
        + Clone
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
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

impl<T> Point<T>
where
    T: std::cmp::PartialOrd<T>
        + Copy
        + Clone
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    pub fn in_bounds(&self, start: Self, dimensions: Self) -> bool {
        self.0 >= start.0 && self.0 < dimensions.0 && self.1 >= start.1 && self.1 < dimensions.1
    }
}

impl<T> std::ops::Add for Point<T>
where
    T: Copy
        + Clone
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl<T> std::ops::Sub for Point<T>
where
    T: Copy
        + Clone
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl<T> std::ops::Mul<T> for Point<T>
where
    T: Copy
        + Clone
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    type Output = Self;
    fn mul(self, factor: T) -> Self::Output {
        Point(self.0 * factor, self.1 * factor)
    }
}

impl<T> std::ops::Div<T> for Point<T>
where
    T: Copy
        + Clone
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    type Output = Self;
    fn div(self, factor: T) -> Self::Output {
        Point(self.0 / factor, self.1 / factor)
    }
}
