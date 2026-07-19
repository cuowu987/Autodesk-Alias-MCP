//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use crate::*;


//按照C语言内存规则
#[repr(C)]
#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct RU_3dVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl RU_3dVector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    ///返回单位向量
    pub fn UnitVector(&self) -> Self {
        let len = self.Length();
        if len < 1e-12 {
            panic!("Length too small");
        }
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    pub fn Unitize(&mut self) {
        let len = self.Length();
        if len < 1e-12 {
            panic!("Length too small");
        }
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }


    ///Angle
    pub fn Angle(&self, other: Self) -> f64 {
        let dot = *self * other;
        let len = self.Length() * other.Length();
        (dot / len).acos()
    }


    pub const NAN: Self = Self {
        x: f64::NAN,
        y: f64::NAN,
        z: f64::NAN,
    };
    pub const X_AXIS: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const Y_AXIS: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const Z_AXIS: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };


    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }


    /// 将三维方向向量映射到 (u,v) 参数方向分量
    /// 输入:
    /// - `Su`, `Sv`: 曲面在该点的两条切线方向
    /// 输出:
    /// - `(du, dv)`: 在参数空间中的分量（未归一化）
    pub fn MapToUV(&self, Su: RU_3dVector, Sv: RU_3dVector) -> (f64, f64) {
        //法线与投影
        let normal = (Su ^ Sv).UnitVector();
        let dir_proj = *self - normal * (normal * *self);
        let len = dir_proj.Length();


        // 避免几乎平行于法线的情况
        if len < 1e-12 {
            return (0.0, 0.0);
        }


        //计算第一基本形式分量
        let SuSu = Su * Su;
        let SuSv = Su * Sv;
        let SvSv = Sv * Sv;


        //右端项
        let rhs_u = Su * dir_proj;
        let rhs_v = Sv * dir_proj;


        //解线性系统
        let det = SuSu * SvSv - SuSv * SuSv;
        if det.abs() < 1e-12 {
            return (0.0, 0.0);
        }


        let du = (rhs_u * SvSv - rhs_v * SuSv) / det;
        let dv = (rhs_v * SuSu - rhs_u * SuSv) / det;
        (du, dv)
    }
    pub fn MapUVToVector(&self, Su: RU_3dVector, Sv: RU_3dVector) -> Result<RU_3dVector, String> {
        let vector = *self;
        let N = (Su ^ Sv).UnitVector();


        // 投影到切平面
        let Vt = vector - N * (vector * N);
        if Vt.Length() < 1e-6 {
            return Err("vector is perpendicular to surface".to_string());
        }
        let t = Vt.UnitVector();
        let (u, v) = t.MapToUV(Su.UnitVector(), Sv.UnitVector());
        let a = Su.Length() / u;
        let b = Sv.Length() / v;
        let result = if a.abs() < b.abs() { t * a } else { t * b };
        Ok(result)
    }
    pub fn Similar(&self, vectors: &[RU_3dVector]) -> usize {
        let (idx, _) = vectors
            .iter()
            .map(|v| *v * *self)
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        idx
    }
    pub fn Reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}


//加法
impl std::ops::Add for RU_3dVector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl std::ops::Add<RU_3dPoint> for RU_3dVector {
    type Output = RU_3dPoint;
    fn add(self, other: RU_3dPoint) -> RU_3dPoint {
        RU_3dPoint {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
//减法
impl std::ops::Sub for RU_3dVector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
//乘法(dot)
impl std::ops::Mul for RU_3dVector {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}


//乘法 (浮点数) (左)
impl std::ops::Mul<f64> for RU_3dVector {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
//乘法 (浮点数) (右)
impl std::ops::Mul<RU_3dVector> for f64 {
    type Output = RU_3dVector;
    fn mul(self, rhs: RU_3dVector) -> RU_3dVector {
        RU_3dVector {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}
//负数
impl std::ops::Neg for RU_3dVector {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


//除法(浮点数)
impl std::ops::Div<f64> for RU_3dVector {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
//cross
impl std::ops::BitXor for RU_3dVector {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}


impl Point_Trait for RU_3dVector {
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


impl Vector_Trait for RU_3dVector {}





