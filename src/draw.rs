pub struct Line {
    start: Point<i32>,
    end: Point<i32>,
    current: usize,
    steps: usize,
}

struct LineItem {
    pos: Point<i32>,
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
    pub fn new(start: Point<i32>, end: Point<i32>) -> Line {
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
