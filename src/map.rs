use std::ops::{Add, Sub, Div, Neg};

#[derive(Debug, Copy, Clone)]
pub enum MapItem {
    Empty,
    Tree,
}

pub struct GameMap {
    size: Coord,
    grid: Vec<MapItem>,
}

impl GameMap {
    pub fn new(size: Coord) -> GameMap {
        let item_count = (size.x * size.y) as usize;
        let mut grid = Vec::with_capacity(item_count);
        grid.resize(item_count, MapItem::Empty);
        GameMap { size, grid }
    }

    pub fn at(&self, point: Coord) -> MapItem {
        let idx = (point.x + point.y * self.size.x) as usize;
        self.grid[idx]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ScreenPoint {
    pub x: f64,
    pub y: f64,
}

impl ScreenPoint {
    pub fn new(x: f64, y: f64) -> ScreenPoint {
        ScreenPoint { x, y }
    }

    pub fn flip_y(self) -> ScreenPoint {
        Self::new(self.x, -self.y)
    }

    pub fn zero() -> ScreenPoint {
        Self::new(0f64, 0f64)
    }

}

impl Add for ScreenPoint {
    type Output = ScreenPoint;

    fn add(self, rhs: ScreenPoint) -> ScreenPoint {
        ScreenPoint::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for ScreenPoint {
    type Output = ScreenPoint;

    fn sub(self, rhs: ScreenPoint) -> ScreenPoint {
        ScreenPoint::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Div<i32> for ScreenPoint {
    type Output = ScreenPoint;

    fn div(self, rhs: i32) -> ScreenPoint {
        ScreenPoint::new(self.x / rhs as f64, self.y / rhs as f64)
    }
}

impl Neg for ScreenPoint {
    type Output = ScreenPoint;

    fn neg(self) -> ScreenPoint {
        ScreenPoint::new(-self.x, -self.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord::new(self.x + other.x, self.y + other.y)
    }
}
