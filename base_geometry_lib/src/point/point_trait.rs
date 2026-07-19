#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::*;

pub trait Point_Trait {
    fn X(&self) -> f64;
    fn Y(&self) -> f64;
    fn Z(&self) -> f64;
    fn W(&self) -> f64;

    fn SetX(&mut self, x: f64);
    fn SetY(&mut self, y: f64);
    fn SetZ(&mut self, z: f64);
    fn SetW(&mut self, w: f64);

    fn ToRU_3dPoint(&self) -> RU_3dPoint {
        RU_3dPoint::new(self.X(), self.Y(), self.Z())
    }
    fn ToRU_4dPoint(&self) -> RU_4dPoint {
        RU_4dPoint::new(self.X(), self.Y(), self.Z(), self.W())
    }
    fn ToRU_2dPoint(&self) -> RU_2dPoint {
        RU_2dPoint::new(self.X(), self.Y())
    }

    fn is_2dpoint(&self) -> bool {
        self.Z() == 0.0
    }
    fn to_3dvector(&self) -> RU_3dVector {
        RU_3dVector::new(self.X(), self.Y(), self.Z())
    }
    fn as_array3(&self) -> [f64; 3] {
        [self.X(), self.Y(), self.Z()]
    }
    fn as_array4(&self) -> [f64; 4] {
        [self.X(), self.Y(), self.Z(), self.W()]
    }

    /// # 容差端点量化
    fn quantize(&self, tol: f64) -> (i64, i64, i64) {
        (
            (self.X() / tol).round() as i64,
            (self.Y() / tol).round() as i64,
            (self.Z() / tol).round() as i64,
        )
    }
}


pub trait Vector_Trait: Point_Trait {
    fn LengthSquared(&self) -> f64 {
        self.X() * self.X() + self.Y() * self.Y() + self.Z() * self.Z()
    }
    fn Length(&self) -> f64 {
        self.LengthSquared().sqrt()
    }
    ///Unitize
    fn Unitize(&mut self) -> bool {
        let len = self.Length();
        if len < 1e-12 {
            return false;
        }
        self.SetX(self.X() / len);
        self.SetY(self.Y() / len);
        self.SetZ(self.Z() / len);
        true
    }
}

//为[f64,3] 实现 Vector_Trait
impl Point_Trait for [f64; 3] {
    fn X(&self) -> f64 {
        self[0]
    }
    fn Y(&self) -> f64 {
        self[1]
    }
    fn Z(&self) -> f64 {
        self[2]
    }
    fn W(&self) -> f64 {
        1.0
    }
    fn SetX(&mut self, x: f64) {
        self[0] = x;
    }
    fn SetY(&mut self, y: f64) {
        self[1] = y;
    }
    fn SetZ(&mut self, z: f64) {
        self[2] = z;
    }
    fn SetW(&mut self, _w: f64) {
        //do nothing
    }
}

impl Vector_Trait for [f64; 3] {}
//为[f64,4] 实现 Vector_Trait
impl Point_Trait for [f64; 4] {
    fn X(&self) -> f64 {
        self[0]
    }
    fn Y(&self) -> f64 {
        self[1]
    }
    fn Z(&self) -> f64 {
        self[2]
    }
    fn W(&self) -> f64 {
        self[3]
    }
    fn SetX(&mut self, x: f64) {
        self[0] = x;
    }
    fn SetY(&mut self, y: f64) {
        self[1] = y;
    }
    fn SetZ(&mut self, z: f64) {
        self[2] = z;
    }
    fn SetW(&mut self, w: f64) {
        self[3] = w;
    }
}

impl Vector_Trait for [f64; 4] {}
