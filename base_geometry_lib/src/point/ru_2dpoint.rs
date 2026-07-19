//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]




use crate::*;


//按照C语言内存规则
#[repr(C)]
#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone)]
//使用默认参数
pub struct RU_2dPoint {
    pub x: f64,
    pub y: f64,
}


impl RU_2dPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y}
    }
}
impl Point_Trait for RU_2dPoint {
    fn X(&self) -> f64 {
        self.x
    }
    fn Y(&self) -> f64 {
        self.y
    }
    fn Z(&self) -> f64 {
        0.0
    }
    fn W(&self) -> f64 {
        1.0
    }
    fn SetX(&mut self, x: f64){
        self.x = x;
    }
    fn SetY(&mut self, y: f64){
        self.y = y;
    }
    fn SetZ(&mut self, _z: f64){
        //do nothing
    }
    fn SetW(&mut self, _w: f64){
        //do nothing
    }
}



