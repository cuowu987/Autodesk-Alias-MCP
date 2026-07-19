#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct RU_2iPoint {
    pub x: i32,
    pub y: i32,
}

impl RU_2iPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn length_sq(&self) -> i64 {
        (self.x as i64) * (self.x as i64) + (self.y as i64) * (self.y as i64)
    }

    pub fn length(&self) -> f64 {
        ((self.x as f64) * (self.x as f64) + (self.y as f64) * (self.y as f64)).sqrt()
    }

    pub fn distance_sq(&self, other: Self) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx as i64) * (dx as i64) + (dy as i64) * (dy as i64)
    }

    pub fn distance(&self, other: Self) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn dot(&self, other: Self) -> i64 {
        (self.x as i64) * (other.x as i64) + (self.y as i64) * (other.y as i64)
    }

    pub fn to_f64(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }

    pub fn clamp(&self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.max(min.x).min(max.x),
            y: self.y.max(min.y).min(max.y),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl std::ops::Add for RU_2iPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for RU_2iPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<i32> for RU_2iPoint {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl std::ops::Mul<RU_2iPoint> for i32 {
    type Output = RU_2iPoint;
    fn mul(self, rhs: RU_2iPoint) -> RU_2iPoint {
        RU_2iPoint {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl std::ops::Div<i32> for RU_2iPoint {
    type Output = Self;
    fn div(self, other: i32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl std::ops::Neg for RU_2iPoint {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::AddAssign for RU_2iPoint {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for RU_2iPoint {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::MulAssign<i32> for RU_2iPoint {
    fn mul_assign(&mut self, other: i32) {
        self.x *= other;
        self.y *= other;
    }
}

impl std::ops::DivAssign<i32> for RU_2iPoint {
    fn div_assign(&mut self, other: i32) {
        self.x /= other;
        self.y /= other;
    }
}