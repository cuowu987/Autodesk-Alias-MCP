//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]




use crate::*;


//按照C语言内存规则
#[repr(C)]
#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone)]
//使用默认参数
pub struct RU_4dPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}


impl RU_4dPoint {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }


}


impl Point_Trait for RU_4dPoint {
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
        self.w
    }
    fn SetX(&mut self, x: f64){
        self.x = x;
    }
    fn SetY(&mut self, y: f64){
        self.y = y;
    }
    fn SetZ(&mut self, z: f64){
        self.z = z;
    }
    fn SetW(&mut self, w: f64){
        self.w = w;
    }
}




