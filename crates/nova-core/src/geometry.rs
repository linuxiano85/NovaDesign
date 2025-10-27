use serde::{Deserialize, Serialize};

/// 2D point in plan view
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0)
    }
}

/// 3D point for future 3D rendering
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

/// Line segment in 2D
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}

impl Line2D {
    pub fn new(start: Point2D, end: Point2D) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Rectangle in 2D space
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

/// Polygon for complex shapes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<Point2D>,
}

impl Polygon {
    pub fn new(points: Vec<Point2D>) -> Self {
        Self { points }
    }

    /// Calculate approximate area using shoelace formula
    pub fn area(&self) -> f64 {
        if self.points.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        let n = self.points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            area += self.points[i].x * self.points[j].y;
            area -= self.points[j].x * self.points[i].y;
        }

        area.abs() / 2.0
    }
}
