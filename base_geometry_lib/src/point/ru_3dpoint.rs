//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use crate::*;

//按照C语言内存规则
#[repr(C)]
#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone)]
//使用默认参数
pub struct RU_3dPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl RU_3dPoint {
    //构造函数
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    pub const NAN: Self = Self {
        x: f64::NAN,
        y: f64::NAN,
        z: f64::NAN,
    };
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
    pub fn PointKey(&self) -> String {
        format!("({:.3},{:.3},{:.3})", self.x, self.y, self.z)
    }
    pub const ORIGIN: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub fn ToKey(&self, precision: usize) -> String {
        #[inline]
        fn round_clean(x: f64, p: usize) -> f64 {
            let factor = 10_f64.powi(p as i32);
            let v = (x * factor).round() / factor;
            if v == 0.0 { 0.0 } else { v }
        }


        format!(
            "({:.precision$},{:.precision$},{:.precision$})",
            round_clean(self.x, precision),
            round_clean(self.y, precision),
            round_clean(self.z, precision),
        )
    }
    
    pub fn quantize(&self, tol: f64) -> (i64, i64, i64) {
        (
            (self.x / tol).round() as i64,
            (self.y / tol).round() as i64,
            (self.z / tol).round() as i64,
        )
    }
    pub fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

//加法
impl std::ops::Add for RU_3dPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
//加法(向量)
impl std::ops::Add<RU_3dVector> for RU_3dPoint {
    type Output = Self;
    fn add(self, other: RU_3dVector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
//减法
impl std::ops::Sub for RU_3dPoint {
    type Output = RU_3dVector;
    fn sub(self, other: Self) -> RU_3dVector {
        RU_3dVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
//减法(向量)
impl std::ops::Sub<RU_3dVector> for RU_3dPoint {
    type Output = Self;
    fn sub(self, other: RU_3dVector) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


//乘法(浮点) 左
impl std::ops::Mul<f64> for RU_3dPoint {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
//乘法(浮点) 右
impl std::ops::Mul<RU_3dPoint> for f64 {
    type Output = RU_3dPoint;
    fn mul(self, other: RU_3dPoint) -> RU_3dPoint {
        RU_3dPoint {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}


//除法
impl std::ops::Div<f64> for RU_3dPoint {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}


impl Point_Trait for RU_3dPoint {
    fn X(&self) -> f64 {
        self.x
    }
    fn Y(&self) -> f64 {
        self.y
    }
    fn Z(&self) -> f64 {
        self.z
    }
    fn W(&self) -> f64 {
        1.0
    }
    fn SetX(&mut self, x: f64) {
        self.x = x;
    }
    fn SetY(&mut self, y: f64) {
        self.y = y;
    }
    fn SetZ(&mut self, z: f64) {
        self.z = z;
    }
    fn SetW(&mut self, _w: f64) {
        //do nothing
    }
}
//显示
impl std::fmt::Display for RU_3dPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.4},{:.4},{:.4})", self.x, self.y, self.z)
    }
}





