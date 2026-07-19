//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]




//use rayon::prelude::*;




use crate::*;




//按照C语言内存规则
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
//使用默认参数
pub struct RU_Interval {
    pub m_t0: f64,
    pub m_t1: f64,
}




impl RU_Interval {
    pub fn new(t0: f64, t1: f64) -> Self {
        Self { m_t0: t0, m_t1: t1 }
    }
    //构造函数
    pub fn createByStartAndEnd(t0: f64, t1: f64) -> Self {
        Self { m_t0: t0, m_t1: t1 }
    }
    pub fn Min(&self) -> f64 {
        self.m_t0.min(self.m_t1)
    }
    pub fn mid(&self) -> f64 {
        0.5 * (self.m_t0 + self.m_t1)
    }
    pub const NAN: Self = Self {
        m_t0: f64::NAN,
        m_t1: f64::NAN,
    };
    pub fn is_nan(&self) -> bool {
        self.m_t0.is_nan() || self.m_t1.is_nan()
    }




    pub fn Max(&self) -> f64 {
        self.m_t0.max(self.m_t1)
    }
    pub fn Length(&self) -> f64 {
        (self.m_t1 - self.m_t0).abs()
    }




    pub fn range(&self, div: usize) -> Vec<f64> {
        linspace(self.m_t0, self.m_t1, div + 1)
    }
    pub fn rangeX(&self, num: usize) -> Vec<f64> {
        linspace(self.m_t0, self.m_t1, num)
    }




    /// 计算t在[0,1]区间的映射值
    pub fn ParameterAt(&self, t: f64) -> f64 {
        self.m_t0 + (self.m_t1 - self.m_t0) * t
    }
    /// 计算t在[m_t0,m_t1]区间的映射值[0.0,1.0]
    pub fn NormalizedParameterAt(&self, t: f64) -> f64 {
        (t - self.m_t0) / (self.m_t1 - self.m_t0)
    }
    /// 拓展区间
    pub fn Extend(&mut self, t: f64) {
        if t < self.m_t0 {
            self.m_t0 = t;
        }
        if t > self.m_t1 {
            self.m_t1 = t;
        }
    }
    /// # 是否被包含
    pub fn Contains_other(&self, other: &RU_Interval) -> bool {
        self.m_t0 <= other.m_t0 && self.m_t1 >= other.m_t1
    }
    /// # 是否被包含
    pub fn Contains(&self, t: f64) -> bool {
        self.m_t0 <= t && self.m_t1 >= t
    }




    /// #  二分法求解
    /// F: f64 -> bool
    ///
    /// esp: 默认为1e-8
    pub fn bisection<F>(&self, mut func: F, eps: Option<f64>) -> Result<f64, String>
    where
        F: FnMut(f64) -> Result<bool, String>,
    {
        let eps = eps.unwrap_or(1e-8);




        let (mut a, mut b) = (self.Min(), self.Max());
        let mut fa = func(a)?;
        let fb = func(b)?;




        if fa == fb {
            return Err("bisection fa == fb".to_string()); //注意 -1的时候是fa与fb相等，无法进行二分法
        }




        while (b - a).abs() > eps {
            let mid = 0.5 * (a + b);
            let fm = func(mid)?;




            if fa != fm {
                b = mid;
                //fb = fm;
            } else {
                a = mid;
                fa = fm;
            }
        }




        Ok(0.5 * (a + b))
    }




    /// # 限制参数最大最小值
    /// t: f64
    ///
    /// return: 返回是否被限制
    pub fn Clamp(&self, t: &mut f64) -> bool {
        if *t < self.m_t0 {
            *t = self.m_t0;
            return true;
        }
        if *t > self.m_t1 {
            *t = self.m_t1;
            return true;
        }
        false
    }




    pub fn linspace(&self, n: usize) -> Vec<f64> {
        linspace(self.m_t0, self.m_t1, n)
    }
}
impl Default for RU_Interval {
    fn default() -> Self {
        Self {
            m_t0: 0.0,
            m_t1: 1.0,
        }
    }
}




//打印显示
impl std::fmt::Display for RU_Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RU_Interval({:.2},{:.2})", self.m_t0, self.m_t1)
    }
}















